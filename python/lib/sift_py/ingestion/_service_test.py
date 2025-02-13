import random
from contextlib import contextmanager
from datetime import datetime, timezone
from time import sleep
from typing import Callable, List

import pytest
from pytest_mock import MockFixture
from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataStreamRequest
from sift.ingestion_configs.v2.ingestion_configs_pb2 import FlowConfig as FlowConfigPb
from sift.ingestion_configs.v2.ingestion_configs_pb2 import IngestionConfig as IngestionConfigPb

import sift_py.ingestion._internal.ingest
from sift_py._internal.test_util.channel import MockChannel
from sift_py._internal.test_util.fn import _mock_path as _mock_path_imp
from sift_py.error import SiftAPIDeprecationWarning
from sift_py.ingestion._internal.error import IngestionValidationError
from sift_py.ingestion._internal.ingestion_config import (
    create_flow_configs,
    create_ingestion_config,
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

    with mock_ctx_manager():
        with ingestion_service.buffered_ingestion() as buffered_ingestion:
            for _ in range(6_600):
                buffered_ingestion.ingest_flows(
                    {
                        "flow_name": "readings",
                        "timestamp": datetime.now(timezone.utc),
                        "channel_values": [double_value(random.random())],
                    }
                )

            assert mock_ingest.call_count == 6
            assert len(buffered_ingestion._buffer) == 600

            with pytest.raises(Exception):
                raise

        assert len(buffered_ingestion._buffer) == 0
        assert mock_ingest.call_count == 7

    with mock_ctx_manager():
        on_error_spy = mocker.stub()

        def on_error(
            err: BaseException, requests: List[IngestWithConfigDataStreamRequest], _flush: Callable
        ):
            on_error_spy()
            pass

        with pytest.raises(Exception):
            with ingestion_service.buffered_ingestion(on_error=on_error) as buffered_ingestion:
                for _ in range(6_600):
                    buffered_ingestion.ingest_flows(
                        {
                            "flow_name": "readings",
                            "timestamp": datetime.now(timezone.utc),
                            "channel_values": [double_value(random.random())],
                        }
                    )
                raise

        on_error_spy.assert_called_once()
        assert len(buffered_ingestion._buffer) == 600
        assert mock_ingest.call_count == 6

    with mock_ctx_manager():
        on_error_flush_spy = mocker.stub()

        def on_error(
            err: BaseException, requests: List[IngestWithConfigDataStreamRequest], _flush: Callable
        ):
            on_error_flush_spy()
            _flush()
            pass

        with pytest.raises(Exception):
            with ingestion_service.buffered_ingestion(on_error=on_error) as buffered_ingestion:
                for _ in range(6_600):
                    buffered_ingestion.ingest_flows(
                        {
                            "flow_name": "readings",
                            "timestamp": datetime.now(timezone.utc),
                            "channel_values": [double_value(random.random())],
                        }
                    )
                raise

        on_error_spy.assert_called_once()
        assert len(buffered_ingestion._buffer) == 0
        assert mock_ingest.call_count == 7


def test_ingestion_service_modify_existing_channel_configs(mocker: MockFixture):
    """
    Tests modifying existing channel configs in telemetry config. If a channel config
    is modified in a telemetry config after it has already been used for ingestion
    then we should create a new flow. If a user modifies a channel back to a previous
    version (same component and name), then we should re-use an existing channel.
    """

    mock_ingestion_config = IngestionConfigPb(
        ingestion_config_id="my-ingestion-config-id",
        client_key="my-ingestion-config",
        asset_id="my-asset-id",
    )

    with pytest.warns(SiftAPIDeprecationWarning, match="component"):
        channel_a = ChannelConfig(
            name="channel_a",
            component="A",
            data_type=ChannelDataType.DOUBLE,
        )

    flow_a = FlowConfig(
        name="flow_a",
        channels=[channel_a],
    )

    telemetry_config = TelemetryConfig(
        asset_name="my-asset-name",
        ingestion_client_key=mock_ingestion_config.ingestion_config_id,
        flows=[flow_a],
    )

    mock_get_ingestion_config_by_client_key = mocker.patch(
        _mock_path(get_ingestion_config_by_client_key)
    )
    mock_get_ingestion_config_by_client_key.return_value = None

    mock_create_ingestion_config = mocker.patch(_mock_path(create_ingestion_config))
    mock_create_ingestion_config.return_value = mock_ingestion_config

    mock_get_ingestion_config_flows = mocker.patch(_mock_path(get_ingestion_config_flows))
    mock_get_ingestion_config_flows.return_value = [flow_a.as_pb(FlowConfigPb)]

    mock_channel = MockChannel()

    ingestion_service = IngestionService(
        channel=mock_channel,
        config=telemetry_config,
    )

    mock_create_ingestion_config.assert_called_once_with(
        mock_channel,
        telemetry_config.asset_name,
        telemetry_config.flows,
        telemetry_config.ingestion_client_key,
        None,
    )
    assert ingestion_service.flow_configs_by_name[flow_a.name].channels[0] == channel_a

    # Modify an existing channel but don't modify flow
    channel_a.data_type = ChannelDataType.STRING

    mock_create_flow_configs = mocker.patch(_mock_path(create_flow_configs))
    mock_create_flow_configs.return_value = None

    mock_get_ingestion_config_by_client_key.reset_mock()
    mock_get_ingestion_config_by_client_key.return_value = mock_ingestion_config

    # Re-initialize ingestion service
    ingestion_service = IngestionService(
        channel=mock_channel,
        config=telemetry_config,
    )

    # Assert that we are trying to create a new flow with the same name as `flow_a`
    # but with a new channel.
    mock_create_flow_configs.assert_called_once_with(
        mock_channel, mock_ingestion_config.ingestion_config_id, [flow_a]
    )
    assert ingestion_service.flow_configs_by_name[flow_a.name].channels[0] == channel_a

    # Okay now what happens if someone were to change the channel config back to the original..

    # Modify back to original
    channel_a.data_type = ChannelDataType.DOUBLE

    mock_create_flow_configs.reset_mock()

    # Re-initialize ingestion service
    ingestion_service = IngestionService(
        channel=mock_channel,
        config=telemetry_config,
    )

    # We shouldn't be creating a new flow, should re-use an existing flow.
    mock_create_flow_configs.assert_not_called()
    assert ingestion_service.flow_configs_by_name[flow_a.name].channels[0] == channel_a


def test_ingestion_service_register_new_flow(mocker: MockFixture):
    mock_ingestion_config = IngestionConfigPb(
        ingestion_config_id="my-ingestion-config-id",
        client_key="my-ingestion-config",
        asset_id="my-asset-id",
    )

    with pytest.warns(SiftAPIDeprecationWarning, match="component"):
        channel_a = ChannelConfig(
            name="channel_a",
            component="A",
            data_type=ChannelDataType.DOUBLE,
        )

    flow_a = FlowConfig(
        name="flow_a",
        channels=[channel_a],
    )

    telemetry_config = TelemetryConfig(
        asset_name="my-asset-name",
        ingestion_client_key=mock_ingestion_config.ingestion_config_id,
        flows=[flow_a],
    )

    mock_get_ingestion_config_by_client_key = mocker.patch(
        _mock_path(get_ingestion_config_by_client_key)
    )
    mock_get_ingestion_config_by_client_key.return_value = None

    mock_create_ingestion_config = mocker.patch(_mock_path(create_ingestion_config))
    mock_create_ingestion_config.return_value = mock_ingestion_config

    mock_get_ingestion_config_flows = mocker.patch(_mock_path(get_ingestion_config_flows))
    mock_get_ingestion_config_flows.return_value = [flow_a.as_pb(FlowConfigPb)]

    mock_channel = MockChannel()

    ingestion_service = IngestionService(
        channel=mock_channel,
        config=telemetry_config,
    )

    new_flow_config = FlowConfig(
        name="my_new_flow", channels=[ChannelConfig("new_channel", ChannelDataType.DOUBLE)]
    )

    mock_create_flow_configs = mocker.patch(_mock_path(create_flow_configs))
    mock_create_flow_configs.return_value = None

    assert ingestion_service.flow_configs_by_name.get("my_new_flow") is None

    ingestion_service.try_create_flow(new_flow_config)

    mock_create_flow_configs.assert_called_once_with(
        mock_channel, mock_ingestion_config.ingestion_config_id, [new_flow_config]
    )
    assert ingestion_service.flow_configs_by_name["my_new_flow"] == new_flow_config

    # Test the name collision
    new_flow_config_name_collision = FlowConfig(
        name="my_new_flow", channels=[ChannelConfig("foobar", ChannelDataType.DOUBLE)]
    )

    with pytest.raises(IngestionValidationError):
        ingestion_service.try_create_flow(new_flow_config_name_collision)

    # Bypass the validation
    ingestion_service.create_flow(new_flow_config_name_collision)
    assert ingestion_service.flow_configs_by_name["my_new_flow"] == new_flow_config_name_collision
    assert ingestion_service.flow_configs_by_name["my_new_flow"] != new_flow_config


def test_ingestion_service_buffered_ingestion_flush_timeout(mocker: MockFixture):
    """
    Test for timeout based flush mechanism in buffered ingestion. If buffer hasn't been flushed
    after a certain time then the buffer will be automatically flushed.
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
        with ingestion_service.buffered_ingestion(flush_interval_sec=2) as buffered_ingestion:
            assert buffered_ingestion._buffer_size == 1_000

            for _ in range(1_500):
                buffered_ingestion.try_ingest_flows(
                    {
                        "flow_name": "readings",
                        "timestamp": datetime.now(timezone.utc),
                        "channel_values": [
                            {"channel_name": "my-channel", "value": double_value(random.random())}
                        ],
                    }
                )
            assert mock_ingest.call_count == 1
            assert len(buffered_ingestion._buffer) == 500

            # This will cause the flush timer to flush based on provided interval
            sleep(5)
            assert mock_ingest.call_count == 2
            assert len(buffered_ingestion._buffer) == 0
