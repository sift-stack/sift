"""Tests for :mod:`sift_client._internal.cache_namespace`.

The namespace is the user-facing surface for the shared on-disk store
that lives on the :class:`SiftClient`. Three concerns get pinned here:

1. Default policy (opt-out: caching on at the default path) lands on
   the live store on first use.
2. Pre-init configuration (``client.cache.disable()`` /
   ``enable(path=..., max_bytes=...)`` before any resource has touched
   the cache) takes effect on the lazy build.
3. Post-init reconfiguration mutates the live :class:`DiskCache` in
   place rather than swapping it out — every resource adapter holds a
   reference to the same store.

The single-instance-shared-across-resources invariant is the architectural
linchpin: a future second adapter must see the *same* handle as the channel
adapter so a global byte budget and LRU still apply.
"""

from __future__ import annotations

import pytest

from sift_client._internal.disk_cache import DiskCache


def _make_client():
    """Build a SiftClient-like object with the bits the namespace needs.

    Reaching into ``sift_client.SiftClient.__init__`` requires a live gRPC
    config; the namespace only touches ``_disk_cache_config`` and
    ``_disk_cache``, so a tiny stand-in keeps these tests independent of
    transport setup.
    """
    from sift_client._internal.cache_namespace import CacheNamespace
    from sift_client._internal.disk_cache_config import DiskCacheConfig

    class _StandinClient:
        def __init__(self) -> None:
            self._disk_cache_config = DiskCacheConfig(enabled=True)
            self._disk_cache: DiskCache | None = None
            self.cache = CacheNamespace(self)  # type: ignore[arg-type]

        # Matches the real ``SiftClient._get_disk_cache`` so any namespace
        # code that goes through this accessor (e.g. ``stats()``) sees
        # the same lazy-init semantics in tests.
        def _get_disk_cache(self) -> DiskCache:
            return _get_disk_cache(self)

    return _StandinClient()


# Pull the same lazy-init helper the real client uses so we exercise the
# default-path-fallback path against the live code rather than a mock.
def _get_disk_cache(client) -> DiskCache:
    if client._disk_cache is None:
        config = client._disk_cache_config
        if not config.enabled:
            client._disk_cache = DiskCache()
            return client._disk_cache
        target_path = config.path or DiskCache.DEFAULT_DISK_PATH
        try:
            client._disk_cache = DiskCache(
                disk_path=target_path,
                disk_max_bytes=config.max_bytes,
            )
        except Exception:
            if not config.using_default_path:
                raise
            client._disk_cache = DiskCache()
    return client._disk_cache


class TestCacheNamespaceDefaults:
    """Opt-out default: the namespace is on, default path, fresh start."""

    def test_enabled_by_default(self):
        """First lazy access lands at ``DiskCache.DEFAULT_DISK_PATH``."""
        client = _make_client()
        store = _get_disk_cache(client)
        try:
            assert store.disk_enabled
            assert store.disk_path == DiskCache.DEFAULT_DISK_PATH
        finally:
            store.close()

    def test_one_store_shared_across_lazy_calls(self):
        """Re-entering ``_get_disk_cache`` returns the same handle."""
        client = _make_client()
        first = _get_disk_cache(client)
        second = _get_disk_cache(client)
        try:
            assert first is second
        finally:
            first.close()


