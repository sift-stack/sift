"""Tests for the Exports API."""

from __future__ import annotations

from datetime import datetime, timedelta, timezone
from typing import TYPE_CHECKING
from unittest.mock import AsyncMock, MagicMock, patch

import pytest

from sift_client._internal.low_level_wrappers.exports import _build_calc_channel_configs
from sift_client._internal.util.channels import resolve_calculated_channels

if TYPE_CHECKING:
    from sift_client import SiftClient
from sift_client.resources import DataExportAPI
from sift_client.resources.exports import DataExportAPIAsync
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
CSV = ExportOutputFormat.CSV


@pytest.fixture
def exports_api_async(sift_client: SiftClient):
    return sift_client.async_.data_export


@pytest.fixture
def exports_api_sync(sift_client: SiftClient):
    return sift_client.data_export


@pytest.fixture
def mock_client():
    client = MagicMock()
    client.grpc_client = MagicMock()
    client.async_ = MagicMock()
    client.async_.jobs = MagicMock()
    client.async_.channels = MagicMock()
    client.async_.channels.find = AsyncMock(return_value=None)
    return client


@pytest.fixture
def mock_job():
    job = MagicMock(spec=Job)
    job._id_or_error = "job-123"
    job.job_status = JobStatus.FINISHED
    return job


@pytest.fixture
def exports_api(mock_client, mock_job):
    with patch("sift_client.resources.exports.ExportsLowLevelClient", autospec=True) as mock_ll:
        api = DataExportAPIAsync(mock_client)
        api._low_level_client = mock_ll.return_value
        api._low_level_client.export_data = AsyncMock(return_value="job-123")
        mock_client.async_.jobs.get = AsyncMock(return_value=mock_job)
        return api


@pytest.mark.integration
def test_client_binding(sift_client):
    assert isinstance(sift_client.data_export, DataExportAPI)
    assert isinstance(sift_client.async_.data_export, DataExportAPIAsync)


@pytest.mark.integration
class TestExportsIntegration:
    @pytest.mark.asyncio
    async def test_export_by_run(self, exports_api_async, nostromo_run):
        start = nostromo_run.start_time
        job = await exports_api_async.export(
            runs=[nostromo_run],
            start_time=start,
            stop_time=start + timedelta(seconds=10),
            output_format=CSV,
        )
        assert isinstance(job, Job)
        assert job.id_ is not None

    @pytest.mark.asyncio
    async def test_export_by_asset(self, exports_api_async, nostromo_asset, nostromo_run):
        start = nostromo_run.start_time
        job = await exports_api_async.export(
            assets=[nostromo_asset],
            start_time=start,
            stop_time=start + timedelta(seconds=10),
            output_format=CSV,
        )
        assert isinstance(job, Job)

    @pytest.mark.asyncio
    async def test_export_by_time_range(self, exports_api_async, sift_client, nostromo_run):
        channels = await sift_client.async_.channels.list_(limit=1)
        assert channels, "No channels available"
        start = nostromo_run.start_time
        job = await exports_api_async.export(
            start_time=start,
            stop_time=start + timedelta(seconds=10),
            channels=[channels[0]],
            output_format=CSV,
        )
        assert isinstance(job, Job)

    @pytest.mark.asyncio
    async def test_wait_and_download(self, exports_api_async, nostromo_run, tmp_path):
        start = nostromo_run.start_time
        job = await exports_api_async.export(
            runs=[nostromo_run],
            start_time=start,
            stop_time=start + timedelta(seconds=10),
            output_format=CSV,
        )
        files = await exports_api_async.wait_and_download(
            job=job, output_dir=tmp_path, timeout_secs=300
        )
        assert len(files) > 0
        assert all(f.exists() for f in files)

    def test_sync_export_by_run(self, exports_api_sync, nostromo_run):
        start = nostromo_run.start_time
        job = exports_api_sync.export(
            runs=[nostromo_run],
            start_time=start,
            stop_time=start + timedelta(seconds=10),
            output_format=CSV,
        )
        assert isinstance(job, Job)

    def test_sync_export_by_asset(self, exports_api_sync, nostromo_asset, nostromo_run):
        start = nostromo_run.start_time
        job = exports_api_sync.export(
            assets=[nostromo_asset],
            start_time=start,
            stop_time=start + timedelta(seconds=10),
            output_format=CSV,
        )
        assert isinstance(job, Job)

    def test_sync_export_by_time_range(self, exports_api_sync, sift_client, nostromo_run):
        channels = sift_client.channels.list_(limit=1)
        assert channels, "No channels available"
        start = nostromo_run.start_time
        job = exports_api_sync.export(
            start_time=start,
            stop_time=start + timedelta(seconds=10),
            channels=[channels[0]],
            output_format=CSV,
        )
        assert isinstance(job, Job)


