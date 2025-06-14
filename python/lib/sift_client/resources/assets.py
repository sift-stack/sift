from __future__ import annotations

import logging
from datetime import datetime
import re

from google.protobuf.field_mask_pb2 import FieldMask

from sift_client._internal.low_level_wrappers.assets import AssetsLowLevelClient
from sift_client.errors import ClientError, RequestError
from sift_client.transport import GrpcClient, WithGrpcClient
from sift_client.types.asset import Asset
from sift_client.util import cel_utils

# Configure logging
logger = logging.getLogger(__name__)


class AssetsAPIAsync(WithGrpcClient):
    """
    High-level API for interacting with assets.

    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """

    def __init__(self, grpc_client: GrpcClient):
        """
        Initialize the AssetsAPI.

        Args:
            grpc_client: The gRPC client to use for making API calls.
        """
        super().__init__(grpc_client)
        self._low_level_client = AssetsLowLevelClient(self._grpc_client)

    async def get(
        self,
        asset_id: str = None,
        name: str = None,
    ) -> Asset:
        """
        Get an asset by ID.

        Args:
            asset_id: The ID of the asset.
            name: The name of the asset.

        Returns:
            The Asset.
        """
        if asset_id:
            return await self._low_level_client.get_asset(asset_id)

        if name:
            assets = await self._low_level_client.list_all_assets(query_filter=cel_utils.equals("name", name))
            if len(assets) < 1:
                raise ValueError(f"No asset found with name '{name}'")
            if len(assets) > 1:
                raise ValueError(f"Multiple assets found with name '{name}'") # should not happen
            return assets[0]

        raise ValueError("Either asset_id or name must be provided")

    async def list_(
        self,
        name: str = None,
        name_contains: str = None,
        name_regex: str | re.Pattern = None,
        created_after: datetime = None,
        created_before: datetime = None,
        modified_after: datetime = None,
        modified_before: datetime = None,
        # created_by: User = None,
        # modified_by: User = None,
        tags: list[str] = None,
        filter_query: str = None,
        order_by: str = None,
    ) -> list[Asset]:
        """
        List assets.

        Args:
            order_by: How to order the retrieved assets.

        Returns:
            A list of Assets.

        """
        if not filter_query:
            filters = []
            if name:
                filters.append(cel_utils.equals("name", name))
            if name_contains:
                filters.append(cel_utils.contains("name", name_contains))
            if name_regex:
                filters.append(cel_utils.match("name", name_regex))
            # if created_after:
            #     filter_query += f"created_after='{created_after.isoformat()}'"
            # if created_before:
            #     filter_query += f"created_before='{created_before.isoformat()}'"
            # if modified_after:
            filter_query = cel_utils.and_(*filters)


        return await self._low_level_client.list_all_assets(
            query_filter=filter_query,
            order_by=order_by,
        )


    def update_tags(self, asset_id: str, tags: list[str]) -> Asset:
        """
        Update the tags of an asset.

        Args:
            asset_id: The ID of the asset to update.
            tags: The new tags for the asset.

        Returns:
            The updated asset.

        Raises:
            ClientError: If the request fails.
        """
        try:
            # Get the current asset
            asset = self._low_level_client.get_asset(asset_id)

            # Update the tags
            asset.tags[:] = tags

            # Create the update mask
            update_mask = FieldMask(paths=["tags"])

            # Update the asset
            return self._low_level_client.update_asset(asset, update_mask)
        except ClientError:
            raise
        except Exception as e:
            logger.error(f"Unexpected error updating asset tags: {e}")
            raise RequestError(f"Failed to update asset tags: {e}")

    def delete(self, asset_id: str) -> None:
        """
        Delete an asset.

        Args:
            asset_id: The ID of the asset to delete.

        Raises:
            ClientError: If the request fails.
        """
        try:
            self._low_level_client.delete_asset(asset_id)
        except ClientError:
            raise
        except Exception as e:
            logger.error(f"Unexpected error deleting asset: {e}")
            raise RequestError(f"Failed to delete asset: {e}")

    def find_by_name(self, name: str) -> Asset | None:
        """
        Find an asset by name.

        Args:
            name: The name of the asset to find.

        Returns:
            The asset, or None if not found.

        Raises:
            ClientError: If the request fails.
            ValueError: If multiple assets are found with the same name.
        """
        try:
            assets = self._low_level_client.get_assets_by_name([name])

            if not assets:
                return None

            if len(assets) > 1:
                raise ValueError(f"Multiple assets found with name '{name}'")

            return assets[0]
        except ClientError:
            raise
        except ValueError:
            raise
        except Exception as e:
            logger.error(f"Unexpected error finding asset by name: {e}")
            raise RequestError(f"Failed to find asset by name: {e}")

    def find_by_names(self, names: list[str]) -> list[Asset]:
        """
        Find assets by name.

        Args:
            names: The names of the assets to find.

        Returns:
            The assets.

        Raises:
            ClientError: If the request fails.
        """
        try:
            return self._low_level_client.get_assets_by_name(names)
        except ClientError:
            raise
        except Exception as e:
            logger.error(f"Unexpected error finding assets by name: {e}")
            raise RequestError(f"Failed to find assets by name: {e}")

    def find_by_tag(self, tag: str) -> list[Asset]:
        """
        Find assets by tag.

        Args:
            tag: The tag of the assets to find.

        Returns:
            The assets.

        Raises:
            ClientError: If the request fails.
        """
        try:
            return self._low_level_client.get_assets_by_tag([tag])
        except ClientError:
            raise
        except Exception as e:
            logger.error(f"Unexpected error finding assets by tag: {e}")
            raise RequestError(f"Failed to find assets by tag: {e}")

    def find_by_tags(self, tags: list[str]) -> list[Asset]:
        """
        Find assets by tags.

        Args:
            tags: The tags of the assets to find.

        Returns:
            The assets.

        Raises:
            ClientError: If the request fails.
        """
        try:
            return self._low_level_client.get_assets_by_tag(tags)
        except ClientError:
            raise
        except Exception as e:
            logger.error(f"Unexpected error finding assets by tags: {e}")
            raise RequestError(f"Failed to find assets by tags: {e}")
