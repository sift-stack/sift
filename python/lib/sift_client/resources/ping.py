"""
High-level API for interacting with assets.

This module provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.
"""

from __future__ import annotations

import logging

from sift_client._internal.low_level_wrappers.ping import PingLowLevelClient
from sift_client.transport.grpc_transport import GrpcClient

# Configure logging
logger = logging.getLogger(__name__)


class PingAPIAsync:
    """
    High-level API for performing health checks.
    """

    def __init__(self, client: GrpcClient):
        """
        Initialize the AssetsAPI.

        Args:
            client: The gRPC client to use for making API calls.
        """
        self._client = client
        self._low_level_client = PingLowLevelClient(client)

    async def ping(self) -> str:
        """
        Send a ping request to the server.

        Returns:
            The response from the server.
        """
        return await self._low_level_client.ping()

