from __future__ import annotations

from datetime import datetime
import warnings
from enum import Enum
from typing import Any, List, Optional, TypedDict, Union

import pandas as pd
import sift.common.type.v1.channel_data_type_pb2 as channel_pb
from sift.data.v2.data_pb2 import (
    DoubleValues,
    FloatValues,
    StringValues,
    EnumValues,
    BitFieldValues,
    BoolValues,
    Int32Values,
    Int64Values,
    Uint32Values,
    Uint64Values,
)
from pydantic import BaseModel, ConfigDict
from sift.common.type.v1.channel_bit_field_element_pb2 import (
    ChannelBitFieldElement as ChannelBitFieldElementPb,
)
from sift.common.type.v1.channel_enum_type_pb2 import ChannelEnumType as ChannelEnumTypePb
from sift.ingestion_configs.v2.ingestion_configs_pb2 import ChannelConfig as ChannelConfigProto
from sift.channels.v3.channels_pb2 import Channel as ChannelProto
from sift_client.types._base import BaseType


# TypedDicts for channel values
class ChannelValue(TypedDict, total=False):
    channel_name: str
    component: Optional[str]  # Deprecated
    value: Union[int, float, bool, str, None]


class _AbstractChannel(TypedDict, total=False):
    channel_name: str
    component: Optional[str]  # Deprecated


# Enum for string representation of channel data types
class ChannelDataTypeStrRep(str, Enum):
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
        for item in ChannelDataTypeStrRep:
            if "CHANNEL_DATA_TYPE_" + item.name == val:
                return item
        return None


class ChannelTypeUrls(str, Enum):
    DOUBLE = "sift.data.v2.DoubleValues"
    FLOAT = "sift.data.v2.FloatValues"
    STRING = "sift.data.v2.StringValues"
    ENUM = "sift.data.v2.EnumValues"
    BIT_FIELD = "sift.data.v2.BitFieldValues"
    BOOL = "sift.data.v2.BoolValues"
    INT_32 = "sift.data.v2.Int32Values"
    INT_64 = "sift.data.v2.Int64Values"
    UINT_32 = "sift.data.v2.Uint32Values"
    UINT_64 = "sift.data.v2.Uint64Values"


# Enum for channel data types (mimics protobuf values, but as int for now)
class ChannelDataType(int, Enum):
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

    @staticmethod
    def from_str(raw: str) -> Optional["ChannelDataType"]:
        if raw.startswith("CHANNEL_DATA_TYPE_"):
            val = ChannelDataTypeStrRep.from_api_format(raw)
            if val is None:
                return None
            for item in ChannelDataType:
                if item.name == val.name:
                    return item
            raise Exception(
                "Unreachable. ChannelDataTypeStrRep and ChannelDataType enum names are out of sync."
            )
        elif raw.startswith("sift.data"):
            val = ChannelTypeUrls(raw)
            if val is None:
                return None
            for item in ChannelDataType:
                if item.name == val.name:
                    return item
            raise Exception(
                "Unreachable. ChannelTypeUrls and ChannelDataType enum names are out of sync."
            )
        else:
            try:
                val = ChannelDataTypeStrRep(raw)
            except ValueError:
                return None

    @staticmethod
    def proto_data_class(data_type: ChannelDataType) -> Any:
        if data_type == ChannelDataType.DOUBLE:
            return DoubleValues
        elif data_type == ChannelDataType.FLOAT:
            return FloatValues
        elif data_type == ChannelDataType.STRING:
            return StringValues
        elif data_type == ChannelDataType.ENUM:
            return EnumValues
        elif data_type == ChannelDataType.BIT_FIELD:
            return BitFieldValues
        elif data_type == ChannelDataType.BOOL:
            return BoolValues
        elif data_type == ChannelDataType.INT_32:
            return Int32Values
        elif data_type == ChannelDataType.INT_64:
            return Int64Values
        elif data_type == ChannelDataType.UINT_32:
            return Uint32Values
        elif data_type == ChannelDataType.UINT_64:
            return Uint64Values
        else:
            raise ValueError(f"Unknown data type: {data_type}")


