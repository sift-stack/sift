"""Pytest tests for the Channels API.

These tests demonstrate and validate the usage of the Channels API including:
- Basic channel operations (get, list, find)
- Channel filtering and searching
- Channel data retrieval
- Error handling and edge cases
"""

import asyncio
import uuid
from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock
from urllib.parse import urljoin

import pytest
import requests
from sift.unit.v2.unit_pb2 import Unit as UnitProto

from sift_client import SiftClient
from sift_client.resources import ChannelsAPI, ChannelsAPIAsync
from sift_client.sift_types import Channel
from sift_client.sift_types.channel import ChannelDataType


@pytest.mark.integration
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


@pytest.mark.integration
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

    class TestArchive:
        """Tests for the async archive method."""

        @pytest.mark.asyncio
        async def test_create_archive_unarchive_flow(self, channels_api_async, test_channel):
            """Create a channel via REST schemaless ingest, then archive/unarchive via channels API; verify at each step with find."""
            asset_name = test_channel.asset.name
            asset_id = test_channel.asset_id
            unique_name = f"archive-test-channel-{uuid.uuid4().hex}"

            rest_client = channels_api_async.client.rest_client
            rest_url = urljoin(rest_client.base_url, "api/v2/ingest")
            api_key = rest_client._config.api_key

            # Create the channel by ingesting a single data point (schemaless).
            #
            # This is currently the simplest way to create a channel. Simply
            # creating a channel schema is not sufficient since schemaless channels
            # that have no data are filtered out of the `ListChannels` response.
            payload = {
                "asset_name": asset_name,
                "data": [
                    {
                        "timestamp": "2024-11-06T10:27:20-07:00",
                        "values": [
                            {"channel": unique_name, "value": 1},
                        ],
                    }
                ],
            }
            resp = requests.post(
                rest_url,
                headers={
                    "Authorization": f"Bearer {api_key}",
                    "Content-Type": "application/json",
                },
                json=payload,
                timeout=30,
            )
            resp.raise_for_status()

            # Retry find until the channel is visible.
            created = None
            for _ in range(20):
                created = await channels_api_async.find(name=unique_name, asset=asset_id)
                if created is not None:
                    break
                await asyncio.sleep(0.5)
            assert created is not None, f"Channel {unique_name} did not appear after ingest"

            await channels_api_async.archive([created])
            found_archived = await channels_api_async.find(
                name=unique_name, asset=asset_id, archived=True
            )
            assert found_archived is not None

            await channels_api_async.unarchive([created])
            found_active = await channels_api_async.find(name=unique_name, asset=asset_id)
            assert found_active is not None

            # Cleanup by archiving the channel again
            await channels_api_async.archive([created])

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


@pytest.mark.integration
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


def _make_api() -> ChannelsAPIAsync:
    """Build a Channels resource with both low-level clients replaced by mocks."""
    api = ChannelsAPIAsync(MagicMock())
    api._low_level_client = MagicMock()
    api._units_low_level_client = MagicMock()
    return api


def _mock_channel(unit: str) -> Channel:
    """Build a minimal Channel whose `unit` field holds the given value."""
    now = datetime.now(timezone.utc)
    return Channel(
        id_="ch-1",
        name="channel",
        data_type=ChannelDataType.DOUBLE,
        description="",
        unit=unit,
        asset_id="asset-1",
        is_archived=False,
        created_date=now,
        modified_date=now,
        created_by_user_id="user-1",
        modified_by_user_id="user-1",
    )


