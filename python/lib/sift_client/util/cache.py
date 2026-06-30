"""User-facing surface for the shared on-disk cache.

This module hosts the small bag of methods exposed as ``client.cache``,
plus the :class:`CacheStats` snapshot type returned by
:meth:`CacheNamespace.stats`. Both classes are public — they're
re-exported from the top-level ``sift_client`` package and surfaced in
the generated API docs.

The cache itself (a :class:`~sift_client._internal.disk_cache.DiskCache`)
lives on :class:`~sift_client.client.SiftClient` so every resource that
wants to persist results across calls can reach into one shared store.
The namespace deliberately mirrors :class:`DiskCache` rather than the
old per-resource API (``client.channels.enable_data_cache_disk(...)``):
since the store is shared, configuration is global.
"""

from __future__ import annotations

import logging
from dataclasses import dataclass
from typing import TYPE_CHECKING

from sift_client._internal.disk_cache import DiskCache

if TYPE_CHECKING:
    import os

    from sift_client.client import SiftClient

logger = logging.getLogger(__name__)


_BYTE_UNITS = (
    (1024**4, "TiB"),
    (1024**3, "GiB"),
    (1024**2, "MiB"),
    (1024, "KiB"),
)


def _format_bytes(n: int) -> str:
    """Render ``n`` bytes in the largest unit that doesn't underflow to zero."""
    for threshold, suffix in _BYTE_UNITS:
        if n >= threshold:
            return f"{n / threshold:.1f} {suffix}"
    return f"{n} B"


@dataclass(frozen=True)
class CacheStats:
    """Snapshot of the shared on-disk cache at call time.

    Returned by :meth:`CacheNamespace.stats`. Frozen dataclass so it
    plays well with logging, snapshot tests, and "compare two readings"
    diagnostics without surprise mutation.

    Field semantics:

    * **enabled** — whether the disk handle is open. When ``False``, all
      the size/count fields are zero regardless of on-disk state.
    * **path** — directory the cache is open against, or ``None`` when
      disabled. Useful for "where does this cache actually live?".
    * **max_bytes** — configured byte cap on disk usage, or ``None``
      when disabled. ``diskcache``'s LRU evicts once usage approaches
      this.
    * **size_bytes** — current on-disk usage including SQLite overhead.
      Tends to trend slightly higher than the sum of per-entry
      ``size_bytes`` the resources hand to the store.
    * **entry_count** — total cache keys across all adapter prefixes
      (channel index + segment rows, plus any future foreign-adapter
      rows).
    * **channel_buckets** — number of distinct
      ``(channel_id, run_id)`` channel buckets currently cached.
      Counted by walking the channel adapter's index keys; one index
      per bucket regardless of how many segments live under it.
    * **channel_segments** — total cached segment frames across every
      bucket. Each ``put_segment`` call adds one; ``diskcache`` LRU
      evictions can drop them independently of their parent index, so
      this can be lower than the sum of segments the adapter has
      written over the session.

    ``str(stats)`` prints a multi-line summary suitable for
    notebook/REPL display; the structured fields are available for
    programmatic checks.
    """

    enabled: bool
    path: str | None
    max_bytes: int | None
    size_bytes: int
    entry_count: int
    channel_buckets: int
    channel_segments: int

    def __str__(self) -> str:
        if not self.enabled:
            return "Sift cache: disabled"
        cap = _format_bytes(self.max_bytes) if self.max_bytes is not None else "no cap"
        pct = f" ({self.size_bytes / self.max_bytes * 100:.1f}%)" if self.max_bytes else ""
        return (
            "Sift cache:\n"
            f"  path:     {self.path}\n"
            f"  used:     {_format_bytes(self.size_bytes)} / {cap}{pct}\n"
            f"  channels: {self.channel_buckets} buckets, "
            f"{self.channel_segments} segments\n"
            f"  entries:  {self.entry_count} total"
        )


