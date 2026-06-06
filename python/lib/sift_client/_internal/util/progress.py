"""Progress-bar helpers that degrade gracefully when stdout is unavailable.

``alive_progress`` defaults its output to ``sys.stdout`` and requires a stream
with ``write``/``flush``/``fileno``. When ``sys.stdout`` is ``None`` (for example
a PyInstaller ``--noconsole`` executable, ``pythonw.exe``, or a detached
process) it raises while setting up the bar, so these wrappers suppress the bar
in that case instead of crashing.
"""

from __future__ import annotations

import sys
from contextlib import contextmanager
from typing import Any, Iterator

from alive_progress import alive_bar as _alive_bar  # type: ignore[import-untyped]


def _stdout_supports_progress() -> bool:
    """Whether the current stdout can back an alive_progress bar."""
    stream = sys.stdout
    return stream is not None and all(
        hasattr(stream, attr) for attr in ("write", "flush", "fileno")
    )


class _NoOpBar:
    """Stand-in for an alive_progress bar handle used when progress is suppressed."""

    def __call__(self, *args: Any, **kwargs: Any) -> None:
        return None

    def __getattr__(self, name: str) -> Any:
        return lambda *args, **kwargs: None


@contextmanager
def alive_bar(*args: Any, **kwargs: Any) -> Iterator[Any]:
    """Drop-in for ``alive_progress.alive_bar`` that no-ops when stdout can't back a bar."""
    if not _stdout_supports_progress():
        yield _NoOpBar()
        return
    with _alive_bar(*args, **kwargs) as bar:
        yield bar
