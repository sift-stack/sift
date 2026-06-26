"""Tests for :mod:`sift_client._internal.disk_cache`.

Two classes, narrowest scope first:

* :class:`TestDiskCache` — direct unit tests on :class:`DiskCache`:
  the disabled-when-no-path no-op, fresh writes/reads, cross-session
  reload, oversize guard + dedup keyed on the full namespaced key, and
  the marker-guarded :meth:`DiskCache.clear_disk` classmethod.
* :class:`TestClearDisk` — the classmethod's defensive guards.

The store is intentionally key/value-agnostic — every test treats it as
a plain ``str``-keyed dict that happens to persist across handles, with
``size_bytes`` supplied by the caller. The channel-specific adapter
(:class:`ChannelDataCache`) is exercised separately in ``test_data.py``.
"""

from __future__ import annotations

import logging
from contextlib import contextmanager
from typing import Iterator

import pytest

from sift_client._internal.disk_cache import DiskCache

# Snapshot of the production constant captured at import time. The autouse
# ``_isolate_default_disk_cache_path`` fixture in ``conftest.py`` overrides
# the class attribute per test; the constant-shape test still needs the
# real value to assert against.
_PRODUCTION_DEFAULT_DISK_PATH = DiskCache.DEFAULT_DISK_PATH


@contextmanager
def _capture_disk_cache_warnings() -> Iterator[list[logging.LogRecord]]:
    """Capture warnings emitted by the disk-cache logger directly.

    Pytest's ``caplog`` reads from the root logger, but the Sift pytest
    plugin sets ``propagate=False`` on the ``sift_client`` logger when
    audit logging is active, so records emitted from any descendant don't
    reach the root. Attaching a list-backed handler at the leaf logger
    bypasses that.
    """
    target = logging.getLogger("sift_client._internal.disk_cache")
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


