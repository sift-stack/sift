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


TimeRange = Tuple[datetime, datetime]


class SegmentRef(BaseModel):
    """Pointer to one cached segment, stored on the per-bucket index.

    ``seg_id`` is ``None`` for an **empty ref** — a record of "we
    queried this range and the wire returned no data". Empty refs
    contribute to coverage (so a repeat of a known-empty range
    doesn't hit the wire).
    """

    model_config = ConfigDict(frozen=True)
    seg_id: int | None
    start_time: datetime
    end_time: datetime


class SegmentIndex(BaseModel):
    """Per-(channel, run) index of cached segments."""

    model_config = ConfigDict(arbitrary_types_allowed=True)
    next_seg_id: int = 0
    segments: list[SegmentRef] = []


class ChannelDataCache:
    """Channel-side adapter over the shared :class:`DiskCache` store.

    Each ``(channel_id, run_id)`` bucket is split across two key shapes
    in the underlying store:

    * One **index** entry (``channel:v1:<run>:<id>:idx``) holding a
      :class:`SegmentIndex` — a tiny list of :class:`SegmentRef` ptrs
      describing every segment that exists for the bucket. Some refs
      are *empty* (``seg_id is None``) — they record "we queried this
      range and the wire returned no data" without a backing segment.
    * One **segment** entry per fetch with data
      (``channel:v1:<run>:<id>:seg:<n>``) holding the
      :class:`pandas.DataFrame` for that fetch's slice. The
      :class:`SegmentRef` on the index already carries the claimed
      time range and is the source of truth for coverage, so segment
      bodies are stored as raw frames.

    Reads stitch the relevant segments together via :meth:`get_range`,
    which also reports which sub-ranges of the requested window have no
    cached coverage (``gaps``) so the caller can fetch only the holes.

    Eviction tolerance: ``diskcache``'s LRU can drop a segment while
    keeping the index. The reader treats a missing segment as a gap,
    which forces a wire fetch for that range. The opposite (index
    evicted, segments orphaned) is also fine — orphans are unreachable
    and will eventually LRU-evict.

    Attributes:
        KEY_PREFIX: Versioned namespace prefix for every key this
            adapter writes to the shared :class:`DiskCache`. Picked at
            class scope so adapters in other resources can pick
            distinct prefixes without runtime negotiation. The trailing
            ``v<N>`` component is the schema version: bump it (e.g.
            from ``"channel:v1:"`` to ``"channel:v2:"``) when either
            entry shape changes incompatibly so old keys are silently
            unreachable rather than mis-deserialized.
        MAX_SEGMENTS_PER_BUCKET: Per-bucket cap on segment count
            before :meth:`_compact_bucket` folds them into one.
            Without compaction a long incrementally-pulled run grows
            ``len(idx.segments)`` without bound, so every full-range
            ``get_range`` would load and concat all segments —
            swapping the old O(n^2) write for unbounded read fan-out.
            The cap bounds that fan-out at a known constant.

    """

    KEY_PREFIX: str = "channel:v1:"
    MAX_SEGMENTS_PER_BUCKET: int = 16

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

            clamped = (max(ref.start_time, start_time), min(ref.end_time, end_time))

            if ref.seg_id is None:
                # Empty ref: the wire was asked about this range and
                # returned no data. Count it toward coverage so the
                # caller doesn't refetch.
                present_ranges.append(clamped)
                continue

            frame = self._load_segment(channel_id, run_id, ref.seg_id)
            if frame is None:
                # Evicted by diskcache LRU (or index/segment got out of
                # sync). Treat the segment's range as uncovered so the
                # caller refetches it.
                continue

            present_ranges.append(clamped)
            sliced = frame[start_time:end_time]  # type: ignore[misc]
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
            data: This fetch's rows, indexed by tz-aware
                ``DatetimeIndex``.
            start_time: Claimed lower bound of cache coverage for this
                segment. Callers can claim more than ``data`` actually
                spans to record "we asked the wire about this range" and
                avoid re-fetching empty sub-ranges.
            end_time: Claimed upper bound; same semantics as start.

        Per-fetch disk write is O(this segment) — no rewrite of any
        already-cached segment.

        Write order is segment-first, then index: an interrupted update
        leaves an unreachable orphan segment (harmless; LRU-evicts) but
        never leaves the index pointing at a missing segment. Empty
        puts skip the segment write entirely and update the index
        directly.
        """
        idx = self._load_index(channel_id, run_id) or SegmentIndex()

        if len(data) == 0:
            # Empty ref: coverage-only entry, no segment body. Doesn't
            # bump ``next_seg_id`` because there's no segment to key.
            idx.segments.append(SegmentRef(seg_id=None, start_time=start_time, end_time=end_time))
            self._write_index(channel_id, run_id, idx)
        else:
            if not data.index.is_monotonic_increasing:
                data = data.sort_index()
            size_bytes = int(data.memory_usage(deep=True).sum())
            seg_id = idx.next_seg_id

            # Segment first.
            seg_key = self._segment_key(channel_id, run_id, seg_id)
            self._store.put(seg_key, data, size_bytes=size_bytes)
            if seg_key not in self._store:
                # The store rejected the segment (oversize, disabled,
                # etc.). Skipping the index update keeps us from
                # leaving a dangling reference to a never-written
                # segment.
                return

            idx.segments.append(SegmentRef(seg_id=seg_id, start_time=start_time, end_time=end_time))
            idx.next_seg_id += 1
            self._write_index(channel_id, run_id, idx)

        # Bound read fan-out: collapse all segments into one once the
        # bucket would carry more than the cap.
        if len(idx.segments) > self.MAX_SEGMENTS_PER_BUCKET:
            self._compact_bucket(channel_id, run_id)

    def invalidate(self, channel_id: str, run_id: str | None = None) -> None:
        """Drop every segment in a bucket plus the index. Safe when absent.

        Only touches the one ``(channel_id, run_id)`` bucket — segments
        under other runs survive. Empty refs carry no segment key, so
        only data refs (``seg_id is not None``) hit the store here.
        """
        idx = self._load_index(channel_id, run_id)
        if idx is None:
            return
        for ref in idx.segments:
            if ref.seg_id is None:
                continue
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
        return raw

    def _load_segment(
        self, channel_id: str, run_id: str | None, seg_id: int
    ) -> pd.DataFrame | None:
        raw = self._store.get(self._segment_key(channel_id, run_id, seg_id))
        if not isinstance(raw, pd.DataFrame):
            return None
        return raw

    def _write_index(self, channel_id: str, run_id: str | None, idx: SegmentIndex) -> None:
        # Pydantic indexes serialize small (refs are scalars), but we
        # still pass a size hint that scales with segment count so the
        # store's per-entry-size eviction sees an honest weight.
        self._store.put(
            self._index_key(channel_id, run_id),
            idx,
            size_bytes=max(1024, 128 * len(idx.segments)),
        )

    def _compact_bucket(self, channel_id: str, run_id: str | None) -> None:
        """Fold every ref in a bucket into a single merged ref.

        Called by :meth:`put_segment` once the per-bucket ref count
        crosses :attr:`MAX_SEGMENTS_PER_BUCKET`. The motivation is read
        fan-out: without compaction ``get_range`` on a long
        incrementally-pulled run would walk one ref per prior fetch,
        so the per-read cost climbs linearly in the number of writes.
        After compaction the bucket holds at most one data ref (or one
        empty ref, if every prior fetch returned no data) until the
        next put.

        Empty refs (``seg_id is None``) carry coverage but no body;
        their claimed ``[start, end]`` is folded into the merged ref's
        claim along with the data refs. If every loadable body went
        away (all-empty or every data ref LRU-evicted), the bucket
        compacts to a single empty ref covering the union of all
        prior claims so the "no data here" coverage is preserved.

        Write ordering (crash safety):

        1. Write the merged segment under a fresh ``seg_id`` so a
           crash here leaves the old segments + old index intact
           (the orphan merged seg LRU-evicts).
        2. Rewrite the index to point at *only* the merged seg. A
           crash here leaves the old segs as orphans (LRU-evicts).
        3. Delete the old segment keys. Any crash here is harmless
           since the index already points elsewhere.

        Cost note: under a flat cap M, total work across N fetches is
        O(N^2 / M) — quadratic in fetches with the constant reduced by
        the cap. That's the documented first-cut; if compaction shows
        up as a hot spot in stats, the next step is LSM-style
        geometric levels rather than tuning M.
        """
        idx = self._load_index(channel_id, run_id)
        if idx is None or len(idx.segments) <= self.MAX_SEGMENTS_PER_BUCKET:
            # Bucket already cleaned up (concurrent invalidate, LRU,
            # or a parallel compactor) — nothing to do.
            return

        refs = sorted(idx.segments, key=lambda r: r.start_time)

        # Empty refs have no body to load — their contribution is
        # purely coverage, which we fold into the merged claim below.
        # LRU may also have dropped a data ref's body; those refs
        # degrade to gaps (we don't merge missing data).
        frames: list[pd.DataFrame] = []
        for ref in refs:
            if ref.seg_id is None:
                continue
            frame = self._load_segment(channel_id, run_id, ref.seg_id)
            if frame is not None:
                frames.append(frame)

        old_seg_ids: list[int] = [r.seg_id for r in refs if r.seg_id is not None]

        # Claimed coverage of the merged ref spans *every* ref's claim
        # (data + empty + evicted) — the index already represents "we
        # asked the wire about this range" and compaction must preserve
        # that, even if the underlying data went away.
        merged_start = min(r.start_time for r in refs)
        merged_end = max(r.end_time for r in refs)

        if not frames:
            # No usable data bodies left — bucket collapses to a single
            # empty ref covering the union of all prior claims. Future
            # reads in that range are cache hits returning no rows.
            self._replace_index_and_drop_segments(
                channel_id,
                run_id,
                new_segments=[
                    SegmentRef(seg_id=None, start_time=merged_start, end_time=merged_end)
                ],
                next_seg_id=idx.next_seg_id,
                drop_seg_ids=old_seg_ids,
            )
            logger.debug(
                "compacted bucket %r:%r from %d refs to 1 empty ref",
                run_id or "",
                channel_id,
                len(refs),
            )
            return

        # Mirror :meth:`get_range`'s dedup: later segments win on
        # boundary-overlapping timestamps (refs are already sorted by
        # start_time above).
        merged_data = frames[0] if len(frames) == 1 else pd.concat(frames).groupby(level=0).last()
        merged_size = int(merged_data.memory_usage(deep=True).sum())
        new_seg_id = idx.next_seg_id

        # Step 1: write the merged segment first.
        new_seg_key = self._segment_key(channel_id, run_id, new_seg_id)
        self._store.put(new_seg_key, merged_data, size_bytes=merged_size)
        if new_seg_key not in self._store:
            # Store rejected (typically oversize). Leave the bucket
            # alone so the per-fetch segments remain readable; the
            # next ``put_segment`` will retry the cap check.
            logger.debug(
                "compaction skipped for bucket %r:%r (merged size %d exceeded store cap)",
                run_id or "",
                channel_id,
                merged_size,
            )
            return

        # Steps 2 + 3: rewrite the index to point at only the merged
        # seg, then drop superseded segment keys.
        self._replace_index_and_drop_segments(
            channel_id,
            run_id,
            new_segments=[
                SegmentRef(seg_id=new_seg_id, start_time=merged_start, end_time=merged_end)
            ],
            next_seg_id=new_seg_id + 1,
            drop_seg_ids=old_seg_ids,
        )
        logger.debug(
            "compacted bucket %r:%r from %d refs to 1 (merged size %d)",
            run_id or "",
            channel_id,
            len(refs),
            merged_size,
        )

    def _replace_index_and_drop_segments(
        self,
        channel_id: str,
        run_id: str | None,
        *,
        new_segments: list[SegmentRef],
        next_seg_id: int,
        drop_seg_ids: list[int],
    ) -> None:
        """Rewrite the index, then invalidate superseded segment keys.

        Order matters for crash safety: the new index goes down first
        so that an interrupt before the deletes leaves the old segs as
        unreachable orphans (which LRU-evict) rather than leaving the
        index pointing at deleted keys.
        """
        self._write_index(
            channel_id,
            run_id,
            SegmentIndex(next_seg_id=next_seg_id, segments=new_segments),
        )
        for seg_id in drop_seg_ids:
            self._store.invalidate(self._segment_key(channel_id, run_id, seg_id))

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
        fetched_ranges_per_channel: dict[str, list[TimeRange]],
        start_time: datetime,
        end_time: datetime,
        run_id: str | None = None,
    ):
        """Write each channel's fresh data or empty-ref as a new segment.

        Per-fetch disk write is O(this segment) — no merging with prior
        segments and no re-pickle of accumulated data, so n sequential
        incremental pulls cost O(n) total disk write instead of O(n²).

        Bitfield grouping: ``try_deserialize_channel_data`` returns one
        dotted-name DataFrame per bitfield element, all mapping to the
        same parent ``channel_id`` via :attr:`name_id_map`. We group
        them by parent id and concat into one wide frame so each fetch
        produces exactly one segment per channel, regardless of how
        many elements that channel exposes.

        Empty results are recorded as empty refs (no segment body) —
        one ref per fetched range — so a repeat of a known-empty range
        is a cache hit returning no rows instead of another wire call.
        Only the **unscoped** path records empty refs: run-scoped
        absence isn't assertable and would permanently mask data an
        ongoing run ingests after the empty query (subsequent same-
        range queries would hit the cache and never refetch).

        Args:
            channel_data: Merged per-name frames coming out of
                :meth:`_merge_pages`. A channel absent from this dict
                returned zero rows across every page; a present channel
                with a zero-row frame had every page return zero rows
                (bitfield elements that all came back empty).
            fetched_ranges_per_channel: Per-channel-id list of the
                ``[start, end]`` windows we actually asked the wire
                about this call. Drives empty-ref recording: any cid
                in this dict whose data dropped out for the empty case
                gets one empty ref per range. Pure cache hits don't
                reach this method (the caller skips when nothing was
                fetched).
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

        ids_with_data: set[str] = set()
        for channel_id, frames in by_channel_id.items():
            if len(frames) == 1:
                combined = frames[0]
            else:
                # Bitfield: per-element single-column frames → one wide
                # frame. ``groupby.last`` dedups any boundary overlaps.
                combined = pd.concat(frames).groupby(level=0).last()

            if len(combined) == 0:
                # Bitfield with every element empty, or other edge
                # cases — handled by the empty-ref loop below.
                continue

            ids_with_data.add(channel_id)

            # Segment coverage range. For run-scoped queries, claim
            # only what the data actually spans (we can't assert
            # absence outside the data — the run might not have
            # started yet). For unscoped queries, claim the full
            # requested range so a follow-up of the same range hits.
            seg_end = end_time
            if run_id:
                # ``combined.index`` is a ``DatetimeIndex`` (built from
                # the wire's nanosecond timestamps), so ``index[0]`` is
                # always ``pd.Timestamp`` at runtime; pandas-stubs types
                # it as the wider ``Scalar`` union.
                seg_start = cast("pd.Timestamp", combined.index[0]).to_pydatetime()
            else:
                seg_start = start_time

            self.channel_cache.put_segment(
                channel_id=channel_id,
                run_id=run_id,
                data=combined,
                start_time=seg_start,
                end_time=seg_end,
            )

        # Empty refs for channels that were queried this call but came
        # back with zero rows. One ref per fetched range so the
        # coverage claim matches what we actually asked the wire about
        # — overclaiming would let a future query for an adjacent
        # range hit the cache and miss data that actually exists.
        #
        # Skipped for run-scoped queries: absence at query time
        # doesn't imply future absence (the run may still be
        # ingesting), and an empty ref would permanently mask data
        # that arrives later on the same window.
        if run_id:
            return
        empty_df = pd.DataFrame()
        for cid, ranges in fetched_ranges_per_channel.items():
            if cid in ids_with_data:
                continue
            for fetched_start, fetched_end in ranges:
                self.channel_cache.put_segment(
                    channel_id=cid,
                    run_id=run_id,
                    data=empty_df,
                    start_time=fetched_start,
                    end_time=fetched_end,
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
        # and go one fetch at a time. ``fetched_ranges_per_channel``
        # records the exact ``[s, e]`` windows we asked the wire about,
        # per channel — used downstream to record empty refs (a "no
        # data here" coverage claim) so a repeat of a known-empty
        # query is a cache hit instead of another wire call.
        fully_uncached: list[str] = []
        partial_gaps: list[tuple[str, list[TimeRange]]] = []
        fetched_ranges_per_channel: dict[str, list[TimeRange]] = {}

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
            fetched_ranges_per_channel.setdefault(cid, []).extend(gaps)
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

        # Pure cache hits never reach ``_update_cache`` because nothing
        # was fetched (``fetched_ranges_per_channel`` is empty). When
        # we did fetch, ``_update_cache`` writes both data segments and
        # empty refs — the latter covers the "asked the wire, got
        # nothing" case so a repeat doesn't refetch.
        if not ignore_cache and fetched_ranges_per_channel:
            self._update_cache(
                channel_data=ret_data,
                fetched_ranges_per_channel=fetched_ranges_per_channel,
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

        ``initial`` carries the cached slices ``get_channel_data``
        stitched inline via :meth:`ChannelDataCache.get_range` before
        dispatching wire fetches for the gaps. Cached entries are
        folded in as the first frame for their channel so they
        participate in the same final concat; ``groupby(level=0).last()``
        preserves the previous behavior of letting a later-positioned
        (fresher) value win on duplicate timestamps.
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
