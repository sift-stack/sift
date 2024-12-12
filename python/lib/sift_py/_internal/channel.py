from typing import List, Optional, cast

from sift.channels.v2.channels_pb2 import Channel as ChannelPb
from sift.channels.v2.channels_pb2 import ListChannelsRequest, ListChannelsResponse
from sift.channels.v2.channels_pb2_grpc import ChannelServiceStub


def channel_fqn(name: str, component: Optional[str]) -> str:
    return name if component is None or len(component) == 0 else f"{component}.{name}"


def get_channels(
    channel_service: ChannelServiceStub,
    filter: str,
    page_size: int = 1_000,
    page_token: str = "",
) -> List[ChannelPb]:
    """
    Queries all channels with the given filter. Filter must be a CEL expression.
    """
    channels_pb: List[ChannelPb] = []

    req = ListChannelsRequest(
        filter=filter,
        page_size=page_size,
        page_token=page_token,
    )
    res = cast(ListChannelsResponse, channel_service.ListChannels(req))
    channels_pb.extend(res.channels)
    next_page_token = res.next_page_token

    while len(next_page_token) > 0:
        req = ListChannelsRequest(
            filter=filter,
            page_size=page_size,
            page_token=page_token,
        )
        res = cast(ListChannelsResponse, channel_service.ListChannels(req))
        channels_pb.extend(res.channels)
        next_page_token = res.next_page_token

    return channels_pb
