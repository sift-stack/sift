from pathlib import Path
from typing import Any, Dict, cast

import pytest
import yaml
from pytest_mock import MockerFixture, MockFixture

import sift_py.ingestion.config.telemetry
import sift_py.ingestion.config.yaml.load
from sift_py._internal.test_util.fn import _mock_path as _mock_path_imp
from sift_py.error import SiftAPIDeprecationWarning
from sift_py.ingestion.channel import ChannelConfig, ChannelDataType
from sift_py.ingestion.config.telemetry import TelemetryConfig, TelemetryConfigValidationError
from sift_py.ingestion.config.yaml.load import (
    _validate_yaml,
    load_named_expression_modules,
    read_and_validate,
)
from sift_py.ingestion.flow import FlowConfig
from sift_py.ingestion.rule.config import (
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleActionKind,
    RuleConfig,
)

_mock_path = _mock_path_imp(sift_py.ingestion.config.telemetry)


def test_telemetry_config_load_from_yaml(mocker: MockFixture):
    raw_yaml_config = cast(Dict[Any, Any], yaml.safe_load(TEST_YAML_CONFIG_STR))
    yaml_config = _validate_yaml(raw_yaml_config)

    mock_read_and_validate = mocker.patch(_mock_path(read_and_validate))
    mock_read_and_validate.return_value = yaml_config

    mock_load_named_expression_modules = mocker.patch(_mock_path(load_named_expression_modules))
    mock_load_named_expression_modules.return_value = {
        "log_substring_contains": "contains($1, $substr)",
        "kinetic_energy_gt": "0.5 * $mass * $1 * $1 > $threshold",
    }

    dummy_yaml_path = Path()
    dummy_named_expr_mod_path = Path()

    with pytest.warns(SiftAPIDeprecationWarning, match="component"):
        telemetry_config = TelemetryConfig.try_from_yaml(
            dummy_yaml_path, [dummy_named_expr_mod_path]
        )

    assert telemetry_config.asset_name == "LunarVehicle426"
    assert telemetry_config.ingestion_client_key == "lunar_vehicle_426"
    assert len(telemetry_config.flows) == 3

    flow_configs = telemetry_config.flows
    assert flow_configs[0].name == "readings"
    assert flow_configs[1].name == "partial_readings"
    assert flow_configs[2].name == "logs"

    readings_flow, partial_readings_flow, logs_flow = flow_configs
    assert len(readings_flow.channels) == 4
    assert len(partial_readings_flow.channels) == 2
    assert len(logs_flow.channels) == 1

    log_channel = logs_flow.channels[0]
    assert log_channel.name == "log"
    assert log_channel.description == "asset logs"
    assert log_channel.data_type == ChannelDataType.STRING

    velocity_channel, voltage_channel, vehicle_state_channel, gpio_channel = readings_flow.channels
    assert velocity_channel.name == "mainmotor.velocity"
    assert velocity_channel.data_type == ChannelDataType.DOUBLE
    assert velocity_channel.unit == "Miles Per Hour"
    assert velocity_channel.component is None  # Deprecated, should only be None
    assert velocity_channel.description == "speed"

    assert voltage_channel.name == "voltage"
    assert voltage_channel.data_type == ChannelDataType.INT_32
    assert voltage_channel.unit == "Volts"
    assert voltage_channel.description == "voltage at the source"

    assert vehicle_state_channel.name == "vehicle_state"
    assert vehicle_state_channel.data_type == ChannelDataType.ENUM
    assert vehicle_state_channel.unit == "vehicle state"
    assert vehicle_state_channel.description == "vehicle state"
    assert len(vehicle_state_channel.enum_types) == 3
    assert vehicle_state_channel.enum_types[0].name == "Accelerating"
    assert vehicle_state_channel.enum_types[0].key == 0
    assert vehicle_state_channel.enum_types[1].name == "Decelerating"
    assert vehicle_state_channel.enum_types[1].key == 1
    assert vehicle_state_channel.enum_types[2].name == "Stopped"
    assert vehicle_state_channel.enum_types[2].key == 2

    assert gpio_channel.name == "gpio"
    assert gpio_channel.data_type == ChannelDataType.BIT_FIELD
    assert gpio_channel.unit is None
    assert gpio_channel.description == "on/off values for pins on gpio"
    assert len(gpio_channel.bit_field_elements) == 4
    assert gpio_channel.bit_field_elements[0].name == "12v"
    assert gpio_channel.bit_field_elements[0].index == 0
    assert gpio_channel.bit_field_elements[0].bit_count == 1
    assert gpio_channel.bit_field_elements[1].name == "charge"
    assert gpio_channel.bit_field_elements[1].index == 1
    assert gpio_channel.bit_field_elements[1].bit_count == 2
    assert gpio_channel.bit_field_elements[2].name == "led"
    assert gpio_channel.bit_field_elements[2].index == 3
    assert gpio_channel.bit_field_elements[2].bit_count == 4
    assert gpio_channel.bit_field_elements[3].name == "heater"
    assert gpio_channel.bit_field_elements[3].index == 7
    assert gpio_channel.bit_field_elements[3].bit_count == 1

    assert len(telemetry_config.rules) == 4

    (
        overheating_rule,
        speeding_rule,
        failures_rule,
        kinetic_energy_rule,
    ) = telemetry_config.rules

    assert overheating_rule.name == "overheating"
    assert overheating_rule.description == "Checks for vehicle overheating"
    assert overheating_rule.expression == '$1 == "Accelerating" && $2 > 80'
    assert overheating_rule.action.kind() == RuleActionKind.ANNOTATION  # type: ignore
    assert isinstance(overheating_rule.action, RuleActionCreateDataReviewAnnotation)

    assert speeding_rule.name == "speeding"
    assert speeding_rule.description == "Checks high vehicle speed"
    assert speeding_rule.expression == "$1 > 20"
    assert overheating_rule.action.kind() == RuleActionKind.ANNOTATION
    assert isinstance(speeding_rule.action, RuleActionCreatePhaseAnnotation)

    assert failures_rule.name == "failures"
    assert failures_rule.description == "Checks for failure logs"
    assert failures_rule.expression == 'contains($1, "ERROR")'
    assert overheating_rule.action.kind() == RuleActionKind.ANNOTATION
    assert isinstance(failures_rule.action, RuleActionCreateDataReviewAnnotation)

    assert kinetic_energy_rule.name == "kinetic_energy"
    assert kinetic_energy_rule.description == "Tracks high energy output while in motion"
    assert kinetic_energy_rule.expression == "0.5 * 10 * $1 * $1 > 470"
    assert overheating_rule.action.kind() == RuleActionKind.ANNOTATION
    assert isinstance(kinetic_energy_rule.action, RuleActionCreateDataReviewAnnotation)


