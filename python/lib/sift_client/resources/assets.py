from __future__ import annotations

from typing import TYPE_CHECKING,Any
import logging
from datetime import datetime
import re
import inspect

from sift_client._internal.low_level_wrappers.assets import AssetsLowLevelClient
from sift_client.resources.base import ResourceBase
from sift_client.types.asset import Asset
from sift_client.util import cel_utils

if TYPE_CHECKING:
    from sift_client.client import SiftClient

# Configure logging
logger = logging.getLogger(__name__)


class AssetsAPIAsync(ResourceBase):
    """
    High-level API for interacting with assets.

    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """

    def __init__(self, sift_client: "SiftClient"):
        """
        Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = AssetsLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(
        self,
        asset_id: str = None,
        name: str = None,
    ) -> Asset:
        """
        Get an Asset.

        Args:
            asset_id: The ID of the asset.
            name: The name of the asset.

        Returns:
            The Asset.
        """
        if asset_id:
            asset = await self._low_level_client.get_asset(asset_id)

        elif name:
            assets = await self._low_level_client.list_all_assets(query_filter=cel_utils.equals("name", name))
            if len(assets) < 1:
                raise ValueError(f"No asset found with name '{name}'")
            if len(assets) > 1:
                raise ValueError(f"Multiple assets found with name '{name}'") # should not happen
            asset = assets[0]

        else:
            raise ValueError("Either asset_id or name must be provided")

        return self._apply_client_to_instance(asset)

    async def list_(
        self,
        name: str = None,
        name_contains: str = None,
        name_regex: str | re.Pattern = None,
        created_after: datetime = None,
        created_before: datetime = None,
        modified_after: datetime = None,
        modified_before: datetime = None,
        created_by: Any = None,
        modified_by: Any = None,
        tags: list[str] = None,
        include_archived: bool = False,
        filter_query: str = None,
        order_by: str = None,
        limit: int = None,
    ) -> list[Asset]:
        """
        List assets with optional filtering.

        Args:
            name: Exact name of the asset.
            name_contains: Partial name of the asset.
            name_regex: Regular expression string to filter assets by name.
            created_after: Created after this date.
            created_before: Created before this date.
            modified_after: Modified after this date.
            modified_before: Modified before this date.
            created_by: Assets created by this user.
            modified_by: Assets last modified by this user.
            tags: Assets with these tags.
            include_archived: Include archived assets.
            filter_query: Explicit CEL query to filter assets.
            order_by: How to order the retrieved assets. # TODO: tooling for this?
            limit: How many assets to retrieve. If None, retrieves all matches.

        Returns:
            A list of Assets that matches the filter.

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
            if tags:
                # TODO: implement in API
                raise NotImplementedError
            if not include_archived:
                filters.append(cel_utils.equals_null("archived_date"))
            filter_query = cel_utils.and_(*filters)


        assets = await self._low_level_client.list_all_assets(
            query_filter=filter_query,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(assets)

    async def find(self, *args, **kwargs) -> Asset | None:
        """
        Find a single asset matching the given query. Takes the same arguments as `list_`. If more than one asset is found,
        raises an error.

        Args:
            *args:
            **kwargs:

        Returns:
            The Asset found.
        """
        assets = await self.list_(*args, **kwargs)
        if len(assets) > 1:
            raise ValueError("Multiple assets found for query")
        elif len(assets) == 1:
            return assets[0]
        return None

    async def archive(self, asset_id: str = None, asset: Asset = None) -> Asset:
        """
       Archive an asset.

        Args:
            asset_id: The ID of the asset to archive.
            asset: The Asset to archive.

        Raises:
            ClientError: If the request fails.
        """
        if not asset_id and not asset:
            raise ValueError("Either asset_id or asset must be provided")

        await self._low_level_client.delete_asset(asset_id or asset.asset_id)

        return await self.get(asset_id=asset_id or asset.asset_id)


