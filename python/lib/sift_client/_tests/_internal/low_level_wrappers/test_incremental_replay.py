"""Unit tests for incremental log-replay resume, with no live backend.

These pin the resume-tick behavior of
``TestResultsLowLevelClient.import_log_file(incremental=True)``: the
CreateTestReport line is uploaded on an earlier tick, so a resuming tick rebuilds
replay state from scratch and must apply the remaining lines without an
in-memory report. The real gRPC create/update calls are stubbed, so these run
offline -- unlike the end-to-end resume test, which needs the integration server.
"""

from __future__ import annotations

import logging
from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock

import pytest

from sift_client._internal.low_level_wrappers._test_results_log import LogTracking
from sift_client._internal.low_level_wrappers.test_results import (
    # Aliased so pytest doesn't try to collect the `Test`-prefixed client as a suite.
    TestResultsLowLevelClient as ResultsLowLevelClient,
)
from sift_client.sift_types.test_report import (
    TestReport,
    TestReportCreate,
    TestReportUpdate,
    TestStatus,
    TestStep,
    TestStepCreate,
    TestStepType,
)
from sift_client.sift_types.test_report import (
    # Aliased so pytest doesn't try to collect the `Test`-prefixed update model.
    TestStepUpdate as StepUpdate,
)

T0 = datetime(2026, 1, 1, tzinfo=timezone.utc)


def _make_report(id_: str) -> TestReport:
    return TestReport(
        id_=id_,
        status=TestStatus.FAILED,
        name="n",
        test_system_name="s",
        test_case="c",
        start_time=T0,
        end_time=T0,
        metadata={},
        is_archived=False,
    )


def _make_step(id_: str) -> TestStep:
    return TestStep(
        id_=id_,
        test_report_id="real-report",
        name="step",
        step_type=TestStepType.ACTION,
        step_path="1",
        status=TestStatus.PASSED,
        start_time=T0,
        end_time=T0,
    )


def _report_create() -> TestReportCreate:
    return TestReportCreate(
        status=TestStatus.IN_PROGRESS,
        name="n",
        test_system_name="s",
        test_case="c",
        start_time=T0,
        end_time=T0,
    )


@pytest.mark.asyncio
async def test_resume_applies_trailing_report_update(tmp_path):
    """Resume whose remaining chunk is the final UpdateTestReport must apply it.

    Pre-fix this raised "UpdateTestReport found before CreateTestReport"; the
    status update then never landed and the report stayed IN_PROGRESS.
    """
    log_file = tmp_path / "resume_report_update.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())

    # Build the log offline via the simulate path: CreateTestReport + UpdateTestReport.
    report = await client.create_test_report(test_report=_report_create(), log_file=log_file)
    update = TestReportUpdate(status=TestStatus.FAILED)
    update.resource_id = report.id_
    await client.update_test_report(update=update, log_file=log_file)

    # An earlier tick already uploaded the CreateTestReport (line 1); the report
    # exists on the server under its real ID.
    LogTracking(last_uploaded_line=1, id_map={report.id_: "real-report"}).save(log_file)

    # Stub the real RPC the resumed tick will issue.
    client.update_test_report = AsyncMock(return_value=_make_report("real-report"))

    result = await client.import_log_file(log_file, incremental=True)

    client.update_test_report.assert_awaited_once()
    sent = client.update_test_report.await_args.kwargs["request"]
    assert sent.test_report.test_report_id == "real-report"
    assert sent.test_report.status == TestStatus.FAILED.value
    assert result.report is not None
    assert result.report.id_ == "real-report"


@pytest.mark.asyncio
async def test_resume_with_only_steps_does_not_require_report(tmp_path):
    """A resume tick carrying only steps must not demand an in-memory report.

    Pre-fix this raised "No CreateTestReport found in log file" (the field-report
    trace), aborting replay of the remaining step lines.
    """
    log_file = tmp_path / "resume_steps_only.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())

    report = await client.create_test_report(test_report=_report_create(), log_file=log_file)
    await client.create_test_step(
        test_step=TestStepCreate(
            test_report_id=report.id_,
            name="s1",
            step_type=TestStepType.ACTION,
            step_path="1",
            status=TestStatus.PASSED,
            start_time=T0,
            end_time=T0,
        ),
        log_file=log_file,
    )

    LogTracking(last_uploaded_line=1, id_map={report.id_: "real-report"}).save(log_file)

    client.create_test_step = AsyncMock(return_value=_make_step("real-step"))

    result = await client.import_log_file(log_file, incremental=True)

    client.create_test_step.assert_awaited_once()
    sent = client.create_test_step.await_args.kwargs["request"]
    # The step's report ID was remapped from the simulated ID to the real one.
    assert sent.test_step.test_report_id == "real-report"
    # The report was created on the earlier tick, so this resume tick has no report.
    assert result.report is None
    assert len(result.steps) == 1


