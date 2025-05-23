import os

from comparison_examples.util import use_sift_betterproto_channel
from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig

load_dotenv()
apikey = os.getenv("SIFT_API_KEY")
uri = os.getenv("BASE_URI")
channel_config: SiftChannelConfig = {
    "apikey": apikey,
    "uri": uri,
}

import asyncio

from betterproto2_lib.sift.assets.v1 import AssetServiceStub
from betterproto2_lib.sift.ping.v1 import PingServiceStub
from betterproto2_lib.sift.runs.v2 import RunServiceStub
from betterproto2_lib.sift.users.v2 import UserServiceStub
from betterproto2_lib.sift.views.v2 import ViewServiceAsyncStub, ViewServiceStub
from sift_py.grpc.transport import use_sift_channel


class SiftApiService:
    def __init__(self, channel_config: SiftChannelConfig):
        self.channel = use_sift_channel(channel_config)
        self.channel_async = use_sift_betterproto_channel(channel_config)
        self.ping = PingServiceStub(self.channel)
        self.users = UserServiceStub(self.channel)
        self.assets = AssetServiceStub(self.channel)
        self.runs = RunServiceStub(self.channel)
        self.views = ViewServiceStub(self.channel)
        self.views_async = ViewServiceAsyncStub(self.channel_async)

    def close(self):
        self.channel.close()

    # Async context manager protocol
    async def __aenter__(self):
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        self.close()
        self.channel_async.close()

    # For synchronous use with asyncio.run()
    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        return self.close()


from betterproto2_lib.sift.views.v2 import (
    CreateViewRequest,
)


def create_view():
    with SiftApiService(channel_config) as api:
        req = CreateViewRequest.from_dict(
            dict(
                view=dict(
                    name="test-view4",
                    channels=[
                        {
                            "name": "voltage",
                            "data_type": "double",
                            "axis_group": "left1",
                        }
                    ],
                    axis_groups={"left": ["left1"], "right": []},
                )
            )
        )
        view = api.views.create_view(req)
        print(view)


async def create_view_async():
    with SiftApiService(channel_config) as api:
        req = CreateViewRequest.from_dict(
            dict(
                view=dict(
                    name="test-view5",
                    channels=[
                        {
                            "name": "voltage",
                            "data_type": "double",
                            "axis_group": "left1",
                        }
                    ],
                    axis_groups={"left": ["left1"], "right": []},
                )
            )
        )
        view = await api.views_async.create_view(req)
        print(view)


asyncio.run(create_view_async())
