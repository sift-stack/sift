from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.user_attributes import UserAttributesLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.user_attributes import (
        UserAttributeKey,
        UserAttributeKeyUpdate,
        UserAttributeValue,
    )


class UserAttributesAPIAsync(ResourceBase):
    """High-level API for interacting with user attributes."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the UserAttributesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = UserAttributesLowLevelClient(grpc_client=self.client.grpc_client)

    # User Attribute Key methods

    async def create_key(
        self,
        name: str,
        description: str | None = None,
        value_type: int | None = None,  # UserAttributeValueType enum value
    ) -> UserAttributeKey:
        """Create a new user attribute key.

        Args:
            name: The name of the user attribute key.
            description: Optional description.
            value_type: The UserAttributeValueType enum value.

        Returns:
            The created UserAttributeKey.
        """
        if value_type is None:
            raise ValueError("value_type is required")
        key = await self._low_level_client.create_user_attribute_key(
            name=name, description=description, value_type=value_type
        )
        return self._apply_client_to_instance(key)

    async def create_or_get_key(
        self,
        name: str,
        description: str | None = None,
        value_type: int | None = None,  # UserAttributeValueType enum value
        organization_id: str | None = None,
    ) -> UserAttributeKey:
        """Create a new user attribute key or get an existing one with the same name.

        First checks if a key with the given name exists in the organization (or all accessible
        organizations if organization_id is not provided). If found, returns the existing key.
        Otherwise, creates a new key with the provided parameters.

        Args:
            name: The name of the user attribute key.
            description: Optional description (only used when creating a new key).
            value_type: The UserAttributeValueType enum value (required when creating a new key).
            organization_id: Optional organization ID to filter the search. If not provided,
                           searches across all accessible organizations.

        Returns:
            The existing or newly created UserAttributeKey.
        """
        # Search for existing key with the same name
        existing_keys = await self.list_keys(name=name, organization_id=organization_id, limit=1)
        if existing_keys:
            return existing_keys[0]

        # Key doesn't exist, create it
        if value_type is None:
            raise ValueError("value_type is required when creating a new key")
        return await self.create_key(name=name, description=description, value_type=value_type)

    async def get_key(self, key_id: str) -> UserAttributeKey:
        """Get a user attribute key by ID.

        Args:
            key_id: The user attribute key ID.

        Returns:
            The UserAttributeKey.
        """
        key = await self._low_level_client.get_user_attribute_key(key_id)
        return self._apply_client_to_instance(key)

    async def list_keys(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        key_id: str | None = None,
        organization_id: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[UserAttributeKey]:
        """List user attribute keys with optional filtering.

        Args:
            name: Exact name of the key.
            name_contains: Partial name of the key.
            key_id: Filter by key ID.
            organization_id: Filter by organization ID.
            include_archived: If True, include archived keys in results.
            filter_query: Explicit CEL query to filter keys.
            order_by: How to order the retrieved keys.
            limit: How many keys to retrieve. If None, retrieves all matches.

        Returns:
            A list of UserAttributeKeys that match the filter.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, name_contains=name_contains, name_regex=None, names=None
            ),
            *self._build_common_cel_filters(
                filter_query=filter_query,
            ),
        ]

        if key_id:
            filter_parts.append(cel.equals("user_attribute_key_id", key_id))
        if organization_id:
            filter_parts.append(cel.equals("organization_id", organization_id))

        query_filter = cel.and_(*filter_parts) if filter_parts else None

        keys = await self._low_level_client.list_all_user_attribute_keys(
            query_filter=query_filter,
            order_by=order_by,
            organization_id=organization_id,
            include_archived=include_archived,
            max_results=limit,
        )
        return self._apply_client_to_instances(keys)

    async def update_key(
        self, key: str | UserAttributeKey, update: UserAttributeKeyUpdate | dict
    ) -> UserAttributeKey:
        """Update a user attribute key.

        Args:
            key: The UserAttributeKey or key ID to update.
            update: Updates to apply to the key.

        Returns:
            The updated UserAttributeKey.
        """
        updated_key = await self._low_level_client.update_user_attribute_key(key, update)
        return self._apply_client_to_instance(updated_key)

    async def archive_key(self, key_id: str) -> None:
        """Archive a user attribute key.

        Args:
            key_id: The user attribute key ID to archive.
        """
        await self._low_level_client.archive_user_attribute_keys([key_id])

    async def unarchive_key(self, key_id: str) -> None:
        """Unarchive a user attribute key.

        Args:
            key_id: The user attribute key ID to unarchive.
        """
        await self._low_level_client.unarchive_user_attribute_keys([key_id])

    async def batch_archive_keys(self, key_ids: list[str]) -> None:
        """Archive multiple user attribute keys.

        Args:
            key_ids: List of user attribute key IDs to archive.
        """
        await self._low_level_client.archive_user_attribute_keys(key_ids)

    async def batch_unarchive_keys(self, key_ids: list[str]) -> None:
        """Unarchive multiple user attribute keys.

        Args:
            key_ids: List of user attribute key IDs to unarchive.
        """
        await self._low_level_client.unarchive_user_attribute_keys(key_ids)

    # User Attribute Value methods

    async def create_value(
        self,
        key_id: str,
        user_ids: str | list[str],
        string_value: str | None = None,
        number_value: float | None = None,
        boolean_value: bool | None = None,
    ) -> UserAttributeValue | list[UserAttributeValue]:
        """Create a user attribute value for one or more users.

        Args:
            key_id: The user attribute key ID.
            user_ids: Single user ID (str) or list of user IDs (list[str]).
            string_value: String value (if applicable).
            number_value: Number value (if applicable).
            boolean_value: Boolean value (if applicable).

        Returns:
            Single UserAttributeValue if user_ids is a string, list of UserAttributeValues if it's a list.
        """
        if isinstance(user_ids, str):
            # Single user
            value = await self._low_level_client.create_user_attribute_value(
                key_id=key_id,
                user_id=user_ids,
                string_value=string_value,
                number_value=number_value,
                boolean_value=boolean_value,
            )
            return self._apply_client_to_instance(value)
        else:
            # Multiple users - use batch
            values = await self._low_level_client.batch_create_user_attribute_value(
                key_id=key_id,
                user_ids=user_ids,
                string_value=string_value,
                number_value=number_value,
                boolean_value=boolean_value,
            )
            return self._apply_client_to_instances(values)

    async def create_or_get_value(
        self,
        key_id: str,
        user_id: str,
        string_value: str | None = None,
        number_value: float | None = None,
        boolean_value: bool | None = None,
    ) -> UserAttributeValue:
        """Create a user attribute value or get an existing one for the given key and user.

        First checks if a value with the given key_id and user_id exists. If found, returns the
        existing value. Otherwise, creates a new value with the provided parameters.

        Args:
            key_id: The user attribute key ID.
            user_id: The user ID.
            string_value: String value (if applicable, only used when creating a new value).
            number_value: Number value (if applicable, only used when creating a new value).
            boolean_value: Boolean value (if applicable, only used when creating a new value).

        Returns:
            The existing or newly created UserAttributeValue.
        """
        # Search for existing value with the same key_id and user_id
        existing_values = await self.list_values(key_id=key_id, user_id=user_id, limit=1)
        if existing_values:
            return existing_values[0]

        # Value doesn't exist, create it
        return await self.create_value(
            key_id=key_id,
            user_ids=user_id,
            string_value=string_value,
            number_value=number_value,
            boolean_value=boolean_value,
        )

    async def get_value(self, value_id: str) -> UserAttributeValue:
        """Get a user attribute value by ID.

        Args:
            value_id: The user attribute value ID.

        Returns:
            The UserAttributeValue.
        """
        value = await self._low_level_client.get_user_attribute_value(value_id)
        return self._apply_client_to_instance(value)

    async def list_values(
        self,
        *,
        key_id: str | None = None,
        user_id: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[UserAttributeValue]:
        """List user attribute values with optional filtering.

        Args:
            key_id: Filter by user attribute key ID.
            user_id: Filter by user ID.
            include_archived: If True, include archived values in results.
            filter_query: Explicit CEL query to filter values.
            order_by: How to order the retrieved values.
            limit: How many values to retrieve. If None, retrieves all matches.

        Returns:
            A list of UserAttributeValues that match the filter.
        """
        filter_parts = []
        if key_id:
            filter_parts.append(cel.equals("user_attribute_key_id", key_id))
        if user_id:
            filter_parts.append(cel.equals("user_id", user_id))

        if filter_query:
            filter_parts.append(filter_query)  # filter_query is already a CEL expression string

        query_filter = cel.and_(*filter_parts) if filter_parts else None

        values = await self._low_level_client.list_all_user_attribute_values(
            query_filter=query_filter,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(values)

    async def archive_value(self, value_id: str) -> None:
        """Archive a user attribute value.

        Args:
            value_id: The user attribute value ID to archive.
        """
        await self._low_level_client.archive_user_attribute_values([value_id])

    async def unarchive_value(self, value_id: str) -> None:
        """Unarchive a user attribute value.

        Args:
            value_id: The user attribute value ID to unarchive.
        """
        await self._low_level_client.unarchive_user_attribute_values([value_id])

    async def batch_archive_values(self, value_ids: list[str]) -> None:
        """Archive multiple user attribute values.

        Args:
            value_ids: List of user attribute value IDs to archive.
        """
        await self._low_level_client.archive_user_attribute_values(value_ids)

    async def batch_unarchive_values(self, value_ids: list[str]) -> None:
        """Unarchive multiple user attribute values.

        Args:
            value_ids: List of user attribute value IDs to unarchive.
        """
        await self._low_level_client.unarchive_user_attribute_values(value_ids)
