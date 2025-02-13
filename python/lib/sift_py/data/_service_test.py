from contextlib import contextmanager
from datetime import datetime, timedelta, timezone
from typing import Dict, Iterator

import pytest
from google.protobuf.any_pb2 import Any
from pytest_mock import MockFixture, MockType
from sift.assets.v1.assets_pb2 import Asset
from sift.channels.v3.channels_pb2 import Channel
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
from sift.runs.v2.runs_pb2 import Run

from sift_py._internal.test_util.channel import MockAsyncChannel
from sift_py._internal.time import to_timestamp_pb
from sift_py.data.query import ChannelQuery, DataQuery
from sift_py.data.service import DataService
from sift_py.error import SiftAPIDeprecationWarning


@pytest.mark.asyncio
async def test_data_service_execute_regular_channels(mocker: MockFixture):
    with patch_grpc_calls_channels(mocker) as mocks:
        channel = MockAsyncChannel()
        data_service = DataService(channel)

        start_time = datetime.now(timezone.utc)
        end_time = start_time + timedelta(minutes=2)

        with pytest.warns(SiftAPIDeprecationWarning, match="component"):
            chan_with_component = ChannelQuery(
                channel_name="velocity",
                component="mainmotor",
                run_name="[NostromoLV426].1720141748.047512",
            )

        query = DataQuery(
            asset_name="NostromoLV428",
            start_time=start_time,
            end_time=end_time,
            sample_ms=0,
            channels=[
                chan_with_component,
                ChannelQuery(
                    channel_name="gpio",
                    run_name="[NostromoLV426].1720141748.047512",
                ),
                ChannelQuery(
                    channel_name="valve.pressure",
                    run_name="[NostromoLV426].1720141748.047512",
                ),
            ],
        )

        result = await data_service.execute(query)

        mock_get_asset = mocks["mock_get_asset_by_name"]
        mock_get_channels = mocks["mock_get_channels_by_asset_id"]
        mock_get_runs = mocks["mock_get_runs_by_names"]

        mock_get_asset.assert_called_once()
        mock_get_channels.assert_called_once()
        mock_get_runs.assert_called_once()

        # bit field elements count as separate channels
        assert len(result.all_channels()) == 4
        assert not result.channel("velocity")
        assert not result.channels("velocity")
        assert len(result.channels("mainmotor.velocity")) == 1

        velocity = result.channel("mainmotor.velocity")
        assert velocity is not None
        assert len(velocity.timestamps) == 2
        assert len(velocity.time_column()["time"]) == 2
        assert len(velocity.time_column("custom_column_name")["custom_column_name"]) == 2
        assert len(velocity.value_column()["mainmotor.velocity"]) == 2
        assert len(velocity.value_column("custom_column_name")["custom_column_name"]) == 2

        all_columns = velocity.columns()
        assert len(all_columns) == 2
        assert len(all_columns["time"]) == 2
        assert len(all_columns["mainmotor.velocity"]) == 2

        all_columns_custom = velocity.columns(
            time_column_name="ts",
            value_column_name="velocity",
        )
        assert len(all_columns_custom) == 2
        assert len(all_columns_custom["ts"]) == 2
        assert len(all_columns_custom["velocity"]) == 2

        gpio = result.channel("gpio")
        assert not gpio

        gpio_12v = result.channel("gpio.12v")
        assert gpio_12v is not None
        assert len(gpio_12v.timestamps) == 1
        assert len(gpio_12v.time_column()["time"]) == 1
        assert len(gpio_12v.time_column("custom_column_name")["custom_column_name"]) == 1
        assert len(gpio_12v.value_column()["gpio.12v"]) == 1
        assert len(gpio_12v.value_column("12v")["12v"]) == 1

        gpio_heater = result.channel("gpio.heater")
        assert gpio_heater is not None
        assert len(gpio_heater.timestamps) == 1
        assert len(gpio_heater.time_column()["time"]) == 1
        assert len(gpio_heater.time_column("custom_column_name")["custom_column_name"]) == 1
        assert len(gpio_heater.value_column()["gpio.heater"]) == 1
        assert len(gpio_heater.value_column("heater")["heater"]) == 1

        pressure = result.channel("valve.pressure")
        assert pressure is not None
        assert len(pressure.timestamps) == 2
        assert len(pressure.time_column()["time"]) == 2
        assert len(pressure.time_column("custom_column_name")["custom_column_name"]) == 2
        assert len(pressure.value_column()["valve.pressure"]) == 2
        assert len(pressure.value_column("custom_column_name")["custom_column_name"]) == 2

        all_columns = pressure.columns()
        assert len(all_columns) == 2
        assert len(all_columns["time"]) == 2
        assert len(all_columns["valve.pressure"]) == 2

        all_columns_custom = pressure.columns(
            time_column_name="ts",
            value_column_name="valve.pressure",
        )
        assert len(all_columns_custom) == 2
        assert len(all_columns_custom["ts"]) == 2
        assert len(all_columns_custom["valve.pressure"]) == 2