class TestUpdateUnitResolution:
    """update() resolves a unit name to an id for the write. Resolving ids back to
    names on the returned channel is the low-level client's job (tested there).
    """

    @pytest.mark.asyncio
    async def test_resolves_unit_name_to_id_for_write(self):
        """update() turns the unit name into an id via create_unit before writing."""
        api = _make_api()
        api._units_low_level_client.create_unit = AsyncMock(
            return_value=UnitProto(unit_id="u-1", abbreviated_name="volts")
        )
        captured = {}

        async def fake_update_channel(update):
            captured["update"] = update
            return _mock_channel(unit="volts")

        api._low_level_client.update_channel = AsyncMock(side_effect=fake_update_channel)

        result = await api.update("ch-1", {"unit": "volts"})

        api._units_low_level_client.create_unit.assert_awaited_once_with("volts")
        # The write carries the resolved unit id, not the raw name.
        assert captured["update"].unit == "u-1"
        assert captured["update"].resource_id == "ch-1"
        assert result.unit == "volts"

    @pytest.mark.asyncio
    async def test_update_without_unit_skips_create_unit(self):
        """An update that does not change the unit never calls create_unit."""
        api = _make_api()
        api._units_low_level_client.create_unit = AsyncMock()
        api._low_level_client.update_channel = AsyncMock(return_value=_mock_channel(unit="ms"))

        await api.update("ch-1", {"description": "new"})

        api._units_low_level_client.create_unit.assert_not_awaited()

    @pytest.mark.asyncio
    async def test_empty_unit_string_clears_without_creating_a_unit(self):
        """An empty unit string is passed through to clear the unit, not sent to create_unit."""
        api = _make_api()
        api._units_low_level_client.create_unit = AsyncMock()
        captured = {}

        async def fake_update_channel(update):
            captured["update"] = update
            return _mock_channel(unit="")

        api._low_level_client.update_channel = AsyncMock(side_effect=fake_update_channel)

        await api.update("ch-1", {"unit": ""})

        api._units_low_level_client.create_unit.assert_not_awaited()
        assert captured["update"].unit == ""


class TestConfigureDataCache:
    """``configure_data_cache`` is the resource-level knob for the in-memory
    channel data cache. Before the cache is initialized, it stashes the value
    for the lazy-init path; after, it retunes the live cache.

    Each test that triggers ``_ensure_data_low_level_client`` opens the
    opt-out disk tier (redirected to ``tmp_path`` by the conftest fixture)
    and closes the handle in ``finally`` so the diskcache lock doesn't leak
    into the next test.
    """

    def test_before_lazy_init_propagates_to_cache(self):
        """Configuring before the first ``get_data`` lands on the cache at init."""
        api = _make_api()
        api.configure_data_cache(max_bytes=123)
        assert api._data_low_level_client is None  # still lazy
        api._ensure_data_low_level_client()
        try:
            assert api._data_low_level_client.channel_cache.max_bytes == 123
        finally:
            api._data_low_level_client.channel_cache.close()

    def test_after_lazy_init_updates_live_cache(self):
        """Configuring after first use retunes the live cache in place."""
        api = _make_api()
        api._ensure_data_low_level_client()
        try:
            original_client = api._data_low_level_client
            api.configure_data_cache(max_bytes=456)
            # Same wrapper instance — we mutated, not replaced.
            assert api._data_low_level_client is original_client
            assert api._data_low_level_client.channel_cache.max_bytes == 456
        finally:
            api._data_low_level_client.channel_cache.close()

    def test_zero_disables_cache_via_resource(self):
        """Resource-level ``max_bytes=0`` end-to-end disables the cache."""
        api = _make_api()
        api.configure_data_cache(max_bytes=0)
        api._ensure_data_low_level_client()
        try:
            assert not api._data_low_level_client.channel_cache.enabled
        finally:
            api._data_low_level_client.channel_cache.close()

    def test_negative_raises(self):
        api = _make_api()
        with pytest.raises(ValueError, match="max_bytes"):
            api.configure_data_cache(max_bytes=-1)


