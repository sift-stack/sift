from __future__ import annotations

from enum import Enum
from typing import List, Optional, Type, TypedDict, Union

import sift.common.type.v1.channel_data_type_pb2 as channel_pb
from google.protobuf.empty_pb2 import Empty
from sift.channels.v3.channels_pb2 import Channel as ChannelPb
from sift.common.type.v1.channel_bit_field_element_pb2 import (
    ChannelBitFieldElement as ChannelBitFieldElementPb,
)
from sift.common.type.v1.channel_enum_type_pb2 import (
    ChannelEnumType as ChannelEnumTypePb,
)
from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataChannelValue
from sift.ingestion_configs.v2.ingestion_configs_pb2 import ChannelConfig as ChannelConfigPb
from typing_extensions import NotRequired, Self

from sift_py._internal.channel import channel_fqn as _channel_fqn
from sift_py._internal.convert.protobuf import AsProtobuf
from sift_py.error import _component_deprecation_warning


class ChannelValue(TypedDict):
    """
    Represents a fully qualified data point for a channel
    """

    channel_name: str
    component: NotRequired[str]  # Deprecated
    value: IngestWithConfigDataChannelValue


class ChannelConfig(AsProtobuf):
    """
    A description for a channel
    """

    name: str
    data_type: ChannelDataType
    description: Optional[str]
    unit: Optional[str]
    component: Optional[str]  # Deprecated
    bit_field_elements: List[ChannelBitFieldElement]
    enum_types: List[ChannelEnumType]
    identifier: str

    def __init__(
        self,
        name: str,
        data_type: ChannelDataType,
        description: Optional[str] = None,
        unit: Optional[str] = None,
        component: Optional[str] = None,  # Deprecated
        bit_field_elements: List[ChannelBitFieldElement] = [],
        enum_types: List[ChannelEnumType] = [],
    ):
        self.name = name
        self.data_type = data_type
        self.description = description
        self.unit = unit

        self.component = None  # Field kept for backwards compatibility
        if component:
            _component_deprecation_warning()
            self.name = _channel_fqn(name=self.name, component=component)
            self.component = None

        self.bit_field_elements = bit_field_elements
        self.enum_types = enum_types
        self.identifier = self.fqn()

    def value_from(
        self, value: Optional[Union[int, float, bool, str]]
    ) -> Optional[IngestWithConfigDataChannelValue]:
        """
        Like `try_value_from` except will return `None` there is a failure to produce a channel value due to a type mismatch.
        """
        try:
            return self.try_value_from(value)
        except ValueError:
            return None

    def try_value_from(
        self, value: Optional[Union[int, float, bool, str]]
    ) -> IngestWithConfigDataChannelValue:
        """
        Generate a channel value for this particular channel configuration. This will raise an exception
        if there is a type match, namely, if `value` isn't consistent with the channel's data-type. For a version
        of this function that does not raise an exception and simply ignores type mistmatches, see `value_from`. If `value`
        is `None` then an empty value will be generated.
        """
        if value is None:
            return empty_value()

        if isinstance(value, bool) and self.data_type == ChannelDataType.BOOL:
            return bool_value(value)
        elif isinstance(value, int) or isinstance(value, float):
            if self.data_type == ChannelDataType.INT_32:
                return int32_value(int(value))
            elif self.data_type == ChannelDataType.INT_64:
                return int64_value(int(value))
            elif self.data_type == ChannelDataType.UINT_32:
                return uint32_value(int(value))
            elif self.data_type == ChannelDataType.UINT_64:
                return uint64_value(int(value))
            elif self.data_type == ChannelDataType.FLOAT:
                return float_value(float(value))
            elif self.data_type == ChannelDataType.DOUBLE:
                return double_value(float(value))
            elif self.data_type == ChannelDataType.ENUM:
                return enum_value(int(value))
        elif isinstance(value, str) and self.data_type == ChannelDataType.STRING:
            return string_value(value)

        raise ValueError(f"Failed to cast value of type {type(value)} to {self.data_type}")

    def as_pb(self, klass: Type[ChannelConfigPb]) -> ChannelConfigPb:
        return klass(
            name=self.name,
            unit=self.unit or "",
            description=self.description or "",
            data_type=self.data_type.value,
            enum_types=[etype.as_pb(ChannelEnumTypePb) for etype in self.enum_types],
            bit_field_elements=[
                el.as_pb(ChannelBitFieldElementPb) for el in self.bit_field_elements
            ],
        )

    @classmethod
    def from_pb(cls, message: ChannelConfigPb) -> Self:
        return cls(
            name=message.name,
            data_type=ChannelDataType.from_pb(message.data_type),
            description=message.description,
            unit=message.unit,
            bit_field_elements=[
                ChannelBitFieldElement.from_pb(el) for el in message.bit_field_elements
            ],
            enum_types=[ChannelEnumType.from_pb(etype) for etype in message.enum_types],
        )

    def fqn(self) -> str:
        """
        NOTE: Component field of Channel has been deprecated. Function kept for backwards compatibility.

        The fully-qualified channel name of a channel called 'voltage' is simply `voltage`. The
        fully qualified name of a channel called 'temperature' of component 'motor' is a `motor.temperature'.
        """
        return channel_fqn(self)


