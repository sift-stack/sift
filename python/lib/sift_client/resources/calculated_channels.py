from __future__ import annotations

import re
from datetime import datetime
from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.calculated_channels import (
    CalculatedChannelsLowLevelClient,
)
from sift_client.resources._base import ResourceBase
from sift_client.types.calculated_channel import (
    CalculatedChannel,
    CalculatedChannelAbstractChannelReference,
    CalculatedChannelAssetConfiguration,
    CalculatedChannelAssetSelection,
    CalculatedChannelConfiguration,
    CalculatedChannelQueryConfiguration,
    CalculatedChannelUpdate,
)
from sift_client.util import cel_utils

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class CalculatedChannelsAPIAsync(ResourceBase):
    """
    High-level API for interacting with calculated channels.

    This class provides a Pythonic, notebook-friendly interface for interacting with the CalculatedChannelsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the CalculatedChannel class from the low-level wrapper, which is a user-friendly
    representation of a calculated channel using standard Python data structures and types.
    """

    def __init__(self, sift_client: "SiftClient"):
        """
        Initialize the CalculatedChannelsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = CalculatedChannelsLowLevelClient(
            grpc_client=self.client.grpc_client
        )

    async def get(
        self,
        *,
        calculated_channel_id: str | None = None,
        client_key: str | None = None,
        organization_id: str | None = None,
    ) -> CalculatedChannel:
        """
        Get a Calculated Channel.

        Args:
            calculated_channel_id: The ID of the calculated channel.
            client_key: The client key of the calculated channel.
            organization_id: The organization ID (required if using client_key and user belongs to multiple organizations).

        Returns:
            The CalculatedChannel.

        Raises:
            ValueError: If neither calculated_channel_id nor client_key is provided.
        """
        if not calculated_channel_id and not client_key:
            raise ValueError("Either calculated_channel_id or client_key must be provided")

        calculated_channel = await self._low_level_client.get_calculated_channel(
            calculated_channel_id=calculated_channel_id,
            client_key=client_key,
            organization_id=organization_id,
        )

        return self._apply_client_to_instance(calculated_channel)

    async def list_(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | None = None,
        modified_by: Any | None = None,
        client_key: str | None = None,
        asset_id: str | None = None,
        asset_name: str | None = None,
        tag_id: str | None = None,
        tag_name: str | None = None,
        version: int | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        organization_id: str | None = None,
    ) -> list[CalculatedChannel]:
        """
        List calculated channels with optional filtering.

        Args:
            name: Exact name of the calculated channel.
            name_contains: Partial name of the calculated channel.
            name_regex: Regular expression string to filter calculated channels by name.
            created_after: Created after this date.
            created_before: Created before this date.
            modified_after: Modified after this date.
            modified_before: Modified before this date.
            created_by: Calculated channels created by this user.
            modified_by: Calculated channels last modified by this user.
            client_key: The client key of the calculated channel.
            asset_id: The asset ID associated with the calculated channel.
            asset_name: The asset name associated with the calculated channel.
            tag_id: The tag ID associated with the calculated channel.
            tag_name: The tag name associated with the calculated channel.
            version: The version of the calculated channel.
            include_archived: Include archived calculated channels.
            filter_query: Explicit CEL query to filter calculated channels.
            order_by: How to order the retrieved calculated channels.
            limit: How many calculated channels to retrieve. If None, retrieves all matches.
            organization_id: The organization ID (required if user belongs to multiple organizations).

        Returns:
            A list of CalculatedChannels that matches the filter.
        """
        if not filter_query:
            filters = []
            if name:
                filters.append(cel_utils.equals("name", name))
            if name_contains:
                filters.append(cel_utils.contains("name", name_contains))
            if name_regex:
                filters.append(cel_utils.match("name", name_regex))
            if created_after:
                filters.append(cel_utils.greater_than("created_date", created_after))
            if created_before:
                filters.append(cel_utils.less_than("created_date", created_before))
            if modified_after:
                filters.append(cel_utils.greater_than("modified_date", modified_after))
            if modified_before:
                filters.append(cel_utils.less_than("modified_date", modified_before))
            if created_by:
                raise NotImplementedError
            if modified_by:
                raise NotImplementedError
            if client_key:
                filters.append(cel_utils.equals("client_key", client_key))
            if asset_id:
                filters.append(cel_utils.equals("asset_id", asset_id))
            if asset_name:
                filters.append(cel_utils.equals("asset_name", asset_name))
            if tag_id:
                filters.append(cel_utils.equals("tag_id", tag_id))
            if tag_name:
                filters.append(cel_utils.equals("tag_name", tag_name))
            if version:
                filters.append(cel_utils.equals("version", version))
            if not include_archived:
                filters.append(cel_utils.equals_null("archived_date"))
            filter_query = cel_utils.and_(*filters)

        calculated_channels = await self._low_level_client.list_all_calculated_channels(
            query_filter=filter_query,
            order_by=order_by,
            max_results=limit,
            organization_id=organization_id,
        )
        return self._apply_client_to_instances(calculated_channels)

    async def find(self, **kwargs) -> CalculatedChannel | None:
        """
        Find a single calculated channel matching the given query. Takes the same arguments as `list_`. If more than one calculated channel is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The CalculatedChannel found or None.
        """
        calculated_channels = await self.list_(**kwargs)
        if len(calculated_channels) > 1:
            raise ValueError("Multiple calculated channels found for query")
        elif len(calculated_channels) == 1:
            return calculated_channels[0]
        return None

    async def create(
        self,
        name: str,
        expression: str,
        channel_references: list[CalculatedChannelAbstractChannelReference],
        *,
        description: str = "",
        units: str | None = None,
        client_key: str | None = None,
        asset_ids: list[str] | None = None,
        tag_ids: list[str] | None = None,
        all_assets: bool = False,
        user_notes: str = "",
    ) -> CalculatedChannel:
        """
        Create a calculated channel.

        Args:
            name: The name of the calculated channel.
            expression: The expression to calculate the value of the calculated channel.
            channel_references: A list of channel references that are used in the expression.
            description: The description of the calculated channel.
            units: The units of the calculated channel.
            client_key: A user-defined unique identifier for the calculated channel.
            asset_ids: A list of asset IDs to make the calculation available for.
            tag_ids: A list of tag IDs to make the calculation available for.
            all_assets: A flag that, when set to True, associates the calculated channel with all assets.
            user_notes: User notes for the calculated channel.

        Returns:
            The created CalculatedChannel.

        Raises:
            ValueError: If asset configuration is invalid.
        """
        # Validate asset configuration
        if all_assets and (asset_ids or tag_ids):
            raise ValueError("Cannot specify both all_assets and asset_ids/tag_ids")
        if not all_assets and not asset_ids and not tag_ids:
            raise ValueError("Must specify either all_assets=True or provide asset_ids/tag_ids")

        # Create asset configuration
        if all_assets:
            asset_config = CalculatedChannelAssetConfiguration(all_assets=True)
        else:
            asset_selection = CalculatedChannelAssetSelection(
                asset_ids=asset_ids or [], tag_ids=tag_ids or []
            )
            asset_config = CalculatedChannelAssetConfiguration(selection=asset_selection)

        # Create query configuration
        query_config = CalculatedChannelQueryConfiguration(
            expression=expression, expression_channel_references=channel_references
        )

        # Create full configuration
        config = CalculatedChannelConfiguration(
            asset_configuration=asset_config, query_configuration=query_config
        )

        # Convert to protobuf (this would need to be implemented)
        # For now, we'll use a placeholder
        proto_config = self._convert_config_to_proto(config)

        (
            calculated_channel,
            inapplicable_assets,
        ) = await self._low_level_client.create_calculated_channel(
            name=name,
            calculated_channel_configuration=proto_config,
            description=description,
            user_notes=user_notes,
            units=units,
            client_key=client_key,
        )

        return self._apply_client_to_instance(calculated_channel)

    async def update(
        self,
        calculated_channel: str | CalculatedChannel,
        update: CalculatedChannelUpdate | dict,
        *,
        user_notes: str | None = None,
    ) -> CalculatedChannel:
        """
        Update a Calculated Channel.

        Args:
            calculated_channel: The CalculatedChannel or calculated_channel_id to update.
            update: Updates to apply to the CalculatedChannel.
            user_notes: User notes for the update.

        Returns:
            The updated CalculatedChannel.
        """
        calculated_channel_id = (
            calculated_channel.calculated_channel_id
            if isinstance(calculated_channel, CalculatedChannel)
            else calculated_channel
        )

        if isinstance(update, dict):
            update = CalculatedChannelUpdate.model_validate(update)

        update.resource_id = calculated_channel_id

        # Convert to protobuf and field mask
        proto_calculated_channel, update_mask = update.to_proto_with_mask()

        (
            updated_calculated_channel,
            inapplicable_assets,
        ) = await self._low_level_client.update_calculated_channel(
            update=update, user_notes=user_notes
        )

        return self._apply_client_to_instance(updated_calculated_channel)

    async def list_versions(
        self,
        calculated_channel_id: str | None = None,
        client_key: str | None = None,
        organization_id: str | None = None,
        *,
        page_size: int | None = None,
        page_token: str | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[CalculatedChannel]:
        """
        List versions of a calculated channel.

        Args:
            calculated_channel_id: The ID of the calculated channel.
            client_key: The client key of the calculated channel.
            organization_id: The organization ID.
            page_size: The number of results to return per page.
            page_token: The page token for pagination.
            filter_query: The CEL query filter.
            order_by: The field to order by.
            limit: How many versions to retrieve. If None, retrieves all matches.

        Returns:
            A list of CalculatedChannel versions.

        Raises:
            ValueError: If neither calculated_channel_id nor client_key is provided.
        """
        if not calculated_channel_id and not client_key:
            raise ValueError("Either calculated_channel_id or client_key must be provided")

        versions, next_page_token = await self._low_level_client.list_calculated_channel_versions(
            calculated_channel_id=calculated_channel_id,
            client_key=client_key,
            organization_id=organization_id,
            page_size=page_size,
            page_token=page_token,
            query_filter=filter_query,
            order_by=order_by,
        )

        # Handle pagination if limit is specified
        if limit and len(versions) < limit:
            while next_page_token and len(versions) < limit:
                (
                    more_versions,
                    next_page_token,
                ) = await self._low_level_client.list_calculated_channel_versions(
                    calculated_channel_id=calculated_channel_id,
                    client_key=client_key,
                    organization_id=organization_id,
                    page_size=min(page_size or 50, limit - len(versions)),
                    page_token=next_page_token,
                    query_filter=filter_query,
                    order_by=order_by,
                )
                versions.extend(more_versions)

        return self._apply_client_to_instances(versions[:limit] if limit else versions)

    def _convert_config_to_proto(self, config: CalculatedChannelConfiguration) -> Any:
        """
        Convert a CalculatedChannelConfiguration to protobuf format.

        This is a placeholder method that would need to be implemented to convert
        the Python configuration objects to protobuf messages.

        Args:
            config: The CalculatedChannelConfiguration to convert.

        Returns:
            The protobuf configuration object.
        """
        # TODO: Implement conversion to protobuf
        raise NotImplementedError("Proto conversion not yet implemented")
