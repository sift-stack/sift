# Auto-generated stub

from __future__ import annotations
import logging
from google.protobuf.field_mask_pb2 import FieldMask
from sift_client._internal.low_level_wrappers.assets import AssetsLowLevelClient
from sift_client.errors import ClientError, RequestError
from sift_client.transport import GrpcClient, WithGrpcClient
from sift_client.types.asset import Asset
from sift_client._internal.low_level_wrappers.ping import PingLowLevelClient

class AssetsAPI:
    """
    Sync counterpart to `AssetsAPIAsync`.
    
    
    High-level API for interacting with assets.
    
    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.
    
    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """
    
    def __init__(self, grpc_client: GrpcClient):
        """Initialize the AssetsAPI.
        
        Args:
            grpc_client: The gRPC client to use for making API calls."""
        ...
    
    def _run(self, coro):
        """"""
        ...
    
    def delete(self, asset_id: str) -> None:
        """Delete an asset.
        
        Args:
            asset_id: The ID of the asset to delete.
        
        Raises:
            ClientError: If the request fails."""
        ...
    
    def find_by_name(self, name: str) -> Asset | None:
        """Find an asset by name.
        
        Args:
            name: The name of the asset to find.
        
        Returns:
            The asset, or None if not found.
        
        Raises:
            ClientError: If the request fails.
            ValueError: If multiple assets are found with the same name."""
        ...
    
    def find_by_names(self, names: list[str]) -> list[Asset]:
        """Find assets by name.
        
        Args:
            names: The names of the assets to find.
        
        Returns:
            The assets.
        
        Raises:
            ClientError: If the request fails."""
        ...
    
    def find_by_tag(self, tag: str) -> list[Asset]:
        """Find assets by tag.
        
        Args:
            tag: The tag of the assets to find.
        
        Returns:
            The assets.
        
        Raises:
            ClientError: If the request fails."""
        ...
    
    def find_by_tags(self, tags: list[str]) -> list[Asset]:
        """Find assets by tags.
        
        Args:
            tags: The tags of the assets to find.
        
        Returns:
            The assets.
        
        Raises:
            ClientError: If the request fails."""
        ...
    
    def get(self, asset_id: str = None, name: str = None) -> Asset:
        """Get an asset by ID.
        
        Args:
            asset_id: The ID of the asset to get.
        
        Returns:
            The asset."""
        ...
    
    def list_(self, page_size: int = None, page_token: str = None, filter: str = None, order_by: str = None) -> tuple[list[Asset], str]:
        """List assets.
        
        Args:
            page_size: The maximum number of assets to return.
            page_token: A page token, received from a previous `list` call.
            filter: A filter string.
            order_by: How to order the retrieved assets.
        
        Returns:
            A tuple containing the list of assets and the next page token.
        
        Raises:
            ClientError: If the request fails."""
        ...
    
    def update_tags(self, asset_id: str, tags: list[str]) -> Asset:
        """Update the tags of an asset.
        
        Args:
            asset_id: The ID of the asset to update.
            tags: The new tags for the asset.
        
        Returns:
            The updated asset.
        
        Raises:
            ClientError: If the request fails."""
        ...



class PingAPI:
    """
    Sync counterpart to `PingAPIAsync`.
    
    
    High-level API for performing health checks.
    """
    
    def __init__(self, grpc_client: GrpcClient = None):
        """Initialize the AssetsAPI.
        
        Args:
            grpc_client: The gRPC client to use for making API calls."""
        ...
    
    def _run(self, coro):
        """"""
        ...
    
    def ping(self) -> str:
        """Send a ping request to the server.
        
        Returns:
            The response from the server."""
        ...