def test_telemetry_config_err_if_duplicate_channels_in_flow(mocker: MockerFixture):
    """
    Raise an error if there are duplicate channels in a flow.
    """
    raw_yaml_config = cast(
        Dict[Any, Any], yaml.safe_load(DUPLICATE_CHANNEL_IN_FLOW_TELEMETRY_CONFIG)
    )
    yaml_config = _validate_yaml(raw_yaml_config)

    mock_read_and_validate = mocker.patch(_mock_path(read_and_validate))
    mock_read_and_validate.return_value = yaml_config

    mock_load_named_expression_modules = mocker.patch(_mock_path(load_named_expression_modules))
    mock_load_named_expression_modules.return_value = {
        "log_substring_contains": "contains($1, $substr)",
        "kinetic_energy_gt": "0.5 * $mass * $1 * $1 > $threshold",
    }

    dummy_yaml_path = Path()
    dummy_named_expr_mod_path = Path()

    with pytest.raises(TelemetryConfigValidationError, match="Can't have two identical channels"):
        _ = TelemetryConfig.try_from_yaml(dummy_yaml_path, [dummy_named_expr_mod_path])


def test_telemetry_config_named_expression_interpolation():
    pass


def test_telemetry_config_validations_duplicate_rules():
    channel = ChannelConfig(
        name="my_channel",
        data_type=ChannelDataType.DOUBLE,
    )

    rule_on_my_channel_a = RuleConfig(
        name="rule_a",
        description="",
        expression="$1 > 10",
        channel_references=[
            {"channel_reference": "$1", "channel_identifier": channel.fqn()},
        ],
        action=RuleActionCreateDataReviewAnnotation(
            assignee="bob@example.com",
            tags=["barometer"],
        ),
    )

    another_rule_on_my_channel_a = RuleConfig(
        name="rule_a",  # same name
        description="",
        expression="$1 > 11",
        channel_references=[
            {"channel_reference": "$1", "channel_identifier": channel.fqn()},
        ],
        action=RuleActionCreateDataReviewAnnotation(
            assignee="bob@example.com",
            tags=["barometer"],
        ),
    )

    with pytest.raises(TelemetryConfigValidationError, match="Can't have two rules"):
        TelemetryConfig(
            asset_name="my_asset",
            ingestion_client_key="my_asset_key",
            organization_id="my_organization_id",
            flows=[
                FlowConfig(
                    name="my_flow",
                    channels=[channel],
                )
            ],
            rules=[rule_on_my_channel_a, another_rule_on_my_channel_a],
        )


