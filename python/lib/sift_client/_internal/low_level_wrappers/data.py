from __future__ import annotations

import asyncio
import logging
from collections import OrderedDict
from datetime import datetime, timezone
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
    """LRU-ordered, byte-bounded cache of per-channel DataFrames.

    ``max_bytes <= 0`` disables retention: every ``get`` misses, ``put`` returns
    without storing.
    """

    def __init__(self, max_bytes: int = DEFAULT_DATA_CACHE_MAX_BYTES):
        if max_bytes < 0:
            raise ValueError(f"data_cache_max_bytes must be >= 0, got {max_bytes}")
        self.name_id_map: dict[str, str] = {}
        self._entries: OrderedDict[str, ChannelCacheEntry] = OrderedDict()
        self._total_bytes: int = 0
        self._max_bytes: int = max_bytes

    @property
    def enabled(self) -> bool:
        return self._max_bytes > 0

    @property
    def max_bytes(self) -> int:
        return self._max_bytes

    @max_bytes.setter
    def max_bytes(self, value: int) -> None:
        """Reconfigure the byte cap and immediately evict any excess.

        Used by ``ChannelsAPIAsync.configure_data_cache`` to retune a live
        cache. Lowering the cap below ``total_bytes`` triggers LRU eviction
        in the same loop ``put`` uses, so the invariant ``total_bytes <=
        max_bytes`` is restored before the setter returns.
        """
        if value < 0:
            raise ValueError(f"data_cache_max_bytes must be >= 0, got {value}")
        self._max_bytes = value
        self._evict_until_under_bound()

    @property
    def total_bytes(self) -> int:
        return self._total_bytes

    def __len__(self) -> int:
        return len(self._entries)

    def __contains__(self, channel_id: str) -> bool:
        return channel_id in self._entries

    def get(self, channel_id: str) -> ChannelCacheEntry | None:
        """Return the entry for ``channel_id`` if cached, otherwise None.

        Promotes the entry to most-recently-used on hit.
        """
        entry = self._entries.get(channel_id)
        if entry is not None:
            self._entries.move_to_end(channel_id)
        return entry

    def put(self, channel_id: str, entry: ChannelCacheEntry) -> None:
        """Insert or replace ``channel_id``, then evict LRU until within size bounds.

        Reclaims any prior entry's byte count BEFORE adding the new one's, so a
        re-insert (e.g. concat-merge of fresh data into an existing entry)
        accounts for the size delta correctly rather than double-counting.
        """
        if not self.enabled:
            return
        prior = self._entries.pop(channel_id, None)
        if prior is not None:
            self._total_bytes -= prior.size_bytes
        self._entries[channel_id] = entry
        self._total_bytes += entry.size_bytes
        self._evict_until_under_bound()

    def invalidate(self, channel_id: str) -> None:
        prior = self._entries.pop(channel_id, None)
        if prior is not None:
            self._total_bytes -= prior.size_bytes

    def clear(self) -> None:
        self._entries.clear()
        self._total_bytes = 0

    def _evict_until_under_bound(self) -> None:
        # ``popitem(last=False)`` drops the oldest entry. A single fresh entry
        # whose ``size_bytes`` alone exceeds ``max_bytes`` ends up evicted on
        # the final iteration.
        while self._entries and self._total_bytes > self._max_bytes:
            _, dropped = self._entries.popitem(last=False)
            self._total_bytes -= dropped.size_bytes


class DataLowLevelClient(LowLevelClientBase, WithGrpcClient):
    """Low-level client for fetching channel data.

    This class provides a thin wrapper around the autogenerated bindings for the DataAPI.
    """

    def __init__(
        self,
        grpc_client: GrpcClient,
        *,
        data_cache_max_bytes: int = DEFAULT_DATA_CACHE_MAX_BYTES,
    ):
        """Initialize the DataLowLevelClient.

        Args:
            grpc_client: The gRPC client to use for making API calls.
            data_cache_max_bytes: Cap on the in-memory channel-data cache (bytes).
                Set to ``0`` to disable caching. See ``ChannelCache``.
        """
        super().__init__(grpc_client)
        self.channel_cache = ChannelCache(max_bytes=data_cache_max_bytes)

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
