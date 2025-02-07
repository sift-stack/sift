from __future__ import annotations

from datetime import datetime, timezone

import pytest
from pytest_mock import MockFixture
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    FlowConfig as FlowConfigPb,
)
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    IngestionConfig as IngestionConfigPb,
)

import sift_py.ingestion._internal.ingest
from sift_py._internal.test_util.channel import MockChannel
from sift_py._internal.test_util.fn import _mock_path as _mock_path_imp
from sift_py.error import SiftAPIDeprecationWarning
from sift_py.ingestion._internal.error import IngestionValidationError
from sift_py.ingestion._internal.ingest import (
    _IngestionServiceImpl,
)
from sift_py.ingestion._internal.ingestion_config import (
    create_flow_configs,
    create_ingestion_config,
    get_ingestion_config_by_client_key,
    get_ingestion_config_flows,
)
from sift_py.ingestion.channel import (
    ChannelConfig,
    ChannelDataType,
    double_value,
    int32_value,
    string_value,
)
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import FlowConfig
from sift_py.ingestion.rule.config import RuleActionCreateDataReviewAnnotation, RuleConfig

_mock_path = _mock_path_imp(sift_py.ingestion._internal.ingest)


def test_ingestion_service_update_flow_configs_updates_flows(mocker: MockFixture):
    """
    Tests to ensure that if a user adds a new flow to the telemetry config,
    `update_flow_configs` will ensure that it gets created.
    """
    ingestion_config_id = "ingestion-config-id"

    flow_a = FlowConfig(
        name="flow_a",
        channels=[
            ChannelConfig(
                name="channel_a",
                data_type=ChannelDataType.DOUBLE,
            ),
        ],
    )

    flow_b = FlowConfig(
        name="flow_b",
        channels=[
            ChannelConfig(
                name="channel_b",
                data_type=ChannelDataType.DOUBLE,
            ),
        ],
    )

    flows_from_api = [flow_a.as_pb(FlowConfigPb)]

    telemetry_config = TelemetryConfig(
        asset_name="my-config",
        ingestion_client_key="my-key",
        flows=[flow_a, flow_b],
    )

    mock_get_ingestion_config_flow_names = mocker.patch(_mock_path(get_ingestion_config_flows))
    mock_get_ingestion_config_flow_names.return_value = flows_from_api

    mock_create_flow_configs = mocker.patch(_mock_path(create_flow_configs))
    mock_create_flow_configs.return_value = None

    mock_channel = MockChannel()
    _IngestionServiceImpl._update_flow_configs(mock_channel, ingestion_config_id, telemetry_config)
    mock_create_flow_configs.assert_called_once_with(mock_channel, ingestion_config_id, [flow_b])


def test_ingestion_service_get_or_create_ingestion_config_retrieves_existing(mocker: MockFixture):
    """
    Ensure that if an ingestion config is queried by client key, a new one is not created.
    """

    mock_ingestion_config = IngestionConfigPb(
        ingestion_config_id="ingestion-config-id",
        asset_id="asset-id",
        client_key="client-key",
    )

    mock_telemetry_config = TelemetryConfig(
        asset_name="asset_name",
        ingestion_client_key=mock_ingestion_config.client_key,
    )

    mock_get_ingestion_config_by_client_key = mocker.patch(
        _mock_path(get_ingestion_config_by_client_key)
    )
    mock_get_ingestion_config_by_client_key.return_value = mock_ingestion_config

    mock_create_ingestion_config = mocker.patch(_mock_path(create_ingestion_config))

    mock_channel = MockChannel()

    ingestion_config = _IngestionServiceImpl._get_or_create_ingestion_config(
        mock_channel,
        mock_telemetry_config,
    )

    mock_create_ingestion_config.assert_not_called()

    assert ingestion_config.ingestion_config_id == mock_ingestion_config.ingestion_config_id


def test_ingestion_service_get_or_create_ingestion_config_create_if_not_exist(mocker: MockFixture):
    """
    Ensure that if an ingestion config does not exist for a given client key then a new
    ingestion config is created based on the telemetry config.
    """

    mock_ingestion_config = IngestionConfigPb(
        ingestion_config_id="ingestion-config-id",
        asset_id="asset-id",
        client_key="client-key",
    )

    mock_telemetry_config = TelemetryConfig(
        asset_name="asset_name",
        ingestion_client_key=mock_ingestion_config.client_key,
        organization_id="my-org-id",
    )

    mock_get_ingestion_config_by_client_key = mocker.patch(
        _mock_path(get_ingestion_config_by_client_key)
    )
    mock_get_ingestion_config_by_client_key.return_value = None

    mock_create_ingestion_config = mocker.patch(_mock_path(create_ingestion_config))
    mock_create_ingestion_config.return_value = mock_ingestion_config

    mock_channel = MockChannel()

    ingestion_config = _IngestionServiceImpl._get_or_create_ingestion_config(
        mock_channel,
        mock_telemetry_config,
    )

    mock_create_ingestion_config.assert_called_once_with(
        mock_channel,
        mock_telemetry_config.asset_name,
        [],
        mock_ingestion_config.client_key,
        mock_telemetry_config.organization_id,
    )

    assert ingestion_config.ingestion_config_id == mock_ingestion_config.ingestion_config_id


