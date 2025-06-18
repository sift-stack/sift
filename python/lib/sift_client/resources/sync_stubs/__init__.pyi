# Auto-generated stub

from __future__ import annotations

import re
from datetime import datetime
from typing import Any

from sift_client.client import SiftClient
from sift_client.types.asset import Asset, AssetUpdate

class AssetsAPI:
    """
    Sync counterpart to `AssetsAPIAsync`.


    High-level API for interacting with assets.

    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """

    def __init__(self, sift_client: "SiftClient"):
        """Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use."""
        ...

    def _run(self, coro):
        """"""
        ...

    def archive(self, asset: str | Asset, archive_runs: bool = False) -> Asset:
        """Archive an asset.

        Args:
            asset: The Asset or asset ID to archive.
            archive_runs: If True, archive all Runs associated with the Asset.

        Returns:
            The archived Asset."""
        ...

    def find(self, *args, **kwargs) -> Asset | None:
        """Find a single asset matching the given query. Takes the same arguments as `list_`. If more than one asset is found,
        raises an error.

        Args:
            *args:
            **kwargs:

        Returns:
            The Asset found or None."""
        ...

    def get(self, asset_id: str = None, name: str = None) -> Asset:
        """Get an Asset.

        Args:
            asset_id: The ID of the asset.
            name: The name of the asset.

        Returns:
            The Asset."""
        ...

    def list_(
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
        metadata: list[Any] = None,
        include_archived: bool = False,
        filter_query: str = None,
        order_by: str = None,
        limit: int = None,
    ) -> list[Asset]:
        """List assets with optional filtering.

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
            A list of Assets that matches the filter."""
        ...

    def update(self, asset: str | Asset, update: AssetUpdate | dict) -> Asset:
        """Update an Asset.

        Args:
            asset: The Asset or asset ID to update.
            update: Updates to apply to the Asset.

        Returns:
            The updated Asset."""
        ...

class PingAPI:
    """
    Sync counterpart to `PingAPIAsync`.


    High-level API for performing health checks.
    """

    def __init__(self, sift_client: "SiftClient"):
        """Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use."""
        ...

    def _run(self, coro):
        """"""
        ...

    def ping(self) -> str:
        """Send a ping request to the server.

        Returns:
            The response from the server."""
        ...
