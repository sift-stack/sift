"""Pytest tests for the Exports API.

These tests validate the usage of the ExportsAPIAsync including:
- Request construction for all three export methods (by run, asset, time range)
- Synchronous (presigned_url) and asynchronous (job polling) response handling
- Calculated channel config conversion to proto messages
- Input validation and error handling
"""

from __future__ import annotations

from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock, patch

import pytest
from sift.exports.v1.exports_pb2 import (
    ExportDataResponse,
)
from sift.exports.v1.exports_pb2 import (
    ExportOutputFormat as ExportOutputFormatProto,
)

from sift_client.resources.exports import ExportsAPIAsync, _build_calc_configs
from sift_client.sift_types.export import (
    ChannelReference,
    ExportCalculatedChannel,
    ExportOutputFormat,
)
from sift_client.sift_types.job import DataExportStatusDetails, Job, JobStatus


@pytest.fixture
def mock_client():
    """Create a mock SiftClient for unit testing."""
    client = MagicMock()
    client.grpc_client = MagicMock()
    client.async_ = MagicMock()
    client.async_.jobs = MagicMock()
    return client


@pytest.fixture
def exports_api(mock_client):
    """Create an ExportsAPIAsync with a mocked low-level client."""
    with patch("sift_client.resources.exports.ExportsLowLevelClient", autospec=True) as mock_ll:
        api = ExportsAPIAsync(mock_client)
        api._low_level_client = mock_ll.return_value
        return api


@pytest.fixture
def sample_calc_channels():
    """Create sample calculated channel configs for testing."""
    return [
        ExportCalculatedChannel(
            name="speed_doubled",
            expression="$1 * 2",
            channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="ch-uuid-1"),
            ],
            units="m/s",
        ),
        ExportCalculatedChannel(
            name="no_units",
            expression="$1 + $2",
            channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="ch-uuid-1"),
                ChannelReference(channel_reference="$2", channel_identifier="ch-uuid-2"),
            ],
        ),
    ]


START = datetime(2025, 1, 1, tzinfo=timezone.utc)
STOP = datetime(2025, 1, 2, tzinfo=timezone.utc)


class TestBuildCalcConfigs:
    """Tests for the _build_calc_configs helper."""

    def test_returns_none_for_none(self):
        """Test that None input returns None."""
        assert _build_calc_configs(None) is None

    def test_returns_none_for_empty_list(self):
        """Test that an empty list returns None."""
        assert _build_calc_configs([]) is None

    def test_converts_to_proto(self, sample_calc_channels):
        """Test converting Pydantic models to proto CalculatedChannelConfig messages."""
        result = _build_calc_configs(sample_calc_channels)
        assert len(result) == 2

        first = result[0]
        assert first.name == "speed_doubled"
        assert first.expression == "$1 * 2"
        assert first.units == "m/s"
        assert len(first.channel_references) == 1
        assert first.channel_references[0].channel_reference == "$1"
        assert first.channel_references[0].channel_identifier == "ch-uuid-1"

        second = result[1]
        assert second.name == "no_units"
        assert second.units == ""  # proto default for unset optional string
        assert len(second.channel_references) == 2


