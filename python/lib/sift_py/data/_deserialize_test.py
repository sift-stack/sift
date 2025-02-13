from google.protobuf.any_pb2 import Any
from sift.common.type.v1.channel_bit_field_element_pb2 import ChannelBitFieldElement
from sift.common.type.v1.channel_data_type_pb2 import (
    CHANNEL_DATA_TYPE_BIT_FIELD,
    CHANNEL_DATA_TYPE_DOUBLE,
)
from sift.data.v2.data_pb2 import (
    BitFieldElementValues,
    BitFieldValue,
    BitFieldValues,
    DoubleValue,
    DoubleValues,
    Metadata,
)

from sift_py._internal.time import to_timestamp_nanos, to_timestamp_pb
from sift_py.data._deserialize import try_deserialize_channel_data


def test_try_deserialize_channel_data_double():
    metadata = Metadata(
        data_type=CHANNEL_DATA_TYPE_DOUBLE, channel=Metadata.Channel(name="double-channel")
    )

    time_a = "2024-07-04T18:09:08.555-07:00"
    time_b = "2024-07-04T18:09:09.555-07:00"

    double_values = DoubleValues(
        metadata=metadata,
        values=[
            DoubleValue(
                timestamp=to_timestamp_pb(time_a),
                value=10,
            ),
            DoubleValue(
                timestamp=to_timestamp_pb(time_b),
                value=11,
            ),
        ],
    )

    raw_values = Any()
    raw_values.Pack(double_values)

    deserialized_data = try_deserialize_channel_data(raw_values)

    assert len(deserialized_data) == 1

    metadata, time_series = deserialized_data[0]

    assert metadata.data_type == CHANNEL_DATA_TYPE_DOUBLE
    assert metadata.channel.name == "double-channel"
    assert len(time_series.time_column) == 2
    assert len(time_series.value_column) == 2
    assert time_series.value_column[0] == 10
    assert time_series.value_column[1] == 11
    assert time_series.time_column[0] == to_timestamp_nanos(time_a)
    assert time_series.time_column[1] == to_timestamp_nanos(time_b)


def test_try_deserialize_channel_data_bit_field_elements():
    metadata = Metadata(
        data_type=CHANNEL_DATA_TYPE_BIT_FIELD,
        channel=Metadata.Channel(
            name="gpio",
            bit_field_elements=[
                ChannelBitFieldElement(
                    name="12v",
                    index=0,
                    bit_count=4,
                ),
                ChannelBitFieldElement(
                    name="heater",
                    index=4,
                    bit_count=4,
                ),
            ],
        ),
    )

    time_a = "2024-07-04T18:09:08.555-07:00"

    value_a = int("10000001", 2)
    value_b = int("11110001", 2)

    bit_field_values = BitFieldValues(
        metadata=metadata,
        values=[
            BitFieldElementValues(
                name="12v",
                values=[
                    BitFieldValue(
                        timestamp=to_timestamp_pb(time_a),
                        value=value_a,
                    )
                ],
            ),
            BitFieldElementValues(
                name="heater",
                values=[
                    BitFieldValue(
                        timestamp=to_timestamp_pb(time_a),
                        value=value_b,
                    )
                ],
            ),
        ],
    )

    raw_values = Any()
    raw_values.Pack(bit_field_values)

    deserialized_data = try_deserialize_channel_data(raw_values)

    assert len(deserialized_data) == 2

    metadata_12v, time_series_12v = deserialized_data[0]
    metadata_heater, time_series_heater = deserialized_data[1]

    assert metadata_12v.data_type == CHANNEL_DATA_TYPE_BIT_FIELD
    assert metadata_heater.data_type == CHANNEL_DATA_TYPE_BIT_FIELD

    assert metadata_12v.channel.name == "gpio.12v"
    assert metadata_heater.channel.name == "gpio.heater"

    assert len(time_series_12v.time_column) == 1
    assert len(time_series_12v.value_column) == 1
    assert len(time_series_heater.time_column) == 1
    assert len(time_series_heater.value_column) == 1

    assert time_series_12v.value_column[0] == value_a
    assert time_series_heater.value_column[0] == value_b
    assert time_series_12v.time_column[0] == to_timestamp_nanos(time_a)
    assert time_series_heater.time_column[0] == to_timestamp_nanos(time_a)
