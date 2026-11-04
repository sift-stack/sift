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
from sift_client.errors import SiftReplayReconciledWarning, SiftWarning
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


def test_worker_clean_exit_with_reconciled_measurements_warns() -> None:
    """A reconciled run exits 0, so the marker on stderr is the only signal.

    ``_import_proc.returncode`` can't carry this: 0 is also what a run with
    nothing to reconcile returns. ``RECONCILED_MARKER`` on stderr is what
    `__exit__` has to check regardless of exit code.
    """
    rc = _make_context(
        [
            sys.executable,
            "-c",
            "import sys; sys.stderr.write('SIFT_REPLAY_RECONCILED:3'); sys.exit(0)",
        ]
    )
    with pytest.warns(SiftReplayReconciledWarning) as recorded:
        with rc:
            pass
    assert rc._import_proc is not None
    assert rc._import_proc.returncode == 0
    messages = "\n".join(str(w.message) for w in recorded)
    assert "reconciled 3 measurement(s)" in messages


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
    # Recovery must resume rather than re-upload (which would duplicate what the
    # worker already sent). The plain command resumes off the tracking sidecar,
    # so the hint must not send anyone to --incremental for it.
    assert "import-test-result-log" in messages
    assert "--incremental" not in messages


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
    assert "import-test-result-log" in messages
    assert "--incremental" not in messages


def test_replay_command_runs_module_through_current_interpreter() -> None:
    """Replay argv must not depend on PATH resolution.

    The ``import-test-result-log`` console script installs into the venv's
    ``bin/``, which is absent from PATH under ``sudo`` (sudoers' secure_path
    replaces PATH even with ``-E``) and when pytest runs from a non-activated
    venv. Spawning by bare name raised FileNotFoundError out of ``__enter__``
    there, erroring the session-scoped fixture and blocking every test.
    """
    rc = ReportContext(_make_simulate_client(), name="test", log_file=True)
    cmd = rc._build_replay_command()
    assert cmd[:3] == [
        sys.executable,
        "-m",
        "sift_client._internal.pytest_plugin.replay_worker",
    ]


_IGNORE_SIGTERM = (
    "import signal, time; signal.signal(signal.SIGTERM, signal.SIG_IGN); time.sleep(30)"
)


@pytest.mark.skipif(sys.platform == "win32", reason="SIGTERM is a hard kill on Windows")
def test_worker_that_ignores_sigterm_is_killed_after_the_grace_period() -> None:
    """The stop request is polite; a worker that does not honor it is still killed."""
    rc = _make_context([sys.executable, "-c", _IGNORE_SIGTERM], timeout=0.5)
    rc._import_proc_stop_grace = 0.3
    with pytest.warns(SiftWarning):
        with rc:
            pass
    assert rc._import_proc is not None
    assert rc._import_proc.returncode is not None


_REPORT_RECONCILED_ON_SIGTERM = (
    "import signal, sys, time\n"
    "def stop(*_):\n"
    "    sys.stderr.write('SIFT_REPLAY_RECONCILED:2')\n"
    "    sys.exit(0)\n"
    "signal.signal(signal.SIGTERM, stop)\n"
    "time.sleep(30)"
)


@pytest.mark.skipif(sys.platform == "win32", reason="SIGTERM is a hard kill on Windows")
def test_worker_stopped_on_timeout_still_reports_reconciled_measurements() -> None:
    """A worker stopped at the timeout warns about what it reconciled before stopping.

    The timeout branch returns before the normal-exit path, so it has to read
    the marker from the stop's stderr itself.
    """
    rc = _make_context([sys.executable, "-c", _REPORT_RECONCILED_ON_SIGTERM], timeout=0.5)
    with pytest.warns(SiftWarning) as recorded:
        with rc:
            pass
    reconciled = [w for w in recorded if issubclass(w.category, SiftReplayReconciledWarning)]
    assert len(reconciled) == 1
    assert "reconciled 2 measurement(s)" in str(reconciled[0].message)
    assert rc.replay_incomplete is True


def test_windows_timeout_asks_the_worker_to_stop_through_a_file(monkeypatch, tmp_path) -> None:
    """terminate() is a hard kill on Windows, so the stop request is a file the worker polls."""
    import types

    from sift_client.util.test_results import context_manager

    log = tmp_path / "log.jsonl"
    watcher = (
        "import os, sys, time\n"
        "deadline = time.time() + 20\n"
        "while not os.path.exists(sys.argv[1]) and time.time() < deadline:\n"
        "    time.sleep(0.05)\n"
        "sys.exit(0 if os.path.exists(sys.argv[1]) else 3)\n"
    )
    stop_file = str(log) + ".stop"
    rc = ReportContext(_make_simulate_client(), name="test", log_file=str(log))
    rc._build_replay_command = lambda: [sys.executable, "-c", watcher, stop_file]  # type: ignore[method-assign]
    rc._import_proc_timeout = 0.5
    rc._import_proc_stop_grace = 10.0
    monkeypatch.setattr(
        context_manager, "sys", types.SimpleNamespace(**{**vars(sys), "platform": "win32"})
    )
    with pytest.warns(SiftWarning):
        with rc:
            pass
    assert rc._import_proc is not None
    # The worker saw the file and left on its own, rather than being killed.
    assert rc._import_proc.returncode == 0
    # The file does not outlive the run, or it would stop the recovery upload.
    assert not (tmp_path / "log.jsonl.stop").exists()


def test_stale_stop_file_is_removed_when_the_worker_starts(tmp_path) -> None:
    log = tmp_path / "log.jsonl"
    stop_file = tmp_path / "log.jsonl.stop"
    stop_file.touch()
    check = "import os, sys; sys.exit(1 if os.path.exists(sys.argv[1]) else 0)"
    rc = ReportContext(_make_simulate_client(), name="test", log_file=str(log))
    rc._build_replay_command = lambda: [sys.executable, "-c", check, str(stop_file)]  # type: ignore[method-assign]
    rc._import_proc_timeout = 5.0
    with rc:
        pass
    assert rc._import_proc is not None
    assert rc._import_proc.returncode == 0
    assert not stop_file.exists()


def test_pending_upload_timeout_is_set_only_while_a_worker_can_be_waited_on() -> None:
    rc = _make_context([sys.executable, "-c", "pass"], timeout=5.0)
    assert rc.pending_upload_timeout is None
    with rc:
        assert rc.pending_upload_timeout == 5.0
    assert rc.pending_upload_timeout is None


@pytest.mark.parametrize(
    ("timeout", "verbose", "expected_lines"),
    [(120.0, 0, 1), (None, 0, 0), (120.0, -1, 0)],
    ids=["waiting", "no-worker", "quiet"],
)
def test_pending_upload_message(timeout, verbose, expected_lines) -> None:
    import types

    from sift_client.pytest_plugin import _announce_pending_upload

    lines: list[str] = []
    reporter = types.SimpleNamespace(write_line=lines.append)
    config = types.SimpleNamespace(
        pluginmanager=types.SimpleNamespace(get_plugin=lambda _name: reporter),
        option=types.SimpleNamespace(verbose=verbose),
    )
    context = types.SimpleNamespace(pending_upload_timeout=timeout)

    _announce_pending_upload(config, context)  # type: ignore[arg-type]

    assert len(lines) == expected_lines
    if expected_lines:
        assert "120s" in lines[0]
