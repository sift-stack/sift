"""Tests for the Channels low-level wrapper.

The low-level client builds Channels from protos and resolves each unit id to its
name, so a Channel's `unit` field is always a human-readable name, never an id.
"""

from unittest.mock import AsyncMock, MagicMock

import pytest
import sift.common.type.v1.channel_data_type_pb2 as channel_pb
from sift.channels.v3.channels_pb2 import Channel as ChannelProto
from sift.unit.v2.unit_pb2 import Unit as UnitProto

from sift_client._internal.low_level_wrappers.channels import ChannelsLowLevelClient


def _client() -> ChannelsLowLevelClient:
    """A low-level channels client with the units client mocked."""
    client = ChannelsLowLevelClient(MagicMock())
    client._units_low_level_client = MagicMock()
    return client


def _proto(*, unit_id: str = "", display_unit_id: str = "") -> ChannelProto:
    return ChannelProto(
        channel_id="c1",
        name="channel",
        data_type=channel_pb.CHANNEL_DATA_TYPE_DOUBLE,
        asset_id="a1",
        unit_id=unit_id,
        display_unit_id=display_unit_id,
    )


class TestBuildChannels:
    @pytest.mark.asyncio
    async def test_resolves_unit_id_to_name(self):
        """The proto's unit id becomes the abbreviated name on the Channel."""
        client = _client()
        client._units_low_level_client.list_all_units = AsyncMock(
            return_value=[UnitProto(unit_id="u1", abbreviated_name="volts")]
        )

        channels = await client._build_channels([_proto(unit_id="u1")])

        assert channels[0].unit == "volts"

    @pytest.mark.asyncio
    async def test_prefers_display_unit_over_canonical(self):
        """The display unit id wins over the canonical unit id."""
        client = _client()
        client._units_low_level_client.list_all_units = AsyncMock(
            return_value=[UnitProto(unit_id="disp", abbreviated_name="ms")]
        )

        channels = await client._build_channels([_proto(unit_id="canon", display_unit_id="disp")])

        assert channels[0].unit == "ms"

    @pytest.mark.asyncio
    async def test_no_lookup_when_no_unit(self):
        """A channel with no unit short-circuits before any Units call."""
        client = _client()
        client._units_low_level_client.list_all_units = AsyncMock()

        channels = await client._build_channels([_proto()])

        assert channels[0].unit == ""
        client._units_low_level_client.list_all_units.assert_not_awaited()

    @pytest.mark.asyncio
    async def test_unresolved_id_becomes_empty(self):
        """An id with no matching unit resolves to an empty string, never the id."""
        client = _client()
        client._units_low_level_client.list_all_units = AsyncMock(return_value=[])

        channels = await client._build_channels([_proto(unit_id="missing")])

        assert channels[0].unit == ""
