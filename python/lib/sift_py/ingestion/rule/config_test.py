import pytest

from sift_py.ingestion.channel import ChannelConfig, ChannelDataType
from sift_py.ingestion.config.yaml.spec import RuleYamlSpec

from .config import (
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleActionKind,
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
        channel_references=[
            {
                "channel_reference": "$1",
                "channel_config": ChannelConfig(
                    name="vehicle_state",
                    component="motor",
                    data_type=ChannelDataType.INT_32,
                ),
            },
            {
                "channel_reference": "$2",
                "channel_config": ChannelConfig(
                    name="temperature",
                    component="motor",
                    data_type=ChannelDataType.INT_32,
                ),
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


def test_rule_namespace():
    namespace_rules = {
        "valid_namespace": [
            RuleYamlSpec(
                {
                    "name": "valid_rule",
                    "description": "A rule in a namespace",
                    "expression": "$1 > 10",
                    "type": "review",
                    "assignee": "bob@example.com",
                    "tags": ["foo", "bar"],
                    "rule_client_key": "valid_rule_client_key",
                    "asset_names": ["asset1", "asset2"],
                    "tag_names": ["tag1", "tag2"],
                }
            ),
            RuleYamlSpec(
                {
                    "name": "another_valid_rule",
                    "description": "Another rule in a namespace",
                    "expression": "$1 < 10",
                    "type": "review",
                    "assignee": "mary@example.com",
                    "tags": ["baz", "qux"],
                    "rule_client_key": "another_valid_rule_client_key",
                    "asset_names": ["asset2"],
                    "tag_names": ["tag2"],
                }
            ),
        ]
    }

    valid_namespace_rule = RuleConfig(
        name="valid_rule",
        namespace="valid_namespace",
        namespace_rules=namespace_rules,
        channel_references=[
            {
                "channel_reference": "$1",
                "channel_config": ChannelConfig(
                    name="a_channel",
                    data_type=ChannelDataType.DOUBLE,
                ),
            }
        ],
        rule_client_key="valid_rule_client_key",
    )
    assert valid_namespace_rule.name == "valid_rule"
    assert valid_namespace_rule.description == "A rule in a namespace"
    assert valid_namespace_rule.expression == "$1 > 10"
    assert valid_namespace_rule.action.assignee == "bob@example.com"
    assert valid_namespace_rule.action.tags == ["foo", "bar"]
    assert valid_namespace_rule.action.kind() == RuleActionKind.ANNOTATION
    assert valid_namespace_rule.rule_client_key == "valid_rule_client_key"
    assert isinstance(valid_namespace_rule.action, RuleActionCreateDataReviewAnnotation)


def test_rule_namespace_missing_namespace_rules():
    with pytest.raises(ValueError, match="Namespace rules must be provided with namespace key."):
        RuleConfig(
            name="a_rule",
            namespace="a_namespace",
            channel_references=[
                {
                    "channel_reference": "$1",
                    "channel_config": ChannelConfig(
                        name="a_channel",
                        data_type=ChannelDataType.DOUBLE,
                    ),
                }
            ],
        )


def test_rule_namespace_missing_namespace():
    with pytest.raises(ValueError, match="Couldn't find namespace"):
        namespace_rules = {
            "a_namespace": [
                RuleYamlSpec(
                    {
                        "name": "valid_rule",
                        "description": "A rule in a namespace",
                        "expression": "$1 > 10",
                        "type": "review",
                        "assignee": "bob@example.com",
                        "tags": ["foo", "bar"],
                    }
                ),
            ],
            "another_namespace": [
                RuleYamlSpec(
                    {
                        "name": "valid_rule",
                        "description": "A rule in a namespace",
                        "expression": "$1 > 10",
                        "type": "review",
                        "assignee": "bob@example.com",
                        "tags": ["foo", "bar"],
                    }
                ),
            ],
        }

        RuleConfig(
            name="valid_rule",
            namespace="a_missing_namespace",
            namespace_rules=namespace_rules,
            channel_references=[
                {
                    "channel_reference": "$1",
                    "channel_config": ChannelConfig(
                        name="a_channel",
                        data_type=ChannelDataType.DOUBLE,
                    ),
                }
            ],
        )


def test_rule_namespace_missing_rule():
    with pytest.raises(ValueError, match="Does this rule exist in the namespace?"):
        namespace_rules = {
            "a_namespace": [
                RuleYamlSpec(
                    {
                        "name": "a_rule_in_namespace",
                        "description": "A rule in a namespace",
                        "expression": "$1 > 10",
                        "type": "review",
                        "assignee": "bob@example.com",
                        "tags": ["foo", "bar"],
                    }
                ),
            ],
            "another_namespace": [
                RuleYamlSpec(
                    {
                        "name": "another_rule_in_namespace",
                        "description": "A rule in a namespace",
                        "expression": "$1 > 10",
                        "type": "review",
                        "assignee": "bob@example.com",
                        "tags": ["foo", "bar"],
                    }
                ),
            ],
        }

        RuleConfig(
            name="a_missing_rule",
            namespace="a_namespace",
            namespace_rules=namespace_rules,
            channel_references=[
                {
                    "channel_reference": "$1",
                    "channel_config": ChannelConfig(
                        name="a_channel",
                        data_type=ChannelDataType.DOUBLE,
                    ),
                }
            ],
        )