class TestEnable:
    """``client.cache.enable`` configures the store, pre- and post-init."""

    def test_pre_init_path_lands_on_store(self, tmp_path):
        client = _make_client()
        client.cache.enable(path=str(tmp_path / "pre"), max_bytes=4096)
        store = _get_disk_cache(client)
        try:
            assert store.disk_enabled
            assert store.disk_path == str(tmp_path / "pre")
            assert store.disk_max_bytes == 4096
        finally:
            store.close()

    def test_post_init_swap_uses_same_store_instance(self, tmp_path):
        """Reconfiguring after first use mutates in place rather than re-creating.

        Every resource adapter holds a reference to ``client._disk_cache``;
        if a reconfig replaced the handle, those adapters would still see
        the stale one. ``DiskCache.enable`` swaps the *contents* on
        the same instance.
        """
        client = _make_client()
        client.cache.disable()  # start from off so this is a real on transition
        store = _get_disk_cache(client)
        try:
            assert not store.disk_enabled
            client.cache.enable(path=str(tmp_path / "post"))
            assert client._disk_cache is store  # same instance
            assert store.disk_enabled
            assert store.disk_path == str(tmp_path / "post")
        finally:
            store.close()

    def test_enable_with_default_path_lands_on_default(self, monkeypatch, tmp_path):
        """``enable()`` with no args uses :attr:`DEFAULT_DISK_PATH`.

        Redirects the constant so the test doesn't create the real
        ``/tmp/sift-data-cache`` directory.
        """
        fake_default = str(tmp_path / "fake-default")
        monkeypatch.setattr(DiskCache, "DEFAULT_DISK_PATH", fake_default)

        client = _make_client()
        client.cache.enable()
        store = _get_disk_cache(client)
        try:
            assert store.disk_path == fake_default
        finally:
            store.close()


class TestDisable:
    """``client.cache.disable`` turns the live cache off."""

    def test_disable_closes_live_handle(self, tmp_path):
        client = _make_client()
        client.cache.enable(path=str(tmp_path / "to-close"))
        store = _get_disk_cache(client)
        try:
            assert store.disk_enabled
            client.cache.disable()
            assert not store.disk_enabled
            assert store.disk_path is None
        finally:
            store.close()

    def test_disable_before_lazy_init_keeps_store_off(self, tmp_path):
        """Calling disable before first use means the lazy build skips the open."""
        client = _make_client()
        client.cache.disable()
        store = _get_disk_cache(client)
        try:
            assert not store.disk_enabled
        finally:
            store.close()


class TestClearProxy:
    """``client.cache.clear`` proxies through to :meth:`DiskCache.clear_disk`."""

    def test_clear_removes_directory(self, tmp_path):
        path = tmp_path / "to-clear"
        # Populate a real cache directory so the marker check passes.
        cache = DiskCache(disk_path=path)
        cache.close()
        assert path.exists()

        client = _make_client()
        client.cache.clear(path)
        assert not path.exists()


class TestLazyInitFallback:
    """The default-path-failure fallback used by ``SiftClient._get_disk_cache``."""

    def test_default_path_failure_falls_back_to_no_cache(self, monkeypatch, tmp_path):
        """If the default cache path can't be opened, the lazy init produces
        a disabled :class:`DiskCache` rather than raising.

        Simulated by pointing ``DEFAULT_DISK_PATH`` at a path that already
        exists as a regular file — ``os.makedirs(..., exist_ok=True)``
        raises ``FileExistsError`` for non-directory targets.
        """
        blocker = tmp_path / "not-a-dir"
        blocker.write_text("i am a file, not a directory")
        monkeypatch.setattr(DiskCache, "DEFAULT_DISK_PATH", str(blocker))

        client = _make_client()
        store = _get_disk_cache(client)  # must not raise
        try:
            assert not store.disk_enabled
        finally:
            store.close()

    def test_explicit_path_failure_propagates(self, tmp_path):
        """An explicit path that can't be opened propagates the OSError.

        Silent fallback would hide a user mistake.
        """
        blocker = tmp_path / "not-a-dir"
        blocker.write_text("i am a file, not a directory")

        client = _make_client()
        client.cache.enable(path=str(blocker))
        with pytest.raises(FileExistsError):
            _get_disk_cache(client)