class TestExportDelegation:
    """Verify each mode correctly delegates to the low-level client."""

    @pytest.mark.asyncio
    async def test_by_runs(self, exports_api):
        await exports_api.export(
            runs=["run-1", "run-2"],
            output_format=CSV,
            start_time=START,
            stop_time=STOP,
            channels=["ch-1"],
            simplify_channel_names=True,
            combine_runs=True,
            split_export_by_asset=True,
        )
        exports_api._low_level_client.export_data.assert_awaited_once_with(
            run_ids=["run-1", "run-2"],
            asset_ids=None,
            output_format=CSV,
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
    async def test_by_assets(self, exports_api):
        await exports_api.export(
            assets=["asset-1"],
            start_time=START,
            stop_time=STOP,
            output_format=CSV,
            channels=["ch-1", "ch-2"],
        )
        exports_api._low_level_client.export_data.assert_awaited_once_with(
            run_ids=None,
            asset_ids=["asset-1"],
            start_time=START,
            stop_time=STOP,
            output_format=CSV,
            channel_ids=["ch-1", "ch-2"],
            calculated_channels=None,
            simplify_channel_names=False,
            combine_runs=False,
            split_export_by_asset=False,
            split_export_by_run=False,
        )

    @pytest.mark.asyncio
    async def test_by_time_range(self, exports_api):
        await exports_api.export(
            start_time=START,
            stop_time=STOP,
            output_format=ExportOutputFormat.SUN,
            channels=["ch-1"],
        )
        exports_api._low_level_client.export_data.assert_awaited_once_with(
            run_ids=None,
            asset_ids=None,
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


class TestDomainObjectResolution:
    @pytest.mark.asyncio
    async def test_run_objects_to_ids(self, exports_api):
        mock_run = MagicMock(spec=Run)
        mock_run._id_or_error = "resolved-run-id"
        await exports_api.export(runs=[mock_run, "raw-id"], output_format=CSV)
        assert exports_api._low_level_client.export_data.call_args.kwargs["run_ids"] == [
            "resolved-run-id",
            "raw-id",
        ]

    @pytest.mark.asyncio
    async def test_asset_objects_to_ids(self, exports_api):
        mock_asset = MagicMock(spec=Asset)
        mock_asset._id_or_error = "resolved-asset-id"
        await exports_api.export(
            assets=[mock_asset, "raw-id"], start_time=START, stop_time=STOP, output_format=CSV
        )
        assert exports_api._low_level_client.export_data.call_args.kwargs["asset_ids"] == [
            "resolved-asset-id",
            "raw-id",
        ]

    @pytest.mark.asyncio
    async def test_channel_objects_to_ids(self, exports_api):
        mock_ch = MagicMock(spec=Channel)
        mock_ch._id_or_error = "resolved-ch-id"
        await exports_api.export(runs=["run-1"], output_format=CSV, channels=[mock_ch, "raw-ch-id"])
        assert exports_api._low_level_client.export_data.call_args.kwargs["channel_ids"] == [
            "resolved-ch-id",
            "raw-ch-id",
        ]


class TestDictConversion:
    @pytest.mark.asyncio
    async def test_calculated_channel_dict_converted(self, exports_api):
        await exports_api.export(
            runs=["run-1"],
            output_format=CSV,
            calculated_channels=[
                {
                    "name": "calc",
                    "expression": "$1 + 1",
                    "expression_channel_references": [
                        {"channel_reference": "$1", "channel_identifier": "ch-1"}
                    ],
                }
            ],
        )
        cc = exports_api._low_level_client.export_data.call_args.kwargs["calculated_channels"]
        assert cc is not None
        assert len(cc) == 1
        assert isinstance(cc[0], CalculatedChannelCreate)
        assert cc[0].name == "calc"


class TestExportValidation:
    @pytest.mark.asyncio
    async def test_runs_and_assets_raises(self, exports_api):
        with pytest.raises(ValueError, match="not both"):
            await exports_api.export(
                runs=["r"], assets=["a"], start_time=START, stop_time=STOP, output_format=CSV
            )

    @pytest.mark.asyncio
    async def test_nothing_provided_raises(self, exports_api):
        with pytest.raises(ValueError, match="At least one"):
            await exports_api.export(output_format=CSV)


class TestBuildCalcChannelConfigs:
    @pytest.mark.parametrize("input_val", [None, []])
    def test_empty_input(self, input_val):
        assert _build_calc_channel_configs(input_val) == []

    def test_create_objects(self):
        ccs = [
            CalculatedChannelCreate(
                name="speed_doubled",
                expression="$1 * 2",
                units="m/s",
                expression_channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier="ch-1")
                ],
            ),
            CalculatedChannelCreate(
                name="no_units",
                expression="$1 + $2",
                expression_channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier="ch-1"),
                    ChannelReference(channel_reference="$2", channel_identifier="ch-2"),
                ],
            ),
        ]
        result = _build_calc_channel_configs(ccs)
        assert len(result) == 2
        assert result[0].name == "speed_doubled"
        assert result[0].units == "m/s"
        assert result[1].name == "no_units"
        assert result[1].units == ""
        assert len(result[1].channel_references) == 2

    def test_existing_calculated_channel(self):
        cc = MagicMock(spec=CalculatedChannel)
        cc.name, cc.expression, cc.units = "derived", "$1 / $2", "m/s"
        cc.channel_references = [
            ChannelReference(channel_reference="$1", channel_identifier="ch-dist"),
            ChannelReference(channel_reference="$2", channel_identifier="ch-time"),
        ]
        result = _build_calc_channel_configs([cc])
        assert len(result) == 1
        assert result[0].name == "derived"
        assert [r.channel_identifier for r in result[0].channel_references] == [
            "ch-dist",
            "ch-time",
        ]