class TestEnableDataCacheDisk:
    """``enable_data_cache_disk`` / ``disable_data_cache_disk`` plumb the disk
    tier setting to the underlying ``ChannelCache``, both pre- and post-init.

    The disk tier itself is exercised directly in
    ``test_data.py::TestChannelCacheDisk``; the tests here just verify the
    resource-level wiring around it.
    """

    def test_enabled_by_default(self):
        """Disk persistence is opt-out: the default-constructed resource
        lands at ``ChannelCache.DEFAULT_DISK_PATH`` on first ``get_data``.

        The autouse ``_isolate_default_disk_cache_path`` fixture in
        ``conftest.py`` redirects the constant to a per-test tmp dir so this
        doesn't litter the real ``/tmp``.
        """
        from sift_client._internal.low_level_wrappers.data import ChannelCache

        api = _make_api()
        api._ensure_data_low_level_client()
        cache = api._data_low_level_client.channel_cache
        try:
            assert cache.disk_enabled
            assert cache.disk_path == ChannelCache.DEFAULT_DISK_PATH
        finally:
            cache.close()

    def test_enable_before_lazy_init_propagates(self, tmp_path):
        api = _make_api()
        api.enable_data_cache_disk(path=str(tmp_path / "pre-init"), max_bytes=4096)
        api._ensure_data_low_level_client()
        cache = api._data_low_level_client.channel_cache
        try:
            assert cache.disk_enabled
            assert cache.disk_path == str(tmp_path / "pre-init")
            assert cache.disk_max_bytes == 4096
        finally:
            cache.close()

    def test_enable_after_lazy_init_updates_live_cache(self, tmp_path):
        """``disable_data_cache_disk`` → ``enable_data_cache_disk`` round-trip
        on a live cache swaps the disk handle without recreating the wrapper.
        """
        api = _make_api()
        # Start from the disk-off state so the test exercises the "off → on"
        # transition rather than "default-on → reconfigured-on".
        api.disable_data_cache_disk()
        api._ensure_data_low_level_client()
        cache = api._data_low_level_client.channel_cache
        try:
            assert not cache.disk_enabled
            api.enable_data_cache_disk(path=str(tmp_path / "post-init"))
            assert cache.disk_enabled
            assert cache.disk_path == str(tmp_path / "post-init")
        finally:
            cache.close()

    def test_enable_with_default_path_lands_on_default(self, monkeypatch, tmp_path):
        """Calling ``enable_data_cache_disk()`` with no args uses the default path.

        Redirects ``ChannelCache.DEFAULT_DISK_PATH`` to ``tmp_path`` so the
        test doesn't create the real ``/tmp/sift-channel-data-cache``
        directory.
        """
        from sift_client._internal.low_level_wrappers.data import ChannelCache

        fake_default = str(tmp_path / "fake-default")
        monkeypatch.setattr(ChannelCache, "DEFAULT_DISK_PATH", fake_default)

        api = _make_api()
        api.enable_data_cache_disk()
        api._ensure_data_low_level_client()
        cache = api._data_low_level_client.channel_cache
        try:
            assert cache.disk_path == fake_default
        finally:
            cache.close()

    def test_disable_closes_live_disk_handle(self, tmp_path):
        api = _make_api()
        api.enable_data_cache_disk(path=str(tmp_path / "to-close"))
        api._ensure_data_low_level_client()
        cache = api._data_low_level_client.channel_cache
        try:
            assert cache.disk_enabled
            api.disable_data_cache_disk()
            assert not cache.disk_enabled
            assert cache.disk_path is None
        finally:
            cache.close()

    def test_clear_data_cache_on_disk_proxies_to_cache(self, tmp_path):
        """The resource method removes the directory by proxying to ChannelCache."""
        from sift_client._internal.low_level_wrappers.data import ChannelCache

        path = tmp_path / "to-clear"
        # Populate a real disk-cache directory so the marker check passes.
        cache = ChannelCache(max_bytes=10_000_000, disk_path=path)
        cache.close()
        assert path.exists()

        api = _make_api()
        api.clear_data_cache_on_disk(path)
        assert not path.exists()

    def test_default_path_failure_falls_back_to_memory(self, monkeypatch, tmp_path):
        """If the opt-out default disk path can't be opened, the wrapper logs
        a warning and continues with the in-memory cache only.

        Simulated by pointing ``DEFAULT_DISK_PATH`` at a path that already
        exists as a regular file — ``os.makedirs(..., exist_ok=True)`` raises
        ``FileExistsError`` for non-directory targets.
        """
        from sift_client._internal.low_level_wrappers.data import ChannelCache

        blocker = tmp_path / "not-a-dir"
        blocker.write_text("i am a file, not a directory")
        monkeypatch.setattr(ChannelCache, "DEFAULT_DISK_PATH", str(blocker))

        api = _make_api()
        api._ensure_data_low_level_client()  # must not raise
        cache = api._data_low_level_client.channel_cache
        try:
            # Disk silently dropped, memory still working.
            assert not cache.disk_enabled
            assert cache.enabled
        finally:
            cache.close()

    def test_explicit_path_failure_propagates(self, tmp_path):
        """An explicit ``enable_data_cache_disk(path=...)`` that can't open
        propagates the OSError — silent fallback would hide a user mistake.
        """
        blocker = tmp_path / "not-a-dir"
        blocker.write_text("i am a file, not a directory")

        api = _make_api()
        api.enable_data_cache_disk(path=str(blocker))
        with pytest.raises(FileExistsError):
            api._ensure_data_low_level_client()
