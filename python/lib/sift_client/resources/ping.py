"""
High-level API for interacting with assets.

This module provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.
"""

from __future__ import annotations

import asyncio
import logging
from typing import Any

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

    async def ping(self):
        """
        Send a ping request to the server.

        Returns:
            The response from the server.
        """
        return await self._low_level_client.ping()


class PingAPI:
    """
    Synchronous façade around HighLevelAsyncClient.

    Internally, each method calls into an event loop to run the async code.
    """

    def __init__(self, client: GrpcClient):
        """
        Initialize the PingAPI.

        Args:
            client: The gRPC client to use for making API calls.
        """
        self._client = client
        self._async_impl = PingAPIAsync(client)

    def _run(self, coro) -> Any:
        """
        Run the given coroutine on the client's default sync loop.
        """
        future = asyncio.run_coroutine_threadsafe(coro, self._client._default_loop)
        return future.result()

    def ping(self) -> str:
        """
        Send a ping request to the server synchronously.

        Returns:
            The response from the server.
        """
        return self._run(self._async_impl.ping())
