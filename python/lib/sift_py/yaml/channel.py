from __future__ import annotations

from typing import List, Literal, Union

from typing_extensions import NotRequired, TypedDict


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
