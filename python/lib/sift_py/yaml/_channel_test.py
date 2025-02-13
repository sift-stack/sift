import pytest

from sift_py.error import SiftAPIDeprecationWarning
from sift_py.ingestion.config.yaml.error import YamlConfigError
from sift_py.yaml import channel


def test__validate_channel_anchor():
    channel._validate_channel_anchor("foo")

    with pytest.raises(YamlConfigError, match="Expected '<str>'"):
        channel._validate_channel_anchor(3)


def test__validate_enum_type():
    channel._validate_enum_type(
        {
            "name": "foo",
            "key": 0,
        }
    )

    with pytest.raises(YamlConfigError, match="Expected 'key' to be <int> but it is <str>"):
        channel._validate_enum_type(
            {
                "name": "foo",
                "key": "foobar",
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'name' to be <str> but it is <int>"):
        channel._validate_enum_type(
            {
                "name": 123,
                "key": 0,
            }
        )


def test__validate_bit_field_element():
    channel._validate_bit_field_element(
        {
            "name": "heater",
            "index": 0,
            "bit_count": 3,
        }
    )

    with pytest.raises(YamlConfigError, match="Expected 'name' to be <str> but it is <int>"):
        channel._validate_bit_field_element(
            {
                "name": 0,
                "index": 0,
                "bit_count": 3,
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'index' to be <int> but it is <str>"):
        channel._validate_bit_field_element(
            {
                "name": "heater",
                "index": "foobar",
                "bit_count": 3,
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'bit_count' to be <int> but it is <str>"):
        channel._validate_bit_field_element(
            {
                "name": "heater",
                "index": 0,
                "bit_count": "foo",
            }
        )


def test__validate_channel():
    channel._validate_channel(
        {
            "name": "force",
            "data_type": "double",
            "unit": "N",
        }
    )

    with pytest.raises(YamlConfigError, match="Expected 'data_type' to be"):
        channel._validate_channel(
            {
                "name": "force",
                "data_type": "dubble",
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'description' to be <str> but it is <int>"):
        channel._validate_channel(
            {
                "name": "force",
                "description": 9001,
                "data_type": "double",
                "unit": "N",
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'unit' to be <str> but it is <list>"):
        # won't warn since exception is raised first
        # with pytest.warns(SiftAPIDeprecationWarning, match="component"):
        channel._validate_channel(
            {
                "name": "force",
                "description": "use the force",
                "data_type": "double",
                "component": "motor",
                "unit": [1, 2, 3],
            }
        )

    with pytest.raises(YamlConfigError, match="Expected 'component' to be <str> but it is <dict>"):
        with pytest.warns(SiftAPIDeprecationWarning, match="component"):
            channel._validate_channel(
                {
                    "name": "force",
                    "description": "use the force",
                    "data_type": "double",
                    "component": {},
                    "unit": "N",
                }
            )

    with pytest.raises(YamlConfigError, match="should not have 'bit_field_elements' set"):
        with pytest.warns(SiftAPIDeprecationWarning, match="component"):
            channel._validate_channel(
                {
                    "name": "force",
                    "description": "use the force",
                    "data_type": "double",
                    "component": "motor",
                    "unit": "N",
                    "bit_field_elements": [{"name": "heat", "index": 0, "bit_count": 3}],
                }
            )
    with pytest.warns(SiftAPIDeprecationWarning, match="component"):
        channel._validate_channel(
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
        with pytest.warns(SiftAPIDeprecationWarning, match="component"):
            channel._validate_channel(
                {
                    "name": "force",
                    "description": "use the force",
                    "data_type": "double",
                    "component": "motor",
                    "unit": "N",
                    "enum_types": [{"name": "heat", "key": 0}],
                }
            )

    with pytest.warns(SiftAPIDeprecationWarning, match="component"):
        channel._validate_channel(
            {
                "name": "force",
                "description": "use the force",
                "data_type": "enum",
                "component": "motor",
                "unit": "N",
                "enum_types": [{"name": "heat", "key": 0}],
            }
        )
