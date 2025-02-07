from __future__ import annotations

from datetime import datetime
from typing import Dict, List, Type, TypedDict

from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataChannelValue
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    ChannelConfig as ChannelConfigPb,
)
from sift.ingestion_configs.v2.ingestion_configs_pb2 import (
    FlowConfig as FlowConfigPb,
)
from typing_extensions import Self

from sift_py._internal.convert.protobuf import AsProtobuf
from sift_py.ingestion.channel import ChannelConfig, ChannelValue, channel_fqn


class FlowConfig(AsProtobuf):
    """
    Describes a flow which is a set of channels whose values are often ingested together, allowing
    users to send multiple data points for multiple channels in a single request.

    `channel_by_fqn`:
        A mapping of a channel's fully-qualified name to the index of the `sift_py.ingestion.channel.ChannelConfig`
        as it appears in the `channels` attribute.
    """

    name: str
    channels: List[ChannelConfig]
    channel_by_fqn: Dict[str, int]

    def __init__(self, name: str, channels: List[ChannelConfig]):
        self.name = name
        self.channels = channels
        self.channel_by_fqn = {channel_fqn(c): i for i, c in enumerate(channels)}

    def as_pb(self, klass: Type[FlowConfigPb]) -> FlowConfigPb:
        return klass(
            name=self.name,
            channels=[conf.as_pb(ChannelConfigPb) for conf in self.channels],
        )

    @classmethod
    def from_pb(cls, message: FlowConfigPb) -> Self:
        return cls(
            name=message.name,
            channels=[ChannelConfig.from_pb(c) for c in message.channels],
        )


class Flow(TypedDict):
    """
    Represents a single flow that will be sent to Sift. Because this class uses `sift_py.ingestion.channel.ChannelValue`
    which is a fully qualified channel value, a specific ordering of items in `channel_values` is not required. If a
    particular flow has 5 channels, it is okay to send only data for 3 channels using this class.
    """

    flow_name: str
    timestamp: datetime
    channel_values: List[ChannelValue]


class FlowOrderedChannelValues(TypedDict):
    """
    Represents a single flow that will be sent to Sift. Unlike `sift_py.ingestion.flow.Flow`, this class requires
    that the ordering of channel values in `channel_values` match what the flow associated with `flow_name` expects.
    If a channel doesn't have particular data to send for a particular time, `sift_py.ingestion.channel.empty_value` should be used
    """

    flow_name: str
    timestamp: datetime
    channel_values: List[IngestWithConfigDataChannelValue]
