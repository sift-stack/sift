from .config import (
    RuleActionCreateDataReviewAnnotation,
    RuleConfig,
    RuleActionCreatePhaseAnnotation,
)
from ..channel import ChannelConfig, ChannelDataType


def test_rule_config_basic_expression():
    expression = "$1 > 10"
    config = RuleConfig(
        name="High Voltage",
        description="Rock & Roll",
        expression=expression,
        action=RuleActionCreatePhaseAnnotation(),
        channel_references={
            "$1": ChannelConfig(
                name="voltage",
                data_type=ChannelDataType.DOUBLE,
            ),
        },
    )
    assert config.expression == expression
    assert (
        config.as_json()
        == '{"name": "High Voltage", "description": "Rock & Roll", "expression": "$1 > 10", "expression_channel_references": [{"channel_reference": "$1", "channel_identifier": "voltage"}], "type": "phase", "tags": null}'
    )


def test_rule_config_test_sub_expression():
    expression = '$1 == "Accelerating" && $2 > $3'
    config = RuleConfig(
        name="overheating",
        description="checks if vehicle overheats while accelerating",
        expression=expression,
        action=RuleActionCreateDataReviewAnnotation(
            tags=["foo", "bar"],
            assignee="foobar@baz.com",
        ),
        channel_references={
            "$1": ChannelConfig(
                name="vehicle_state",
                component="motor",
                data_type=ChannelDataType.INT_32,
            ),
            "$2": ChannelConfig(
                name="temperature",
                component="motor",
                data_type=ChannelDataType.INT_32,
            ),
        },
        sub_expressions={
            "$3": 80,
        },
    )
    assert (
        config.as_json()
        == '{"name": "overheating", "description": "checks if vehicle overheats while accelerating", "expression": "$1 == \\"Accelerating\\" && $2 > 80", "expression_channel_references": [{"channel_reference": "$1", "channel_identifier": "motor.vehicle_state"}, {"channel_reference": "$2", "channel_identifier": "motor.temperature"}], "type": "review", "tags": ["foo", "bar"], "assignee": "foobar@baz.com"}'
    )
