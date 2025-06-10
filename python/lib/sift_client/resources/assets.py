"""
High-level API for interacting with assets.

This module provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.
"""

from __future__ import annotations

import logging

from google.protobuf.field_mask_pb2 import FieldMask

from sift_client._internal.low_level_wrappers.assets import AssetsLowLevelClient
from sift_client.errors import ClientError, RequestError
from sift_client.transport.grpc_transport import GrpcClient
from sift_client.types.asset import Asset

# Configure logging
logger = logging.getLogger(__name__)


class AssetsAPI:
    """
    High-level API for interacting with assets.

    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """

    def __init__(self, client: GrpcClient):
        """
        Initialize the AssetsAPI.

        Args:
            client: The gRPC client to use for making API calls.
        """
        self._client = client
        self._low_level_client = AssetsLowLevelClient(client)

    def get(self, asset_id: str) -> Asset:
        """
        Get an asset by ID.

        Args:
            asset_id: The ID of the asset to get.

        Returns:
            The asset.

        Raises:
            ClientError: If the request fails.
        """
        try:
            return self._low_level_client.get_asset(asset_id)
        except ClientError:
            raise
        except Exception as e:
            logger.error(f"Unexpected error getting asset: {e}")
            raise RequestError(f"Failed to get asset: {e}")

    def list(
        self,
        page_size: int | None = None,
        page_token: str | None = None,
        filter: str | None = None,
        order_by: str | None = None,
    ) -> tuple[list[Asset], str]:
        """
        List assets.

        Args:
            page_size: The maximum number of assets to return.
            page_token: A page token, received from a previous `list` call.
            filter: A filter string.
            order_by: How to order the retrieved assets.

        Returns:
            A tuple containing the list of assets and the next page token.

        Raises:
            ClientError: If the request fails.
        """
        try:
            return self._low_level_client.list_assets(
                page_size=page_size,
                page_token=page_token,
                query_filter=filter,
                order_by=order_by,
            )
        except ClientError:
            raise
        except Exception as e:
            logger.error(f"Unexpected error listing assets: {e}")
            raise RequestError(f"Failed to list assets: {e}")

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
