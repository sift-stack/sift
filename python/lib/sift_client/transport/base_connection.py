from __future__ import annotations

from abc import ABC
from typing import TYPE_CHECKING

from sift_client.transport.grpc_transport import GrpcClient, GrpcConfig
from sift_client.transport.rest_transport import RestClient, RestConfig

if TYPE_CHECKING:
    import asyncio


class SiftConnectionConfig:
    """Configuration for Grpc and Rest connections.

    This class provides a unified configuration for both gRPC and REST connections,
    allowing for consistent settings across different transport protocols.
    """

    def __init__(
        self,
        grpc_url: str,
        rest_url: str,
        api_key: str,
        use_ssl: bool = True,
        cert_via_openssl: bool = False,
    ):
        """Initialize the connection configuration.

        Args:
            grpc_url: The URL for the gRPC service.
            rest_url: The URL for the REST service.
            api_key: The API key for authentication.
            use_ssl: Whether to use SSL/TLS for secure connections.
            cert_via_openssl: Whether to use OpenSSL for certificate validation.
        """
        self.api_key = api_key
        self.grpc_url = grpc_url
        self.rest_url = rest_url
        self.use_ssl = use_ssl
        self.cert_via_openssl = cert_via_openssl

    def get_grpc_config(self):
        """Create and return a GrpcConfig with the current settings.

        Returns:
            A GrpcConfig object configured with this instance's settings.
        """
        return GrpcConfig(
            url=self.grpc_url,
            api_key=self.api_key,
            use_ssl=self.use_ssl,
            cert_via_openssl=self.cert_via_openssl,
        )

    def get_rest_config(self):
        """Create and return a RestConfig with the current settings.

        Returns:
            A RestConfig object configured with this instance's settings.
        """
        return RestConfig(
            base_url=self.rest_url,
            api_key=self.api_key,
            use_ssl=self.use_ssl,
            cert_via_openssl=self.cert_via_openssl,
        )


class WithGrpcClient(ABC):
    """Abstract base class for classes that require a gRPC client.

    This class provides access to a gRPC client for making API calls.
    """

    _grpc_client: GrpcClient

    def __init__(self, grpc_client: GrpcClient):
        """Initialize with a gRPC client.

        Args:
            grpc_client: The gRPC client to use for API calls.
        """
        self._grpc_client = grpc_client

    def get_asyncio_loop(self) -> asyncio.AbstractEventLoop:
        """Gets the default asyncio loop used by the gRPC client.

        Returns:
            The default asyncio loop.
        """
        return self._grpc_client.default_loop


class WithRestClient(ABC):
    """Abstract base class for classes that require a REST client.

    This class provides access to a REST client for making API calls.
    """

    _rest_client: RestClient

    def __init__(self, rest_client: RestClient):
        """Initialize with a REST client.

        Args:
            rest_client: The REST client to use for API calls.
        """
        self._rest_client = rest_client
