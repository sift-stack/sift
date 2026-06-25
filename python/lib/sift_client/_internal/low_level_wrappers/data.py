from __future__ import annotations

import asyncio
import logging
import os
import shutil
import tempfile
from collections import OrderedDict
from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING, Any, cast

import pandas as pd
from pydantic import BaseModel, ConfigDict
from sift.data.v2.data_pb2 import (
    BitFieldValues,
    ChannelQuery,
    GetDataRequest,
    GetDataResponse,
    Query,
)
from sift.data.v2.data_pb2_grpc import DataServiceStub

from sift_client._internal.low_level_wrappers.base import LowLevelClientBase
from sift_client._internal.time import to_timestamp_nanos
from sift_client.sift_types.channel import Channel, ChannelDataType
from sift_client.transport import WithGrpcClient

if TYPE_CHECKING:
    import diskcache

    from sift_client.transport.grpc_transport import GrpcClient

# Configure logging
logger = logging.getLogger(__name__)

CHANNELS_DEFAULT_PAGE_SIZE = 10_000
# TODO: There is a pagination issue API side when requesting multiple channels in single request.
# If all data points for all channels in a single request don't fit into a single page, then
# paging seems to omit all but a single channel. We can increase this batch size once that issue
# has been resolved. In the mean time each channel gets its own request.
REQUEST_BATCH_SIZE = 1

# Default in-memory budget for cached channel DataFrames, per ``DataLowLevelClient``
# instance. 512 MiB is well below typical limits while still letting common
# interactive workloads stay in cache. Override via ``SiftClient(data_cache_max_bytes=...)``.
DEFAULT_DATA_CACHE_MAX_BYTES = 512 * 1024 * 1024


class ChannelCacheEntry(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)
    data: pd.DataFrame
    start_time: datetime
    end_time: datetime
    size_bytes: int


def _new_cache_entry(
    data: pd.DataFrame, start_time: datetime, end_time: datetime
) -> ChannelCacheEntry:
    return ChannelCacheEntry(
        data=data,
        start_time=start_time,
        end_time=end_time,
        size_bytes=int(data.memory_usage(deep=True).sum()),
    )