class TestExportByRun:
    """Tests for the export_by_run method."""

    @pytest.mark.asyncio
    async def test_builds_correct_request_and_returns_presigned_url(self, exports_api):
        """Test request construction with all parameters and synchronous presigned URL response."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(presigned_url="https://download.test/run.zip")
        )

        url = await exports_api.export_by_run(
            run_ids=["run-1", "run-2"],
            output_format=ExportOutputFormat.CSV,
            start_time=START,
            stop_time=STOP,
            channel_ids=["ch-1"],
            use_legacy_format=True,
            simplify_channel_names=True,
            combine_runs=True,
            split_export_by_asset=True,
            split_export_by_run=True,
        )

        assert url == "https://download.test/run.zip"
        req = exports_api._low_level_client.export_data.call_args.kwargs["request"]
        assert list(req.runs_and_time_range.run_ids) == ["run-1", "run-2"]
        assert req.runs_and_time_range.HasField("start_time")
        assert req.runs_and_time_range.HasField("stop_time")
        assert list(req.channel_ids) == ["ch-1"]
        assert req.output_format == ExportOutputFormatProto.EXPORT_OUTPUT_FORMAT_CSV
        assert req.export_options.use_legacy_format is True
        assert req.export_options.simplify_channel_names is True
        assert req.export_options.combine_runs is True
        assert req.export_options.split_export_by_asset is True
        assert req.export_options.split_export_by_run is True

    @pytest.mark.asyncio
    async def test_minimal_args(self, exports_api):
        """Test request construction with only required parameters."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(presigned_url="https://download.test/min.zip")
        )

        url = await exports_api.export_by_run(
            run_ids=["run-1"],
            output_format=ExportOutputFormat.SUN,
        )

        assert url == "https://download.test/min.zip"
        req = exports_api._low_level_client.export_data.call_args.kwargs["request"]
        assert list(req.runs_and_time_range.run_ids) == ["run-1"]
        assert not req.runs_and_time_range.HasField("start_time")
        assert not req.runs_and_time_range.HasField("stop_time")
        assert list(req.channel_ids) == []
        assert req.output_format == ExportOutputFormatProto.EXPORT_OUTPUT_FORMAT_SUN

    @pytest.mark.asyncio
    async def test_with_calculated_channels(self, exports_api, sample_calc_channels):
        """Test that calculated channel configs are included in the request."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(presigned_url="https://download.test/calc.zip")
        )

        await exports_api.export_by_run(
            run_ids=["run-1"],
            output_format=ExportOutputFormat.CSV,
            calculated_channel_configs=sample_calc_channels,
        )

        req = exports_api._low_level_client.export_data.call_args.kwargs["request"]
        assert len(req.calculated_channel_configs) == 2
        assert req.calculated_channel_configs[0].name == "speed_doubled"

    @pytest.mark.asyncio
    async def test_async_job_path(self, exports_api, mock_client):
        """Test that an empty presigned_url falls back to job polling and get_download_url."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(job_id="job-123")
        )

        mock_job = MagicMock(spec=Job)
        mock_job.job_status = JobStatus.FINISHED
        mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=mock_job)

        exports_api._low_level_client.get_download_url = AsyncMock(
            return_value="https://download.test/async.zip"
        )

        url = await exports_api.export_by_run(
            run_ids=["run-1"],
            output_format=ExportOutputFormat.CSV,
            polling_interval_secs=1,
            timeout_secs=10,
        )

        assert url == "https://download.test/async.zip"
        mock_client.async_.jobs.wait_until_complete.assert_awaited_once_with(
            job="job-123", polling_interval_secs=1, timeout_secs=10
        )
        exports_api._low_level_client.get_download_url.assert_awaited_once_with(job_id="job-123")

    @pytest.mark.asyncio
    async def test_async_job_failed_raises_with_reason(self, exports_api, mock_client):
        """Test that a failed job raises RuntimeError with the error message from status details."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(job_id="job-fail")
        )
        mock_job = MagicMock(spec=Job)
        mock_job.job_status = JobStatus.FAILED
        mock_job.job_status_details = DataExportStatusDetails(error_message="out of memory")
        mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=mock_job)

        with pytest.raises(RuntimeError, match=r"failed.*out of memory"):
            await exports_api.export_by_run(run_ids=["run-1"], output_format=ExportOutputFormat.CSV)

    @pytest.mark.asyncio
    async def test_async_job_failed_raises_without_reason(self, exports_api, mock_client):
        """Test that a failed job with no status details still raises RuntimeError."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(job_id="job-fail")
        )
        mock_job = MagicMock(spec=Job)
        mock_job.job_status = JobStatus.FAILED
        mock_job.job_status_details = None
        mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=mock_job)

        with pytest.raises(RuntimeError, match="failed"):
            await exports_api.export_by_run(run_ids=["run-1"], output_format=ExportOutputFormat.CSV)

    @pytest.mark.asyncio
    async def test_async_job_cancelled_raises(self, exports_api, mock_client):
        """Test that a cancelled job raises RuntimeError."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(job_id="job-cancel")
        )
        mock_job = MagicMock(spec=Job)
        mock_job.job_status = JobStatus.CANCELLED
        mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=mock_job)

        with pytest.raises(RuntimeError, match="cancelled"):
            await exports_api.export_by_run(run_ids=["run-1"], output_format=ExportOutputFormat.CSV)

    @pytest.mark.asyncio
    async def test_empty_run_ids_raises(self, exports_api):
        """Test that an empty run_ids list raises ValueError."""
        with pytest.raises(ValueError, match="run_ids"):
            await exports_api.export_by_run(run_ids=[], output_format=ExportOutputFormat.CSV)

    @pytest.mark.asyncio
    async def test_null_run_id_raises(self, exports_api):
        """Test that a run_ids list containing an empty string raises ValueError."""
        with pytest.raises(ValueError, match="empty or null"):
            await exports_api.export_by_run(
                run_ids=["", "run-1"], output_format=ExportOutputFormat.CSV
            )

    @pytest.mark.asyncio
    async def test_start_after_stop_raises(self, exports_api):
        """Test that start_time >= stop_time raises ValueError."""
        with pytest.raises(ValueError, match="start_time"):
            await exports_api.export_by_run(
                run_ids=["run-1"],
                output_format=ExportOutputFormat.CSV,
                start_time=STOP,
                stop_time=START,
            )

    @pytest.mark.asyncio
    async def test_start_without_stop_raises(self, exports_api):
        """Test that providing start_time without stop_time raises ValueError."""
        with pytest.raises(ValueError, match="both be provided or both omitted"):
            await exports_api.export_by_run(
                run_ids=["run-1"],
                output_format=ExportOutputFormat.CSV,
                start_time=START,
            )

    @pytest.mark.asyncio
    async def test_stop_without_start_raises(self, exports_api):
        """Test that providing stop_time without start_time raises ValueError."""
        with pytest.raises(ValueError, match="both be provided or both omitted"):
            await exports_api.export_by_run(
                run_ids=["run-1"],
                output_format=ExportOutputFormat.CSV,
                stop_time=STOP,
            )


class TestExportByAsset:
    """Tests for the export_by_asset method."""

    @pytest.mark.asyncio
    async def test_builds_correct_request(self, exports_api):
        """Test request construction with assets, time range, and channel IDs."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(presigned_url="https://download.test/asset.zip")
        )

        url = await exports_api.export_by_asset(
            asset_ids=["asset-1"],
            start_time=START,
            stop_time=STOP,
            output_format=ExportOutputFormat.CSV,
            channel_ids=["ch-1", "ch-2"],
        )

        assert url == "https://download.test/asset.zip"
        req = exports_api._low_level_client.export_data.call_args.kwargs["request"]
        assert list(req.assets_and_time_range.asset_ids) == ["asset-1"]
        assert req.assets_and_time_range.HasField("start_time")
        assert req.assets_and_time_range.HasField("stop_time")
        assert list(req.channel_ids) == ["ch-1", "ch-2"]
        assert req.output_format == ExportOutputFormatProto.EXPORT_OUTPUT_FORMAT_CSV

    @pytest.mark.asyncio
    async def test_empty_asset_ids_raises(self, exports_api):
        """Test that an empty asset_ids list raises ValueError."""
        with pytest.raises(ValueError, match="asset_ids"):
            await exports_api.export_by_asset(
                asset_ids=[], start_time=START, stop_time=STOP, output_format=ExportOutputFormat.CSV
            )

    @pytest.mark.asyncio
    async def test_null_asset_id_raises(self, exports_api):
        """Test that an asset_ids list containing an empty string raises ValueError."""
        with pytest.raises(ValueError, match="empty or null"):
            await exports_api.export_by_asset(
                asset_ids=[""],
                start_time=START,
                stop_time=STOP,
                output_format=ExportOutputFormat.CSV,
            )

    @pytest.mark.asyncio
    async def test_start_after_stop_raises(self, exports_api):
        """Test that start_time >= stop_time raises ValueError."""
        with pytest.raises(ValueError, match="start_time"):
            await exports_api.export_by_asset(
                asset_ids=["asset-1"],
                start_time=STOP,
                stop_time=START,
                output_format=ExportOutputFormat.CSV,
            )


