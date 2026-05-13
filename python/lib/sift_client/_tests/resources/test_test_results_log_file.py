"""Unit tests for log_file plumbing in TestResultsAPIAsync."""

from __future__ import annotations

from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock, patch

import pytest
import pytest_asyncio

from sift_client.resources.test_results import TestResultsAPIAsync
from sift_client.sift_types.test_report import (
    TestMeasurement,
    TestMeasurementType,
    TestReport,
    TestStatus,
    TestStep,
    TestStepType,
)

T0 = datetime(2026, 1, 1, tzinfo=timezone.utc)


def _make_report(id_: str = "sim-report") -> TestReport:
    return TestReport(
        id_=id_,
        status=TestStatus.IN_PROGRESS,
        name="n",
        test_system_name="s",
        test_case="c",
        start_time=T0,
        end_time=T0,
        metadata={},
        is_archived=False,
    )


def _make_step(id_: str = "sim-step") -> TestStep:
    return TestStep(
        id_=id_,
        test_report_id="sim-report",
        name="step",
        step_type=TestStepType.ACTION,
        step_path="1",
        status=TestStatus.IN_PROGRESS,
        start_time=T0,
        end_time=T0,
    )


def _make_measurement(id_: str = "sim-meas") -> TestMeasurement:
    return TestMeasurement(
        id_=id_,
        measurement_type=TestMeasurementType.BOOLEAN,
        name="m",
        test_step_id="sim-step",
        boolean_value=True,
        passed=True,
        timestamp=T0,
    )


@pytest.fixture
def mock_client():
    client = MagicMock()
    client.grpc_client = MagicMock()
    client.rest_client = MagicMock()
    return client


@pytest_asyncio.fixture
def api(mock_client):
    """Build a TestResultsAPIAsync with mocked low-level + upload clients."""
    with patch(
        "sift_client.resources.test_results.TestResultsLowLevelClient",
        autospec=True,
    ), patch(
        "sift_client.resources.test_results.UploadLowLevelClient",
        autospec=True,
    ):
        return TestResultsAPIAsync(mock_client)


LOG = "/tmp/log.jsonl"


class TestCreateStamping:
    @pytest.mark.asyncio
    async def test_create_stamps_log_file(self, api):
        api._low_level_client.create_test_report = AsyncMock(return_value=_make_report())
        report_data = {
            "status": TestStatus.IN_PROGRESS,
            "name": "n",
            "test_system_name": "s",
            "test_case": "c",
            "start_time": T0,
            "end_time": T0,
        }
        result = await api.create(report_data, log_file=LOG)
        assert result._log_file == LOG
        assert api._low_level_client.create_test_report.call_args.kwargs["log_file"] == LOG

    @pytest.mark.asyncio
    async def test_create_step_stamps_log_file(self, api):
        api._low_level_client.create_test_step = AsyncMock(return_value=_make_step())
        step_data = {
            "test_report_id": "sim-report",
            "name": "step",
            "step_type": TestStepType.ACTION,
            "step_path": "1",
            "status": TestStatus.IN_PROGRESS,
            "start_time": T0,
            "end_time": T0,
        }
        result = await api.create_step(step_data, log_file=LOG)
        assert result._log_file == LOG

    @pytest.mark.asyncio
    async def test_create_measurement_stamps_log_file(self, api):
        api._low_level_client.create_test_measurement = AsyncMock(return_value=_make_measurement())
        meas_data = {
            "test_step_id": "sim-step",
            "name": "m",
            "measurement_type": TestMeasurementType.BOOLEAN,
            "boolean_value": True,
            "passed": True,
            "timestamp": T0,
        }
        result = await api.create_measurement(meas_data, log_file=LOG)
        assert result._log_file == LOG


