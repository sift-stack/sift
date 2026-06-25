"""Tests for :mod:`sift_client._internal.low_level_wrappers.data`.

Four classes, narrowest scope first:

* :class:`TestChannelCache` — disk-backed :class:`ChannelCache` unit tests
  (fresh open, cross-session reload, invalidate/clear, oversized guards,
  disable/reconfigure).
* :class:`TestChannelCacheClearDisk` — ``ChannelCache.clear_disk``
  classmethod (default path, custom path, safety guard).
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

import logging
from contextlib import contextmanager
from datetime import datetime, timedelta, timezone
from typing import Any, Iterator
from unittest.mock import MagicMock, patch

import pandas as pd
import pytest

from sift_client._internal.low_level_wrappers.data import (
    ChannelCache,
    ChannelCacheEntry,
    DataLowLevelClient,
    _new_cache_entry,
)
from sift_client.sift_types.channel import Channel, ChannelDataType

_NOW = datetime(2025, 1, 1, tzinfo=timezone.utc)
_WINDOW_END = _NOW + timedelta(days=1)

# Snapshot of the real ``DEFAULT_DISK_PATH`` constant captured at module import.
# The autouse ``_isolate_default_disk_cache_path`` fixture in ``conftest.py``
# overrides the class attribute on every test for isolation; the
# ``TestChannelCacheClearDisk::test_default_path_constant_under_tmp`` test still
# needs to see the production value to verify its shape.
_PRODUCTION_DEFAULT_DISK_PATH = ChannelCache.DEFAULT_DISK_PATH


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


def _client_with_cache(tmp_path, subdir: str = "cache") -> DataLowLevelClient:
    """Build a ``DataLowLevelClient`` whose ``ChannelCache`` points at ``tmp_path``.

    Tests that exercise cache behaviour (hits/misses/eviction) need an
    actual disk-backed cache, so ``disk_cache_path`` must be supplied. A
    plain ``DataLowLevelClient(MagicMock())`` defaults to no-cache mode
    and would silently turn every cache test into a wire-path test.
    """
    return DataLowLevelClient(MagicMock(), disk_cache_path=tmp_path / subdir)


@contextmanager
def _capture_data_warnings() -> Iterator[list[logging.LogRecord]]:
    """Capture warnings emitted by the ``data`` module's logger directly.

    Pytest's ``caplog`` reads from the root logger, but the Sift pytest plugin
    sets ``propagate=False`` on the ``sift_client`` logger when audit logging
    is active, so records emitted from any descendant don't reach the root.
    Attaching a list-backed handler at the leaf logger bypasses that and
    surfaces exactly the records we emit.
    """
    target = logging.getLogger("sift_client._internal.low_level_wrappers.data")
    records: list[logging.LogRecord] = []

    class _ListHandler(logging.Handler):
        def emit(self, record: logging.LogRecord) -> None:
            records.append(record)

    handler = _ListHandler(level=logging.WARNING)
    target.addHandler(handler)
    try:
        yield records
    finally:
        target.removeHandler(handler)


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
    """Disk-backed :class:`ChannelCache` behaviour.

    Five invariants must hold across these tests:

    1. Constructing without a ``disk_path`` yields a no-op cache (every
       operation is silent; ``__contains__`` returns ``False``).
    2. A fresh disk directory starts empty and accepts new writes.
    3. Closing a populated cache and reopening at the same path surfaces
       the previous entries on read (the "previous session" requirement
       that powers cold-start reuse).
    4. Oversized entries are skipped with a deduped warning rather than
       being inserted and triggering an eviction storm.
    5. ``invalidate``/``clear`` reset the oversized-warning dedup state
       so a future regression re-warns.

    All tests confine writes to ``tmp_path`` so nothing leaks into the
    real ``/tmp/sift-channel-data-cache``.
    """

    def test_disabled_when_no_path(self) -> None:
        """``ChannelCache()`` with no ``disk_path`` is a silent no-op."""
        cache = ChannelCache()
        assert cache.disk_enabled is False
        assert cache.disk_path is None
        assert cache.disk_max_bytes is None
        # Operations don't raise; the cache just stays empty.
        cache.put("chan-1", _entry(rows=4))
        assert "chan-1" not in cache
        assert cache.get("chan-1") is None
        cache.invalidate("chan-1")
        cache.clear()
        cache.close()

    def test_fresh_cache_writes_and_reads(self, tmp_path) -> None:
        """A fresh disk directory accepts writes and serves them back."""
        path = tmp_path / "fresh"
        cache = ChannelCache(disk_path=path)
        try:
            assert cache.disk_enabled
            assert cache.disk_path == str(path)
            assert cache.disk_max_bytes == ChannelCache.DEFAULT_DISK_MAX_BYTES
            entry = _entry(rows=8)
            cache.put("chan-1", entry)
            assert "chan-1" in cache
            got = cache.get("chan-1")
            assert got is not None
            pd.testing.assert_frame_equal(got.data, entry.data)
            assert got.start_time == entry.start_time
            assert got.end_time == entry.end_time
        finally:
            cache.close()

    def test_reopen_existing_dir_sees_prior_session_entries(self, tmp_path) -> None:
        """Closing then reopening at the same path makes prior entries hit.

        This is the "look for existing caches from previous sessions"
        guarantee: a new ``ChannelCache`` at a populated directory finds
        entries on disk and returns them on the next read.
        """
        path = tmp_path / "prev-session"
        df = _frame("chan-1", rows=12, freq="s")
        original_entry = _new_cache_entry(
            data=df,
            start_time=df.index[0].to_pydatetime(),
            end_time=df.index[-1].to_pydatetime(),
        )
        # Session 1: populate and close.
        session1 = ChannelCache(disk_path=path)
        session1.put("chan-1", original_entry)
        session1.close()

        # Session 2: fresh process simulated by a brand-new ChannelCache
        # at the same directory.
        session2 = ChannelCache(disk_path=path)
        try:
            assert "chan-1" in session2
            got = session2.get("chan-1")
            assert got is not None
            pd.testing.assert_frame_equal(got.data, original_entry.data)
            assert got.start_time == original_entry.start_time
            assert got.end_time == original_entry.end_time
        finally:
            session2.close()

    def test_repeated_put_overwrites(self, tmp_path) -> None:
        """A second ``put`` on the same key replaces the prior entry."""
        cache = ChannelCache(disk_path=tmp_path / "overwrite")
        try:
            small = _entry(rows=10)
            bigger = _entry(rows=100)
            cache.put("chan", small)
            cache.put("chan", bigger)
            got = cache.get("chan")
            assert got is not None
            pd.testing.assert_frame_equal(got.data, bigger.data)
        finally:
            cache.close()

    def test_invalidate_removes_entry(self, tmp_path) -> None:
        """``invalidate`` drops the entry; safe to call when absent."""
        cache = ChannelCache(disk_path=tmp_path / "inval")
        try:
            cache.invalidate("never_added")  # safe before any puts
            cache.put("chan-1", _entry(rows=4))
            cache.invalidate("chan-1")
            assert "chan-1" not in cache
            assert cache.get("chan-1") is None
        finally:
            cache.close()

    def test_clear_wipes_disk(self, tmp_path) -> None:
        cache = ChannelCache(disk_path=tmp_path / "clear")
        try:
            cache.put("chan-1", _entry(rows=4))
            cache.put("chan-2", _entry(rows=4))
            cache.clear()
            assert "chan-1" not in cache
            assert "chan-2" not in cache
        finally:
            cache.close()

    def test_disable_disk_closes_handle(self, tmp_path) -> None:
        """Turning off disk closes the handle and silences subsequent ops."""
        cache = ChannelCache(disk_path=tmp_path / "disable")
        try:
            cache.put("chan-1", _entry(rows=4))
            cache.disable_disk()
            assert not cache.disk_enabled
            assert cache.disk_path is None
            assert "chan-1" not in cache  # no handle → no hits
            assert cache.get("chan-1") is None
            # Subsequent puts are silently dropped.
            cache.put("chan-2", _entry(rows=4))
            assert "chan-2" not in cache
        finally:
            cache.close()

    def test_enable_disk_reconfigures_path(self, tmp_path) -> None:
        """Reconfiguring to a different path closes the old handle.

        The new directory starts empty: ``chan-1`` lived in the old
        directory's diskcache, so the lookup at the new path misses.
        """
        cache = ChannelCache(disk_path=tmp_path / "a")
        try:
            cache.put("chan-1", _entry(rows=4))
            cache.enable_disk(path=tmp_path / "b")
            assert cache.disk_path == str(tmp_path / "b")
            assert "chan-1" not in cache  # fresh directory
        finally:
            cache.close()

    def test_enable_disk_noop_when_same_settings(self, tmp_path) -> None:
        """Re-enabling with identical settings doesn't churn the disk handle."""
        cache = ChannelCache(disk_path=tmp_path / "noop")
        try:
            handle_before = cache._disk
            cache.enable_disk(path=tmp_path / "noop", max_bytes=ChannelCache.DEFAULT_DISK_MAX_BYTES)
            assert cache._disk is handle_before
        finally:
            cache.close()

    def test_oversized_entry_skips_cache_preserves_neighbours(self, tmp_path) -> None:
        """An entry larger than the cap is skipped without evicting peers.

        Without this guard, ``diskcache``'s cull would evict every other
        row trying to fit an unfittable entry, then drop the entry itself
        — the wipe-everything failure mode the bounded-cache work
        originally fixed. The disk-tier guard mirrors that fix.

        Memory is sized to accept small entries but reject the oversized one
        so memory-tier writes don't compete with disk-tier writes. We
        assert on the disk ``_disk`` mapping directly because that's where
        the contested behavior lives.

        ``disk_max_bytes`` has to leave room for ``diskcache``'s pickle
        envelope around each small entry (a few KB) AND be small enough
        that the oversized entry trips the guard. Half the oversized
        DataFrame's raw byte size hits both constraints comfortably.
        """
        small = _entry(rows=4)
        oversized = _entry(rows=10_000)
        cache = ChannelCache(
            disk_path=tmp_path / "disk-oversize",
            disk_max_bytes=oversized.size_bytes // 2,
        )
        try:
            cache.put("small-1", small)
            cache.put("small-2", small)
            assert cache._disk is not None
            with _capture_data_warnings() as records:
                cache.put("huge", oversized)
            # Prior entries survive; oversized one was not written.
            assert "small-1" in cache
            assert "small-2" in cache
            assert "huge" not in cache
            assert any("larger than the disk cache cap" in r.getMessage() for r in records)
        finally:
            cache.close()

    def test_oversized_put_drops_prior_entry(self, tmp_path) -> None:
        """An oversized re-insert must drop the prior slice, not silently keep it.

        Otherwise a stale subrange would masquerade as a hit on the next
        ``get`` even though the caller's intent was to refresh the entry.
        """
        small = _entry(rows=4)
        oversized = _entry(rows=10_000)
        cache = ChannelCache(
            disk_path=tmp_path / "drop-prior",
            disk_max_bytes=oversized.size_bytes // 2,
        )
        try:
            cache.put("chan", small)
            assert "chan" in cache
            cache.put("chan", oversized)
            assert "chan" not in cache
        finally:
            cache.close()

    def test_oversized_put_warns_once_per_channel(self, tmp_path) -> None:
        """Repeated oversized puts for the same channel log once, not on every call.

        Without dedup, every ``get_data`` for an oversized channel would
        write a fresh WARNING line — quickly drowning out other signal in
        the logs.
        """
        oversized = _entry(rows=10_000)
        cache = ChannelCache(
            disk_path=tmp_path / "dedup",
            disk_max_bytes=oversized.size_bytes // 2,
        )
        try:
            with _capture_data_warnings() as records:
                for _ in range(5):
                    cache.put("chan", oversized)
            warnings = [r for r in records if "larger than the disk cache cap" in r.getMessage()]
            assert len(warnings) == 1
        finally:
            cache.close()

    def test_oversized_warning_resets_after_normal_put(self, tmp_path) -> None:
        """A successful normal-sized put clears the dedup bit.

        Used by callers who narrow a time window after seeing the warning:
        the next oversized regression should re-warn rather than stay silent.
        """
        small = _entry(rows=4)
        oversized = _entry(rows=10_000)
        cache = ChannelCache(
            disk_path=tmp_path / "reset-after-normal",
            disk_max_bytes=oversized.size_bytes // 2,
        )
        try:
            with _capture_data_warnings() as records:
                cache.put("chan", oversized)  # 1st warning
                cache.put("chan", small)  # resets state
                cache.put("chan", oversized)  # 2nd warning
            warnings = [r for r in records if "larger than the disk cache cap" in r.getMessage()]
            assert len(warnings) == 2
        finally:
            cache.close()

    def test_invalidate_resets_oversized_warning(self, tmp_path) -> None:
        """``invalidate`` is a fresh start; the next oversized put re-warns."""
        oversized = _entry(rows=10_000)
        cache = ChannelCache(
            disk_path=tmp_path / "reset-invalidate",
            disk_max_bytes=oversized.size_bytes // 2,
        )
        try:
            with _capture_data_warnings() as records:
                cache.put("chan", oversized)
                cache.invalidate("chan")
                cache.put("chan", oversized)
            warnings = [r for r in records if "larger than the disk cache cap" in r.getMessage()]
            assert len(warnings) == 2
        finally:
            cache.close()

    def test_clear_resets_oversized_warning(self, tmp_path) -> None:
        """``clear`` resets dedup state across channels."""
        oversized = _entry(rows=10_000)
        cache = ChannelCache(
            disk_path=tmp_path / "reset-clear",
            disk_max_bytes=oversized.size_bytes // 2,
        )
        try:
            with _capture_data_warnings() as records:
                cache.put("chan-a", oversized)
                cache.put("chan-b", oversized)
                cache.clear()
                cache.put("chan-a", oversized)
                cache.put("chan-b", oversized)
            warnings = [r for r in records if "larger than the disk cache cap" in r.getMessage()]
            assert len(warnings) == 4
        finally:
            cache.close()


