"""Unit tests for incremental log-replay resume, with no live backend.

These pin the resume-tick behavior of
``TestResultsLowLevelClient.import_log_file(incremental=True)``: the
CreateTestReport line is uploaded on an earlier tick, so a resuming tick rebuilds
replay state from scratch and must apply the remaining lines without an
in-memory report. The real gRPC create/update calls are stubbed, so these run
offline -- unlike the end-to-end resume test, which needs the integration server.
"""

from __future__ import annotations

import json
import logging
from contextlib import contextmanager
from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock

import pytest
from grpc import RpcError, StatusCode

from sift_client._internal.low_level_wrappers._test_results_log import LogTracking
from sift_client._internal.low_level_wrappers.test_results import (
    # Aliased so pytest doesn't try to collect the `Test`-prefixed client as a suite.
    TestResultsLowLevelClient as ResultsLowLevelClient,
)
from sift_client.sift_types.test_report import (
    TestMeasurement,
    TestMeasurementCreate,
    TestMeasurementType,
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


@contextmanager
def _captured_replay_logs():
    """Collect the replay module's log messages for the duration of the block.

    Captured on the module logger directly: the Sift plugin sets
    propagate=False on the sift_client logger, so caplog's root handler would
    not see these records.
    """
    module_logger = logging.getLogger("sift_client._internal.low_level_wrappers.test_results")
    messages: list[str] = []
    handler = logging.Handler()
    handler.emit = lambda record: messages.append(record.getMessage())  # type: ignore[method-assign]
    prior_level = module_logger.level
    module_logger.addHandler(handler)
    module_logger.setLevel(logging.DEBUG)
    try:
        yield messages
    finally:
        module_logger.removeHandler(handler)
        module_logger.setLevel(prior_level)


def _make_measurement(id_: str) -> TestMeasurement:
    return TestMeasurement(
        id_=id_,
        test_step_id="real-step",
        name="m",
        passed=True,
        timestamp=T0,
        measurement_type=TestMeasurementType.DOUBLE,
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

    with _captured_replay_logs() as messages:
        await client.import_log_file(log_file, incremental=True)

    upload_lines = [m for m in messages if m.startswith("replay.upload")]
    update_line = next(line for line in upload_lines if "type=UpdateTestStep" in line)
    # Pre-fix this line read ``sim_id=- real_id=-``; now it names the target.
    assert f"sim_id={step.id_}" in update_line
    assert "real_id=real-step" in update_line


# ---------------------------------------------------------------------------
# Resuming an interrupted upload
# ---------------------------------------------------------------------------


class _NotFoundError(RpcError):
    """Stand-in for the server's response when a report has been deleted."""

    def code(self) -> StatusCode:
        return StatusCode.NOT_FOUND


class _PermissionDeniedError(RpcError):
    """Stand-in for a failure that resuming into a new report would not fix."""

    def code(self) -> StatusCode:
        return StatusCode.PERMISSION_DENIED


async def _build_log(client, log_file, *, step_names=("s1", "s2")):
    """Write a log offline: create a report, create each step, then close step one.

    Returns the simulated report and steps, whose IDs are the keys a later
    resume looks up in the tracking sidecar.
    """
    report = await client.create_test_report(test_report=_report_create(), log_file=log_file)
    steps = []
    for index, name in enumerate(step_names, start=1):
        steps.append(
            await client.create_test_step(
                test_step=TestStepCreate(
                    test_report_id=report.id_,
                    name=name,
                    step_type=TestStepType.ACTION,
                    step_path=str(index),
                    status=TestStatus.IN_PROGRESS,
                    start_time=T0,
                    end_time=T0,
                ),
                log_file=log_file,
            )
        )
    step_update = StepUpdate(status=TestStatus.PASSED)
    step_update.resource_id = steps[0].id_
    await client.update_test_step(update=step_update, log_file=log_file)
    return report, steps


def _answer_real_creates(client, *, report_id, step_ids=(), measurement_ids=()):
    """Answer the real create calls with canned IDs, leaving simulation alone.

    Batch replay drives its in-memory collapse through the same create methods
    with ``simulate=True``, so a blanket mock would swallow those too. Returns
    the list that records the name of each real create, in order; the report is
    recorded as ``"report"`` since a report create carries no step name.
    """
    created: list[str] = []

    def answer(name, canned_ids, describe):
        real = getattr(client, name)
        remaining = iter(canned_ids)

        async def call(*args, **kwargs):
            if kwargs.get("simulate") or kwargs.get("log_file"):
                return await real(*args, **kwargs)
            created.append(describe(*args))
            return next(remaining)

        setattr(client, name, call)

    answer("create_test_report", [_make_report(report_id)], lambda *_: "report")
    answer("create_test_step", [_make_step(sid) for sid in step_ids], lambda create: create.name)
    answer(
        "create_test_measurement",
        [_make_measurement(mid) for mid in measurement_ids],
        lambda create: create.name,
    )
    return created


@pytest.mark.asyncio
async def test_batch_upload_records_what_it_created(tmp_path):
    """A batch upload leaves a sidecar naming every entity it created.

    Without it an interrupted batch upload is unrecoverable: nothing on disk
    says which report and steps already reached the server.
    """
    log_file = tmp_path / "batch.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, steps = await _build_log(client, log_file)

    created = _answer_real_creates(
        client, report_id="real-report", step_ids=["real-step-1", "real-step-2"]
    )

    await client.import_log_file(log_file)

    assert created == ["report", "s1", "s2"]
    tracking = LogTracking.load(log_file)
    assert tracking.id_map == {
        report.id_: "real-report",
        steps[0].id_: "real-step-1",
        steps[1].id_: "real-step-2",
    }
    # Everything reached the server, so a re-run has nothing to do. The cursor
    # stays at zero: batch creates in collapsed order, not log order.
    assert tracking.complete
    assert tracking.last_uploaded_line == 0


@pytest.mark.asyncio
async def test_interrupted_batch_upload_resumes_into_same_report(tmp_path):
    """Re-running after a batch upload died finishes it instead of duplicating it."""
    log_file = tmp_path / "partial.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, steps = await _build_log(client, log_file)

    # The batch upload created the report and the first step, then died. Its
    # cursor stays at zero: batch creates in collapsed order, not log order.
    LogTracking(
        last_uploaded_line=0,
        id_map={report.id_: "real-report", steps[0].id_: "real-step-1"},
    ).save(log_file)

    client.get_test_report = AsyncMock(return_value=_make_report("real-report"))
    client.create_test_report = AsyncMock(return_value=_make_report("duplicate-report"))
    client.create_test_step = AsyncMock(return_value=_make_step("real-step-2"))
    client.update_test_step = AsyncMock(return_value=_make_step("real-step-1"))

    result = await client.import_log_file(log_file)

    # The report and the first step were already sent, so neither is re-created.
    client.create_test_report.assert_not_awaited()
    client.create_test_step.assert_awaited_once()
    assert client.create_test_step.await_args.kwargs["request"].test_step.name == "s2"
    # The first step's closing update still has to be applied, or it stays open.
    client.update_test_step.assert_awaited_once()
    assert (
        client.update_test_step.await_args.kwargs["request"].test_step.test_step_id == "real-step-1"
    )
    assert result.report is not None
    assert result.report.id_ == "real-report"
    assert LogTracking.load(log_file).id_map[steps[1].id_] == "real-step-2"


@pytest.mark.asyncio
async def test_worker_tick_never_marks_the_log_complete(tmp_path):
    """A worker tick must not mark a log that is still being written as complete.

    The worker ticks against a growing log, so reaching the end of the file is
    not the end of the run. Marking it complete would make a manual re-run after
    the worker died silently upload nothing, dropping every entry logged after
    the last tick.
    """
    log_file = tmp_path / "worker_tick.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, steps = await _build_log(client, log_file, step_names=("s1",))

    # An earlier tick uploaded the report; the cursor is past line one.
    LogTracking(last_uploaded_line=1, id_map={report.id_: "real-report"}).save(log_file)

    client.create_test_step = AsyncMock(return_value=_make_step("real-step-1"))
    client.update_test_step = AsyncMock(return_value=_make_step("real-step-1"))

    await client.import_log_file(log_file, incremental=True)

    tracking = LogTracking.load(log_file)
    assert not tracking.complete
    assert tracking.last_uploaded_line == 3


@pytest.mark.asyncio
async def test_idle_worker_tick_is_silent_and_writes_nothing(tmp_path):
    """A tick with nothing new to upload must not log or touch the sidecar.

    The worker ticks once a second for the whole session, so anything it does on
    an idle tick is multiplied by the length of the run: audit-log noise that
    buries the real entries, and sidecar rewrites that are pure churn.
    """
    log_file = tmp_path / "idle_tick.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, _ = await _build_log(client, log_file, step_names=("s1",))

    # The sidecar is caught up with every line currently in the log.
    LogTracking(last_uploaded_line=3, id_map={report.id_: "real-report"}).save(log_file)
    sidecar = LogTracking.sidecar_path(log_file)
    before = sidecar.read_bytes(), sidecar.stat().st_mtime_ns

    with _captured_replay_logs() as messages:
        await client.import_log_file(log_file, incremental=True)

    assert messages == []
    assert (sidecar.read_bytes(), sidecar.stat().st_mtime_ns) == before


@pytest.mark.asyncio
async def test_resume_marks_the_log_complete(tmp_path):
    """Finishing a resumed upload marks it complete, so a re-run does nothing."""
    log_file = tmp_path / "resume_completes.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, steps = await _build_log(client, log_file, step_names=("s1",))

    LogTracking(id_map={report.id_: "real-report"}).save(log_file)

    client.get_test_report = AsyncMock(return_value=_make_report("real-report"))
    client.create_test_step = AsyncMock(return_value=_make_step("real-step-1"))
    client.update_test_step = AsyncMock(return_value=_make_step("real-step-1"))

    await client.import_log_file(log_file)

    assert LogTracking.load(log_file).complete


@pytest.mark.asyncio
async def test_completed_upload_is_a_noop(tmp_path):
    """A log whose sidecar is caught up is not replayed again."""
    log_file = tmp_path / "done.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, steps = await _build_log(client, log_file)

    LogTracking(
        complete=True,
        id_map={report.id_: "real-report", steps[0].id_: "real-step-1"},
    ).save(log_file)

    client.get_test_report = AsyncMock()
    client.create_test_report = AsyncMock()
    client.create_test_step = AsyncMock()

    result = await client.import_log_file(log_file)

    client.get_test_report.assert_not_awaited()
    client.create_test_report.assert_not_awaited()
    client.create_test_step.assert_not_awaited()
    assert result.report is None
    assert result.steps == []


@pytest.mark.asyncio
async def test_new_report_abandons_the_partial_upload(tmp_path):
    """``new_report`` starts over and keeps the old sidecar as a backup."""
    log_file = tmp_path / "restart.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, steps = await _build_log(client, log_file)

    LogTracking(id_map={report.id_: "abandoned-report"}).save(log_file)

    client.get_test_report = AsyncMock()
    created = _answer_real_creates(
        client, report_id="fresh-report", step_ids=["real-step-1", "real-step-2"]
    )

    result = await client.import_log_file(log_file, new_report=True)

    client.get_test_report.assert_not_awaited()
    assert created == ["report", "s1", "s2"]
    assert result.report is not None
    assert result.report.id_ == "fresh-report"
    # The abandoned report's ID survives, so it can still be found and cleaned up.
    backup = LogTracking.backup_path(log_file)
    assert json.loads(backup.read_text())["idMap"] == {report.id_: "abandoned-report"}
    assert LogTracking.load(log_file).id_map[report.id_] == "fresh-report"


@pytest.mark.asyncio
async def test_resume_into_deleted_report_explains_the_override(tmp_path):
    """A report that no longer exists fails before anything is uploaded."""
    log_file = tmp_path / "deleted.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, _ = await _build_log(client, log_file)

    LogTracking(id_map={report.id_: "gone-report"}).save(log_file)

    client.get_test_report = AsyncMock(side_effect=_NotFoundError())
    client.create_test_report = AsyncMock()
    client.create_test_step = AsyncMock()

    with pytest.raises(ValueError, match="--new-report"):
        await client.import_log_file(log_file)

    client.create_test_report.assert_not_awaited()
    client.create_test_step.assert_not_awaited()


async def _build_measurement_log(client, log_file, *, batched):
    """Write a log holding a report, one step, and three measurements.

    ``batched`` picks how the measurements are logged: one ``CreateTestMeasurements``
    line covering all three, or a separate ``CreateTestMeasurement`` line each.
    """
    report = await client.create_test_report(test_report=_report_create(), log_file=log_file)
    step = await client.create_test_step(
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
    creates = [
        TestMeasurementCreate(
            name=f"m{index}",
            test_step_id=step.id_,
            passed=True,
            timestamp=T0,
            numeric_value=float(index),
        )
        for index in (1, 2, 3)
    ]
    if batched:
        _, measurement_ids = await client.create_test_measurements(
            test_measurements=creates, log_file=log_file
        )
    else:
        measurement_ids = [
            (await client.create_test_measurement(test_measurement=create, log_file=log_file)).id_
            for create in creates
        ]
    return report, step, measurement_ids


@pytest.mark.asyncio
async def test_resume_sends_only_the_missing_part_of_a_batched_line(tmp_path):
    """One log line can create many measurements, and only some may have made it.

    Batch replay creates measurements one at a time, so an interrupted run can
    leave a batched line half done. Re-sending the whole line would duplicate
    the measurements that already exist.
    """
    log_file = tmp_path / "batched_measurements.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, step, measurement_ids = await _build_measurement_log(client, log_file, batched=True)

    # The interrupted run got through the report, the step, and the first
    # measurement of the batched line.
    LogTracking(
        id_map={
            report.id_: "real-report",
            step.id_: "real-step",
            measurement_ids[0]: "real-meas-1",
        },
    ).save(log_file)

    client.get_test_report = AsyncMock(return_value=_make_report("real-report"))
    client.create_test_measurements = AsyncMock(return_value=(2, ["real-meas-2", "real-meas-3"]))

    await client.import_log_file(log_file)

    client.create_test_measurements.assert_awaited_once()
    sent = client.create_test_measurements.await_args.kwargs["request"]
    assert [m.name for m in sent.test_measurements] == ["m2", "m3"]
    tracking = LogTracking.load(log_file)
    assert tracking.id_map[measurement_ids[1]] == "real-meas-2"
    assert tracking.id_map[measurement_ids[2]] == "real-meas-3"
    # The one that already existed keeps the ID the interrupted run recorded.
    assert tracking.id_map[measurement_ids[0]] == "real-meas-1"


@pytest.mark.asyncio
async def test_resume_skips_a_measurement_already_created(tmp_path):
    """A measurement logged on its own line is skipped once it is in the id map."""
    log_file = tmp_path / "single_measurements.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, step, measurement_ids = await _build_measurement_log(client, log_file, batched=False)

    LogTracking(
        id_map={
            report.id_: "real-report",
            step.id_: "real-step",
            measurement_ids[0]: "real-meas-1",
            measurement_ids[1]: "real-meas-2",
        },
    ).save(log_file)

    client.get_test_report = AsyncMock(return_value=_make_report("real-report"))
    client.create_test_measurement = AsyncMock(return_value=_make_measurement("real-meas-3"))

    await client.import_log_file(log_file)

    client.create_test_measurement.assert_awaited_once()
    sent = client.create_test_measurement.await_args.kwargs["request"]
    assert sent.test_measurement.name == "m3"
    assert sent.test_measurement.test_step_id == "real-step"


@pytest.mark.asyncio
async def test_batch_upload_records_measurements(tmp_path):
    """Measurements created by a batch upload are recorded like steps are.

    Without this the tail of a large upload is the part a resume cannot skip.
    """
    log_file = tmp_path / "batch_measurements.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, step, measurement_ids = await _build_measurement_log(client, log_file, batched=False)

    created = _answer_real_creates(
        client,
        report_id="real-report",
        step_ids=["real-step"],
        measurement_ids=["real-meas-1", "real-meas-2", "real-meas-3"],
    )

    await client.import_log_file(log_file)

    assert created == ["report", "s1", "m1", "m2", "m3"]
    tracking = LogTracking.load(log_file)
    assert [tracking.id_map[mid] for mid in measurement_ids] == [
        "real-meas-1",
        "real-meas-2",
        "real-meas-3",
    ]


@pytest.mark.asyncio
async def test_batch_upload_tolerates_an_untagged_create(tmp_path):
    """A create logged without a response ID is uploaded but cannot be recorded.

    Nothing in the client writes such a line today, and one carrying later
    updates would fail replay outright since the updates could not be remapped.
    A trailing untagged create still has to reach the server; it just cannot be
    skipped by a later resume, which beats aborting the whole upload.
    """
    log_file = tmp_path / "untagged.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report = await client.create_test_report(test_report=_report_create(), log_file=log_file)
    step = await client.create_test_step(
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
    log_file.write_text(
        log_file.read_text().replace(f"[CreateTestStep:{step.id_}]", "[CreateTestStep]")
    )

    created = _answer_real_creates(client, report_id="real-report", step_ids=["real-step-1"])

    await client.import_log_file(log_file)

    assert created == ["report", "s1"]
    tracking = LogTracking.load(log_file)
    assert tracking.id_map == {report.id_: "real-report"}


@pytest.mark.asyncio
async def test_resume_without_a_recorded_report_explains_the_override(tmp_path):
    """A sidecar that records work but no report cannot say what to resume into."""
    log_file = tmp_path / "no_report.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    _, steps = await _build_log(client, log_file)

    LogTracking(id_map={steps[0].id_: "real-step-1"}).save(log_file)

    client.get_test_report = AsyncMock()
    client.create_test_report = AsyncMock()

    with pytest.raises(ValueError, match="--new-report"):
        await client.import_log_file(log_file)

    client.get_test_report.assert_not_awaited()
    client.create_test_report.assert_not_awaited()


@pytest.mark.asyncio
async def test_resume_propagates_errors_other_than_a_missing_report(tmp_path):
    """Only a missing report is turned into resume guidance.

    A permissions or connectivity failure is not something --new-report fixes,
    so it surfaces as itself.
    """
    log_file = tmp_path / "denied.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    report, _ = await _build_log(client, log_file)

    LogTracking(id_map={report.id_: "real-report"}).save(log_file)

    client.get_test_report = AsyncMock(side_effect=_PermissionDeniedError())

    with pytest.raises(RpcError):
        await client.import_log_file(log_file)


@pytest.mark.asyncio
async def test_incremental_and_new_report_are_rejected_together(tmp_path):
    """The two flags contradict each other, so asking for both is an error.

    Incremental replay continues whatever the sidecar records, which is exactly
    what new_report discards. Silently honouring one of them would upload into
    the wrong report.
    """
    log_file = tmp_path / "conflict.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    await _build_log(client, log_file, step_names=("s1",))

    with pytest.raises(ValueError, match="mutually exclusive"):
        await client.import_log_file(log_file, incremental=True, new_report=True)


@pytest.mark.asyncio
async def test_new_report_on_a_fresh_log_writes_no_backup(tmp_path):
    """``new_report`` against a log that was never uploaded has nothing to move aside."""
    log_file = tmp_path / "fresh.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    await _build_log(client, log_file, step_names=("s1",))

    created = _answer_real_creates(client, report_id="fresh-report", step_ids=["real-step-1"])

    await client.import_log_file(log_file, new_report=True)

    assert created == ["report", "s1"]
    assert not LogTracking.backup_path(log_file).exists()


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
