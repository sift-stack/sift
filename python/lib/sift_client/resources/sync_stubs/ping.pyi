# Generated stubs for sift_client.resources.ping

from sift_client.resources.ping import *

class PingAPI:
    """Sync counterpart to `PingAPIAsync`.


    High-level API for performing health checks.
    """

    def __init__(self, client: 'GrpcClient') -> None: ...
    def ping(self, ) -> str:
        """Send a ping request to the server.

        Returns:
            The response from the server."""
        ...