class TestResolveCalculatedChannels:
    @pytest.mark.asyncio
    async def test_none_passthrough(self):
        api = MagicMock()
        api.find = AsyncMock(return_value=None)
        assert await resolve_calculated_channels(None, channels_api=api) is None

    @pytest.mark.asyncio
    async def test_resolves_name_to_uuid(self):
        mock_ch = MagicMock(spec=Channel)
        mock_ch._id_or_error = "resolved-uuid"
        api = MagicMock()
        api.find = AsyncMock(return_value=mock_ch)

        cc = MagicMock(spec=CalculatedChannel)
        cc.name, cc.expression, cc.units = "calc", "$1 + 10", "m/s"
        cc.asset_ids = ["asset-1"]
        cc.channel_references = [
            ChannelReference(channel_reference="$1", channel_identifier="sensor.vel")
        ]

        result = await resolve_calculated_channels([cc], channels_api=api)
        assert result is not None
        assert len(result) == 1
        refs = result[0].expression_channel_references
        assert refs is not None
        assert refs[0].channel_identifier == "resolved-uuid"

    @pytest.mark.asyncio
    async def test_keeps_identifier_when_not_found(self):
        api = MagicMock()
        api.find = AsyncMock(return_value=None)
        cc = CalculatedChannelCreate(
            name="x",
            expression="$1",
            units="m",
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="ch-1")
            ],
        )
        result = await resolve_calculated_channels([cc], channels_api=api)
        assert result is not None
        assert result[0] == cc


@pytest.fixture
def download_setup(exports_api, mock_client, tmp_path):
    completed_job = MagicMock(spec=Job)
    completed_job.job_status = JobStatus.FINISHED
    mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=completed_job)
    exports_api._low_level_client.get_download_url = AsyncMock(
        return_value="https://dl.test/export.zip"
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
        "loop": mock_loop,
    }


class TestWaitAndDownload:
    @pytest.mark.asyncio
    async def test_success(self, download_setup):
        s = download_setup
        job = MagicMock(spec=Job)
        job._id_or_error = "job-123"
        with patch("asyncio.get_running_loop", return_value=s["loop"]):
            with patch("sift_client.resources.exports.extract_zip", return_value=[s["fake_file"]]):
                result = await s["api"].wait_and_download(job=job, output_dir=s["tmp_path"])
        assert result == [s["fake_file"]]
        s["client"].async_.jobs.wait_until_complete.assert_awaited_once_with(
            job="job-123", polling_interval_secs=5, timeout_secs=None
        )

    @pytest.mark.asyncio
    async def test_job_id_string(self, download_setup):
        s = download_setup
        with patch("asyncio.get_running_loop", return_value=s["loop"]):
            with patch("sift_client.resources.exports.extract_zip", return_value=[s["fake_file"]]):
                result = await s["api"].wait_and_download(job="job-456", output_dir=s["tmp_path"])
        assert result == [s["fake_file"]]

    @pytest.mark.asyncio
    async def test_custom_polling_and_timeout(self, download_setup):
        s = download_setup
        job = MagicMock(spec=Job)
        job._id_or_error = "job-123"
        with patch("asyncio.get_running_loop", return_value=s["loop"]):
            with patch("sift_client.resources.exports.extract_zip", return_value=[s["fake_file"]]):
                await s["api"].wait_and_download(
                    job=job, polling_interval_secs=1, timeout_secs=10, output_dir=s["tmp_path"]
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
    async def test_terminal_status_raises(self, exports_api, mock_client, status, details, match):
        job = MagicMock(spec=Job)
        job._id_or_error = "job-err"
        completed = MagicMock(spec=Job)
        completed.job_status = status
        completed.job_status_details = details
        mock_client.async_.jobs.wait_until_complete = AsyncMock(return_value=completed)
        with pytest.raises(RuntimeError, match=match):
            await exports_api.wait_and_download(job=job)
