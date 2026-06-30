from __future__ import annotations

import asyncio
from typing import TYPE_CHECKING, Any, Union

from sift_client._internal.low_level_wrappers.resource_attributes import (
    ResourceAttributesLowLevelClient,
)
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.channel import Channel
from sift_client.sift_types.resource_attribute import (
    ResourceAttribute,
    ResourceAttributeEntity,
    ResourceAttributeEntityType,
    ResourceAttributeEnumValue,
    ResourceAttributeKey,
    ResourceAttributeKeyType,
)
from sift_client.sift_types.run import Run
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient

# Max resources per BatchCreateResourceAttributes call; keeps request bodies well under
# gRPC message size limits when assigning to large resource sets.
ASSIGN_BATCH_SIZE = 1000

ResourceLike = Union[ResourceAttributeEntity, Asset, Channel, Run, str]
SUPPORTED_RESOURCE_ENTITY_TYPES = {
    ResourceAttributeEntityType.ASSET,
    ResourceAttributeEntityType.CHANNEL,
    ResourceAttributeEntityType.RUN,
}


def _resolve_resource_object(resource: ResourceLike) -> ResourceAttributeEntity:
    """Resolve a supported resource object to a ResourceAttributeEntity."""
    if isinstance(resource, ResourceAttributeEntity):
        if resource.entity_type not in SUPPORTED_RESOURCE_ENTITY_TYPES:
            raise ValueError("Resource attributes currently support assets, channels, and runs.")
        return resource
    if isinstance(resource, Asset):
        return ResourceAttributeEntity.for_asset(resource._id_or_error)
    if isinstance(resource, Channel):
        return ResourceAttributeEntity.for_channel(resource._id_or_error)
    if isinstance(resource, Run):
        return ResourceAttributeEntity.for_run(resource._id_or_error)
    raise TypeError(
        f"Cannot resolve resource of type {type(resource).__name__}. Pass a supported resource "
        "object, such as an Asset, Channel, or Run; a resource ID string; or a "
        "ResourceAttributeEntity."
    )


def _enum_value_id(value: ResourceAttributeEnumValue | str) -> str:
    return value._id_or_error if isinstance(value, ResourceAttributeEnumValue) else value


def _chunks(items: list[Any], size: int):
    for i in range(0, len(items), size):
        yield items[i : i + size]


