"""User-facing surface for the shared on-disk cache.

This module hosts the small bag of methods exposed as ``client.cache``.
The cache itself (a :class:`~sift_client._internal.disk_cache.DiskCache`)
lives on :class:`~sift_client.client.SiftClient` so every resource that
wants to persist results across calls can reach into one shared store.

The namespace deliberately mirrors :class:`DiskCache` rather than the
old per-resource API (``client.channels.enable_data_cache_disk(...)``):
since the store is shared, configuration is global.
"""

from __future__ import annotations

import logging
from typing import TYPE_CHECKING

from sift_client._internal.disk_cache import DiskCache

if TYPE_CHECKING:
    import os

    from sift_client.client import SiftClient

logger = logging.getLogger(__name__)


class CacheNamespace:
    """Resource-agnostic surface for the on-disk cache shared by all resources.

    Exposed as ``client.cache``. The actual handle (:class:`DiskCache`) is
    constructed lazily on first use so importing :mod:`sift_client` doesn't
    pay the diskcache cost up front. Configuration changes made before
    that first use are recorded against the
    :class:`~sift_client._internal.disk_cache_config.DiskCacheConfig` on the
    client and applied when the store opens; changes after first use are
    routed directly to the live :class:`DiskCache`.

    Default policy: disk caching is **opt-out** (the ``DiskCacheConfig`` is
    constructed with ``enabled=True``). Users who don't want any state on
    disk call :meth:`disable_disk` to silence it; users who want a custom
    location or byte cap call :meth:`enable_disk` with arguments.
    """

    def __init__(self, client: SiftClient):
        self._client = client

    def enable_disk(
        self,
        *,
        path: str | os.PathLike[str] | None = None,
        max_bytes: int | None = None,
    ) -> None:
        """Enable (or reconfigure) on-disk caching.

        Disk caching is **on by default** at :attr:`DiskCache.DEFAULT_DISK_PATH`;
        use this method to override the path or size, or to turn the cache
        back on after a prior :meth:`disable_disk` call.

        Reconfiguring a live cache (different ``path`` or ``max_bytes``)
        closes the previous handle and opens a new one. Existing entries
        at the new path become available as cache hits.

        An explicit ``path`` that can't be opened (permission denied,
        read-only filesystem, ...) raises so the caller knows their
        request didn't take. The default-path open does *not* raise — see
        :meth:`SiftClient._get_disk_cache` for the silent fall-back.

        Args:
            path: Directory to persist to. ``None`` (the default) uses
                :attr:`DiskCache.DEFAULT_DISK_PATH`.
            max_bytes: Byte cap on disk usage. ``None`` uses
                :attr:`DiskCache.DEFAULT_DISK_MAX_BYTES` (4 GiB). When the
                bound is reached, ``diskcache``'s LRU eviction takes over.

        Example:
            client.cache.enable_disk(path="/data/sift-cache")
            client.cache.enable_disk(max_bytes=1024 ** 3)  # 1 GiB
        """
        client = self._client
        client._disk_cache_config.enable(path=path, max_bytes=max_bytes)
        if client._disk_cache is not None:
            client._disk_cache.enable_disk(path=path, max_bytes=max_bytes)

    def disable_disk(self) -> None:
        """Opt out of on-disk caching (no reads or writes).

        Caching is on by default; call this when you don't want any
        cached data written to or read from disk. Closes any open cache
        file handle. The on-disk directory is NOT deleted — use
        :meth:`clear_disk` to wipe it.
        """
        client = self._client
        client._disk_cache_config.disable()
        if client._disk_cache is not None:
            client._disk_cache.disable_disk()

    def clear_disk(self, path: str | os.PathLike[str] | None = None) -> None:
        """Delete a previously-persisted on-disk cache directory.

        Drops stale caches from previous sessions, recovers from a
        corrupt cache, or reclaims disk space. Removes the directory
        entirely; if disk caching is on, the next access re-opens an
        empty cache at the same path.

        Args:
            path: Directory of the cache to clear. ``None`` (the default)
                targets :attr:`DiskCache.DEFAULT_DISK_PATH`.

        Raises:
            ValueError: If ``path`` exists but does not look like a sift
                data cache directory.
        """
        DiskCache.clear_disk(path)
