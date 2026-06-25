from __future__ import annotations

import logging
from typing import TYPE_CHECKING

from sift_client._internal.disk_cache_config import DiskCacheConfig
from sift_client._internal.low_level_wrappers.channels import ChannelsLowLevelClient
from sift_client._internal.low_level_wrappers.units import UnitsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.channel import Channel, ChannelUpdate
from sift_client.sift_types.run import Run
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import os
    import re
    from datetime import datetime

    import pandas as pd
    import pyarrow as pa

    from sift_client.client import SiftClient

logger = logging.getLogger(__name__)


def _channel_ids_from_list(items: list[str | Channel]) -> list[str]:
    """Resolve a list of channel IDs or Channel objects to a list of channel IDs.

    Args:
        items: List of channel IDs (str) or Channel objects.

    Returns:
        List of channel ID strings.

    Raises:
        ValueError: If any Channel object has no id set.
    """
    ids: list[str] = []
    for item in items:
        if isinstance(item, str):
            ids.append(item)
        else:
            try:
                ids.append(item._id_or_error)
            except ValueError:
                raise ValueError("One or more Channel objects have no id set.") from None
    return ids


class ChannelsAPIAsync(ResourceBase):
    """High-level API for interacting with channels.

    This class provides a Pythonic, notebook-friendly interface for interacting with the ChannelsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Channel class from the low-level wrapper, which is a user-friendly
    representation of a channel using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the ChannelsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ChannelsLowLevelClient(grpc_client=self.client.grpc_client)
        self._units_low_level_client = UnitsLowLevelClient(grpc_client=self.client.grpc_client)
        self._data_low_level_client = None
        self._disk_cache_config = DiskCacheConfig(enabled=True)

    def enable_data_cache_disk(
        self,
        *,
        path: str | os.PathLike[str] | None = None,
        max_bytes: int | None = None,
    ) -> None:
        """Configure (or re-enable after ``disable_data_cache_disk``) the disk cache.

        Disk persistence is **on by default** at ``ChannelCache.DEFAULT_DISK_PATH``;
        use this method when you want to override the path or size, or to turn
        the cache back on after a prior ``disable_data_cache_disk`` call.

        Each entry that ``get_data`` returns is written to the cache and read
        back on subsequent calls, even after process restart. The default
        path lives under ``tempfile.gettempdir()`` and is shared across
        sessions, so a re-run of the same workload picks up previously-cached
        windows without a fetch.

        Safe to call before or after the first ``get_data``. Reconfiguring
        (different ``path`` or ``max_bytes``) closes the previous handle and
        opens a new one.

        An explicit ``path`` that can't be opened (e.g. permission denied,
        read-only filesystem) raises so the caller knows the request didn't
        take. The default-path open does *not* raise — see
        ``_ensure_data_low_level_client`` for the silent fall-back behaviour.

        Args:
            path: Directory to persist the cache to. ``None`` (the default)
                uses ``ChannelCache.DEFAULT_DISK_PATH``. Existing entries at
                the path become available as cache hits.
            max_bytes: Byte cap on disk usage. ``None`` uses
                ``ChannelCache.DEFAULT_DISK_MAX_BYTES`` (4 GiB). When the
                bound is reached, ``diskcache``'s LRU eviction takes over.

        Example:
            client.channels.enable_data_cache_disk(path="/data/sift-cache")
            client.channels.enable_data_cache_disk(max_bytes=1024 ** 3)  # 1 GiB
        """
        self._disk_cache_config.enable(path=path, max_bytes=max_bytes)
        if self._data_low_level_client is not None:
            self._data_low_level_client.channel_cache.enable_disk(path=path, max_bytes=max_bytes)

    def disable_data_cache_disk(self) -> None:
        """Opt out of caching for ``get_data`` (no reads or writes).

        Caching is on by default; call this when you don't want any cached
        data written to or read from disk. Closes any open cache file
        handle. The on-disk directory is NOT deleted — use
        :meth:`clear_data_cache_on_disk` to wipe it.
        """
        self._disk_cache_config.disable()
        if self._data_low_level_client is not None:
            self._data_low_level_client.channel_cache.disable_disk()

    def clear_data_cache_on_disk(self, path: str | os.PathLike[str] | None = None) -> None:
        """Delete a previously-persisted on-disk channel data cache directory.

        Drops stale caches from previous sessions, recovers from a corrupt
        cache, or reclaims disk space. Removes the directory entirely; if disk
        persistence is on, the next ``get_data`` re-opens an empty cache at
        the same path.

        This is a thin proxy around
        :meth:`ChannelCache.clear_disk <sift_client._internal.low_level_wrappers.data.ChannelCache.clear_disk>`
        — exposed on the resource so callers don't need to reach into
        ``_internal`` modules. The underlying classmethod is also reachable
        directly (``ChannelCache.clear_disk(...)``) if the caller doesn't have
        a ``SiftClient`` handy.

        Args:
            path: Directory of the cache to clear. ``None`` (the default)
                targets ``ChannelCache.DEFAULT_DISK_PATH``.

        Raises:
            ValueError: If ``path`` exists but does not look like a sift
                channel data cache directory.
        """
        from sift_client._internal.low_level_wrappers.data import ChannelCache

        ChannelCache.clear_disk(path)

    async def get(
        self,
        *,
        channel_id: str,
    ) -> Channel:
        """Get a Channel.

        Args:
            channel_id: The ID of the channel.

        Returns:
            The Channel.
        """
        channel = await self._low_level_client.get_channel(channel_id=channel_id)
        return self._apply_client_to_instance(channel)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        channel_ids: list[str] | None = None,
        # created/modified ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        # channel specific
        asset: Asset | str | None = None,
        assets: list[str | Asset] | None = None,
        run: Run | str | None = None,
        # common filters
        description_contains: str | None = None,
        archived: bool | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[Channel]:
        """List channels with optional filtering.

        Args:
            name: Exact name of the channel.
            names: List of channel names to filter by.
            name_contains: Partial name of the channel.
            name_regex: Regular expression to filter channels by name.
            channel_ids: Filter to channels with any of these IDs.
            created_after: Filter channels created after this datetime. Note: This is related to the channel creation time, not the timestamp of the underlying data.
            created_before: Filter channels created before this datetime. Note: This is related to the channel creation time, not the timestamp of the underlying data.
            modified_after: Filter channels modified after this datetime.
            modified_before: Filter channels modified before this datetime.
            asset: Filter channels associated with this Asset or asset ID.
            assets: Filter channels associated with these Assets or asset IDs.
            run: Filter channels associated with this Run or run ID.
            description_contains: Partial description of the channel.
            archived: If True, searches for archived channels.
            filter_query: Explicit CEL query to filter channels.
            order_by: Field and direction to order results by.
            limit: Maximum number of channels to return. If None, returns all matches.
            page_size: Number of results to fetch per request. Lower this if you hit gRPC
                message size limits on responses. If None, uses the server default.

        Returns:
            A list of Channels that matches the filter criteria.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
            ),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                filter_query=filter_query,
            ),
        ]
        if channel_ids:
            filter_parts.append(cel.in_("channel_id", channel_ids))
        if asset is not None:
            asset_id = asset._id_or_error if isinstance(asset, Asset) else asset
            filter_parts.append(cel.equals("asset_id", asset_id))
        if assets:
            asset_ids = [
                asset._id_or_error if isinstance(asset, Asset) else asset for asset in assets
            ]
            filter_parts.append(cel.in_("asset_id", asset_ids))
        if run is not None:
            run_id = run.id_ if isinstance(run, Run) else run
            filter_parts.append(cel.equals("run_id", run_id))

        # This is opposite of usual archived state
        if archived is not None:
            filter_parts.append(cel.equals("active", not archived))

        query_filter = cel.and_(*filter_parts)

        channels = await self._low_level_client.list_all_channels(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(channels)

    async def find(self, **kwargs) -> Channel | None:
        """Find a single channel matching the given query. Takes the same arguments as `list`. If more than one channel is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Channel found or None.
        """
        channels = await self.list_(**kwargs)
        if len(channels) > 1:
            raise ValueError(f"Multiple ({len(channels)}) channels found for query")
        elif len(channels) == 1:
            return channels[0]
        return None

    async def update(
        self,
        channel: str | Channel,
        update: ChannelUpdate | dict,
    ) -> Channel:
        """Update a Channel.

        Args:
            channel: The Channel or channel ID to update.
            update: Updates to apply to the Channel. See ChannelUpdate for the updatable fields
                (description, unit, metadata, and archived status).

        Returns:
            The updated Channel.
        """
        channel_id = channel._id_or_error if isinstance(channel, Channel) else channel
        if isinstance(update, dict):
            update = ChannelUpdate.model_validate(update)
        # Resolve the caller's unit name to a unit id for the update.
        if update.unit:
            unit = await self._units_low_level_client.create_unit(update.unit)
            update = update.model_copy(update={"unit": unit.unit_id})
        update.resource_id = channel_id
        updated_channel = await self._low_level_client.update_channel(update=update)
        return self._apply_client_to_instance(updated_channel)

    async def archive(self, channels: list[str | Channel]) -> None:
        """Batch archive channels by setting active to false.

        Args:
            channels: List of channel IDs or Channel objects to archive. If a Channel
                has no id set, raises ValueError.
        """
        channel_ids = _channel_ids_from_list(channels)
        await self._low_level_client.batch_archive_channels(channel_ids)

    async def unarchive(self, channels: list[str | Channel]) -> None:
        """Batch unarchive channels by setting active to true.

        Args:
            channels: List of channel IDs or Channel objects to unarchive. If a Channel
                has no id set, raises ValueError.
        """
        channel_ids = _channel_ids_from_list(channels)
        await self._low_level_client.batch_unarchive_channels(channel_ids)

    def _ensure_data_low_level_client(self):
        """Ensure that the data low level client is initialized. Separated out like this to not require large dependencies (pandas/pyarrow) for the client if not fetching data."""
        if self._data_low_level_client is None:
            from sift_client._internal.low_level_wrappers.data import (
                ChannelCache,
                DataLowLevelClient,
            )

            kwargs: dict = {}
            disk_config = self._disk_cache_config
            if disk_config.enabled:
                # ``disk_path=None`` means "no cache" to ChannelCache; substitute
                # the default explicitly so the opt-out default still opens
                # the cache. ``DEFAULT_DISK_PATH`` is read here (not at
                # config construction) so test fixtures that monkeypatch the
                # class attribute see the override.
                kwargs["disk_cache_path"] = disk_config.path or ChannelCache.DEFAULT_DISK_PATH
                if disk_config.max_bytes is not None:
                    kwargs["disk_cache_max_bytes"] = disk_config.max_bytes
            try:
                self._data_low_level_client = DataLowLevelClient(
                    grpc_client=self.client.grpc_client,
                    **kwargs,
                )
            except Exception:
                # Explicit user-supplied paths failures propagate so the
                # caller knows their request didn't take. Default-path failures
                # (read-only ``/tmp``, restricted containers, etc.) degrade
                # silently to no-cache mode so ``get_data`` still works.
                if not disk_config.using_default_path:
                    raise
                logger.warning(
                    "Could not open the default channel data cache at %r; "
                    "falling back to no caching for ``get_data``. Call "
                    "``client.channels.disable_data_cache_disk()`` to silence "
                    "this warning, or pass an explicit path via "
                    "``enable_data_cache_disk(path=...)``.",
                    kwargs.get("disk_cache_path"),
                    exc_info=True,
                )
                self._data_low_level_client = DataLowLevelClient(
                    grpc_client=self.client.grpc_client,
                )

    async def get_data(
        self,
        *,
        channels: list[Channel],
        run: Run | str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
        limit: int | None = None,
        ignore_cache: bool = False,
    ) -> dict[str, pd.DataFrame]:
        """Get data for one or more channels.

        Args:
            channels: The channels to get data for.
            run: The Run or run_id to get data for.
            start_time: The start time to get data for.
            end_time: The end time to get data for.
            limit: The maximum number of data points to return. Will be in increments of page_size or default page size defined by the call if no page_size is provided.
            ignore_cache: Whether to ignore cached data and fetch fresh data from the server.

        Returns:
            A dictionary mapping channel names to pandas DataFrames containing the channel data.
        """
        self._ensure_data_low_level_client()

        run_id = run._id_or_error if isinstance(run, Run) else run
        return await self._data_low_level_client.get_channel_data(  # type: ignore
            channels=channels,
            run_id=run_id,
            start_time=start_time,
            end_time=end_time,
            max_results=limit,
            ignore_cache=ignore_cache,
        )

    async def get_data_as_arrow(
        self,
        *,
        channels: list[Channel],
        run: Run | str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
        limit: int | None = None,
        ignore_cache: bool = False,
    ) -> dict[str, pa.Table]:
        """Get data for one or more channels as pyarrow tables."""
        from pyarrow import Table as ArrowTable

        run_id = run.id_ if isinstance(run, Run) else run
        data = await self.get_data(
            channels=channels,
            run=run_id,
            start_time=start_time,
            end_time=end_time,
            limit=limit,
            ignore_cache=ignore_cache,
        )
        return {k: ArrowTable.from_pandas(v) for k, v in data.items()}
