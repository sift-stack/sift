"""Tier 1 tests for `ReportContext.__exit__`'s replay-worker handling.

Each test substitutes the `import-test-result-log` argv with a tiny Python
`-c` invocation that produces a controlled end-state (clean exit / hang /
non-zero exit), then enters and exits a `ReportContext` against a
simulate-mode `SiftClient`. This validates that real subprocess outcomes
route to the right branch of `__exit__` without depending on the real
replay binary or a Sift backend.
"""

from __future__ import annotations

import logging
import sys
from typing import TYPE_CHECKING

from sift_client import SiftClient, SiftConnectionConfig
from sift_client.util.test_results import ReportContext

if TYPE_CHECKING:
    import pytest


def _make_simulate_client() -> SiftClient:
    """Build a SiftClient flagged for in-process simulation.

    Constructor URLs are placeholders; nothing dials them because every
    test-results write short-circuits through the simulate path.
    """
    client = SiftClient(
        connection_config=SiftConnectionConfig(
            api_key="test",
            grpc_url="test.invalid:0",
            rest_url="http://test.invalid",
        )
    )
    client._simulate = True
    return client


def _make_context(command: list[str]) -> ReportContext:
    """Build a ReportContext whose replay subprocess is the provided command.

    `log_file=True` triggers the temp-file path so `_open_import_proc` fires
    on `__enter__`. The substitute argv is swapped in via the public-ish
    `_build_replay_command` hook so the production Popen kwargs stay
    exercised.
    """
    rc = ReportContext(_make_simulate_client(), name="test", log_file=True)
    rc._build_replay_command = lambda: command  # type: ignore[method-assign]
    return rc


def test_worker_clean_exit_is_silent(caplog: pytest.LogCaptureFixture) -> None:
    """Worker exits with code 0 → __exit__ is silent (case 1)."""
    rc = _make_context([sys.executable, "-c", "pass"])
    with caplog.at_level(logging.ERROR):
        with rc:
            pass
    assert "Import process" not in caplog.text
    assert "replay-test-result-log" not in caplog.text
    assert rc._import_proc is not None
    assert rc._import_proc.returncode == 0


def test_worker_timeout_kills_and_logs(caplog: pytest.LogCaptureFixture) -> None:
    """Worker still running at session end → kill + log, no raise (case 2)."""
    rc = _make_context([sys.executable, "-c", "import time; time.sleep(30)"])
    with caplog.at_level(logging.ERROR):
        with rc:
            pass
    assert rc._import_proc is not None
    # `kill()` + `wait()` were called; process is dead.
    assert rc._import_proc.poll() is not None
    assert "did not exit in 1s" in caplog.text
    assert "replay-test-result-log" in caplog.text


def test_worker_nonzero_exit_logs_stderr_no_raise(caplog: pytest.LogCaptureFixture) -> None:
    """Worker exits non-zero with stderr → log stderr + replay hint, no raise (case 3)."""
    rc = _make_context(
        [
            sys.executable,
            "-c",
            "import sys; sys.stderr.write('rpc deadline exceeded'); sys.exit(2)",
        ]
    )
    with caplog.at_level(logging.ERROR):
        with rc:
            pass
    assert rc._import_proc is not None
    assert rc._import_proc.returncode == 2
    assert "exited with code 2" in caplog.text
    assert "rpc deadline exceeded" in caplog.text
    assert "replay-test-result-log" in caplog.text
