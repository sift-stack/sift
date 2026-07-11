"""Helpers shared by the resource- and principal-attribute resources.

Both sides expose the same three-tier primitive (key, enum value, assignment), so the
ID extraction, key resolution, and value-to-kwargs mapping are identical modulo types.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Any, TypeVar

from sift_client.sift_types._base import BaseType

if TYPE_CHECKING:
    from collections.abc import Awaitable, Callable
    from enum import Enum

KeyT = TypeVar("KeyT", bound=BaseType)


def id_of(value: BaseType | str) -> str:
    """Return the ID of a sift type instance, or the value itself if already an ID."""
    return value._id_or_error if isinstance(value, BaseType) else value


async def resolve_key(
    key: KeyT | str, *, key_cls: type[KeyT], getter: Callable[[str], Awaitable[KeyT]]
) -> KeyT:
    """Resolve a key or key ID to a full key instance, fetching it when given an ID."""
    if isinstance(key, key_cls):
        return key
    if not isinstance(key, str):
        raise TypeError(f"assign requires a {key_cls.__name__} or key ID string.")
    if not key:
        raise ValueError("Key ID cannot be empty.")
    return await getter(key)


def attribute_value_kwargs(value_type: Enum, value: Any) -> dict[str, Any]:
    """Map a user-supplied value to the batch-create value kwargs for a key's value type.

    Dispatches on the enum member name, which is identical for the resource and
    principal value-type enums (ENUM, BOOLEAN, NUMBER, SET_OF_ENUM).
    """
    if value_type.name == "SET_OF_ENUM":
        if not isinstance(value, (list, tuple)):
            raise TypeError("SET_OF_ENUM keys require a list of enum values.")
        if not value:
            raise ValueError(
                "SET_OF_ENUM keys require at least one enum value; archive the existing "
                "assignments to clear a set."
            )
        return {"enum_value_ids": [id_of(v) for v in value]}
    if value_type.name == "ENUM":
        if isinstance(value, (list, tuple)):
            if len(value) != 1:
                raise ValueError("ENUM keys require exactly one enum value.")
            value = value[0]
        return {"enum_value_id": id_of(value)}
    if value_type.name == "BOOLEAN":
        if not isinstance(value, bool):
            raise TypeError("BOOLEAN keys require a bool value.")
        return {"boolean_value": value}
    if value_type.name == "NUMBER":
        if isinstance(value, bool) or not isinstance(value, int):
            raise TypeError("NUMBER keys require an int value.")
        return {"number_value": value}
    raise ValueError(f"Cannot assign a value for value type {value_type}.")
