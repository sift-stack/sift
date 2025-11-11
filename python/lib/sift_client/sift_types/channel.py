from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING

import sift.common.type.v1.channel_data_type_pb2 as channel_pb
from pydantic import BaseModel, Field
from sift.channels.v3.channels_pb2 import Channel as ChannelProto
from sift.common.type.v1.channel_bit_field_element_pb2 import (
    ChannelBitFieldElement as ChannelBitFieldElementPb,
)
from sift.common.type.v1.channel_enum_type_pb2 import ChannelEnumType as ChannelEnumTypePb
from sift.data.v2.data_pb2 import (
    BitFieldValues,
    BoolValues,
    BytesValues,
    DoubleValues,
    EnumValues,
    FloatValues,
    Int32Values,
    Int64Values,
    StringValues,
    Uint32Values,
    Uint64Values,
)

from sift_client.sift_types._base import BaseType

if TYPE_CHECKING:
    from sift_stream_bindings import ChannelBitFieldElementPy, ChannelDataTypePy

    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset
    from sift_client.sift_types.run import Run


class ChannelDataType(Enum):
    """Enum for channel data types (mimics protobuf values, but as int for now)."""

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
    BYTES = channel_pb.CHANNEL_DATA_TYPE_BYTES

    def __str__(self) -> str:
        ret = self.name.lower()
        if "int" in ret:
            ret = ret.replace("int_", "int")
        return ret

    @staticmethod
    def from_api_format(val: str) -> ChannelDataType | None:
        """Convert API format string to ChannelDataType.

        Args:
            val: API format string representation of ChannelDataType.

        Returns:
            ChannelDataType if conversion is successful, None otherwise.
        """
        for item in ChannelDataType:
            if "CHANNEL_DATA_TYPE_" + item.name == val:
                return item
        return None

    @staticmethod
    def from_str(raw: str) -> ChannelDataType | None:
        """Convert string representation to ChannelDataType.

        Args:
            raw: String representation of ChannelDataType.

        Returns:
            ChannelDataType if conversion is successful, None otherwise.

        Raises:
            Exception: If the string format is recognized but cannot be converted.
        """
        if raw.startswith("CHANNEL_DATA_TYPE_"):
            val = ChannelDataType.from_api_format(raw)
            if val is None:
                return None
            for item in ChannelDataType:
                if item.name == val.name:
                    return item
            raise Exception(
                "Unreachable. ChannelDataTypeStrRep and ChannelDataType enum names are out of sync."
            )
        elif raw.startswith("sift.data"):
            for item in ChannelDataType:
                val = raw.split(".")[-1].lower().replace("values", "")  # type: ignore
                val = "bit_field" if val == "bitfield" else val  # type: ignore
                if item.__str__() == val:
                    return item
            raise Exception(
                f"{raw} type not found. ChannelTypeUrls and ChannelDataType enum names are out of sync."
            )
        else:
            try:
                for item in ChannelDataType:
                    if item.__str__() == raw.lower():
                        return item
            except ValueError:
                return None
        raise Exception(f"Unknown channel data type: {raw}")

    @staticmethod
    def _from_rust_type(channel_data_type_py: ChannelDataTypePy) -> ChannelDataType:
        # Use enum name for comparison to avoid PyO3 enum comparison issues
        # Extract the enum name from the string representation
        enum_str = str(channel_data_type_py)
        enum_name = enum_str.split(".")[-1] if "." in enum_str else enum_str

        mapping = {
            "Double": ChannelDataType.DOUBLE,
            "String": ChannelDataType.STRING,
            "Enum": ChannelDataType.ENUM,
            "BitField": ChannelDataType.BIT_FIELD,
            "Bool": ChannelDataType.BOOL,
            "Float": ChannelDataType.FLOAT,
            "Int32": ChannelDataType.INT_32,
            "Uint32": ChannelDataType.UINT_32,
            "Int64": ChannelDataType.INT_64,
            "Uint64": ChannelDataType.UINT_64,
            "Bytes": ChannelDataType.BYTES,
        }

        if enum_name in mapping:
            return mapping[enum_name]
        else:
            raise ValueError(f"Unknown channel data type: {channel_data_type_py}")

    @staticmethod
    def proto_data_class(data_type: ChannelDataType):
        """Return the appropriate protobuf class for the given channel data type.

        Args:
            data_type: The channel data type.

        Returns:
            The protobuf class corresponding to the data type.

        Raises:
            ValueError: If the data type is not recognized.
        """
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
        elif data_type == ChannelDataType.BYTES:
            return BytesValues
        else:
            raise ValueError(f"Unknown data type: {data_type}")

    # TODO: Can we get rid of this? Is hashing the same between clients that likely to ever actually discover a conflict?
    def hash_str(self, api_format: bool = False) -> str:
        """Get the hash string for this channel data type."""
        if self == ChannelDataType.DOUBLE:
            return "CHANNEL_DATA_TYPE_DOUBLE" if api_format else ChannelDataType.DOUBLE.__str__()
        elif self == ChannelDataType.STRING:
            return "CHANNEL_DATA_TYPE_STRING" if api_format else ChannelDataType.STRING.__str__()
        elif self == ChannelDataType.ENUM:
            return "CHANNEL_DATA_TYPE_ENUM" if api_format else ChannelDataType.ENUM.__str__()
        elif self == ChannelDataType.BIT_FIELD:
            return (
                "CHANNEL_DATA_TYPE_BIT_FIELD" if api_format else ChannelDataType.BIT_FIELD.__str__()
            )
        elif self == ChannelDataType.BOOL:
            return "CHANNEL_DATA_TYPE_BOOL" if api_format else ChannelDataType.BOOL.__str__()
        elif self == ChannelDataType.FLOAT:
            return "CHANNEL_DATA_TYPE_FLOAT" if api_format else ChannelDataType.FLOAT.__str__()
        elif self == ChannelDataType.INT_32:
            return "CHANNEL_DATA_TYPE_INT_32" if api_format else ChannelDataType.INT_32.__str__()
        elif self == ChannelDataType.INT_64:
            return "CHANNEL_DATA_TYPE_INT_64" if api_format else ChannelDataType.INT_64.__str__()
        elif self == ChannelDataType.UINT_32:
            return "CHANNEL_DATA_TYPE_UINT_32" if api_format else ChannelDataType.UINT_32.__str__()
        elif self == ChannelDataType.UINT_64:
            return "CHANNEL_DATA_TYPE_UINT_64" if api_format else ChannelDataType.UINT_64.__str__()
        elif self == ChannelDataType.BYTES:
            return "CHANNEL_DATA_TYPE_BYTES" if api_format else ChannelDataType.BYTES.__str__()
        else:
            raise Exception("Unreachable.")


