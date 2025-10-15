from typing import Any


def count_non_none(*args: Any) -> int:
    """Count the number of non-none arguments."""
    return sum(1 for arg in args if arg is not None)
