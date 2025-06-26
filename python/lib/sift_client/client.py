from __future__ import annotations

from typing import NamedTuple

from sift_client.errors import _sift_client_experimental_warning
from sift_client.resources import (
    AssetsAPI,
    AssetsAPIAsync,
    CalculatedChannelsAPI,
    CalculatedChannelsAPIAsync,
    PingAPI,
    PingAPIAsync,
    RunsAPI,
    RunsAPIAsync,
)
from sift_client.transport import (
    GrpcClient,
    GrpcConfig,
    RestClient,
    RestConfig,
    SiftConnectionConfig,
    WithGrpcClient,
    WithRestClient,
)

_sift_client_experimental_warning()


class AsyncAPIs(NamedTuple):
    """Simple accessor for the asynchronous APIs, still uses the SiftClient instance."""

    """Instance of the Ping API for making asynchronous requests."""
    ping: PingAPIAsync
    """Instance of the Assets API for making asynchronous requests."""
    assets: AssetsAPIAsync
    """Instance of the Calculated Channels API for making asynchronous requests."""
    calculated_channels: CalculatedChannelsAPIAsync
    """Instance of the Runs API for making asynchronous requests."""
    runs: RunsAPIAsync


class SiftClient(
    WithGrpcClient,
    WithRestClient,
):
    """
    SiftClient is a high-level client for interacting with Sift's APIs.

    It provides both synchronous and asynchronous interfaces, strong type checking, and a Pythonic API design.

    Example usage:
        from sift_client import SiftClient
        from datetime import datetime

        # Initialize with individual parameters
        client = SiftClient(
            api_key="your-api-key",
            grpc_url="your-sift-grpc-url",
            rest_url="your-sift-rest-url")

        # Or use a connection configuration to customize connection behavior
        connection_config = SiftConnectionConfig(
            grpc_config=GrpcConfig(),
            rest_config=RestConfig())

        sift = SiftClient(connection_config=connection_config)

        # Use the client to make requests
        response = sift.ping.ping()

        # Or asynchronously
        response = await sift.async_.ping.ping()
    """

    """Instance of the Ping API for making synchronous requests."""
    ping: PingAPI
    """Instance of the Assets API for making synchronous requests."""
    assets: AssetsAPI
    """Instance of the Calculated Channels API for making synchronous requests."""
    calculated_channels: CalculatedChannelsAPI
    """Instance of the Runs API for making synchronous requests."""
    runs: RunsAPI

    def __init__(
        self,
        api_key: str | None = None,
        grpc_url: str | None = None,
        rest_url: str | None = None,
        connection_config: SiftConnectionConfig | None = None,
    ):
        """
        Initialize the SiftClient with specific connection parameters or a connection_config.

        Args:
            api_key: The Sift API key for authentication.
            grpc_url: The Sift gRPC API URL.
            rest_url: The Sift REST API URL.
            connection_config: A SiftConnectionConfig object to configure the connection behavior of the SiftClient.
        """

        if not (api_key and grpc_url and rest_url) and not connection_config:
            raise ValueError(
                "Either api_key, grpc_url and rest_url or connection_config must be provided to establish a connection."
            )

        if connection_config:
            grpc_client = GrpcClient(connection_config.get_grpc_config())
            rest_client = RestClient(connection_config.get_rest_config())
        elif api_key and grpc_url and rest_url:
            grpc_client = GrpcClient(GrpcConfig(grpc_url, api_key))
            rest_client = RestClient(RestConfig(rest_url, api_key))
        else:
            raise ValueError(
                "Invalid connection configuration. Please provide api_key, grpc_uri and rest_uri or a connection_config."
            )

        WithGrpcClient.__init__(self, grpc_client=grpc_client)
        WithRestClient.__init__(self, rest_client=rest_client)

        self.ping = PingAPI(self)
        self.assets = AssetsAPI(self)
        self.calculated_channels = CalculatedChannelsAPI(self)
        self.runs = RunsAPI(self)
        # Accessor for the asynchronous APIs
        self.async_ = AsyncAPIs(
            ping=PingAPIAsync(self),
            assets=AssetsAPIAsync(self),
            calculated_channels=CalculatedChannelsAPIAsync(self),
            runs=RunsAPIAsync(self),
        )

    @property
    def grpc_client(self) -> GrpcClient:
        """The gRPC client used by the SiftClient for making gRPC API calls."""
        return self._grpc_client

    @property
    def rest_client(self) -> RestClient:
        """The REST client used by the SiftClient for making REST API calls."""
        return self._rest_client
