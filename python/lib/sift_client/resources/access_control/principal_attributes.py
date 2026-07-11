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
    PrincipalRef,
    PrincipalType,
)
from sift_client.sift_types.user import User
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from datetime import datetime

    from sift_client.client import SiftClient


def _as_principal_ref(principal: PrincipalRef | User | str) -> PrincipalRef:
    """Resolve a principal input to a typed reference; bare IDs are rejected."""
    if isinstance(principal, PrincipalRef):
        return principal
    if isinstance(principal, User):
        return PrincipalRef.user(principal)
    if isinstance(principal, str) and "@" in principal:
        # An email names its own type: only user principals have emails.
        return PrincipalRef.user(principal)
    raise TypeError(
        f"Cannot resolve principal {principal!r}. Pass PrincipalRef.user(...) or "
        "PrincipalRef.user_group(...), a User object, or a user email address; a bare ID "
        "does not say which kind of principal it refers to."
    )


class PrincipalAttributesAPIAsync(ResourceBase):
    """High-level API for principal attributes.

    Principal attributes describe the users or groups an access decision applies to.
    A principal is the "who" in an access decision, such as a user or user group.

    Create or fetch an attribute key via `keys`, define enum values via `enum_values`
    when the key uses them, then assign a value to principals via `assignments`. Pass
    ``User`` objects, ``PrincipalRef`` references, or user email addresses; use
    ``PrincipalRef.user_group(...)`` for user groups.
    """

    keys: PrincipalAttributeKeysAPIAsync
    """Nested keys API. See `PrincipalAttributeKeysAPIAsync`."""

    enum_values: PrincipalAttributeEnumValuesAPIAsync
    """Nested enum values API. See `PrincipalAttributeEnumValuesAPIAsync`."""

    assignments: PrincipalAttributeAssignmentsAPIAsync
    """Nested assignments API. See `PrincipalAttributeAssignmentsAPIAsync`."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the PrincipalAttributesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self.keys = PrincipalAttributeKeysAPIAsync(sift_client)
        self.enum_values = PrincipalAttributeEnumValuesAPIAsync(sift_client)
        self.assignments = PrincipalAttributeAssignmentsAPIAsync(sift_client)


class PrincipalAttributeKeysAPIAsync(ResourceBase):
    """High-level API for principal attribute keys.

    Accessed as a nested resource via ``client.access_control.principal_attributes.keys``.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the PrincipalAttributeKeysAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = PrincipalAttributesLowLevelClient(
            grpc_client=self.client.grpc_client
        )

    async def get(self, *, key_id: str) -> PrincipalAttributeKey:
        """Get a principal attribute key by ID.

        Args:
            key_id: The ID of the key.

        Returns:
            The key.
        """
        key = await self._low_level_client.get_key(key_id)
        return self._apply_client_to_instance(key)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        value_type: PrincipalAttributeValueType | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: str | None = None,
        modified_by: str | None = None,
        description_contains: str | None = None,
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
            created_after: Filter to keys created after this datetime.
            created_before: Filter to keys created before this datetime.
            modified_after: Filter to keys modified after this datetime.
            modified_before: Filter to keys modified before this datetime.
            created_by: Filter to keys created by this user ID.
            modified_by: Filter to keys last modified by this user ID.
            description_contains: Substring match on the description.
            include_archived: If True, include archived keys.
            filter_query: Explicit CEL query.
            order_by: Field and direction to order by.
            limit: Maximum number of keys to return.
            page_size: Results to fetch per request.

        Returns:
            The matching keys.
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
        filter_parts.extend(
            self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
                modified_by=modified_by,
            )
        )
        if description_contains:
            filter_parts.append(cel.contains("description", description_contains))
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

    async def find(self, **kwargs) -> PrincipalAttributeKey | None:
        """Find a single key matching the query. Takes the same arguments as `list_`.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The key found, or None if no key matches.

        Raises:
            ValueError: If more than one key matches.
        """
        keys = await self.list_(**kwargs)
        if len(keys) > 1:
            raise ValueError(f"Multiple ({len(keys)}) principal attribute keys found for query")
        return keys[0] if keys else None

    async def create(
        self,
        display_name: str,
        value_type: PrincipalAttributeValueType,
        *,
        description: str = "",
    ) -> PrincipalAttributeKey:
        """Create a principal attribute key.

        Args:
            display_name: The human-readable name of the key.
            value_type: The value type of the key.
            description: Optional description.

        Returns:
            The created key.
        """
        key = await self._low_level_client.create_key(
            display_name=display_name, value_type=value_type.value, description=description
        )
        return self._apply_client_to_instance(key)

    async def get_or_create(
        self,
        display_name: str,
        value_type: PrincipalAttributeValueType,
        *,
        description: str = "",
    ) -> PrincipalAttributeKey:
        """Get a key by display name, creating it if it does not exist.

        Args:
            display_name: The human-readable name of the key.
            value_type: The value type used if the key is created.
            description: Optional description used if the key is created.

        Returns:
            The existing or newly created key.

        Note:
            Display names are not guaranteed unique. If multiple keys share the display
            name, the first active match is returned.
        """
        existing = await self.list_(name=display_name, include_archived=False)
        match = next((k for k in existing if k.display_name == display_name), None)
        if match is not None:
            return match
        return await self.create(display_name, value_type, description=description)

    async def update(
        self,
        key: str | PrincipalAttributeKey,
        update: PrincipalAttributeKeyUpdate | dict,
    ) -> PrincipalAttributeKey:
        """Update a key.

        Args:
            key: The key or key ID to update.
            update: Updates to apply to the key.

        Returns:
            The updated key.
        """
        if isinstance(update, dict):
            update = PrincipalAttributeKeyUpdate.model_validate(update)
        update.resource_id = id_of(key)
        updated = await self._low_level_client.update_key(update=update)
        return self._apply_client_to_instance(updated)

    async def archive(self, key: str | PrincipalAttributeKey) -> PrincipalAttributeKey:
        """Archive a key. Cascades to its enum values and assignments.

        Args:
            key: The key or key ID to archive.

        Returns:
            The archived key.
        """
        key_id = id_of(key)
        await self._low_level_client.archive_key(key_id)
        return await self.get(key_id=key_id)

    async def unarchive(self, key: str | PrincipalAttributeKey) -> PrincipalAttributeKey:
        """Unarchive a key. Does not restore its cascaded enum values or assignments.

        Args:
            key: The key or key ID to unarchive.

        Returns:
            The unarchived key.
        """
        key_id = id_of(key)
        await self._low_level_client.unarchive_key(key_id)
        return await self.get(key_id=key_id)

    async def check_archive_impact(self, key: str | PrincipalAttributeKey) -> int:
        """Check how many assignments archiving a key would affect.

        Counts both user and user-group assignments.

        Args:
            key: The key or key ID to check.

        Returns:
            The number of active assignments archiving this key would affect.
        """
        return await self._low_level_client.check_key_archive_impact(id_of(key))


