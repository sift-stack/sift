from __future__ import annotations

from pydantic import BaseModel, ConfigDict


class ChannelReference(BaseModel):
    """
    Channel reference for calculated channel or rule.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    channel_reference: str  # The key of the channel in the expression i.e. $1, $2, etc.
    channel_identifier: str  # The name of the channel

    @classmethod
    def _from_proto(cls, proto) -> ChannelReference:
        return cls(
            channel_reference=proto.channel_reference,
            channel_identifier=proto.channel_identifier,
        )
