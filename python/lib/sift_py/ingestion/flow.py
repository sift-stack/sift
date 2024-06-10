from __future__ import annotations
from .channel import ChannelConfig
from sift_internal.convert.protobuf import try_convert_pb, AsProtobuf, ProtobufMessage
from sift.ingestion_configs.v1.ingestion_configs_pb2 import (
    ChannelConfig as ChannelConfigPb,
    FlowConfig as FlowConfigPb,
)
from typing import Dict, List, Optional, Type


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
        self.channel_by_fqn = {
            self.__class__.compute_fqn(c.name, c.component): i for i, c in enumerate(channels)
        }

    def get_channel(self, name: str, component: Optional[str] = "") -> Optional[ChannelConfig]:
        """
        Retrieves a `ChannelConfig` by its fully qualified name. Returns `None` if it cannot be found.
        """
        fqn = self.__class__.compute_fqn(name, component)
        index = self.channel_by_fqn[fqn]

        try:
            return self.channels[index]
        except IndexError:
            return None

    def as_pb(self, klass: Type[ProtobufMessage]) -> ProtobufMessage:
        return FlowConfigPb(
            name=self.name,
            channels=[try_convert_pb(conf, ChannelConfigPb) for conf in self.channels],
        )

    @staticmethod
    def compute_fqn(name: str, component: Optional[str]) -> str:
        """
        The fully-qualified channel name of a channel called 'voltage' is simply `voltage'. The
        fully qualified name of a channel called 'temperature' of component 'motor' is a `motor.temperature'.
        """
        if component is None or len(component) == "":
            return name
        else:
            return f"{component}.{name}"
