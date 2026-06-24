"""Tests for :mod:`sift_client._internal.low_level_wrappers.data`.

Four classes, narrowest scope first:

* :class:`TestChannelCache` — pure ``ChannelCache`` unit tests (byte
  accounting, LRU promotion, eviction).
* :class:`TestMergePages` — ``DataLowLevelClient._merge_pages``, the
  per-channel concat helper.
* :class:`TestDataLowLevelClient` — constructor wiring and per-instance
  isolation.
* :class:`TestGetChannelData` — end-to-end on the public
  ``get_channel_data`` API against a mocked ``_get_data_impl``.

The OOM regression that motivated this code happened because the cache was
a class attribute that grew without bound. ``test_per_instance_isolation``
is the canary that catches anyone re-introducing that pattern.
"""

from __future__ import annotations

from contextlib import contextmanager
from datetime import datetime, timedelta, timezone
from typing import Any, Iterator
from unittest.mock import MagicMock, patch

import pandas as pd
import pytest

from sift_client._internal.low_level_wrappers.data import (
    DEFAULT_DATA_CACHE_MAX_BYTES,
    ChannelCache,
    ChannelCacheEntry,
    DataLowLevelClient,
    _new_cache_entry,
)
from sift_client.sift_types.channel import Channel, ChannelDataType

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


