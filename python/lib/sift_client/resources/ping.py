from __future__ import annotations

import logging

from sift_client._internal.low_level_wrappers.ping import PingLowLevelClient
from sift_client.transport import GrpcClient, WithGrpcClient

# Configure logging
logger = logging.getLogger(__name__)


class PingAPIAsync(WithGrpcClient):
    """
    High-level API for performing health checks.
    """

    def __init__(self, grpc_client: GrpcClient = None):
        """
        Initialize the AssetsAPI.

        Args:
            grpc_client: The gRPC client to use for making API calls.
        """
        super().__init__(grpc_client)
        self._low_level_client = PingLowLevelClient(self._grpc_client)

    async def ping(self) -> str:
        """
        Send a ping request to the server.

        Returns:
            The response from the server.
        """
        return await self._low_level_client.ping()
