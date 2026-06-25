"""User-expressed configuration for a resource's optional disk-cache tier."""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import os


class DiskCacheConfig:
    """Holds a resource's disk-cache enable/path/max-bytes intent.

    Resources own one instance, mutate it via :meth:`enable` / :meth:`disable`
    in response to user calls, and read the properties at lazy-init time to
    decide what kwargs to forward to their cache-aware wrapper.

    The :attr:`using_default_path` property is the key invariant for the
    silent-fallback-vs-loud-raise distinction in resource lazy-init code:
    if the user picked a specific path and opening fails, the failure
    surfaces; if the user left the default and opening fails, the resource
    falls back to memory-only without disrupting the call.

    Args:
        enabled: Initial enabled state. Pass ``True`` for opt-out (the disk
            tier is on by default and users call ``disable`` to turn it off);
            pass ``False`` for opt-in (users call ``enable`` to turn it on).
    """

    def __init__(self, *, enabled: bool = True) -> None:
        self._enabled = enabled
        self._path: str | None = None
        self._max_bytes: int | None = None

    @property
    def enabled(self) -> bool:
        """Whether the disk tier should be opened on the next lazy init."""
        return self._enabled

    @property
    def path(self) -> str | None:
        """User-supplied disk-cache path, or ``None`` to defer to the cache's default."""
        return self._path

    @property
    def max_bytes(self) -> int | None:
        """User-supplied disk-cache byte cap, or ``None`` to defer to the cache's default."""
        return self._max_bytes

    @property
    def using_default_path(self) -> bool:
        """``True`` when the disk tier is enabled *and* the path is the cache's default.

        Resources use this to decide whether to silently fall back to memory
        on a disk-open failure (default path: the user didn't ask for it
        specifically, so degrade gracefully) or to re-raise (explicit path:
        the user asked for it, so failure must surface).
        """
        return self._enabled and self._path is None

    def enable(
        self,
        *,
        path: str | os.PathLike[str] | None = None,
        max_bytes: int | None = None,
    ) -> None:
        """Mark the disk tier as enabled, optionally with a custom path or byte cap.

        Args:
            path: Directory to persist to. ``None`` leaves the cache's
                default in effect.
            max_bytes: Byte cap on the disk tier. ``None`` leaves the
                cache's default in effect.
        """
        self._enabled = True
        self._path = str(path) if path is not None else None
        self._max_bytes = max_bytes

    def disable(self) -> None:
        """Mark the disk tier as disabled and clear any custom path / byte cap.

        Subsequent :meth:`enable` calls re-enable at the cache's defaults
        unless overrides are supplied.
        """
        self._enabled = False
        self._path = None
        self._max_bytes = None
