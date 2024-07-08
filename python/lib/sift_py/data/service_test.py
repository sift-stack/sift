from contextlib import contextmanager
from datetime import datetime, timedelta, timezone

from pytest_mock import MockFixture
from sift.assets.v1.assets_pb2 import Asset
from sift.channels.v2.channels_pb2 import Channel
from sift.common.type.v1.channel_data_type_pb2 import (
    CHANNEL_DATA_TYPE_DOUBLE,
    CHANNEL_DATA_TYPE_ENUM,
)
from sift.runs.v2.runs_pb2 import Run

from sift_py._internal.test_util.channel import MockChannel
from sift_py.data.query import ChannelQuery, DataQuery
from sift_py.data.service import DataService


def test_data_service_execute(mocker: MockFixture):
    with patch_grpc_calls(mocker):
        channel = MockChannel()
        data_service = DataService(channel, {
            "uri": "http://localhost:50051",
            "apikey": "KUFfAn0ikCQipEDohjddPfw2bUSlPd2P9tGgzy1h",
        })

        start_time = datetime.now(timezone.utc)
        end_time = start_time + timedelta(minutes=2)

        query = DataQuery(
            asset_name="NostromoLV428",
            start_time=start_time,
            end_time=end_time,
            sample_ms=0,
            channels=[
                ChannelQuery(
                    channel_name="velocity",
                    component="mainmotor",
                    run_name="[NostromoLV426].1720141748.047512"
                ),
                ChannelQuery(
                    channel_name="vehicle_state",
                    run_name="[NostromoLV426].1720141748.047512"
                ),
            ]
        )

        data_service.execute(query)

@contextmanager
def patch_grpc_calls(mocker: MockFixture):
    mock__get_asset_by_name = mocker.patch.object(DataService, "_get_asset_by_name")
    mock__get_asset_by_name.return_value = Asset(
        asset_id="b7955799-9893-4acf-bf14-50052284020c",
        name="NostromoLV428"
    )

    mock__get_channels_by_asset_id = mocker.patch.object(DataService, "_get_channels_by_asset_id")
    mock__get_channels_by_asset_id.return_value = [
        Channel(
            channel_id="e8662647-12f7-465f-85dc-cb02513944e0",
            name="velocity",
            component="mainmotor",
            data_type=CHANNEL_DATA_TYPE_DOUBLE,
        ),
        Channel(
            channel_id="97e25141-ed3e-4538-b063-c3eac30838ce",
            name="vehicle_state",
            data_type=CHANNEL_DATA_TYPE_ENUM,
        ),
    ]

    mock__get_runs_by_names = mocker.patch.object(DataService, "_get_runs_by_names")
    mock__get_runs_by_names.return_value = [
        Run(
            run_id="9b7f6c5f-cabc-4481-b048-6f12fc6b5b68",
            name="[NostromoLV426].1720141748.047512",
        )
    ]
    yield
