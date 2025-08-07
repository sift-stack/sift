from __future__ import annotations

import math
from datetime import datetime
from typing import TYPE_CHECKING, Any, List

from google.protobuf.empty_pb2 import Empty
from pydantic import ConfigDict
from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataChannelValue
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    FlowConfig,
)
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    IngestionConfig as IngestionConfigProto,
)
from sift_stream_bindings import (
    ChannelBitFieldElementPy,
    ChannelConfigPy,
    ChannelDataTypePy,
    ChannelEnumTypePy,
    ChannelValuePy,
    FlowConfigPy,
)

from sift_client.types._base import BaseType
from sift_client.types.channel import Channel, ChannelDataType

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class IngestionConfig(BaseType[IngestionConfigProto, "IngestionConfig"]):
    """
    Model of the Sift Ingestion Config.
    """

    id: str
    asset_id: str
    client_key: str

    @classmethod
    def _from_proto(
        cls, proto: IngestionConfigProto, sift_client: SiftClient | None = None
    ) -> "IngestionConfig":
        return cls(
            id=proto.ingestion_config_id,
            asset_id=proto.asset_id,
            client_key=proto.client_key,
            _client=sift_client,
        )


class Flow(BaseType[FlowConfig, "Flow"]):
    model_config = ConfigDict(frozen=False)
    name: str
    channels: List[Channel]
    ingestion_config_id: str | None = None
    run_id: str | None = None

    @classmethod
    def _from_proto(cls, proto: FlowConfig, sift_client: SiftClient | None = None) -> Flow:
        return cls(
            name=proto.name,
            channels=[Channel._from_proto(channel) for channel in proto.channels],
            _client=sift_client,
        )

    def _to_proto(self) -> FlowConfig:
        return FlowConfig(
            name=self.name,
            channels=[channel._to_config_proto() for channel in self.channels],
        )

    def _to_rust_config(self) -> FlowConfigPy:
        return FlowConfigPy(
            name=self.name,
            channels=[_channel_to_rust_config(channel) for channel in self.channels],
        )

    def add_channel(self, channel: Channel):
        if self.ingestion_config_id:
            # TODO: Do we allow this or not?
            raise ValueError("Cannot add a channel to a flow after creation")
        self.channels.append(channel)

    def ingest(self, *, timestamp: datetime, channel_values: dict[str, Any]):
        if self.ingestion_config_id is None:
            raise ValueError("Ingestion config ID is not set.")
        self.client.ingestion.ingest(
            flow=self,
            timestamp=timestamp,
            channel_values=channel_values,
        )


# Converter functions.
def _channel_to_rust_config(channel: Channel) -> ChannelConfigPy:
    return ChannelConfigPy(
        name=channel.name,
        data_type=_to_rust_type(channel.data_type),
        description=channel.description or "",
        unit=channel.unit or "",
        bit_field_elements=[
            ChannelBitFieldElementPy(name=bfe.name, index=bfe.index, bit_count=bfe.bit_count)
            for bfe in channel.bit_field_elements or []
        ],
        enum_types=[
            ChannelEnumTypePy(key=enum_key, name=enum_name)
            for enum_name, enum_key in channel.enum_types.items()
        ]
        if channel.enum_types
        else [],
    )


def _rust_channel_value_from_bitfield(channel: Channel, value: Any) -> ChannelValuePy:
    """Helper function to convert a bitfield value to a ChannelValuePy object.

    Args:
        value: The value to convert to a ChannelValuePy object.
            - A single int or bytes will be treated as representing bytes directly
            - Dicts or list of ints will be treated as representing individual bitfield elements.

    Returns:
        A ChannelValuePy object.
    """
    assert channel.bit_field_elements is not None
    # We expect individual ints or bytes to represent full bitfield values.
    if isinstance(value, bytes) or isinstance(value, int):
        cast_value = [value] if isinstance(value, int) else value
        return ChannelValuePy.bitfield(
            channel.name,
            cast_value,
        )

    # We expect a dict or list of ints to represent individual bitfield elements.
    list_value = value
    if isinstance(value, dict):
        list_value = [value[field.name] for field in channel.bit_field_elements]

    if len(list_value) != len(channel.bit_field_elements):
        raise ValueError(
            f"Expected number of values passed as list to match number of bit field elements  for {channel.name}, but got {len(list_value)}"
        )

    packed = 0
    for i, field in enumerate(channel.bit_field_elements):
        packed |= list_value[i] << field.bit_count
    byte_array = packed.to_bytes(math.ceil(packed.bit_length() / 8), "little")
    return ChannelValuePy.bitfield(
        channel.name,
        byte_array,
    )


