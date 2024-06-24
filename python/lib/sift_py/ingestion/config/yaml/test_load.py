import pytest

from sift_py.ingestion.config.yaml import load
from sift_py.ingestion.config.yaml.error import YamlConfigError


def test__validate_channel_anchor():
    load._validate_channel_anchor("foo")

    with pytest.raises(YamlConfigError, match="Expected '<str>'"):
        load._validate_channel_anchor(3)


def test__validate_sub_expression():
    load._validate_sub_expression({"$mass": 10})
    load._validate_sub_expression({"$m": 10})

    with pytest.raises(YamlConfigError, match="Invalid sub-expression key"):
        load._validate_sub_expression({"mass": 10})

    with pytest.raises(YamlConfigError, match="Invalid sub-expression key"):
        load._validate_sub_expression({"$!mass": 10})

    with pytest.raises(YamlConfigError, match="Invalid sub-expression key"):
        load._validate_sub_expression({"$$mass": 10})


def test__validate_enum_type():
    load._validate_enum_type(
        {
            "name": "foo",
            "key": 0,
        }
    )

    with pytest.raises(YamlConfigError, match="Expected 'key' to be <int> but it is <str>"):
        load._validate_enum_type(
            {
                "name": "foo",
                "key": "foobar",
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'name' to be <str> but it is <int>"):
        load._validate_enum_type(
            {
                "name": 123,
                "key": 0,
            }
        )


def test__validate_bit_field_element():
    load._validate_bit_field_element(
        {
            "name": "heater",
            "index": 0,
            "bit_count": 3,
        }
    )

    with pytest.raises(YamlConfigError, match="Expected 'name' to be <str> but it is <int>"):
        load._validate_bit_field_element(
            {
                "name": 0,
                "index": 0,
                "bit_count": 3,
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'index' to be <int> but it is <str>"):
        load._validate_bit_field_element(
            {
                "name": "heater",
                "index": "foobar",
                "bit_count": 3,
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'bit_count' to be <int> but it is <str>"):
        load._validate_bit_field_element(
            {
                "name": "heater",
                "index": 0,
                "bit_count": "foo",
            }
        )


def test__validate_channel():
    load._validate_channel(
        {
            "name": "force",
            "data_type": "double",
            "unit": "N",
        }
    )

    with pytest.raises(YamlConfigError, match="Expected 'data_type' to be"):
        load._validate_channel(
            {
                "name": "force",
                "data_type": "dubble",
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'description' to be <str> but it is <int>"):
        load._validate_channel(
            {
                "name": "force",
                "description": 9001,
                "data_type": "double",
                "unit": "N",
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'unit' to be <str> but it is <list>"):
        load._validate_channel(
            {
                "name": "force",
                "description": "use the force",
                "data_type": "double",
                "component": "motor",
                "unit": [1, 2, 3],
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'component' to be <str> but it is <dict>"):
        load._validate_channel(
            {
                "name": "force",
                "description": "use the force",
                "data_type": "double",
                "component": {},
                "unit": "N",
            }
        )

    with pytest.raises(YamlConfigError, match="should not have 'bit_field_elements' set"):
        load._validate_channel(
            {
                "name": "force",
                "description": "use the force",
                "data_type": "double",
                "component": "motor",
                "unit": "N",
                "bit_field_elements": [{"name": "heat", "index": 0, "bit_count": 3}],
            }
        )

    load._validate_channel(
        {
            "name": "force",
            "description": "use the force",
            "data_type": "bit_field",
            "component": "motor",
            "unit": "N",
            "bit_field_elements": [{"name": "heat", "index": 0, "bit_count": 3}],
        }
    )

    with pytest.raises(YamlConfigError, match="should not have 'enum_types' set"):
        load._validate_channel(
            {
                "name": "force",
                "description": "use the force",
                "data_type": "double",
                "component": "motor",
                "unit": "N",
                "enum_types": [{"name": "heat", "key": 0}],
            }
        )

    load._validate_channel(
        {
            "name": "force",
            "description": "use the force",
            "data_type": "enum",
            "component": "motor",
            "unit": "N",
            "enum_types": [{"name": "heat", "key": 0}],
        }
    )


def test__validate_rule():
    load._validate_rule(
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
        }
    )

    with pytest.raises(YamlConfigError, match="Expected 'name' to be <str> but it is <int>"):
        load._validate_rule(
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
        load._validate_rule(
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
        load._validate_rule(
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
        load._validate_rule(
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
        load._validate_rule(
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
        load._validate_rule(
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
        load._validate_rule(
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


def test__validate_flow():
    load._validate_flow(
        {
            "name": "reading",
            "channels": [
                {"name": "voltage", "data_type": "double"},
            ],
        }
    )

    with pytest.raises(YamlConfigError):
        load._validate_flow(
            {
                "name": "reading",
                "channels": [
                    {"name": "voltage", "data_type": "dubble"},
                ],
            }
        )
