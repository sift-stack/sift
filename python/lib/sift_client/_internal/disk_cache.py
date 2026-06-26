"""Shared on-disk key/value store used by every resource that wants to cache results.

One :class:`DiskCache` instance lives on the :class:`SiftClient` (see
``client._disk_cache``). Resources don't construct their own — they receive
a reference and wrap it in a typed adapter that namespaces keys (e.g.
``ChannelDataCache`` in ``low_level_wrappers/data.py``). The store itself
is deliberately value-agnostic: callers hand in ``size_bytes`` for the
oversize guard, ``diskcache`` pickles whatever object the caller supplied,
and the store never needs to know what's inside.

This module is the sibling of :mod:`._disk_cache_config` — the config
holds user intent (enabled / path / max_bytes) and the store is the live
handle keyed off that intent.

Key behaviours pinned here so the adapter layer can stay thin:

* Default path lives under :func:`tempfile.gettempdir` and is shared
  across processes, so a fresh session reads previously-cached entries.
* The byte cap is one global budget; LRU eviction spans all resources
  sharing the store (channels, calculated channels, exports, ...).
* :meth:`clear_disk` (classmethod) refuses to delete a directory that
  doesn't look like a sift cache (no diskcache marker), so a typo'd
  path can't take out the user's documents.
* Oversized entries are skipped with a one-shot warning per key —
  otherwise diskcache's eviction loop would drain every other row
  trying to fit an unfittable entry.
* Construction with ``disk_path=None`` (or after :meth:`disable_disk`)
  is a silent no-op store. Callers don't need to branch on disabled
  state; reads always miss and writes are dropped.
"""

from __future__ import annotations

import logging
import os
import shutil
import tempfile
from pathlib import Path
from typing import TYPE_CHECKING, Any, Iterator, cast

if TYPE_CHECKING:
    import diskcache

logger = logging.getLogger(__name__)


