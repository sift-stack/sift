from typing import Any, Optional, Type, TypeVar

T = TypeVar("T")


def any_as(value: Any, target_klass: Type[T]) -> Optional[T]:
    """
    Attempts to convert `value` of type `Any` to `target_klass`, otherwise return `None`.
    """

    if isinstance(value, target_klass):
        return value
    else:
        return None