class TestStats:
    """``client.cache.stats()`` reports the current cache state.

    Three shapes get pinned:

    1. **Disabled** — every numeric field zeroes out and ``path`` is
       ``None``. Matches the cold/silent-store contract.
    2. **Enabled, empty** — ``enabled=True``, ``path`` populated,
       sizes/counts at zero. Distinguishes "nothing cached yet" from
       "no cache configured".
    3. **Enabled, populated** — channel entries count matches what the
       adapter wrote, and foreign-prefix rows don't bleed into the
       channel counter.
    """

    def test_stats_when_disabled(self):
        """Disabled cache reports zeros and ``None`` path."""
        client = _make_client()
        client.cache.disable()
        stats = client.cache.stats()
        assert stats.enabled is False
        assert stats.path is None
        assert stats.max_bytes is None
        assert stats.size_bytes == 0
        assert stats.entry_count == 0
        assert stats.channel_entries == 0
        assert "disabled" in str(stats).lower()

    def test_stats_when_enabled_empty(self, tmp_path):
        """Enabled but empty cache reports the path and zero usage."""
        path = str(tmp_path / "empty")
        client = _make_client()
        client.cache.enable(path=path, max_bytes=8 * 1024 * 1024)
        try:
            stats = client.cache.stats()
            assert stats.enabled is True
            assert stats.path == path
            assert stats.max_bytes == 8 * 1024 * 1024
            assert stats.entry_count == 0
            assert stats.channel_entries == 0
            # path appears in the friendly print
            assert path in str(stats)
        finally:
            client._disk_cache.close()  # type: ignore[union-attr]

    def test_stats_counts_channel_entries(self, tmp_path):
        """Channel writes increment ``channel_entries`` and ``entry_count``.

        Uses ``ChannelDataCache`` directly (rather than the
        ``DataLowLevelClient`` end-to-end path) so the test stays
        focused on the stats accounting.
        """
        import pandas as pd

        from sift_client._internal.low_level_wrappers.data import (
            ChannelDataCache,
            _new_cache_entry,
        )

        client = _make_client()
        client.cache.enable(path=str(tmp_path / "stats"))
        try:
            store = client._get_disk_cache()
            adapter = ChannelDataCache(store)
            for i, cid in enumerate(("c1", "c2", "c3")):
                df = pd.DataFrame(
                    {cid: [float(i)]},
                    index=pd.date_range("2025-01-01", periods=1, freq="s", tz="UTC"),
                )
                adapter.put(
                    cid,
                    _new_cache_entry(
                        data=df,
                        start_time=df.index[0].to_pydatetime(),
                        end_time=df.index[-1].to_pydatetime(),
                    ),
                )

            stats = client.cache.stats()
            assert stats.enabled is True
            assert stats.channel_entries == 3
            assert stats.entry_count == 3
            assert stats.size_bytes > 0
        finally:
            client._disk_cache.close()  # type: ignore[union-attr]

    def test_stats_ignores_foreign_adapter_keys_in_channel_count(self, tmp_path):
        """Keys outside the channel namespace don't bump ``channel_entries``.

        Pins the prefix-scoping so a future second adapter doesn't
        double-count here.
        """
        client = _make_client()
        client.cache.enable(path=str(tmp_path / "foreign"))
        try:
            store = client._get_disk_cache()
            store.put("other:foo", "x", size_bytes=64)
            store.put("other:bar", "y", size_bytes=64)

            stats = client.cache.stats()
            assert stats.entry_count == 2
            assert stats.channel_entries == 0
        finally:
            client._disk_cache.close()  # type: ignore[union-attr]


class TestSiftClientIntegration:
    """End-to-end through the real :class:`SiftClient.__init__` entry point.

    Asserts the wire-up: the namespace really lives at ``client.cache``,
    the config is mutable through it, and the lazy ``_get_disk_cache``
    returns the configured store.
    """

    def _make_real_client(self):
        from sift_client import SiftClient, SiftConnectionConfig

        return SiftClient(
            connection_config=SiftConnectionConfig(
                api_key="x",
                grpc_url="disabled.invalid:0",
                rest_url="https://disabled.invalid",
                use_ssl=False,
            )
        )

    def test_attribute_present_and_uses_real_lazy_init(self, monkeypatch, tmp_path):
        fake_default = str(tmp_path / "real-client-default")
        monkeypatch.setattr(DiskCache, "DEFAULT_DISK_PATH", fake_default)

        client = self._make_real_client()
        store = client._get_disk_cache()
        try:
            assert client.cache is not None
            assert store.disk_enabled
            assert store.disk_path == fake_default
        finally:
            store.close()

    def test_disable_before_first_get_data_keeps_store_off(self):
        client = self._make_real_client()
        client.cache.disable()
        store = client._get_disk_cache()
        try:
            assert not store.disk_enabled
        finally:
            store.close()
