import random
from contextlib import contextmanager
from datetime import datetime, timezone

from pytest_mock import MockFixture
from sift.ingestion_configs.v1.ingestion_configs_pb2 import FlowConfig as FlowConfigPb
from sift.ingestion_configs.v1.ingestion_configs_pb2 import IngestionConfig as IngestionConfigPb

import sift_py.ingestion._internal.ingest
from sift_py._internal.test_util.channel import MockChannel
from sift_py._internal.test_util.fn import _mock_path as _mock_path_imp
from sift_py.ingestion._internal.ingestion_config import (
    get_ingestion_config_by_client_key,
    get_ingestion_config_flows,
)
from sift_py.ingestion.channel import ChannelConfig, ChannelDataType, double_value
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import FlowConfig
from sift_py.ingestion.service import IngestionService

_mock_path = _mock_path_imp(sift_py.ingestion._internal.ingest)


def test_ingestion_service_buffered_ingestion(mocker: MockFixture):
    """
    Ensures that the ingestion method is being called the expected amount of times
    when using the buffered method of ingestion.
    """

    mock_ingest = mocker.patch.object(IngestionService, "ingest")
    mock_ingest.return_value = None

    readings_flow = FlowConfig(
        name="readings",
        channels=[
            ChannelConfig(
                name="my-channel",
                data_type=ChannelDataType.DOUBLE,
            ),
        ],
    )

    telemetry_config = TelemetryConfig(
        asset_name="my-asset",
        ingestion_client_key="ingestion-client-key",
        flows=[readings_flow],
    )

    mock_ingestion_config = IngestionConfigPb(
        ingestion_config_id="ingestion-config-id",
        asset_id="asset-id",
        client_key="client-key",
    )

    mock_get_ingestion_config_by_client_key = mocker.patch(
        _mock_path(get_ingestion_config_by_client_key)
    )
    mock_get_ingestion_config_by_client_key.return_value = mock_ingestion_config

    mock_get_ingestion_config_flows = mocker.patch(_mock_path(get_ingestion_config_flows))
    mock_get_ingestion_config_flows.return_value = [readings_flow.as_pb(FlowConfigPb)]

    ingestion_service = IngestionService(MockChannel(), telemetry_config)

    @contextmanager
    def mock_ctx_manager():
        yield
        mock_ingest.reset_mock()

    with mock_ctx_manager():
        with ingestion_service.buffered_ingestion() as buffered_ingestion:
            assert buffered_ingestion._buffer_size == 1_000

            for _ in range(10_000):
                buffered_ingestion.try_ingest_flows(
                    {
                        "flow_name": "readings",
                        "timestamp": datetime.now(timezone.utc),
                        "channel_values": [
                            {"channel_name": "my-channel", "value": double_value(random.random())}
                        ],
                    }
                )
            assert mock_ingest.call_count == 10
            assert len(buffered_ingestion._buffer) == 0

        # No additional buffered items so no need for an extra ingest call
        assert mock_ingest.call_count == 10

    with mock_ctx_manager():
        with ingestion_service.buffered_ingestion() as buffered_ingestion:
            assert buffered_ingestion._buffer_size == 1_000

            for _ in range(10_500):
                buffered_ingestion.try_ingest_flows(
                    {
                        "flow_name": "readings",
                        "timestamp": datetime.now(timezone.utc),
                        "channel_values": [
                            {"channel_name": "my-channel", "value": double_value(random.random())}
                        ],
                    }
                )

            assert mock_ingest.call_count == 10
            assert len(buffered_ingestion._buffer) == 500

        # Exiting the context manager should call flush one more time
        assert mock_ingest.call_count == 11

    with mock_ctx_manager():
        with ingestion_service.buffered_ingestion(500) as buffered_ingestion:
            assert buffered_ingestion._buffer_size == 500

            for _ in range(5_200):
                buffered_ingestion.try_ingest_flows(
                    {
                        "flow_name": "readings",
                        "timestamp": datetime.now(timezone.utc),
                        "channel_values": [
                            {"channel_name": "my-channel", "value": double_value(random.random())}
                        ],
                    }
                )

            assert mock_ingest.call_count == 10
            assert len(buffered_ingestion._buffer) == 200

        assert mock_ingest.call_count == 11

    with mock_ctx_manager():
        with ingestion_service.buffered_ingestion(800) as buffered_ingestion:
            for _ in range(5_200):
                buffered_ingestion.ingest_flows(
                    {
                        "flow_name": "readings",
                        "timestamp": datetime.now(timezone.utc),
                        "channel_values": [double_value(random.random())],
                    }
                )

            assert mock_ingest.call_count == 6
            assert len(buffered_ingestion._buffer) == 400

        assert mock_ingest.call_count == 7

    with mock_ctx_manager():
        with ingestion_service.buffered_ingestion() as buffered_ingestion:
            for _ in range(6_000):
                buffered_ingestion.ingest_flows(
                    {
                        "flow_name": "readings",
                        "timestamp": datetime.now(timezone.utc),
                        "channel_values": [double_value(random.random())],
                    }
                )

            assert mock_ingest.call_count == 6
            assert len(buffered_ingestion._buffer) == 0

        assert mock_ingest.call_count == 6
