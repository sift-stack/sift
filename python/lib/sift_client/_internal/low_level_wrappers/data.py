from __future__ import annotations

import asyncio
import logging
from datetime import datetime, timezone
from typing import TYPE_CHECKING, Any, Tuple, cast

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


TimeRange = Tuple[datetime, datetime]


class SegmentRef(BaseModel):
    """Pointer to one cached segment, stored on the per-bucket index."""

    model_config = ConfigDict(frozen=True)
    seg_id: int
    start_time: datetime
    end_time: datetime
    size_bytes: int


class SegmentIndex(BaseModel):
    """Per-(channel, run) index of cached segments."""

    model_config = ConfigDict(arbitrary_types_allowed=True)
    schema_version: int = 1
    next_seg_id: int = 0
    segments: list[SegmentRef] = []


class ChannelDataCache:
    """Channel-side adapter over the shared :class:`DiskCache` store.

    Each ``(channel_id, run_id)`` bucket is split across two key shapes
    in the underlying store:

    * One **index** entry (``channel:<run>:<id>:idx``) holding a
      :class:`SegmentIndex` — a tiny list of :class:`SegmentRef` ptrs
      describing every segment that exists for the bucket.
    * One **segment** entry per fetch (``channel:<run>:<id>:seg:<n>``)
      holding the :class:`ChannelCacheEntry` for that fetch's slice.

    Reads stitch the relevant segments together via :meth:`get_range`,
    which also reports which sub-ranges of the requested window have no
    cached coverage (``gaps``) so the caller can fetch only the holes.

    Eviction tolerance: ``diskcache``'s LRU can drop a segment while
    keeping the index. The reader treats a missing segment as a gap,
    which forces a wire fetch for that range. The opposite (index
    evicted, segments orphaned) is also fine — orphans are unreachable
    and will eventually LRU-evict.

    Attributes:
        KEY_PREFIX: Namespace prefix for every key this adapter writes
            to the shared :class:`DiskCache`. Picked at class scope so
            adapters in other resources can pick distinct prefixes
            without runtime negotiation.
        SCHEMA_VERSION: Index entry schema version. Bump when the
            shape changes incompatibly so older on-disk indexes are
            discarded on read rather than mis-deserialized.

    """

    KEY_PREFIX: str = "channel:"
    SCHEMA_VERSION: int = 1

    def __init__(self, store: DiskCache):
        """Wrap ``store`` with channel-data semantics.

        Args:
            store: The shared :class:`DiskCache` instance owned by the
                :class:`SiftClient`. Multiple adapters may share one store.
        """
        self._store = store
        self.name_id_map: dict[str, str] = {}

    # --- key helpers ---

    def _bucket_prefix(self, channel_id: str, run_id: str | None) -> str:
        # Stem all index/segment keys for a bucket share. Empty run
        # segment is safe because real run ids are UUIDs (never empty)
        # so there's no collision between the run-scoped and unscoped
        # buckets.
        return f"{self.KEY_PREFIX}{run_id or ''}:{channel_id}"

    def _index_key(self, channel_id: str, run_id: str | None) -> str:
        return f"{self._bucket_prefix(channel_id, run_id)}:idx"

    def _segment_key(self, channel_id: str, run_id: str | None, seg_id: int) -> str:
        return f"{self._bucket_prefix(channel_id, run_id)}:seg:{seg_id}"

    @property
    def store(self) -> DiskCache:
        """The shared underlying store. Tests reach in for store-level state."""
        return self._store

    # --- public API ---

    def has_any(self, channel_id: str, run_id: str | None = None) -> bool:
        """True if at least one segment is cached for this bucket.

        Cheap check (one index read) — does NOT touch segment data.
        Useful for "should I consult the cache at all?" gates.
        """
        idx = self._load_index(channel_id, run_id)
        return idx is not None and bool(idx.segments)

    def get_range(
        self,
        channel_id: str,
        run_id: str | None,
        start_time: datetime,
        end_time: datetime,
    ) -> tuple[pd.DataFrame | None, list[TimeRange]]:
        """Return cached data covering ``[start_time, end_time]`` plus gaps.

        Walks the bucket's segments, slices each one to the query range,
        stitches them together, and reports which sub-ranges still need
        a wire fetch.

        Returns:
            ``(stitched_data, gaps)`` where ``stitched_data`` is the
            concat of every overlapping segment sliced to the query
            range (or ``None`` if no rows were found), and ``gaps`` is
            the list of ``(gap_start, gap_end)`` sub-ranges within the
            query window not covered by any present segment.
            ``gaps == []`` means the cache fully covers the request.
        """
        idx = self._load_index(channel_id, run_id)
        if idx is None or not idx.segments:
            return None, [(start_time, end_time)]

        # Sort by start_time so the stitch order is deterministic.
        sorted_refs = sorted(idx.segments, key=lambda r: r.start_time)

        frames: list[pd.DataFrame] = []
        present_ranges: list[TimeRange] = []
        for ref in sorted_refs:
            # Skip non-overlapping segments cheaply (no segment load).
            if ref.end_time < start_time or ref.start_time > end_time:
                continue
            entry = self._load_segment(channel_id, run_id, ref.seg_id)
            if entry is None:
                # Evicted by diskcache LRU (or index/segment got out of
                # sync). Treat the segment's range as uncovered so the
                # caller refetches it.
                continue

            present_ranges.append((max(ref.start_time, start_time), min(ref.end_time, end_time)))
            sliced = entry.data[start_time:end_time]  # type: ignore[misc]
            if len(sliced) > 0:
                frames.append(sliced)

        gaps = self._compute_gaps(start_time, end_time, present_ranges)
        if not frames:
            return None, gaps
        if len(frames) == 1:
            return frames[0], gaps
        # Multiple segments → stitch. ``groupby.last()`` dedups any
        # boundary-overlapping timestamps and keeps the later segment's
        # value on conflict (sorted by start_time above).
        return pd.concat(frames).groupby(level=0).last(), gaps

    def put_segment(
        self,
        channel_id: str,
        run_id: str | None,
        data: pd.DataFrame,
        start_time: datetime,
        end_time: datetime,
    ) -> None:
        """Write a new segment and update the index.

        Args:
            channel_id: Parent channel id (bitfield elements all share
                the same id — group them upstream and pass one wide
                frame here).
            run_id: Per-run cache dimension; ``None`` for the unscoped
                bucket.
            data: This fetch's rows, indexed by tz-aware ``DatetimeIndex``.
            start_time: Claimed lower bound of cache coverage for this
                segment. Callers can claim more than ``data`` actually
                spans to record "we asked the wire about this range" and
                avoid re-fetching empty sub-ranges.
            end_time: Claimed upper bound; same semantics as start.

        Per-fetch disk write is O(this segment) — no rewrite of any
        already-cached segment.

        Write order is segment-first, then index: an interrupted update
        leaves an unreachable orphan segment (harmless; LRU-evicts) but
        never leaves the index pointing at a missing segment.
        """
        if len(data) == 0:
            # Skipping empty puts keeps the segment list shorter at the
            # cost of re-fetching no-data ranges. Acceptable trade for
            # the draft; see the class-level TODO.
            return

        size_bytes = int(data.memory_usage(deep=True).sum())
        idx = self._load_index(channel_id, run_id) or SegmentIndex(
            schema_version=self.SCHEMA_VERSION
        )
        seg_id = idx.next_seg_id

        entry = ChannelCacheEntry(
            data=data,
            start_time=start_time,
            end_time=end_time,
            size_bytes=size_bytes,
        )

        # Segment first.
        seg_key = self._segment_key(channel_id, run_id, seg_id)
        self._store.put(seg_key, entry, size_bytes=size_bytes)
        if seg_key not in self._store:
            # The store rejected the segment (oversize, disabled, etc.).
            # Skipping the index update keeps us from leaving a
            # dangling reference to a never-written segment.
            return

        idx.segments.append(
            SegmentRef(
                seg_id=seg_id,
                start_time=start_time,
                end_time=end_time,
                size_bytes=size_bytes,
            )
        )
        idx.next_seg_id += 1
        self._store.put(
            self._index_key(channel_id, run_id),
            idx,
            size_bytes=max(1024, 128 * len(idx.segments)),
        )

    def invalidate(self, channel_id: str, run_id: str | None = None) -> None:
        """Drop every segment in a bucket plus the index. Safe when absent.

        Only touches the one ``(channel_id, run_id)`` bucket — segments
        under other runs survive.
        """
        idx = self._load_index(channel_id, run_id)
        if idx is None:
            return
        for ref in idx.segments:
            self._store.invalidate(self._segment_key(channel_id, run_id, ref.seg_id))
        self._store.invalidate(self._index_key(channel_id, run_id))

    def clear(self) -> None:
        """Wipe every channel entry (all buckets, all runs)."""
        for key in list(self._store):
            if key.startswith(self.KEY_PREFIX):
                self._store.invalidate(key)

    # --- internal helpers ---

    def _load_index(self, channel_id: str, run_id: str | None) -> SegmentIndex | None:
        raw = self._store.get(self._index_key(channel_id, run_id))
        if not isinstance(raw, SegmentIndex):
            return None
        if raw.schema_version != self.SCHEMA_VERSION:
            # Future migration: discard incompatible on-disk indexes.
            return None
        return raw

    def _load_segment(
        self, channel_id: str, run_id: str | None, seg_id: int
    ) -> ChannelCacheEntry | None:
        raw = self._store.get(self._segment_key(channel_id, run_id, seg_id))
        if not isinstance(raw, ChannelCacheEntry):
            return None
        return raw

    @staticmethod
    def _compute_gaps(
        query_start: datetime,
        query_end: datetime,
        covered: list[TimeRange],
    ) -> list[TimeRange]:
        """Sub-ranges of ``[query_start, query_end]`` not in ``covered``.

        ``covered`` is the list of segment ranges (already clamped to
        the query window). Algorithm: merge overlapping/adjacent
        intervals, then sweep emitting gaps between them.
        """
        if not covered:
            return [(query_start, query_end)]

        sorted_ranges = sorted(covered, key=lambda r: r[0])
        merged: list[TimeRange] = [sorted_ranges[0]]
        for seg_start, seg_end in sorted_ranges[1:]:
            last_start, last_end = merged[-1]
            if seg_start <= last_end:
                merged[-1] = (last_start, max(last_end, seg_end))
            else:
                merged.append((seg_start, seg_end))

        gaps: list[TimeRange] = []
        cursor = query_start
        for seg_start, seg_end in merged:
            if seg_start > cursor:
                gaps.append((cursor, seg_start))
            cursor = max(cursor, seg_end)
        if cursor < query_end:
            gaps.append((cursor, query_end))
        return gaps


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

    def _update_cache(
        self,
        *,
        channel_data: dict[str, pd.DataFrame],
        start_time: datetime,
        end_time: datetime,
        run_id: str | None = None,
    ):
        """Write each channel's fresh data as a new segment.

        Per-fetch disk write is O(this segment) — no merging with prior
        segments and no re-pickle of accumulated data, so n sequential
        incremental pulls cost O(n) total disk write instead of O(n²).

        Bitfield grouping: ``try_deserialize_channel_data`` returns one
        dotted-name DataFrame per bitfield element, all mapping to the
        same parent ``channel_id`` via :attr:`name_id_map`. We group
        them by parent id and concat into one wide frame so each fetch
        produces exactly one segment per channel, regardless of how
        many elements that channel exposes.

        Empty data is skipped (no zero-row segments). This means a
        no-run query that returns nothing won't record coverage, so the
        next identical query will re-fetch — see the ``put_segment``
        TODO for "we tried, no data" segments.
        """
        assert start_time is not None
        assert end_time is not None
        name_id_map = self.channel_cache.name_id_map

        # Group dotted-name frames by parent channel id so bitfield
        # elements land in one segment.
        by_channel_id: dict[str, list[pd.DataFrame]] = {}
        for channel_name, data in channel_data.items():
            channel_id = name_id_map.get(channel_name)
            if not channel_id:
                raise ValueError(
                    f"{channel_name} not found in name_id_map. Not sure got "
                    f"data for this channel without a call that should've "
                    f"updated the map."
                )
            by_channel_id.setdefault(channel_id, []).append(data)

        for channel_id, frames in by_channel_id.items():
            if len(frames) == 1:
                combined = frames[0]
            else:
                # Bitfield: per-element single-column frames → one wide
                # frame. ``groupby.last`` dedups any boundary overlaps.
                combined = pd.concat(frames).groupby(level=0).last()

            if len(combined) == 0:
                continue

            # Segment coverage range. For run-scoped queries, claim
            # only what the data actually spans (we can't assert
            # absence outside the data — the run might not have
            # started yet). For unscoped queries, claim the full
            # requested range so a follow-up of the same range hits.
            if run_id:
                seg_start = combined.index[0]
                if not isinstance(seg_start, datetime):
                    seg_start = seg_start.to_pydatetime()
                seg_end = end_time
            else:
                seg_start = start_time
                seg_end = end_time

            self.channel_cache.put_segment(
                channel_id=channel_id,
                run_id=run_id,
                data=combined,
                start_time=seg_start,
                end_time=seg_end,
            )

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
        ret_data: dict[str, pd.DataFrame] = {}
        # No data will be returned if end_time is not provided.
        start_time = start_time or datetime.fromtimestamp(0, tz=timezone.utc)
        end_time = end_time or datetime.now(timezone.utc)

        self._update_name_id_map(channels)

        # Two work queues. Fully uncached channels share the full range
        # and get batched; partial-hit channels carry per-gap ranges
        # and go one fetch at a time.
        fully_uncached: list[str] = []
        partial_gaps: list[tuple[str, list[TimeRange]]] = []

        for channel in channels:
            cid = channel.id_
            assert cid is not None
            if ignore_cache:
                cached_data: pd.DataFrame | None = None
                gaps: list[TimeRange] = [(start_time, end_time)]
            else:
                cached_data, gaps = self.channel_cache.get_range(cid, run_id, start_time, end_time)

            if cached_data is not None:
                # Slice per column so each result key carries only its
                # own element frame (matches the per-element shape
                # ``try_deserialize_channel_data`` produces; without
                # this slice, a bitfield's wide cached frame would land
                # under every dotted key).
                for name in cached_data.columns:
                    ret_data[name] = cached_data[[name]]

            if not gaps:
                continue
            if len(gaps) == 1 and gaps[0] == (start_time, end_time):
                fully_uncached.append(cid)
            else:
                partial_gaps.append((cid, gaps))

        tasks = []
        # Batch fully-uncached channels (sharing the full requested
        # range) into one wire call each.
        batch_size = REQUEST_BATCH_SIZE
        for i in range(0, len(fully_uncached), batch_size):
            batch = fully_uncached[i : i + batch_size]
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

        # Partial gaps: one fetch per (channel, gap).
        for cid, gaps in partial_gaps:
            for gap_start, gap_end in gaps:
                task = asyncio.create_task(
                    self._handle_pagination(
                        self._get_data_impl,
                        kwargs={
                            "channel_ids": [cid],
                            "run_id": run_id,
                            "start_time": gap_start,
                            "end_time": gap_end,
                        },
                        page_size=page_size,
                        max_results=max_results,
                    )
                )
                tasks.append(task)

        pages = await asyncio.gather(*tasks)
        ret_data = self._merge_pages(pages, initial=ret_data)

        # Skip the cache update when no fresh pages arrived (pure cache
        # hit). Avoids the redundant "rewrite same bytes" case that
        # Tier 1A also targeted under the old single-entry shape.
        had_fresh_data = any(pages)
        if not ignore_cache and had_fresh_data:
            self._update_cache(
                channel_data=ret_data,
                start_time=start_time,
                end_time=end_time,
                run_id=run_id,
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