class ChannelBitFieldElement(AsProtobuf):
    name: str
    index: int
    bit_count: int

    def __init__(self, name: str, index: int, bit_count: int):
        self.name = name
        self.index = index
        self.bit_count = bit_count

    def as_pb(self, klass: Type[ChannelBitFieldElementPb]) -> ChannelBitFieldElementPb:
        return klass(
            name=self.name,
            index=self.index,
            bit_count=self.bit_count,
        )

    @classmethod
    def from_pb(cls, message: ChannelBitFieldElementPb) -> Self:
        return cls(
            name=message.name,
            index=message.index,
            bit_count=message.bit_count,
        )


class ChannelEnumType(AsProtobuf):
    name: str
    key: int

    def __init__(self, name: str, key: int):
        self.name = name
        self.key = key

    def as_pb(self, klass: Type[ChannelEnumTypePb]) -> ChannelEnumTypePb:
        return klass(name=self.name, key=self.key)

    @classmethod
    def from_pb(cls, message: ChannelEnumTypePb) -> Self:
        return cls(name=message.name, key=message.key)


class ChannelDataTypeStrRep(Enum):
    DOUBLE = "double"
    STRING = "string"
    ENUM = "enum"
    BIT_FIELD = "bit_field"
    BOOL = "bool"
    FLOAT = "float"
    INT_32 = "int32"
    INT_64 = "int64"
    UINT_32 = "uint32"
    UINT_64 = "uint64"

    @staticmethod
    def from_api_format(val: str) -> Optional["ChannelDataTypeStrRep"]:
        try:
            return {
                "CHANNEL_DATA_TYPE_DOUBLE": ChannelDataTypeStrRep.DOUBLE,
                "CHANNEL_DATA_TYPE_STRING": ChannelDataTypeStrRep.STRING,
                "CHANNEL_DATA_TYPE_ENUM": ChannelDataTypeStrRep.ENUM,
                "CHANNEL_DATA_TYPE_BIT_FIELD": ChannelDataTypeStrRep.BIT_FIELD,
                "CHANNEL_DATA_TYPE_BOOL": ChannelDataTypeStrRep.BOOL,
                "CHANNEL_DATA_TYPE_FLOAT": ChannelDataTypeStrRep.FLOAT,
                "CHANNEL_DATA_TYPE_INT_32": ChannelDataTypeStrRep.INT_32,
                "CHANNEL_DATA_TYPE_INT_64": ChannelDataTypeStrRep.INT_64,
                "CHANNEL_DATA_TYPE_UINT_32": ChannelDataTypeStrRep.UINT_32,
                "CHANNEL_DATA_TYPE_UINT_64": ChannelDataTypeStrRep.UINT_64,
            }[val]
        except KeyError:
            return None


