"""Tests for :mod:`sift_client._internal.low_level_wrappers.data`.

Five classes, narrowest scope first:

* :class:`TestChannelDataCache` — the typed adapter over the shared
  :class:`DiskCache`. Covers key namespacing, eviction-tolerant
  stitching, gap computation, and the prefix-scoped ``clear``.
* :class:`TestMergePages` — ``DataLowLevelClient._merge_pages``, the
  per-channel concat helper.
* :class:`TestDataLowLevelClient` — constructor wiring and per-instance
  isolation.
* :class:`TestGetChannelData` — end-to-end on the public
  ``get_channel_data`` API against a mocked ``_get_data_impl``.
* :class:`TestBitFieldChannels` — the channel-id-vs-channel-name seam
  where bitfield elements share one parent id but surface as multiple
  dotted-name keys in the result.

Storage-layer behaviour (oversize guards, marker-checked clear,
cross-session reload) lives in ``_tests/_internal/test_disk_cache.py``;
this file stays focused on the channel-data path.

The OOM regression that motivated this code happened because the cache
was a class attribute that grew without bound. ``test_per_instance_isolation``
is the canary that catches anyone re-introducing that pattern, even though
ownership has since moved to the client.

The cache stores data as per-fetch *segments* under one index per
``(channel_id, run_id)`` bucket. ``put_segment`` writes a new segment
+ updates the index; ``get_range`` walks the index, slices each
overlapping segment to the query window, and reports any uncovered
sub-ranges as gaps. The segment model is what eliminates the prior
write amplification on incremental pulls (where every fetch re-pickled
the entire accumulated frame).
"""

from __future__ import annotations

from contextlib import contextmanager
from datetime import datetime, timedelta, timezone
from typing import Any, Iterator, cast
from unittest.mock import MagicMock, patch

import pandas as pd
import pytest

from sift_client._internal.disk_cache import DiskCache
from sift_client._internal.low_level_wrappers.data import (
    ChannelDataCache,
    DataLowLevelClient,
)
from sift_client.sift_types.channel import (
    Channel,
    ChannelBitFieldElement,
    ChannelDataType,
)

_NOW = datetime(2025, 1, 1, tzinfo=timezone.utc)
_WINDOW_END = _NOW + timedelta(days=1)


# ---------- shared helpers -----------


def _frame(
    cid: str = "value",
    *,
    rows: int = 5,
    start: datetime = _NOW,
    offset: int = 0,
    freq: str = "ms",
    value_dtype: str = "float64",
) -> pd.DataFrame:
    """DataFrame indexed by a tz-aware DatetimeIndex with ``rows`` rows."""
    index = pd.date_range(start, periods=rows, freq=freq, tz=timezone.utc)
    return pd.DataFrame(
        {cid: [(offset + i) * 1.0 for i in range(rows)]},
        index=index,
    ).astype({cid: value_dtype})


def _channel(cid: str) -> Channel:
    """Minimal ``Channel`` with required fields populated."""
    return Channel(
        id_=cid,
        name=cid,
        data_type=ChannelDataType.DOUBLE,
        description="",
        unit="",
        asset_id="a1",
        is_archived=False,
        created_date=_NOW,
        modified_date=_NOW,
        created_by_user_id="u1",
        modified_by_user_id="u1",
    )


def _bitfield_channel(*, cid: str, name: str, elements: list[str]) -> Channel:
    """Bitfield ``Channel`` with the named elements (8-bit, indexed in order).

    ``cid`` and ``name`` are kept distinct so test assertions can distinguish
    "is the cache keyed by id?" from "is the result dict keyed by name?".
    """
    return Channel(
        id_=cid,
        name=name,
        data_type=ChannelDataType.BIT_FIELD,
        description="",
        unit="",
        asset_id="a1",
        is_archived=False,
        bit_field_elements=[
            ChannelBitFieldElement(name=el, index=i, bit_count=8) for i, el in enumerate(elements)
        ],
        created_date=_NOW,
        modified_date=_NOW,
        created_by_user_id="u1",
        modified_by_user_id="u1",
    )


def _client_with_cache(tmp_path, subdir: str = "cache") -> DataLowLevelClient:
    """Build a ``DataLowLevelClient`` whose adapter points at ``tmp_path``.

    Tests that exercise cache behaviour (hits/misses) need an actual
    disk-backed adapter, so the store has to be opened explicitly. A
    plain ``DataLowLevelClient(MagicMock())`` defaults to a no-op store
    and would silently turn every cache test into a wire-path test.
    """
    store = DiskCache(disk_path=tmp_path / subdir)
    return DataLowLevelClient(MagicMock(), channel_cache=ChannelDataCache(store))


def _patch_deserializer(sentinel_to_frames: dict[str, dict[str, pd.DataFrame]]) -> Any:
    """Patch ``try_deserialize_channel_data`` to translate string sentinels.

    Lets tests pass strings in lieu of protos. Returned object is a context
    manager; callers use ``with _patch_deserializer(...):``.
    """
    return patch.object(
        DataLowLevelClient,
        "try_deserialize_channel_data",
        staticmethod(lambda s: sentinel_to_frames[s]),
    )


@contextmanager
def _fake_grpc(
    client: DataLowLevelClient,
    channel_to_pages: dict[str, list[pd.DataFrame | dict[str, pd.DataFrame]]],
) -> Iterator[list[dict[str, Any]]]:
    """Mock the gRPC boundary so each "page" is a sentinel string.

    ``_get_data_impl`` is replaced with a coroutine that pops one page off
    ``channel_to_pages[cid]`` per call per channel, until exhausted.
    ``try_deserialize_channel_data`` is patched to map the sentinel back
    to the corresponding ``{channel: DataFrame}`` dict.

    A page entry can be either:

    * ``pd.DataFrame`` — wrapped as ``{cid: df}`` (the single-channel
      shape ``try_deserialize_channel_data`` returns for non-bitfield
      channels).
    * ``dict[str, pd.DataFrame]`` — used as-is (the multi-name shape
      ``BitFieldValues`` produces, with keys like ``"<channel>.<element>"``
      per bitfield element).

    Yields a ``call_log`` list so tests can assert which channels actually
    hit the wire. The patch is torn down and ``_get_data_impl`` restored
    on exit.
    """
    sentinel_to_frames: dict[str, dict[str, pd.DataFrame]] = {}
    next_page_index: dict[str, int] = dict.fromkeys(channel_to_pages, 0)
    call_log: list[dict[str, Any]] = []

    async def fake_impl(
        *,
        channel_ids: list[str],
        page_size: int | None = None,
        page_token: str | None = None,
        order_by: str | None = None,
        **kwargs: Any,
    ) -> tuple[list[str], str]:
        call_log.append({"channel_ids": list(channel_ids), **kwargs})
        data: list[str] = []
        more_remaining = False
        for cid in channel_ids:
            i = next_page_index[cid]
            if i >= len(channel_to_pages[cid]):
                continue  # this channel is exhausted; just emit nothing
            sentinel = f"{cid}|{i}"
            page = channel_to_pages[cid][i]
            sentinel_to_frames[sentinel] = dict(page) if isinstance(page, dict) else {cid: page}
            data.append(sentinel)
            next_page_index[cid] += 1
            if next_page_index[cid] < len(channel_to_pages[cid]):
                more_remaining = True
        # ``_handle_pagination`` loops until it sees ``page_token == ""``.
        return data, ("next" if more_remaining else "")

    original_impl = client._get_data_impl
    client._get_data_impl = fake_impl  # type: ignore[method-assign]
    try:
        with _patch_deserializer(sentinel_to_frames):
            yield call_log
    finally:
        client._get_data_impl = original_impl  # type: ignore[method-assign]


# ---------- tests -----------


