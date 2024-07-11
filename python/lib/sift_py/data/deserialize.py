from copy import deepcopy
from enum import Enum
from typing import List, Tuple, cast

from google.protobuf.any_pb2 import Any
from sift.data.v1.data_pb2 import (
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


def try_deserialize_channel_data(channel_values: Any) -> List[Tuple[Metadata, ChannelTimeSeries]]:
    if channel_values.type_url == ChannelValues.DOUBLE_VALUES.value:
        double_values = cast(DoubleValues, DoubleValues.FromString(channel_values.value))
        metadata = double_values.metadata

        time_column = []
        double_value_column = []

        for v in double_values.values:
            time_column.append(to_datetime(v.timestamp))
            double_value_column.append(v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, double_value_column
        )
        return [(metadata, time_series)]

    elif channel_values.type_url == ChannelValues.FLOAT_VALUES.value:
        float_values = cast(FloatValues, FloatValues.FromString(channel_values.value))
        metadata = float_values.metadata

        time_column = []
        float_value_column = []

        for float_v in float_values.values:
            time_column.append(to_datetime(float_v.timestamp))
            float_value_column.append(float_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, float_value_column
        )
        return [(metadata, time_series)]

    elif channel_values.type_url == ChannelValues.STRING_VALUES.value:
        string_values = cast(StringValues, StringValues.FromString(channel_values.value))
        metadata = string_values.metadata

        time_column = []
        string_value_column = []

        for string_v in string_values.values:
            time_column.append(to_datetime(string_v.timestamp))
            string_value_column.append(string_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, string_value_column
        )
        return [(metadata, time_series)]

    elif channel_values.type_url == ChannelValues.ENUM_VALUES.value:
        enum_values = cast(EnumValues, EnumValues.FromString(channel_values.value))
        metadata = enum_values.metadata

        time_column = []
        enum_value_column = []

        for enum_v in enum_values.values:
            time_column.append(to_datetime(enum_v.timestamp))
            enum_value_column.append(enum_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, enum_value_column
        )
        return [(metadata, time_series)]

    elif channel_values.type_url == ChannelValues.BOOL_VALUES.value:
        bool_values = cast(BoolValues, BoolValues.FromString(channel_values.value))
        metadata = bool_values.metadata

        time_column = []
        bool_value_column = []

        for bool_v in bool_values.values:
            time_column.append(to_datetime(bool_v.timestamp))
            bool_value_column.append(bool_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, bool_value_column
        )
        return [(metadata, time_series)]

    elif channel_values.type_url == ChannelValues.INT32_VALUES.value:
        int32_values = cast(Int32Values, Int32Values.FromString(channel_values.value))
        metadata = int32_values.metadata

        time_column = []
        int32_value_column = []

        for int32_v in int32_values.values:
            time_column.append(to_datetime(int32_v.timestamp))
            int32_value_column.append(int32_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, int32_value_column
        )
        return [(metadata, time_series)]

    elif channel_values.type_url == ChannelValues.INT64_VALUES.value:
        int64_values = cast(Int64Values, Int64Values.FromString(channel_values.value))
        metadata = int64_values.metadata

        time_column = []
        int64_value_column = []

        for int64_v in int64_values.values:
            time_column.append(to_datetime(int64_v.timestamp))
            int64_value_column.append(int64_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, int64_value_column
        )
        return [(metadata, time_series)]

    elif channel_values.type_url == ChannelValues.UINT32_VALUES.value:
        uint32_values = cast(Uint32Values, Uint32Values.FromString(channel_values.value))
        metadata = uint32_values.metadata

        time_column = []
        uint32_value_column = []

        for uint32_v in uint32_values.values:
            time_column.append(to_datetime(uint32_v.timestamp))
            uint32_value_column.append(uint32_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, uint32_value_column
        )
        return [(metadata, time_series)]

    elif channel_values.type_url == ChannelValues.UINT64_VALUES.value:
        uint64_values = cast(Uint64Values, Uint64Values.FromString(channel_values.value))
        metadata = uint64_values.metadata

        time_column = []
        uint64_value_column = []

        for uint64_v in uint64_values.values:
            time_column.append(to_datetime(uint64_v.timestamp))
            uint64_value_column.append(uint64_v.value)

        time_series = ChannelTimeSeries(
            ChannelDataType.from_pb(metadata.data_type), time_column, uint64_value_column
        )
        return [(metadata, time_series)]

    elif channel_values.type_url == ChannelValues.BIT_FIELD_VALUES.value:
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
                time_column.append(to_datetime(bf_v.timestamp))
                bit_field_el_column.append(bf_v.value)

            time_series = ChannelTimeSeries(data_type, time_column, bit_field_el_column)
            parsed_data.append((md_copy, time_series))

        return parsed_data

    raise SiftError("Received an unknown channel-type.")
