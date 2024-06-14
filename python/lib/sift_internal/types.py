from typing import Any, Optional, Type, TypeVar

T = TypeVar("T")
def any_as(val: Any, target_type: Type[T]) -> Optional[T]:
    """
    Tries to cast `val` into `target_type` otherwise returns `None`.
    """

    if val is None:
        return None

    if isinstance(val, target_type):
        return val
    else:
        return None