def _put(
    adapter: ChannelDataCache,
    channel_id: str,
    *,
    data: pd.DataFrame | None = None,
    rows: int = 5,
    start: datetime = _NOW,
    offset: int = 0,
    freq: str = "ms",
    seg_start: datetime | None = None,
    seg_end: datetime | None = None,
    run_id: str | None = None,
) -> pd.DataFrame:
    """Convenience: build a frame, write it as one segment, return it.

    ``seg_start`` / ``seg_end`` default to the data's actual range so
    tests get tightly-bounded segments unless they specifically want to
    claim extra coverage.
    """
    if data is None:
        data = _frame(channel_id, rows=rows, start=start, offset=offset, freq=freq)
    if seg_start is None:
        seg_start = cast("pd.Timestamp", data.index[0]).to_pydatetime()
    if seg_end is None:
        seg_end = cast("pd.Timestamp", data.index[-1]).to_pydatetime()
    adapter.put_segment(
        channel_id=channel_id,
        run_id=run_id,
        data=data,
        start_time=seg_start,
        end_time=seg_end,
    )
    return data


class TestChannelDataCache:
    """The typed adapter over the shared :class:`DiskCache`.

    Five invariants get pinned across the per-segment shape:

    1. Every operation routes through the namespaced key
       (``channel:v2:<run_id>:<id>:{idx,seg:N}``), so two adapters sharing
       one store don't collide on bare resource ids.
    2. Run id is part of the cache dimension: the same ``channel_id``
       under two different runs is two cache buckets, not one.
    3. :meth:`ChannelDataCache.get_range` stitches multiple segments
       and reports uncovered sub-ranges as gaps. Missing (evicted)
       segments degrade to gaps, never to errors.
    4. :meth:`ChannelDataCache.invalidate` drops every segment in a
       bucket and the index, leaving other buckets untouched.
    5. :meth:`ChannelDataCache.clear` wipes only the adapter's namespace
       — entries belonging to other adapters survive.

    Store-level behaviour (oversized guards, cross-session reload,
    marker-checked clear_disk) is exercised in ``test_disk_cache.py``.
    """

    def test_miss_returns_none_and_full_gap(self, tmp_path):
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "miss"))
        try:
            assert not adapter.has_any("c1")
            data, gaps = adapter.get_range("c1", None, _NOW, _WINDOW_END)
            assert data is None
            assert gaps == [(_NOW, _WINDOW_END)]
        finally:
            adapter.store.close()

    def test_round_trip_single_segment(self, tmp_path):
        """Put one segment, get_range back covers the whole frame and reports no gap."""
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "rt"))
        try:
            df = _put(adapter, "c1", rows=8)
            assert adapter.has_any("c1")
            got, gaps = adapter.get_range("c1", None, df.index[0], df.index[-1])
            assert got is not None
            pd.testing.assert_frame_equal(got, df)
            assert gaps == []
        finally:
            adapter.store.close()

    def test_writes_use_namespaced_index_and_segment_keys(self, tmp_path):
        """The raw store sees ``channel:v2:<run>:<id>:idx`` + ``...:seg:0``.

        Pins the per-segment key shape: one index plus one segment key
        per fetch. Without the prefix, a second adapter that happens to
        share an id with the channel adapter would clobber the rows.
        The ``v2`` is the schema version baked into the prefix so a
        bump silently retires the entire old keyspace.
        """
        store = DiskCache(disk_path=tmp_path / "ns")
        adapter = ChannelDataCache(store)
        try:
            _put(adapter, "c1", rows=4)
            assert "channel:v2::c1:idx" in store
            assert "channel:v2::c1:seg:0" in store
            assert "c1" not in store
            assert "channel:c1" not in store  # never the bare-id shape
            assert "channel::c1:idx" not in store  # never the unversioned shape
        finally:
            store.close()

    def test_run_id_is_part_of_the_key(self, tmp_path):
        """Same channel under two runs is two cache buckets, not one.

        Regression guard for the run-scoping bug: a bare ``channel:<id>``
        key conflated runs and served run-A's data to a query for run-B.
        With segments, that turns into "the two buckets share an index"
        — the test below pins them as fully independent.
        """
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "runs"))
        try:
            df_a = _put(adapter, "c1", rows=4, run_id="run-A")
            df_b = _put(adapter, "c1", rows=8, run_id="run-B")

            assert adapter.has_any("c1", "run-A")
            assert adapter.has_any("c1", "run-B")
            assert not adapter.has_any("c1")  # unscoped bucket stays empty
            assert not adapter.has_any("c1", "run-C")  # unknown run still misses

            got_a, _ = adapter.get_range("c1", "run-A", df_a.index[0], df_a.index[-1])
            got_b, _ = adapter.get_range("c1", "run-B", df_b.index[0], df_b.index[-1])
            assert got_a is not None
            assert len(got_a) == 4
            assert got_b is not None
            assert len(got_b) == 8
        finally:
            adapter.store.close()

    def test_unscoped_and_scoped_buckets_are_independent(self, tmp_path):
        """An unscoped put (``run_id=None``) doesn't satisfy a run-scoped get."""
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "indep"))
        try:
            _put(adapter, "c1", rows=4)  # no run
            assert adapter.has_any("c1")
            assert not adapter.has_any("c1", "run-A")
            data, gaps = adapter.get_range("c1", "run-A", _NOW, _WINDOW_END)
            assert data is None
            assert gaps == [(_NOW, _WINDOW_END)]
        finally:
            adapter.store.close()

    def test_get_range_isinstance_filters_foreign_segments(self, tmp_path):
        """A segment row with the wrong shape reads as a miss → gap.

        Models a corrupt entry or a key collision from another writer.
        ``_load_segment`` isinstance-checks before handing back; the
        evicted-segment-as-gap fallback covers this too.
        """
        store = DiskCache(disk_path=tmp_path / "foreign")
        adapter = ChannelDataCache(store)
        try:
            _put(adapter, "c1", rows=4)
            # Overwrite the segment's payload with foreign data; the
            # index still claims it exists, so the read should treat
            # the segment range as a gap.
            store.put("channel:v2::c1:seg:0", {"not": "an entry"}, size_bytes=64)
            data, gaps = adapter.get_range("c1", None, _NOW, _WINDOW_END)
            assert data is None
            # The whole query range is uncovered (one merged gap).
            assert gaps == [(_NOW, _WINDOW_END)]
        finally:
            store.close()

    def test_invalidate_is_run_scoped(self, tmp_path):
        """``invalidate`` only drops the named ``(channel, run)`` bucket."""
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "inval"))
        try:
            adapter.invalidate("never_added")  # safe before any puts
            _put(adapter, "c1", rows=4, run_id="run-A")
            _put(adapter, "c1", rows=8, run_id="run-B")
            adapter.invalidate("c1", "run-A")
            assert not adapter.has_any("c1", "run-A")
            assert adapter.has_any("c1", "run-B")  # run-B survives
        finally:
            adapter.store.close()

    def test_clear_is_prefix_scoped(self, tmp_path):
        """``clear`` drops channel rows across all runs, leaves other adapters alone."""
        store = DiskCache(disk_path=tmp_path / "scoped")
        adapter = ChannelDataCache(store)
        try:
            _put(adapter, "c1", rows=4)  # unscoped
            _put(adapter, "c2", rows=4, run_id="run-A")
            store.put("other:1", "foreign-value", size_bytes=64)
            adapter.clear()
            assert not adapter.has_any("c1")
            assert not adapter.has_any("c2", "run-A")
            assert "other:1" in store
        finally:
            store.close()

    def test_size_bytes_propagates_to_store(self, tmp_path):
        """Oversized segments are skipped by the store; index/segment-write order matters.

        The segment is written first, then the index. When the store
        refuses the segment (oversize), the index stays empty and
        ``has_any`` reports false. Without this ordering you'd get an
        index entry pointing at a missing segment.
        """
        big = _frame("c1", rows=10_000)
        size_bytes = int(big.memory_usage(deep=True).sum())
        store = DiskCache(disk_path=tmp_path / "size", disk_max_bytes=size_bytes // 2)
        adapter = ChannelDataCache(store)
        try:
            adapter.put_segment(
                "c1", None, big, big.index[0].to_pydatetime(), big.index[-1].to_pydatetime()
            )
            assert not adapter.has_any("c1")
        finally:
            store.close()

    def test_no_op_store_keeps_adapter_silent(self):
        """An adapter on a disabled store behaves like a cold cache."""
        adapter = ChannelDataCache(DiskCache())
        assert not adapter.store.disk_enabled
        _put(adapter, "c1", rows=4)
        assert not adapter.has_any("c1")
        data, gaps = adapter.get_range("c1", None, _NOW, _WINDOW_END)
        assert data is None
        assert gaps == [(_NOW, _WINDOW_END)]
        adapter.invalidate("c1")
        adapter.clear()

    # --- stitching + gap behaviour (new with the per-segment shape) ---

    def test_get_range_stitches_multiple_segments(self, tmp_path):
        """Two segments whose claimed ranges together cover the query → one stitched frame.

        Segments claim ``[seg_start, seg_end]`` boundaries that abut at
        the query midpoint so gap math reports zero gaps. The stitch
        path concats and dedups via ``groupby.last``.
        """
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "stitch"))
        try:
            df1 = _frame("c1", rows=5, start=_NOW, freq="ms", offset=0)
            df1_end = df1.index[-1].to_pydatetime()
            df2 = _frame(
                "c1",
                rows=5,
                start=df1_end + timedelta(milliseconds=1),
                freq="ms",
                offset=100,
            )
            query_start = df1.index[0].to_pydatetime()
            query_end = df2.index[-1].to_pydatetime()
            # Claim seg2 starts right where seg1 ends so the abutting
            # ranges leave no gap. Real callers do this via
            # ``_update_cache`` claiming the requested window.
            adapter.put_segment("c1", None, df1, query_start, df1_end)
            adapter.put_segment("c1", None, df2, df1_end, query_end)
            got, gaps = adapter.get_range("c1", None, query_start, query_end)
            assert got is not None
            assert len(got) == 10
            expected = pd.concat([df1, df2]).groupby(level=0).last()
            pd.testing.assert_frame_equal(got.sort_index(), expected.sort_index(), check_freq=False)
            assert gaps == []  # abutting claimed ranges → no gap
        finally:
            adapter.store.close()

    def test_get_range_reports_internal_gap_between_segments(self, tmp_path):
        """Query window wider than the two segments → one gap between them."""
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "midgap"))
        try:
            seg_a_start = _NOW
            seg_a_end = _NOW + timedelta(seconds=5)
            seg_b_start = _NOW + timedelta(seconds=10)
            seg_b_end = _NOW + timedelta(seconds=15)
            adapter.put_segment(
                "c1",
                None,
                _frame("c1", rows=2, start=seg_a_start, freq="s"),
                seg_a_start,
                seg_a_end,
            )
            adapter.put_segment(
                "c1",
                None,
                _frame("c1", rows=2, start=seg_b_start, freq="s"),
                seg_b_start,
                seg_b_end,
            )
            _, gaps = adapter.get_range("c1", None, seg_a_start, seg_b_end)
            assert gaps == [(seg_a_end, seg_b_start)]
        finally:
            adapter.store.close()

    def test_get_range_reports_outer_gaps(self, tmp_path):
        """Query wider than the cached segment on both sides → two gaps."""
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "outer"))
        try:
            seg_start = _NOW + timedelta(seconds=5)
            seg_end = _NOW + timedelta(seconds=10)
            adapter.put_segment(
                "c1",
                None,
                _frame("c1", rows=2, start=seg_start, freq="s"),
                seg_start,
                seg_end,
            )
            query_end = _NOW + timedelta(seconds=15)
            _, gaps = adapter.get_range("c1", None, _NOW, query_end)
            assert gaps == [(_NOW, seg_start), (seg_end, query_end)]
        finally:
            adapter.store.close()

    def test_put_segment_normalizes_non_monotonic_index_before_write(self, tmp_path):
        """A shuffled-index frame stored and re-read: no ``KeyError``, sorted result.

        Pandas raises ``KeyError`` on value-based partial slicing of a
        non-monotonic ``DatetimeIndex``, so :meth:`get_range`'s
        ``frame[start:end]`` label slice crashes if a segment ever lands
        on disk unsorted. The SDK can't request descending order today,
        but the wire could theoretically return interleaved pages;
        :meth:`put_segment` sorts on store so the read path is safe
        without a per-slice ``try``/``except``.
        """
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "unsorted"))
        try:
            sorted_frame = _frame("c1", rows=5, freq="s")
            # Shuffle to a definitely non-monotonic order.
            shuffled = sorted_frame.iloc[[2, 0, 4, 1, 3]]
            assert not shuffled.index.is_monotonic_increasing

            seg_start = sorted_frame.index[0].to_pydatetime()
            seg_end = sorted_frame.index[-1].to_pydatetime()
            adapter.put_segment("c1", None, shuffled, seg_start, seg_end)

            data, gaps = adapter.get_range("c1", None, seg_start, seg_end)
            assert data is not None
            assert data.index.is_monotonic_increasing
            pd.testing.assert_frame_equal(data, sorted_frame, check_freq=False)
            assert gaps == []
        finally:
            adapter.store.close()

    def test_evicted_segment_degrades_to_gap(self, tmp_path):
        """If the store loses a segment (LRU), the reader treats its range as a gap.

        Pins the eviction-tolerance contract. The index can outlive
        its segments under memory pressure; reads must not error and
        must surface the uncovered range so the caller refetches.
        """
        store = DiskCache(disk_path=tmp_path / "evict")
        adapter = ChannelDataCache(store)
        try:
            df = _put(adapter, "c1", rows=5)
            store.invalidate("channel:v2::c1:seg:0")  # simulate eviction
            data, gaps = adapter.get_range("c1", None, df.index[0], df.index[-1])
            assert data is None
            assert gaps == [(df.index[0], df.index[-1])]
        finally:
            store.close()

    def test_segment_count_grows_with_each_put(self, tmp_path):
        """``put_segment`` writes a new key per call — no merging on the write path.

        Pins the "no rewrite of accumulated bytes per fetch" contract.
        ``next_seg_id`` advances; raw store key count grows by one
        index update + one new segment per put.
        """
        store = DiskCache(disk_path=tmp_path / "grow")
        adapter = ChannelDataCache(store)
        try:
            for i in range(3):
                _put(
                    adapter,
                    "c1",
                    rows=2,
                    start=_NOW + timedelta(seconds=i * 10),
                    freq="s",
                )
            # Expect: 1 index + 3 segments.
            channel_keys = sorted(k for k in store if k.startswith("channel:"))
            assert channel_keys == [
                "channel:v2::c1:idx",
                "channel:v2::c1:seg:0",
                "channel:v2::c1:seg:1",
                "channel:v2::c1:seg:2",
            ]
        finally:
            store.close()

    def test_empty_put_records_empty_ref_and_skips_refetch(self, tmp_path):
        """``put_segment`` with empty data records a ``seg_id=None`` ref.

        Empty ref behavior:

        * ``has_any`` returns True (the bucket has a coverage claim,
          just no body).
        * ``get_range`` over the claimed range returns ``(None, [])``
          — no data, no gaps, so the caller doesn't refetch.
        * No segment key lands in the store (only the index gets a
          row); the empty ref lives entirely on the index.
        """
        store = DiskCache(disk_path=tmp_path / "empty")
        adapter = ChannelDataCache(store)
        try:
            empty = pd.DataFrame(
                {"c1": []},
                index=pd.DatetimeIndex([], tz=timezone.utc),
            )
            adapter.put_segment("c1", None, empty, _NOW, _WINDOW_END)

            assert adapter.has_any("c1")
            data, gaps = adapter.get_range("c1", None, _NOW, _WINDOW_END)
            assert data is None
            assert gaps == []

            channel_keys = sorted(k for k in store if k.startswith("channel:"))
            assert channel_keys == ["channel:v2::c1:idx"]
        finally:
            store.close()

    def test_empty_ref_only_covers_its_claimed_range(self, tmp_path):
        """Reads outside the empty ref's claimed range still report gaps.

        Pins the "claim only what you queried" contract: an empty ref
        covering ``[T0, T5]`` doesn't suppress a refetch for
        ``[T5, T10]``.
        """
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "narrow"))
        try:
            claim_end = _NOW + timedelta(seconds=5)
            adapter.put_segment("c1", None, pd.DataFrame(), _NOW, claim_end)

            _, gaps = adapter.get_range("c1", None, _NOW, _WINDOW_END)
            # Empty ref covers [_NOW, claim_end]; remainder is a gap.
            assert gaps == [(claim_end, _WINDOW_END)]
        finally:
            adapter.store.close()

    def test_empty_ref_alongside_data_segment(self, tmp_path):
        """Stitching data + empty ref returns the data and reports no gap.

        Mirrors a real partial-hit scenario: cache already has data
        for ``[T0, T5]``, the caller queries ``[T0, T10]``, the gap
        ``[T5, T10]`` fetch returns no rows and lands as an empty
        ref. Subsequent ``get_range(T0, T10)`` returns the cached
        data with ``gaps == []``.
        """
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "mix"))
        try:
            data = _frame("c1", rows=5, freq="s")
            mid = data.index[-1].to_pydatetime()
            tail = mid + timedelta(seconds=5)

            adapter.put_segment("c1", None, data, data.index[0].to_pydatetime(), mid)
            adapter.put_segment("c1", None, pd.DataFrame(), mid, tail)

            got, gaps = adapter.get_range("c1", None, data.index[0].to_pydatetime(), tail)
            assert got is not None
            pd.testing.assert_frame_equal(got, data)
            assert gaps == []
        finally:
            adapter.store.close()

    # --- gap-math unit tests ---

    @pytest.mark.parametrize(
        ("covered", "expected"),
        [
            # Fully uncovered.
            ([], [(_NOW, _NOW + timedelta(seconds=10))]),
            # Fully covered.
            ([(_NOW, _NOW + timedelta(seconds=10))], []),
            # Left-only gap.
            (
                [(_NOW + timedelta(seconds=5), _NOW + timedelta(seconds=10))],
                [(_NOW, _NOW + timedelta(seconds=5))],
            ),
            # Right-only gap.
            (
                [(_NOW, _NOW + timedelta(seconds=5))],
                [(_NOW + timedelta(seconds=5), _NOW + timedelta(seconds=10))],
            ),
            # Two segments, one internal gap.
            (
                [
                    (_NOW, _NOW + timedelta(seconds=2)),
                    (_NOW + timedelta(seconds=7), _NOW + timedelta(seconds=10)),
                ],
                [(_NOW + timedelta(seconds=2), _NOW + timedelta(seconds=7))],
            ),
            # Overlapping covered ranges merge before gap math.
            (
                [
                    (_NOW, _NOW + timedelta(seconds=6)),
                    (_NOW + timedelta(seconds=4), _NOW + timedelta(seconds=8)),
                ],
                [(_NOW + timedelta(seconds=8), _NOW + timedelta(seconds=10))],
            ),
        ],
        ids=[
            "fully_uncovered",
            "fully_covered",
            "left_gap",
            "right_gap",
            "internal_gap",
            "overlapping_covered_merges",
        ],
    )
    def test_compute_gaps(self, covered, expected):
        query_start = _NOW
        query_end = _NOW + timedelta(seconds=10)
        assert ChannelDataCache._compute_gaps(query_start, query_end, covered) == expected


