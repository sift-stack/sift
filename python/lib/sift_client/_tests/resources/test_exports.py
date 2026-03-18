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
from sift_client.sift_types.calculated_channel import (
    CalculatedChannel,
    CalculatedChannelCreate,
    ChannelReference,
)
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
    client.async_.channels = MagicMock()
    client.async_.channels.find = AsyncMock(return_value=None)
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
        api._low_level_client.export_data = AsyncMock(return_value="job-123")
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


@pytest.fixture
def mock_calculated_channel():
    """Create a mock fetched CalculatedChannel with name-based channel_identifier."""
    cc = MagicMock(spec=CalculatedChannel)
    cc.name = "my_calc"
    cc.expression = "$1 + 10"
    cc.units = "m/s"
    cc.asset_ids = ["asset-1"]
    cc.channel_references = [
        ChannelReference(channel_reference="$1", channel_identifier="sensor.velocity"),
    ]
    return cc


@pytest.fixture
def mock_resolved_channel():
    """Create a mock Channel returned by channels.find during resolution."""
    ch = MagicMock(spec=Channel)
    ch._id_or_error = "resolved-ch-uuid"
    return ch


@pytest.fixture
def completed_export_setup(exports_api, mock_client, tmp_path):
    """Set up mocks for a successful wait_until_complete call.

    Returns a dict with the exports_api, mock_client, tmp_path, and fake_file.
    """
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

    return {
        "api": exports_api,
        "client": mock_client,
        "tmp_path": tmp_path,
        "fake_file": fake_file,
        "mock_loop": mock_loop,
    }


