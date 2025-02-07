from copy import deepcopy
from enum import Enum
from typing import List, Tuple, cast

from google.protobuf.any_pb2 import Any
from sift.data.v2.data_pb2 import (
    BitFieldValues,
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

from sift_py._internal.time import to_timestamp_nanos
from sift_py.data._channel import ChannelTimeSeries
from sift_py.error import SiftError
from sift_py.ingestion.channel import ChannelDataType


class ChannelValues(Enum):
    DOUBLE_VALUES = "sift.data.v2.DoubleValues"
    FLOAT_VALUES = "sift.data.v2.FloatValues"
    STRING_VALUES = "sift.data.v2.StringValues"
    ENUM_VALUES = "sift.data.v2.EnumValues"
    BIT_FIELD_VALUES = "sift.data.v2.BitFieldValues"
    BOOL_VALUES = "sift.data.v2.BoolValues"
    INT32_VALUES = "sift.data.v2.Int32Values"
    INT64_VALUES = "sift.data.v2.Int64Values"
    UINT32_VALUES = "sift.data.v2.Uint32Values"
    UINT64_VALUES = "sift.data.v2.Uint64Values"


def try_deserialize_channel_data(channel_values: Any) -> List[Tuple[Metadata, ChannelTimeSeries]]:
    if ChannelValues.DOUBLE_VALUES.value in channel_values.type_url:
        double_values = cast(DoubleValues, DoubleValues.FromString(channel_values.value))
        metadata = double_values.metadata

        time_column = []
        double_value_column = []

        for v in double_values.values:
            time_column.append(to_timestamp_nanos(v.timestamp))
            double_value_column.append(v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, double_value_column
        )
        return [(metadata, time_series)]

    elif ChannelValues.FLOAT_VALUES.value in channel_values.type_url:
        float_values = cast(FloatValues, FloatValues.FromString(channel_values.value))
        metadata = float_values.metadata

        time_column = []
        float_value_column = []

        for float_v in float_values.values:
            time_column.append(to_timestamp_nanos(float_v.timestamp))
            float_value_column.append(float_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, float_value_column
        )
        return [(metadata, time_series)]

    elif ChannelValues.STRING_VALUES.value in channel_values.type_url:
        string_values = cast(StringValues, StringValues.FromString(channel_values.value))
        metadata = string_values.metadata

        time_column = []
        string_value_column = []

        for string_v in string_values.values:
            time_column.append(to_timestamp_nanos(string_v.timestamp))
            string_value_column.append(string_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, string_value_column
        )
        return [(metadata, time_series)]

    elif ChannelValues.ENUM_VALUES.value in channel_values.type_url:
        enum_values = cast(EnumValues, EnumValues.FromString(channel_values.value))
        metadata = enum_values.metadata

        time_column = []
        enum_value_column = []

        for enum_v in enum_values.values:
            time_column.append(to_timestamp_nanos(enum_v.timestamp))
            enum_value_column.append(enum_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, enum_value_column
        )
        return [(metadata, time_series)]

    elif ChannelValues.BOOL_VALUES.value in channel_values.type_url:
        bool_values = cast(BoolValues, BoolValues.FromString(channel_values.value))
        metadata = bool_values.metadata

        time_column = []
        bool_value_column = []

        for bool_v in bool_values.values:
            time_column.append(to_timestamp_nanos(bool_v.timestamp))
            bool_value_column.append(bool_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, bool_value_column
        )
        return [(metadata, time_series)]

    elif ChannelValues.INT32_VALUES.value in channel_values.type_url:
        int32_values = cast(Int32Values, Int32Values.FromString(channel_values.value))
        metadata = int32_values.metadata

        time_column = []
        int32_value_column = []

        for int32_v in int32_values.values:
            time_column.append(to_timestamp_nanos(int32_v.timestamp))
            int32_value_column.append(int32_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, int32_value_column
        )
        return [(metadata, time_series)]

    elif ChannelValues.INT64_VALUES.value in channel_values.type_url:
        int64_values = cast(Int64Values, Int64Values.FromString(channel_values.value))
        metadata = int64_values.metadata

        time_column = []
        int64_value_column = []

        for int64_v in int64_values.values:
            time_column.append(to_timestamp_nanos(int64_v.timestamp))
            int64_value_column.append(int64_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, int64_value_column
        )
        return [(metadata, time_series)]

    elif ChannelValues.UINT32_VALUES.value in channel_values.type_url:
        uint32_values = cast(Uint32Values, Uint32Values.FromString(channel_values.value))
        metadata = uint32_values.metadata

        time_column = []
        uint32_value_column = []

        for uint32_v in uint32_values.values:
            time_column.append(to_timestamp_nanos(uint32_v.timestamp))
            uint32_value_column.append(uint32_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, uint32_value_column
        )
        return [(metadata, time_series)]

    elif ChannelValues.UINT64_VALUES.value in channel_values.type_url:
        uint64_values = cast(Uint64Values, Uint64Values.FromString(channel_values.value))
        metadata = uint64_values.metadata

        time_column = []
        uint64_value_column = []

        for uint64_v in uint64_values.values:
            time_column.append(to_timestamp_nanos(uint64_v.timestamp))
            uint64_value_column.append(uint64_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, uint64_value_column
        )
        return [(metadata, time_series)]

    elif ChannelValues.BIT_FIELD_VALUES.value in channel_values.type_url:
        bit_field_values = cast(BitFieldValues, BitFieldValues.FromString(channel_values.value))
        metadata = bit_field_values.metadata
        data_type = ChannelDataType.from_pb(metadata.data_type)
        channel_name = metadata.channel.name

        parsed_data: List[Tuple[Metadata, ChannelTimeSeries]] = []

        for bit_field_element in bit_field_values.values:
            md_copy = deepcopy(bit_field_values.metadata)
            md_copy.channel.name = f"{channel_name}.{bit_field_element.name}"

            time_column = []
            bit_field_el_column = []

            for bf_v in bit_field_element.values:
                time_column.append(to_timestamp_nanos(bf_v.timestamp))
                bit_field_el_column.append(bf_v.value)

            time_series = ChannelTimeSeries(data_type, time_column, bit_field_el_column)
            parsed_data.append((md_copy, time_series))

        return parsed_data

    raise SiftError(f"Received an unknown channel-type '{channel_values.type_url}'.")