class TestCompaction:
    """``ChannelDataCache.MAX_SEGMENTS_PER_BUCKET`` triggers a merge.

    The cap bounds read fan-out: without it, every incremental
    ``put_segment`` would add another segment that ``get_range`` would
    have to load. Compaction folds the bucket into a single segment
    once the count crosses the cap, so the next ``get_range`` only
    walks one segment.

    The interesting invariants:

    1. **Below cap, no merge.** Compaction is a write-time side effect,
       not a constant background tax; under the cap the bucket layout
       is unchanged.
    2. **At cap+1, fires inline.** ``put_segment`` returns with the
       bucket already collapsed — the next reader doesn't pay for a
       fragmented bucket.
    3. **Data preserved through merge.** Concrete row count and column
       contents survive; LRU-evicted bodies degrade to absence (not
       errors) but their refs are still dropped from the index so the
       index never points at nothing.
    4. **Old segment keys are deleted.** Otherwise the store carries
       the merged seg + every superseded seg until LRU catches up,
       which would defeat the bucket's byte-budget accounting.

    A ``monkeypatch`` lowers the cap so tests stay fast — the prod
    constant is large enough that a literal 17-fetch test would dwarf
    the rest of the suite.
    """

    def test_below_cap_no_compaction(self, tmp_path, monkeypatch):
        """Three segments under a cap of four stay separate."""
        monkeypatch.setattr(ChannelDataCache, "MAX_SEGMENTS_PER_BUCKET", 4)
        store = DiskCache(disk_path=tmp_path / "below")
        adapter = ChannelDataCache(store)
        try:
            for i in range(3):
                _put(adapter, "c1", rows=2, start=_NOW + timedelta(seconds=i * 10))

            channel_keys = sorted(k for k in store if k.startswith("channel:"))
            assert channel_keys == [
                "channel:v2::c1:idx",
                "channel:v2::c1:seg:0",
                "channel:v2::c1:seg:1",
                "channel:v2::c1:seg:2",
            ]
        finally:
            store.close()

    def test_crossing_cap_collapses_to_single_segment(self, tmp_path, monkeypatch):
        """One write past the cap triggers an inline compaction.

        Pins all three observable effects in one place:
        * Index ends up with exactly one ref.
        * The merged segment's ``seg_id`` is fresh (no overlap with
          any pre-compaction id).
        * Old segment keys are gone from the raw store.
        """
        monkeypatch.setattr(ChannelDataCache, "MAX_SEGMENTS_PER_BUCKET", 3)
        store = DiskCache(disk_path=tmp_path / "cross")
        adapter = ChannelDataCache(store)
        try:
            for i in range(4):
                _put(adapter, "c1", rows=2, start=_NOW + timedelta(seconds=i * 10))

            idx = adapter._load_index("c1", None)
            assert idx is not None
            assert len(idx.segments) == 1
            merged_id = idx.segments[0].seg_id
            assert merged_id is not None
            assert merged_id >= 4

            channel_keys = sorted(k for k in store if k.startswith("channel:"))
            assert channel_keys == [
                "channel:v2::c1:idx",
                f"channel:v2::c1:seg:{merged_id}",
            ]
        finally:
            store.close()

    def test_compaction_preserves_data(self, tmp_path, monkeypatch):
        """``get_range`` after compaction returns the same rows as before.

        Builds disjoint, time-sorted frames so the dedup path is a
        no-op; correctness of the dedup itself is covered by the
        overlapping-timestamps test below.
        """
        monkeypatch.setattr(ChannelDataCache, "MAX_SEGMENTS_PER_BUCKET", 2)
        store = DiskCache(disk_path=tmp_path / "preserve")
        adapter = ChannelDataCache(store)
        try:
            frames = []
            for i in range(3):
                frame = _put(adapter, "c1", rows=2, start=_NOW + timedelta(seconds=i * 10))
                frames.append(frame)

            idx = adapter._load_index("c1", None)
            assert idx is not None
            assert len(idx.segments) == 1

            expected = pd.concat(frames)
            got, gaps = adapter.get_range(
                "c1",
                None,
                frames[0].index[0].to_pydatetime(),
                frames[-1].index[-1].to_pydatetime(),
            )
            assert gaps == []
            assert got is not None
            pd.testing.assert_frame_equal(got, expected)
        finally:
            store.close()

    def test_compaction_dedups_overlapping_timestamps(self, tmp_path, monkeypatch):
        """When two segments share a timestamp, the later put wins.

        Matches :meth:`get_range`'s ``groupby(level=0).last()`` dedup,
        so a refetch that overwrites a stale cached value still ends
        up with the fresh value after compaction.
        """
        monkeypatch.setattr(ChannelDataCache, "MAX_SEGMENTS_PER_BUCKET", 1)
        store = DiskCache(disk_path=tmp_path / "dedup")
        adapter = ChannelDataCache(store)
        try:
            ts = _NOW
            stale = pd.DataFrame(
                {"c1": [1.0]},
                index=pd.DatetimeIndex([ts], tz=timezone.utc),
            )
            fresh = pd.DataFrame(
                {"c1": [2.0]},
                index=pd.DatetimeIndex([ts], tz=timezone.utc),
            )
            adapter.put_segment("c1", None, stale, ts, ts)
            adapter.put_segment("c1", None, fresh, ts, ts)

            idx = adapter._load_index("c1", None)
            assert idx is not None
            assert len(idx.segments) == 1

            got, _ = adapter.get_range("c1", None, ts, ts)
            assert got is not None
            assert got.iloc[0, 0] == 2.0
        finally:
            store.close()

    def test_empty_refs_fold_into_merged_claim(self, tmp_path, monkeypatch):
        """Empty refs alongside data refs collapse into one data ref.

        After compaction the bucket carries a single data segment
        whose claimed range spans all prior refs (data + empty). The
        empty refs are dropped from the index, their coverage now
        represented by the merged ref's wider claim.
        """
        monkeypatch.setattr(ChannelDataCache, "MAX_SEGMENTS_PER_BUCKET", 2)
        store = DiskCache(disk_path=tmp_path / "fold_empty")
        adapter = ChannelDataCache(store)
        try:
            data = _put(adapter, "c1", rows=4, start=_NOW, freq="s")
            data_end = data.index[-1].to_pydatetime()
            empty_end = data_end + timedelta(seconds=10)
            # Second put (empty) crosses the cap of 2 → compacts.
            adapter.put_segment("c1", None, pd.DataFrame(), data_end, empty_end)
            adapter.put_segment(
                "c1", None, pd.DataFrame(), empty_end, empty_end + timedelta(seconds=10)
            )

            idx = adapter._load_index("c1", None)
            assert idx is not None
            assert len(idx.segments) == 1
            sole = idx.segments[0]
            assert sole.seg_id is not None  # data ref survives
            assert sole.start_time == data.index[0].to_pydatetime()
            assert sole.end_time == empty_end + timedelta(seconds=10)

            # Query the wider claim — data slice returns the actual
            # rows, and the empty tail counts as covered (no gap).
            got, gaps = adapter.get_range("c1", None, data.index[0].to_pydatetime(), sole.end_time)
            assert got is not None
            pd.testing.assert_frame_equal(got, data)
            assert gaps == []
        finally:
            store.close()

    def test_all_empty_bucket_collapses_to_single_empty_ref(self, tmp_path, monkeypatch):
        """Compacting a bucket with only empty refs leaves one empty ref.

        The merged claim is the union of every prior empty ref's
        claim, so the "no data anywhere in this window" coverage is
        preserved while the per-fetch refs collapse.
        """
        monkeypatch.setattr(ChannelDataCache, "MAX_SEGMENTS_PER_BUCKET", 2)
        store = DiskCache(disk_path=tmp_path / "all_empty")
        adapter = ChannelDataCache(store)
        try:
            t0 = _NOW
            t1 = _NOW + timedelta(seconds=10)
            t2 = _NOW + timedelta(seconds=20)
            t3 = _NOW + timedelta(seconds=30)
            for s, e in [(t0, t1), (t1, t2), (t2, t3)]:
                adapter.put_segment("c1", None, pd.DataFrame(), s, e)

            idx = adapter._load_index("c1", None)
            assert idx is not None
            assert len(idx.segments) == 1
            sole = idx.segments[0]
            assert sole.seg_id is None
            assert sole.start_time == t0
            assert sole.end_time == t3

            data, gaps = adapter.get_range("c1", None, t0, t3)
            assert data is None
            assert gaps == []

            # No segment keys at all — index-only bucket.
            seg_keys = [k for k in store if k.startswith("channel:") and ":seg:" in k]
            assert seg_keys == []
        finally:
            store.close()


