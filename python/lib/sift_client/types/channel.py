from __future__ import annotations

import warnings
from datetime import datetime
from enum import Enum
from typing import TYPE_CHECKING, Any, List, Optional, TypedDict

import sift.common.type.v1.channel_data_type_pb2 as channel_pb
from google.protobuf.empty_pb2 import Empty
from pydantic import BaseModel, ConfigDict
from sift.channels.v3.channels_pb2 import Channel as ChannelProto
from sift.common.type.v1.channel_bit_field_element_pb2 import (
    ChannelBitFieldElement as ChannelBitFieldElementPb,
)
from sift.common.type.v1.channel_enum_type_pb2 import ChannelEnumType as ChannelEnumTypePb
from sift.data.v2.data_pb2 import (
    BitFieldValues,
    BoolValues,
    DoubleValues,
    EnumValues,
    FloatValues,
    Int32Values,
    Int64Values,
    StringValues,
    Uint32Values,
    Uint64Values,
)
from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataChannelValue
from sift.ingestion_configs.v2.ingestion_configs_pb2 import ChannelConfig, FlowConfig
from sift_stream_bindings import (
    ChannelBitFieldElementPy,
    ChannelEnumTypePy,
    ChannelValuePy,
)

from sift_client.types._base import BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient


# TypedDicts for channel values
class ChannelValue(TypedDict, total=False):
    channel_name: str
    component: Optional[str]  # Deprecated
    value: int | float | bool | str | None


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
            val = ChannelTypeUrls(raw).value  # type: ignore # mypy doesn't understand scope
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
        raise Exception(f"Unknown channel data type: {raw}")

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

    @staticmethod
    def to_ingestion_value(type: ChannelDataType, value: Any) -> IngestWithConfigDataChannelValue:
        if value is None:
            return IngestWithConfigDataChannelValue(empty=Empty())
        ingestion_type_string = type.name.lower().replace("int_", "int")
        return IngestWithConfigDataChannelValue(**{ingestion_type_string: value})

    @staticmethod
    def to_rust_value(type: ChannelDataType, value: Any) -> ChannelValuePy:
        if value is None:
            return ChannelValuePy.empty()
        # TODO: Make more elegant?
        if type == ChannelDataType.DOUBLE:
            return ChannelValuePy.double(value)
        elif type == ChannelDataType.FLOAT:
            return ChannelValuePy.float(value)
        elif type == ChannelDataType.STRING:
            return ChannelValuePy.string(value)
        elif type == ChannelDataType.ENUM:
            return ChannelValuePy.enum_value(
                type.name, ChannelEnumTypePy(f"{type.value}", type.value)
            )
        elif type == ChannelDataType.BIT_FIELD:
            # TODO: fix
            return ChannelValuePy.bit_field(
                "bitfield type", ChannelBitFieldElementPy(name=type.name, index=0, bit_count=1)
            )
        elif type == ChannelDataType.BOOL:
            return ChannelValuePy.bool(value)
        elif type == ChannelDataType.INT_32:
            return ChannelValuePy.int32(value)
        elif type == ChannelDataType.INT_64:
            return ChannelValuePy.int64(value)
        elif type == ChannelDataType.UINT_32:
            return ChannelValuePy.uint32(value)
        elif type == ChannelDataType.UINT_64:
            return ChannelValuePy.uint64(value)


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

    def to_proto(self) -> ChannelBitFieldElementPb:
        return ChannelBitFieldElementPb(
            name=self.name,
            index=self.index,
            bit_count=self.bit_count,
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

    def to_proto(self) -> ChannelEnumTypePb:
        return ChannelEnumTypePb(
            name=self.name,
            key=self.key,
        )


# Channel config model
class Channel(BaseType[ChannelProto, "Channel"]):
    model_config = ConfigDict(arbitrary_types_allowed=True)

    id: str
    name: str
    data_type: ChannelDataType
    description: str | None = None
    unit: str | None = None
    component: str | None = None  # Deprecated
    bit_field_elements: List[ChannelBitFieldElement] | None = None
    enum_types: List[ChannelEnumType] | None = None  # TODO: Dict?
    asset_id: str | None = None

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
    def _from_proto(
        cls, message: ChannelProto | ChannelConfig, sift_client: SiftClient | None = None
    ) -> Channel:
        if isinstance(message, ChannelProto):
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
                _client=sift_client,
            )
        else:
            return cls(
                id=message.name,
                name=message.name,
                data_type=ChannelDataType(message.data_type),
                _client=sift_client,
            )

    def to_config_proto(self) -> ChannelConfig:
        return ChannelConfig(
            name=self.name,
            data_type=self.data_type.value,
            description=self.description,  # type: ignore
            unit=self.unit,  # type: ignore
            bit_field_elements=[el.to_proto() for el in self.bit_field_elements]
            if self.bit_field_elements
            else None,
            enum_types=[etype.to_proto() for etype in self.enum_types if etype is not None]
            if self.enum_types
            else None,
        )

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
        data = self.client.channels.get_data(
            channels=[self.id], run_id=run_id, start_time=start_time, end_time=end_time, limit=limit
        )
        return data


# Utility function for fully qualified channel name
def channel_fqn(
    channel: Channel,
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

    channel_reference: str  # The key of the channel in the expression i.e. $1, $2, etc.
    channel_identifier: str  # The name of the channel

    @classmethod
    def _from_proto(cls, proto) -> ChannelReference:
        return cls(
            channel_reference=proto.channel_reference,
            channel_identifier=proto.channel_identifier,
        )


class Flow(BaseType[FlowConfig, "Flow"]):
    model_config = ConfigDict(frozen=False, arbitrary_types_allowed=True)
    name: str
    channels: List[Channel]
    ingestion_config_id: str | None = None

    @classmethod
    def _from_proto(cls, proto, sift_client: SiftClient | None = None) -> Flow:
        return cls(
            name=proto.name,
            channels=[Channel._from_proto(channel) for channel in proto.channels],
            _client=sift_client,
        )

    def to_proto(self) -> FlowConfig:
        return FlowConfig(
            name=self.name,
            channels=[channel.to_config_proto() for channel in self.channels],
        )

    def add_channel(self, channel: Channel):
        if self.ingestion_config_id:
            # TODO: Do we allow this or not?
            raise ValueError("Cannot add a channel to a flow after creation")
        self.channels.append(channel)

    # TODO: Make this async
    def ingest(self, *, time: datetime, channel_values: dict[str, Any]):
        if self.ingestion_config_id is None:
            raise ValueError("Ingestion config ID is not set.")
        self.client.ingestion.ingest(
            flow=self,
            time=time,
            channel_values=channel_values,
        )
