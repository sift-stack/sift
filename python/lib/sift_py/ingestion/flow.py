from __future__ import annotations

from typing import Dict, List, Optional, Type

from sift.ingestion_configs.v1.ingestion_configs_pb2 import (
    ChannelConfig as ChannelConfigPb,
)
from sift.ingestion_configs.v1.ingestion_configs_pb2 import (
    FlowConfig as FlowConfigPb,
)
from sift_internal.convert.protobuf import AsProtobuf, ProtobufMessage, try_cast_pb

from .channel import ChannelConfig, channel_fqn


class FlowConfig(AsProtobuf):
    """
    Describes a flow which is a set of channels whose values are often
    ingested together.

    The `channel_by_fqn` attribute is a mapping of a channel's fully-qualified name
    to the index of the `ChannelConfig` instance as it appears in the `channels` attribute.
    """

    name: str
    channels: List[ChannelConfig]
    channel_by_fqn: Dict[str, int]

    def __init__(self, name: str, channels: List[ChannelConfig]):
        self.name = name
        self.channels = channels
        self.channel_by_fqn = {channel_fqn(c): i for i, c in enumerate(channels)}

    def as_pb(self, klass: Type[ProtobufMessage]) -> Optional[ProtobufMessage]:
        return FlowConfigPb(
            name=self.name,
            channels=[try_cast_pb(conf, ChannelConfigPb) for conf in self.channels],
        )