class TestDiskCache:
    """End-to-end behaviour of the shared on-disk store."""

    def test_disabled_when_no_path(self) -> None:
        """``DiskCache()`` with no ``disk_path`` is a silent no-op."""
        cache = DiskCache()
        assert cache.disk_enabled is False
        assert cache.disk_path is None
        assert cache.disk_max_bytes is None
        # Every operation no-ops; no AttributeError, no warning.
        cache.put("k", "v", size_bytes=4)
        assert "k" not in cache
        assert cache.get("k") is None
        assert list(iter(cache)) == []
        cache.invalidate("k")
        cache.clear()
        cache.close()

    def test_fresh_cache_writes_and_reads(self, tmp_path) -> None:
        """A fresh disk directory accepts writes and serves them back."""
        cache = DiskCache(disk_path=tmp_path / "fresh")
        try:
            assert cache.disk_enabled
            assert cache.disk_path == str(tmp_path / "fresh")
            assert cache.disk_max_bytes == DiskCache.DEFAULT_DISK_MAX_BYTES
            cache.put("k", {"hello": "world"}, size_bytes=64)
            assert "k" in cache
            assert cache.get("k") == {"hello": "world"}
        finally:
            cache.close()

    def test_reopen_existing_dir_sees_prior_session_entries(self, tmp_path) -> None:
        """Closing then reopening at the same path surfaces prior entries.

        This is the cold-start reuse guarantee: a fresh process pointing
        at a populated directory reads back what an earlier process wrote.
        """
        path = tmp_path / "prev-session"
        session1 = DiskCache(disk_path=path)
        session1.put("k", [1, 2, 3], size_bytes=24)
        session1.close()

        session2 = DiskCache(disk_path=path)
        try:
            assert "k" in session2
            assert session2.get("k") == [1, 2, 3]
        finally:
            session2.close()

    def test_repeated_put_overwrites(self, tmp_path) -> None:
        cache = DiskCache(disk_path=tmp_path / "overwrite")
        try:
            cache.put("k", "first", size_bytes=8)
            cache.put("k", "second", size_bytes=8)
            assert cache.get("k") == "second"
        finally:
            cache.close()

    def test_invalidate_removes_entry(self, tmp_path) -> None:
        cache = DiskCache(disk_path=tmp_path / "inval")
        try:
            cache.invalidate("never_added")  # safe before any puts
            cache.put("k", "v", size_bytes=4)
            cache.invalidate("k")
            assert "k" not in cache
            assert cache.get("k") is None
        finally:
            cache.close()

    def test_clear_wipes_store(self, tmp_path) -> None:
        cache = DiskCache(disk_path=tmp_path / "clear")
        try:
            cache.put("a", 1, size_bytes=8)
            cache.put("b", 2, size_bytes=8)
            cache.clear()
            assert "a" not in cache
            assert "b" not in cache
        finally:
            cache.close()

    def test_iter_yields_keys(self, tmp_path) -> None:
        """``__iter__`` exposes the keyspace so adapters can prefix-clear."""
        cache = DiskCache(disk_path=tmp_path / "iter")
        try:
            cache.put("alpha:1", 1, size_bytes=8)
            cache.put("beta:1", 2, size_bytes=8)
            cache.put("alpha:2", 3, size_bytes=8)
            assert set(cache) == {"alpha:1", "alpha:2", "beta:1"}
        finally:
            cache.close()

    def test_disable_disk_closes_handle(self, tmp_path) -> None:
        """Turning off disk closes the handle and silences subsequent ops."""
        cache = DiskCache(disk_path=tmp_path / "disable")
        try:
            cache.put("k", "v", size_bytes=4)
            cache.disable_disk()
            assert not cache.disk_enabled
            assert cache.disk_path is None
            assert "k" not in cache
            assert cache.get("k") is None
            cache.put("new", "x", size_bytes=4)  # silently dropped
            assert "new" not in cache
        finally:
            cache.close()

    def test_enable_disk_reconfigures_path(self, tmp_path) -> None:
        """Reconfiguring to a different path closes the old handle.

        The new directory starts empty: ``k`` lived in the old directory
        so the lookup at the new path misses.
        """
        cache = DiskCache(disk_path=tmp_path / "a")
        try:
            cache.put("k", "v", size_bytes=4)
            cache.enable_disk(path=tmp_path / "b")
            assert cache.disk_path == str(tmp_path / "b")
            assert "k" not in cache
        finally:
            cache.close()

    def test_enable_disk_noop_when_same_settings(self, tmp_path) -> None:
        """Re-enabling with identical settings doesn't churn the disk handle."""
        cache = DiskCache(disk_path=tmp_path / "noop")
        try:
            handle_before = cache._disk
            cache.enable_disk(
                path=tmp_path / "noop", max_bytes=DiskCache.DEFAULT_DISK_MAX_BYTES
            )
            assert cache._disk is handle_before
        finally:
            cache.close()

    def test_oversized_entry_skipped_and_preserves_neighbours(self, tmp_path) -> None:
        """An entry larger than the cap is skipped without evicting peers.

        Without this guard, ``diskcache``'s cull would evict every other
        row trying to fit an unfittable entry, then drop the entry itself
        — the wipe-everything failure mode the cache work originally fixed.

        Cap is sized to leave plenty of room for diskcache's pickle
        envelope around the small entries while still being small enough
        that the declared oversized ``size_bytes`` (10 MB) trips the
        guard. ``size_bytes`` is the caller's contract — the store
        compares that, not the actual on-disk size.
        """
        cap = 1 * 1024 * 1024  # 1 MiB
        cache = DiskCache(disk_path=tmp_path / "oversize", disk_max_bytes=cap)
        try:
            cache.put("small-1", "value", size_bytes=64)
            cache.put("small-2", "value", size_bytes=64)
            with _capture_disk_cache_warnings() as records:
                cache.put("huge", "value", size_bytes=10 * 1024 * 1024)
            assert "small-1" in cache
            assert "small-2" in cache
            assert "huge" not in cache
            assert any("larger than the disk cache cap" in r.getMessage() for r in records)
        finally:
            cache.close()

    def test_oversized_put_drops_prior_entry(self, tmp_path) -> None:
        """An oversized re-insert must drop the prior value, not silently keep it."""
        cap = 1 * 1024 * 1024
        cache = DiskCache(disk_path=tmp_path / "drop-prior", disk_max_bytes=cap)
        try:
            cache.put("k", "small", size_bytes=64)
            assert "k" in cache
            cache.put("k", "big", size_bytes=10 * 1024 * 1024)
            assert "k" not in cache
        finally:
            cache.close()

    def test_oversized_put_warns_once_per_key(self, tmp_path) -> None:
        """Repeated oversized puts for the same key log once, not every call."""
        cap = 1 * 1024 * 1024
        cache = DiskCache(disk_path=tmp_path / "dedup", disk_max_bytes=cap)
        try:
            with _capture_disk_cache_warnings() as records:
                for _ in range(5):
                    cache.put("k", "v", size_bytes=10 * 1024 * 1024)
            warnings = [r for r in records if "larger than the disk cache cap" in r.getMessage()]
            assert len(warnings) == 1
        finally:
            cache.close()

    def test_oversized_warning_resets_after_normal_put(self, tmp_path) -> None:
        """A successful normal-sized put clears the dedup bit for that key."""
        cap = 1 * 1024 * 1024
        cache = DiskCache(disk_path=tmp_path / "reset-normal", disk_max_bytes=cap)
        try:
            with _capture_disk_cache_warnings() as records:
                cache.put("k", "v", size_bytes=10 * 1024 * 1024)  # 1st warning
                cache.put("k", "v", size_bytes=64)  # resets state
                cache.put("k", "v", size_bytes=10 * 1024 * 1024)  # 2nd warning
            warnings = [r for r in records if "larger than the disk cache cap" in r.getMessage()]
            assert len(warnings) == 2
        finally:
            cache.close()

    def test_dedup_keys_on_full_namespaced_key(self, tmp_path) -> None:
        """Dedup is per-key, so two adapters' colliding bare ids don't share state.

        Pins the design choice that the oversize warning dedup tracks the
        full namespaced key handed to ``put`` (e.g. ``channel:foo`` vs
        ``calc:foo``) rather than collapsing on the bare id. Two different
        prefixes for the same suffix each get their own one-shot warning.
        """
        cap = 1 * 1024 * 1024
        cache = DiskCache(disk_path=tmp_path / "two-prefixes", disk_max_bytes=cap)
        try:
            with _capture_disk_cache_warnings() as records:
                cache.put("alpha:foo", "v", size_bytes=10 * 1024 * 1024)
                cache.put("beta:foo", "v", size_bytes=10 * 1024 * 1024)
            warnings = [r for r in records if "larger than the disk cache cap" in r.getMessage()]
            assert len(warnings) == 2
            messages = [r.getMessage() for r in warnings]
            assert any("alpha:foo" in m for m in messages)
            assert any("beta:foo" in m for m in messages)
        finally:
            cache.close()

    def test_invalidate_resets_oversized_warning(self, tmp_path) -> None:
        cap = 1 * 1024 * 1024
        cache = DiskCache(disk_path=tmp_path / "reset-inval", disk_max_bytes=cap)
        try:
            with _capture_disk_cache_warnings() as records:
                cache.put("k", "v", size_bytes=10 * 1024 * 1024)
                cache.invalidate("k")
                cache.put("k", "v", size_bytes=10 * 1024 * 1024)
            warnings = [r for r in records if "larger than the disk cache cap" in r.getMessage()]
            assert len(warnings) == 2
        finally:
            cache.close()

    def test_clear_resets_oversized_warning(self, tmp_path) -> None:
        cap = 1 * 1024 * 1024
        cache = DiskCache(disk_path=tmp_path / "reset-clear", disk_max_bytes=cap)
        try:
            with _capture_disk_cache_warnings() as records:
                cache.put("a", "v", size_bytes=10 * 1024 * 1024)
                cache.put("b", "v", size_bytes=10 * 1024 * 1024)
                cache.clear()
                cache.put("a", "v", size_bytes=10 * 1024 * 1024)
                cache.put("b", "v", size_bytes=10 * 1024 * 1024)
            warnings = [r for r in records if "larger than the disk cache cap" in r.getMessage()]
            assert len(warnings) == 4
        finally:
            cache.close()


