# Auto-generated stub

from __future__ import annotations
import logging
from sift_client._internal.low_level_wrappers.ping import PingLowLevelClient
from sift_client.transport.grpc_transport import GrpcClient


class PingAPI:
    """
    Sync counterpart to `PingAPIAsync`.
    
    
    High-level API for performing health checks.
    """
    def __init__(self, client, args, kwargs):
        """"""
        ...
    def _run(self, coro):
        """"""
        ...
    def ping(self) -> str:
        """Send a ping request to the server.
        
        Returns:
            The response from the server."""
        ...
    def test(self, a: logging.Logger = None) -> GrpcClient:
        """"""
        ...

