from typing import List, cast

from sift.channels.v2.channels_pb2 import Channel as ChannelPb
from sift.channels.v2.channels_pb2 import ListChannelsRequest, ListChannelsResponse
from sift.channels.v2.channels_pb2_grpc import ChannelServiceStub
from sift.rules.v1.rules_pb2 import ChannelReference

from sift_py.grpc.transport import SiftChannel


def get_asset_channels(
    transport_channel: SiftChannel,
    asset_id: str,
) -> List[ChannelPb]:
    """
    Queries all channels for the given `asset_id`.
    """
    channels_pb: List[ChannelPb] = []

    svc = ChannelServiceStub(transport_channel)
    req = ListChannelsRequest(
        filter=f'asset_id=="{asset_id}"',
        page_size=1_000,
        page_token="",
    )
    res = cast(ListChannelsResponse, svc.ListChannels(req))
    channels_pb.extend(res.channels)
    next_page_token = res.next_page_token

    while len(next_page_token) > 0:
        req = ListChannelsRequest(
            filter=f'asset_id=="{asset_id}"',
            page_size=1_000,
            page_token=next_page_token,
        )
        res = cast(ListChannelsResponse, svc.ListChannels(req))
        channels_pb.extend(res.channels)
        next_page_token = res.next_page_token

    return channels_pb


def channel_reference_from_fqn(fqn: str) -> ChannelReference:
    parts = fqn.split(".")

    if len(parts) == 1:
        return ChannelReference(name=parts[0])

    component_parts = parts[: len(parts) - 1]

    return ChannelReference(name=parts[-1], component=".".join(component_parts))