class TestBuildCalcChannelConfigs:
    """Tests for the _build_calc_channel_configs helper in the low-level client."""

    @pytest.mark.parametrize("input_val", [None, []])
    def test_returns_empty_list_for_empty_input(self, input_val):
        """Test that None or empty list returns an empty list."""
        assert _build_calc_channel_configs(input_val) == []

    def test_converts_create_objects_to_proto(self, sample_calc_channels):
        """Test converting CalculatedChannelCreate objects to proto CalculatedChannelConfig."""
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

    def test_converts_existing_calculated_channel_to_proto(self):
        """Test converting an existing CalculatedChannel (full model) to proto.

        Exercises the else-branch that reads from 'channel_references'
        instead of 'expression_channel_references'.
        """
        mock_cc = MagicMock(spec=CalculatedChannel)
        mock_cc.name = "derived_speed"
        mock_cc.expression = "$1 / $2"
        mock_cc.channel_references = [
            ChannelReference(channel_reference="$1", channel_identifier="ch-distance"),
            ChannelReference(channel_reference="$2", channel_identifier="ch-time"),
        ]
        mock_cc.units = "m/s"

        result = _build_calc_channel_configs([mock_cc])
        assert len(result) == 1
        config = result[0]
        assert config.name == "derived_speed"
        assert config.expression == "$1 / $2"
        assert config.units == "m/s"
        assert len(config.channel_references) == 2
        assert config.channel_references[0].channel_identifier == "ch-distance"
        assert config.channel_references[1].channel_identifier == "ch-time"


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
            with pytest.raises(ValueError, match="runs"):
                await exports_api.export_by_run(runs=[], output_format=ExportOutputFormat.CSV)

        @pytest.mark.asyncio
        async def test_null_run_raises(self, exports_api):
            with pytest.raises(ValueError, match="empty or null"):
                await exports_api.export_by_run(
                    runs=["", "run-1"], output_format=ExportOutputFormat.CSV
                )

        @pytest.mark.asyncio
        async def test_start_without_stop_raises(self, exports_api):
            with pytest.raises(ValueError, match="both be provided or both omitted"):
                await exports_api.export_by_run(
                    runs=["run-1"],
                    output_format=ExportOutputFormat.CSV,
                    start_time=START,
                )

        @pytest.mark.asyncio
        async def test_stop_without_start_raises(self, exports_api):
            with pytest.raises(ValueError, match="both be provided or both omitted"):
                await exports_api.export_by_run(
                    runs=["run-1"],
                    output_format=ExportOutputFormat.CSV,
                    stop_time=STOP,
                )

    class TestExportByAsset:
        """Tests for the export_by_asset method."""

        @pytest.mark.asyncio
        async def test_delegates_to_low_level_and_returns_job(self, exports_api):
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
        async def test_with_calculated_channels(self, exports_api, sample_calc_channels):
            await exports_api.export_by_asset(
                assets=["asset-1"],
                start_time=START,
                stop_time=STOP,
                output_format=ExportOutputFormat.CSV,
                calculated_channels=sample_calc_channels,
            )

            call_kwargs = exports_api._low_level_client.export_data.call_args.kwargs
            assert call_kwargs["calculated_channels"] == sample_calc_channels
            assert call_kwargs["channel_ids"] == []

        @pytest.mark.asyncio
        async def test_resolves_asset_objects_to_ids(self, exports_api):
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
            with pytest.raises(ValueError, match="assets"):
                await exports_api.export_by_asset(
                    assets=[],
                    start_time=START,
                    stop_time=STOP,
                    output_format=ExportOutputFormat.CSV,
                )

        @pytest.mark.asyncio
        async def test_null_asset_raises(self, exports_api):
            with pytest.raises(ValueError, match="empty or null"):
                await exports_api.export_by_asset(
                    assets=[""],
                    start_time=START,
                    stop_time=STOP,
                    output_format=ExportOutputFormat.CSV,
                )

    class TestExportByTimeRange:
        """Tests for the export_by_time_range method."""

        @pytest.mark.asyncio
        async def test_delegates_to_low_level_with_channels(self, exports_api):
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
            with pytest.raises(ValueError, match=r"channels.*calculated_channels"):
                await exports_api.export_by_time_range(
                    start_time=START, stop_time=STOP, output_format=ExportOutputFormat.CSV
                )

    class TestSharedValidation:
        """Validation rules shared across all three export methods."""

        @pytest.mark.asyncio
        @pytest.mark.parametrize(
            ("method", "kwargs"),
            [
                ("export_by_run", {"runs": ["r-1"], "output_format": ExportOutputFormat.CSV}),
                ("export_by_asset", {"assets": ["a-1"], "output_format": ExportOutputFormat.CSV}),
                (
                    "export_by_time_range",
                    {"output_format": ExportOutputFormat.CSV, "channels": ["ch-1"]},
                ),
            ],
        )
        async def test_start_after_stop_raises(self, exports_api, method, kwargs):
            with pytest.raises(ValueError, match="start_time"):
                await getattr(exports_api, method)(start_time=STOP, stop_time=START, **kwargs)

        @pytest.mark.asyncio
        @pytest.mark.parametrize(
            ("method", "kwargs"),
            [
                ("export_by_run", {"runs": ["r-1"], "output_format": ExportOutputFormat.CSV}),
                (
                    "export_by_asset",
                    {
                        "assets": ["a-1"],
                        "output_format": ExportOutputFormat.CSV,
                        "start_time": START,
                        "stop_time": STOP,
                    },
                ),
                (
                    "export_by_time_range",
                    {
                        "output_format": ExportOutputFormat.CSV,
                        "channels": ["ch-1"],
                        "start_time": START,
                        "stop_time": STOP,
                    },
                ),
            ],
        )
        async def test_combine_runs_with_split_by_run_raises(self, exports_api, method, kwargs):
            with pytest.raises(ValueError, match="combine_runs.*split_export_by_run"):
                await getattr(exports_api, method)(
                    combine_runs=True, split_export_by_run=True, **kwargs
                )

    class TestResolveCalculatedChannels:
        """Tests for the _resolve_calculated_channels helper."""

        @pytest.mark.asyncio
        async def test_passes_through_none(self, exports_api):
            result = await exports_api._resolve_calculated_channels(None)
            assert result is None

        @pytest.mark.asyncio
        async def test_preserves_objects_when_identifiers_not_found(
            self, exports_api, sample_calc_channels
        ):
            """channels.find returns None → identifiers assumed to be UUIDs, objects preserved."""
            result = await exports_api._resolve_calculated_channels(sample_calc_channels)
            assert result[0] == sample_calc_channels[0]
            assert result[1] == sample_calc_channels[1]

        @pytest.mark.asyncio
        async def test_resolves_fetched_calculated_channel(
            self, exports_api, mock_client, mock_calculated_channel, mock_resolved_channel
        ):
            """A fetched CalculatedChannel's name-based identifier is resolved to a UUID."""
            mock_client.async_.channels.find = AsyncMock(return_value=mock_resolved_channel)

            result = await exports_api._resolve_calculated_channels([mock_calculated_channel])

            assert len(result) == 1
            resolved = result[0]
            assert isinstance(resolved, CalculatedChannelCreate)
            assert resolved.name == "my_calc"
            assert resolved.expression == "$1 + 10"
            assert resolved.units == "m/s"
            assert resolved.expression_channel_references is not None
            assert (
                resolved.expression_channel_references[0].channel_identifier == "resolved-ch-uuid"
            )
            mock_client.async_.channels.find.assert_awaited_once_with(
                name="sensor.velocity", assets=["asset-1"]
            )

        @pytest.mark.asyncio
        async def test_keeps_identifier_when_not_found(self, exports_api, mock_calculated_channel):
            """channels.find returns None → identifier kept as-is."""
            mock_calculated_channel.channel_references = [
                ChannelReference(
                    channel_reference="$1",
                    channel_identifier="d8e64798-ad6f-41b8-b830-7e009806f365",
                ),
            ]

            result = await exports_api._resolve_calculated_channels([mock_calculated_channel])
            resolved = result[0]
            assert isinstance(resolved, CalculatedChannelCreate)
            assert resolved.expression_channel_references is not None
            assert (
                resolved.expression_channel_references[0].channel_identifier
                == "d8e64798-ad6f-41b8-b830-7e009806f365"
            )

        @pytest.mark.asyncio
        async def test_resolves_create_object_with_name_identifier(
            self, exports_api, mock_client, mock_resolved_channel
        ):
            """A CalculatedChannelCreate with a name-based identifier gets resolved."""
            mock_resolved_channel._id_or_error = "d8e64798-ad6f-41b8-b830-7e009806f365"
            mock_client.async_.channels.find = AsyncMock(return_value=mock_resolved_channel)

            inline_cc = CalculatedChannelCreate(
                name="inline_calc",
                expression="$1 + 30",
                expression_channel_references=[
                    ChannelReference(
                        channel_reference="$1", channel_identifier="DiningRoomLight.rssi"
                    ),
                ],
            )

            result = await exports_api._resolve_calculated_channels([inline_cc])

            resolved = result[0]
            assert isinstance(resolved, CalculatedChannelCreate)
            assert resolved.expression_channel_references is not None
            assert (
                resolved.expression_channel_references[0].channel_identifier
                == "d8e64798-ad6f-41b8-b830-7e009806f365"
            )
            mock_client.async_.channels.find.assert_awaited_once_with(
                name="DiningRoomLight.rssi", assets=None
            )

        @pytest.mark.asyncio
        async def test_mixed_create_and_existing(
            self,
            exports_api,
            mock_client,
            sample_calc_channels,
            mock_calculated_channel,
            mock_resolved_channel,
        ):
            """Mix of CalculatedChannelCreate and CalculatedChannel resolves only names."""
            mock_calculated_channel.channel_references = [
                ChannelReference(channel_reference="$1", channel_identifier="sensor.rpm"),
            ]
            mock_resolved_channel._id_or_error = "rpm-uuid"

            async def find_side_effect(name, assets=None):
                return mock_resolved_channel if name == "sensor.rpm" else None

            mock_client.async_.channels.find = AsyncMock(side_effect=find_side_effect)

            result = await exports_api._resolve_calculated_channels(
                [sample_calc_channels[0], mock_calculated_channel]
            )

            assert len(result) == 2
            assert result[0] == sample_calc_channels[0]
            assert isinstance(result[1], CalculatedChannelCreate)
            assert result[1].expression_channel_references[0].channel_identifier == "rpm-uuid"

    class TestWaitUntilComplete:
        """Tests for the wait_until_complete method."""

        @pytest.mark.asyncio
        async def test_returns_file_paths_on_success(self, completed_export_setup):
            s = completed_export_setup
            mock_job = MagicMock(spec=Job)
            mock_job._id_or_error = "job-123"

            with patch("asyncio.get_event_loop", return_value=s["mock_loop"]):
                result = await s["api"].wait_until_complete(job=mock_job, output_dir=s["tmp_path"])

            assert result == [s["fake_file"]]
            s["client"].async_.jobs.wait_until_complete.assert_awaited_once_with(
                job="job-123", polling_interval_secs=5, timeout_secs=None
            )
            s["api"]._low_level_client.get_download_url.assert_awaited_once_with(job_id="job-123")

        @pytest.mark.asyncio
        async def test_accepts_job_id_string(self, completed_export_setup):
            s = completed_export_setup

            with patch("asyncio.get_event_loop", return_value=s["mock_loop"]):
                result = await s["api"].wait_until_complete(job="job-456", output_dir=s["tmp_path"])

            assert result == [s["fake_file"]]
            s["client"].async_.jobs.wait_until_complete.assert_awaited_once_with(
                job="job-456", polling_interval_secs=5, timeout_secs=None
            )

        @pytest.mark.asyncio
        async def test_custom_polling_and_timeout(self, completed_export_setup):
            s = completed_export_setup
            mock_job = MagicMock(spec=Job)
            mock_job._id_or_error = "job-123"

            with patch("asyncio.get_event_loop", return_value=s["mock_loop"]):
                await s["api"].wait_until_complete(
                    job=mock_job, polling_interval_secs=1, timeout_secs=10, output_dir=s["tmp_path"]
                )

            s["client"].async_.jobs.wait_until_complete.assert_awaited_once_with(
                job="job-123", polling_interval_secs=1, timeout_secs=10
            )

        @pytest.mark.asyncio
        @pytest.mark.parametrize(
            ("status", "details", "match"),
            [
                (
                    JobStatus.FAILED,
                    DataExportStatusDetails(error_message="out of memory"),
                    r"failed.*out of memory",
                ),
                (JobStatus.FAILED, None, "failed"),
                (JobStatus.CANCELLED, None, "cancelled"),
            ],
        )
        async def test_terminal_job_status_raises(
            self, exports_api, mock_client, status, details, match
        ):
            mock_job = MagicMock(spec=Job)
            mock_job._id_or_error = "job-err"

            completed_job = MagicMock(spec=Job)
            completed_job.job_status = status
            completed_job.job_status_details = details
            mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=completed_job)

            with pytest.raises(RuntimeError, match=match):
                await exports_api.wait_until_complete(job=mock_job)
