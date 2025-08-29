from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, Any

import sift.common.type.v1.channel_data_type_pb2 as channel_pb
from pydantic import BaseModel
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
from sift.ingestion_configs.v2.ingestion_configs_pb2 import ChannelConfig

from sift_client.sift_types._base import BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset
    from sift_client.sift_types.run import Run


# Enum for channel data types (mimics protobuf values, but as int for now)
class ChannelDataType(Enum):
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
        for item in ChannelDataType:
            if "CHANNEL_DATA_TYPE_" + item.name == val:
                return item
        return None

    @staticmethod
    def from_str(raw: str) -> ChannelDataType | None:
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
        elif data_type == ChannelDataType.BYTES:
            return BytesValues
        else:
            raise ValueError(f"Unknown data type: {data_type}")

    # TODO: Can we get rid of this? Is hashing the same between clients that likely to ever actually discover a conflict?
    def hash_str(self, api_format: bool = False) -> str:
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


# Bit field element model
class ChannelBitFieldElement(BaseModel):
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

    def _to_proto(self) -> ChannelBitFieldElementPb:
        return ChannelBitFieldElementPb(
            name=self.name,
            index=self.index,
            bit_count=self.bit_count,
        )


# Channel config model
class Channel(BaseType[ChannelProto, "Channel"]):
    name: str
    data_type: ChannelDataType
    description: str | None = None
    unit: str | None = None
    bit_field_elements: list[ChannelBitFieldElement] | None = None
    enum_types: dict[str, int] | None = None
    asset_id: str | None = None
    created_date: datetime | None = None
    modified_date: datetime | None = None
    created_by_user_id: str | None = None
    modified_by_user_id: str | None = None

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
    def _from_proto(
        cls, proto: ChannelProto | ChannelConfig, sift_client: SiftClient | None = None
    ) -> Channel:
        if isinstance(proto, ChannelProto):
            return cls(
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
        elif isinstance(proto, ChannelConfig):
            return cls(
                id_=proto.name,
                name=proto.name,
                data_type=ChannelDataType(proto.data_type),
                _client=sift_client,
            )

    def _to_config_proto(self) -> ChannelConfig:
        return ChannelConfig(
            name=self.name,
            data_type=self.data_type.value,
            description=self.description,  # type: ignore
            unit=self.unit,  # type: ignore
            bit_field_elements=[el._to_proto() for el in self.bit_field_elements]
            if self.bit_field_elements
            else None,
            enum_types=self._enum_types_to_proto_list(self.enum_types),
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
        """
        Retrieve channel data for this channel during the specified run.

        Args:
            run_id: The run ID to get data for.
            start_time: The start time to get data for.
            end_time: The end time to get data for.
            limit: The maximum number of data points to return.

        Returns:
            A dict of channel name to pandas DataFrame or Arrow Table object.
        """
        if as_arrow:
            data = self.client.channels.get_data_as_arrow(
                channels=[self],
                run_id=run_id,
                start_time=start_time,
                end_time=end_time,
                limit=limit,  # type: ignore
            )
        else:
            data = self.client.channels.get_data(
                channels=[self],
                run_id=run_id,
                start_time=start_time,
                end_time=end_time,
                limit=limit,  # type: ignore
            )
        return data

    @property
    def asset(self) -> Asset:
        return self.client.assets.get(asset_id=self.asset_id)

    @property
    def runs(self) -> list[Run]:
        return self.asset.runs


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