def _to_rust_value(channel: Channel, value: Any) -> ChannelValuePy:
    if value is None:
        raise ValueError(
            "Sift rust bindings does not support None values. Expected all channels in flow to have a data point at same time."
        )
    if channel.data_type == ChannelDataType.ENUM:
        enum_name = value
        enum_val = channel.enum_types.get(enum_name)
        if enum_val is None:
            # Try to find the enum value by value instead of string.
            for enum_name, enum_key in channel.enum_types.items() if channel.enum_types else []:
                if enum_key == value:
                    enum_name = enum_name
                    enum_val = enum_key
                    break
        if enum_val is None:
            raise ValueError(
                f"Could not find enum value: {value} in enum options: {channel.enum_types}"
            )
        return ChannelValuePy.enum_value(channel.name, ChannelEnumTypePy(enum_name, enum_val))
    elif channel.data_type == ChannelDataType.BIT_FIELD:
        return _rust_channel_value_from_bitfield(channel, value)
    elif channel.data_type == ChannelDataType.BOOL:
        return ChannelValuePy.bool(channel.name, value)
    elif channel.data_type == ChannelDataType.FLOAT:
        return ChannelValuePy.float(channel.name, value)
    elif channel.data_type == ChannelDataType.DOUBLE:
        return ChannelValuePy.double(channel.name, value)
    elif channel.data_type == ChannelDataType.INT_32:
        return ChannelValuePy.int32(channel.name, value)
    elif channel.data_type == ChannelDataType.INT_64:
        return ChannelValuePy.int64(channel.name, value)
    elif channel.data_type == ChannelDataType.UINT_32:
        return ChannelValuePy.uint32(channel.name, value)
    elif channel.data_type == ChannelDataType.UINT_64:
        return ChannelValuePy.uint64(channel.name, value)
    else:
        raise ValueError(f"Invalid data type: {channel.data_type}")


def _to_rust_type(data_type: ChannelDataType) -> ChannelDataTypePy:
    # TODO: Make more elegant?
    if data_type == ChannelDataType.DOUBLE:
        return ChannelDataTypePy.Double
    elif data_type == ChannelDataType.FLOAT:
        return ChannelDataTypePy.Float
    elif data_type == ChannelDataType.STRING:
        return ChannelDataTypePy.String
    elif data_type == ChannelDataType.ENUM:
        return ChannelDataTypePy.Enum
    elif data_type == ChannelDataType.BIT_FIELD:
        return ChannelDataTypePy.BitField
    elif data_type == ChannelDataType.BOOL:
        return ChannelDataTypePy.Bool
    elif data_type == ChannelDataType.INT_32:
        return ChannelDataTypePy.Int32
    elif data_type == ChannelDataType.INT_64:
        return ChannelDataTypePy.Int64
    elif data_type == ChannelDataType.UINT_32:
        return ChannelDataTypePy.Uint32
    elif data_type == ChannelDataType.UINT_64:
        return ChannelDataTypePy.Uint64
    raise ValueError(f"Unknown data type: {data_type}")


def _to_ingestion_value(type: ChannelDataType, value: Any) -> IngestWithConfigDataChannelValue:
    if value is None:
        return IngestWithConfigDataChannelValue(empty=Empty())
    ingestion_type_string = type.name.lower().replace("int_", "int")
    return IngestWithConfigDataChannelValue(**{ingestion_type_string: value})
