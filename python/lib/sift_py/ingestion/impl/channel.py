# from typing import List, cast

# from sift.channels.v2.channels_pb2 import ListChannelsRequest, ListChannelsResponse
# from sift.channels.v2.channels_pb2_grpc import ChannelServiceStub
# from sift.ingestion_configs.v1.ingestion_configs_pb2 import ChannelConfig as ChannelConfigPb
# from sift_py.grpc.transport import SiftChannel
# from sift_py.ingestion.channel import ChannelConfig


# def list_asset_channels(
# transport_channel: SiftChannel,
# asset_id: str,
# ) -> List[ChannelConfig]:
# """
# Queries all channels for the given `asset_id` and returns
# a list of all of their fully qualified names.
# """

# channels = []

# svc = ChannelServiceStub(transport_channel)
# req = ListChannelsRequest(
# filter=f'asset_id=="{asset_id}"',
# page_size=1_000,
# page_token="",
# )

# res = cast(ListChannelsResponse, svc.ListChannels(req))

# for channel in res.channels:
# channels.append(ChannelConfig.from_pb())

# return channels
