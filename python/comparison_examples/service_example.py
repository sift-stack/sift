import os

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

from betterproto_lib.betterproto_sift.assets.v1 import AssetServiceStub
from betterproto_lib.betterproto_sift.ping.v1 import PingServiceStub
from betterproto_lib.betterproto_sift.runs.v2 import RunServiceStub
from betterproto_lib.betterproto_sift.users.v2 import UserServiceStub
from betterproto_lib.betterproto_sift.views.v2 import ViewServiceStub
from comparison_examples.util import use_sift_betterproto_channel


class SiftApiService:
    def __init__(self, channel_config: SiftChannelConfig):
        self.channel = use_sift_betterproto_channel(channel_config)
        self.ping = PingServiceStub(self.channel)
        self.users = UserServiceStub(self.channel)
        self.assets = AssetServiceStub(self.channel)
        self.runs = RunServiceStub(self.channel)
        self.views = ViewServiceStub(self.channel)

    def close(self):
        self.channel.close()

    # Async context manager protocol
    async def __aenter__(self):
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        self.close()

    # For synchronous use with asyncio.run()
    def __enter__(self):
        self._loop = asyncio.new_event_loop()
        asyncio.set_event_loop(self._loop)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        try:
            self._loop.run_until_complete(self.close())
        finally:
            self._loop.close()
            asyncio.set_event_loop(None)


from betterproto_lib.betterproto_sift.views.v2 import (
    CreateViewRequest,
    View,
    ViewAxisGroups,
    ViewChannel,
)


async def create_view():
    async with SiftApiService(channel_config) as api:
        req = CreateViewRequest.from_dict(
            dict(
                view=dict(
                    view_id="",
                    name="test-view",
                    channels=[
                        {
                            "name": "voltage",
                            "data_type": "double",
                            "axis_group": "left1",
                            "bit_field_names": [],
                        }
                    ],
                    axis_groups={"left": ["left1"], "right": []},
                )
            )
        )
        req = CreateViewRequest(
            view=View(
                name="test-view2",
                channels=[
                    ViewChannel(
                        name="voltage",
                        data_type="double",
                        axis_group="left1",
                        bit_field_names=[],
                    )
                ],
                axis_groups=ViewAxisGroups(left=["left1"], right=[]),
                # created_date=datetime.datetime.now(datetime.timezone.utc),
                # modified_date=datetime.datetime.now(datetime.timezone.utc),
                # created_by_user_id="",
                # modified_by_user_id="",
                # is_pinned=False,
            )
        )
        view = await api.views.create_view(req)
        print(view)


asyncio.run(create_view())

# Demo of creating a view

# def create_rule():
#     with use_sift_betterproto_channel(channel_config) as channel:
#         rule_service = RuleServiceStub(channel)
#         asset_service = AssetServiceStub(channel)
#         channel_service = ChannelServiceStub(channel)
#         user_service = UserServiceStub(channel)
#
#         # create a rule, from a dict of native python types!
#         rule = rule_service.create_rule(
#             CreateRuleRequest.from_dict(
#                 dict(
#                     update=dict(
#                         name="my-rule",
#                         description="description",
#                     )
#                 )
#             )
#         )
