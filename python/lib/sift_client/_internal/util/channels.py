from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client.sift_types.calculated_channel import CalculatedChannel, CalculatedChannelCreate
from sift_client.sift_types.channel import ChannelReference

if TYPE_CHECKING:
    from sift_client.resources.channels import ChannelsAPIAsync


async def resolve_calculated_channels(
    calculated_channels: list[CalculatedChannel | CalculatedChannelCreate] | None,
    channels_api: ChannelsAPIAsync,
) -> list[CalculatedChannel | CalculatedChannelCreate] | None:
    """Resolve channel reference identifiers from names to UUIDs.

    For each channel reference, looks up the identifier as a channel name.
    If found, replaces it with the channel's UUID. If not found, assumes
    the identifier is already a UUID and keeps it as-is.
    """
    if not calculated_channels:
        return None

    resolved: list[CalculatedChannel | CalculatedChannelCreate] = []
    for cc in calculated_channels:
        refs = (
            (cc.expression_channel_references or [])
            if isinstance(cc, CalculatedChannelCreate)
            else cc.channel_references
        )

        resolved_refs: list[ChannelReference] = []
        for ref in refs:
            channel = await channels_api.find(
                name=ref.channel_identifier,
                assets=cc.asset_ids,
            )
            if channel is not None:
                ref = ChannelReference(
                    channel_reference=ref.channel_reference,
                    channel_identifier=channel._id_or_error,
                )
            resolved_refs.append(ref)

        resolved.append(
            CalculatedChannelCreate(
                name=cc.name,
                expression=cc.expression,
                expression_channel_references=resolved_refs,
                units=cc.units or None,
            )
        )
    return resolved
