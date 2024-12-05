import pytest

from sift_py.ingestion.config.yaml.error import YamlConfigError
from sift_py.yaml import rule


def test__validate_sub_expression():
    rule._validate_sub_expression({"$mass": 10})
    rule._validate_sub_expression({"$m": 10})

    with pytest.raises(YamlConfigError, match="Invalid sub-expression key"):
        rule._validate_sub_expression({"mass": 10})

    with pytest.raises(YamlConfigError, match="Invalid sub-expression key"):
        rule._validate_sub_expression({"$!mass": 10})

    with pytest.raises(YamlConfigError, match="Invalid sub-expression key"):
        rule._validate_sub_expression({"$$mass": 10})


def test__validate_rule():
    rule._validate_rule(
        {
            "name": "overheat_rule",
            "description": "some_description",
            "expression": "$1 > 10 && $2 > 10",
            "type": "review",
            "assignee": "homer@example.com",
            "tags": ["foo", "bar"],
            "channel_references": [
                {"$1": {"name": "voltage", "data_type": "double"}},
                {"$2": {"name": "vehicle_state", "data_type": "double"}},
            ],
            "rule_client_key": "overheat_rule_key",
        }
    )

    # Rule with tag and asset names
    rule._validate_rule(
        {
            "name": "overheat_rule",
            "description": "some_description",
            "expression": "$1 > 10 && $2 > 10",
            "type": "review",
            "assignee": "homer@example.com",
            "tags": ["foo", "bar"],
            "channel_references": [
                {"$1": {"name": "voltage", "data_type": "double"}},
                {"$2": {"name": "vehicle_state", "data_type": "double"}},
            ],
            "asset_names": ["NostromoLV426"],
            "tag_names": ["vehicle"],
            "rule_client_key": "overheat_rule_key",
        }
    )

    with pytest.raises(YamlConfigError, match="Expected 'name' to be <str> but it is <int>"):
        rule._validate_rule(
            {
                "name": 0,
                "description": "some_description",
                "expression": "$1 > 10 && $2 > 10",
                "type": "review",
                "assignee": "homer@example.com",
                "tags": ["foo", "bar"],
                "channel_references": [
                    {"$1": {"name": "voltage", "data_type": "double"}},
                    {"$2": {"name": "vehicle_state", "data_type": "double"}},
                ],
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'description' to be <str> but it is <int>"):
        rule._validate_rule(
            {
                "name": "overheat_rule",
                "description": 0,
                "expression": "$1 > 10 && $2 > 10",
                "type": "review",
                "assignee": "homer@example.com",
                "tags": ["foo", "bar"],
                "channel_references": [
                    {"$1": {"name": "voltage", "data_type": "double"}},
                    {"$2": {"name": "vehicle_state", "data_type": "double"}},
                ],
            }
        )

    with pytest.raises(
        YamlConfigError, match="Expected 'expression' to be <<class 'str'> | <class 'dict'>>"
    ):
        rule._validate_rule(
            {
                "name": "overheat_rule",
                "description": "some_description",
                "expression": 123,
                "type": "review",
                "assignee": "homer@example.com",
                "tags": ["foo", "bar"],
                "channel_references": [
                    {"$1": {"name": "voltage", "data_type": "double"}},
                    {"$2": {"name": "vehicle_state", "data_type": "double"}},
                ],
            }
        )

    with pytest.raises(
        YamlConfigError, match="Expected 'type' to be <review | phase> but it is <str>"
    ):
        rule._validate_rule(
            {
                "name": "overheat_rule",
                "description": "foobar",
                "expression": "$1 > 10 && $2 > 10",
                "type": "not_valid_type",
                "assignee": "homer@example.com",
                "tags": ["foo", "bar"],
                "channel_references": [
                    {"$1": {"name": "voltage", "data_type": "double"}},
                    {"$2": {"name": "vehicle_state", "data_type": "double"}},
                ],
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'tags' to be"):
        rule._validate_rule(
            {
                "name": "overheat_rule",
                "description": "foobar",
                "expression": "$1 > 10 && $2 > 10",
                "type": "phase",
                "assignee": "homer@example.com",
                "tags": 123,
                "channel_references": [
                    {"$1": {"name": "voltage", "data_type": "double"}},
                    {"$2": {"name": "vehicle_state", "data_type": "double"}},
                ],
            }
        )

    with pytest.raises(YamlConfigError, match="Invalid channel reference key"):
        rule._validate_rule(
            {
                "name": "overheat_rule",
                "description": "foobar",
                "expression": "$1 > 10 && $2 > 10",
                "type": "phase",
                "assignee": "homer@example.com",
                "channel_references": [
                    {"$foo": {"name": "voltage", "data_type": "double"}},
                    {"$2": {"name": "vehicle_state", "data_type": "double"}},
                ],
            }
        )

    with pytest.raises(YamlConfigError, match="Invalid sub-expression key"):
        rule._validate_rule(
            {
                "name": "overheat_rule",
                "description": "foobar",
                "expression": {"name": "kinetic_energy_gt"},
                "type": "phase",
                "assignee": "homer@example.com",
                "channel_references": [
                    {"$1": {"name": "velocity", "data_type": "double"}},
                ],
                "sub_expressions": [
                    {"mass": None},
                ],
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'asset_names' to be"):
        rule._validate_rule(
            {
                "name": "overheat_rule",
                "description": "some_description",
                "expression": "$1 > 10 && $2 > 10",
                "type": "review",
                "assignee": "homer@example.com",
                "tags": ["foo", "bar"],
                "channel_references": [
                    {"$1": {"name": "voltage", "data_type": "double"}},
                    {"$2": {"name": "vehicle_state", "data_type": "double"}},
                ],
                "rule_client_key": "overheat_rule_key",
                "asset_names": 123,
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'tag_names' to be"):
        rule._validate_rule(
            {
                "name": "overheat_rule",
                "description": "some_description",
                "expression": "$1 > 10 && $2 > 10",
                "type": "review",
                "assignee": "homer@example.com",
                "tags": ["foo", "bar"],
                "channel_references": [
                    {"$1": {"name": "voltage", "data_type": "double"}},
                    {"$2": {"name": "vehicle_state", "data_type": "double"}},
                ],
                "rule_client_key": "overheat_rule_key",
                "tag_names": 123,
            }
        )
