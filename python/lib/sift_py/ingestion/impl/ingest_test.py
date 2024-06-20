from __future__ import annotations

from datetime import datetime, timezone
from typing import Callable

import pytest
from pytest_mock import MockFixture
from sift.ingestion_configs.v1.ingestion_configs_pb2 import IngestionConfig as IngestionConfigPb
from sift_internal.test_util.channel import MockChannel
from sift_py.ingestion.channel import (
    ChannelConfig,
    ChannelDataType,
    double_value,
    int32_value,
    string_value,
)
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import FlowConfig
from sift_py.ingestion.impl.error import IngestionValidationError
from sift_py.ingestion.impl.ingest import (
    IngestionServiceImpl,
    create_flow_configs,
    get_ingestion_config_flow_names,
)
from sift_py.ingestion.impl.ingestion_config import (
    create_ingestion_config,
    get_ingestion_config_by_client_key,
)
from sift_py.ingestion.impl.rule import get_asset_rules_json, update_rules
from sift_py.ingestion.rule.config import RuleActionCreateDataReviewAnnotation, RuleConfig

SUBJECT_MODULE = "sift_py.ingestion.impl.ingest"


def _mock_path(fn: Callable) -> str:
    return f"{SUBJECT_MODULE}.{fn.__name__}"


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

    flows_loaded_from_config = [flow_a, flow_b]

    flow_names_queried_from_api = ["flow_a"]

    mock_get_ingestion_config_flow_names = mocker.patch(_mock_path(get_ingestion_config_flow_names))
    mock_get_ingestion_config_flow_names.return_value = flow_names_queried_from_api

    mock_create_flow_configs = mocker.patch(_mock_path(create_flow_configs))
    mock_create_flow_configs.return_value = None

    mock_channel = MockChannel()
    IngestionServiceImpl.update_flow_configs(
        mock_channel, ingestion_config_id, flows_loaded_from_config
    )
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

    ingestion_config = IngestionServiceImpl.get_or_create_ingestion_config(
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

    ingestion_config = IngestionServiceImpl.get_or_create_ingestion_config(
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
        IngestionServiceImpl, "get_or_create_ingestion_config"
    )
    mock_get_or_create_ingestion_config.return_value = mock_ingestion_config

    mock_update_flow_configs = mocker.patch.object(IngestionServiceImpl, "update_flow_configs")
    mock_update_flow_configs.return_value = None

    mock_update_rules = mocker.patch(_mock_path(update_rules))
    mock_update_rules.return_value = None

    transport_channel = MockChannel()

    svc = IngestionServiceImpl(
        channel=transport_channel,
        config=telemetry_config,
        overwrite_rules=True,
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


def test_ingestion_service_init_ensures_rules_synchonized(mocker: MockFixture):
    """
    Ensures that rules in Sift match rules in config, otherwise an exception is
    raised asking user to update their local config. Also test `overwrite_rules`
    which will ignore the difference and replace all rules in Sift with what's
    in the config
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
        channel_references={
            "$1": voltage_channel,
        },
        action=RuleActionCreateDataReviewAnnotation(
            assignee="bob@example.com",
            tags=["motor"],
        ),
    )

    rule_on_pressure = RuleConfig(
        name="pressure_rule",
        description="",
        expression="$1 > 10",
        channel_references={
            "$1": pressure_channel,
        },
        action=RuleActionCreateDataReviewAnnotation(
            assignee="bob@example.com",
            tags=["barometer"],
        ),
    )

    # This rule won't be in the config. It will be "returned" by the API.
    rule_on_logs = RuleConfig(
        name="log_rule",
        description="",
        expression='contains($1, "ERROR")',
        channel_references={
            "$1": logs_channel,
        },
        action=RuleActionCreateDataReviewAnnotation(
            assignee="bob@example.com",
            tags=["log"],
        ),
    )

    mock_ingestion_config = IngestionConfigPb(
        ingestion_config_id="my-ingestion-config",
        asset_id="my-asset-id",
        client_key="my-client-key",
    )

    mock_get_or_create_ingestion_config = mocker.patch.object(
        IngestionServiceImpl, "get_or_create_ingestion_config"
    )
    mock_get_or_create_ingestion_config.return_value = mock_ingestion_config

    mock_update_flow_configs = mocker.patch.object(IngestionServiceImpl, "update_flow_configs")
    mock_update_flow_configs.return_value = None

    mock_get_asset_rules_json = mocker.patch(_mock_path(get_asset_rules_json))

    mock_get_asset_rules_json.return_value = [
        rule_on_logs.as_json(),
        rule_on_pressure.as_json(),
        rule_on_voltage.as_json(),
    ]

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

    with pytest.raises(Exception, match="not found in local"):
        _ = IngestionServiceImpl(
            channel=mock_channel,
            config=telemetry_config,
        )

    # Now we make sure that we can overwrite rules
    mock_update_rules = mocker.patch(_mock_path(update_rules))
    mock_update_rules.return_value = None

    _ = IngestionServiceImpl(
        channel=mock_channel,
        config=telemetry_config,
        overwrite_rules=True,
    )

    mock_update_rules.assert_called_once_with(
        mock_channel,
        mock_ingestion_config.asset_id,
        telemetry_config.rules,
    )
