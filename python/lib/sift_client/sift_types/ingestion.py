from __future__ import annotations

import logging
import math
from datetime import datetime, timezone
from typing import TYPE_CHECKING, Any

from pydantic import BaseModel, ConfigDict, Field, model_validator
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    ChannelConfig as ChannelConfigProto,
)
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    CreateIngestionConfigRequest as CreateIngestionConfigRequestProto,
)
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    FlowConfig as FlowConfigProto,
)
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    IngestionConfig as IngestionConfigProto,
)

from sift_client.sift_types._base import (
    BaseType,
    ModelCreate,
)
from sift_client.sift_types.channel import ChannelBitFieldElement, ChannelDataType

logger = logging.getLogger(__name__)

if TYPE_CHECKING:
    from sift_stream_bindings import (
        ChannelConfigPy,
        ChannelDataTypePy,
        FlowConfigPy,
        FlowPy,
        IngestionConfigFormPy,
        IngestWithConfigDataChannelValuePy,
    )

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


class IngestionConfigCreate(ModelCreate[CreateIngestionConfigRequestProto]):
    """Create model for IngestionConfig."""

    asset_name: str
    flows: list[FlowConfig] | None = None
    organization_id: str | None = None
    client_key: str | None = None

    def _get_proto_class(self) -> type[CreateIngestionConfigRequestProto]:
        return CreateIngestionConfigRequestProto

    def _to_rust_form(self) -> IngestionConfigFormPy:
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        from sift_stream_bindings import IngestionConfigFormPy

        # Imported here to avoid circular dependancy
        from sift_client._internal.low_level_wrappers.ingestion import _hash_flows

        if self.organization_id:
            logger.warning(
                "OrgId is ignored when passing an IngestionConfigCreate to the ingestion client"
            )

        if self.client_key:
            client_key = self.client_key
        else:
            client_key = _hash_flows(self.asset_name, self.flows or [])

        return IngestionConfigFormPy(
            asset_name=self.asset_name,
            flows=[flow_config._to_rust_config() for flow_config in self.flows]
            if self.flows
            else [],
            client_key=client_key,
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
        elif self.data_type == ChannelDataType.BIT_FIELD and not self.bit_field_elements:
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
                [ChannelBitFieldElement._from_proto(el) for el in proto.bit_field_elements]
                if proto.bit_field_elements
                else None
            ),
            enum_types=(
                {enum.name: enum.key for enum in proto.enum_types} if proto.enum_types else None
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
            bit_field_elements=(channel.bit_field_elements if channel.bit_field_elements else None),
            enum_types=channel.enum_types,
        )

    @classmethod
    def _from_rust_config(cls, channel_config_py: ChannelConfigPy) -> ChannelConfig:
        return ChannelConfig(
            name=channel_config_py.name,
            description=channel_config_py.description or None,
            unit=channel_config_py.unit or None,
            data_type=ChannelDataType._from_rust_type(channel_config_py.data_type),
            bit_field_elements=[
                ChannelBitFieldElement._from_rust_type(bfe)
                for bfe in channel_config_py.bit_field_elements
            ],
            enum_types={enum.name: enum.key for enum in channel_config_py.enum_types},
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

    def as_channel_value(self, value: Any) -> ChannelValue:
        """Create a ChannelValue from a value using this channel's configuration.

        Args:
            value: The value to wrap in a ChannelValue. The type should match
                this channel's data_type.

        Returns:
            A ChannelValue instance with this channel's name and data type,
            containing the provided value.
        """
        return ChannelValue(
            name=self.name,
            ty=self.data_type,
            value=value,
        )


class FlowConfig(BaseType[FlowConfigProto, "FlowConfig"]):
    """Model representing a data flow config for ingestion.

    A FlowConfig represents the configuration of a collection of channels that are ingested together.
    """

    model_config = ConfigDict(frozen=False)
    name: str
    channels: list[ChannelConfig]
    ingestion_config_id: str | None = None
    run_id: str | None = None

    @classmethod
    def _from_proto(
        cls, proto: FlowConfigProto, sift_client: SiftClient | None = None
    ) -> FlowConfig:
        return cls(
            proto=proto,
            name=proto.name,
            channels=[ChannelConfig._from_proto(channel) for channel in proto.channels],
            _client=sift_client,
        )

    @classmethod
    def _from_rust_config(cls, flow_config_py: FlowConfigPy) -> FlowConfig:
        return FlowConfig(
            name=flow_config_py.name,
            channels=[
                ChannelConfig._from_rust_config(channel) for channel in flow_config_py.channels
            ],
        )

    def _to_proto(self) -> FlowConfigProto:
        return FlowConfigProto(
            name=self.name,
            channels=[channel._to_config_proto() for channel in self.channels],
        )

    def _to_rust_config(self) -> FlowConfigPy:
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        from sift_stream_bindings import FlowConfigPy

        return FlowConfigPy(
            name=self.name,
            channels=[_channel_config_to_rust_config(channel) for channel in self.channels],
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

    def as_flow(self, *, timestamp: datetime | None = None, values: dict[str, Any]) -> Flow:
        """Create a Flow from this FlowConfig with the provided values.

        Args:
            timestamp: The timestamp for the flow. If None, uses the current UTC time.
            values: A dictionary mapping channel names to their values. Only channels
                present in this dictionary will be included in the resulting Flow.

        Returns:
            A Flow object with channel values created from the provided values dictionary.
        """
        # Get current timestamp ASAP if not provided
        timestamp = timestamp or datetime.now(timezone.utc)

        found_values: set[str] = set()
        channel_values = []
        for channel in self.channels:
            if channel.name in values:
                channel_values.append(channel.as_channel_value(values[channel.name]))
                found_values.add(channel.name)

        missing_values = values.keys() - found_values
        if missing_values:
            raise ValueError(
                f"Provided channel values which do not exist in the flow config: {missing_values}"
            )

        return Flow(
            flow=self.name,
            timestamp=timestamp,
            channel_values=channel_values,
        )


class Flow(BaseModel):
    """Model representing a data flow for ingestion.

    A Flow represents a collection of channels that are ingested together.

    A representation of the IngestWithConfigDataStreamRequest proto
    """

    model_config = ConfigDict(frozen=False)
    ingestion_config_id: str | None = None
    flow: str
    timestamp: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))
    channel_values: list[ChannelValue]
    run_id: str | None = None
    end_stream_on_validation_error: bool | None = None
    organization_id: str | None = None

    def _to_rust_form(self) -> FlowPy:
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        from sift_stream_bindings import FlowPy

        from sift_client._internal.low_level_wrappers.ingestion import _to_rust_py_timestamp

        return FlowPy(
            flow_name=self.flow,
            timestamp=_to_rust_py_timestamp(self.timestamp),
            values=[channel_value._to_rust_form() for channel_value in self.channel_values],
        )


