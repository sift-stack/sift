"""Tests for :mod:`sift_client._internal.low_level_wrappers.data`.

Five classes, narrowest scope first:

* :class:`TestChannelDataCache` — the typed adapter over the shared
  :class:`DiskCache`. Covers key namespacing, the isinstance guard on
  ``get``, and the prefix-scoped ``clear``.
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
"""

from __future__ import annotations

from contextlib import contextmanager
from datetime import datetime, timedelta, timezone
from typing import Any, Iterator
from unittest.mock import MagicMock, patch

import pandas as pd
import pytest

from sift_client._internal.disk_cache import DiskCache
from sift_client._internal.low_level_wrappers.data import (
    ChannelCacheEntry,
    ChannelDataCache,
    DataLowLevelClient,
    _new_cache_entry,
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


def _entry(*, rows: int = 5, value_dtype: str = "float64") -> ChannelCacheEntry:
    """``ChannelCacheEntry`` wrapping a small generated DataFrame."""
    data = _frame(rows=rows, value_dtype=value_dtype)
    return _new_cache_entry(
        data=data,
        start_time=data.index[0].to_pydatetime(),
        end_time=data.index[-1].to_pydatetime(),
    )


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
            ChannelBitFieldElement(name=el, index=i, bit_count=8)
            for i, el in enumerate(elements)
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


class TestChannelDataCache:
    """The typed adapter over the shared :class:`DiskCache`.

    Four invariants get pinned:

    1. Every operation routes through the namespaced key
       (``channel:<run_id>:<id>``), so two adapters sharing one store
       don't collide on bare resource ids.
    2. Run id is part of the cache dimension: the same ``channel_id``
       under two different runs is two cache buckets, not one.
    3. :meth:`ChannelDataCache.get` returns ``None`` on a type-mismatch
       hit (e.g. a row another adapter wrote) instead of handing
       arbitrary objects to downstream pandas code.
    4. :meth:`ChannelDataCache.clear` wipes only the adapter's namespace
       — entries belonging to other adapters survive.

    Store-level behaviour (oversized guards, cross-session reload,
    marker-checked clear_disk) is exercised in ``test_disk_cache.py``.
    """

    def test_get_miss_returns_none(self, tmp_path):
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "miss"))
        try:
            assert not adapter.has("c1")
            assert adapter.get("c1") is None
        finally:
            adapter.store.close()

    def test_round_trip(self, tmp_path):
        """Put then get returns an equivalent entry."""
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "rt"))
        try:
            entry = _entry(rows=8)
            adapter.put("c1", entry)
            assert adapter.has("c1")
            got = adapter.get("c1")
            assert got is not None
            pd.testing.assert_frame_equal(got.data, entry.data)
            assert got.start_time == entry.start_time
            assert got.end_time == entry.end_time
        finally:
            adapter.store.close()

    def test_writes_use_namespaced_key(self, tmp_path):
        """The raw store sees ``channel:<run>:<id>``, not the bare id.

        Pins the key-shape contract two adapters share. Without it, a
        second adapter that happens to share an id with the channel
        adapter would clobber the channel row. Empty run segment marks
        the unscoped bucket; UUIDs can never be empty so there's no
        collision risk.
        """
        store = DiskCache(disk_path=tmp_path / "ns")
        adapter = ChannelDataCache(store)
        try:
            adapter.put("c1", _entry(rows=4))
            assert "channel::c1" in store
            assert "c1" not in store
            assert "channel:c1" not in store  # old (pre-run-scoping) shape
        finally:
            store.close()

    def test_run_id_is_part_of_the_key(self, tmp_path):
        """Same channel under two runs is two cache buckets, not one.

        Regression guard for the bug this code was added to fix: a bare
        ``channel:<id>`` key conflated runs and served run-A's data to a
        query for run-B. ``run_id`` is now a label on the cache
        dimension; put under run-A must not be visible to a get under
        run-B (or the unscoped bucket).
        """
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "runs"))
        try:
            entry_a = _entry(rows=4)
            entry_b = _entry(rows=8)
            adapter.put("c1", entry_a, run_id="run-A")
            adapter.put("c1", entry_b, run_id="run-B")

            assert adapter.has("c1", "run-A")
            assert adapter.has("c1", "run-B")
            assert not adapter.has("c1")  # unscoped bucket stays empty
            assert not adapter.has("c1", "run-C")  # unknown run still misses

            got_a = adapter.get("c1", "run-A")
            got_b = adapter.get("c1", "run-B")
            assert got_a is not None
            assert got_b is not None
            assert len(got_a.data) == 4
            assert len(got_b.data) == 8
        finally:
            adapter.store.close()

    def test_unscoped_and_scoped_buckets_are_independent(self, tmp_path):
        """An unscoped put (``run_id=None``) doesn't satisfy a run-scoped get.

        Pins the other direction of the run-scope contract.
        """
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "indep"))
        try:
            adapter.put("c1", _entry(rows=4))  # no run
            assert adapter.has("c1")
            assert not adapter.has("c1", "run-A")
            assert adapter.get("c1", "run-A") is None
        finally:
            adapter.store.close()

    def test_get_isinstance_check_filters_foreign_rows(self, tmp_path):
        """A row whose value isn't a ChannelCacheEntry reads as a miss.

        Models a corrupt entry or a key collision from another writer.
        ``ChannelDataCache.get`` must isinstance-check the raw value so
        callers downstream never receive the wrong shape.
        """
        store = DiskCache(disk_path=tmp_path / "foreign")
        adapter = ChannelDataCache(store)
        try:
            store.put("channel::c1", {"not": "an entry"}, size_bytes=64)
            assert adapter.get("c1") is None
        finally:
            store.close()

    def test_invalidate_is_run_scoped(self, tmp_path):
        """``invalidate`` only drops the named ``(channel, run)`` bucket.

        Entries for the same channel under other runs survive — runs
        are independent cache dimensions, so dropping one shouldn't
        cascade.
        """
        adapter = ChannelDataCache(DiskCache(disk_path=tmp_path / "inval"))
        try:
            adapter.invalidate("never_added")  # safe before any puts
            adapter.put("c1", _entry(rows=4), run_id="run-A")
            adapter.put("c1", _entry(rows=8), run_id="run-B")
            adapter.invalidate("c1", "run-A")
            assert not adapter.has("c1", "run-A")
            assert adapter.get("c1", "run-A") is None
            assert adapter.has("c1", "run-B")  # run-B survives
        finally:
            adapter.store.close()

    def test_clear_is_prefix_scoped(self, tmp_path):
        """``clear`` drops channel rows across all runs, leaves other adapters alone.

        Simulates a second resource writing to the same store with a
        different prefix; the channel adapter's clear must not be a
        whole-store wipe, but it must reach every run-scoped bucket.
        """
        store = DiskCache(disk_path=tmp_path / "scoped")
        adapter = ChannelDataCache(store)
        try:
            adapter.put("c1", _entry(rows=4))  # unscoped
            adapter.put("c2", _entry(rows=4), run_id="run-A")
            # Simulate a row written by a different adapter.
            store.put("other:1", "foreign-value", size_bytes=64)
            adapter.clear()
            assert not adapter.has("c1")
            assert not adapter.has("c2", "run-A")
            assert "other:1" in store
        finally:
            store.close()

    def test_size_bytes_propagates_to_store(self, tmp_path):
        """The adapter forwards the entry's ``size_bytes`` to the store guard.

        Sized below the entry's actual ``size_bytes`` so the store's
        oversize guard kicks in. The adapter never measures size itself;
        it relies on ``_new_cache_entry`` having stamped the value.
        """
        entry = _entry(rows=10_000)
        store = DiskCache(disk_path=tmp_path / "size", disk_max_bytes=entry.size_bytes // 2)
        adapter = ChannelDataCache(store)
        try:
            adapter.put("c1", entry)
            assert not adapter.has("c1")  # oversize skipped by the store
        finally:
            store.close()

    def test_no_op_store_keeps_adapter_silent(self):
        """An adapter on a disabled store behaves like a cold cache.

        Disabling the store is the path ``client.cache.disable()``
        exercises; resources can keep their adapter reference and every
        operation just no-ops.
        """
        adapter = ChannelDataCache(DiskCache())
        assert not adapter.store.disk_enabled
        adapter.put("c1", _entry(rows=4))
        assert not adapter.has("c1")
        assert adapter.get("c1") is None
        adapter.invalidate("c1")
        adapter.clear()


class TestMergePages:
    """Behaviour of :meth:`DataLowLevelClient._merge_pages`.

    The helper replaces a previously inline O(N²) per-page concat loop with
    a single batched concat per channel. These tests pin the merge
    semantics so a future refactor can't silently drift:

    * Single-frame channels skip the concat entirely (cheap identity path).
    * Multi-frame channels concat in collected order; ``groupby.last``
      makes the latest frame win on overlapping timestamps.
    * Cached slices from ``_check_cache`` are folded in as the *first*
      frame so fresh pages still win on overlap.
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
        """Cached slice from ``_check_cache`` is the first frame in the merge.

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
            client_a.channel_cache.put("c1", _entry(rows=10))
            assert client_a.channel_cache.has("c1")
            assert not client_b.channel_cache.has("c1")
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
            assert len(log_b) >= 1, (
                "run-B should still hit the wire even though run-A cached c1"
            )

            pd.testing.assert_frame_equal(first["c1"].sort_index(), df_a.sort_index())
            pd.testing.assert_frame_equal(second["c1"].sort_index(), df_b.sort_index())
            # Both runs end up cached independently; the unscoped bucket
            # stays empty because every query named a run.
            assert client.channel_cache.has("c1", "run-A")
            assert client.channel_cache.has("c1", "run-B")
            assert not client.channel_cache.has("c1")
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
            original_put = client.channel_cache.put

            def spy_put(*args: Any, **kwargs: Any) -> None:
                put_calls.append((args, kwargs))
                original_put(*args, **kwargs)

            client.channel_cache.put = spy_put  # type: ignore[method-assign]
            with _fake_grpc(client, {"c1": []}):
                await client.get_channel_data(
                    channels=[_channel("c1")],
                    start_time=_NOW,
                    end_time=_WINDOW_END,
                )
            assert put_calls == [], (
                "second call is a full cache hit; no disk write should occur"
            )
        finally:
            client.channel_cache.store.close()

    @pytest.mark.asyncio
    async def test_disjoint_forward_paging_preserves_all_rows(self, tmp_path) -> None:
        """Strictly disjoint append skips ``groupby.last`` and keeps every row.

        Pins the Tier-1C optimization: when the second query's range
        starts strictly after the cached range, ``_update_cache`` uses
        plain ``pd.concat`` instead of ``concat + groupby.last``. The
        groupby would be correct but wasted (no overlap to dedup); this
        test guards against a future regression that drops rows on the
        fast path.
        """
        client = _client_with_cache(tmp_path)
        df1 = _frame("c1", rows=5, start=_NOW, offset=0)
        df2 = _frame(
            "c1", rows=5, start=_NOW + timedelta(minutes=1), offset=100
        )
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

            entry = client.channel_cache.get("c1")
            assert entry is not None
            assert len(entry.data) == 10
            assert set(entry.data.columns) == {"c1"}
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
            assert not client.channel_cache.has("c1")
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
    2. Element frames merge into a single wide-DataFrame cache entry
       under the parent id, and ``_filter_cached_channels`` correctly
       treats the bitfield as cached as a unit.
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

    def test_update_cache_merges_elements_under_parent_id(self, tmp_path) -> None:
        """All element frames land in one cache entry keyed by the parent id.

        ``_update_cache`` iterates element frames; the first put writes
        a single-column entry, the second sees ``existing`` for the
        same parent id and merges via
        ``pd.concat([...]).groupby(level=0).last()`` into a wide frame.
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
                start_time=_NOW,
                end_time=_WINDOW_END,
            )
            assert client.channel_cache.has("abc")
            assert not client.channel_cache.has("ch1.lo")
            assert not client.channel_cache.has("ch1.hi")
            entry = client.channel_cache.get("abc")
            assert entry is not None
            assert set(entry.data.columns) == {"ch1.lo", "ch1.hi"}
            # Overlapping indices collapse via groupby.last; 5 rows in,
            # 5 rows out (with both element columns populated).
            assert len(entry.data) == 5
        finally:
            client.channel_cache.store.close()

    def test_filter_cached_channels_treats_bitfield_as_one(self, tmp_path) -> None:
        """``_filter_cached_channels`` consults the parent id, not element names.

        Pins the read side of the id/name asymmetry: a bitfield warmed
        by a prior fetch is "cached" as a unit. Without this, a second
        call would re-issue the wire fetch for the parent id even
        though the elements are in the cache.
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
                start_time=_NOW,
                end_time=_WINDOW_END,
            )
            cached, not_cached = client._filter_cached_channels(["abc"])
            assert cached == ["abc"]
            assert not_cached == []
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
            # Cache holds one wide entry under the parent id.
            entry = client.channel_cache.get("abc")
            assert entry is not None
            assert set(entry.data.columns) == {"ch1.lo", "ch1.hi"}
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