class PrincipalAttributeEnumValuesAPIAsync(ResourceBase):
    """High-level API for the enum values defined on principal attribute keys.

    Accessed as a nested resource via
    ``client.access_control.principal_attributes.enum_values``.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the PrincipalAttributeEnumValuesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = PrincipalAttributesLowLevelClient(
            grpc_client=self.client.grpc_client
        )

    async def create(
        self,
        key: str | PrincipalAttributeKey,
        display_name: str,
        *,
        description: str = "",
    ) -> PrincipalAttributeEnumValue:
        """Create a single enum value for a key.

        Args:
            key: The key or key ID the enum value belongs to.
            display_name: The human-readable name of the enum value.
            description: Optional description.

        Returns:
            The created enum value.
        """
        key_id = id_of(key)
        value = await self._low_level_client.create_enum_value(
            key_id=key_id, display_name=display_name, description=description
        )
        return self._apply_client_to_instance(value)

    async def list_(
        self,
        key: str | PrincipalAttributeKey,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: str | None = None,
        modified_by: str | None = None,
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[PrincipalAttributeEnumValue]:
        """List the enum values defined for a key.

        Args:
            key: The key or key ID to list enum values for.
            name: Exact display name of the enum value.
            names: Display names to filter by.
            name_contains: Substring match on the display name.
            name_regex: Regex match on the display name.
            created_after: Filter to enum values created after this datetime.
            created_before: Filter to enum values created before this datetime.
            modified_after: Filter to enum values modified after this datetime.
            modified_before: Filter to enum values modified before this datetime.
            created_by: Filter to enum values created by this user ID.
            modified_by: Filter to enum values last modified by this user ID.
            description_contains: Substring match on the description.
            include_archived: If True, include archived enum values.
            filter_query: Explicit CEL query.
            order_by: Field and direction to order by.
            limit: Maximum number of enum values to return.
            page_size: Results to fetch per request.

        Returns:
            The matching enum values.
        """
        key_id = id_of(key)
        filter_parts = self._build_name_cel_filters(
            name=name, names=names, name_contains=name_contains, name_regex=name_regex
        )
        filter_parts.extend(
            self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
                modified_by=modified_by,
            )
        )
        if description_contains:
            filter_parts.append(cel.contains("description", description_contains))
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

    async def get_or_create(
        self, key: str | PrincipalAttributeKey, names: list[str]
    ) -> list[PrincipalAttributeEnumValue]:
        """Get enum values for a key by name, creating any that don't exist.

        Args:
            key: The key or key ID the enum values belong to.
            names: Display names of the enum values to get or create.

        Returns:
            The enum values, in the same order as ``names``.
        """
        key_id = id_of(key)
        existing = await self.list_(key_id, include_archived=False)
        by_name = {v.display_name: v for v in existing}
        result: list[PrincipalAttributeEnumValue] = []
        for name in names:
            value = by_name.get(name)
            if value is None:
                value = await self.create(key_id, name)
                by_name[name] = value
            result.append(value)
        return result

    async def archive(
        self,
        enum_value: str | PrincipalAttributeEnumValue,
        *,
        replacement: str | PrincipalAttributeEnumValue | None = None,
    ) -> int:
        """Archive an enum value, migrating existing assignments to a replacement.

        Args:
            enum_value: The enum value or enum value ID to archive.
            replacement: Optional enum value or enum value ID that existing
                assignments are migrated to.

        Returns:
            The number of assignments migrated.
        """
        enum_value_id = id_of(enum_value)
        replacement_id = id_of(replacement) if replacement is not None else ""
        return await self._low_level_client.archive_enum_value(
            enum_value_id, replacement_enum_value_id=replacement_id
        )

    async def unarchive(
        self, enum_value: str | PrincipalAttributeEnumValue
    ) -> PrincipalAttributeEnumValue:
        """Unarchive an enum value.

        Args:
            enum_value: The enum value or enum value ID to unarchive.

        Returns:
            The unarchived enum value.
        """
        enum_value_id = id_of(enum_value)
        await self._low_level_client.unarchive_enum_value(enum_value_id)
        value = await self._low_level_client.get_enum_value(enum_value_id)
        return self._apply_client_to_instance(value)


class PrincipalAttributeAssignmentsAPIAsync(ResourceBase):
    """High-level API for principal attribute assignments.

    Accessed as a nested resource via
    ``client.access_control.principal_attributes.assignments``.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the PrincipalAttributeAssignmentsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = PrincipalAttributesLowLevelClient(
            grpc_client=self.client.grpc_client
        )

    async def create(
        self,
        key: str | PrincipalAttributeKey,
        principals: list[PrincipalRef | User | str],
        *,
        value: PrincipalAttributeValueLike,
    ) -> list[PrincipalAttributeAssignment]:
        """Assign a key's value to principals.

        Args:
            key: The key or key ID to assign. Its ``value_type`` determines how ``value`` is interpreted.
            principals: Principals to assign to. Pass ``PrincipalRef.user(...)`` /
                ``PrincipalRef.user_group(...)`` references, ``User`` objects, or user
                email addresses (resolved to user IDs automatically). Bare IDs are
                rejected because they do not say which kind of principal they refer to.
            value: For ``SET_OF_ENUM``, a list of enum values (or their IDs) that becomes the
                full set on each principal; for ``ENUM``, a single enum value; for ``BOOLEAN``,
                a bool; for ``NUMBER``, an int.

        Returns:
            The created assignments, one per enum value per principal for
            ``SET_OF_ENUM`` keys. Order is not guaranteed to match the input order.
        """
        resolved_key = await resolve_key(
            key,
            key_cls=PrincipalAttributeKey,
            getter=lambda key_id: self._low_level_client.get_key(key_id),
        )
        refs = await self._resolve_email_refs([_as_principal_ref(p) for p in principals])
        refs = list({(ref.principal_type, ref.principal_id): ref for ref in refs}.values())
        create_kwargs = attribute_value_kwargs(resolved_key.value_type, value)

        grouped: dict[PrincipalType, list[str]] = {}
        for ref in refs:
            grouped.setdefault(ref.principal_type, []).append(ref.principal_id)

        created: list[PrincipalAttributeAssignment] = []
        for principal_type, principal_ids in grouped.items():
            created.extend(
                await self._low_level_client.batch_create_values(
                    key_id=resolved_key._id_or_error,
                    principal_ids=principal_ids,
                    principal_type=principal_type.value,
                    **create_kwargs,
                )
            )
        return self._apply_client_to_instances(created)

    async def get(
        self,
        *,
        assignment_id: str,
        principal_type: PrincipalType,
    ) -> PrincipalAttributeAssignment:
        """Get a single assignment by ID and principal type.

        Args:
            assignment_id: The ID of the assignment.
            principal_type: The kind of principal the assignment applies to.

        Returns:
            The assignment.
        """
        value = await self._low_level_client.get_value(
            assignment_id, principal_type=principal_type.value
        )
        return self._apply_client_to_instance(value)

    async def list_(
        self,
        *,
        key: str | PrincipalAttributeKey | None = None,
        principal: PrincipalRef | User | str | None = None,
        principal_type: PrincipalType | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        created_by: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[PrincipalAttributeAssignment]:
        """List principal attribute assignments.

        For ``SET_OF_ENUM`` keys, each enum value is returned as its own assignment.

        Args:
            key: Filter to assignments of this key.
            principal: Filter to assignments for this principal. Pass a ``PrincipalRef``,
                a ``User`` object, or a user email address.
            principal_type: The kind of principal to list assignments for when
                ``principal`` is not given. Defaults to ``USER``. When ``principal`` is
                given, its own type is used and this must match it if set.
            created_after: Filter to assignments created after this datetime.
            created_before: Filter to assignments created before this datetime.
            created_by: Filter to assignments created by this user ID.
            include_archived: If True, include archived assignments.
            filter_query: Explicit CEL query.
            order_by: Field and direction to order by.
            limit: Maximum number of assignments to return.
            page_size: Results to fetch per request.

        Returns:
            The matching assignments.

        Raises:
            ValueError: If ``principal_type`` conflicts with the type of ``principal``.
        """
        filter_parts = []
        if principal is not None:
            (ref,) = await self._resolve_email_refs([_as_principal_ref(principal)])
            if principal_type is not None and principal_type != ref.principal_type:
                raise ValueError(
                    f"principal_type {principal_type.name} conflicts with the principal's "
                    f"own type {ref.principal_type.name}."
                )
            principal_type = ref.principal_type
            filter_parts.append(cel.equals("principal_id", ref.principal_id))
        elif principal_type is None:
            principal_type = PrincipalType.USER
        filter_parts.extend(
            self._build_time_cel_filters(
                created_after=created_after, created_before=created_before, created_by=created_by
            )
        )
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

    async def archive(
        self,
        assignments: list[str | PrincipalAttributeAssignment],
        *,
        principal_type: PrincipalType,
    ) -> None:
        """Batch archive assignments of the given principal type.

        Args:
            assignments: The assignments or assignment IDs to archive.
            principal_type: The kind of principal the assignments apply to.
        """
        ids = [id_of(a) for a in assignments]
        await self._low_level_client.archive_values(ids, principal_type=principal_type.value)

    async def unarchive(
        self,
        assignments: list[str | PrincipalAttributeAssignment],
        *,
        principal_type: PrincipalType,
    ) -> None:
        """Batch unarchive assignments of the given principal type.

        Args:
            assignments: The assignments or assignment IDs to unarchive.
            principal_type: The kind of principal the assignments apply to.
        """
        ids = [id_of(a) for a in assignments]
        await self._low_level_client.unarchive_values(ids, principal_type=principal_type.value)

    async def _resolve_email_refs(self, refs: list[PrincipalRef]) -> list[PrincipalRef]:
        """Resolve user emails (``@``) in principal references to user IDs."""
        emails = [
            ref.principal_id
            for ref in refs
            if ref.principal_type == PrincipalType.USER and "@" in ref.principal_id
        ]
        email_to_id = await self.client.async_.users.resolve_ids(emails) if emails else {}
        resolved: list[PrincipalRef] = []
        for ref in refs:
            if "@" not in ref.principal_id:
                resolved.append(ref)
            elif ref.principal_type != PrincipalType.USER:
                raise ValueError(
                    f"Email resolution is only supported for USER principals; got "
                    f"{ref.principal_id!r} for principal_type {ref.principal_type.name}. "
                    "Pass a principal ID instead."
                )
            elif ref.principal_id not in email_to_id:
                raise ValueError(f"No user found for email {ref.principal_id!r}")
            else:
                resolved.append(PrincipalRef.user(email_to_id[ref.principal_id]))
        return resolved