class TestUpdateStamping:
    @pytest.mark.asyncio
    async def test_update_stamps_log_file(self, api):
        existing = _make_report()
        api._low_level_client.update_test_report = AsyncMock(return_value=existing)
        result = await api.update(
            test_report=existing, update={"status": TestStatus.FAILED}, log_file=LOG
        )
        assert result._log_file == LOG
        assert api._low_level_client.update_test_report.call_args.kwargs["log_file"] == LOG

    @pytest.mark.asyncio
    async def test_update_step_stamps_log_file(self, api):
        existing = _make_step()
        api._low_level_client.update_test_step = AsyncMock(return_value=existing)
        result = await api.update_step(
            test_step=existing, update={"description": "x"}, log_file=LOG
        )
        assert result._log_file == LOG
        assert api._low_level_client.update_test_step.call_args.kwargs["log_file"] == LOG

    @pytest.mark.asyncio
    async def test_update_measurement_stamps_log_file(self, api):
        existing = _make_measurement()
        api._low_level_client.update_test_measurement = AsyncMock(return_value=existing)
        result = await api.update_measurement(
            test_measurement=existing, update={"passed": False}, log_file=LOG
        )
        assert result._log_file == LOG
        assert api._low_level_client.update_test_measurement.call_args.kwargs["log_file"] == LOG


CACHED = "/tmp/cached.jsonl"
KWARG = "/tmp/kwarg.jsonl"


class TestResourceMethodReadsStampedEntity:
    """Resource-level fallback: when no log_file kwarg is passed, read it off
    the entity. Symmetric with the entity-level convenience method's behavior.
    """

    @pytest.mark.parametrize(
        ("cached", "kwarg", "expected"),
        [
            (None, None, None),
            (CACHED, None, CACHED),  # the fallback
            (CACHED, KWARG, KWARG),  # kwarg wins
        ],
    )
    @pytest.mark.asyncio
    async def test_update_reads_log_file_from_test_report(self, api, cached, kwarg, expected):
        entity = _make_report()
        if cached is not None:
            entity.__dict__["_log_file"] = cached
        api._low_level_client.update_test_report = AsyncMock(return_value=entity)

        await api.update(test_report=entity, update={"status": TestStatus.FAILED}, log_file=kwarg)

        assert api._low_level_client.update_test_report.call_args.kwargs["log_file"] == expected

    @pytest.mark.parametrize(
        ("cached", "kwarg", "expected"),
        [
            (None, None, None),
            (CACHED, None, CACHED),
            (CACHED, KWARG, KWARG),
        ],
    )
    @pytest.mark.asyncio
    async def test_update_step_reads_log_file_from_test_step(self, api, cached, kwarg, expected):
        entity = _make_step()
        if cached is not None:
            entity.__dict__["_log_file"] = cached
        api._low_level_client.update_test_step = AsyncMock(return_value=entity)

        await api.update_step(test_step=entity, update={"description": "x"}, log_file=kwarg)

        assert api._low_level_client.update_test_step.call_args.kwargs["log_file"] == expected

    @pytest.mark.parametrize(
        ("cached", "kwarg", "expected"),
        [
            (None, None, None),
            (CACHED, None, CACHED),
            (CACHED, KWARG, KWARG),
        ],
    )
    @pytest.mark.asyncio
    async def test_update_measurement_reads_log_file_from_test_measurement(
        self, api, cached, kwarg, expected
    ):
        entity = _make_measurement()
        if cached is not None:
            entity.__dict__["_log_file"] = cached
        api._low_level_client.update_test_measurement = AsyncMock(return_value=entity)

        await api.update_measurement(
            test_measurement=entity, update={"passed": False}, log_file=kwarg
        )

        assert (
            api._low_level_client.update_test_measurement.call_args.kwargs["log_file"] == expected
        )

    @pytest.mark.asyncio
    async def test_update_with_string_id_has_no_fallback(self, api):
        """Passing a bare ID (no entity) means no _log_file to read; the resource
        forwards None to the low-level wrapper.
        """
        api._low_level_client.update_test_report = AsyncMock(return_value=_make_report())
        await api.update(test_report="some-id", update={"status": TestStatus.FAILED})
        assert api._low_level_client.update_test_report.call_args.kwargs["log_file"] is None

    @pytest.mark.asyncio
    async def test_update_step_with_string_id_has_no_fallback(self, api):
        api._low_level_client.update_test_step = AsyncMock(return_value=_make_step())
        await api.update_step(test_step="some-id", update={"description": "x"})
        assert api._low_level_client.update_test_step.call_args.kwargs["log_file"] is None


