"""Tier 1 tests for `ReportContext.__exit__`'s replay-worker handling.

Each test substitutes the `import-test-result-log` argv with a tiny Python
`-c` invocation that produces a controlled end-state (clean exit / hang /
non-zero exit), then enters and exits a `ReportContext` against a
simulate-mode `SiftClient`. This validates that real subprocess outcomes
route to the right branch of `__exit__` without depending on the real
replay binary or a Sift backend.
"""

from __future__ import annotations

import sys
import warnings

import pytest

from sift_client import SiftClient, SiftConnectionConfig
from sift_client.errors import SiftWarning
from sift_client.util.test_results import ReportContext


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


def _make_context(command: list[str], *, timeout: float = 0.5) -> ReportContext:
    """Build a ReportContext whose replay subprocess is the provided command.

    ``log_file=True`` triggers the temp-file path so ``_open_import_proc`` fires
    on ``__enter__``. The substitute argv is swapped in via the public-ish
    ``_build_replay_command`` hook so the production Popen kwargs stay
    exercised. ``timeout`` overrides the worker grace window so tests don't
    wait the full production timeout for the timeout branch to trigger.
    """
    rc = ReportContext(_make_simulate_client(), name="test", log_file=True)
    rc._build_replay_command = lambda: command  # type: ignore[method-assign]
    rc._import_proc_timeout = timeout
    return rc


def test_worker_clean_exit_is_silent() -> None:
    """Worker exits with code 0 → __exit__ emits no SiftWarning (case 1)."""
    rc = _make_context([sys.executable, "-c", "pass"])
    with warnings.catch_warnings(record=True) as recorded:
        warnings.simplefilter("always")
        with rc:
            pass
    sift_warnings = [w for w in recorded if issubclass(w.category, SiftWarning)]
    assert sift_warnings == []
    assert rc._import_proc is not None
    assert rc._import_proc.returncode == 0


def test_worker_timeout_kills_and_warns() -> None:
    """Worker still running at session end → kill + SiftWarning, no raise (case 2)."""
    rc = _make_context([sys.executable, "-c", "import time; time.sleep(30)"], timeout=0.2)
    with pytest.warns(SiftWarning) as recorded:
        with rc:
            pass
    assert rc._import_proc is not None
    # `kill()` + `wait()` were called; process is dead.
    assert rc._import_proc.poll() is not None
    messages = "\n".join(str(w.message) for w in recorded)
    assert "did not exit in 0.2s" in messages
    # Recovery must resume from the tracking cursor, not batch-replay (which would
    # duplicate already-uploaded entries), so the hint carries --incremental.
    assert "import-test-result-log --incremental" in messages


def test_worker_nonzero_exit_warns_stderr_no_raise() -> None:
    """Worker exits non-zero with stderr → SiftWarning with stderr + replay hint, no raise (case 3)."""
    rc = _make_context(
        [
            sys.executable,
            "-c",
            "import sys; sys.stderr.write('rpc deadline exceeded'); sys.exit(2)",
        ]
    )
    with pytest.warns(SiftWarning) as recorded:
        with rc:
            pass
    assert rc._import_proc is not None
    assert rc._import_proc.returncode == 2
    messages = "\n".join(str(w.message) for w in recorded)
    assert "exited with code 2" in messages
    assert "rpc deadline exceeded" in messages
    assert "import-test-result-log --incremental" in messages