def _invariant_holds(cache: ChannelCache) -> bool:
    """``total_bytes`` must equal the sum of per-entry sizes at all times."""
    return cache.total_bytes == sum(e.size_bytes for e in cache._entries.values())


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
    channel_to_pages: dict[str, list[pd.DataFrame]],
) -> Iterator[list[dict[str, Any]]]:
    """Mock the gRPC boundary so each "page" is a sentinel string.

    ``_get_data_impl`` is replaced with a coroutine that pops one DataFrame
    off ``channel_to_pages[cid]`` per call per channel, until exhausted.
    ``try_deserialize_channel_data`` is patched to map the sentinel back to
    the corresponding ``{channel: DataFrame}`` dict.

    Yields a ``call_log`` list so tests can assert which channels actually
    hit the wire. The patch is torn down and ``_get_data_impl`` restored on
    exit.
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
            sentinel_to_frames[sentinel] = {cid: channel_to_pages[cid][i]}
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


class TestChannelCache:
    """Byte accounting, LRU promotion, eviction."""

    def test_put_get_roundtrip_and_size_replacement(self) -> None:
        """First put records size; second put on same key replaces it.

        Without size reclamation on the second put, ``total_bytes`` would
        double-count and trip the eviction loop on the next insert.
        """
        cache = ChannelCache(max_bytes=DEFAULT_DATA_CACHE_MAX_BYTES)
        small, big = _entry(rows=10), _entry(rows=1000)
        cache.put("c1", small)
        assert cache.get("c1") is small
        assert cache.total_bytes == small.size_bytes
        cache.put("c1", big)
        assert cache.get("c1") is big
        assert cache.total_bytes == big.size_bytes  # not small + big
        assert _invariant_holds(cache)

    def test_invalidate(self) -> None:
        """Removes a present entry and decrements bytes; no-op for missing keys."""
        cache = ChannelCache(max_bytes=DEFAULT_DATA_CACHE_MAX_BYTES)
        cache.invalidate("never_added")  # safe before any puts
        assert cache.total_bytes == 0
        cache.put("c1", _entry(rows=10))
        cache.invalidate("c1")
        assert cache.get("c1") is None
        assert cache.total_bytes == 0
        assert _invariant_holds(cache)

    def test_clear(self) -> None:
        cache = ChannelCache(max_bytes=DEFAULT_DATA_CACHE_MAX_BYTES)
        cache.put("c1", _entry(rows=10))
        cache.put("c2", _entry(rows=20))
        cache.clear()
        assert cache.total_bytes == 0
        assert len(cache) == 0
        assert _invariant_holds(cache)

    def test_oldest_entry_evicted_first(self) -> None:
        """Insertion order determines eviction when only puts have happened."""
        a, b, c = _entry(rows=50), _entry(rows=50), _entry(rows=50)
        cache = ChannelCache(max_bytes=a.size_bytes + b.size_bytes)  # room for two
        cache.put("a", a)
        cache.put("b", b)
        cache.put("c", c)  # evicts "a"
        assert "a" not in cache
        assert "b" in cache
        assert "c" in cache
        assert cache.total_bytes <= a.size_bytes + b.size_bytes
        assert _invariant_holds(cache)

    def test_get_promotes_to_most_recent(self) -> None:
        """Reading an entry must protect it from the next eviction."""
        a, b, c = _entry(rows=50), _entry(rows=50), _entry(rows=50)
        cache = ChannelCache(max_bytes=a.size_bytes + b.size_bytes)
        cache.put("a", a)
        cache.put("b", b)
        assert cache.get("a") is a  # promote a
        cache.put("c", c)  # b is now oldest, gets evicted
        assert "a" in cache
        assert "b" not in cache
        assert "c" in cache
        assert _invariant_holds(cache)

    def test_oversized_entry_evicts_with_neighbours(self) -> None:
        """A single entry larger than the cap ends up evicted itself.

        The alternative ("keep the oversized entry and accept that the cap
        is soft") would silently reintroduce unbounded growth for any
        workload whose typical entry is bigger than ``max_bytes``.
        """
        small_a, small_b, oversized = _entry(rows=10), _entry(rows=10), _entry(rows=10_000)
        cache = ChannelCache(max_bytes=small_a.size_bytes + small_b.size_bytes)
        cache.put("a", small_a)
        cache.put("b", small_b)
        cache.put("huge", oversized)
        assert "huge" not in cache
        # Every other entry was evicted in the failed attempt to make room.
        assert "a" not in cache
        assert "b" not in cache
        assert cache.total_bytes == 0
        assert _invariant_holds(cache)

    def test_max_bytes_zero_disables_cache(self) -> None:
        cache = ChannelCache(max_bytes=0)
        cache.put("c1", _entry(rows=100))
        assert not cache.enabled
        assert cache.get("c1") is None
        assert cache.total_bytes == 0
        assert len(cache) == 0

    def test_negative_max_bytes_raises(self) -> None:
        with pytest.raises(ValueError, match="data_cache_max_bytes"):
            ChannelCache(max_bytes=-1)

    def test_repeated_concat_updates_stay_under_bound(self) -> None:
        """Simulates the customer's sliding-window pull: same channel, growing.

        Without size reclamation on update, ``total_bytes`` would creep
        above the cap silently. We re-build the entry each iteration to
        mimic the ``_update_cache`` concat path.
        """
        cap = 1_000_000  # ~1 MB
        cache = ChannelCache(max_bytes=cap)
        accumulated = pd.DataFrame()
        for i in range(50):
            chunk = _frame(rows=1000, start=_NOW + timedelta(seconds=i), freq="us")
            accumulated = pd.concat([accumulated, chunk])
            cache.put(
                "c1",
                _new_cache_entry(
                    data=accumulated,
                    start_time=accumulated.index[0].to_pydatetime(),
                    end_time=accumulated.index[-1].to_pydatetime(),
                ),
            )
            assert cache.total_bytes <= cap, (
                f"iteration {i}: total_bytes={cache.total_bytes} exceeded cap={cap}"
            )
            assert _invariant_holds(cache)


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

    @pytest.mark.parametrize(
        "pages", [[], [[]]], ids=["no_tasks_queued", "task_returned_empty"]
    )
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

    def test_per_instance_isolation(self) -> None:
        """Two clients must not share cache state.

        Regression test for the original OOM bug: ``channel_cache`` was a
        class attribute, so every ``SiftClient`` in the process appended to
        the same dict. Two fresh clients must have independent caches.
        """
        client_a = DataLowLevelClient(MagicMock())
        client_b = DataLowLevelClient(MagicMock())
        client_a.channel_cache.put("c1", _entry(rows=10))
        assert "c1" in client_a.channel_cache
        assert "c1" not in client_b.channel_cache
        assert client_b.channel_cache.total_bytes == 0

    def test_data_cache_max_bytes_kwarg_propagates(self) -> None:
        """``data_cache_max_bytes`` is forwarded to the underlying cache.

        The disabled-cache *behaviour* itself is covered by
        :meth:`TestChannelCache.test_max_bytes_zero_disables_cache`; this
        test just verifies the constructor passes the kwarg through.
        """
        assert DataLowLevelClient(MagicMock(), data_cache_max_bytes=0).channel_cache.max_bytes == 0
        assert DataLowLevelClient(MagicMock(), data_cache_max_bytes=42).channel_cache.max_bytes == 42


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
    async def test_cache_hit_short_circuits_grpc(self) -> None:
        """Second request for the same channel + window skips ``_get_data_impl``.

        Stages two pages-worth of data so a faulty cache that falls through
        wouldn't silently pass by hitting EOF — any second-call invocation
        would consume the second page and bump ``len(call_log)``.
        """
        client = DataLowLevelClient(MagicMock())
        df = _frame("c1")
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

    @pytest.mark.asyncio
    async def test_partial_cache_hit_merges_cached_and_fresh(self) -> None:
        """Cached + uncached channels resolved together in one return dict.

        Only the uncached channel triggers ``_get_data_impl``.
        """
        client = DataLowLevelClient(MagicMock())
        c1_df, c2_df = _frame("c1"), _frame("c2", offset=100)
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
            assert call["channel_ids"] == ["c2"], (
                f"only c2 should hit the wire, saw {call!r}"
            )
        assert set(result.keys()) == {"c1", "c2"}
        pd.testing.assert_frame_equal(result["c1"].sort_index(), c1_df.sort_index())
        pd.testing.assert_frame_equal(result["c2"].sort_index(), c2_df.sort_index())

    @pytest.mark.asyncio
    async def test_ignore_cache_true_returns_fresh_and_skips_write(self) -> None:
        """``ignore_cache=True`` returns mock data and leaves the cache empty.

        End-to-end version of the latent bug that compounded the customer's
        OOM: pre-fix, ``_update_cache`` ran even when the caller had asked
        the cache to be ignored.
        """
        client = DataLowLevelClient(MagicMock())
        df = _frame("c1")
        with _fake_grpc(client, {"c1": [df]}):
            result = await client.get_channel_data(
                channels=[_channel("c1")],
                start_time=_NOW,
                end_time=_WINDOW_END,
                ignore_cache=True,
            )
        pd.testing.assert_frame_equal(result["c1"], df)
        assert "c1" not in client.channel_cache
        assert client.channel_cache.total_bytes == 0