class TestReadPathsDoNotStamp:
    """get/list_/find/import_log_file return real entities; they must not carry _log_file."""

    @pytest.mark.asyncio
    async def test_get_does_not_stamp(self, api):
        api._low_level_client.get_test_report = AsyncMock(return_value=_make_report("real-id"))
        result = await api.get(test_report_id="real-id")
        assert result._log_file is None

    @pytest.mark.asyncio
    async def test_list_does_not_stamp(self, api):
        api._low_level_client.list_all_test_reports = AsyncMock(
            return_value=[_make_report("a"), _make_report("b")]
        )
        results = await api.list_()
        assert all(r._log_file is None for r in results)

    @pytest.mark.asyncio
    async def test_import_log_file_does_not_stamp(self, api, tmp_path):
        from sift_client._internal.low_level_wrappers.test_results import ReplayResult

        log_path = tmp_path / "log.jsonl"
        log_path.touch()
        replay_result = ReplayResult(
            report=_make_report("real-report"),
            steps=[_make_step("real-step")],
            measurements=[_make_measurement("real-meas")],
        )
        api._low_level_client.import_log_file = AsyncMock(return_value=replay_result)

        result = await api.import_log_file(log_path)

        assert result.report._log_file is None
        assert all(s._log_file is None for s in result.steps)
        assert all(m._log_file is None for m in result.measurements)


class TestEndToEndLogFileRouting:
    """Full pipeline: resource -> real low-level client -> actual file write.

    No mocking of the low-level client; the GrpcClient stub is mocked but is
    never invoked because the file-write branch in the low-level wrapper
    short-circuits before any gRPC call when log_file is set. Proves the
    cached-_log_file plumbing reaches the file on disk.
    """

    @pytest.fixture
    def real_api(self, mock_client):
        """TestResultsAPIAsync wired through a real TestResultsLowLevelClient."""
        return TestResultsAPIAsync(mock_client)

    @pytest.mark.asyncio
    async def test_metadata_update_round_trips_through_log_file(self, real_api, tmp_path):
        """The actual ENG-11152 regression: update with metadata via cached
        _log_file, then read the JSONL line back through the same parser the
        replay path uses and verify every key/value round-trips. Proves the
        user-visible payload (not just an opaque entry) lands on disk.
        """
        from google.protobuf import json_format
        from sift.test_reports.v1.test_reports_pb2 import UpdateTestReportRequest

        from sift_client._internal.low_level_wrappers._test_results_log import (
            iter_log_data_lines,
        )
        from sift_client.util.metadata import metadata_proto_to_dict

        log_file = tmp_path / "metadata.jsonl"
        report_data = {
            "status": TestStatus.IN_PROGRESS,
            "name": "n",
            "test_system_name": "s",
            "test_case": "c",
            "start_time": T0,
            "end_time": T0,
        }
        report = await real_api.create(report_data, log_file=log_file)
        assert report._log_file == log_file

        # Mix of string, number, and boolean to cover all three MetadataValue arms.
        metadata = {
            "run_id": "run-abc-123",
            "operator": "test-user",
            "trial_number": 42.5,
            "is_dry_run": True,
        }
        # No log_file kwarg — the resource layer must read it off the entity.
        await real_api.update(test_report=report, update={"metadata": metadata})

        # Find the UpdateTestReport line and decode it the same way replay does.
        update_entries = [
            (rt, rid, js)
            for rt, rid, js in iter_log_data_lines(log_file)
            if rt == "UpdateTestReport"
        ]
        assert len(update_entries) == 1
        _, _, json_str = update_entries[0]

        request = UpdateTestReportRequest()
        json_format.Parse(json_str, request)

        assert "metadata" in request.update_mask.paths
        round_tripped = metadata_proto_to_dict(list(request.test_report.metadata))
        assert round_tripped == metadata
        # And confirm we never reached the gRPC stub.
        real_api._low_level_client._grpc_client.get_stub.assert_not_called()
