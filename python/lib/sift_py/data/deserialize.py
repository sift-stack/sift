from enum import Enum
from typing import Tuple, cast

from google.protobuf.any_pb2 import Any
from sift.data.v1.data_pb2 import (
    BoolValues,
    DoubleValues,
    EnumValues,
    FloatValues,
    Int32Values,
    Int64Values,
    Metadata,
    StringValues,
    Uint32Values,
    Uint64Values,
)

from sift_py._internal.channel import to_datetime
from sift_py.data.channel import ChannelTimeSeries
from sift_py.error import SiftError
from sift_py.ingestion.channel import ChannelDataType


class ChannelValues(Enum):
    DOUBLE_VALUES = "sift.data.v1.DoubleValues"
    FLOAT_VALUES = "sift.data.v1.FloatValues"
    STRING_VALUES = "sift.data.v1.StringValues"
    ENUM_VALUES = "sift.data.v1.EnumValues"
    BIT_FIELD_VALUES = "sift.data.v1.BitFieldValues"
    BOOL_VALUES = "sift.data.v1.BoolValues"
    INT32_VALUES = "sift.data.v1.Int32Values"
    INT64_VALUES = "sift.data.v1.Int64Values"
    UINT32_VALUES = "sift.data.v1.Uint32Values"
    UINT64_VALUES = "sift.data.v1.Uint64Values"


def try_deserialize_channel_data(channel_values: Any) -> Tuple[Metadata, ChannelTimeSeries]:
    if channel_values.type_url == ChannelValues.DOUBLE_VALUES.value:
        double_values = cast(DoubleValues, DoubleValues.FromString(channel_values.value))
        metadata = double_values.metadata

        time_column = [to_datetime(v.timestamp) for v in double_values.values]
        double_value_column = [v.value for v in double_values.values]

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, double_value_column
        )
        return metadata, time_series

    elif channel_values.type_url == ChannelValues.FLOAT_VALUES.value:
        float_values = cast(FloatValues, FloatValues.FromString(channel_values.value))
        metadata = float_values.metadata

        time_column = [to_datetime(v.timestamp) for v in float_values.values]
        float_value_column = [v.value for v in float_values.values]

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, float_value_column
        )
        return metadata, time_series

    elif channel_values.type_url == ChannelValues.STRING_VALUES.value:
        string_values = cast(StringValues, StringValues.FromString(channel_values.value))
        metadata = string_values.metadata

        time_column = [to_datetime(v.timestamp) for v in string_values.values]
        string_value_column = [v.value for v in string_values.values]

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, string_value_column
        )
        return metadata, time_series

    elif channel_values.type_url == ChannelValues.ENUM_VALUES.value:
        enum_values = cast(EnumValues, EnumValues.FromString(channel_values.value))
        metadata = enum_values.metadata

        time_column = [to_datetime(v.timestamp) for v in enum_values.values]
        enum_value_column = [v.value for v in enum_values.values]

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, enum_value_column
        )
        return metadata, time_series

    elif channel_values.type_url == ChannelValues.BOOL_VALUES.value:
        bool_values = cast(BoolValues, BoolValues.FromString(channel_values.value))
        metadata = bool_values.metadata

        time_column = [to_datetime(v.timestamp) for v in bool_values.values]
        bool_value_column = [v.value for v in bool_values.values]

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, bool_value_column
        )
        return metadata, time_series

    elif channel_values.type_url == ChannelValues.INT32_VALUES.value:
        int32_values = cast(Int32Values, Int32Values.FromString(channel_values.value))
        metadata = int32_values.metadata

        time_column = [to_datetime(v.timestamp) for v in int32_values.values]
        int32_value_column = [v.value for v in int32_values.values]

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, int32_value_column
        )
        return metadata, time_series

    elif channel_values.type_url == ChannelValues.INT64_VALUES.value:
        int64_values = cast(Int64Values, Int64Values.FromString(channel_values.value))
        metadata = int64_values.metadata

        time_column = [to_datetime(v.timestamp) for v in int64_values.values]
        int64_value_column = [v.value for v in int64_values.values]

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, int64_value_column
        )
        return metadata, time_series

    elif channel_values.type_url == ChannelValues.UINT32_VALUES.value:
        uint32_values = cast(Uint32Values, Uint32Values.FromString(channel_values.value))
        metadata = uint32_values.metadata

        time_column = [to_datetime(v.timestamp) for v in uint32_values.values]
        uint32_value_column = [v.value for v in uint32_values.values]

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, uint32_value_column
        )
        return metadata, time_series

    elif channel_values.type_url == ChannelValues.UINT64_VALUES.value:
        uint64_values = cast(Uint64Values, Uint64Values.FromString(channel_values.value))
        metadata = uint64_values.metadata

        time_column = [to_datetime(v.timestamp) for v in uint64_values.values]
        uint64_value_column = [v.value for v in uint64_values.values]

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, uint64_value_column
        )
        return metadata, time_series

    elif channel_values.type_url == ChannelValues.BIT_FIELD_VALUES.value:
        # Handle deserialization for BIT_FIELD_VALUES
        pass

    raise SiftError("Received an unknown channel-type.")
