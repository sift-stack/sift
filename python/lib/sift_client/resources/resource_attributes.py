from __future__ import annotations

from typing import TYPE_CHECKING, Any

from sift.resource_attribute.v1.resource_attribute_pb2 import ResourceAttributeEntityIdentifier

from sift_client._internal.low_level_wrappers.resource_attribute import (
    ResourceAttributeLowLevelClient,
)
from sift_client.resources._base import ResourceBase
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.resource_attribute import (
        ResourceAttribute,
        ResourceAttributeEnumValue,
        ResourceAttributeEnumValueUpdate,
        ResourceAttributeKey,
        ResourceAttributeKeyUpdate,
    )


class ResourceAttributesAPIAsync(ResourceBase):
    """High-level API for interacting with resource attributes."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the ResourceAttributesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ResourceAttributeLowLevelClient(
            grpc_client=self.client.grpc_client
        )

    # Resource Attribute Key methods

    async def create_key(
        self,
        display_name: str,
        description: str | None = None,
        key_type: int | None = None,  # ResourceAttributeKeyType enum value
        initial_enum_values: list[dict] | None = None,
    ) -> ResourceAttributeKey:
        """Create a new resource attribute key.

        Args:
            display_name: The display name of the key.
            description: Optional description.
            key_type: The ResourceAttributeKeyType enum value.
            initial_enum_values: Optional list of initial enum values [{display_name: str, description: str}].

        Returns:
            The created ResourceAttributeKey.
        """
        if key_type is None:
            raise ValueError("key_type is required")
        key = await self._low_level_client.create_resource_attribute_key(
            display_name=display_name,
            description=description,
            key_type=key_type,
            initial_enum_values=initial_enum_values,
        )
        return self._apply_client_to_instance(key)

    async def create_or_get_key(
        self,
        display_name: str,
        description: str | None = None,
        key_type: int | None = None,  # ResourceAttributeKeyType enum value
        initial_enum_values: list[dict] | None = None,
    ) -> ResourceAttributeKey:
        """Create a new resource attribute key or get an existing one with the same display name.

        First checks if a key with the given display_name exists. If found, returns the existing key.
        Otherwise, creates a new key with the provided parameters.

        Args:
            display_name: The display name of the key.
            description: Optional description (only used when creating a new key).
            key_type: The ResourceAttributeKeyType enum value (required when creating a new key).
            initial_enum_values: Optional list of initial enum values (only used when creating a new key).

        Returns:
            The existing or newly created ResourceAttributeKey.
        """
        # Search for existing key with the same display_name using exact match filter
        # Note: CEL filter uses 'name' field, not 'display_name'
        filter_query = cel.equals("name", display_name)
        existing_keys = await self.list_keys(filter_query=filter_query, limit=1)
        if existing_keys:
            return existing_keys[0]

        # Key doesn't exist, create it
        if key_type is None:
            raise ValueError("key_type is required when creating a new key")
        return await self.create_key(
            display_name=display_name,
            description=description,
            key_type=key_type,
            initial_enum_values=initial_enum_values,
        )

    async def get_key(self, key_id: str) -> ResourceAttributeKey:
        """Get a resource attribute key by ID.

        Args:
            key_id: The resource attribute key ID.

        Returns:
            The ResourceAttributeKey.
        """
        key = await self._low_level_client.get_resource_attribute_key(key_id)
        return self._apply_client_to_instance(key)

    async def list_keys(
        self,
        *,
        key_id: str | None = None,
        name_contains: str | None = None,
        key_type: int | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[ResourceAttributeKey]:
        """List resource attribute keys with optional filtering.

        Args:
            key_id: Filter by key ID.
            name_contains: Partial display name of the key.
            key_type: Filter by ResourceAttributeKeyType enum value.
            include_archived: If True, include archived keys in results.
            filter_query: Explicit CEL query to filter keys.
            order_by: How to order the retrieved keys.
            limit: How many keys to retrieve. If None, retrieves all matches.

        Returns:
            A list of ResourceAttributeKeys that match the filter.
        """
        filter_parts = []
        if key_id:
            filter_parts.append(cel.equals("resource_attribute_key_id", key_id))
        if name_contains:
            filter_parts.append(cel.contains("display_name", name_contains))
        if key_type is not None:
            filter_parts.append(cel.equals("type", key_type))
        if not include_archived:
            filter_parts.append(cel.equals("is_archived", False))

        if filter_query:
            filter_parts.append(filter_query)  # filter_query is already a CEL expression string

        query_filter = cel.and_(*filter_parts) if filter_parts else None

        keys = await self._low_level_client.list_all_resource_attribute_keys(
            query_filter=query_filter,
            order_by=order_by,
            include_archived=include_archived,
            max_results=limit,
        )
        return self._apply_client_to_instances(keys)

    async def update_key(
        self, key: str | ResourceAttributeKey, update: ResourceAttributeKeyUpdate | dict
    ) -> ResourceAttributeKey:
        """Update a resource attribute key.

        Args:
            key: The ResourceAttributeKey or key ID to update.
            update: Updates to apply to the key.

        Returns:
            The updated ResourceAttributeKey.
        """
        updated_key = await self._low_level_client.update_resource_attribute_key(key, update)
        return self._apply_client_to_instance(updated_key)

    async def archive_key(self, key_id: str) -> None:
        """Archive a resource attribute key.

        Args:
            key_id: The resource attribute key ID to archive.
        """
        await self._low_level_client.archive_resource_attribute_key(key_id)

    async def unarchive_key(self, key_id: str) -> None:
        """Unarchive a resource attribute key.

        Args:
            key_id: The resource attribute key ID to unarchive.
        """
        await self._low_level_client.unarchive_resource_attribute_key(key_id)

    async def batch_archive_keys(self, key_ids: list[str]) -> None:
        """Archive multiple resource attribute keys.

        Args:
            key_ids: List of resource attribute key IDs to archive.
        """
        await self._low_level_client.batch_archive_resource_attribute_keys(key_ids)

    async def batch_unarchive_keys(self, key_ids: list[str]) -> None:
        """Unarchive multiple resource attribute keys.

        Args:
            key_ids: List of resource attribute key IDs to unarchive.
        """
        await self._low_level_client.batch_unarchive_resource_attribute_keys(key_ids)

    # Resource Attribute Enum Value methods

    async def create_enum_value(
        self,
        key_id: str,
        display_name: str,
        description: str | None = None,
    ) -> ResourceAttributeEnumValue:
        """Create a new resource attribute enum value.

        Args:
            key_id: The resource attribute key ID.
            display_name: The display name of the enum value.
            description: Optional description.

        Returns:
            The created ResourceAttributeEnumValue.
        """
        enum_value = await self._low_level_client.create_resource_attribute_enum_value(
            key_id=key_id, display_name=display_name, description=description
        )
        return self._apply_client_to_instance(enum_value)

    async def create_or_get_enum_value(
        self,
        key_id: str,
        display_name: str,
        description: str | None = None,
    ) -> ResourceAttributeEnumValue:
        """Create a new resource attribute enum value or get an existing one with the same key and display name.

        First checks if an enum value with the given key_id and display_name exists. If found,
        returns the existing enum value. Otherwise, creates a new enum value with the provided parameters.

        Args:
            key_id: The resource attribute key ID.
            display_name: The display name of the enum value.
            description: Optional description (only used when creating a new enum value).

        Returns:
            The existing or newly created ResourceAttributeEnumValue.
        """
        # Search for existing enum value with the same key_id and display_name using exact match filter
        # Note: CEL filter uses 'name' field, not 'display_name'
        filter_query = cel.equals("name", display_name)
        existing_enum_values = await self.list_enum_values(key_id=key_id, filter_query=filter_query, limit=1)
        if existing_enum_values:
            return existing_enum_values[0]

        # Enum value doesn't exist, create it
        return await self.create_enum_value(
            key_id=key_id, display_name=display_name, description=description
        )

    async def get_enum_value(self, enum_value_id: str) -> ResourceAttributeEnumValue:
        """Get a resource attribute enum value by ID.

        Args:
            enum_value_id: The resource attribute enum value ID.

        Returns:
            The ResourceAttributeEnumValue.
        """
        enum_value = await self._low_level_client.get_resource_attribute_enum_value(enum_value_id)
        return self._apply_client_to_instance(enum_value)

    async def list_enum_values(
        self,
        key_id: str,
        *,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[ResourceAttributeEnumValue]:
        """List resource attribute enum values for a key with optional filtering.

        Args:
            key_id: The resource attribute key ID.
            include_archived: If True, include archived enum values in results.
            filter_query: Explicit CEL query to filter enum values.
            order_by: How to order the retrieved enum values.
            limit: How many enum values to retrieve. If None, retrieves all matches.

        Returns:
            A list of ResourceAttributeEnumValues that match the filter.
        """
        filter_parts = []
        if not include_archived:
            filter_parts.append(cel.equals("is_archived", False))

        if filter_query:
            filter_parts.append(filter_query)  # filter_query is already a CEL expression string

        query_filter = cel.and_(*filter_parts) if filter_parts else None

        enum_values = await self._low_level_client.list_all_resource_attribute_enum_values(
            key_id=key_id,
            query_filter=query_filter,
            order_by=order_by,
            include_archived=include_archived,
            max_results=limit,
        )
        return self._apply_client_to_instances(enum_values)

    async def update_enum_value(
        self,
        enum_value: str | ResourceAttributeEnumValue,
        update: ResourceAttributeEnumValueUpdate | dict,
    ) -> ResourceAttributeEnumValue:
        """Update a resource attribute enum value.

        Args:
            enum_value: The ResourceAttributeEnumValue or enum value ID to update.
            update: Updates to apply to the enum value.

        Returns:
            The updated ResourceAttributeEnumValue.
        """
        updated_enum_value = await self._low_level_client.update_resource_attribute_enum_value(
            enum_value, update
        )
        return self._apply_client_to_instance(updated_enum_value)

    async def archive_enum_value(self, enum_value_id: str, replacement_enum_value_id: str) -> int:
        """Archive a resource attribute enum value and migrate attributes.

        Args:
            enum_value_id: The enum value ID to archive.
            replacement_enum_value_id: The enum value ID to migrate attributes to.

        Returns:
            The number of resource attributes migrated.
        """
        return await self._low_level_client.archive_resource_attribute_enum_value(
            enum_value_id, replacement_enum_value_id
        )

    async def unarchive_enum_value(self, enum_value_id: str) -> None:
        """Unarchive a resource attribute enum value.

        Args:
            enum_value_id: The resource attribute enum value ID to unarchive.
        """
        await self._low_level_client.unarchive_resource_attribute_enum_value(enum_value_id)

    async def batch_archive_enum_values(self, archival_requests: list[dict]) -> int:
        """Archive multiple resource attribute enum values and migrate attributes.

        Args:
            archival_requests: List of dicts with 'archived_id' and 'replacement_id' keys.

        Returns:
            Total number of resource attributes migrated.
        """
        return await self._low_level_client.batch_archive_resource_attribute_enum_values(
            archival_requests
        )

    async def batch_unarchive_enum_values(self, enum_value_ids: list[str]) -> None:
        """Unarchive multiple resource attribute enum values.

        Args:
            enum_value_ids: List of resource attribute enum value IDs to unarchive.
        """
        await self._low_level_client.batch_unarchive_resource_attribute_enum_values(enum_value_ids)

    # Resource Attribute methods

    async def create(
        self,
        key_id: str,
        entities: str | dict | list[str] | list[dict],
        entity_type: int | None = None,  # ResourceAttributeEntityType enum value
        resource_attribute_enum_value_id: str | None = None,
        boolean_value: bool | None = None,
        number_value: float | None = None,
    ) -> ResourceAttribute | list[ResourceAttribute]:
        """Create a resource attribute for one or more entities.

        Args:
            key_id: The resource attribute key ID.
            entities: Single entity_id (str), single entity dict ({entity_id: str, entity_type: int}),
                     list of entity_ids (list[str]), or list of entity dicts (list[dict]).
            entity_type: Required if entities is str or list[str]. The ResourceAttributeEntityType enum value.
            resource_attribute_enum_value_id: Enum value ID (if applicable).
            boolean_value: Boolean value (if applicable).
            number_value: Number value (if applicable).

        Returns:
            Single ResourceAttribute if entities is a single value, list of ResourceAttributes if it's a list.
        """
        # Handle single entity (str or dict)
        if isinstance(entities, str):
            if entity_type is None:
                raise ValueError("entity_type is required when entities is a string")
            attr = await self._low_level_client.create_resource_attribute(
                key_id=key_id,
                entity_id=entities,
                entity_type=entity_type,
                resource_attribute_enum_value_id=resource_attribute_enum_value_id,
                boolean_value=boolean_value,
                number_value=number_value,
            )
            return self._apply_client_to_instance(attr)
        elif isinstance(entities, dict):
            # Single entity dict
            entity_id = entities["entity_id"]
            entity_type_val = entities.get("entity_type", entity_type)
            if entity_type_val is None:
                raise ValueError("entity_type must be provided in entities dict or as parameter")
            attr = await self._low_level_client.create_resource_attribute(
                key_id=key_id,
                entity_id=entity_id,
                entity_type=entity_type_val,
                resource_attribute_enum_value_id=resource_attribute_enum_value_id,
                boolean_value=boolean_value,
                number_value=number_value,
            )
            return self._apply_client_to_instance(attr)
        elif isinstance(entities, list) and len(entities) > 0:
            # Multiple entities
            if isinstance(entities[0], str):
                # List of entity IDs
                if entity_type is None:
                    raise ValueError("entity_type is required when entities is a list of strings")
                entity_ids: list[str] = entities  # type: ignore[assignment]
                entity_identifiers = [
                    ResourceAttributeEntityIdentifier(entity_id=eid, entity_type=entity_type)  # type: ignore[arg-type]
                    for eid in entity_ids
                ]
            else:
                # List of entity dicts
                entity_dicts: list[dict[str, Any]] = entities  # type: ignore[assignment]
                entity_identifiers = [
                    ResourceAttributeEntityIdentifier(
                        entity_id=str(e["entity_id"]),
                        entity_type=int(e.get("entity_type", entity_type) or 0),  # type: ignore[arg-type]
                    )
                    for e in entity_dicts
                ]
                if entity_type is None and any(e.get("entity_type") is None for e in entity_dicts):
                    raise ValueError(
                        "entity_type must be provided in each entity dict or as parameter"
                    )

            attrs = await self._low_level_client.batch_create_resource_attributes(
                key_id=key_id,
                entities=entity_identifiers,
                resource_attribute_enum_value_id=resource_attribute_enum_value_id,
                boolean_value=boolean_value,
                number_value=number_value,
            )
            return self._apply_client_to_instances(attrs)
        else:
            raise ValueError("entities must be a string, dict, or non-empty list")

    async def get(self, attribute_id: str) -> ResourceAttribute:
        """Get a resource attribute by ID.

        Args:
            attribute_id: The resource attribute ID.

        Returns:
            The ResourceAttribute.
        """
        attr = await self._low_level_client.get_resource_attribute(attribute_id)
        return self._apply_client_to_instance(attr)

    async def list(
        self,
        *,
        entity_id: str | None = None,
        entity_type: int | None = None,
        key_id: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[ResourceAttribute]:
        """List resource attributes with optional filtering.

        Args:
            entity_id: Filter by entity ID.
            entity_type: Filter by ResourceAttributeEntityType enum value.
            key_id: Filter by resource attribute key ID.
            include_archived: If True, include archived attributes in results.
            filter_query: Explicit CEL query to filter attributes.
            order_by: How to order the retrieved attributes.
            limit: How many attributes to retrieve. If None, retrieves all matches.

        Returns:
            A list of ResourceAttributes that match the filter.
        """
        # Use dedicated entity endpoint only for simple case: entity filtering with no other filters
        # (CEL filters don't support entity.entity_id, and the entity endpoint doesn't support order_by/filter_query)
        use_entity_endpoint = (
            entity_id is not None
            and entity_type is not None
            and not key_id
            and not filter_query
            and not order_by
        )

        if use_entity_endpoint:
            # Type narrowing: entity_id and entity_type are guaranteed to be non-None here
            assert entity_id is not None
            assert entity_type is not None
            attrs = await self._low_level_client.list_all_resource_attributes_by_entity(
                entity_id=entity_id,
                entity_type=entity_type,
                include_archived=include_archived,
                max_results=limit,
            )
            return self._apply_client_to_instances(attrs)

        # Otherwise, use CEL filter approach and filter entity in memory if needed
        filter_parts = []
        if key_id:
            filter_parts.append(cel.equals("resource_attribute_key_id", key_id))
        if not include_archived:
            filter_parts.append(cel.equals("is_archived", False))
        if filter_query:
            filter_parts.append(filter_query)

        query_filter = cel.and_(*filter_parts) if filter_parts else None

        attrs = await self._low_level_client.list_all_resource_attributes(
            query_filter=query_filter,
            order_by=order_by,
            include_archived=include_archived,
            max_results=limit,
        )

        # Filter by entity in memory (CEL doesn't support entity.entity_id)
        if entity_id is not None or entity_type is not None:
            if entity_id is not None:
                attrs = [attr for attr in attrs if attr.entity_id == entity_id]
            if entity_type is not None:
                attrs = [attr for attr in attrs if attr.entity_type == entity_type]

        return self._apply_client_to_instances(attrs)

    async def archive(self, attribute_id: str) -> None:
        """Archive a resource attribute.

        Args:
            attribute_id: The resource attribute ID to archive.
        """
        await self._low_level_client.archive_resource_attribute(attribute_id)

    async def unarchive(self, attribute_id: str) -> None:
        """Unarchive a resource attribute.

        Args:
            attribute_id: The resource attribute ID to unarchive.
        """
        await self._low_level_client.unarchive_resource_attribute(attribute_id)

    async def batch_archive(self, attribute_ids: list[str]) -> None:  # type: ignore[valid-type]
        """Archive multiple resource attributes.

        Args:
            attribute_ids: List of resource attribute IDs to archive.
        """
        await self._low_level_client.batch_archive_resource_attributes(attribute_ids)

    async def batch_unarchive(self, attribute_ids: list[str]) -> None:  # type: ignore[valid-type]
        """Unarchive multiple resource attributes.

        Args:
            attribute_ids: List of resource attribute IDs to unarchive.
        """
        await self._low_level_client.batch_unarchive_resource_attributes(attribute_ids)
