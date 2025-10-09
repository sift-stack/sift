from __future__ import annotations

import math
from typing import TYPE_CHECKING, Any

from google.protobuf.empty_pb2 import Empty
from pydantic import ConfigDict, model_validator
from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataChannelValue
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    ChannelConfig as ChannelConfigProto,
)
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
    FlowConfigPy,
    IngestWithConfigDataChannelValuePy,
)

from sift_client.sift_types._base import BaseType
from sift_client.sift_types.channel import ChannelBitFieldElement, ChannelDataType

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.channel import Channel


class IngestionConfig(BaseType[IngestionConfigProto, "IngestionConfig"]):
    """Model of the Sift Ingestion Config."""

    asset_id: str
    client_key: str

    @classmethod
    def _from_proto(
        cls, proto: IngestionConfigProto, sift_client: SiftClient | None = None
    ) -> IngestionConfig:
        return cls(
            proto=proto,
            id_=proto.ingestion_config_id,
            asset_id=proto.asset_id,
            client_key=proto.client_key,
            _client=sift_client,
        )


class ChannelConfig(BaseType[ChannelConfigProto, "ChannelConfig"]):
    """Channel configuration model for ingestion purposes.

    This model contains only the fields needed for ingestion configuration,
    without the full metadata from the Channels API.
    """

    model_config = ConfigDict(frozen=False)
    name: str
    data_type: ChannelDataType
    description: str | None = None
    unit: str | None = None
    bit_field_elements: list[ChannelBitFieldElement] | None = None
    enum_types: dict[str, int] | None = None

    @model_validator(mode="after")
    def _validate_enum_types(self):
        """Validate that enum_types is provided when data_type is ENUM."""
        if self.data_type == ChannelDataType.ENUM and not self.enum_types:
            raise ValueError(
                f"Channel '{self.name}' has data_type ENUM but enum_types is not provided"
            )
        elif (
            self.data_type == ChannelDataType.BIT_FIELD and not self.bit_field_elements
        ):
            raise ValueError(
                f"Channel '{self.name}' has data_type BIT_FIELD but bit_field_elements is not provided"
            )
        return self

    @classmethod
    def _from_proto(
        cls, proto: ChannelConfigProto, sift_client: SiftClient | None = None
    ) -> ChannelConfig:
        """Create ChannelConfig from ChannelConfigProto."""
        return cls(
            proto=proto,
            name=proto.name,
            data_type=ChannelDataType(proto.data_type),
            description=proto.description if proto.description else None,
            unit=proto.unit if proto.unit else None,
            bit_field_elements=(
                [
                    ChannelBitFieldElement._from_proto(el)
                    for el in proto.bit_field_elements
                ]
                if proto.bit_field_elements
                else None
            ),
            enum_types=(
                {enum.name: enum.key for enum in proto.enum_types}
                if proto.enum_types
                else None
            ),
            _client=sift_client,
        )

    @classmethod
    def from_channel(cls, channel: Channel) -> ChannelConfig:
        """Create ChannelConfig from a Channel.

        Args:
            channel: The Channel to convert.

        Returns:
            A ChannelConfig with the channel's configuration data.
        """
        return cls(
            name=channel.name,
            data_type=channel.data_type,
            description=channel.description,
            unit=channel.unit,
            bit_field_elements=(
                channel.bit_field_elements if channel.bit_field_elements else None
            ),
            enum_types=channel.enum_types,
        )

    def _to_config_proto(self) -> ChannelConfigProto:
        """Convert to ChannelConfigProto for ingestion."""
        from sift.common.type.v1.channel_bit_field_element_pb2 import (
            ChannelBitFieldElement as ChannelBitFieldElementPb,
        )
        from sift.common.type.v1.channel_enum_type_pb2 import (
            ChannelEnumType as ChannelEnumTypePb,
        )

        return ChannelConfigProto(
            name=self.name,
            data_type=self.data_type.value,
            description=self.description or "",
            unit=self.unit or "",
            bit_field_elements=[
                ChannelBitFieldElementPb(
                    name=bfe.name,
                    index=bfe.index,
                    bit_count=bfe.bit_count,
                )
                for bfe in self.bit_field_elements or []
            ],
            enum_types=[
                ChannelEnumTypePb(name=name, key=key)
                for name, key in (self.enum_types or {}).items()
            ],
        )


