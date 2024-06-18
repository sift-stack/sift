from __future__ import annotations

from typing import cast

import pytest
import yaml

from ..channel import ChannelDataType
from ..rule.config import (
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleActionKind,
)
from .yaml import (
    NamedExpressionsYamlSpec,
    YamlConfigError,
    _try_from_yaml_str,
)


def test_telemetry_config():
    telemetry_config = _try_from_yaml_str(
        TELEMETRY_CONFIG, {"named_expressions": [TEST_NAMED_EXPRESSIONS_YAML_STR]}
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
    assert velocity_channel.name == "velocity"
    assert velocity_channel.data_type == ChannelDataType.DOUBLE
    assert velocity_channel.unit == "Miles Per Hour"
    assert velocity_channel.component == "mainmotor"
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

    assert len(telemetry_config.rules) == 3

    overheating_rule, speeding_rule, failures_rule = telemetry_config.rules

    assert overheating_rule.name == "overheating"
    assert overheating_rule.description == "Checks for vehicle overheating"
    assert overheating_rule.expression == '$1 == "Accelerating" && $2 > 80'
    assert overheating_rule.action.kind() == RuleActionKind.ANNOTATION
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


def test_no_duplicate_channels_telemetry_config():
    """
    Raise an error if there are duplicate channels in a flow.
    """
    with pytest.raises(YamlConfigError):
        _ = _try_from_yaml_str(DUPLICATE_CHANNEL_IN_FLOW_TELEMETRY_CONFIG)


def test_named_expressions():
    named_expressions = cast(
        NamedExpressionsYamlSpec, yaml.safe_load(TEST_NAMED_EXPRESSIONS_YAML_STR)
    )

    log_substring_contains = named_expressions.get("log_substring_contains")
    assert log_substring_contains is not None
    assert log_substring_contains == "contains($1, $2)"

    is_even = named_expressions.get("is_even")
    assert is_even is not None
    assert is_even == "mod($1, 2) == 0"


TELEMETRY_CONFIG = """
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
      - $2: ERROR
    tags:
        - foo
        - bar
        - baz

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

TEST_NAMED_EXPRESSIONS_YAML_STR = """
log_substring_contains:
  contains($1, $2)
is_even:
  mod($1, 2) == 0
"""
