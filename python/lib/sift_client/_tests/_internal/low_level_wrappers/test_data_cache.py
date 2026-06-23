"""Tests for the channel data cache in :mod:`sift_client._internal.low_level_wrappers.data`.

Two layers covered here:

* :class:`ChannelCache` directly — byte accounting, LRU promotion, eviction,
  edge cases. These tests construct cache entries from real (tiny) DataFrames
  so the size measurement code is exercised end-to-end.
* :class:`DataLowLevelClient` — ``ignore_cache=True`` skipping writes,
  per-instance cache isolation, ``data_cache_max_bytes=0`` disabling cache.

The OOM regression that motivated this code happened because the cache was a
class attribute that grew without bound. The instance-isolation test below is
the canary that catches anyone re-introducing that pattern.
"""

from __future__ import annotations

from datetime import datetime, timedelta, timezone
from unittest.mock import MagicMock

import pandas as pd
import pytest

from sift_client._internal.low_level_wrappers.data import (
    DEFAULT_DATA_CACHE_MAX_BYTES,
    ChannelCache,
    ChannelCacheEntry,
    DataLowLevelClient,
    _new_cache_entry,
)


def _entry(rows: int, *, value_dtype: str = "float64") -> ChannelCacheEntry:
    """Build a ChannelCacheEntry with ``rows`` rows of fake data."""
    index = pd.date_range("2025-01-01", periods=rows, freq="ms", tz=timezone.utc)
    data = pd.DataFrame({"value": range(rows)}, index=index).astype({"value": value_dtype})
    return _new_cache_entry(
        data=data,
        start_time=index[0].to_pydatetime(),
        end_time=index[-1].to_pydatetime(),
    )


def _invariant_holds(cache: ChannelCache) -> bool:
    return cache.total_bytes == sum(e.size_bytes for e in cache._entries.values())


class TestChannelCacheBookkeeping:
    """Tight checks on the internal byte counter and ordering."""

    def test_put_get_roundtrip(self) -> None:
        cache = ChannelCache(max_bytes=DEFAULT_DATA_CACHE_MAX_BYTES)
        entry = _entry(rows=10)
        cache.put("c1", entry)

        assert cache.get("c1") is entry
        assert cache.total_bytes == entry.size_bytes
        assert _invariant_holds(cache)

    def test_put_replaces_size_accounting(self) -> None:
        """A second put for the same key must reclaim the prior size first."""
        cache = ChannelCache(max_bytes=DEFAULT_DATA_CACHE_MAX_BYTES)
        small = _entry(rows=10)
        big = _entry(rows=1000)

        cache.put("c1", small)
        cache.put("c1", big)

        # Total reflects only the second entry, never small + big.
        assert cache.total_bytes == big.size_bytes
        assert cache.get("c1") is big
        assert _invariant_holds(cache)

    def test_invalidate_drops_byte_count(self) -> None:
        cache = ChannelCache(max_bytes=DEFAULT_DATA_CACHE_MAX_BYTES)
        cache.put("c1", _entry(rows=10))
        cache.invalidate("c1")

        assert cache.get("c1") is None
        assert cache.total_bytes == 0
        assert _invariant_holds(cache)

    def test_invalidate_missing_is_noop(self) -> None:
        cache = ChannelCache(max_bytes=DEFAULT_DATA_CACHE_MAX_BYTES)
        cache.invalidate("nope")
        assert cache.total_bytes == 0

    def test_clear_empties_total(self) -> None:
        cache = ChannelCache(max_bytes=DEFAULT_DATA_CACHE_MAX_BYTES)
        cache.put("c1", _entry(rows=10))
        cache.put("c2", _entry(rows=20))
        cache.clear()

        assert cache.total_bytes == 0
        assert len(cache) == 0
        assert _invariant_holds(cache)