class TestChannelCacheClearDisk:
    """``ChannelCache.clear_disk`` removes a cache dir, refuses other dirs.

    The classmethod is the source of truth that the resource-level
    ``ChannelsAPIAsync.clear_data_cache_on_disk`` proxies through, so it
    must be defensive against pointing at the wrong directory.
    """

    def test_clear_removes_directory(self, tmp_path) -> None:
        path = tmp_path / "victim"
        cache = ChannelCache(disk_path=path)
        cache.put("chan-1", _entry(rows=4))
        cache.close()
        assert path.exists()
        ChannelCache.clear_disk(path)
        assert not path.exists()

    def test_clear_missing_path_is_noop(self, tmp_path) -> None:
        ChannelCache.clear_disk(tmp_path / "never-existed")  # no raise

    def test_clear_refuses_non_diskcache_directory(self, tmp_path) -> None:
        """A typo'd path with unrelated contents must not be wiped."""
        target = tmp_path / "user-stuff"
        target.mkdir()
        (target / "important.txt").write_text("don't delete me")
        with pytest.raises(ValueError, match="does not look like a sift channel data cache"):
            ChannelCache.clear_disk(target)
        assert (target / "important.txt").read_text() == "don't delete me"

    def test_default_path_constant_under_tmp(self) -> None:
        """Default lives under the OS tmp dir, not a user directory.

        Reads the module-level snapshot captured at import time rather than
        ``ChannelCache.DEFAULT_DISK_PATH`` directly, because the autouse
        ``_isolate_default_disk_cache_path`` fixture monkeypatches that
        attribute for every test to keep ``/tmp`` clean.
        """
        import tempfile

        assert _PRODUCTION_DEFAULT_DISK_PATH.startswith(tempfile.gettempdir())
        assert _PRODUCTION_DEFAULT_DISK_PATH.endswith("sift-channel-data-cache")


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

    def test_no_cache_when_disk_path_omitted(self) -> None:
        """Default construction leaves the cache in no-op mode.

        The ``ChannelsAPIAsync`` resource is the public surface for
        opting into disk persistence; the bare ``DataLowLevelClient``
        keeps caching off so unit tests don't accidentally write to
        ``/tmp`` just by instantiating the wrapper.
        """
        client = DataLowLevelClient(MagicMock())
        assert not client.channel_cache.disk_enabled

    def test_per_instance_isolation(self, tmp_path) -> None:
        """Two clients with separate disk paths must not share cache state.

        Regression test for the original OOM bug: ``channel_cache`` was a
        class attribute, so every ``SiftClient`` in the process appended
        to the same dict. Two fresh clients with distinct directories must
        have independent caches.
        """
        client_a = _client_with_cache(tmp_path, "a")
        client_b = _client_with_cache(tmp_path, "b")
        try:
            client_a.channel_cache.put("c1", _entry(rows=10))
            assert "c1" in client_a.channel_cache
            assert "c1" not in client_b.channel_cache
        finally:
            client_a.channel_cache.close()
            client_b.channel_cache.close()

    def test_disk_cache_kwargs_propagate(self, tmp_path) -> None:
        """Constructor kwargs land on the underlying ``ChannelCache``."""
        path = tmp_path / "kwargs"
        client = DataLowLevelClient(
            MagicMock(),
            disk_cache_path=path,
            disk_cache_max_bytes=8_192,
        )
        try:
            assert client.channel_cache.disk_enabled
            assert client.channel_cache.disk_path == str(path)
            assert client.channel_cache.disk_max_bytes == 8_192
        finally:
            client.channel_cache.close()


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
            client.channel_cache.close()

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
            client.channel_cache.close()

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
            assert "c1" not in client.channel_cache
        finally:
            client.channel_cache.close()
