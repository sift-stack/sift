"""Unit tests for incremental log-replay resume, with no live backend.

These pin the resume-tick behavior of
``TestResultsLowLevelClient.import_log_file(incremental=True)``: the
CreateTestReport line is uploaded on an earlier tick, so a resuming tick rebuilds
replay state from scratch and must apply the remaining lines without an
in-memory report. The real gRPC create/update calls are stubbed, so these run
offline -- unlike the end-to-end resume test, which needs the integration server.
"""

from __future__ import annotations

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