class ChannelValue(BaseModel):
    """Model representing a channel value for ingestion.

    A ChannelValue represents the data of a channel to be ingested.
    """

    model_config = ConfigDict(frozen=False)
    name: str
    ty: ChannelDataType
    value: Any

    def _to_rust_form(self):
        """Convert this ChannelValue to its Rust form for ingestion."""
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        from sift_stream_bindings import ChannelValuePy, ValuePy

        if self.ty == ChannelDataType.BIT_FIELD:
            value_py = ValuePy.BitField(self.value)
        elif self.ty == ChannelDataType.ENUM:
            value_py = ValuePy.Enum(self.value)
        elif self.ty == ChannelDataType.BOOL:
            value_py = ValuePy.Bool(self.value)
        elif self.ty == ChannelDataType.FLOAT:
            value_py = ValuePy.Float(self.value)
        elif self.ty == ChannelDataType.DOUBLE:
            value_py = ValuePy.Double(self.value)
        elif self.ty == ChannelDataType.INT_32:
            value_py = ValuePy.Int32(self.value)
        elif self.ty == ChannelDataType.INT_64:
            value_py = ValuePy.Int64(self.value)
        elif self.ty == ChannelDataType.UINT_32:
            value_py = ValuePy.Uint32(self.value)
        elif self.ty == ChannelDataType.UINT_64:
            value_py = ValuePy.Uint64(self.value)
        elif self.ty == ChannelDataType.STRING:
            value_py = ValuePy.String(self.value)
        else:
            raise ValueError(f"Invalid data type: {self.ty}")

        return ChannelValuePy(
            name=self.name,
            value=value_py,
        )


# Converter functions.
def _channel_config_to_rust_config(channel: ChannelConfig) -> ChannelConfigPy:
    # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
    from sift_stream_bindings import (
        ChannelBitFieldElementPy,
        ChannelConfigPy,
        ChannelEnumTypePy,
    )

    return ChannelConfigPy(
        name=channel.name,
        data_type=_to_rust_type(channel.data_type),
        description=channel.description or "",
        unit=channel.unit or "",
        bit_field_elements=[
            ChannelBitFieldElementPy(name=bfe.name, index=bfe.index, bit_count=bfe.bit_count)
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
    # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
    from sift_stream_bindings import IngestWithConfigDataChannelValuePy

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


def _to_rust_value(channel: ChannelConfig, value: Any) -> IngestWithConfigDataChannelValuePy:
    # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
    from sift_stream_bindings import IngestWithConfigDataChannelValuePy

    if value is None:
        return IngestWithConfigDataChannelValuePy.empty()
    if channel.data_type == ChannelDataType.ENUM and channel.enum_types is not None:
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
    # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
    from sift_stream_bindings import ChannelDataTypePy

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
