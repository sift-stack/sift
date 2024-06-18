from ..channel import ChannelConfig, ChannelDataType
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
        channel_references={
            "$1": ChannelConfig(
                name="voltage",
                data_type=ChannelDataType.DOUBLE,
            ),
        },
    )
    assert voltage_rule_config.expression == voltage_rule_expression

    overheating_rule_expression = '$1 == "Accelerating" && $2 > $3'
    overheating_rule_config = RuleConfig(
        name="overheating",
        description="checks if vehicle overheats while accelerating",
        expression=overheating_rule_expression,
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
        channel_references={
            "$1": ChannelConfig(
                name="log",
                data_type=ChannelDataType.INT_32,
            ),
        },
        sub_expressions={
            "$2": "Error",
        },
    )
    assert contains_rule_config.expression == 'contains($1, "Error")'
