from __future__ import annotations

from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.assets import AssetsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset, AssetUpdate
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.tag import Tag


class AssetsAPIAsync(ResourceBase):
    """High-level API for interacting with assets.

    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = AssetsLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(
        self,
        *,
        asset_id: str | None = None,
        name: str | None = None,
    ) -> Asset:
        """Get an Asset.

        Args:
            asset_id: The ID of the asset.
            name: The name of the asset.

        Returns:
            The Asset.
        """
        asset: Asset | None
        if asset_id is not None:
            asset = await self._low_level_client.get_asset(asset_id)
        elif name is not None:
            asset = await self.find(name=name)
            if asset is None:
                raise ValueError(f"No asset found with name '{name}'")
        else:
            raise ValueError("Either asset_id or name must be provided")

        return self._apply_client_to_instance(asset)

    async def list_(
        self,
        *,
        # name
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        asset_ids: list[str] | None = None,
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
    ) -> list[Asset]:
        """List assets with optional filtering.

        Args:
            name: Exact name of the asset.
            names: List of asset names to filter by.
            name_contains: Partial name of the asset.
            name_regex: Regular expression to filter assets by name.
            asset_ids: Filter to assets with any of these Ids.
            created_after: Filter assets created after this datetime.
            created_before: Filter assets created before this datetime.
            modified_after: Filter assets modified after this datetime.
            modified_before: Filter assets modified before this datetime.
            created_by: Filter assets created by this User or user ID.
            modified_by: Filter assets last modified by this User or user ID.
            tags: Filter assets with any of these Tags or tag names.
            metadata: Filter assets by metadata criteria.
            description_contains: Partial description of the asset.
            include_archived: If True, include archived assets in results.
            filter_query: Explicit CEL query to filter assets.
            order_by: Field and direction to order results by.
            limit: Maximum number of assets to return. If None, returns all matches.

        Returns:
            A list of Asset objects that match the filter criteria.
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
        if asset_ids:
            filter_parts.append(cel.in_("asset_id", asset_ids))
        filter_query = cel.and_(*filter_parts)

        assets = await self._low_level_client.list_all_assets(
            query_filter=filter_query or None,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(assets)

    async def find(self, **kwargs) -> Asset | None:
        """Find a single asset matching the given query. Takes the same arguments as `list_`. If more than one asset is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Asset found or None.
        """
        assets = await self.list_(**kwargs)
        if len(assets) > 1:
            raise ValueError(f"Multiple ({len(assets)}) assets found for query")
        elif len(assets) == 1:
            return assets[0]
        return None

    async def update(self, asset: str | Asset, update: AssetUpdate | dict) -> Asset:
        """Update an Asset.

        Args:
            asset: The Asset or asset ID to update.
            update: Updates to apply to the Asset.

        Returns:
            The updated Asset.

        """
        asset_id = asset._id_or_error if isinstance(asset, Asset) else asset
        if isinstance(update, dict):
            update = AssetUpdate.model_validate(update)
        update.resource_id = asset_id
        asset = await self._low_level_client.update_asset(update=update)
        return self._apply_client_to_instance(asset)

    async def archive(self, asset: str | Asset, *, archive_runs: bool = False) -> Asset:
        """Archive an asset.

        Args:
             asset: The Asset or asset ID to archive.
             archive_runs: If True, archive all Runs associated with the Asset.

        Returns:
             The archived Asset.
        """
        asset_id = asset._id_or_error if isinstance(asset, Asset) else asset

        await self._low_level_client.archive_asset(asset_id, archive_runs=archive_runs)

        return await self.get(asset_id=asset_id)

    async def unarchive(self, asset: str | Asset) -> Asset:
        """Unarchive an asset.

        Args:
             asset: The Asset or asset ID to unarchive.

        Returns:
             The unarchived Asset.
        """
        return await self.update(asset, AssetUpdate(is_archived=False))
