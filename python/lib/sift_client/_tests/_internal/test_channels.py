from __future__ import annotations

from unittest.mock import AsyncMock, MagicMock

import pytest

from sift_client._internal.util.channels import resolve_calculated_channels
from sift_client.sift_types.calculated_channel import (
    CalculatedChannel,
    CalculatedChannelCreate,
    ChannelReference,
)
from sift_client.sift_types.channel import Channel


class TestResolveCalculatedChannels:
    @pytest.mark.asyncio
    async def test_none_passthrough(self):
        api = MagicMock()
        api.find = AsyncMock(return_value=None)
        assert await resolve_calculated_channels(None, channels_api=api) is None

    @pytest.mark.asyncio
    async def test_resolves_name_to_uuid(self):
        mock_ch = MagicMock(spec=Channel)
        mock_ch._id_or_error = "resolved-uuid"
        api = MagicMock()
        api.find = AsyncMock(return_value=mock_ch)

        cc = MagicMock(spec=CalculatedChannel)
        cc.name, cc.expression, cc.units = "calc", "$1 + 10", "m/s"
        cc.asset_ids = ["asset-1"]
        cc.channel_references = [
            ChannelReference(channel_reference="$1", channel_identifier="sensor.vel")
        ]

        result = await resolve_calculated_channels([cc], channels_api=api)
        assert result is not None
        assert len(result) == 1
        refs = result[0].expression_channel_references
        assert refs is not None
        assert refs[0].channel_identifier == "resolved-uuid"

    @pytest.mark.asyncio
    async def test_keeps_identifier_when_not_found(self):
        api = MagicMock()
        api.find = AsyncMock(return_value=None)
        cc = CalculatedChannelCreate(
            name="x",
            expression="$1",
            units="m",
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="ch-1")
            ],
        )
        result = await resolve_calculated_channels([cc], channels_api=api)
        assert result is not None
        assert result[0] == cc