class ResourceAttributesAPIAsync(ResourceBase):
    """High-level API for resource attributes.

    Resource attributes describe the Sift objects an access decision applies to. A
    resource is the "what" in an access decision.

    Create or fetch an attribute key, define enum values when the key uses them, then
    assign a value to resources. For currently supported resource types, you can pass
    existing ``Asset``, ``Channel``, and ``Run`` objects or their IDs directly.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the ResourceAttributesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ResourceAttributesLowLevelClient(
            grpc_client=self.client.grpc_client
        )

    async def _resolve_key(self, key: str | ResourceAttributeKey) -> ResourceAttributeKey:
        if isinstance(key, ResourceAttributeKey):
            return key
        if not isinstance(key, str):
            raise TypeError("assign requires a ResourceAttributeKey or key ID string.")
        if not key:
            raise ValueError("Key ID cannot be empty.")
        return await self.get_key(key_id=key)

    async def _resolve_resource(self, resource: ResourceLike) -> ResourceAttributeEntity:
        return (await self._resolve_resources([resource]))[0]

    async def _resolve_resources(
        self, resources: list[ResourceAttributeEntity | Asset | Channel | Run | str]
    ) -> list[ResourceAttributeEntity]:
        resolved: list[ResourceAttributeEntity | str] = []
        resource_ids: list[str] = []
        for resource in resources:
            if isinstance(resource, str):
                if not resource:
                    raise ValueError("Resource ID cannot be empty.")
                resolved.append(resource)
                resource_ids.append(resource)
            else:
                resolved.append(_resolve_resource_object(resource))

        if not resource_ids:
            return [
                resource for resource in resolved if isinstance(resource, ResourceAttributeEntity)
            ]

        resources_by_id = await self._resolve_resource_ids(resource_ids)
        return [
            resources_by_id[resource] if isinstance(resource, str) else resource
            for resource in resolved
        ]

    async def _resolve_resource_ids(
        self, resource_ids: list[str]
    ) -> dict[str, ResourceAttributeEntity]:
        unique_ids = list(dict.fromkeys(resource_ids))
        matched_types: dict[str, set[ResourceAttributeEntityType]] = {}
        for batch in _chunks(unique_ids, ASSIGN_BATCH_SIZE):
            assets, channels, runs = await asyncio.gather(
                self.client.async_.assets.list_(asset_ids=batch, include_archived=True),
                self.client.async_.channels.list_(channel_ids=batch),
                self.client.async_.runs.list_(run_ids=batch, include_archived=True),
            )
            for resources_for_type, entity_type in (
                (assets, ResourceAttributeEntityType.ASSET),
                (channels, ResourceAttributeEntityType.CHANNEL),
                (runs, ResourceAttributeEntityType.RUN),
            ):
                for resource in resources_for_type:
                    matched_types.setdefault(resource._id_or_error, set()).add(entity_type)

        ambiguous_ids = [
            resource_id
            for resource_id, entity_types in matched_types.items()
            if len(entity_types) > 1
        ]
        if ambiguous_ids:
            raise ValueError(
                f"Resource ID {ambiguous_ids[0]!r} matched multiple currently supported "
                "resource types."
            )

        missing_ids = [
            resource_id for resource_id in unique_ids if resource_id not in matched_types
        ]
        if missing_ids:
            preview = ", ".join(repr(resource_id) for resource_id in missing_ids[:3])
            if len(missing_ids) > 3:
                preview = f"{preview}, and {len(missing_ids) - 3} more"
            raise ValueError(
                f"Could not resolve resource ID(s) {preview} as currently supported resources "
                "(asset, channel, or run)."
            )

        return {
            resource_id: ResourceAttributeEntity(
                entity_id=resource_id, entity_type=next(iter(entity_types))
            )
            for resource_id, entity_types in matched_types.items()
        }

    async def get_key(self, *, key_id: str) -> ResourceAttributeKey:
        """Get a resource attribute key by ID."""
        key = await self._low_level_client.get_key(key_id)
        return self._apply_client_to_instance(key)

    async def list_keys(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        key_type: ResourceAttributeKeyType | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[ResourceAttributeKey]:
        """List resource attribute keys with optional filtering.

        Args:
            name: Exact display name of the key.
            names: Display names to filter by.
            name_contains: Substring match on the display name.
            name_regex: Regex match on the display name.
            key_type: Filter to keys of this value type.
            include_archived: If True, include archived keys.
            filter_query: Explicit CEL query.
            order_by: Field and direction to order by.
            limit: Maximum number of keys to return.
            page_size: Results to fetch per request.

        Returns:
            The matching keys.
        """
        # The key list filter exposes the display name as the CEL field `name`.
        filter_parts = self._build_name_cel_filters(
            name=name, names=names, name_contains=name_contains, name_regex=name_regex
        )
        if key_type is not None:
            filter_parts.append(cel.equals("key_type", key_type.value))
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

    async def find_key(self, **kwargs) -> ResourceAttributeKey | None:
        """Find a single key matching the query. Raises if more than one matches."""
        keys = await self.list_keys(**kwargs)
        if len(keys) > 1:
            raise ValueError(f"Multiple ({len(keys)}) resource attribute keys found for query")
        return keys[0] if keys else None

    async def create_key(
        self,
        display_name: str,
        key_type: ResourceAttributeKeyType,
        *,
        description: str = "",
    ) -> ResourceAttributeKey:
        """Create a resource attribute key.

        Args:
            display_name: The human-readable name of the key.
            key_type: The value type of the key.
            description: Optional description.

        Returns:
            The created key.
        """
        key = await self._low_level_client.create_key(
            display_name=display_name, key_type=key_type.value, description=description
        )
        return self._apply_client_to_instance(key)

    async def get_or_create_key(
        self,
        display_name: str,
        key_type: ResourceAttributeKeyType,
        *,
        description: str = "",
    ) -> ResourceAttributeKey:
        """Get a key by display name, creating it if it does not exist.

        Note:
            Display names are not guaranteed unique. If multiple keys share the display
            name, the first active match is returned.
        """
        existing = await self.list_keys(name=display_name, include_archived=False)
        match = next((k for k in existing if k.display_name == display_name), None)
        if match is not None:
            return match
        return await self.create_key(display_name, key_type, description=description)

    async def update_key(
        self,
        key: str | ResourceAttributeKey,
        *,
        display_name: str | None = None,
        description: str | None = None,
    ) -> ResourceAttributeKey:
        """Update a key's display name or description."""
        key_id = key._id_or_error if isinstance(key, ResourceAttributeKey) else key
        updated = await self._low_level_client.update_key(
            key_id, display_name=display_name, description=description
        )
        return self._apply_client_to_instance(updated)

    async def archive_key(self, key: str | ResourceAttributeKey) -> ResourceAttributeKey:
        """Archive a key. Cascades to its enum values and assignments."""
        key_id = key._id_or_error if isinstance(key, ResourceAttributeKey) else key
        await self._low_level_client.archive_key(key_id)
        return await self.get_key(key_id=key_id)

    async def unarchive_key(self, key: str | ResourceAttributeKey) -> ResourceAttributeKey:
        """Unarchive a key. Does not restore its cascaded enum values or assignments."""
        key_id = key._id_or_error if isinstance(key, ResourceAttributeKey) else key
        await self._low_level_client.unarchive_key(key_id)
        return await self.get_key(key_id=key_id)

    async def check_key_archive_impact(self, key: str | ResourceAttributeKey) -> int:
        """Return the number of active assignments archiving this key would affect."""
        key_id = key._id_or_error if isinstance(key, ResourceAttributeKey) else key
        return await self._low_level_client.check_key_archive_impact(key_id)

    async def create_enum_value(
        self,
        key: str | ResourceAttributeKey,
        display_name: str,
        *,
        description: str = "",
    ) -> ResourceAttributeEnumValue:
        """Create a single enum value for a key."""
        key_id = key._id_or_error if isinstance(key, ResourceAttributeKey) else key
        value = await self._low_level_client.create_enum_value(
            key_id=key_id, display_name=display_name, description=description
        )
        return self._apply_client_to_instance(value)

    async def list_enum_values(
        self,
        key: str | ResourceAttributeKey,
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
    ) -> list[ResourceAttributeEnumValue]:
        """List the enum values defined for a key."""
        key_id = key._id_or_error if isinstance(key, ResourceAttributeKey) else key
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
        self, key: str | ResourceAttributeKey, names: list[str]
    ) -> list[ResourceAttributeEnumValue]:
        """Get enum values for a key by name, creating any that don't exist.

        Returns the values in the same order as ``names``.
        """
        key_id = key._id_or_error if isinstance(key, ResourceAttributeKey) else key
        existing = await self.list_enum_values(key_id, include_archived=False)
        by_name = {v.display_name: v for v in existing}
        result: list[ResourceAttributeEnumValue] = []
        for name in names:
            value = by_name.get(name)
            if value is None:
                value = await self.create_enum_value(key_id, name)
                by_name[name] = value
            result.append(value)
        return result

    async def archive_enum_value(
        self,
        enum_value: str | ResourceAttributeEnumValue,
        *,
        replacement: str | ResourceAttributeEnumValue | None = None,
    ) -> int:
        """Archive an enum value, migrating existing assignments to a replacement.

        Returns the number of assignments migrated.
        """
        enum_value_id = _enum_value_id(enum_value)
        replacement_id = _enum_value_id(replacement) if replacement is not None else ""
        return await self._low_level_client.archive_enum_value(
            enum_value_id, replacement_enum_value_id=replacement_id
        )

    async def unarchive_enum_value(
        self, enum_value: str | ResourceAttributeEnumValue
    ) -> ResourceAttributeEnumValue:
        """Unarchive an enum value."""
        enum_value_id = _enum_value_id(enum_value)
        await self._low_level_client.unarchive_enum_value(enum_value_id)
        value = await self._low_level_client.get_enum_value(enum_value_id)
        return self._apply_client_to_instance(value)

    async def assign(
        self,
        key: str | ResourceAttributeKey,
        resources: list[ResourceAttributeEntity | Asset | Channel | Run | str],
        *,
        value: Any,
    ) -> list[ResourceAttribute]:
        """Assign a key's value to resources.

        Args:
            key: The key or key ID to assign. Its ``key_type`` determines how ``value`` is interpreted.
            resources: Resources to assign to. For currently supported resource types, pass
                ``Asset``, ``Channel``, or ``Run`` objects, their IDs, or
                ``ResourceAttributeEntity`` when you already know the resource type.
            value: For ``SET_OF_ENUM``, a list of enum values (or their IDs) that becomes the
                full set on each resource; for ``ENUM``, a single enum value; for ``BOOLEAN``, a
                bool; for ``NUMBER``, an int.

        Returns:
            The created assignments.
        """
        resolved_key = await self._resolve_key(key)
        resolved = await self._resolve_resources(resources)
        create_kwargs = _resource_value_kwargs(resolved_key.key_type, value)

        created: list[ResourceAttribute] = []
        for batch in _chunks(resolved, ASSIGN_BATCH_SIZE):
            attrs = await self._low_level_client.batch_create_resource_attributes(
                key_id=resolved_key._id_or_error, entities=batch, **create_kwargs
            )
            created.extend(attrs)
        return self._apply_client_to_instances(created)

    async def get_assignment(self, *, assignment_id: str) -> ResourceAttribute:
        """Get a single assignment by ID."""
        attr = await self._low_level_client.get_resource_attribute(assignment_id)
        return self._apply_client_to_instance(attr)

    async def list_assignments(
        self,
        *,
        key: str | ResourceAttributeKey | None = None,
        resource: ResourceAttributeEntity | Asset | Channel | Run | str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[ResourceAttribute]:
        """List resource attribute assignments.

        Args:
            key: Filter to assignments of this key.
            resource: Filter to assignments on this resource. When set, other filters are ignored.
                Pass a resource object, resource ID, or ``ResourceAttributeEntity``.
            include_archived: If True, include archived assignments.
            filter_query: Explicit CEL query.
            order_by: Field and direction to order by.
            limit: Maximum number of assignments to return.
            page_size: Results to fetch per request.
        """
        if resource is not None:
            resolved = await self._resolve_resource(resource)
            attrs = await self._low_level_client.list_all_resource_attributes_by_entity(
                entity=resolved,
                include_archived=include_archived,
                max_results=limit,
                **({"page_size": page_size} if page_size is not None else {}),
            )
            return self._apply_client_to_instances(attrs)

        filter_parts = []
        if key is not None:
            key_id = key._id_or_error if isinstance(key, ResourceAttributeKey) else key
            filter_parts.append(cel.equals("resource_attribute_key_id", key_id))
        if filter_query:
            filter_parts.append(filter_query)

        attrs = await self._low_level_client.list_all_resource_attributes(
            query_filter=cel.and_(*filter_parts) or None,
            order_by=order_by,
            include_archived=include_archived,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(attrs)

    async def archive_assignments(self, assignments: list[str | ResourceAttribute]) -> None:
        """Batch archive assignments."""
        ids = [_assignment_id(a) for a in assignments]
        for batch in _chunks(ids, ASSIGN_BATCH_SIZE):
            await self._low_level_client.batch_archive_resource_attributes(batch)

    async def unarchive_assignments(self, assignments: list[str | ResourceAttribute]) -> None:
        """Batch unarchive assignments."""
        ids = [_assignment_id(a) for a in assignments]
        for batch in _chunks(ids, ASSIGN_BATCH_SIZE):
            await self._low_level_client.batch_unarchive_resource_attributes(batch)


def _assignment_id(assignment: str | ResourceAttribute) -> str:
    return assignment._id_or_error if isinstance(assignment, ResourceAttribute) else assignment


def _resource_value_kwargs(key_type: ResourceAttributeKeyType, value: Any) -> dict[str, Any]:
    """Map a user-supplied value to the BatchCreateResourceAttributes value kwargs."""
    if key_type == ResourceAttributeKeyType.SET_OF_ENUM:
        if not isinstance(value, (list, tuple)):
            raise TypeError("SET_OF_ENUM keys require a list of enum values.")
        return {"enum_value_ids": [_enum_value_id(v) for v in value]}
    if key_type == ResourceAttributeKeyType.ENUM:
        if isinstance(value, (list, tuple)):
            if len(value) != 1:
                raise ValueError("ENUM keys require exactly one enum value.")
            value = value[0]
        return {"enum_value_id": _enum_value_id(value)}
    if key_type == ResourceAttributeKeyType.BOOLEAN:
        if not isinstance(value, bool):
            raise TypeError("BOOLEAN keys require a bool value.")
        return {"boolean_value": value}
    if key_type == ResourceAttributeKeyType.NUMBER:
        if isinstance(value, bool) or not isinstance(value, int):
            raise TypeError("NUMBER keys require an int value.")
        return {"number_value": value}
    raise ValueError(f"Cannot assign a value for key type {key_type}.")