class DiskCache:
    """Process-wide disk-backed key/value store.

    Wraps a :class:`diskcache.Cache` with the lifecycle management and
    safety rails sift resources rely on. The instance is shared — each
    resource adapter namespaces its keys (e.g. ``channel:<id>``) so multiple
    resources can write to the same store without colliding.

    When ``disk_path`` is ``None``, the instance is a silent no-op: every
    ``get`` misses, every ``put`` is dropped, and ``__contains__`` is
    always ``False``. This lets callers treat "caching disabled" the same
    as a cold cache, with no branching needed at the read/write site.

    Args:
        disk_path: Directory to persist to. ``None`` keeps the store
            disabled. A previously-populated directory is reused, so a
            fresh process reading the same path sees existing entries.
        disk_max_bytes: Byte cap on the store. ``None`` falls back to
            :attr:`DEFAULT_DISK_MAX_BYTES`. Ignored when ``disk_path``
            is ``None``.
    """

    #: Default directory for the shared cache. Lives under
    #: :func:`tempfile.gettempdir` so it survives across sessions of the
    #: same user but doesn't pollute the home directory. The suffix is
    #: fixed so multiple ``SiftClient`` instances naturally share the
    #: same store and pick up each other's prior sessions.
    DEFAULT_DISK_PATH: str = os.path.join(tempfile.gettempdir(), "sift-data-cache")

    #: Default byte cap when :meth:`enable_disk` is called without an
    #: explicit ``max_bytes``. 4 GiB is generous for the typical ``/tmp``
    #: filesystem; ``diskcache`` enforces the cap with its own SQLite-
    #: backed LRU eviction once the bound is reached.
    DEFAULT_DISK_MAX_BYTES: int = 4 * 1024 * 1024 * 1024

    #: Marker file ``diskcache`` writes inside every cache directory. The
    #: classmethod :meth:`clear_disk` checks for this before any
    #: ``shutil.rmtree`` so a typo'd path can't wipe out an unrelated
    #: directory.
    _DISKCACHE_MARKER: str = "cache.db"

    def __init__(
        self,
        *,
        disk_path: str | os.PathLike[str] | None = None,
        disk_max_bytes: int | None = None,
    ):
        # Keys we've already logged an "entry exceeds disk cap" warning
        # for. Tracks the full namespaced key (e.g. ``channel:foo``), not
        # the resource-side id, so two adapters that happen to share an
        # id space don't collide on dedup. A successful normal put
        # clears the bit so a future regression re-warns.
        self._oversized_warned: set[str] = set()
        self._disk: diskcache.Cache | None = None
        self._disk_path: str | None = None
        self._disk_max_bytes: int | None = None
        if disk_path is not None:
            self._open_disk(
                str(disk_path),
                disk_max_bytes if disk_max_bytes is not None else self.DEFAULT_DISK_MAX_BYTES,
            )

    @classmethod
    def clear_disk(cls, path: str | os.PathLike[str] | None = None) -> None:
        """Delete a previously-persisted on-disk cache directory.

        Use this to drop stale caches from previous sessions, recover
        from a corrupt cache, or reclaim disk space. The directory is
        removed entirely; a future :meth:`enable_disk` call at the same
        path opens a fresh empty cache.

        Args:
            path: Directory of the cache to clear. ``None`` (the default)
                targets :attr:`DEFAULT_DISK_PATH`.

        Raises:
            ValueError: If ``path`` exists but does not look like a sift
                cache directory (missing the ``diskcache`` marker file).
                The guard makes accidental misuse a hard error rather
                than silent data loss.
        """
        target = Path(path) if path is not None else Path(cls.DEFAULT_DISK_PATH)
        if not target.exists():
            return
        if not (target / cls._DISKCACHE_MARKER).exists():
            raise ValueError(
                f"{str(target)!r} does not look like a sift data cache "
                f"directory (missing {cls._DISKCACHE_MARKER!r} marker). "
                f"Refusing to delete."
            )
        shutil.rmtree(target)

    @property
    def disk_enabled(self) -> bool:
        """Whether a disk handle is currently open."""
        return self._disk is not None

    @property
    def disk_path(self) -> str | None:
        """Filesystem path of the cache when enabled, else ``None``."""
        return self._disk_path

    @property
    def disk_max_bytes(self) -> int | None:
        """Configured byte cap on disk usage, or ``None`` when disabled."""
        return self._disk_max_bytes

    def __contains__(self, key: str) -> bool:
        """True if ``key`` is cached. Always ``False`` when disabled."""
        if self._disk is None:
            return False
        return key in self._disk

    def __iter__(self) -> Iterator[str]:
        """Yield cached keys. Lets adapters scope a clear to their prefix.

        Yields nothing when disabled. The underlying diskcache iterator
        is snapshot-style, but callers that intend to mutate during
        iteration should still wrap with ``list(...)`` to be safe.

        ``diskcache.Cache`` is typed as yielding ``bytes | str | ...``
        because it supports arbitrary key types; the cast narrows to the
        ``str`` contract this layer enforces. Adapters never write
        non-string keys.
        """
        if self._disk is None:
            return
        for key in self._disk:
            yield cast("str", key)

    def enable_disk(
        self,
        *,
        path: str | os.PathLike[str] | None = None,
        max_bytes: int | None = None,
    ) -> None:
        """Open the disk handle, replacing any previous one.

        Reconfiguring to a different ``path`` or ``max_bytes`` closes the
        prior handle first. Existing entries at the new path become
        available via :meth:`get` without further setup.

        Args:
            path: Directory to persist to. ``None`` uses
                :attr:`DEFAULT_DISK_PATH`.
            max_bytes: Byte cap (``None`` → :attr:`DEFAULT_DISK_MAX_BYTES`).
        """
        target_path = str(path) if path is not None else self.DEFAULT_DISK_PATH
        target_max = max_bytes if max_bytes is not None else self.DEFAULT_DISK_MAX_BYTES
        if (
            self._disk is not None
            and self._disk_path == target_path
            and self._disk_max_bytes == target_max
        ):
            return
        self._close_disk()
        self._open_disk(target_path, target_max)

    def disable_disk(self) -> None:
        """Close the disk handle (if open). Does not touch on-disk contents.

        Use :meth:`clear_disk` to remove a directory from disk.
        """
        self._close_disk()

    def get(self, key: str) -> Any | None:
        """Return the cached value for ``key`` or ``None`` on a miss.

        Returns ``None`` for misses, decoded values for hits, and ``None``
        (after self-invalidating the row) for corrupt entries surfaced
        by ``diskcache`` as ``sqlite3.DatabaseError`` or similar. The
        caller is expected to ``isinstance``-check the result against
        the type they wrote.
        """
        if self._disk is None:
            return None
        try:
            return self._disk.get(key, default=None, retry=True)
        except Exception:
            # diskcache surfaces ``sqlite3.DatabaseError`` (and friends)
            # for corrupt or partially-written entries from a prior
            # session. Treat as a miss and force-drop the bad row so
            # we don't repeatedly trip the same path.
            logger.warning("disk cache read failed for %s; invalidating", key)
            try:
                del self._disk[key]
            except Exception:
                pass
            return None

    def put(self, key: str, value: Any, *, size_bytes: int) -> None:
        """Write ``value`` under ``key``. No-op when disabled.

        Entries whose ``size_bytes`` exceeds :attr:`disk_max_bytes` are
        skipped with a one-shot warning per key, since diskcache's
        eviction loop would otherwise drain every other row trying — and
        failing — to fit an oversized entry. Callers are responsible
        for measuring the size; the store stays value-agnostic.

        Args:
            key: Namespaced key (e.g. ``"channel:<id>"``). Adapters are
                responsible for picking a prefix that won't collide with
                other adapters writing to the same store.
            value: Anything ``diskcache`` can pickle.
            size_bytes: Caller-measured size used for the oversize guard.
        """
        if self._disk is None:
            return
        if self._disk_max_bytes is not None and size_bytes > self._disk_max_bytes:
            if key not in self._oversized_warned:
                logger.warning(
                    "Entry for %s (%d bytes) is larger than the disk "
                    "cache cap (%d bytes); skipping disk cache for this "
                    "entry so other entries aren't evicted. Raise the "
                    "cap via ``client.cache.enable_disk(max_bytes=...)`` "
                    "to cache this entry on disk.",
                    key,
                    size_bytes,
                    self._disk_max_bytes,
                )
                self._oversized_warned.add(key)
            try:
                self._disk.delete(key, retry=True)
            except Exception:
                pass
            return
        try:
            self._disk.set(key, value, retry=True)
            self._oversized_warned.discard(key)
        except Exception:
            # Best-effort persistence: keep going on disk errors so the
            # caller's request still succeeds. Drop the (possibly
            # partial) disk row.
            logger.warning("disk cache write failed for %s; invalidating", key)
            try:
                self._disk.delete(key, retry=True)
            except Exception:
                pass

    def invalidate(self, key: str) -> None:
        """Remove ``key`` from the cache. Safe to call when absent."""
        # Invalidation is a fresh start for this key; the next put should
        # re-evaluate against the current cap and re-warn if still too big.
        self._oversized_warned.discard(key)
        if self._disk is not None:
            try:
                self._disk.delete(key, retry=True)
            except Exception:
                pass

    def clear(self) -> None:
        """Wipe every entry from the store. The directory itself remains.

        Spans all adapters sharing the store — typically used at test
        teardown or for full reset. Adapters that want to wipe only their
        own namespace should iterate ``self`` and call :meth:`invalidate`
        on matching keys.
        """
        self._oversized_warned.clear()
        if self._disk is not None:
            self._disk.clear()

    def close(self) -> None:
        """Release the disk file handle. Safe to call when disabled."""
        self._close_disk()

    def _open_disk(self, path: str, max_bytes: int) -> None:
        import diskcache

        os.makedirs(path, exist_ok=True)
        self._disk = diskcache.Cache(
            directory=path,
            size_limit=max_bytes,
            eviction_policy="least-recently-used",
            statistics=0,
            tag_index=0,
        )
        self._disk_path = path
        self._disk_max_bytes = max_bytes

    def _close_disk(self) -> None:
        if self._disk is None:
            return
        try:
            self._disk.close()
        except Exception:
            pass
        self._disk = None
        self._disk_path = None
        self._disk_max_bytes = None
