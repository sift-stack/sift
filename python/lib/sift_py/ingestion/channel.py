from __future__ import annotations
from google.protobuf.empty_pb2 import Empty
from sift_internal.convert.protobuf import AsProtobuf, ProtobufMessage, try_cast_pb
from enum import Enum
from sift.common.type.v1.channel_enum_type_pb2 import (
    ChannelEnumType as ChannelEnumTypePb,
)
from sift.common.type.v1.channel_bit_field_element_pb2 import (
    ChannelBitFieldElement as ChannelBitFieldElementPb,
)
from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataChannelValue
from sift.ingestion_configs.v1.ingestion_configs_pb2 import (
    ChannelConfig as ChannelConfigPb,
)
from typing import List, Optional, Type, TypedDict, Union

import sift.common.type.v1.channel_data_type_pb2 as channel_pb


class ChannelValue(TypedDict):
    """
    Represents a fully qualified data point for a channel
    """

    channel_name: str
    component: Union[str, None]
    value: IngestWithConfigDataChannelValue


class ChannelConfig(AsProtobuf):
    """
    A description for a channel
    """

    name: str
    data_type: ChannelDataType
    description: Optional[str]
    unit: Optional[str]
    component: Optional[str]
    bit_field_elements: List[ChannelBitFieldElement]
    enum_types: List[ChannelEnumType]

    def __init__(
        self,
        name: str,
        data_type: ChannelDataType,
        description: Optional[str] = None,
        unit: Optional[str] = None,
        component: Optional[str] = None,
        bit_field_elements: List[ChannelBitFieldElement] = [],
        enum_types: List[ChannelEnumType] = [],
    ):
        self.name = name
        self.data_type = data_type
        self.description = description
        self.unit = unit
        self.component = component
        self.bit_field_elements = bit_field_elements
        self.enum_types = enum_types

    def as_pb(self, klass: Type[ProtobufMessage]) -> Optional[ProtobufMessage]:
        return ChannelConfigPb(
            name=self.name,
            component=self.component or "",
            unit=self.unit or "",
            description=self.description or "",
            data_type=self.data_type.value,
            enum_types=[try_cast_pb(etype, ChannelEnumTypePb) for etype in self.enum_types],
            bit_field_elements=[
                try_cast_pb(el, ChannelBitFieldElementPb) for el in self.bit_field_elements
            ],
        )


class ChannelBitFieldElement(AsProtobuf):
    name: str
    index: int
    bit_count: int

    def __init__(self, name: str, index: int, bit_count: int):
        self.name = name
        self.index = index
        self.bit_count = bit_count

    def as_pb(self, klass: Type[ProtobufMessage]) -> Optional[ProtobufMessage]:
        return ChannelBitFieldElementPb(
            name=self.name,
            index=self.index,
            bit_count=self.bit_count,
        )


class ChannelEnumType(AsProtobuf):
    name: str
    key: int

    def __init__(self, name: str, key: int):
        self.name = name
        self.key = key

    def as_pb(self, klass: Type[ProtobufMessage]) -> Optional[ProtobufMessage]:
        return ChannelEnumTypePb(name=self.name, key=self.key)


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
    def from_str(cls, val: str) -> Optional["ChannelDataType"]:
        if val == "CHANNEL_DATA_TYPE_DOUBLE":
            return cls.DOUBLE
        elif val == "CHANNEL_DATA_TYPE_STRING":
            return cls.STRING
        elif val == "CHANNEL_DATA_TYPE_ENUM":
            return cls.ENUM
        elif val == "CHANNEL_DATA_TYPE_BIT_FIELD":
            return cls.BIT_FIELD
        elif val == "CHANNEL_DATA_TYPE_BOOL":
            return cls.BOOL
        elif val == "CHANNEL_DATA_TYPE_FLOAT":
            return cls.FLOAT
        elif val == "CHANNEL_DATA_TYPE_INT_32":
            return cls.INT_32
        elif val == "CHANNEL_DATA_TYPE_INT_64":
            return cls.INT_64
        elif val == "CHANNEL_DATA_TYPE_UINT_32":
            return cls.UINT_32
        elif val == "CHANNEL_DATA_TYPE_UINT_64":
            return cls.UINT_64

        return None


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