class Flow(BaseType[FlowConfig, "Flow"]):
    """Model representing a data flow for ingestion.

    A Flow represents a collection of channels that are ingested together.
    """

    model_config = ConfigDict(frozen=False)
    name: str
    channels: list[ChannelConfig]
    ingestion_config_id: str | None = None
    run_id: str | None = None

    @classmethod
    def _from_proto(
        cls, proto: FlowConfig, sift_client: SiftClient | None = None
    ) -> Flow:
        return cls(
            proto=proto,
            name=proto.name,
            channels=[ChannelConfig._from_proto(channel) for channel in proto.channels],
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

    def add_channel(self, channel: ChannelConfig):
        """Add a ChannelConfig to this Flow.

        Args:
            channel: The ChannelConfig to add.

        Raises:
            ValueError: If the flow has already been created with an ingestion config.
        """
        if self.ingestion_config_id:
            raise ValueError("Cannot add a channel to a flow after creation")
        self.channels.append(channel)

    def ingest(self, *, timestamp: datetime, channel_values: dict[str, Any]):
        """Ingest data for this Flow.

        Args:
            timestamp: The timestamp of the data.
            channel_values: Dictionary mapping Channel names to their values.

        Raises:
            ValueError: If the ingestion config ID is not set.
        """
        if self.ingestion_config_id is None:
            raise ValueError("Ingestion config ID is not set.")
        self.client.async_.ingestion.ingest(
            flow=self,
            timestamp=timestamp,
            channel_values=channel_values,
        )


# Converter functions.
def _channel_to_rust_config(channel: ChannelConfig) -> ChannelConfigPy:
    return ChannelConfigPy(
        name=channel.name,
        data_type=_to_rust_type(channel.data_type),
        description=channel.description or "",
        unit=channel.unit or "",
        bit_field_elements=[
            ChannelBitFieldElementPy(
                name=bfe.name, index=bfe.index, bit_count=bfe.bit_count
            )
            for bfe in channel.bit_field_elements or []
        ],
        enum_types=(
            [
                ChannelEnumTypePy(key=enum_key, name=enum_name)
                for enum_name, enum_key in channel.enum_types.items()
            ]
            if channel.enum_types
            else []
        ),
    )


def _rust_channel_value_from_bitfield(
    channel: ChannelConfig, value: Any
) -> IngestWithConfigDataChannelValuePy:
    """Helper function to convert a bitfield value to a ChannelValuePy object.

    Args:
        channel: The channel object for the bitfield value.
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
        return IngestWithConfigDataChannelValuePy.bitfield(cast_value)

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
    return IngestWithConfigDataChannelValuePy.bitfield(byte_array)


def _to_rust_value(
    channel: ChannelConfig, value: Any
) -> IngestWithConfigDataChannelValuePy:
    if value is None:
        return IngestWithConfigDataChannelValuePy.empty()
    if channel.data_type == ChannelDataType.ENUM and channel.enum_types is not None:
        enum_name = value
        enum_val = channel.enum_types.get(enum_name)
        if enum_val is None:
            # Try to find the enum value by value instead of string.
            for enum_name, enum_key in (
                channel.enum_types.items() if channel.enum_types else []
            ):
                if enum_key == value:
                    enum_name = enum_name
                    enum_val = enum_key
                    break
        if enum_val is None:
            raise ValueError(
                f"Could not find enum value: {value} in enum options: {channel.enum_types}"
            )
        return IngestWithConfigDataChannelValuePy.enum_value(enum_val)
    elif channel.data_type == ChannelDataType.BIT_FIELD:
        return _rust_channel_value_from_bitfield(channel, value)
    elif channel.data_type == ChannelDataType.BOOL:
        return IngestWithConfigDataChannelValuePy.bool(value)
    elif channel.data_type == ChannelDataType.FLOAT:
        return IngestWithConfigDataChannelValuePy.float(value)
    elif channel.data_type == ChannelDataType.DOUBLE:
        return IngestWithConfigDataChannelValuePy.double(value)
    elif channel.data_type == ChannelDataType.INT_32:
        return IngestWithConfigDataChannelValuePy.int32(value)
    elif channel.data_type == ChannelDataType.INT_64:
        return IngestWithConfigDataChannelValuePy.int64(value)
    elif channel.data_type == ChannelDataType.UINT_32:
        return IngestWithConfigDataChannelValuePy.uint32(value)
    elif channel.data_type == ChannelDataType.UINT_64:
        return IngestWithConfigDataChannelValuePy.uint64(value)
    else:
        raise ValueError(f"Invalid data type: {channel.data_type}")


def _to_rust_type(data_type: ChannelDataType) -> ChannelDataTypePy:
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


def _to_ingestion_value(
    data_type: ChannelDataType, value: Any
) -> IngestWithConfigDataChannelValue:
    if value is None:
        return IngestWithConfigDataChannelValue(empty=Empty())
    ingestion_type_string = data_type.name.lower().replace("int_", "int")
    return IngestWithConfigDataChannelValue(**{ingestion_type_string: value})