class TestMergePages:
    """Behaviour of :meth:`DataLowLevelClient._merge_pages`.

    The helper replaces a previously inline O(N²) per-page concat loop with
    a single batched concat per channel. These tests pin the merge
    semantics so a future refactor can't silently drift:

    * Single-frame channels skip the concat entirely (cheap identity path).
    * Multi-frame channels concat in collected order; ``groupby.last``
      makes the latest frame win on overlapping timestamps.
    * Cached slices from the segment-stitched read are folded in as
      the *first* frame so fresh pages still win on overlap.
    """

    @pytest.mark.parametrize("pages", [[], [[]]], ids=["no_tasks_queued", "task_returned_empty"])
    def test_no_fresh_data_returns_initial(self, pages: list) -> None:
        """No fresh pages → initial dict passes through by identity."""
        client = DataLowLevelClient(MagicMock())
        initial_df = _frame("chan", rows=5)
        with _patch_deserializer({}):
            result = client._merge_pages(pages=pages, initial={"chan": initial_df})
        assert result["chan"] is initial_df

    def test_single_frame_skips_concat(self) -> None:
        """One frame for a channel → returned by identity, no concat call."""
        only_df = _frame("chan", rows=5)
        client = DataLowLevelClient(MagicMock())
        with _patch_deserializer({"p1": {"chan": only_df}}):
            result = client._merge_pages(pages=[["p1"]], initial={})
        assert result["chan"] is only_df

    def test_disjoint_pages_concat_in_order(self) -> None:
        """Multiple disjoint pages for one channel → single concat result."""
        df1 = _frame("chan", rows=10, start=_NOW, offset=0, freq="s")
        df2 = _frame("chan", rows=10, start=_NOW + timedelta(minutes=1), offset=10, freq="s")
        df3 = _frame("chan", rows=10, start=_NOW + timedelta(minutes=2), offset=20, freq="s")
        client = DataLowLevelClient(MagicMock())
        sentinels = {"p1": {"chan": df1}, "p2": {"chan": df2}, "p3": {"chan": df3}}
        with _patch_deserializer(sentinels):
            result = client._merge_pages(pages=[["p1", "p2"], ["p3"]], initial={})
        expected = pd.concat([df1, df2, df3]).groupby(level=0).last()
        pd.testing.assert_frame_equal(result["chan"].sort_index(), expected.sort_index())
        assert len(result["chan"]) == 30

    def test_overlapping_timestamps_later_page_wins(self) -> None:
        """On overlap, the later page's value survives ``groupby.last``.

        Pins the old inline ``concat([acc, new]).groupby(level=0).last()``
        semantic: latest concat position wins on conflict.
        """
        index = pd.date_range(_NOW, periods=5, freq="ms", tz=timezone.utc)
        df_first = pd.DataFrame({"chan": [0] * 5}, index=index)
        df_second = pd.DataFrame({"chan": [99] * 5}, index=index)
        client = DataLowLevelClient(MagicMock())
        with _patch_deserializer({"p1": {"chan": df_first}, "p2": {"chan": df_second}}):
            result = client._merge_pages(pages=[["p1", "p2"]], initial={})
        assert (result["chan"]["chan"] == 99).all()

    def test_cached_slice_folded_in_first_and_loses_on_overlap(self) -> None:
        """Cached slice from the segment read is the first frame in the merge.

        Fresh pages must overwrite cached values on duplicate timestamps,
        matching the pre-existing "latest fetch wins" semantic.
        """
        index = pd.date_range(_NOW, periods=5, freq="ms", tz=timezone.utc)
        cached = pd.DataFrame({"chan": [-1] * 5}, index=index)
        fresh = pd.DataFrame({"chan": [42] * 5}, index=index)
        client = DataLowLevelClient(MagicMock())
        with _patch_deserializer({"p1": {"chan": fresh}}):
            result = client._merge_pages(pages=[["p1"]], initial={"chan": cached})
        assert (result["chan"]["chan"] == 42).all()

    def test_multiple_channels_independent(self) -> None:
        """Per-channel grouping is independent: one channel's pages don't bleed."""
        a1 = _frame("a", rows=5, start=_NOW, offset=0, freq="s")
        a2 = _frame("a", rows=5, start=_NOW + timedelta(minutes=1), offset=5, freq="s")
        b1 = _frame("b", rows=5, start=_NOW, offset=100, freq="s")
        client = DataLowLevelClient(MagicMock())
        sentinels = {"p_a1": {"a": a1}, "p_a2": {"a": a2}, "p_b1": {"b": b1}}
        with _patch_deserializer(sentinels):
            result = client._merge_pages(pages=[["p_a1", "p_b1"], ["p_a2"]], initial={})
        assert len(result["a"]) == 10
        assert len(result["b"]) == 5
        assert (result["b"]["b"] >= 100).all()

    def test_does_not_mutate_initial(self) -> None:
        """``initial`` is a defensive copy; caller's dict isn't mutated."""
        cached = _frame("chan", rows=5)
        initial = {"chan": cached}
        fresh = _frame("chan", rows=5, start=_NOW + timedelta(seconds=1), offset=10)
        client = DataLowLevelClient(MagicMock())
        with _patch_deserializer({"p1": {"chan": fresh}}):
            client._merge_pages(pages=[["p1"]], initial=initial)
        assert initial["chan"] is cached


