from __future__ import annotations

from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.calculated_channels import (
    CalculatedChannelsLowLevelClient,
)
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.calculated_channel import (
    CalculatedChannel,
    CalculatedChannelCreate,
    CalculatedChannelUpdate,
)
from sift_client.sift_types.run import Run
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.tag import Tag


class CalculatedChannelsAPIAsync(ResourceBase):
    """High-level API for interacting with calculated channels.

    This class provides a Pythonic, notebook-friendly interface for interacting with the CalculatedChannelsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the CalculatedChannel class from the low-level wrapper, which is a user-friendly
    representation of a calculated channel using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the CalculatedChannelsAPI.

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
    ) -> CalculatedChannel:
        """Get a Calculated Channel.

        Args:
            calculated_channel_id: The ID of the calculated channel.
            client_key: The client key of the calculated channel.

        Returns:
            The CalculatedChannel.

        Raises:
            ValueError: If neither calculated_channel_id nor client_key is provided.
        """
        calculated_channel = await self._low_level_client.get_calculated_channel(
            calculated_channel_id=calculated_channel_id,
            client_key=client_key,
        )

        return self._apply_client_to_instance(calculated_channel)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        calculated_channel_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
        # created/modified ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        # created/modified users
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        # tags
        tags: list[Any] | list[str] | list[Tag] | None = None,
        # metadata
        metadata: list[Any] | None = None,
        # calculated channel specific
        asset: Asset | str | None = None,
        run: Run | str | None = None,
        version: int | None = None,
        # common filters
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[CalculatedChannel]:
        """List calculated channels with optional filtering. This will return the latest version. To find all versions, use `list_versions`.

        Args:
            name: Exact name of the calculated channel.
            names: List of calculated channel names to filter by.
            name_contains: Partial name of the calculated channel.
            name_regex: Regular expression string to filter calculated channels by name.
            calculated_channel_ids: Filter to calculated channels with any of these IDs.
            client_keys: Filter to calculated channels with any of these client keys.
            created_after: Created after this date.
            created_before: Created before this date.
            modified_after: Modified after this date.
            modified_before: Modified before this date.
            created_by: Calculated channels created by this user.
            modified_by: Calculated channels last modified by this user.
            tags: Filter calculated channels with any of these Tags or tag names.
            metadata: Filter calculated channels by metadata criteria.
            asset: Filter calculated channels associated with this Asset or asset ID.
            run: Filter calculated channels associated with this Run or run ID.
            version: The version of the calculated channel.
            description_contains: Partial description of the calculated channel.
            include_archived: Include archived calculated channels.
            filter_query: Explicit CEL query to filter calculated channels.
            order_by: How to order the retrieved calculated channels.
            limit: How many calculated channels to retrieve. If None, retrieves all matches.

        Returns:
            A list of CalculatedChannels that matches the filter.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
                modified_by=modified_by,
            ),
            *self._build_tags_metadata_cel_filters(tag_names=tags, metadata=metadata),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                include_archived=include_archived,
                filter_query=filter_query,
            ),
        ]
        if calculated_channel_ids:
            filter_parts.append(cel.in_("calculated_channel_id", calculated_channel_ids))
        if client_keys:
            filter_parts.append(cel.in_("client_key", client_keys))
        if asset:
            asset_id = asset._id_or_error if isinstance(asset, Asset) else asset
            filter_parts.append(cel.equals("asset_id", asset_id))
        if run:
            run_id = run._id_or_error if isinstance(run, Run) else run
            filter_parts.append(cel.equals("run_id", run_id))
        if version:
            filter_parts.append(cel.equals("version", version))

        query_filter = cel.and_(*filter_parts)

        calculated_channels = await self._low_level_client.list_all_calculated_channels(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(calculated_channels)

    async def find(self, **kwargs) -> CalculatedChannel | None:
        """Find a single calculated channel matching the given query. Takes the same arguments as `list` but handles checking for multiple matches.
        Will raise an error if multiple calculated channels are found.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The CalculatedChannel found or None.
        """
        calculated_channels = await self.list_(**kwargs)
        if len(calculated_channels) > 1:
            raise ValueError(
                f"Multiple ({len(calculated_channels)}) calculated channels found for query"
            )
        elif len(calculated_channels) == 1:
            return calculated_channels[0]
        return None

    async def create(
        self,
        create: CalculatedChannelCreate | dict,
    ) -> CalculatedChannel:
        """Create a calculated channel.

        Args:
            create: A CalculatedChannelCreate object or dictionary with configuration for the new calculated channel.
                   This should include properties like name, expression, channel_references, etc.

        Returns:
            The created CalculatedChannel.

        """
        if isinstance(create, dict):
            create = CalculatedChannelCreate.model_validate(create)

        created_calc_channel, _ = await self._low_level_client.create_calculated_channel(
            create=create
        )
        return self._apply_client_to_instance(created_calc_channel)

    async def update(
        self,
        calculated_channel: CalculatedChannel | str,
        update: CalculatedChannelUpdate | dict,
        *,
        user_notes: str | None = None,
    ) -> CalculatedChannel:
        """Update a Calculated Channel.

        Args:
            calculated_channel: The CalculatedChannel or id of the CalculatedChannel to update.
            update: Updates to apply to the CalculatedChannel.
            user_notes: User notes for the update.

        Returns:
            The updated CalculatedChannel.
        """
        calculated_channel_id = (
            calculated_channel.id_
            if isinstance(calculated_channel, CalculatedChannel)
            else calculated_channel
        )

        if isinstance(update, dict):
            update = CalculatedChannelUpdate.model_validate(update)

        update.resource_id = calculated_channel_id

        (
            updated_calculated_channel,
            inapplicable_assets,
        ) = await self._low_level_client.update_calculated_channel(
            update=update, user_notes=user_notes
        )

        return self._apply_client_to_instance(updated_calculated_channel)

    async def archive(self, calculated_channel: str | CalculatedChannel) -> CalculatedChannel:
        """Archive a calculated channel.

        Args:
            calculated_channel: The id or CalculatedChannel object of the calculated channel to archive.

        Returns:
            The archived CalculatedChannel.
        """
        return await self.update(
            calculated_channel=calculated_channel, update=CalculatedChannelUpdate(is_archived=True)
        )

    async def unarchive(self, calculated_channel: str | CalculatedChannel) -> CalculatedChannel:
        """Unarchive a calculated channel.

        Args:
            calculated_channel: The id or CalculatedChannel object of the calculated channel to unarchive.

        Returns:
            The unarchived CalculatedChannel.
        """
        return await self.update(
            calculated_channel=calculated_channel, update=CalculatedChannelUpdate(is_archived=False)
        )

    async def list_versions(
        self,
        *,
        # self ids
        calculated_channel: CalculatedChannel | str | None = None,
        client_key: str | None = None,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # created/modified ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        # created/modified users
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        # tags
        tags: list[Any] | list[str] | list[Tag] | None = None,
        # metadata
        metadata: list[Any] | None = None,
        # common filters
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[CalculatedChannel]:
        """List versions of a calculated channel.

        Args:
            calculated_channel: The CalculatedChannel or ID of the calculated channel to get versions for.
            client_key: The client key of the calculated channel.
            name: Exact name of the calculated channel.
            names: List of calculated channel names to filter by.
            name_contains: Partial name of the calculated channel.
            name_regex: Regular expression string to filter calculated channels by name.
            created_after: Filter versions created after this datetime.
            created_before: Filter versions created before this datetime.
            modified_after: Filter versions modified after this datetime.
            modified_before: Filter versions modified before this datetime.
            created_by: Filter versions created by this user or user ID.
            modified_by: Filter versions modified by this user or user ID.
            tags: Filter versions with any of these Tags or tag names.
            metadata: Filter versions by metadata criteria.
            description_contains: Partial description of the calculated channel.
            include_archived: Include archived versions.
            filter_query: Explicit CEL query to filter versions.
            order_by: How to order the retrieved versions.
            limit: Maximum number of versions to return. If None, returns all matches.

        Returns:
            A list of CalculatedChannel versions that match the filter criteria.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
                modified_by=modified_by,
            ),
            *self._build_tags_metadata_cel_filters(tag_names=tags, metadata=metadata),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                include_archived=include_archived,
                filter_query=filter_query,
            ),
        ]
        query_filter = cel.and_(*filter_parts)

        versions = await self._low_level_client.list_all_calculated_channel_versions(
            client_key=client_key,
            calculated_channel_id=calculated_channel.id_
            if isinstance(calculated_channel, CalculatedChannel)
            else calculated_channel,
            query_filter=query_filter or None,
            order_by=order_by,
            limit=limit,
        )

        return self._apply_client_to_instances(versions)