class ChannelCache:
    """Two-tier cache of per-channel DataFrames.

    Tier 1: an LRU-ordered, byte-bounded in-memory dict (hot path). ``max_bytes
    <= 0`` disables this tier: ``get`` always misses memory, ``put`` doesn't
    populate it.

    Tier 2 (optional, see ``enable_disk``): a ``diskcache``-backed write-through
    layer that survives process restarts. When enabled, ``put`` writes to both
    tiers, ``get`` falls back to disk on a memory miss (promoting the hit back
    into memory), and ``invalidate``/``clear`` cascade to disk. The disk tier
    has its own byte cap that ``diskcache`` enforces with LRU eviction.

    The two tiers are independent: setting ``max_bytes=0`` keeps the disk layer
    active, useful for "cold storage only" workloads.
    """

    #: Default directory for the on-disk tier. Lives under
    #: ``tempfile.gettempdir()`` so it survives across sessions of the same
    #: user but doesn't pollute the user's home dir. The suffix is fixed so
    #: multiple processes (different ``SiftClient`` instances, notebooks, etc.)
    #: naturally share the same store and can read each other's prior sessions.
    DEFAULT_DISK_PATH: str = os.path.join(tempfile.gettempdir(), "sift-channel-data-cache")

    #: Default byte cap for the disk tier when ``enable_disk`` is called
    #: without an explicit ``max_bytes``. 4 GiB is a generous ceiling for the
    #: typical ``/tmp`` filesystem; ``diskcache`` enforces it with its own
    #: SQLite-backed LRU eviction once the bound is reached.
    DEFAULT_DISK_MAX_BYTES: int = 4 * 1024 * 1024 * 1024

    #: Marker file ``diskcache`` writes inside every cache directory. We
    #: sanity-check for this before any ``shutil.rmtree`` so a typo in the
    #: ``clear_disk`` ``path`` argument can't wipe out an unrelated directory.
    _DISKCACHE_MARKER: str = "cache.db"

    def __init__(
        self,
        max_bytes: int = DEFAULT_DATA_CACHE_MAX_BYTES,
        *,
        disk_path: str | os.PathLike[str] | None = None,
        disk_max_bytes: int | None = None,
    ):
        """Construct an in-memory cache, optionally backed by disk.

        Args:
            max_bytes: Byte cap on the in-memory tier. ``0`` disables it.
            disk_path: Directory for the disk tier. ``None`` (the default)
                disables disk. A previously-populated directory is reused,
                so subsequent sessions can read from existing entries.
            disk_max_bytes: Byte cap on the disk tier. ``None`` falls back to
                ``DEFAULT_DISK_MAX_BYTES``. Ignored when ``disk_path`` is
                ``None``.
        """
        if max_bytes < 0:
            raise ValueError(f"data_cache_max_bytes must be >= 0, got {max_bytes}")
        self.name_id_map: dict[str, str] = {}
        self._entries: OrderedDict[str, ChannelCacheEntry] = OrderedDict()
        self._total_bytes: int = 0
        self._max_bytes: int = max_bytes
        # Channels we've already logged an "entry exceeds tier cap" warning
        # for. The check on the put path would otherwise spam the log once
        # per ``get_data`` call for any channel whose typical entry is bigger
        # than the cap. A successful normal put for the same channel clears
        # the bit so a future regression re-warns.
        self._oversized_memory_warned: set[str] = set()
        self._oversized_disk_warned: set[str] = set()
        self._disk: diskcache.Cache | None = None
        self._disk_path: str | None = None
        self._disk_max_bytes: int | None = None
        if disk_path is not None:
            self._open_disk(
                str(disk_path),
                disk_max_bytes if disk_max_bytes is not None else self.DEFAULT_DISK_MAX_BYTES,
            )

    @classmethod
    def clear_disk(cls, path: str | os.PathLike[str] | None = None) -> None:
        """Delete a previously-persisted on-disk cache directory.

        Use this to drop stale caches from previous sessions, recover from a
        corrupt cache, or reclaim disk space. The directory is removed
        entirely; a future ``enable_disk`` call at the same path will see a
        fresh empty cache.

        Args:
            path: Directory of the cache to clear. ``None`` (the default)
                targets :attr:`DEFAULT_DISK_PATH`.

        Raises:
            ValueError: If ``path`` exists but does not look like a sift
                channel data cache directory (missing the ``diskcache``
                marker file). This guard makes accidental misuse a hard
                error rather than silent data loss.
        """
        target = Path(path) if path is not None else Path(cls.DEFAULT_DISK_PATH)
        if not target.exists():
            return
        if not (target / cls._DISKCACHE_MARKER).exists():
            raise ValueError(
                f"{str(target)!r} does not look like a sift channel data cache "
                f"directory (missing {cls._DISKCACHE_MARKER!r} marker). "
                f"Refusing to delete."
            )
        shutil.rmtree(target)

    @property
    def enabled(self) -> bool:
        """Whether the in-memory tier accepts writes (``max_bytes > 0``)."""
        return self._max_bytes > 0

    @property
    def max_bytes(self) -> int:
        return self._max_bytes

    @max_bytes.setter
    def max_bytes(self, value: int) -> None:
        """Reconfigure the in-memory byte cap and immediately evict any excess.

        Used by ``ChannelsAPIAsync.configure_data_cache`` to retune a live
        cache. Lowering the cap below ``total_bytes`` triggers LRU eviction
        in the same loop ``put`` uses, so the invariant ``total_bytes <=
        max_bytes`` is restored before the setter returns. Does not touch
        the disk tier.
        """
        if value < 0:
            raise ValueError(f"data_cache_max_bytes must be >= 0, got {value}")
        self._max_bytes = value
        self._evict_until_under_bound()

    @property
    def total_bytes(self) -> int:
        return self._total_bytes

    @property
    def disk_enabled(self) -> bool:
        """Whether the disk-backed second-chance tier is currently open."""
        return self._disk is not None

    @property
    def disk_path(self) -> str | None:
        """Filesystem path of the disk tier when enabled, else ``None``."""
        return self._disk_path

    @property
    def disk_max_bytes(self) -> int | None:
        """Configured byte cap on the disk tier, or ``None`` when disabled."""
        return self._disk_max_bytes

    def __len__(self) -> int:
        return len(self._entries)

    def __contains__(self, channel_id: str) -> bool:
        """True if the channel is cached in memory OR on disk.

        Used by ``_filter_cached_channels`` to decide whether ``get_data``
        needs to hit the wire. Including the disk tier here lets a fresh
        session served by a warm disk avoid re-fetching.
        """
        if channel_id in self._entries:
            return True
        if self._disk is not None and channel_id in self._disk:
            return True
        return False

    def enable_disk(
        self,
        *,
        path: str | os.PathLike[str] | None = None,
        max_bytes: int | None = None,
    ) -> None:
        """Enable (or reconfigure) the disk-backed second-chance tier.

        If a previous disk tier was open at a different path or with a
        different size cap, it's closed first. Memory contents are left
        intact; they are NOT replayed to disk so disk reflects only future
        writes.

        Args:
            path: Directory to persist to. ``None`` uses
                :attr:`DEFAULT_DISK_PATH`. The directory is created if
                missing; an existing one is opened in place and its
                contents become available to ``get``.
            max_bytes: Byte cap for the disk tier (``None`` →
                :attr:`DEFAULT_DISK_MAX_BYTES`).
        """
        target_path = str(path) if path is not None else self.DEFAULT_DISK_PATH
        target_max = max_bytes if max_bytes is not None else self.DEFAULT_DISK_MAX_BYTES
        if (
            self._disk is not None
            and self._disk_path == target_path
            and self._disk_max_bytes == target_max
        ):
            return
        self._close_disk()
        self._open_disk(target_path, target_max)

    def disable_disk(self) -> None:
        """Close the disk tier (if open). Does not touch the disk contents.

        Use ``sift_client.clear_data_cache_on_disk(path)`` to remove a
        directory from disk.
        """
        self._close_disk()

    def get(self, channel_id: str) -> ChannelCacheEntry | None:
        """Return the entry for ``channel_id`` if cached, otherwise None.

        Memory is consulted first; on a miss, the disk tier (if enabled) is
        checked. A disk hit is promoted back into memory (subject to the
        in-memory cap) so subsequent accesses stay hot.
        """
        entry = self._entries.get(channel_id)
        if entry is not None:
            self._entries.move_to_end(channel_id)
            return entry
        if self._disk is None:
            return None
        try:
            disk_entry = self._disk.get(channel_id, default=None, retry=True)
        except Exception:
            # diskcache surfaces ``sqlite3.DatabaseError`` (and friends) for
            # corrupt or partially-written entries from a prior session.
            # Treat as a miss; force ``invalidate`` to drop the bad row so
            # we don't repeatedly trip the same path.
            logger.warning("disk cache read failed for %s; invalidating", channel_id)
            try:
                del self._disk[channel_id]
            except Exception:
                pass
            return None
        if disk_entry is None or not isinstance(disk_entry, ChannelCacheEntry):
            return None
        if self.enabled:
            # Promote disk hit into memory so subsequent reads are cheap.
            self._put_memory(channel_id, disk_entry)
        return disk_entry

    def put(self, channel_id: str, entry: ChannelCacheEntry) -> None:
        """Insert or replace ``channel_id`` in memory (if enabled) and on disk.

        Memory reclaims any prior entry's byte count BEFORE adding the new
        one's, so a re-insert (e.g. concat-merge of fresh data into an
        existing entry) accounts for the size delta correctly. Disk writes
        replace the prior row.
        """
        if self.enabled:
            self._put_memory(channel_id, entry)
        if self._disk is not None:
            if (
                self._disk_max_bytes is not None
                and entry.size_bytes > self._disk_max_bytes
            ):
                if channel_id not in self._oversized_disk_warned:
                    logger.warning(
                        "Channel %s data (%d bytes) is larger than the disk "
                        "cache cap (%d bytes); skipping disk cache for this "
                        "channel so other entries aren't evicted. Raise the "
                        "cap via ``client.channels.enable_data_cache_disk("
                        "max_bytes=...)`` to cache this channel on disk.",
                        channel_id,
                        entry.size_bytes,
                        self._disk_max_bytes,
                    )
                    self._oversized_disk_warned.add(channel_id)
                try:
                    self._disk.delete(channel_id, retry=True)
                except Exception:
                    pass
                return
            try:
                self._disk.set(channel_id, entry, retry=True)
                self._oversized_disk_warned.discard(channel_id)
            except Exception:
                # Best-effort persistence: keep going on disk errors so the
                # in-memory cache (and the user's ``get_data`` call) still
                # succeeds. Drop the (possibly partial) disk row.
                logger.warning("disk cache write failed for %s; invalidating", channel_id)
                try:
                    self._disk.delete(channel_id, retry=True)
                except Exception:
                    pass

    def invalidate(self, channel_id: str) -> None:
        prior = self._entries.pop(channel_id, None)
        if prior is not None:
            self._total_bytes -= prior.size_bytes
        # Invalidation is a fresh start for this channel; if it was warned
        # about as oversized previously, the next put should re-evaluate
        # against the current cap and re-warn if still too big.
        self._oversized_memory_warned.discard(channel_id)
        self._oversized_disk_warned.discard(channel_id)
        if self._disk is not None:
            try:
                self._disk.delete(channel_id, retry=True)
            except Exception:
                pass

    def clear(self) -> None:
        self._entries.clear()
        self._total_bytes = 0
        self._oversized_memory_warned.clear()
        self._oversized_disk_warned.clear()
        if self._disk is not None:
            self._disk.clear()

    def close(self) -> None:
        """Release the disk-tier file handle. Safe to call without disk enabled."""
        self._close_disk()

    def _put_memory(self, channel_id: str, entry: ChannelCacheEntry) -> None:
        """Memory-tier insert + eviction. Caller has already gated on ``enabled``.
        """
        prior = self._entries.pop(channel_id, None)
        if prior is not None:
            self._total_bytes -= prior.size_bytes
        if entry.size_bytes > self._max_bytes:
            if channel_id not in self._oversized_memory_warned:
                logger.warning(
                    "Channel %s data (%d bytes) is larger than the in-memory "
                    "cache cap (%d bytes); skipping cache for this channel so "
                    "other entries aren't evicted. Raise the cap via "
                    "``client.channels.configure_data_cache(max_bytes=...)`` "
                    "to cache this channel.",
                    channel_id,
                    entry.size_bytes,
                    self._max_bytes,
                )
                self._oversized_memory_warned.add(channel_id)
            return
        self._oversized_memory_warned.discard(channel_id)
        self._entries[channel_id] = entry
        self._total_bytes += entry.size_bytes
        self._evict_until_under_bound()

    def _evict_until_under_bound(self) -> None:
        # ``popitem(last=False)`` drops the oldest entry. A single fresh entry
        # whose ``size_bytes`` alone exceeds ``max_bytes`` ends up evicted on
        # the final iteration.
        while self._entries and self._total_bytes > self._max_bytes:
            _, dropped = self._entries.popitem(last=False)
            self._total_bytes -= dropped.size_bytes

    def _open_disk(self, path: str, max_bytes: int) -> None:
        import diskcache

        os.makedirs(path, exist_ok=True)
        # ``least-recently-used`` matches the in-memory tier's eviction policy;
        # statistics/tag_index are off because we only need plain k/v reads.
        self._disk = diskcache.Cache(
            directory=path,
            size_limit=max_bytes,
            eviction_policy="least-recently-used",
            statistics=0,
            tag_index=0,
        )
        self._disk_path = path
        self._disk_max_bytes = max_bytes

    def _close_disk(self) -> None:
        if self._disk is None:
            return
        try:
            self._disk.close()
        except Exception:
            pass
        self._disk = None
        self._disk_path = None
        self._disk_max_bytes = None