def test_telemetry_config_validations_duplicate_channels():
    channel = ChannelConfig(
        name="my_channel",
        data_type=ChannelDataType.DOUBLE,
    )

    with pytest.raises(TelemetryConfigValidationError, match="Can't have two identical channels"):
        TelemetryConfig(
            asset_name="my_asset",
            ingestion_client_key="my_asset_key",
            organization_id="my_organization_id",
            flows=[
                FlowConfig(
                    name="my_flow",
                    channels=[
                        channel,
                        channel,
                    ],
                )
            ],
        )


def test_telemetry_config_validations_flows_with_same_name():
    channel = ChannelConfig(
        name="my_channel",
        data_type=ChannelDataType.DOUBLE,
    )

    channel_b = ChannelConfig(
        name="my_other_channel",
        data_type=ChannelDataType.DOUBLE,
    )

    with pytest.raises(TelemetryConfigValidationError, match="Can't have two flows"):
        TelemetryConfig(
            asset_name="my_asset",
            ingestion_client_key="my_asset_key",
            organization_id="my_organization_id",
            flows=[
                FlowConfig(
                    name="my_flow",
                    channels=[channel],
                ),
                FlowConfig(
                    name="my_flow",
                    channels=[channel_b],
                ),
            ],
        )


# NOTE: Component is deprecated, but kept in yaml test to validate backwards compatibility
TEST_YAML_CONFIG_STR = """
asset_name: LunarVehicle426
ingestion_client_key: lunar_vehicle_426

channels:
  log_channel: &log_channel
    name: log
    data_type: string
    description: asset logs

  velocity_channel: &velocity_channel
    name: velocity
    data_type: double
    description: speed
    unit: Miles Per Hour
    component: mainmotor

  voltage_channel: &voltage_channel
    name: voltage
    data_type: int32
    description: voltage at the source
    unit: Volts

  vehicle_state_channel: &vehicle_state_channel
    name: vehicle_state
    data_type: enum
    description: vehicle state
    unit: vehicle state
    enum_types:
      - name: Accelerating
        key: 0
      - name: Decelerating
        key: 1
      - name: Stopped
        key: 2

  gpio_channel: &gpio_channel
    name: gpio
    data_type: bit_field
    description: on/off values for pins on gpio
    bit_field_elements:
      - name: 12v
        index: 0
        bit_count: 1
      - name: charge
        index: 1
        bit_count: 2
      - name: led
        index: 3
        bit_count: 4
      - name: heater
        index: 7
        bit_count: 1

rules:
  - name: overheating
    description: Checks for vehicle overheating
    expression: $1 == "Accelerating" && $2 > 80
    channel_references:
      - $1: *vehicle_state_channel
      - $2: *voltage_channel
    type: review

  - name: speeding
    description: Checks high vehicle speed
    type: phase
    expression: $1 > 20
    channel_references:
      - $1: *velocity_channel

  - name: failures
    description: Checks for failure logs
    type: review
    assignee: homer@example.com
    expression:
      name: log_substring_contains
    channel_references:
      - $1: *log_channel
    sub_expressions:
      - $substr: ERROR
    tags:
        - foo
        - bar
        - baz

  - name: kinetic_energy
    description: Tracks high energy output while in motion
    type: review
    assignee: homer@example.com
    expression:
      name: kinetic_energy_gt
    channel_references:
      - $1: *velocity_channel
    sub_expressions:
      - $mass: 10
      - $threshold: 470
    tags:
        - nostromo

flows:
  - name: readings
    channels:
      - <<: *velocity_channel
      - <<: *voltage_channel
      - <<: *vehicle_state_channel
      - <<: *gpio_channel

  - name: partial_readings
    channels:
      - <<: *velocity_channel
      - <<: *voltage_channel

  - name: logs
    channels:
      - <<: *log_channel

"""

DUPLICATE_CHANNEL_IN_FLOW_TELEMETRY_CONFIG = """
asset_name: LunarVehicle426
ingestion_client_key: lunar_vehicle_426

channels:
  velocity_channel: &velocity_channel
    name: velocity
    data_type: double
    description: speed
    unit: Miles Per Hour
    component: mainmotor

flows:
  - name: readings
    channels:
      - <<: *velocity_channel
      - <<: *velocity_channel
"""