class CacheNamespace:
    """Resource-agnostic surface for the on-disk cache shared by all resources.

    Exposed as ``client.cache``. The actual handle
    (:class:`~sift_client._internal.disk_cache.DiskCache`) is constructed
    lazily on first use so importing :mod:`sift_client` doesn't pay the
    diskcache cost up front. Configuration changes made before that
    first use are recorded against the client's
    :class:`~sift_client._internal.disk_cache_config.DiskCacheConfig`
    and applied when the store opens; changes after first use are
    routed directly to the live store.

    Default policy: disk caching is **opt-out** (the config is
    constructed with ``enabled=True``). Users who don't want any state
    on disk call :meth:`disable` to silence it; users who want a custom
    location or byte cap call :meth:`enable` with arguments.
    """

    def __init__(self, client: SiftClient):
        """Bind this namespace to ``client``. Constructed by :class:`SiftClient`."""
        self._client = client

    def enable(
        self,
        *,
        path: str | os.PathLike[str] | None = None,
        max_bytes: int | None = None,
    ) -> None:
        """Enable (or reconfigure) on-disk caching.

        Disk caching is **on by default** at
        :attr:`~sift_client._internal.disk_cache.DiskCache.DEFAULT_DISK_PATH`;
        use this method to override the path or size, or to turn the
        cache back on after a prior :meth:`disable` call.

        Reconfiguring a live cache (different ``path`` or ``max_bytes``)
        closes the previous handle and opens a new one. Existing entries
        at the new path become available as cache hits.

        An explicit ``path`` that can't be opened (permission denied,
        read-only filesystem, ...) raises so the caller knows their
        request didn't take. The default-path open does *not* raise —
        see :meth:`SiftClient._get_disk_cache` for the silent fall-back.

        Args:
            path: Directory to persist to. ``None`` (the default) uses
                the cache's :attr:`DEFAULT_DISK_PATH`.
            max_bytes: Byte cap on disk usage. ``None`` uses the cache's
                :attr:`DEFAULT_DISK_MAX_BYTES` (4 GiB). When the bound
                is reached, ``diskcache``'s LRU eviction takes over.

        Example:
            client.cache.enable(path="/data/sift-cache")
            client.cache.enable(max_bytes=1024 ** 3)  # 1 GiB
        """
        client = self._client
        client._disk_cache_config.enable(path=path, max_bytes=max_bytes)
        if client._disk_cache is not None:
            client._disk_cache.enable(path=path, max_bytes=max_bytes)

    def disable(self) -> None:
        """Opt out of on-disk caching (no reads or writes).

        Caching is on by default; call this when you don't want any
        cached data written to or read from disk. Closes any open cache
        file handle. The on-disk directory is NOT deleted — use
        :meth:`clear` to wipe it.
        """
        client = self._client
        client._disk_cache_config.disable()
        if client._disk_cache is not None:
            client._disk_cache.disable()

    def stats(self) -> CacheStats:
        """Return a snapshot of the current cache state.

        Calling this triggers the lazy disk-handle open if it hasn't
        happened yet (so a stats call on a fresh client doesn't
        mis-report as disabled just because no resource has touched
        the store). After the open it's read-only — no mutation or
        migration.

        Channel-specific counts come from walking the keyspace once
        and counting keys under the channel adapter's namespace prefix.
        If a second cache-aware resource grows its own key prefix
        later, extend this method to surface per-adapter counts; for
        now there's only one adapter so the dedicated field stays flat.

        Example:
            >>> print(client.cache.stats())
            Sift cache:
              path:     /tmp/sift-data-cache
              used:     142.3 MiB / 4.0 GiB (3.5%)
              channels: 12 buckets, 487 segments
              entries:  499 total
        """
        # Importing here keeps this module light at import time — pulling
        # the adapter pulls pandas, which is the whole reason
        # ``_ensure_data_low_level_client`` is lazy too.
        from sift_client._internal.low_level_wrappers.data import ChannelDataCache

        # ``_get_disk_cache`` is idempotent: it opens the store on first
        # use and memoizes, so calling it here for a stats peek is the
        # same cost as the first resource call would have paid.
        # Without this, a fresh client that hasn't yet used a cache-
        # aware resource would mis-report as disabled.
        store = self._client._get_disk_cache()
        if not store.disk_enabled:
            return CacheStats(
                enabled=False,
                path=None,
                max_bytes=None,
                size_bytes=0,
                entry_count=0,
                channel_buckets=0,
                channel_segments=0,
            )

        # One pass over the keyspace. Cheap — diskcache keys are SQLite
        # rows, and we only touch metadata (no value loads).
        # Bucket index keys end in ``:idx`` and segment keys carry
        # ``:seg:`` inside the suffix; those two shapes partition the
        # channel adapter's namespace cleanly.
        channel_prefix = ChannelDataCache.KEY_PREFIX
        channel_buckets = 0
        channel_segments = 0
        for key in store:
            if not key.startswith(channel_prefix):
                continue
            if key.endswith(":idx"):
                channel_buckets += 1
            elif ":seg:" in key:
                channel_segments += 1
        return CacheStats(
            enabled=True,
            path=store.disk_path,
            max_bytes=store.disk_max_bytes,
            size_bytes=store.volume(),
            entry_count=len(store),
            channel_buckets=channel_buckets,
            channel_segments=channel_segments,
        )

    def clear(self, path: str | os.PathLike[str] | None = None) -> None:
        """Delete a previously-persisted on-disk cache directory.

        Drops stale caches from previous sessions, recovers from a
        corrupt cache, or reclaims disk space. Removes the directory
        entirely; if disk caching is on, the next access re-opens an
        empty cache at the same path.

        Args:
            path: Directory of the cache to clear. ``None`` (the default)
                targets the cache's
                :attr:`~sift_client._internal.disk_cache.DiskCache.DEFAULT_DISK_PATH`.

        Raises:
            ValueError: If ``path`` exists but does not look like a sift
                data cache directory.
        """
        DiskCache.clear_disk(path)


__all__ = ["CacheNamespace", "CacheStats"]