class TestDataLowLevelClient:
    """Constructor wiring and per-instance isolation.

    Per-call behaviour (cache hits, ``ignore_cache``, pagination) lives in
    :class:`TestGetChannelData`.
    """

    def test_default_construction_uses_no_op_store(self) -> None:
        """Default construction leaves the adapter wrapping a disabled store.

        Resources wire the shared store in via the keyword arg; the
        ``MagicMock()``-only path here keeps unit tests free of disk I/O.
        """
        client = DataLowLevelClient(MagicMock())
        assert isinstance(client.channel_cache, ChannelDataCache)
        assert not client.channel_cache.store.disk_enabled

    def test_per_instance_isolation(self, tmp_path) -> None:
        """Two clients with distinct stores must not share cache state.

        Regression test for the original OOM bug: ``channel_cache`` was a
        class attribute, so every ``SiftClient`` in the process appended
        to the same dict. Two fresh adapters over independent stores must
        stay independent — even now that store ownership has moved to the
        client, the contract is the same.
        """
        client_a = _client_with_cache(tmp_path, "a")
        client_b = _client_with_cache(tmp_path, "b")
        try:
            _put(client_a.channel_cache, "c1", rows=10)
            assert client_a.channel_cache.has_any("c1")
            assert not client_b.channel_cache.has_any("c1")
        finally:
            client_a.channel_cache.store.close()
            client_b.channel_cache.store.close()

    def test_adapter_kwarg_propagates(self, tmp_path) -> None:
        """The constructor honours an externally-constructed adapter.

        Mirrors the production wiring where ``ChannelsAPIAsync`` builds
        the adapter from ``client._get_disk_cache()`` and hands it in.
        """
        store = DiskCache(disk_path=tmp_path / "external", disk_max_bytes=8_192)
        adapter = ChannelDataCache(store)
        client = DataLowLevelClient(MagicMock(), channel_cache=adapter)
        try:
            assert client.channel_cache is adapter
            assert client.channel_cache.store is store
            assert client.channel_cache.store.disk_max_bytes == 8_192
        finally:
            store.close()