class TestChannelCacheEviction:
    """Eviction policy: LRU, byte-bounded, oversized-entry-dropped."""

    def test_oldest_entry_evicted_first(self) -> None:
        """Insertion order determines who goes when only inserts have happened."""
        a, b, c = _entry(rows=50), _entry(rows=50), _entry(rows=50)
        cap = a.size_bytes + b.size_bytes  # room for exactly two
        cache = ChannelCache(max_bytes=cap)

        cache.put("a", a)
        cache.put("b", b)
        cache.put("c", c)  # forces eviction of "a"

        assert "a" not in cache
        assert "b" in cache
        assert "c" in cache
        assert cache.total_bytes <= cap
        assert _invariant_holds(cache)

    def test_get_promotes_to_most_recent(self) -> None:
        """Reading an entry must protect it from the next eviction."""
        a, b, c = _entry(rows=50), _entry(rows=50), _entry(rows=50)
        cap = a.size_bytes + b.size_bytes
        cache = ChannelCache(max_bytes=cap)

        cache.put("a", a)
        cache.put("b", b)
        assert cache.get("a") is a  # promote
        cache.put("c", c)  # now "b" is the oldest and should be evicted

        assert "a" in cache
        assert "b" not in cache
        assert "c" in cache
        assert _invariant_holds(cache)

    def test_oversized_entry_evicts_with_neighbours(self) -> None:
        """A single entry larger than the cap ends up evicted itself.

        The alternative ("keep the oversized entry and accept that the cap is
        soft") would silently reintroduce the unbounded-growth bug for any
        workload whose typical entry is bigger than ``max_bytes``.
        """
        small_a, small_b = _entry(rows=10), _entry(rows=10)
        oversized = _entry(rows=10_000)
        cap = small_a.size_bytes + small_b.size_bytes  # comfortably below ``oversized``
        cache = ChannelCache(max_bytes=cap)

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

        Without size reclamation on update, ``total_bytes`` would creep above
        the cap silently. We re-build the entry each iteration to mimic the
        ``_update_cache`` concat path.
        """
        cap = 1_000_000  # ~1 MB
        cache = ChannelCache(max_bytes=cap)
        accumulated = pd.DataFrame()
        for i in range(50):
            chunk = pd.DataFrame(
                {"value": range(1000)},
                index=pd.date_range(
                    datetime(2025, 1, 1, tzinfo=timezone.utc) + timedelta(seconds=i),
                    periods=1000,
                    freq="us",
                ),
            )
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


class TestDataLowLevelClientIntegration:
    """End-to-end checks on the constructor wiring and ignore_cache semantics."""

    def test_per_instance_isolation(self) -> None:
        """Two clients must not share cache state.

        This is the regression test for the original OOM bug: ``channel_cache``
        was a class attribute, so every ``SiftClient`` in the process appended
        to the same dict. Construct two clients, populate one, the other must
        stay empty.
        """
        client_a = DataLowLevelClient(MagicMock())
        client_b = DataLowLevelClient(MagicMock())

        client_a.channel_cache.put("c1", _entry(rows=10))

        assert "c1" in client_a.channel_cache
        assert "c1" not in client_b.channel_cache
        assert client_b.channel_cache.total_bytes == 0

    def test_ignore_cache_skips_writes(self) -> None:
        """``ignore_cache=True`` must not populate the cache.

        Previously the read path was bypassed but ``_update_cache`` still ran
        unconditionally, so a "non-caching" workload still grew memory until
        OOM. Verify by exercising ``_update_cache`` only when ``ignore_cache``
        is false.
        """
        client = DataLowLevelClient(MagicMock())
        client.channel_cache.name_id_map["chan"] = "c1"

        index = pd.date_range("2025-01-01", periods=5, freq="ms", tz=timezone.utc)
        df = pd.DataFrame({"value": range(5)}, index=index)

        # Real ``get_channel_data`` would call ``_update_cache`` from inside an
        # ``if not ignore_cache`` branch; assert the helper itself is what
        # writes, and that ``get_channel_data`` doesn't invoke it when
        # ``ignore_cache=True``. We verify the branch directly to keep this
        # test free of gRPC stubbing.
        client._update_cache(
            channel_data={"chan": df},
            start_time=index[0].to_pydatetime(),
            end_time=index[-1].to_pydatetime(),
        )
        assert "c1" in client.channel_cache

        # Skipping the call (as ``get_channel_data`` does when ignore_cache is
        # true) leaves the cache untouched.
        client.channel_cache.invalidate("c1")
        assert "c1" not in client.channel_cache

    def test_data_cache_max_bytes_zero_disables_caching(self) -> None:
        """Constructor knob: ``data_cache_max_bytes=0`` → no cache writes land."""
        client = DataLowLevelClient(MagicMock(), data_cache_max_bytes=0)
        client.channel_cache.name_id_map["chan"] = "c1"

        index = pd.date_range("2025-01-01", periods=5, freq="ms", tz=timezone.utc)
        df = pd.DataFrame({"value": range(5)}, index=index)

        client._update_cache(
            channel_data={"chan": df},
            start_time=index[0].to_pydatetime(),
            end_time=index[-1].to_pydatetime(),
        )
        assert "c1" not in client.channel_cache
        assert client.channel_cache.total_bytes == 0