# Bit field element model
class ChannelBitFieldElement(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)
    name: str
    index: int
    bit_count: int

    @classmethod
    def _from_proto(cls, message: ChannelBitFieldElementPb) -> ChannelBitFieldElement:
        return cls(
            name=message.name,
            index=message.index,
            bit_count=message.bit_count,
        )


# Enum type model
class ChannelEnumType(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)
    name: str
    key: int

    @classmethod
    def _from_proto(cls, message: ChannelEnumTypePb) -> ChannelEnumType:
        return cls(
            name=message.name,
            key=message.key,
        )


# Channel config model
# TODO: Make this a BaseType? with container of ChannelValue's
class Channel(BaseType[ChannelProto, "Channel"]):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    id: str
    name: str
    data_type: ChannelDataType
    description: Optional[str] = None
    unit: Optional[str] = None
    component: Optional[str] = None  # Deprecated
    bit_field_elements: List[ChannelBitFieldElement] | None = None
    enum_types: List[ChannelEnumType] | None = None
    asset_id: Optional[str] = None

    @property
    def identifier(self) -> str:
        return self.fqn()

    def fqn(self) -> str:
        """
        The fully-qualified channel name of a channel called 'voltage' is simply `voltage`. The
        fully qualified name of a channel called 'temperature' of component 'motor' is a `motor.temperature'.
        """
        return channel_fqn(self)

    @classmethod
    def _from_proto(cls, message: ChannelProto) -> Channel:
        if isinstance(message, ChannelProto):  # ChannelProto from channels v3
            return cls(
                id=message.channel_id,
                name=message.name,
                data_type=ChannelDataType(message.data_type),
                description=message.description,
                unit=message.unit_id,
                bit_field_elements=[
                    ChannelBitFieldElement._from_proto(el) for el in message.bit_field_elements
                ],
                enum_types=[ChannelEnumType._from_proto(etype) for etype in message.enum_types],
                asset_id=message.asset_id,
            )
        else:
            raise ValueError(f"Unknown channel type: {type(message)}")

    def data(
        self,
        *,
        run_id: str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
        limit: int | None = None,
    ):
        """
        Retrieve channel data for this channel during the specified run.

        Args:
            run_id: The run ID to get data for.
            start_time: The start time to get data for.
            end_time: The end time to get data for.
            limit: The maximum number of data points to return.

        Returns:
            A ChannelTimeSeries object.
        """
        # TODO: Implement caching
        return self.client.channels.get_data(
            channels=[self.id], run_id=run_id, start_time=start_time, end_time=end_time, limit=limit
        )


class ChannelTimeSeries:
    data_type: ChannelDataType
    time_column: List[pd.Timestamp]
    value_column: List[Any]

    def __init__(
        self,
        data_type: ChannelDataType,
        time_column: List[pd.Timestamp],
        value_column: List[Any],
    ):
        if len(time_column) != len(value_column):
            raise Exception("Both arguments, `time_column` and `value_column` must equal lengths.")

        self.data_type = data_type
        self.time_column = time_column
        self.value_column = value_column

    def sort_time_series(self):
        points = [(t, v) for t, v in zip(self.time_column, self.value_column)]
        points.sort(key=lambda x: x[0])

        time_column = []
        value_column = []

        for ts, val in points:
            time_column.append(ts)
            value_column.append(val)

        self.time_column = time_column
        self.value_column = value_column


# Utility function for fully qualified channel name
def channel_fqn(
    channel: Union[
        Channel,
        # ChannelValue,
        # _AbstractChannel,
    ],
) -> str:
    name = channel.name
    component = getattr(channel, "component", None)
    if component:
        warnings.warn(
            "`component` is deprecated. This function should only be used for compatibility with legacy code.",
            DeprecationWarning,  # Warning ignored by default
        )
        return f"{component}.{name}"
    return name or ""


class ChannelReference(BaseModel):
    """
    Channel reference for calculated channel or rule.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    channel_reference: str  # The key of the channel in the expression i.e. $1, $2, etc.
    channel_identifier: str  # The name of the channel

    @classmethod
    def _from_proto(cls, proto) -> ChannelReference:
        return cls(
            channel_reference=proto.channel_reference,
            channel_identifier=proto.channel_identifier,
        )