class TestGetChannelData:
    """End-to-end assertions on the public ``get_channel_data`` return shape."""

    @pytest.mark.asyncio
    async def test_single_page_per_channel(self) -> None:
        """Result is keyed by channel name; single-page frames pass through unchanged."""
        client = DataLowLevelClient(MagicMock())
        c1_df, c2_df = _frame("c1"), _frame("c2", offset=100)
        with _fake_grpc(client, {"c1": [c1_df], "c2": [c2_df]}):
            result = await client.get_channel_data(
                channels=[_channel("c1"), _channel("c2")],
                start_time=_NOW,
                end_time=_WINDOW_END,
                ignore_cache=True,
            )
        assert set(result.keys()) == {"c1", "c2"}
        pd.testing.assert_frame_equal(result["c1"], c1_df)
        pd.testing.assert_frame_equal(result["c2"], c2_df)

    @pytest.mark.asyncio
    async def test_multi_page_response_concatenated_per_channel(self) -> None:
        """Three disjoint pages for one channel → single merged frame.

        Catches regressions in the ``_handle_pagination`` + ``_merge_pages``
        interaction (the perf fix's batched concat must still produce the
        full 30-row contiguous result).
        """
        client = DataLowLevelClient(MagicMock())
        p1 = _frame("c1", rows=10, start=_NOW, offset=0)
        p2 = _frame("c1", rows=10, start=_NOW + timedelta(seconds=1), offset=10)
        p3 = _frame("c1", rows=10, start=_NOW + timedelta(seconds=2), offset=20)
        with _fake_grpc(client, {"c1": [p1, p2, p3]}):
            result = await client.get_channel_data(
                channels=[_channel("c1")],
                start_time=_NOW,
                end_time=_WINDOW_END,
                ignore_cache=True,
            )
        assert set(result.keys()) == {"c1"}
        assert len(result["c1"]) == 30
        expected = pd.concat([p1, p2, p3]).groupby(level=0).last()
        pd.testing.assert_frame_equal(result["c1"].sort_index(), expected.sort_index())

    @pytest.mark.asyncio
    async def test_cache_hit_short_circuits_grpc(self, tmp_path) -> None:
        """Second request for the same channel + window skips ``_get_data_impl``.

        Stages two pages-worth of data so a faulty cache that falls through
        wouldn't silently pass by hitting EOF — any second-call invocation
        would consume the second page and bump ``len(call_log)``.
        """
        client = _client_with_cache(tmp_path)
        df = _frame("c1")
        try:
            with _fake_grpc(client, {"c1": [df, df]}) as call_log:
                first = await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
                calls_after_first = len(call_log)
                assert calls_after_first >= 1

                second = await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
                assert len(call_log) == calls_after_first, (
                    "second call should be served from cache without invoking _get_data_impl"
                )
            pd.testing.assert_frame_equal(first["c1"].sort_index(), second["c1"].sort_index())
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_partial_cache_hit_merges_cached_and_fresh(self, tmp_path) -> None:
        """Cached + uncached channels resolved together in one return dict.

        Only the uncached channel triggers ``_get_data_impl``.
        """
        client = _client_with_cache(tmp_path)
        c1_df, c2_df = _frame("c1"), _frame("c2", offset=100)
        try:
            with _fake_grpc(client, {"c1": [c1_df], "c2": [c2_df]}) as call_log:
                await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
                calls_after_warmup = len(call_log)

                result = await client.get_channel_data(
                    channels=[_channel("c1"), _channel("c2")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
                new_calls = call_log[calls_after_warmup:]

            assert new_calls, "c2 should hit the wire on the second call"
            for call in new_calls:
                assert call["channel_ids"] == ["c2"], f"only c2 should hit the wire, saw {call!r}"
            assert set(result.keys()) == {"c1", "c2"}
            pd.testing.assert_frame_equal(result["c1"].sort_index(), c1_df.sort_index())
            pd.testing.assert_frame_equal(result["c2"].sort_index(), c2_df.sort_index())
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_run_id_keeps_cache_buckets_separate(self, tmp_path) -> None:
        """A cached entry under run-A must NOT satisfy a query for run-B.

        End-to-end version of the run-scope bug fix. Two sequential
        ``_fake_grpc`` blocks stage distinct single-page data for each
        run; the cache state persists across blocks because it lives on
        the adapter. With the bug unfixed (bare ``channel:<id>`` keys),
        the run-B query would short-circuit on run-A's cache entry
        instead of hitting the wire — the ``len(log_b) >= 1`` assertion
        is the canary.
        """
        client = _client_with_cache(tmp_path)
        df_a = _frame("c1", rows=4, offset=0)
        df_b = _frame("c1", rows=8, offset=100)
        try:
            with _fake_grpc(client, {"c1": [df_a]}) as log_a:
                first = await client.get_channel_data(
                    channels=[_channel("c1")],
                    run_id="run-A",
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert len(log_a) >= 1, "run-A should hit the wire on first call"

            with _fake_grpc(client, {"c1": [df_b]}) as log_b:
                second = await client.get_channel_data(
                    channels=[_channel("c1")],
                    run_id="run-B",
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert len(log_b) >= 1, "run-B should still hit the wire even though run-A cached c1"

            pd.testing.assert_frame_equal(first["c1"].sort_index(), df_a.sort_index())
            pd.testing.assert_frame_equal(second["c1"].sort_index(), df_b.sort_index())
            # Both runs end up cached independently; the unscoped bucket
            # stays empty because every query named a run.
            assert client.channel_cache.has_any("c1", "run-A")
            assert client.channel_cache.has_any("c1", "run-B")
            assert not client.channel_cache.has_any("c1")
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_pure_cache_hit_does_not_rewrite_disk(self, tmp_path) -> None:
        """A full cache hit skips ``_update_cache`` instead of rewriting bytes.

        Mitigates one face of the ``_update_cache`` write amplification:
        without the ``had_fresh_data`` gate, repeating the same query
        would re-merge and re-pickle the entry back to disk every
        call even though nothing changed. Spies on the adapter's
        ``put`` to assert zero writes on the second call.
        """
        client = _client_with_cache(tmp_path)
        df = _frame("c1")
        try:
            with _fake_grpc(client, {"c1": [df]}):
                await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )

            put_calls: list[Any] = []
            original_put = client.channel_cache.put_segment

            def spy_put_segment(*args: Any, **kwargs: Any) -> None:
                put_calls.append((args, kwargs))
                original_put(*args, **kwargs)

            client.channel_cache.put_segment = spy_put_segment  # type: ignore[method-assign]
            with _fake_grpc(client, {"c1": []}):
                await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert put_calls == [], "second call is a full cache hit; no disk write should occur"
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_disjoint_forward_paging_lands_in_separate_segments(self, tmp_path) -> None:
        """Two disjoint forward pulls write two segments and stitch on read.

        Under the segment shape, each fetch lands in its own cache key
        — no merging on the write path. A subsequent read stitches the
        segments back together via ``get_range``.
        """
        client = _client_with_cache(tmp_path)
        df1 = _frame("c1", rows=5, start=_NOW, offset=0)
        df2 = _frame("c1", rows=5, start=_NOW + timedelta(minutes=1), offset=100)
        window1_end = _NOW + timedelta(seconds=30)
        try:
            with _fake_grpc(client, {"c1": [df1]}):
                await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=window1_end,
                )
            with _fake_grpc(client, {"c1": [df2]}):
                await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=window1_end,
                    end_time=_WINDOW_END,
                )

            # Two segments under the parent id (no rewrite on the second
            # fetch — that's the whole point of the per-segment shape).
            idx = client.channel_cache._load_index("c1", None)
            assert idx is not None
            assert len(idx.segments) == 2

            stitched, gaps = client.channel_cache.get_range("c1", None, _NOW, _WINDOW_END)
            assert stitched is not None
            assert len(stitched) == 10
            assert set(stitched.columns) == {"c1"}
            # Reads still report the truly-uncovered window between the
            # two segments (segments only claim what their data spans
            # when run-scoped, but here we used the requested ranges).
            assert gaps == []
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_empty_wire_response_records_empty_ref_and_skips_refetch(self, tmp_path) -> None:
        """A query that returns no rows caches an empty ref; the repeat hits cache.

        End-to-end version of the empty-ref contract: a wire call that
        returns zero rows lands an empty :class:`SegmentRef` in the
        bucket so a repeat of the same query is a full cache hit (no
        wire call, no rows). Pins the "known-empty range doesn't
        refetch" behavior the segment-cache write amplification fix
        depended on for correctness.
        """
        client = _client_with_cache(tmp_path)
        try:
            # First call: zero pages → empty wire response.
            with _fake_grpc(client, {"c1": []}) as log1:
                result1 = await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert log1, "first call should hit the wire"
            assert "c1" not in result1

            # Bucket carries an empty ref for the queried window.
            assert client.channel_cache.has_any("c1")
            idx = client.channel_cache._load_index("c1", None)
            assert idx is not None
            assert len(idx.segments) == 1
            assert idx.segments[0].seg_id is None
            assert idx.segments[0].start_time == _NOW
            assert idx.segments[0].end_time == _WINDOW_END

            # Second identical call: cache hit, no wire traffic.
            with _fake_grpc(client, {"c1": []}) as log2:
                result2 = await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert log2 == [], "second call should be a full cache hit"
            assert "c1" not in result2
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_run_scoped_empty_wire_response_does_not_mask_future_data(self, tmp_path) -> None:
        """Run-scoped empty responses must not cache; later ingest still shows up.

        Absence at query time doesn't imply future absence for a run
        that may still be ingesting. Caching an empty ref over the
        queried window would permanently mask data the run writes
        later — the same window becomes a full cache hit (gaps ==
        ``[]``) and never refetches. This pins the run-scoped
        exception to the empty-ref rule.
        """
        client = _client_with_cache(tmp_path)
        df = _frame("c1", rows=4)
        try:
            with _fake_grpc(client, {"c1": []}) as log1:
                result1 = await client.get_channel_data(
                    channels=[_channel("c1")],
                    run_id="run-A",
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert log1, "first (empty) call should hit the wire"
            assert "c1" not in result1
            assert not client.channel_cache.has_any("c1", "run-A"), (
                "run-scoped empty response must not land an empty ref "
                "(would mask data the run ingests later)"
            )

            with _fake_grpc(client, {"c1": [df]}) as log2:
                result2 = await client.get_channel_data(
                    channels=[_channel("c1")],
                    run_id="run-A",
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert log2, (
                "second call must refetch — the previous empty response "
                "cannot become a cache hit for an ongoing run"
            )
            pd.testing.assert_frame_equal(result2["c1"].sort_index(), df.sort_index())
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_ignore_cache_true_returns_fresh_and_skips_write(self, tmp_path) -> None:
        """``ignore_cache=True`` returns mock data and leaves the cache empty.

        End-to-end version of the latent bug that compounded the customer's
        OOM: pre-fix, ``_update_cache`` ran even when the caller had asked
        the cache to be ignored.
        """
        client = _client_with_cache(tmp_path)
        df = _frame("c1")
        try:
            with _fake_grpc(client, {"c1": [df]}):
                result = await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                    ignore_cache=True,
                )
            pd.testing.assert_frame_equal(result["c1"], df)
            assert not client.channel_cache.has_any("c1")
        finally:
            client.channel_cache.store.close()


class TestBitFieldChannels:
    """Bitfield channels exercise the channel-id-vs-channel-name seam.

    A bitfield channel has one parent ``channel_id`` and N elements that
    surface in result dicts as dotted names (``parent.element``). The
    data path uses ids for the cache key (one entry per channel) but
    names for the result dict (one frame per element), so any test that
    only covers single-channel non-bitfield flows misses the join.

    These tests pin three things:

    1. ``_update_name_id_map`` populates an entry for the parent *and*
       every element, all pointing at the parent id — so when
       ``_update_cache`` looks up a dotted name, it lands on the
       parent's cache row.
    2. Element frames merge into a single wide-DataFrame segment
       under the parent id, and ``get_range`` correctly treats the
       bitfield as cached as a unit (zero gaps for the warmed window).
    3. Fresh-fetch and cache-hit return shapes match — both produce
       one single-column frame per dotted element name. The cache-hit
       branch slices the wide cached frame per column to keep this
       symmetry; without that slice, a cached bitfield would hand the
       full wide frame back under every dotted key.
    """

    def test_update_name_id_map_populates_dotted_and_parent_names(self) -> None:
        """Parent name and every dotted element name map to the parent id.

        This is the contract :meth:`_update_cache` relies on when it
        looks up ``name_id_map.get(channel_name)`` for the dotted names
        returned by ``try_deserialize_channel_data``.
        """
        client = DataLowLevelClient(MagicMock())
        ch = _bitfield_channel(cid="abc", name="ch1", elements=["lo", "hi"])
        client._update_name_id_map([ch])
        assert client.channel_cache.name_id_map == {
            "ch1": "abc",
            "ch1.lo": "abc",
            "ch1.hi": "abc",
        }

    def test_update_cache_merges_elements_into_one_segment(self, tmp_path) -> None:
        """All element frames land in one segment keyed by the parent id.

        ``_update_cache`` groups frames by their parent id before
        writing, so bitfield elements that came in one wire fetch
        produce exactly one wide segment — not one segment per element.
        Nothing should land under a dotted-name key.
        """
        client = _client_with_cache(tmp_path)
        ch = _bitfield_channel(cid="abc", name="ch1", elements=["lo", "hi"])
        client._update_name_id_map([ch])
        df_lo = _frame("ch1.lo", rows=5)
        df_hi = _frame("ch1.hi", rows=5)
        try:
            client._update_cache(
                channel_data={"ch1.lo": df_lo, "ch1.hi": df_hi},
                fetched_ranges_per_channel={"abc": [(_NOW, _WINDOW_END)]},
                start_time=_NOW,
                end_time=_WINDOW_END,
            )
            assert client.channel_cache.has_any("abc")
            assert not client.channel_cache.has_any("ch1.lo")
            assert not client.channel_cache.has_any("ch1.hi")

            # Exactly one segment under the parent id holding both
            # element columns. ``get_range`` returns the wide frame;
            # downstream slicing happens in ``get_channel_data``.
            idx = client.channel_cache._load_index("abc", None)
            assert idx is not None
            assert len(idx.segments) == 1

            data, _ = client.channel_cache.get_range("abc", None, _NOW, _WINDOW_END)
            assert data is not None
            assert set(data.columns) == {"ch1.lo", "ch1.hi"}
            assert len(data) == 5
        finally:
            client.channel_cache.store.close()

    def test_get_range_treats_bitfield_as_one_bucket(self, tmp_path) -> None:
        """A bitfield warmed by a prior fetch reports zero gaps for the same range.

        Pins the read side of the id/name asymmetry: a bitfield warmed
        by a prior fetch is "cached" as a unit under the parent id.
        Without this, a second call would re-fetch the whole window
        even though the elements are in the cache.
        """
        client = _client_with_cache(tmp_path)
        ch = _bitfield_channel(cid="abc", name="ch1", elements=["lo", "hi"])
        client._update_name_id_map([ch])
        try:
            client._update_cache(
                channel_data={
                    "ch1.lo": _frame("ch1.lo", rows=5),
                    "ch1.hi": _frame("ch1.hi", rows=5),
                },
                fetched_ranges_per_channel={"abc": [(_NOW, _WINDOW_END)]},
                start_time=_NOW,
                end_time=_WINDOW_END,
            )
            data, gaps = client.channel_cache.get_range("abc", None, _NOW, _WINDOW_END)
            assert data is not None
            assert gaps == []  # parent id covers the whole window
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_get_channel_data_fresh_bitfield_returns_per_element_frames(
        self, tmp_path
    ) -> None:
        """Fresh fetch: result has dotted keys, each a single-column frame.

        Mirrors ``try_deserialize_channel_data``'s output shape — one
        DataFrame per bitfield element, named after the element column.
        The cache is left warmed under the parent id so a follow-up
        test can compare the cache-hit shape against this baseline.
        """
        client = _client_with_cache(tmp_path)
        ch = _bitfield_channel(cid="abc", name="ch1", elements=["lo", "hi"])
        df_lo = _frame("ch1.lo", rows=5)
        df_hi = _frame("ch1.hi", rows=5)
        try:
            with _fake_grpc(client, {"abc": [{"ch1.lo": df_lo, "ch1.hi": df_hi}]}):
                result = await client.get_channel_data(
                    channels=[ch],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert set(result.keys()) == {"ch1.lo", "ch1.hi"}
            assert list(result["ch1.lo"].columns) == ["ch1.lo"]
            assert list(result["ch1.hi"].columns) == ["ch1.hi"]
            pd.testing.assert_frame_equal(result["ch1.lo"].sort_index(), df_lo.sort_index())
            pd.testing.assert_frame_equal(result["ch1.hi"].sort_index(), df_hi.sort_index())
            # Cache holds one wide segment under the parent id.
            data, _ = client.channel_cache.get_range("abc", None, _NOW, _WINDOW_END)
            assert data is not None
            assert set(data.columns) == {"ch1.lo", "ch1.hi"}
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_get_channel_data_cached_bitfield_returns_per_element_frames(
        self, tmp_path
    ) -> None:
        """Cache-hit shape matches fresh-fetch shape (per-element single-column).

        Same query as :func:`test_get_channel_data_fresh_bitfield_returns_per_element_frames`
        run twice — the second call must NOT hit the wire (full cache
        hit) AND must return the same per-element single-column shape.
        Regression guard for the cache-hit branch's ``cached_data[[name]]``
        slice — without it, a cached bitfield would return the wide
        cached frame under every dotted key.
        """
        client = _client_with_cache(tmp_path)
        ch = _bitfield_channel(cid="abc", name="ch1", elements=["lo", "hi"])
        df_lo = _frame("ch1.lo", rows=5)
        df_hi = _frame("ch1.hi", rows=5)
        try:
            with _fake_grpc(client, {"abc": [{"ch1.lo": df_lo, "ch1.hi": df_hi}]}):
                await client.get_channel_data(
                    channels=[ch],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            with _fake_grpc(client, {"abc": []}) as log:
                cached = await client.get_channel_data(
                    channels=[ch],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert log == [], "second call must be a full cache hit (no wire calls)"
            assert set(cached.keys()) == {"ch1.lo", "ch1.hi"}
            assert list(cached["ch1.lo"].columns) == ["ch1.lo"]
            assert list(cached["ch1.hi"].columns) == ["ch1.hi"]
            # ``check_freq=False``: the cached frame loses its DatetimeIndex
            # ``freq`` attribute through the concat/groupby roundtrip in
            # ``_update_cache``. Values and timestamps still match — only
            # the (non-semantic) freq metadata diverges.
            pd.testing.assert_frame_equal(
                cached["ch1.lo"].sort_index(), df_lo.sort_index(), check_freq=False
            )
            pd.testing.assert_frame_equal(
                cached["ch1.hi"].sort_index(), df_hi.sort_index(), check_freq=False
            )
        finally:
            client.channel_cache.store.close()
