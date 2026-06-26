from __future__ import annotations

import asyncio
import logging
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

from sift_client._internal.disk_cache import DiskCache
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


class ChannelDataCache:
    """Channel-side adapter over the shared :class:`DiskCache` store.

    The store is owned by :class:`~sift_client.client.SiftClient` and
    shared by every cache-aware resource; this adapter is the typed,
    namespaced view of it that the channel data path uses.

    Responsibilities the adapter holds onto:

    * **Key namespacing.** Every read/write goes through :meth:`_key`,
      which prefixes the channel id with ``channel:``. That keeps a
      future calculated-channels or exports adapter on the same store
      from colliding on raw resource ids.
    * **Typing.** ``put`` only accepts :class:`ChannelCacheEntry`;
      ``get`` ``isinstance``-checks the raw value before handing it back,
      so a corrupt or cross-adapter row reads as a miss instead of
      blowing up downstream pandas code.
    * **Size measurement.** The store stays value-agnostic; the adapter
      already computes ``size_bytes`` on the entry via
      :func:`_new_cache_entry` (``DataFrame.memory_usage(deep=True)``) so
      it just forwards that to the store's oversize guard.
    * **Resource-side state.** :attr:`name_id_map` lives here because
      it's channel-specific bookkeeping needed to wire raw fetch
      responses (keyed by channel *name*) back to the cache (keyed by
      channel *id*).

    The :class:`DiskCacheAdapter` ``Protocol`` is intentionally not
    declared yet — there's only one adapter shape so far. When a second
    resource grows its own adapter, extract the Protocol from the two
    real shapes rather than guessing from one.
    """

    #: Namespace prefix for keys this adapter writes to the shared
    #: :class:`DiskCache`. Picked at class scope so adapters in other
    #: resources can pick distinct prefixes without runtime negotiation.
    KEY_PREFIX: str = "channel:"

    def __init__(self, store: DiskCache):
        """Wrap ``store`` with channel-data semantics.

        Args:
            store: The shared :class:`DiskCache` instance owned by the
                :class:`SiftClient`. Multiple adapters may share one store.
        """
        self._store = store
        self.name_id_map: dict[str, str] = {}

    def _key(self, channel_id: str) -> str:
        return f"{self.KEY_PREFIX}{channel_id}"

    @property
    def store(self) -> DiskCache:
        """The shared underlying store. Tests reach in for store-level state."""
        return self._store

    def __contains__(self, channel_id: str) -> bool:
        """True if the channel is cached. False when the store is disabled."""
        return self._key(channel_id) in self._store

    def get(self, channel_id: str) -> ChannelCacheEntry | None:
        """Return the entry for ``channel_id`` if cached, otherwise None.

        Type-checks the raw value before returning so a row written by a
        different adapter (or a corrupt entry that survived) reads as a
        miss instead of being handed back as the wrong type.
        """
        raw = self._store.get(self._key(channel_id))
        if not isinstance(raw, ChannelCacheEntry):
            return None
        return raw

    def put(self, channel_id: str, entry: ChannelCacheEntry) -> None:
        """Insert or replace ``channel_id`` on disk.

        Forwards :attr:`ChannelCacheEntry.size_bytes` to the store so its
        oversize guard can decide whether to write or skip+warn. No-op
        when the underlying store is disabled.
        """
        self._store.put(self._key(channel_id), entry, size_bytes=entry.size_bytes)

    def invalidate(self, channel_id: str) -> None:
        """Remove ``channel_id`` from the cache. Safe when absent."""
        self._store.invalidate(self._key(channel_id))

    def clear(self) -> None:
        """Wipe every channel entry. Other adapters' entries are preserved.

        Walks the shared store's keyspace once and drops anything under
        :attr:`KEY_PREFIX`. ``list(...)`` snapshots the iterator since
        we mutate during iteration.
        """
        for key in list(self._store):
            if key.startswith(self.KEY_PREFIX):
                self._store.invalidate(key)


class DataLowLevelClient(LowLevelClientBase, WithGrpcClient):
    """Low-level client for fetching channel data.

    This class provides a thin wrapper around the autogenerated bindings for the DataAPI.
    """

    def __init__(
        self,
        grpc_client: GrpcClient,
        *,
        channel_cache: ChannelDataCache | None = None,
    ):
        """Initialize the DataLowLevelClient.

        Args:
            grpc_client: The gRPC client to use for making API calls.
            channel_cache: Adapter wrapping the shared :class:`DiskCache` the
                :class:`SiftClient` owns. When ``None`` (only the unit-test
                construction path), the wrapper falls back to a no-op store
                so cache reads/writes are silent. Production callers always
                pass an adapter built from ``client._get_disk_cache()``.
        """
        super().__init__(grpc_client)
        # Production wires the shared store in via the resource. The fallback
        # here lets a bare ``DataLowLevelClient(MagicMock())`` keep working
        # in unit tests without forcing every site to plumb a store.
        if channel_cache is None:
            channel_cache = ChannelDataCache(DiskCache())
        self.channel_cache = channel_cache

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