class ChannelDataType(Enum):
    """
    Utility enum class to simplify working with channel data-types generated from protobuf
    """

    DOUBLE = channel_pb.CHANNEL_DATA_TYPE_DOUBLE
    STRING = channel_pb.CHANNEL_DATA_TYPE_STRING
    ENUM = channel_pb.CHANNEL_DATA_TYPE_ENUM
    BIT_FIELD = channel_pb.CHANNEL_DATA_TYPE_BIT_FIELD
    BOOL = channel_pb.CHANNEL_DATA_TYPE_BOOL
    FLOAT = channel_pb.CHANNEL_DATA_TYPE_FLOAT
    INT_32 = channel_pb.CHANNEL_DATA_TYPE_INT_32
    INT_64 = channel_pb.CHANNEL_DATA_TYPE_INT_64
    UINT_32 = channel_pb.CHANNEL_DATA_TYPE_UINT_32
    UINT_64 = channel_pb.CHANNEL_DATA_TYPE_UINT_64

    @classmethod
    def from_pb(cls, val: channel_pb.ChannelDataType.ValueType) -> "ChannelDataType":
        if val == cls.DOUBLE.value:
            return cls.DOUBLE
        elif val == cls.STRING.value:
            return cls.STRING
        elif val == cls.ENUM.value:
            return cls.ENUM
        elif val == cls.BIT_FIELD.value:
            return cls.BIT_FIELD
        elif val == cls.BOOL.value:
            return cls.BOOL
        elif val == cls.FLOAT.value:
            return cls.FLOAT
        elif val == cls.INT_32.value:
            return cls.INT_32
        elif val == cls.INT_64.value:
            return cls.INT_64
        elif val == cls.UINT_32.value:
            return cls.UINT_32
        elif val == cls.UINT_64.value:
            return cls.UINT_64
        else:
            raise ValueError(f"Unknown channel data type '{val}'.")

    @classmethod
    def from_str(cls, raw: str) -> Optional["ChannelDataType"]:
        if raw.startswith("CHANNEL_DATA_TYPE_"):
            val = ChannelDataTypeStrRep.from_api_format(raw)
            if val is None:
                return None
        else:
            try:
                val = ChannelDataTypeStrRep(raw)
            except ValueError:
                return None

        if val == ChannelDataTypeStrRep.DOUBLE:
            return cls.DOUBLE
        elif val == ChannelDataTypeStrRep.STRING:
            return cls.STRING
        elif val == ChannelDataTypeStrRep.ENUM:
            return cls.ENUM
        elif val == ChannelDataTypeStrRep.BIT_FIELD:
            return cls.BIT_FIELD
        elif val == ChannelDataTypeStrRep.BOOL:
            return cls.BOOL
        elif val == ChannelDataTypeStrRep.FLOAT:
            return cls.FLOAT
        elif val == ChannelDataTypeStrRep.INT_32:
            return cls.INT_32
        elif val == ChannelDataTypeStrRep.INT_64:
            return cls.INT_64
        elif val == ChannelDataTypeStrRep.UINT_32:
            return cls.UINT_32
        elif val == ChannelDataTypeStrRep.UINT_64:
            return cls.UINT_64
        else:
            raise Exception("Unreachable")

    def as_human_str(self, api_format: bool = False) -> str:
        if self == ChannelDataType.DOUBLE:
            return "CHANNEL_DATA_TYPE_DOUBLE" if api_format else ChannelDataTypeStrRep.DOUBLE.value
        elif self == ChannelDataType.STRING:
            return "CHANNEL_DATA_TYPE_STRING" if api_format else ChannelDataTypeStrRep.STRING.value
        elif self == ChannelDataType.ENUM:
            return "CHANNEL_DATA_TYPE_ENUM" if api_format else ChannelDataTypeStrRep.ENUM.value
        elif self == ChannelDataType.BIT_FIELD:
            return (
                "CHANNEL_DATA_TYPE_BIT_FIELD"
                if api_format
                else ChannelDataTypeStrRep.BIT_FIELD.value
            )
        elif self == ChannelDataType.BOOL:
            return "CHANNEL_DATA_TYPE_BOOL" if api_format else ChannelDataTypeStrRep.BOOL.value
        elif self == ChannelDataType.FLOAT:
            return "CHANNEL_DATA_TYPE_FLOAT" if api_format else ChannelDataTypeStrRep.FLOAT.value
        elif self == ChannelDataType.INT_32:
            return "CHANNEL_DATA_TYPE_INT_32" if api_format else ChannelDataTypeStrRep.INT_32.value
        elif self == ChannelDataType.INT_64:
            return "CHANNEL_DATA_TYPE_INT_64" if api_format else ChannelDataTypeStrRep.INT_64.value
        elif self == ChannelDataType.UINT_32:
            return (
                "CHANNEL_DATA_TYPE_UINT_32" if api_format else ChannelDataTypeStrRep.UINT_32.value
            )
        elif self == ChannelDataType.UINT_64:
            return (
                "CHANNEL_DATA_TYPE_UINT_64" if api_format else ChannelDataTypeStrRep.UINT_64.value
            )
        else:
            raise Exception("Unreachable.")


