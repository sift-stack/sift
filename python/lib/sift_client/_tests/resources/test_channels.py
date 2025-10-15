"""Pytest tests for the Channels API.

These tests demonstrate and validate the usage of the Channels API including:
- Basic channel operations (get, list, find)
- Channel filtering and searching
- Channel data retrieval
- Error handling and edge cases
"""

import pytest

from sift_client import SiftClient
from sift_client.resources import ChannelsAPI, ChannelsAPIAsync
from sift_client.sift_types import Channel

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert sift_client.channels
    assert isinstance(sift_client.channels, ChannelsAPI)
    assert sift_client.async_.channels
    assert isinstance(sift_client.async_.channels, ChannelsAPIAsync)


@pytest.fixture
def channels_api_async(sift_client: SiftClient):
    """Get the async channels API instance."""
    return sift_client.async_.channels


@pytest.fixture
def channels_api_sync(sift_client: SiftClient):
    """Get the synchronous channels API instance."""
    return sift_client.channels


@pytest.fixture
def test_channel(channels_api_sync):
    channels = channels_api_sync.list_(limit=1)
    assert channels
    assert len(channels) >= 1
    return channels[0]


class TestChannelsAPIAsync:
    """Test suite for the async Channels API functionality."""

    class TestGet:
        """Tests for the async get method."""

    @pytest.mark.asyncio
    async def test_get_by_id(self, channels_api_async, test_channel):
        """Test getting a specific channel by ID."""
        retrieved_channel = await channels_api_async.get(channel_id=test_channel.id_)

        assert isinstance(retrieved_channel, Channel)
        assert retrieved_channel.id_ == test_channel.id_
        assert retrieved_channel.name == test_channel.name

    class TestList:
        """Tests for the async list_ method."""

        @pytest.mark.asyncio
        async def test_basic_list(self, channels_api_async):
            """Test basic channel listing functionality."""
            channels = await channels_api_async.list_(limit=5)

            # Verify we get a list
            assert isinstance(channels, list)
            assert len(channels) == 5

            # If we have channels, verify their structure
            channel = channels[0]
            assert isinstance(channel, Channel)

        @pytest.mark.asyncio
        async def test_list_with_name_filter(self, channels_api_async):
            """Test channel listing with name filtering."""
            # First get some channels to work with
            all_channels = await channels_api_async.list_(limit=10)

            test_channel_name = all_channels[0].name
            filtered_channels = await channels_api_async.list_(name=test_channel_name)

            # Should find at least one channel with exact name match
            assert isinstance(filtered_channels, list)
            assert len(filtered_channels) >= 1

            # All returned channels should have the exact name
            for channel in filtered_channels:
                assert channel.name == test_channel_name

        @pytest.mark.asyncio
        async def test_list_with_name_contains_filter(self, channels_api_async):
            """Test channel listing with name contains filtering."""
            # Test with a common substring that might exist in channel names
            channels = await channels_api_async.list_(name_contains="test", limit=5)

            assert isinstance(channels, list)
            assert channels

            for channel in channels:
                assert "test" in channel.name.lower()

        @pytest.mark.asyncio
        async def test_list_with_name_regex_filter(self, channels_api_async):
            """Test channel listing with regex name filtering."""
            # Test with a regex pattern
            channels = await channels_api_async.list_(name_regex=r".*test.*", limit=5)

            assert isinstance(channels, list)
            assert channels

            import re

            pattern = re.compile(r".*test.*", re.IGNORECASE)
            for channel in channels:
                assert pattern.match(channel.name)

        @pytest.mark.asyncio
        async def test_list_with_channel_ids_filter(self, channels_api_async):
            """Test channel listing with channel IDs filter."""
            all_channels = await channels_api_async.list_(limit=3)

            if all_channels:
                channel_ids = [ch.id_ for ch in all_channels]
                filtered_channels = await channels_api_async.list_(channel_ids=channel_ids)

                # Should find at least the channels we specified
                assert isinstance(filtered_channels, list)
                assert len(filtered_channels) >= len(all_channels)

                # All returned channels should have IDs in our list
                for channel in filtered_channels:
                    assert channel.id_ in channel_ids

        @pytest.mark.asyncio
        async def test_list_with_asset_filter(self, channels_api_async):
            """Test channel listing with asset filter."""
            # First get a channel to get its asset
            all_channels = await channels_api_async.list_(limit=1)

            if all_channels:
                test_channel = all_channels[0]
                # Filter by asset ID
                filtered_channels = await channels_api_async.list_(asset=test_channel.asset_id)

                # Should find at least one channel for this asset
                assert isinstance(filtered_channels, list)
                assert len(filtered_channels) >= 1

                # All returned channels should belong to the same asset
                for channel in filtered_channels:
                    assert channel.asset_id == test_channel.asset_id

        @pytest.mark.asyncio
        async def test_list_with_description_contains_filter(self, channels_api_async):
            """Test channel listing with description contains filtering."""
            # Test with a common substring that might exist in descriptions
            description_contains = "the"
            channels = await channels_api_async.list_(
                description_contains=description_contains, limit=5
            )

            assert isinstance(channels, list)
            assert channels

            # If we found channels, verify they contain the substring in description
            for channel in channels:
                assert description_contains in channel.description.lower()

        @pytest.mark.asyncio
        async def test_list_with_limit(self, channels_api_async):
            """Test channel listing with different limits."""
            # Test with limit of 1
            channels_1 = await channels_api_async.list_(limit=1)
            assert isinstance(channels_1, list)
            assert len(channels_1) <= 1

            # Test with limit of 3
            channels_3 = await channels_api_async.list_(limit=3)
            assert isinstance(channels_3, list)
            assert len(channels_3) <= 3

        # TODO: active channel test
        # @pytest.mark.asyncio
        # async def test_list_include_archived(self, channels_api_async):
        #     """Test channel listing with archived channels included."""
        #     # Test without archived channels (default)
        #     channels_active = await channels_api_async.list_(limit=5, include_archived=False)
        #     assert isinstance(channels_active, list)
        #
        #     # Test with archived channels included
        #     channels_all = await channels_api_async.list_(limit=5, include_archived=True)
        #     assert isinstance(channels_all, list)
        #
        #     # Should have at least as many channels when including archived
        #     assert len(channels_all) >= len(channels_active)

        @pytest.mark.asyncio
        async def test_list_with_time_filters(self, channels_api_async):
            """Test channel listing with time-based filters."""
            from datetime import datetime, timedelta, timezone

            # Get channels created in the last year
            one_year_ago = datetime.now(timezone.utc) - timedelta(days=365)
            channels = await channels_api_async.list_(created_after=one_year_ago, limit=5)

            assert isinstance(channels, list)
            assert channels

            # If we found channels, verify they were created after the specified time
            for channel in channels:
                assert channel.created_date >= one_year_ago

    class TestFind:
        """Tests for the async find method."""

        @pytest.mark.asyncio
        async def test_find_channel(self, channels_api_async, test_channel):
            """Test finding a single channel."""
            # Find the same channel by name and asset
            found_channel = await channels_api_async.find(
                name=test_channel.name, asset=test_channel.asset_id
            )

            assert found_channel is not None
            assert found_channel.id_ == test_channel.id_

        @pytest.mark.asyncio
        async def test_find_nonexistent_channel(self, channels_api_async):
            """Test finding a non-existent channel returns None."""
            found_channel = await channels_api_async.find(name="nonexistent-channel-name-12345")
            assert found_channel is None

        @pytest.mark.asyncio
        async def test_find_multiple_raises_error(self, channels_api_async):
            """Test finding multiple channels raises an error."""
            with pytest.raises(ValueError, match="Multiple"):
                await channels_api_async.find(name_contains="test", limit=5)

    # TODO: data retrieval tests
    # class TestGetData:
    #     """Tests for the async get_data method."""
    #
    #     @pytest.mark.asyncio
    #     async def test_get_data_basic(self, channels_api_async, test_channel):
    #         """Test getting channel data."""
    #         # Get the channel's asset to find a run
    #         from sift_client.sift_types import Asset
    #
    #         asset = await channels_api_async.client.async_.assets.get(
    #             asset_id=test_channel.asset_id
    #         )
    #         assert isinstance(asset, Asset)
    #
    #         # Get runs for this asset
    #         runs = await channels_api_async.client.async_.runs.list_(limit=1)
    #         assert runs
    #         run = runs[0]
    #         # Get data for the channel
    #         data = await channels_api_async.get_data(
    #             channels=[test_channel], run=run.id_, limit=10
    #         )
    #
    #         # Verify we get a dictionary
    #         assert isinstance(data, dict)
    #         assert data
    #
    #         # Should have an entry for our channel
    #         assert test_channel.name in data or len(data) > 0
    #
    #     @pytest.mark.asyncio
    #     async def test_get_data_with_time_range(self, channels_api_async, test_channel):
    #         """Test getting channel data with time range."""
    #         from datetime import datetime, timedelta, timezone
    #
    #         # Get runs for this asset
    #         runs = await channels_api_async.client.async_.runs.list_(limit=1)
    #         assert runs
    #         run = runs[0]
    #         # Define a time range
    #         end_time = datetime.now(timezone.utc)
    #         start_time = end_time - timedelta(hours=1)
    #
    #         # Get data for the channel with time range
    #         data = await channels_api_async.get_data(
    #             channels=[test_channel],
    #             run=run.id_,
    #             start_time=start_time,
    #             end_time=end_time,
    #             limit=10,
    #         )
    #
    #         # Verify we get a dictionary
    #         assert isinstance(data, dict)
    #
    #     @pytest.mark.asyncio
    #     async def test_get_data_as_arrow(self, channels_api_async, test_channel):
    #         """Test getting channel data as Arrow table."""
    #         import pyarrow as pa
    #
    #         # Get runs for this asset
    #         runs = await channels_api_async.client.async_.runs.list_(limit=1)
    #         assert runs
    #         run = runs[0]
    #         # Get data as Arrow table
    #         data = await channels_api_async.get_data_as_arrow(
    #             channels=[test_channel], run=run.id_, limit=10
    #         )
    #
    #         # Verify we get a dictionary
    #         assert isinstance(data, dict)
    #         assert data
    #         for table in data.values():
    #             assert isinstance(table, pa.Table)
    #
    #     @pytest.mark.asyncio
    #     async def test_get_data_multiple_channels(self, channels_api_async):
    #         """Test getting data for multiple channels."""
    #         # Get multiple channels from the same asset
    #         channels = await channels_api_async.list_(limit=3)
    #
    #         if len(channels) >= 2:
    #             # Get the first asset's channels
    #             first_asset_id = channels[0].asset_id
    #             asset_channels = [
    #                 ch for ch in channels if ch.asset_id == first_asset_id
    #             ][:2]
    #
    #             if len(asset_channels) >= 2:
    #                 # Get runs for this asset
    #                 runs = await channels_api_async.client.async_.runs.list_(limit=1)
    #
    #                 if runs:
    #                     run = runs[0]
    #                     # Get data for multiple channels
    #                     data = await channels_api_async.get_data(
    #                         channels=asset_channels, run=run.id_, limit=10
    #                     )
    #
    #                     # Verify we get a dictionary
    #                     assert isinstance(data, dict)


class TestChannelsAPISync:
    """Test suite for the synchronous Channels API functionality.

    Only includes a single test for basic sync generation. No specific sync behavior difference tests are needed.
    """

    class TestGet:
        """Tests for the sync get method."""

        def test_get_by_id(self, channels_api_sync, test_channel):
            """Test getting a specific channel by ID synchronously."""
            retrieved_channel = channels_api_sync.get(channel_id=test_channel.id_)

            assert retrieved_channel is not None
            assert retrieved_channel.id_ == test_channel.id_
