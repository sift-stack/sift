"""Pytest tests for the Exports API.

These tests validate the usage of the ExportsAPIAsync including:
- Correct delegation to the low-level client for all three export methods
- Domain object resolution (Run -> run_id, Asset -> asset_id, Channel -> channel_id)
- Job lifecycle: export methods return Job, wait_until_complete returns list of file paths
- Input validation and error handling
"""

from __future__ import annotations

from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock, patch

import pytest

from sift_client._internal.low_level_wrappers.exports import _build_calc_channel_configs
from sift_client.resources.exports import ExportsAPIAsync
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.calculated_channel import CalculatedChannelCreate, ChannelReference
from sift_client.sift_types.channel import Channel
from sift_client.sift_types.export import ExportOutputFormat
from sift_client.sift_types.job import DataExportStatusDetails, Job, JobStatus
from sift_client.sift_types.run import Run

START = datetime(2025, 1, 1, tzinfo=timezone.utc)
STOP = datetime(2025, 1, 2, tzinfo=timezone.utc)


@pytest.fixture
def mock_client():
    """Create a mock SiftClient for unit testing."""
    client = MagicMock()
    client.grpc_client = MagicMock()
    client.async_ = MagicMock()
    client.async_.jobs = MagicMock()
    return client


@pytest.fixture
def mock_job():
    """Create a mock Job returned by jobs.get."""
    job = MagicMock(spec=Job)
    job._id_or_error = "job-123"
    job.job_status = JobStatus.FINISHED
    return job


@pytest.fixture
def exports_api(mock_client, mock_job):
    """Create an ExportsAPIAsync with a mocked low-level client."""
    with patch("sift_client.resources.exports.ExportsLowLevelClient", autospec=True) as mock_ll:
        api = ExportsAPIAsync(mock_client)
        api._low_level_client = mock_ll.return_value
        # Default: low-level export_data returns a job_id
        api._low_level_client.export_data = AsyncMock(return_value="job-123")
        # Default: jobs.get returns a mock Job
        mock_client.async_.jobs.get = AsyncMock(return_value=mock_job)
        return api


@pytest.fixture
def sample_calc_channels():
    """Create sample calculated channel definitions for testing."""
    return [
        CalculatedChannelCreate(
            name="speed_doubled",
            expression="$1 * 2",
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="ch-uuid-1"),
            ],
            units="m/s",
        ),
        CalculatedChannelCreate(
            name="no_units",
            expression="$1 + $2",
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="ch-uuid-1"),
                ChannelReference(channel_reference="$2", channel_identifier="ch-uuid-2"),
            ],
        ),
    ]


class TestBuildCalcChannelConfigs:
    """Tests for the _build_calc_channel_configs helper in the low-level client."""

    def test_returns_empty_list_for_none(self):
        """Test that None input returns an empty list."""
        assert _build_calc_channel_configs(None) == []

    def test_returns_empty_list_for_empty_list(self):
        """Test that an empty list returns an empty list."""
        assert _build_calc_channel_configs([]) == []

    def test_converts_to_proto(self, sample_calc_channels):
        """Test converting CalculatedChannelCreate objects to proto CalculatedChannelConfig messages."""
        result = _build_calc_channel_configs(sample_calc_channels)
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