@pytest.mark.asyncio
async def test_replay_upload_log_names_update_target(tmp_path):
    """The ``replay.upload`` line for an update carries the step it acted on.

    Updates mint no new entity, so the audit line used to leave sim_id/real_id
    blank. It now reports the target's simulated and remapped real IDs so a
    reader can tell which step each update touched.
    """
    log_file = tmp_path / "upload_log.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())

    # Build the log offline: create a report + step, then update the step.
    report = await client.create_test_report(test_report=_report_create(), log_file=log_file)
    step = await client.create_test_step(
        test_step=TestStepCreate(
            test_report_id=report.id_,
            name="s1",
            step_type=TestStepType.ACTION,
            step_path="1",
            status=TestStatus.IN_PROGRESS,
            start_time=T0,
            end_time=T0,
        ),
        log_file=log_file,
    )
    step_update = StepUpdate(status=TestStatus.PASSED)
    step_update.resource_id = step.id_
    await client.update_test_step(update=step_update, log_file=log_file)

    # Full replay from line 0; stub the real RPCs the replay issues.
    client.create_test_report = AsyncMock(return_value=_make_report("real-report"))
    client.create_test_step = AsyncMock(return_value=_make_step("real-step"))
    client.update_test_step = AsyncMock(return_value=_make_step("real-step"))

    # Capture directly on the module logger: the Sift plugin sets propagate=False
    # on the sift_client logger, so caplog's root handler wouldn't see the records.
    module_logger = logging.getLogger("sift_client._internal.low_level_wrappers.test_results")
    messages: list[str] = []
    handler = logging.Handler()
    handler.emit = lambda record: messages.append(record.getMessage())  # type: ignore[method-assign]
    prior_level = module_logger.level
    module_logger.addHandler(handler)
    module_logger.setLevel(logging.DEBUG)
    try:
        await client.import_log_file(log_file, incremental=True)
    finally:
        module_logger.removeHandler(handler)
        module_logger.setLevel(prior_level)

    upload_lines = [m for m in messages if m.startswith("replay.upload")]
    update_line = next(line for line in upload_lines if "type=UpdateTestStep" in line)
    # Pre-fix this line read ``sim_id=- real_id=-``; now it names the target.
    assert f"sim_id={step.id_}" in update_line
    assert "real_id=real-step" in update_line


# ---------------------------------------------------------------------------
# Session directory grouping
# ---------------------------------------------------------------------------


def test_make_session_dir_layout(tmp_path, monkeypatch):
    """``_make_session_dir`` creates ``<tmpdir>/sift_test_results/<random>/``.

    The dir name is used as the shared prefix for all session artifacts.
    """
    import tempfile

    from sift_client._internal.pytest_plugin.audit_log import _make_session_dir

    monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
    session_dir = _make_session_dir()
    assert session_dir.parent == tmp_path / "sift_test_results"
    assert session_dir.is_dir()
    # Name is a non-empty random token from mkdtemp.
    assert session_dir.name


def test_make_session_dir_concurrent_calls_unique(tmp_path, monkeypatch):
    """Each ``_make_session_dir`` call produces a distinct directory."""
    import tempfile

    from sift_client._internal.pytest_plugin.audit_log import _make_session_dir

    monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
    dirs = {_make_session_dir() for _ in range(5)}
    assert len(dirs) == 5


def test_cleanup_temp_log_removes_session_dir(tmp_path, monkeypatch):
    """``_cleanup_temp_log`` removes the whole session dir when audit is off.

    Session dir layout: ``<tmpdir>/sift_test_results/<random>/``. The JSONL,
    its tracking sidecar, and any audit files in the dir are all removed.
    """
    import tempfile

    from sift_client.scripts.import_test_result_log import _cleanup_temp_log

    monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
    session_dir = tmp_path / "sift_test_results" / "abc123"
    session_dir.mkdir(parents=True)
    log = session_dir / "abc123.jsonl"
    tracking = session_dir / "abc123.jsonl.tracking"
    audit = session_dir / "abc123-audit.log"
    for f in (log, tracking, audit):
        f.write_text("{}")

    _cleanup_temp_log(str(log))

    assert not session_dir.exists()


def test_cleanup_temp_log_ignores_explicit_path(tmp_path, monkeypatch):
    """``_cleanup_temp_log`` does not touch a log outside the temp dir."""
    import tempfile

    from sift_client.scripts.import_test_result_log import _cleanup_temp_log

    monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
    explicit_log = tmp_path.parent / "my_project_log.jsonl"
    explicit_log.write_text("{}")
    _cleanup_temp_log(str(explicit_log))
    assert explicit_log.exists()
    explicit_log.unlink()


def test_cleanup_temp_log_legacy_flat_layout(tmp_path, monkeypatch):
    """Legacy flat-temp layout: only the JSONL and its tracking sidecar are removed."""
    import tempfile

    from sift_client.scripts.import_test_result_log import _cleanup_temp_log

    monkeypatch.setattr(tempfile, "gettempdir", lambda: str(tmp_path))
    log = tmp_path / "tmp12345.jsonl"
    tracking = tmp_path / "tmp12345.jsonl.tracking"
    other = tmp_path / "other_file.txt"
    for f in (log, tracking, other):
        f.write_text("{}")

    _cleanup_temp_log(str(log))

    assert not log.exists()
    assert not tracking.exists()
    assert other.exists()