@contextmanager
def patch_grpc_calls_channels(mocker: MockFixture) -> Iterator[Dict[str, MockType]]:
    mock__get_asset_by_name = mocker.patch.object(DataService, "_get_asset_by_name")
    mock__get_asset_by_name.return_value = Asset(
        asset_id="b7955799-9893-4acf-bf14-50052284020c", name="NostromoLV428"
    )

    mock__get_channels_by_asset_id = mocker.patch.object(DataService, "_get_channels_by_asset_id")
    mock__get_channels_by_asset_id.return_value = [
        Channel(
            channel_id="e8662647-12f7-465f-85dc-cb02513944e0",
            name="mainmotor.velocity",
            data_type=CHANNEL_DATA_TYPE_DOUBLE,
        ),
        Channel(
            channel_id="97e25141-ed3e-4538-b063-c3eac30838ce",
            name="gpio",
            data_type=CHANNEL_DATA_TYPE_BIT_FIELD,
        ),
        Channel(
            channel_id="87e25141-ed3e-4538-b063-c3eac30838cd",
            name="valve.pressure",
            data_type=CHANNEL_DATA_TYPE_DOUBLE,
        ),
    ]

    mock__get_runs_by_names = mocker.patch.object(DataService, "_get_runs_by_names")
    mock__get_runs_by_names.return_value = [
        Run(
            run_id="9b7f6c5f-cabc-4481-b048-6f12fc6b5b68",
            name="[NostromoLV426].1720141748.047512",
        )
    ]

    time_a = "2024-07-04T18:09:08.555-07:00"
    time_b = "2024-07-04T18:09:09.555-07:00"
    velocity_values = DoubleValues(
        metadata=Metadata(
            data_type=CHANNEL_DATA_TYPE_DOUBLE,
            channel=Metadata.Channel(name="mainmotor.velocity"),
        ),
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

    raw_velocity_values = Any()
    raw_velocity_values.Pack(velocity_values)

    time_a = "2024-07-04T18:09:08.555-07:00"
    time_b = "2024-07-04T18:09:09.555-07:00"

    pressure_values = DoubleValues(
        metadata=Metadata(
            data_type=CHANNEL_DATA_TYPE_DOUBLE,
            channel=Metadata.Channel(name="valve.pressure"),
        ),
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

    raw_pressure_values = Any()
    raw_pressure_values.Pack(pressure_values)

    bit_field_values = BitFieldValues(
        metadata=Metadata(
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
        ),
        values=[
            BitFieldElementValues(
                name="12v",
                values=[
                    BitFieldValue(
                        timestamp=to_timestamp_pb(time_a),
                        value=int("10000001", 2),
                    )
                ],
            ),
            BitFieldElementValues(
                name="heater",
                values=[
                    BitFieldValue(
                        timestamp=to_timestamp_pb(time_a),
                        value=int("11110001", 2),
                    )
                ],
            ),
        ],
    )

    raw_bit_field_values = Any()
    raw_bit_field_values.Pack(bit_field_values)

    mock__get_data = mocker.patch.object(DataService, "_get_data")
    mock__get_data.side_effect = [
        [[raw_velocity_values]],
        [[raw_bit_field_values]],
        [[raw_pressure_values]],
    ]
    yield {
        "mock_get_asset_by_name": mock__get_asset_by_name,
        "mock_get_runs_by_names": mock__get_runs_by_names,
        "mock_get_channels_by_asset_id": mock__get_channels_by_asset_id,
        "mock_get_data": mock__get_data,
    }
