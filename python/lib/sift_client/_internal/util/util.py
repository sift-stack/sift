from __future__ import annotations

import inspect
import os
from pathlib import Path
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from collections.abc import Iterator

_PACKAGE_ROOT = str(Path(__file__).resolve().parent.parent.parent)
_PYDANTIC_PATH = f"{os.sep}pydantic{os.sep}"


def count_non_none(*args: Any) -> int:
    """Count the number of non-none arguments."""
    return sum(1 for arg in args if arg is not None)


def caller_stacklevel() -> int:
    """Return the ``warnings.warn`` stacklevel of the first frame outside the SDK.

    A fixed stacklevel points at SDK or pydantic internals, which tells the
    caller nothing about which of their lines caused the warning.

    Call this from the function that calls ``warnings.warn``.
    """
    frame = inspect.currentframe()
    frame = frame.f_back if frame is not None else None  # the function calling warn()
    level = 1
    while frame is not None:
        filename = frame.f_code.co_filename
        if not filename.startswith(_PACKAGE_ROOT) and _PYDANTIC_PATH not in filename:
            return level
        frame = frame.f_back
        level += 1
    return 2


def chunked(items: list[Any], size: int) -> Iterator[list[Any]]:
    """Yield successive chunks of at most ``size`` items."""
    for i in range(0, len(items), size):
        yield items[i : i + size]
