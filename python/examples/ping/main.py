import os

from dotenv import load_dotenv
from sift.ping.v1.ping_pb2 import PingRequest
from sift.ping.v1.ping_pb2_grpc import PingServiceStub
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel

if __name__ == "__main__":
    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey

    uri = os.getenv("BASE_URI")
    assert uri

    channel_config: SiftChannelConfig = {
        "apikey": apikey,
        "uri": uri,
    }

    with use_sift_channel(channel_config) as channel:
        response = PingServiceStub(channel).Ping(PingRequest())
        print(response)