class TestExportsAPIAsync:
    """Tests for the ExportsAPIAsync high-level client."""

    class TestExportByRun:
        """Tests for the export_by_run method."""

        @pytest.mark.asyncio
        async def test_delegates_to_low_level_and_returns_job(self, exports_api):
            """Test that export_by_run passes correct args to low-level and returns a Job."""
            job = await exports_api.export_by_run(
                runs=["run-1", "run-2"],
                output_format=ExportOutputFormat.CSV,
                start_time=START,
                stop_time=STOP,
                channels=["ch-1"],
                simplify_channel_names=True,
                combine_runs=True,
                split_export_by_asset=True,
            )

            assert isinstance(job, MagicMock)
            exports_api._low_level_client.export_data.assert_awaited_once_with(
                run_ids=["run-1", "run-2"],
                output_format=ExportOutputFormat.CSV,
                start_time=START,
                stop_time=STOP,
                channel_ids=["ch-1"],
                calculated_channels=None,
                simplify_channel_names=True,
                combine_runs=True,
                split_export_by_asset=True,
                split_export_by_run=False,
            )

        @pytest.mark.asyncio
        async def test_minimal_args(self, exports_api):
            """Test that minimal arguments are passed correctly with defaults."""
            await exports_api.export_by_run(
                runs=["run-1"],
                output_format=ExportOutputFormat.SUN,
            )

            exports_api._low_level_client.export_data.assert_awaited_once_with(
                run_ids=["run-1"],
                output_format=ExportOutputFormat.SUN,
                start_time=None,
                stop_time=None,
                channel_ids=[],
                calculated_channels=None,
                simplify_channel_names=False,
                combine_runs=False,
                split_export_by_asset=False,
                split_export_by_run=False,
            )

        @pytest.mark.asyncio
        async def test_with_calculated_channels(self, exports_api, sample_calc_channels):
            """Test that calculated channels are passed through to the low-level client."""
            await exports_api.export_by_run(
                runs=["run-1"],
                output_format=ExportOutputFormat.CSV,
                calculated_channels=sample_calc_channels,
            )

            call_kwargs = exports_api._low_level_client.export_data.call_args.kwargs
            assert call_kwargs["calculated_channels"] == sample_calc_channels

        @pytest.mark.asyncio
        async def test_resolves_run_objects_to_ids(self, exports_api):
            """Test that Run domain objects are resolved to their IDs."""
            mock_run = MagicMock(spec=Run)
            mock_run._id_or_error = "resolved-run-id"

            await exports_api.export_by_run(
                runs=[mock_run, "raw-id"],
                output_format=ExportOutputFormat.CSV,
            )

            call_kwargs = exports_api._low_level_client.export_data.call_args.kwargs
            assert call_kwargs["run_ids"] == ["resolved-run-id", "raw-id"]

        @pytest.mark.asyncio
        async def test_resolves_channel_objects_to_ids(self, exports_api):
            """Test that Channel domain objects are resolved to their IDs."""
            mock_channel = MagicMock(spec=Channel)
            mock_channel._id_or_error = "resolved-ch-id"

            await exports_api.export_by_run(
                runs=["run-1"],
                output_format=ExportOutputFormat.CSV,
                channels=[mock_channel, "raw-ch-id"],
            )

            call_kwargs = exports_api._low_level_client.export_data.call_args.kwargs
            assert call_kwargs["channel_ids"] == ["resolved-ch-id", "raw-ch-id"]

        @pytest.mark.asyncio
        async def test_empty_runs_raises(self, exports_api):
            """Test that an empty runs list raises ValueError."""
            with pytest.raises(ValueError, match="runs"):
                await exports_api.export_by_run(runs=[], output_format=ExportOutputFormat.CSV)

        @pytest.mark.asyncio
        async def test_null_run_raises(self, exports_api):
            """Test that a runs list containing an empty string raises ValueError."""
            with pytest.raises(ValueError, match="empty or null"):
                await exports_api.export_by_run(
                    runs=["", "run-1"], output_format=ExportOutputFormat.CSV
                )

        @pytest.mark.asyncio
        async def test_start_after_stop_raises(self, exports_api):
            """Test that start_time >= stop_time raises ValueError."""
            with pytest.raises(ValueError, match="start_time"):
                await exports_api.export_by_run(
                    runs=["run-1"],
                    output_format=ExportOutputFormat.CSV,
                    start_time=STOP,
                    stop_time=START,
                )

        @pytest.mark.asyncio
        async def test_start_without_stop_raises(self, exports_api):
            """Test that providing start_time without stop_time raises ValueError."""
            with pytest.raises(ValueError, match="both be provided or both omitted"):
                await exports_api.export_by_run(
                    runs=["run-1"],
                    output_format=ExportOutputFormat.CSV,
                    start_time=START,
                )

        @pytest.mark.asyncio
        async def test_stop_without_start_raises(self, exports_api):
            """Test that providing stop_time without start_time raises ValueError."""
            with pytest.raises(ValueError, match="both be provided or both omitted"):
                await exports_api.export_by_run(
                    runs=["run-1"],
                    output_format=ExportOutputFormat.CSV,
                    stop_time=STOP,
                )

        @pytest.mark.asyncio
        async def test_combine_runs_with_split_by_run_raises(self, exports_api):
            """Test that enabling both combine_runs and split_export_by_run raises ValueError."""
            with pytest.raises(ValueError, match="combine_runs.*split_export_by_run"):
                await exports_api.export_by_run(
                    runs=["run-1"],
                    output_format=ExportOutputFormat.CSV,
                    combine_runs=True,
                    split_export_by_run=True,
                )

    class TestExportByAsset:
        """Tests for the export_by_asset method."""

        @pytest.mark.asyncio
        async def test_delegates_to_low_level_and_returns_job(self, exports_api):
            """Test that export_by_asset passes correct args to low-level and returns a Job."""
            job = await exports_api.export_by_asset(
                assets=["asset-1"],
                start_time=START,
                stop_time=STOP,
                output_format=ExportOutputFormat.CSV,
                channels=["ch-1", "ch-2"],
            )

            assert isinstance(job, MagicMock)
            exports_api._low_level_client.export_data.assert_awaited_once_with(
                asset_ids=["asset-1"],
                start_time=START,
                stop_time=STOP,
                output_format=ExportOutputFormat.CSV,
                channel_ids=["ch-1", "ch-2"],
                calculated_channels=None,
                simplify_channel_names=False,
                combine_runs=False,
                split_export_by_asset=False,
                split_export_by_run=False,
            )

        @pytest.mark.asyncio
        async def test_resolves_asset_objects_to_ids(self, exports_api):
            """Test that Asset domain objects are resolved to their IDs."""
            mock_asset = MagicMock(spec=Asset)
            mock_asset._id_or_error = "resolved-asset-id"

            await exports_api.export_by_asset(
                assets=[mock_asset, "raw-id"],
                start_time=START,
                stop_time=STOP,
                output_format=ExportOutputFormat.CSV,
            )

            call_kwargs = exports_api._low_level_client.export_data.call_args.kwargs
            assert call_kwargs["asset_ids"] == ["resolved-asset-id", "raw-id"]

        @pytest.mark.asyncio
        async def test_empty_assets_raises(self, exports_api):
            """Test that an empty assets list raises ValueError."""
            with pytest.raises(ValueError, match="assets"):
                await exports_api.export_by_asset(
                    assets=[],
                    start_time=START,
                    stop_time=STOP,
                    output_format=ExportOutputFormat.CSV,
                )

        @pytest.mark.asyncio
        async def test_null_asset_raises(self, exports_api):
            """Test that an assets list containing an empty string raises ValueError."""
            with pytest.raises(ValueError, match="empty or null"):
                await exports_api.export_by_asset(
                    assets=[""],
                    start_time=START,
                    stop_time=STOP,
                    output_format=ExportOutputFormat.CSV,
                )

        @pytest.mark.asyncio
        async def test_start_after_stop_raises(self, exports_api):
            """Test that start_time >= stop_time raises ValueError."""
            with pytest.raises(ValueError, match="start_time"):
                await exports_api.export_by_asset(
                    assets=["asset-1"],
                    start_time=STOP,
                    stop_time=START,
                    output_format=ExportOutputFormat.CSV,
                )

        @pytest.mark.asyncio
        async def test_combine_runs_with_split_by_run_raises(self, exports_api):
            """Test that enabling both combine_runs and split_export_by_run raises ValueError."""
            with pytest.raises(ValueError, match="combine_runs.*split_export_by_run"):
                await exports_api.export_by_asset(
                    assets=["asset-1"],
                    start_time=START,
                    stop_time=STOP,
                    output_format=ExportOutputFormat.CSV,
                    combine_runs=True,
                    split_export_by_run=True,
                )

    class TestExportByTimeRange:
        """Tests for the export_by_time_range method."""

        @pytest.mark.asyncio
        async def test_delegates_to_low_level_with_channels(self, exports_api):
            """Test that export_by_time_range passes correct args to low-level."""
            await exports_api.export_by_time_range(
                start_time=START,
                stop_time=STOP,
                output_format=ExportOutputFormat.SUN,
                channels=["ch-1"],
            )

            exports_api._low_level_client.export_data.assert_awaited_once_with(
                start_time=START,
                stop_time=STOP,
                output_format=ExportOutputFormat.SUN,
                channel_ids=["ch-1"],
                calculated_channels=None,
                simplify_channel_names=False,
                combine_runs=False,
                split_export_by_asset=False,
                split_export_by_run=False,
            )

        @pytest.mark.asyncio
        async def test_delegates_to_low_level_with_calc_channels(
            self, exports_api, sample_calc_channels
        ):
            """Test that calculated channels are passed through to the low-level client."""
            await exports_api.export_by_time_range(
                start_time=START,
                stop_time=STOP,
                output_format=ExportOutputFormat.CSV,
                calculated_channels=sample_calc_channels,
            )

            call_kwargs = exports_api._low_level_client.export_data.call_args.kwargs
            assert call_kwargs["calculated_channels"] == sample_calc_channels
            assert call_kwargs["channel_ids"] == []

        @pytest.mark.asyncio
        async def test_no_channels_raises(self, exports_api):
            """Test that omitting both channels and calculated_channels raises ValueError."""
            with pytest.raises(ValueError, match=r"channels.*calculated_channels"):
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
                    channels=["ch-1"],
                )

        @pytest.mark.asyncio
        async def test_combine_runs_with_split_by_run_raises(self, exports_api):
            """Test that enabling both combine_runs and split_export_by_run raises ValueError."""
            with pytest.raises(ValueError, match="combine_runs.*split_export_by_run"):
                await exports_api.export_by_time_range(
                    start_time=START,
                    stop_time=STOP,
                    output_format=ExportOutputFormat.CSV,
                    channels=["ch-1"],
                    combine_runs=True,
                    split_export_by_run=True,
                )

    class TestWaitUntilComplete:
        """Tests for the wait_until_complete method."""

        @pytest.mark.asyncio
        async def test_returns_file_paths_on_success(self, exports_api, mock_client, tmp_path):
            """Test that a finished job downloads files and returns their paths."""
            mock_job = MagicMock(spec=Job)
            mock_job._id_or_error = "job-123"

            completed_job = MagicMock(spec=Job)
            completed_job.job_status = JobStatus.FINISHED
            mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=completed_job)
            exports_api._low_level_client.get_download_url = AsyncMock(
                return_value="https://download.test/export.zip"
            )

            fake_file = tmp_path / "data.csv"
            fake_file.write_text("col1,col2\n1,2")

            mock_loop = MagicMock()
            mock_loop.run_in_executor = AsyncMock(return_value=None)

            with patch("asyncio.get_event_loop", return_value=mock_loop):
                result = await exports_api.wait_until_complete(job=mock_job, output_dir=tmp_path)

            assert result == [fake_file]
            mock_client.async_.jobs.wait_until_complete.assert_awaited_once_with(
                job="job-123", polling_interval_secs=5, timeout_secs=None
            )
            exports_api._low_level_client.get_download_url.assert_awaited_once_with(
                job_id="job-123"
            )

        @pytest.mark.asyncio
        async def test_accepts_job_id_string(self, exports_api, mock_client, tmp_path):
            """Test that a raw job_id string is accepted."""
            completed_job = MagicMock(spec=Job)
            completed_job.job_status = JobStatus.FINISHED
            mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=completed_job)
            exports_api._low_level_client.get_download_url = AsyncMock(
                return_value="https://download.test/export.zip"
            )

            fake_file = tmp_path / "data.csv"
            fake_file.write_text("col1,col2\n1,2")

            mock_loop = MagicMock()
            mock_loop.run_in_executor = AsyncMock(return_value=None)

            with patch("asyncio.get_event_loop", return_value=mock_loop):
                result = await exports_api.wait_until_complete(job="job-456", output_dir=tmp_path)

            assert result == [fake_file]
            mock_client.async_.jobs.wait_until_complete.assert_awaited_once_with(
                job="job-456", polling_interval_secs=5, timeout_secs=None
            )

        @pytest.mark.asyncio
        async def test_custom_polling_and_timeout(self, exports_api, mock_client, tmp_path):
            """Test that polling_interval_secs and timeout_secs are forwarded."""
            mock_job = MagicMock(spec=Job)
            mock_job._id_or_error = "job-123"

            completed_job = MagicMock(spec=Job)
            completed_job.job_status = JobStatus.FINISHED
            mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=completed_job)
            exports_api._low_level_client.get_download_url = AsyncMock(
                return_value="https://download.test/export.zip"
            )

            mock_loop = MagicMock()
            mock_loop.run_in_executor = AsyncMock(return_value=None)

            with patch("asyncio.get_event_loop", return_value=mock_loop):
                await exports_api.wait_until_complete(
                    job=mock_job, polling_interval_secs=1, timeout_secs=10, output_dir=tmp_path
                )

            mock_client.async_.jobs.wait_until_complete.assert_awaited_once_with(
                job="job-123", polling_interval_secs=1, timeout_secs=10
            )

        @pytest.mark.asyncio
        async def test_failed_job_raises_with_reason(self, exports_api, mock_client):
            """Test that a failed job raises RuntimeError with the error message."""
            mock_job = MagicMock(spec=Job)
            mock_job._id_or_error = "job-fail"

            completed_job = MagicMock(spec=Job)
            completed_job.job_status = JobStatus.FAILED
            completed_job.job_status_details = DataExportStatusDetails(
                error_message="out of memory"
            )
            mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=completed_job)

            with pytest.raises(RuntimeError, match=r"failed.*out of memory"):
                await exports_api.wait_until_complete(job=mock_job)

        @pytest.mark.asyncio
        async def test_failed_job_raises_without_reason(self, exports_api, mock_client):
            """Test that a failed job with no status details still raises RuntimeError."""
            mock_job = MagicMock(spec=Job)
            mock_job._id_or_error = "job-fail"

            completed_job = MagicMock(spec=Job)
            completed_job.job_status = JobStatus.FAILED
            completed_job.job_status_details = None
            mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=completed_job)

            with pytest.raises(RuntimeError, match="failed"):
                await exports_api.wait_until_complete(job=mock_job)

        @pytest.mark.asyncio
        async def test_cancelled_job_raises(self, exports_api, mock_client):
            """Test that a cancelled job raises RuntimeError."""
            mock_job = MagicMock(spec=Job)
            mock_job._id_or_error = "job-cancel"

            completed_job = MagicMock(spec=Job)
            completed_job.job_status = JobStatus.CANCELLED
            mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=completed_job)

            with pytest.raises(RuntimeError, match="cancelled"):
                await exports_api.wait_until_complete(job=mock_job)
