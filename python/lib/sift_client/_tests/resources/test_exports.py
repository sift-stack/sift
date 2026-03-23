"""Pytest tests for the Exports API.

These tests demonstrate and validate the usage of the Data Export API including:
- Basic export operations (by run, by asset, by time range)
- Wait and download functionality
- Input validation and error handling
- Calculated channel configuration and resolution
"""

from __future__ import annotations

import asyncio
import uuid
from datetime import datetime, timedelta, timezone
from typing import TYPE_CHECKING
from unittest.mock import AsyncMock, MagicMock, patch

import pytest
import pytest_asyncio

from sift_client._internal.low_level_wrappers.exports import _build_calc_channel_configs

if TYPE_CHECKING:
    from sift_client import SiftClient
from sift_client.resources import DataExportAPI
from sift_client.resources.exports import DataExportAPIAsync
from sift_client.resources.ingestion import TracingConfig
from sift_client.sift_types.calculated_channel import (
    CalculatedChannel,
    CalculatedChannelCreate,
    ChannelReference,
)
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.export import ExportOutputFormat
from sift_client.sift_types.ingestion import ChannelConfig, FlowConfig, IngestionConfigCreate
from sift_client.sift_types.job import Job, JobStatus

START = datetime(2025, 1, 1, tzinfo=timezone.utc)
STOP = datetime(2025, 1, 2, tzinfo=timezone.utc)
CSV = ExportOutputFormat.CSV


@pytest.fixture
def exports_api_async(sift_client: SiftClient):
    """Get the async data export API instance."""
    return sift_client.async_.data_export


@pytest.fixture
def exports_api_sync(sift_client: SiftClient):
    """Get the synchronous data export API instance."""
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
    assert sift_client.data_export
    assert isinstance(sift_client.data_export, DataExportAPI)
    assert sift_client.async_.data_export
    assert isinstance(sift_client.async_.data_export, DataExportAPIAsync)


INGEST_TIMESTAMP = datetime(2025, 6, 1, tzinfo=timezone.utc)


@pytest_asyncio.fixture(scope="session")
async def ingested_export_channel(sift_client, nostromo_asset):
    """Ingest a single data point into a unique channel on the nostromo asset for export tests."""
    channel_name = f"export-test-{uuid.uuid4().hex[:8]}"

    flow_config = FlowConfig(
        name="export-test-flow",
        channels=[ChannelConfig(name=channel_name, data_type=ChannelDataType.DOUBLE)],
    )
    ingestion_config = IngestionConfigCreate(
        asset_name=nostromo_asset.name,
        flows=[flow_config],
    )

    async with await sift_client.async_.ingestion.create_ingestion_config_streaming_client(
        ingestion_config=ingestion_config,
        tracing_config=TracingConfig.disabled(),  # suppresses ./logs
    ) as stream:
        await stream.send(
            flow_config.as_flow(timestamp=INGEST_TIMESTAMP, values={channel_name: 42.0})
        )

    channel = None
    for _ in range(20):
        channel = sift_client.channels.find(name=channel_name, asset=nostromo_asset._id_or_error)
        if channel is not None:
            break
        await asyncio.sleep(0.5)
    assert channel is not None, f"Channel {channel_name} did not appear after ingest"

    yield channel

    sift_client.channels.archive([channel])


@pytest.mark.integration
class TestDataExportAPIAsync:
    """Test suite for the async Data Export API functionality."""

    class TestExport:
        """Tests for the async export method."""

        @pytest.mark.asyncio
        async def test_export_by_run(self, exports_api_async, nostromo_run):
            """Test exporting data scoped to a run."""
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
        async def test_export_by_asset(
            self, exports_api_async, nostromo_asset, ingested_export_channel
        ):
            """Test exporting data scoped to an asset with specific channels."""
            job = await exports_api_async.export(
                assets=[nostromo_asset],
                start_time=INGEST_TIMESTAMP - timedelta(seconds=1),
                stop_time=INGEST_TIMESTAMP + timedelta(seconds=1),
                channels=[ingested_export_channel],
                output_format=CSV,
            )
            assert isinstance(job, Job)

        @pytest.mark.asyncio
        async def test_export_by_time_range(self, exports_api_async, sift_client, nostromo_run):
            """Test exporting data by time range with explicit channels."""
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

    class TestWaitAndDownload:
        """Tests for the async wait_and_download method."""

        @pytest.mark.asyncio
        async def test_wait_and_download(self, exports_api_async, nostromo_run, tmp_path):
            """Test exporting data and downloading the result."""
            start = nostromo_run.start_time
            job = await exports_api_async.export(
                runs=[nostromo_run],
                start_time=start,
                stop_time=start + timedelta(seconds=10),
                output_format=CSV,
            )
            files = job.wait_and_download(output_dir=tmp_path, timeout_secs=300)
            assert len(files) > 0
            assert all(f.exists() for f in files)


@pytest.mark.integration
class TestDataExportAPISync:
    """Test suite for the synchronous Data Export API functionality.

    Only includes basic sync tests to verify sync wrappers work. No specific sync behavior
    difference tests are needed.
    """

    class TestExport:
        """Tests for the sync export method."""

        def test_export_by_run(self, exports_api_sync, nostromo_run):
            """Test synchronous export scoped to a run."""
            start = nostromo_run.start_time
            job = exports_api_sync.export(
                runs=[nostromo_run],
                start_time=start,
                stop_time=start + timedelta(seconds=10),
                output_format=CSV,
            )
            assert isinstance(job, Job)

        def test_export_by_asset(self, exports_api_sync, nostromo_asset, ingested_export_channel):
            """Test synchronous export scoped to an asset with specific channels."""
            job = exports_api_sync.export(
                assets=[nostromo_asset],
                start_time=INGEST_TIMESTAMP - timedelta(seconds=1),
                stop_time=INGEST_TIMESTAMP + timedelta(seconds=1),
                channels=[ingested_export_channel],
                output_format=CSV,
            )
            assert isinstance(job, Job)

        def test_export_by_time_range(self, exports_api_sync, sift_client, nostromo_run):
            """Test synchronous export by time range with explicit channels."""
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