class TestExportByTimeRange:
    """Tests for the export_by_time_range method."""

    @pytest.mark.asyncio
    async def test_builds_correct_request_with_channel_ids(self, exports_api):
        """Test request construction with time range and channel IDs."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(presigned_url="https://download.test/time.zip")
        )

        url = await exports_api.export_by_time_range(
            start_time=START,
            stop_time=STOP,
            output_format=ExportOutputFormat.SUN,
            channel_ids=["ch-1"],
        )

        assert url == "https://download.test/time.zip"
        req = exports_api._low_level_client.export_data.call_args.kwargs["request"]
        assert req.time_range.HasField("start_time")
        assert req.time_range.HasField("stop_time")
        assert list(req.channel_ids) == ["ch-1"]
        assert req.output_format == ExportOutputFormatProto.EXPORT_OUTPUT_FORMAT_SUN

    @pytest.mark.asyncio
    async def test_builds_correct_request_with_calc_channels(
        self, exports_api, sample_calc_channels
    ):
        """Test request construction with calculated channels instead of channel IDs."""
        exports_api._low_level_client.export_data = AsyncMock(
            return_value=ExportDataResponse(presigned_url="https://download.test/calc.zip")
        )

        await exports_api.export_by_time_range(
            start_time=START,
            stop_time=STOP,
            output_format=ExportOutputFormat.CSV,
            calculated_channel_configs=sample_calc_channels,
        )

        req = exports_api._low_level_client.export_data.call_args.kwargs["request"]
        assert len(req.calculated_channel_configs) == 2
        assert list(req.channel_ids) == []

    @pytest.mark.asyncio
    async def test_no_channels_raises(self, exports_api):
        """Test that omitting both channel_ids and calculated_channel_configs raises ValueError."""
        with pytest.raises(ValueError, match=r"channel_ids.*calculated_channel_configs"):
            await exports_api.export_by_time_range(
                start_time=START, stop_time=STOP, output_format=ExportOutputFormat.CSV
            )

    @pytest.mark.asyncio
    async def test_start_after_stop_raises(self, exports_api):
        """Test that start_time >= stop_time raises ValueError."""
        with pytest.raises(ValueError, match="start_time"):
            await exports_api.export_by_time_range(
                start_time=STOP,
                stop_time=START,
                output_format=ExportOutputFormat.CSV,
                channel_ids=["ch-1"],
            )
