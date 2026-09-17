"""References to other Sift resources.

Fields that point at another resource accept either the resource or its ID and resolve
to an ID only when a proto is built:

    webhook: Ref[Webhook] | None = None

    def _to_update_request(self):
        ... resolve_id(self.webhook, field="webhook") ...
"""

from __future__ import annotations

from typing import TYPE_CHECKING, TypeVar, Union
from uuid import UUID

if TYPE_CHECKING:
    from collections.abc import Iterable

    from sift_client.sift_types._base import BaseTypeProtocol

T = TypeVar("T")

Ref = Union[T, str]
"""A resource or its ID."""


def resolve_id(value: BaseTypeProtocol | str | None, *, field: str) -> str:
    """Resolve a resource or ID to the UUID string a proto expects.

    Args:
        value: The resource or its ID.
        field: The field name, used in error messages.

    Returns:
        The ID as a canonical UUID string.

    Raises:
        ValueError: If the value is missing or is not a UUID.
    """
    if value is None:
        raise ValueError(f"{field} is required")
    raw = value if isinstance(value, str) else value._id_or_error
    try:
        return str(UUID(raw))
    except ValueError as exc:
        raise ValueError(f"{field} must be a UUID or a Sift resource, got {raw!r}") from exc


def resolve_ids(values: Iterable[BaseTypeProtocol | str] | None, *, field: str) -> list[str] | None:
    """Resolve a collection of resources or IDs, or None if there are none.

    Args:
        values: The resources or IDs.
        field: The field name, used in error messages.

    Returns:
        The IDs as canonical UUID strings.
    """
    if not values:
        return None
    return [resolve_id(value, field=field) for value in values]
