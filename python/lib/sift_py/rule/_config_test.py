import pytest

from sift_py import rule
from sift_py.error import SiftAPIDeprecationWarning
from sift_py.ingestion.channel import ChannelConfig, ChannelDataType

from .config import (
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleConfig,
)


def test_rule_config_json():
    voltage_rule_expression = "$1 > 10"
    voltage_rule_config = RuleConfig(
        name="High Voltage",
        description="Rock & Roll",
        expression=voltage_rule_expression,
        action=RuleActionCreatePhaseAnnotation(),
        channel_references=[
            {
                "channel_reference": "$1",
                "channel_config": ChannelConfig(
                    name="voltage",
                    data_type=ChannelDataType.DOUBLE,
                ),
            }
        ],
        contextual_channels=["temperature"],
    )
    assert voltage_rule_config.expression == voltage_rule_expression
    assert len(voltage_rule_config.contextual_channels) == 1
    assert voltage_rule_config.contextual_channels[0] == "temperature"

    overheating_rule_expression = '$1 == "Accelerating" && $2 > $3'

    with pytest.warns(SiftAPIDeprecationWarning, match="component"):
        channel_with_component1 = ChannelConfig(
            name="vehicle_state",
            component="motor",
            data_type=ChannelDataType.INT_32,
        )
        channel_with_component2 = ChannelConfig(
            name="temperature",
            component="motor",
            data_type=ChannelDataType.INT_32,
        )

    overheating_rule_config = RuleConfig(
        name="overheating",
        description="checks if vehicle overheats while accelerating",
        expression=overheating_rule_expression,
        action=RuleActionCreateDataReviewAnnotation(
            tags=["foo", "bar"],
            assignee="foobar@baz.com",
        ),
        channel_references=[
            {
                "channel_reference": "$1",
                "channel_config": channel_with_component1,
            },
            {
                "channel_reference": "$2",
                "channel_config": channel_with_component2,
            },
        ],
        sub_expressions={
            "$3": 80,
        },
    )
    assert overheating_rule_config.expression == '$1 == "Accelerating" && $2 > 80'

    contains_rule_expression = "contains($1, $2)"
    contains_rule_config = RuleConfig(
        name="contains",
        description="checks if vehicle overheats while accelerating",
        expression=contains_rule_expression,
        action=RuleActionCreateDataReviewAnnotation(
            tags=["foo", "bar"],
            assignee="foobar@baz.com",
        ),
        channel_references=[
            {
                "channel_reference": "$1",
                "channel_config": ChannelConfig(
                    name="log",
                    data_type=ChannelDataType.INT_32,
                ),
            },
        ],
        sub_expressions={
            "$2": "Error",
        },
    )
    assert contains_rule_config.expression == 'contains($1, "Error")'


def test_rule_named_expressions():
    kinetic_energy_gt_expression = "0.5 * $mass * $1 * $1 > $threshold"

    rule_on_kinetic_energy = RuleConfig(
        name="rule_onkinetic_energy",
        description="checks high periods of energy output",
        expression=kinetic_energy_gt_expression,
        action=RuleActionCreatePhaseAnnotation(),
        channel_references=[
            {
                "channel_reference": "$1",
                "channel_config": ChannelConfig(
                    name="velocity",
                    data_type=ChannelDataType.INT_32,
                ),
            },
        ],
        sub_expressions={
            "$mass": 10,
            "$threshold": 35,
        },
    )
    assert rule_on_kinetic_energy.expression == "0.5 * 10 * $1 * $1 > 35"


def test_rule_config_with_contextual_channels():
    """Test that RuleConfig properly handles contextual channels"""
    rule_config = RuleConfig(
        name="test_rule",
        description="test rule with contextual channels",
        expression="$1 > 10",
        action=RuleActionCreatePhaseAnnotation(),
        channel_references=[
            {
                "channel_reference": "$1",
                "channel_config": ChannelConfig(
                    name="temperature",
                    data_type=ChannelDataType.DOUBLE,
                ),
            }
        ],
        contextual_channels=["humidity", "pressure"],
    )

    assert len(rule_config.contextual_channels) == 2
    assert rule_config.contextual_channels[0] == "humidity"
    assert rule_config.contextual_channels[1] == "pressure"

    # Test JSON output includes contextual channels
    json_output = rule_config.as_json()
    assert "contextual_channel_references" in json_output
    assert len(json_output["contextual_channel_references"]) == 2


def test_rule_config_with_hardcoded_channel_references():
    """Test that RuleConfig properly handles hard coded channel references"""
    rule_config = RuleConfig(
        name="test_rule",
        expression='|channel_1| > 10 && |channel_2| == "Hello" || |channel_1| < 10',
        action=RuleActionCreatePhaseAnnotation(),
    )
    expected_channel_references = [
        {'channel_reference': '$1', 'channel_identifier': 'channel_1'},
        {'channel_reference': '$2', 'channel_identifier': 'channel_2'}
    ]
    expected_expression = '$1 > 10 && $2 == "Hello" || $1 < 10'
    assert rule_config.channel_references == expected_channel_references
    assert rule_config.expression == expected_expression


def test_rule_config_with_hardcoded_channel_references_with_spaces():
    """Test that RuleConfig properly handles hard coded channel references"""
    rule_config = RuleConfig(
        name="test_rule",
        expression="|channel with spaces 1| > 10 && |  two leading spaces| == 2 && |two trailing spaces  | == 3",
        action=RuleActionCreatePhaseAnnotation(),
    )
    expected_channel_references = [
        {'channel_reference': '$1', 'channel_identifier': 'channel with spaces 1'},
        {'channel_reference': '$2', 'channel_identifier': '  two leading spaces'},
        {'channel_reference': '$3', 'channel_identifier': 'two trailing spaces  '},
    ]
    expected_expression = '$1 > 10 && $2 == 2 && $3 == 3'
    assert rule_config.channel_references == expected_channel_references
    assert rule_config.expression == expected_expression


def test_rule_config_with_invalid_hardcoded_channel_references():
    """Test that RuleConfig properly handles hard coded channel references"""
    with pytest.raises(ValueError, match="No channel references or hard coded channel names found"):
        RuleConfig(
            name="test_rule",
            expression='$1 > 10',
            action=RuleActionCreatePhaseAnnotation(),
        )

    with pytest.raises(ValueError, match="No channel references or hard coded channel names found"):
        RuleConfig(
            name="test_rule",
            expression='| | > 10',
            action=RuleActionCreatePhaseAnnotation(),
        )

    with pytest.raises(ValueError, match="channel_references and hardcoded channel names defined in rule test_rule"):
        RuleConfig(
            name="test_rule",
            expression='$1 > 10 || |channel_1| == 2',
            action=RuleActionCreatePhaseAnnotation(),
            channel_references=[
                {
                    "channel_reference": "$1",
                    "channel_config": ChannelConfig(
                        name="temperature",
                        data_type=ChannelDataType.DOUBLE,
                    ),
                }
            ],
        )