class ChannelBitFieldElement(BaseModel):
    """Bit field element model."""

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

    @classmethod
    def _from_rust_type(
        cls, bit_field_element_py: ChannelBitFieldElementPy
    ) -> ChannelBitFieldElement:
        return ChannelBitFieldElement(
            name=bit_field_element_py.name,
            index=bit_field_element_py.index,
            bit_count=bit_field_element_py.bit_count,
        )

    def _to_proto(self) -> ChannelBitFieldElementPb:
        return ChannelBitFieldElementPb(
            name=self.name,
            index=self.index,
            bit_count=self.bit_count,
        )


# Channel config model
class Channel(BaseType[ChannelProto, "Channel"]):
    """Model representing a Sift Channel."""

    # Required fields
    name: str
    data_type: ChannelDataType
    description: str
    unit: str
    bit_field_elements: list[ChannelBitFieldElement] = Field(default_factory=list)
    enum_types: dict[str, int] = Field(default_factory=dict)
    asset_id: str
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str

    # Optional fields
    ...

    @staticmethod
    def _enum_types_to_proto_list(enum_types: dict[str, int] | None) -> list[ChannelEnumTypePb]:
        """Convert a dictionary of enum types to a list of ChannelEnumTypePb objects."""
        enum_types = {} if enum_types is None else enum_types
        return [ChannelEnumTypePb(name=name, key=key) for name, key in enum_types.items()]

    @staticmethod
    def _enum_types_from_proto_list(enum_types: list[ChannelEnumTypePb]) -> dict[str, int]:
        """Convert a list of ChannelEnumTypePb objects to a dictionary of enum types."""
        return {enum.name: enum.key for enum in enum_types}

    @classmethod
    def _from_proto(cls, proto: ChannelProto, sift_client: SiftClient | None = None) -> Channel:
        return cls(
            proto=proto,
            id_=proto.channel_id,
            name=proto.name,
            data_type=ChannelDataType(proto.data_type),
            description=proto.description,
            unit=proto.unit_id,
            bit_field_elements=[
                ChannelBitFieldElement._from_proto(el) for el in proto.bit_field_elements
            ],
            enum_types=cls._enum_types_from_proto_list(proto.enum_types),  # type: ignore
            asset_id=proto.asset_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            _client=sift_client,
        )

    def data(
        self,
        *,
        run_id: str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
        limit: int | None = None,
        as_arrow: bool = False,
    ):
        """Retrieve channel data for this channel during the specified run.

        Args:
            run_id: The run ID to get data for.
            start_time: The start time to get data for.
            end_time: The end time to get data for.
            limit: The maximum number of data points to return.
            as_arrow: Whether to return the data as an Arrow table.

        Returns:
            A dict of channel name to pandas DataFrame or Arrow Table object.
        """
        if as_arrow:
            data = self.client.channels.get_data_as_arrow(
                channels=[self],
                run=run_id,
                start_time=start_time,
                end_time=end_time,
                limit=limit,  # type: ignore
            )
        else:
            data = self.client.channels.get_data(
                channels=[self],
                run=run_id,
                start_time=start_time,
                end_time=end_time,
                limit=limit,  # type: ignore
            )
        return data

    @property
    def asset(self) -> Asset:
        """Get the asset that this channel belongs to."""
        return self.client.assets.get(asset_id=self.asset_id)

    # TODO: update this logic to correctly scope to only runs that this channel is associated with.
    @property
    def runs(self) -> list[Run]:
        """Get all runs associated with this channel's asset."""
        return self.asset.runs


class ChannelReference(BaseModel):
    """Channel reference for calculated channel or rule."""

    channel_reference: str  # The key of the channel in the expression i.e. $1, $2, etc.
    channel_identifier: str  # The name of the channel

    @classmethod
    def _from_proto(cls, proto) -> ChannelReference:
        return cls(
            channel_reference=proto.channel_reference,
            channel_identifier=proto.channel_identifier,
        )