class _AbstractChannel(TypedDict):
    channel_name: str
    component: Optional[str]  # Deprecated


def channel_fqn(
    channel: Union[
        ChannelConfig,
        ChannelConfigPb,
        ChannelValue,
        ChannelPb,
        _AbstractChannel,
    ],
) -> str:
    """
    Computes the fully qualified channel name.

    NOTE: Component field of Channel is deprecated and should not be used. Function is left for code compatibility.

    The fully-qualified channel name of a channel called 'voltage' is simply `voltage'. The
    fully qualified name of a channel called 'temperature' of component 'motor' is a `motor.temperature'.
    """

    if isinstance(channel, ChannelConfig):
        if channel.component:
            _component_deprecation_warning()
        return _channel_fqn(channel.name, channel.component)
    elif isinstance(channel, ChannelConfigPb):
        return channel.name
    elif isinstance(channel, ChannelPb):
        return channel.name
    else:
        component = channel.get("component")
        if component:
            _component_deprecation_warning()
        channel_name = channel["channel_name"]
        return _channel_fqn(name=channel_name, component=component)


def string_value(val: str) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(string=val)


def double_value(val: float) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(double=val)


def float_value(val: float) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(float=val)


def bool_value(val: bool) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(bool=val)


def int32_value(val: int) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(int32=val)


def uint32_value(val: int) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(uint32=val)


def int64_value(val: int) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(int64=val)


def uint64_value(val: int) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(uint64=val)


def bit_field_value(val: bytes) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(bit_field=val)


def enum_value(val: int) -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(enum=val)


def empty_value() -> IngestWithConfigDataChannelValue:
    return IngestWithConfigDataChannelValue(empty=Empty())


def is_data_type(val: IngestWithConfigDataChannelValue, target_type: ChannelDataType) -> bool:
    if target_type == ChannelDataType.DOUBLE:
        return val.HasField("double")
    elif target_type == ChannelDataType.STRING:
        return val.HasField("string")
    elif target_type == ChannelDataType.ENUM:
        return val.HasField("enum")
    elif target_type == ChannelDataType.BIT_FIELD:
        return val.HasField("bit_field")
    elif target_type == ChannelDataType.BOOL:
        return val.HasField("bool")
    elif target_type == ChannelDataType.FLOAT:
        return val.HasField("float")
    elif target_type == ChannelDataType.INT_32:
        return val.HasField("int32")
    elif target_type == ChannelDataType.INT_64:
        return val.HasField("int64")
    elif target_type == ChannelDataType.UINT_32:
        return val.HasField("uint32")
    elif target_type == ChannelDataType.UINT_64:
        return val.HasField("uint64")
