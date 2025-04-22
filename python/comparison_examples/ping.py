import os

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel

load_dotenv()
apikey = os.getenv("SIFT_API_KEY")
uri = os.getenv("BASE_URI")
channel_config: SiftChannelConfig = {
    "apikey": apikey,
    "uri": uri,
}


from sift.ping.v1.ping_pb2 import PingRequest
from sift.ping.v1.ping_pb2_grpc import PingServiceStub


def buf():
    with use_sift_channel(channel_config) as channel:
        response = PingServiceStub(channel).Ping(PingRequest())
        print(response)


import asyncio

from betterproto_lib.sift.ping.v1 import PingRequest as BPPingRequest
from betterproto_lib.sift.ping.v1 import PingServiceStub as BPPingServiceStub
from comparison_examples.util import use_sift_betterproto_channel


async def betterproto():
    async with use_sift_betterproto_channel(channel_config) as channel:
        response = await BPPingServiceStub(channel).ping(BPPingRequest())
        print(response)


if __name__ == "__main__":
    buf()
    asyncio.run(betterproto())
