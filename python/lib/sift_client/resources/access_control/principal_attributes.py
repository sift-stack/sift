from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.principal_attributes import (
    PrincipalAttributesLowLevelClient,
)
from sift_client.resources._base import ResourceBase
from sift_client.resources.access_control._common import (
    attribute_value_kwargs,
    id_of,
    resolve_key,
)
from sift_client.sift_types.principal_attribute import (
    PrincipalAttributeAssignment,
    PrincipalAttributeEnumValue,
    PrincipalAttributeKey,
    PrincipalAttributeKeyUpdate,
    PrincipalAttributeValueLike,
    PrincipalAttributeValueType,
    PrincipalType,
)
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient


class PrincipalAttributesAPIAsync(ResourceBase):
    """High-level API for principal attributes.

    Principal attributes describe the users or groups an access decision applies to.
    A principal is the "who" in an access decision, such as a user or user group.

    Create or fetch an attribute key, define enum values when the key uses them, then
    assign a value to principals. User principals accept either user IDs or email
    addresses; user-group principals use user-group IDs.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the PrincipalAttributesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = PrincipalAttributesLowLevelClient(
            grpc_client=self.client.grpc_client
        )

    async def get_key(self, *, key_id: str) -> PrincipalAttributeKey:
        """Get a principal attribute key by ID."""
        key = await self._low_level_client.get_key(key_id)
        return self._apply_client_to_instance(key)

    async def list_keys(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        value_type: PrincipalAttributeValueType | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[PrincipalAttributeKey]:
        """List principal attribute keys with optional filtering.

        Args:
            name: Exact display name of the key.
            names: Display names to filter by.
            name_contains: Substring match on the display name.
            name_regex: Regex match on the display name.
            value_type: Filter to keys of this value type.
            include_archived: If True, include archived keys.
            filter_query: Explicit CEL query.
            order_by: Field and direction to order by.
            limit: Maximum number of keys to return.
            page_size: Results to fetch per request.
        """
        # The key list filter exposes the display name as the CEL field `display_name`.
        filter_parts = self._build_name_cel_filters(
            name=name,
            names=names,
            name_contains=name_contains,
            name_regex=name_regex,
            field="display_name",
        )
        if value_type is not None:
            filter_parts.append(cel.equals("value_type", value_type.value))
        if filter_query:
            filter_parts.append(filter_query)

        keys = await self._low_level_client.list_all_keys(
            query_filter=cel.and_(*filter_parts) or None,
            order_by=order_by,
            include_archived=include_archived,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(keys)

    async def find_key(self, **kwargs) -> PrincipalAttributeKey | None:
        """Find a single key matching the query. Raises if more than one matches."""
        keys = await self.list_keys(**kwargs)
        if len(keys) > 1:
            raise ValueError(f"Multiple ({len(keys)}) principal attribute keys found for query")
        return keys[0] if keys else None

    async def create_key(
        self,
        display_name: str,
        value_type: PrincipalAttributeValueType,
        *,
        description: str = "",
    ) -> PrincipalAttributeKey:
        """Create a principal attribute key."""
        key = await self._low_level_client.create_key(
            display_name=display_name, value_type=value_type.value, description=description
        )
        return self._apply_client_to_instance(key)

    async def get_or_create_key(
        self,
        display_name: str,
        value_type: PrincipalAttributeValueType,
        *,
        description: str = "",
    ) -> PrincipalAttributeKey:
        """Get a key by display name, creating it if it does not exist.

        Note:
            Display names are not guaranteed unique. If multiple keys share the display
            name, the first active match is returned.
        """
        existing = await self.list_keys(name=display_name, include_archived=False)
        match = next((k for k in existing if k.display_name == display_name), None)
        if match is not None:
            return match
        return await self.create_key(display_name, value_type, description=description)

    async def update_key(
        self,
        key: str | PrincipalAttributeKey,
        update: PrincipalAttributeKeyUpdate | dict,
    ) -> PrincipalAttributeKey:
        """Update a key.

        Args:
            key: The key or key ID to update.
            update: Updates to apply to the key.
        """
        if isinstance(update, dict):
            update = PrincipalAttributeKeyUpdate.model_validate(update)
        update.resource_id = id_of(key)
        updated = await self._low_level_client.update_key(update=update)
        return self._apply_client_to_instance(updated)

    async def archive_key(self, key: str | PrincipalAttributeKey) -> PrincipalAttributeKey:
        """Archive a key. Cascades to its enum values and assignments."""
        key_id = id_of(key)
        await self._low_level_client.archive_key(key_id)
        return await self.get_key(key_id=key_id)

    async def unarchive_key(self, key: str | PrincipalAttributeKey) -> PrincipalAttributeKey:
        """Unarchive a key. Does not restore its cascaded enum values or assignments."""
        key_id = id_of(key)
        await self._low_level_client.unarchive_key(key_id)
        return await self.get_key(key_id=key_id)

    async def check_key_archive_impact(self, key: str | PrincipalAttributeKey) -> int:
        """Return the number of active assignments archiving this key would affect.

        Counts both user and user-group assignments.
        """
        return await self._low_level_client.check_key_archive_impact(id_of(key))

    async def create_enum_value(
        self,
        key: str | PrincipalAttributeKey,
        display_name: str,
        *,
        description: str = "",
    ) -> PrincipalAttributeEnumValue:
        """Create a single enum value for a key."""
        key_id = id_of(key)
        value = await self._low_level_client.create_enum_value(
            key_id=key_id, display_name=display_name, description=description
        )
        return self._apply_client_to_instance(value)

    async def list_enum_values(
        self,
        key: str | PrincipalAttributeKey,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[PrincipalAttributeEnumValue]:
        """List the enum values defined for a key."""
        key_id = id_of(key)
        filter_parts = self._build_name_cel_filters(
            name=name, names=names, name_contains=name_contains, name_regex=name_regex
        )
        if filter_query:
            filter_parts.append(filter_query)
        values = await self._low_level_client.list_all_enum_values(
            key_id=key_id,
            query_filter=cel.and_(*filter_parts) or None,
            order_by=order_by,
            include_archived=include_archived,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(values)

    async def get_or_create_enum_values(
        self, key: str | PrincipalAttributeKey, names: list[str]
    ) -> list[PrincipalAttributeEnumValue]:
        """Get enum values for a key by name, creating any that don't exist.

        Returns the values in the same order as ``names``.
        """
        key_id = id_of(key)
        existing = await self.list_enum_values(key_id, include_archived=False)
        by_name = {v.display_name: v for v in existing}
        result: list[PrincipalAttributeEnumValue] = []
        for name in names:
            value = by_name.get(name)
            if value is None:
                value = await self.create_enum_value(key_id, name)
                by_name[name] = value
            result.append(value)
        return result

    async def archive_enum_value(
        self,
        enum_value: str | PrincipalAttributeEnumValue,
        *,
        replacement: str | PrincipalAttributeEnumValue | None = None,
    ) -> int:
        """Archive an enum value, migrating existing assignments to a replacement.

        Returns the number of assignments migrated.
        """
        enum_value_id = id_of(enum_value)
        replacement_id = id_of(replacement) if replacement is not None else ""
        return await self._low_level_client.archive_enum_value(
            enum_value_id, replacement_enum_value_id=replacement_id
        )

    async def unarchive_enum_value(
        self, enum_value: str | PrincipalAttributeEnumValue
    ) -> PrincipalAttributeEnumValue:
        """Unarchive an enum value."""
        enum_value_id = id_of(enum_value)
        await self._low_level_client.unarchive_enum_value(enum_value_id)
        value = await self._low_level_client.get_enum_value(enum_value_id)
        return self._apply_client_to_instance(value)

    async def assign(
        self,
        key: str | PrincipalAttributeKey,
        principals: list[str],
        *,
        value: PrincipalAttributeValueLike,
        principal_type: PrincipalType = PrincipalType.USER,
    ) -> list[PrincipalAttributeAssignment]:
        """Assign a key's value to principals.

        Args:
            key: The key or key ID to assign. Its ``value_type`` determines how ``value`` is interpreted.
            principals: Principal IDs. For ``USER`` principals, entries containing ``@`` are
                treated as email addresses and resolved to user IDs.
            value: For ``SET_OF_ENUM``, a list of enum values (or their IDs) that becomes the
                full set on each principal; for ``ENUM``, a single enum value; for ``BOOLEAN``,
                a bool; for ``NUMBER``, an int.
            principal_type: The kind of principal being assigned to. Defaults to ``USER``. Use
                ``PrincipalType.USER_GROUP`` when assigning to user groups.

        Returns:
            The created assignments.
        """
        resolved_key = await resolve_key(
            key, key_cls=PrincipalAttributeKey, getter=lambda key_id: self.get_key(key_id=key_id)
        )
        resolved_ids = await self._resolve_principal_ids(principals, principal_type=principal_type)
        create_kwargs = attribute_value_kwargs(resolved_key.value_type, value)

        created = await self._low_level_client.batch_create_values(
            key_id=resolved_key._id_or_error,
            principal_ids=resolved_ids,
            principal_type=principal_type.value,
            **create_kwargs,
        )
        return self._apply_client_to_instances(created)

    async def get_assignment(
        self,
        *,
        assignment_id: str,
        principal_type: PrincipalType = PrincipalType.USER,
    ) -> PrincipalAttributeAssignment:
        """Get a single assignment by ID."""
        value = await self._low_level_client.get_value(
            assignment_id, principal_type=principal_type.value
        )
        return self._apply_client_to_instance(value)

    async def list_assignments(
        self,
        *,
        key: str | PrincipalAttributeKey | None = None,
        principal: str | None = None,
        principal_type: PrincipalType = PrincipalType.USER,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[PrincipalAttributeAssignment]:
        """List principal attribute assignments.

        Args:
            key: Filter to assignments of this key.
            principal: Filter to assignments for this principal. Use a user ID or email address
                for users; use a user-group ID with ``PrincipalType.USER_GROUP`` for user groups.
            principal_type: The kind of principal to list assignments for. Defaults to ``USER``.
            include_archived: If True, include archived assignments.
            filter_query: Explicit CEL query.
            order_by: Field and direction to order by.
            limit: Maximum number of assignments to return.
            page_size: Results to fetch per request.
        """
        filter_parts = []
        if principal is not None:
            (resolved,) = await self._resolve_principal_ids(
                [principal], principal_type=principal_type
            )
            filter_parts.append(cel.equals("principal_id", resolved))
        if filter_query:
            filter_parts.append(filter_query)
        query_filter = cel.and_(*filter_parts) or None

        if key is not None:
            values = await self._low_level_client.list_all_key_values(
                key_id=id_of(key),
                principal_type=principal_type.value,
                query_filter=query_filter,
                order_by=order_by,
                include_archived=include_archived,
                max_results=limit,
                **({"page_size": page_size} if page_size is not None else {}),
            )
        else:
            values = await self._low_level_client.list_all_values(
                principal_type=principal_type.value,
                query_filter=query_filter,
                order_by=order_by,
                include_archived=include_archived,
                max_results=limit,
                **({"page_size": page_size} if page_size is not None else {}),
            )
        return self._apply_client_to_instances(values)

    async def archive_assignments(
        self,
        assignments: list[str | PrincipalAttributeAssignment],
        *,
        principal_type: PrincipalType = PrincipalType.USER,
    ) -> None:
        """Batch archive assignments."""
        ids = [id_of(a) for a in assignments]
        await self._low_level_client.archive_values(ids, principal_type=principal_type.value)

    async def unarchive_assignments(
        self,
        assignments: list[str | PrincipalAttributeAssignment],
        *,
        principal_type: PrincipalType = PrincipalType.USER,
    ) -> None:
        """Batch unarchive assignments."""
        ids = [id_of(a) for a in assignments]
        await self._low_level_client.unarchive_values(ids, principal_type=principal_type.value)

    async def _resolve_principal_ids(
        self, principals: list[str], *, principal_type: PrincipalType
    ) -> list[str]:
        """Resolve a list of principals to IDs, treating user emails (``@``) as lookups."""
        emails = [
            p
            for p in principals
            if principal_type == PrincipalType.USER and isinstance(p, str) and "@" in p
        ]
        email_to_id = await self.client.async_.users.resolve_ids(emails) if emails else {}
        resolved: list[str] = []
        for principal in principals:
            if principal in email_to_id:
                resolved.append(email_to_id[principal])
            elif "@" in principal and principal_type != PrincipalType.USER:
                raise ValueError(
                    f"Email resolution is only supported for USER principals; got {principal!r} "
                    f"for principal_type {principal_type.name}. Pass a principal ID instead."
                )
            elif principal_type == PrincipalType.USER and "@" in principal:
                raise ValueError(f"No user found for email {principal!r}")
            else:
                resolved.append(principal)
        return resolved
