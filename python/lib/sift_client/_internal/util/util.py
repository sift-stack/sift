from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from collections.abc import Iterator


def count_non_none(*args: Any) -> int:
    """Count the number of non-none arguments."""
    return sum(1 for arg in args if arg is not None)


def chunked(items: list[Any], size: int) -> Iterator[list[Any]]:
    """Yield successive chunks of at most ``size`` items."""
    for i in range(0, len(items), size):
        yield items[i : i + size]
