from __future__ import annotations

from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.assets import AssetsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset, AssetUpdate
from sift_client.util import cel_utils

if TYPE_CHECKING:
    import re
    from datetime import datetime

    from sift_client.client import SiftClient


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
        if asset_id:
            asset = await self._low_level_client.get_asset(asset_id)
        elif name:
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
        tags: list[Any | str] | None = None,
        # metadata
        metadata: list[Any] | None = None,
        # common filters
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Asset]:
        """List assets with optional filtering.

        Args:
            name: Exact name of the asset.
            name_contains: Partial name of the asset.
            name_regex: Regular expression to filter assets by name.
            asset_ids: List of asset IDs to filter by.
            created_after: Filter assets created after this datetime.
            created_before: Filter assets created before this datetime.
            modified_after: Filter assets modified after this datetime.
            modified_before: Filter assets modified before this datetime.
            created_by: Filter assets created by this User or user ID.
            modified_by: Filter assets last modified by this User or user ID.
            tags: Filter assets with these Tags or tag names.
            metadata: Filter assets by metadata criteria.
            include_archived: If True, include archived assets in results.
            filter_query: Explicit CEL query to filter assets.
            order_by: Field and direction to order results by.
            limit: Maximum number of assets to return. If None, returns all matches.

        Returns:
            A list of Asset objects that match the filter criteria.
        """
        filters = []
        if name:
            filters.append(cel_utils.equals("name", name))
        if name_contains:
            filters.append(cel_utils.contains("name", name_contains))
        if name_regex:
            filters.append(cel_utils.match("name", name_regex))
        if asset_ids:
            filters.append(cel_utils.in_("asset_id", asset_ids))
        if created_after:
            filters.append(cel_utils.greater_than("created_date", created_after))
        if created_before:
            filters.append(cel_utils.less_than("created_date", created_before))
        if modified_after:
            filters.append(cel_utils.greater_than("modified_date", modified_after))
        if modified_before:
            filters.append(cel_utils.less_than("modified_date", modified_before))
        if created_by:
            if isinstance(created_by, str):
                filters.append(cel_utils.equals("created_by_user_id", created_by))
            else:
                raise NotImplementedError
        if modified_by:
            if isinstance(modified_by, str):
                filters.append(cel_utils.equals("modified_by_user_id", created_by))
            else:
                raise NotImplementedError
        if tags:
            if all(isinstance(tag, str) for tag in tags):
                filters.append(cel_utils.in_("tag_name", tags))
            else:
                raise NotImplementedError
        if metadata:
            raise NotImplementedError
        if not include_archived:
            filters.append(cel_utils.equals_null("archived_date"))
        if filter_query:
            filters.append(filter_query)
        filter_query = cel_utils.and_(*filters)

        assets = await self._low_level_client.list_all_assets(
            query_filter=filter_query,
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

    async def archive(self, asset: str | Asset, *, archive_runs: bool = False) -> Asset:
        """Archive an asset.

        Args:
             asset: The Asset or asset ID to archive.
             archive_runs: If True, archive all Runs associated with the Asset.

        Returns:
             The archived Asset.
        """
        asset_id = asset.id_ or "" if isinstance(asset, Asset) else asset

        await self._low_level_client.delete_asset(asset_id or "", archive_runs=archive_runs)

        return await self.get(asset_id=asset_id)

    async def update(self, asset: str | Asset, update: AssetUpdate | dict) -> Asset:
        """Update an Asset.

        Args:
            asset: The Asset or asset ID to update.
            update: Updates to apply to the Asset.

        Returns:
            The updated Asset.

        """
        asset_id = asset.id_ or "" if isinstance(asset, Asset) else asset
        if isinstance(update, dict):
            update = AssetUpdate.model_validate(update)
        update.resource_id = asset_id
        asset = await self._low_level_client.update_asset(update=update)
        return self._apply_client_to_instance(asset)