class TestClearDisk:
    """:meth:`DiskCache.clear_disk` removes a cache dir, refuses other dirs."""

    def test_clear_removes_directory(self, tmp_path) -> None:
        path = tmp_path / "victim"
        cache = DiskCache(disk_path=path)
        cache.put("k", "v", size_bytes=4)
        cache.close()
        assert path.exists()
        DiskCache.clear_disk(path)
        assert not path.exists()

    def test_clear_missing_path_is_noop(self, tmp_path) -> None:
        DiskCache.clear_disk(tmp_path / "never-existed")  # no raise

    def test_clear_refuses_non_diskcache_directory(self, tmp_path) -> None:
        """A typo'd path with unrelated contents must not be wiped."""
        target = tmp_path / "user-stuff"
        target.mkdir()
        (target / "important.txt").write_text("don't delete me")
        with pytest.raises(ValueError, match="does not look like a sift data cache"):
            DiskCache.clear_disk(target)
        assert (target / "important.txt").read_text() == "don't delete me"

    def test_default_path_constant_under_tmp(self) -> None:
        """Default lives under the OS tmp dir, not a user directory.

        Reads the module-level snapshot rather than ``DEFAULT_DISK_PATH``
        directly because the autouse fixture monkeypatches that attribute
        for every test.
        """
        import tempfile

        assert _PRODUCTION_DEFAULT_DISK_PATH.startswith(tempfile.gettempdir())
        assert _PRODUCTION_DEFAULT_DISK_PATH.endswith("sift-data-cache")
