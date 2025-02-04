from __future__ import annotations

import re
from typing import Any, Dict, List, Literal, Union, cast

from typing_extensions import NotRequired, TypedDict

from sift_py.error import _component_deprecation_warning
from sift_py.ingestion.channel import ChannelDataTypeStrRep
from sift_py.ingestion.config.yaml.error import YamlConfigError
from sift_py.yaml.utils import _type_fqn

_CHANNEL_REFERENCE_REGEX = re.compile(r"^\$\d+$")


def _validate_channel_anchor(val: Any):
    if not isinstance(val, str):
        raise YamlConfigError._invalid_property(
            val,
            "<str>",
            "&str",
            ["channels"],
        )


def _validate_channel(val: Any):
    channel = cast(Dict[Any, Any], val)

    name = channel.get("name")

    if not isinstance(name, str):
        raise YamlConfigError._invalid_property(name, "- name", "str", ["channels"])

    description = channel.get("description")

    if description is not None and not isinstance(description, str):
        raise YamlConfigError._invalid_property(description, "- description", "str", ["channels"])

    unit = channel.get("unit")

    if unit is not None and not isinstance(unit, str):
        raise YamlConfigError._invalid_property(unit, "- unit", "str", ["channels"])
    component = channel.get("component")
    if component is not None:
        _component_deprecation_warning()
        if not isinstance(component, str):
            raise YamlConfigError._invalid_property(component, "- component", "str", ["channels"])

    data_type = channel.get("data_type")
    valid_data_type_values = [v.value for v in ChannelDataTypeStrRep]

    if not data_type in valid_data_type_values:
        raise YamlConfigError._invalid_property(
            data_type,
            "- data_type",
            " | ".join(valid_data_type_values),
            ["channels"],
        )

    if data_type == ChannelDataTypeStrRep.ENUM.value:
        enum_types = channel.get("enum_types")

        if not isinstance(enum_types, list):
            raise YamlConfigError._invalid_property(
                enum_types,
                "- enum_types",
                f"List<{_type_fqn(ChannelEnumTypeYamlSpec)}>",
                ["channels"],
            )

        for enum_type in cast(List[Any], enum_types):
            _validate_enum_type(enum_type)

    elif data_type == ChannelDataTypeStrRep.BIT_FIELD.value:
        bit_field_elements = channel.get("bit_field_elements")

        if not isinstance(bit_field_elements, list):
            raise YamlConfigError._invalid_property(
                bit_field_elements,
                "- bit_field_elements",
                f"List<{_type_fqn(ChannelBitFieldElementYamlSpec)}>",
            )

        for bit_field_element in cast(List[Any], bit_field_elements):
            _validate_bit_field_element(bit_field_element)

    else:
        enum_types = channel.get("enum_types")

        if enum_types is not None:
            raise YamlConfigError(
                f"Channel of data-type '{data_type}' should not have 'enum_types' set."
            )

        bit_field_elements = channel.get("bit_field_elements")

        if bit_field_elements is not None:
            raise YamlConfigError(
                f"Channel of data-type '{data_type}' should not have 'bit_field_elements' set."
            )


def _validate_enum_type(val: Any):
    enum_type = cast(Dict[Any, Any], val)

    name = enum_type.get("name")

    if not isinstance(name, str):
        raise YamlConfigError._invalid_property(
            name,
            "- name",
            "str",
            ["channels", "- enum_type"],
        )

    key = enum_type.get("key")

    if not isinstance(key, int):
        raise YamlConfigError._invalid_property(
            key,
            "- key",
            "int",
            ["channels", "- enum_type"],
        )


def _validate_bit_field_element(val: Any):
    bit_field_element = cast(Dict[Any, Any], val)

    name = bit_field_element.get("name")

    if not isinstance(name, str):
        raise YamlConfigError._invalid_property(
            name, "- name", "str", ["channels", "- bit_field_elements"]
        )

    index = bit_field_element.get("index")

    if not isinstance(index, int):
        raise YamlConfigError._invalid_property(
            name, "- index", "int", ["channels", "- bit_field_elements"]
        )

    bit_count = bit_field_element.get("bit_count")

    if not isinstance(bit_count, int):
        raise YamlConfigError._invalid_property(
            name, "- bit_count", "int", ["channels", "- bit_field_elements"]
        )


def _validate_channel_reference(val: Any):
    channel_reference = cast(Dict[Any, Any], val)

    for key, value in channel_reference.items():
        if not isinstance(key, str):
            raise YamlConfigError._invalid_property(
                channel_reference,
                "- <str>",
                f"Dict[str, {_type_fqn(ChannelConfigYamlSpec)}]",
                ["rules", "- channel_references"],
            )

        if _CHANNEL_REFERENCE_REGEX.match(key) is None:
            raise YamlConfigError(
                f"Invalid channel reference key '{key}'. Expected an integer prefixed with '$' e.g. '$1', '$2', and so on."
            )

        if isinstance(value, dict):  # Do this for YamlConfigChannelSpec but not str
            try:
                _validate_channel(value)
            except YamlConfigError as err:
                raise YamlConfigError(f"Rule '{key}' contains an invalid channel reference:\n{err}")


class ChannelConfigYamlSpec(TypedDict):
    """
    Formal spec that defines what a channel should look like in YAML.

    `name`: Name of channel.
    `description`: Optional channel description.
    `unit`: Unit of measurement.
    `component`: Name of component that channel belongs to.
    `data_type`: Type of the data associated with the channel.
    `enum_types`: Required if `data_type` is `enum.
    `bit_field_elements`: Required if `data_type` is `bit_field`.
    """

    name: str
    description: NotRequired[str]
    unit: NotRequired[str]
    component: NotRequired[str]
    data_type: Union[
        Literal["double"],
        Literal["string"],
        Literal["enum"],
        Literal["bit_field"],
        Literal["bool"],
        Literal["float"],
        Literal["int32"],
        Literal["int64"],
        Literal["uint32"],
        Literal["uint64"],
    ]
    enum_types: NotRequired[List[ChannelEnumTypeYamlSpec]]
    bit_field_elements: NotRequired[List[ChannelBitFieldElementYamlSpec]]


class ChannelEnumTypeYamlSpec(TypedDict):
    """
    Formal spec that defines what a channel enum type should look like in YAML.
    """

    name: str
    key: int


class ChannelBitFieldElementYamlSpec(TypedDict):
    """
    Formal spec that defines what a bit-field element should look like in YAML.
    """

    name: str
    index: int
    bit_count: int