def test_ingestion_service_try_create_ingestion_request_validations(mocker: MockFixture):
    """
    Tests all the different validations that happen when trying to create an ingestion request.
    """

    with pytest.warns(SiftAPIDeprecationWarning, match="component"):
        voltage_channel = ChannelConfig(
            name="voltage",
            component="motor",
            data_type=ChannelDataType.DOUBLE,
        )
    pressure_channel = ChannelConfig(
        name="pressure",
        data_type=ChannelDataType.INT_64,
    )
    logs_channel = ChannelConfig(
        name="logs",
        data_type=ChannelDataType.STRING,
    )

    telemetry_config = TelemetryConfig(
        asset_name="my-asset",
        ingestion_client_key="my-client-key",
        flows=[
            FlowConfig(
                name="reading",
                channels=[voltage_channel, pressure_channel],
            ),
            FlowConfig(
                name="pressure",
                channels=[pressure_channel],
            ),
            FlowConfig(
                name="log",
                channels=[logs_channel],
            ),
        ],
    )

    mock_ingestion_config = IngestionConfigPb(
        ingestion_config_id="ingestion-config-id",
        asset_id="my-asset-id",
        client_key="my-client-key",
    )

    mock_get_or_create_ingestion_config = mocker.patch.object(
        _IngestionServiceImpl, "_get_or_create_ingestion_config"
    )
    mock_get_or_create_ingestion_config.return_value = mock_ingestion_config

    mock_update_flow_configs = mocker.patch.object(_IngestionServiceImpl, "_update_flow_configs")
    mock_update_flow_configs.return_value = None

    transport_channel = MockChannel()

    with mocker.patch("sift_py.ingestion._internal.ingest.RuleService"):
        svc = _IngestionServiceImpl(
            channel=transport_channel,
            config=telemetry_config,
        )

    # Non-existent flow
    with pytest.raises(IngestionValidationError, match="could not be found"):
        svc.try_create_ingestion_request(
            flow_name="lerg",  # typo
            timestamp=datetime.now(timezone.utc),
            channel_values=[
                {"channel_name": "logs", "value": string_value("foobar")},
            ],
        )

    # Duplicate values for channel
    with pytest.raises(IngestionValidationError, match="multiple values"):
        svc.try_create_ingestion_request(
            flow_name="log",
            timestamp=datetime.now(timezone.utc),
            channel_values=[
                {"channel_name": "logs", "value": string_value("foobar")},
                {"channel_name": "logs", "value": string_value("foobar")},
            ],
        )

    # Wrong channel value type
    with pytest.raises(IngestionValidationError, match="Expected value"):
        svc.try_create_ingestion_request(
            flow_name="log",
            timestamp=datetime.now(timezone.utc),
            channel_values=[
                {"channel_name": "logs", "value": int32_value(32)},
            ],
        )

    # Wrong channel for flow
    with pytest.raises(IngestionValidationError, match="Unexpected channel"):
        svc.try_create_ingestion_request(
            flow_name="log",
            timestamp=datetime.now(timezone.utc),
            channel_values=[
                {"channel_name": "voltage", "value": double_value(32)},
            ],
        )


def test_ingestion_service_init_with_rules(mocker: MockFixture):
    """
    Ensures that rules are created and updated to include the asset from the
    telemetry config when the ingestion service is initialized.
    """
    voltage_channel = ChannelConfig(
        name="voltage",
        component="motor",
        data_type=ChannelDataType.DOUBLE,
    )
    pressure_channel = ChannelConfig(
        name="pressure",
        data_type=ChannelDataType.INT_64,
    )
    logs_channel = ChannelConfig(
        name="logs",
        data_type=ChannelDataType.STRING,
    )

    rule_on_voltage = RuleConfig(
        name="voltage_rule",
        description="",
        expression="$1 > 10",
        channel_references=[
            {"channel_reference": "$1", "channel_identifier": voltage_channel.fqn()},
        ],
        action=RuleActionCreateDataReviewAnnotation(
            assignee="bob@example.com",
            tags=["motor"],
        ),
        rule_client_key="voltage-rule-key",
    )

    rule_on_pressure = RuleConfig(
        name="pressure_rule",
        description="",
        expression="$1 > 10",
        channel_references=[
            {"channel_reference": "$1", "channel_identifier": pressure_channel.fqn()},
        ],
        action=RuleActionCreateDataReviewAnnotation(
            assignee="bob@example.com",
            tags=["barometer"],
        ),
        rule_client_key="pressure-rule-key",
    )

    mock_ingestion_config = IngestionConfigPb(
        ingestion_config_id="my-ingestion-config",
        asset_id="my-asset-id",
        client_key="my-client-key",
    )

    mock_get_or_create_ingestion_config = mocker.patch.object(
        _IngestionServiceImpl, "_get_or_create_ingestion_config"
    )
    mock_get_or_create_ingestion_config.return_value = mock_ingestion_config

    mock_update_flow_configs = mocker.patch.object(_IngestionServiceImpl, "_update_flow_configs")
    mock_update_flow_configs.return_value = None

    telemetry_config = TelemetryConfig(
        asset_name="my-asset",
        ingestion_client_key=mock_ingestion_config.client_key,
        flows=[
            FlowConfig(
                name="reading",
                channels=[voltage_channel, pressure_channel],
            ),
            FlowConfig(
                name="pressure",
                channels=[pressure_channel],
            ),
            FlowConfig(
                name="log",
                channels=[logs_channel],
            ),
        ],
        rules=[rule_on_voltage, rule_on_pressure],
    )

    mock_channel = MockChannel()

    with mocker.patch("sift_py.ingestion._internal.ingest.RuleService"):
        svc = _IngestionServiceImpl(
            channel=mock_channel,
            config=telemetry_config,
        )
        for rule in svc.rules:
            assert rule.asset_names == ["my-asset"]
