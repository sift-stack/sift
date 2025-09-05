from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.ping import PingLowLevelClient
from sift_client.resources._base import ResourceBase

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class PingAPIAsync(ResourceBase):
    """High-level API for performing health checks."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = PingLowLevelClient(sift_client.grpc_client)

    async def ping(self) -> str:
        """Send a ping request to the server.

        Returns:
            The response from the server.
        """
        return await self._low_level_client.ping()