class DataLowLevelClient(LowLevelClientBase, WithGrpcClient):
    """Low-level client for fetching channel data.

    This class provides a thin wrapper around the autogenerated bindings for the DataAPI.
    """

    def __init__(
        self,
        grpc_client: GrpcClient,
        *,
        data_cache_max_bytes: int = DEFAULT_DATA_CACHE_MAX_BYTES,
        disk_cache_path: str | os.PathLike[str] | None = None,
        disk_cache_max_bytes: int | None = None,
    ):
        """Initialize the DataLowLevelClient.

        Args:
            grpc_client: The gRPC client to use for making API calls.
            data_cache_max_bytes: Cap on the in-memory channel-data cache (bytes).
                Set to ``0`` to disable in-memory caching. See ``ChannelCache``.
            disk_cache_path: Directory for the disk-backed second-chance tier.
                ``None`` disables disk persistence. See ``ChannelCache``.
            disk_cache_max_bytes: Byte cap for the disk tier. ``None`` uses
                ``DEFAULT_DISK_CACHE_MAX_BYTES``. Ignored when
                ``disk_cache_path`` is ``None``.
        """
        super().__init__(grpc_client)
        self.channel_cache = ChannelCache(
            max_bytes=data_cache_max_bytes,
            disk_path=disk_cache_path,
            disk_max_bytes=disk_cache_max_bytes,
        )

    def _update_name_id_map(self, channels: list[Channel]):
        """Update the name id map with the new channels."""
        for channel in channels:
            if channel.bit_field_elements:
                for bit_field_element in channel.bit_field_elements:
                    self.channel_cache.name_id_map[channel.name + "." + bit_field_element.name] = (
                        str(channel.id_)
                    )
            self.channel_cache.name_id_map[channel.name] = str(channel.id_)

    # TODO: Cache calls. Only read cache if end_time is more than 30 min in the past.
    #       Also, consider manually caching full channel data and evaluating start/end times while ignoring pagination. Do this ful caching at a higher  level though to handle case where pagination fails.
    async def _get_data_impl(
        self,
        *,
        channel_ids: list[str],
        run_id: str | None = None,
        start_time: datetime | None = None,
        end_time: datetime,
        page_size: int | None = None,
        page_token: str | None = None,
        order_by: str | None = None,
    ) -> tuple[list[Any], str | None]:
        """Get the data for a channel during a run."""
        queries = [
            Query(channel=ChannelQuery(channel_id=channel_id, run_id=run_id))
            for channel_id in channel_ids
        ]
        request_kwargs: dict[str, Any] = {
            "queries": queries,
            "sample_ms": 0,
            "start_time": start_time,
            "end_time": end_time,
            "page_size": page_size,
            "page_token": page_token,
        }

        request = GetDataRequest(**request_kwargs)
        response = await self._grpc_client.get_stub(DataServiceStub).GetData(request)
        response = cast("GetDataResponse", response)
        return response.data, response.next_page_token  # type: ignore # mypy doesn't know RepeatedCompositeFieldContainer can be treated like a list

    def _filter_cached_channels(self, channel_ids: list[str]) -> tuple[list[str], list[str]]:
        cached_channels = []
        not_cached_channels = []
        for channel_id in channel_ids:
            if channel_id in self.channel_cache:
                cached_channels.append(channel_id)
            else:
                not_cached_channels.append(channel_id)
        return cached_channels, not_cached_channels

    def _check_cache(
        self,
        *,
        channel_id: str,
        start_time: datetime,
        end_time: datetime,
        run_id: str | None = None,
    ) -> tuple[pd.DataFrame | None, datetime | None, datetime | None]:
        """Check if the data for a channel during a run is cached and return how to query remaining data if so.

        There are a variety of requested start/end time vs cached start/end time cases to consider.
        Below diagram represents time aligned ranges for each case:

        Cache interval:               |-------------------------------|
        Case 1:                         |---------------------------|
        Case 2:                              |--------------------------------|
        Case 3:                                                           |----------|
        Case 4:                 |--------------------------------|
        Case 5:         |------| or |-----------------------------------------|

        Returns:
            A tuple of (data, start_time, end_time)
            where data is a pandas dataframe and start and end times are what should be used for the next call based on what is not covered by the cached data.
        """
        cached_data = self.channel_cache.get(channel_id)
        ret_start_time = start_time
        ret_end_time = end_time
        ret_data = None
        if cached_data:
            start_time_cached = cached_data.start_time
            end_time_cached = cached_data.end_time
            ret_data = cached_data.data
            # Filter data to desiredtime range
            ret_data = ret_data[start_time:end_time]  # type: ignore # mypy doesn't understand pandas that well seemingly

            if start_time_cached <= start_time:
                if start_time < end_time_cached:
                    if end_time <= end_time_cached:
                        # Case 1
                        ret_start_time = None  # type: ignore
                        ret_end_time = None  # type: ignore
                    else:
                        # Case 2
                        ret_start_time = end_time_cached
                        ret_end_time = end_time
                else:
                    # Case 3
                    return (None, start_time, end_time)
            else:
                if start_time_cached < end_time and end_time <= end_time_cached:
                    # Case 4
                    ret_start_time = start_time
                    ret_end_time = start_time_cached
                else:
                    # Case 5
                    return (None, start_time, end_time)

        return (ret_data, ret_start_time, ret_end_time)

    def _update_cache(
        self,
        *,
        channel_data: dict[str, pd.DataFrame],
        start_time: datetime,
        end_time: datetime,
        run_id: str | None = None,
    ):
        """Update the cache with the new data and start/end times."""
        assert start_time is not None
        assert end_time is not None
        name_id_map = self.channel_cache.name_id_map

        for channel_name, data in channel_data.items():
            channel_id = name_id_map.get(channel_name)
            if not channel_id:
                raise ValueError(
                    f"{channel_name} not found in name_id_map. Not sure got data for this channel without a call that should've updated the map."
                )

            suggested_start_time = start_time
            if run_id:
                if len(data) > 0:
                    suggested_start_time = data.index[0]
                else:
                    # Because we didn't get any data, we can't know what the start time should be.
                    # And because this was queried w/ a run ID, we can't say there's no data before the run started.
                    # So we just don't update the cache.
                    continue

            existing = self.channel_cache.get(channel_id)
            if existing is not None:
                merged_data = pd.concat([existing.data, data]).groupby(level=0).last()
                entry = _new_cache_entry(
                    data=merged_data,
                    start_time=min(suggested_start_time, existing.start_time),
                    end_time=max(end_time, existing.end_time),
                )
            else:
                entry = _new_cache_entry(
                    data=data,
                    start_time=suggested_start_time,
                    end_time=end_time,
                )
            self.channel_cache.put(channel_id, entry)

    async def get_channel_data(
        self,
        *,
        channels: list[Channel],
        run_id: str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
        max_results: int | None = None,
        page_size: int | None = None,
        ignore_cache: bool = False,
    ) -> dict[str, pd.DataFrame]:
        """Get the data for a channel during a run."""
        ret_data = {}
        # No data will be returned if end_time is not provided.
        start_time = start_time or datetime.fromtimestamp(0, tz=timezone.utc)
        end_time = end_time or datetime.now(timezone.utc)

        self._update_name_id_map(channels)
        channel_ids = [c.id_ for c in channels]
        cached_channels, not_cached_channels = (
            ([], channel_ids) if ignore_cache else self._filter_cached_channels(channel_ids)  # type: ignore
        )

        tasks = []
        # Queue up calls for non-cached channels in batches.
        batch_size = REQUEST_BATCH_SIZE
        for i in range(0, len(not_cached_channels), batch_size):  # type: ignore
            batch = not_cached_channels[i : i + batch_size]  # type: ignore

            task = asyncio.create_task(
                self._handle_pagination(
                    self._get_data_impl,
                    kwargs={
                        "channel_ids": batch,
                        "run_id": run_id,
                        "start_time": start_time,
                        "end_time": end_time,
                    },
                    page_size=page_size,
                    max_results=max_results,
                )
            )
            tasks.append(task)

        # Handling cached channels 1 by 1 instead of in batches to account for channels that may have been cached from calls with different start/end times.
        for channel_id in cached_channels:
            cached_data, new_start_time, new_end_time = self._check_cache(
                channel_id=channel_id,
                start_time=start_time,
                end_time=end_time,
                run_id=run_id,
            )

            if cached_data is not None:
                for name in cached_data.columns:
                    ret_data[name] = cached_data
                if new_start_time is None:
                    # Cache fully encompassed the desired time range so don't queue a call.
                    continue
            task = asyncio.create_task(
                self._handle_pagination(
                    self._get_data_impl,
                    kwargs={
                        "channel_ids": [channel_id],
                        "run_id": run_id,
                        "start_time": new_start_time,
                        "end_time": new_end_time or end_time,
                    },
                    page_size=page_size,
                    max_results=max_results,
                )
            )
            tasks.append(task)

        pages = await asyncio.gather(*tasks)
        ret_data = self._merge_pages(pages, initial=ret_data)

        if not ignore_cache:
            self._update_cache(
                channel_data=ret_data, start_time=start_time, end_time=end_time, run_id=run_id
            )

        return ret_data

    def _merge_pages(
        self,
        pages: list[list[Any]],
        *,
        initial: dict[str, pd.DataFrame],
    ) -> dict[str, pd.DataFrame]:
        """Flatten paged channel data + any cached slices into one DataFrame per channel.

        ``initial`` carries any cached slices already populated by
        ``_check_cache``. Cached entries are folded in as the first frame for
        their channel so they participate in the same final concat;
        ``groupby(level=0).last()`` preserves the previous behavior of letting
        a later-positioned (fresher) value win on duplicate timestamps.
        """
        per_channel_frames: dict[str, list[pd.DataFrame]] = {}
        for page in pages:
            for data in page:
                for name, df in self.try_deserialize_channel_data(data).items():
                    per_channel_frames.setdefault(name, []).append(df)

        ret_data: dict[str, pd.DataFrame] = dict(initial)
        for name, frames in per_channel_frames.items():
            if name in ret_data:
                # Cached slice goes first so fresher pages (positioned later
                # in the list) win on overlapping timestamps after groupby.
                frames.insert(0, ret_data[name])
            if len(frames) == 1:
                ret_data[name] = frames[0]
            else:
                ret_data[name] = pd.concat(frames).groupby(level=0).last()
        return ret_data

    @staticmethod
    def try_deserialize_channel_data(channel_data: Any) -> dict[str, pd.DataFrame]:
        """Deserialize a channel data object into a numpy array."""
        data_type = ChannelDataType.from_str(channel_data.type_url)
        if data_type is None:
            raise ValueError(f"Unknown data type: {channel_data.type_url}")

        proto_data_class = ChannelDataType.proto_data_class(data_type)
        proto_data_value = proto_data_class.FromString(channel_data.value)
        metadata = proto_data_value.metadata
        ret_data = {}

        components = (
            proto_data_value.values if proto_data_class is BitFieldValues else [proto_data_value]
        )
        for component in components:
            name = metadata.channel.name
            time_column = []
            value_column = []
            if proto_data_class is BitFieldValues:
                name += "." + component.name
            for value_obj in component.values:
                time_column.append(to_timestamp_nanos(value_obj.timestamp))
                value_column.append(value_obj.value)
            df = pd.DataFrame({name: value_column}, index=time_column)
            ret_data[name] = df

        return ret_data
