from __future__ import annotations

from datetime import datetime
from typing import Dict, List, Optional, TypedDict, Union

from typing_extensions import NotRequired, TypeAlias

from sift_py._internal.channel import channel_fqn


class DataQuery:
    DEFAULT_PAGE_SIZE = 100_000

    asset_name: str
    start_time: datetime
    end_time: datetime
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
        page_size: int = DEFAULT_PAGE_SIZE,
    ):
        self.asset_name = asset_name
        self.start_time = start_time
        self.end_time = end_time
        self.sample_ms = sample_ms
        self.channels = channels
        self.page_size = page_size


class ChannelQuery:
    channel_name: str
    component: Optional[str]
    run_name: Optional[str]

    def __init__(
        self,
        channel_name: str,
        component: Optional[str] = None,
        run_name: Optional[str] = None,
    ):
        self.channel_name = channel_name
        self.component = component
        self.run_name = run_name

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

    def __init__(
        self,
        channel_key: str,
        run_name: str,
        expression: str,
        expression_channel_references: Dict[ChannelName, ChannelIdentifier],
    ):
        self.channel_key = channel_key
        self.run_name = run_name
        self.expression = expression
        self.expression_channel_references = expression_channel_references
