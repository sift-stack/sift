from __future__ import annotations

from datetime import datetime
from typing import Dict, List, Optional, TypedDict, Union

from typing_extensions import NotRequired, TypeAlias

from sift_py._internal.channel import channel_fqn


class DataQuery:
    DEFAULT_PAGE_SIZE = 100_000

    asset_name: str
    start_time: datetime
    end: datetime
    sample_ms: int
    page_size: int
    channels: List[Union[ChannelQuery, CalculatedChannelQuery]]

    def __init__(
        self,
        asset_name: str,
        start_time: datetime,
        end_time: datetime,
        sample_ms: int,
        channels: List[Union[ChannelQuery, CalculatedChannelQuery]],
        page_size: Optional[int],
    ):
        self.asset_name = asset_name
        self.start_time = start_time
        self.end_time = end_time
        self.sample_ms = sample_ms
        self.channels = channels
        self.page_size = page_size or self.__class__.DEFAULT_PAGE_SIZE


class ChannelQuery:
    channel_name: str
    component: Optional[str]
    run_name: Optional[str]

    def fqn(self) -> str:
        return channel_fqn(self.channel_name, self.component)


class CalculatedChannelQuery:
    ChannelName: TypeAlias = str
    ChannelIdentifier = TypedDict(
        "ChannelIdentifier",
        {
            "channel_name": str,
            "component": NotRequired[str],
        },
    )

    channel_key: str
    run_name: str
    expression: str
    expression_channel_references: Dict[ChannelName, ChannelIdentifier]
