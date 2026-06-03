"""Transport layer for REST communication.

This module provides a simple wrapper around sift_py/rest.py for making REST API calls.
"""

from __future__ import annotations

import logging
from typing import TYPE_CHECKING
from urllib.parse import urljoin

from sift_client._internal.rest import _DEFAULT_REST_RETRY, SiftRestConfig, _RestService

if TYPE_CHECKING:
    import requests
    from urllib3.util import Retry


# Configure logging
logger = logging.getLogger(__name__)

# Default timeout (seconds) for REST requests that don't set their own; applies to
# connect and read. The read leg is the gap between bytes, not the whole transfer, so a
# stalled socket fails fast while a healthy long download is not cut off.
DEFAULT_REST_TIMEOUT: float = 60.0


class RestConfig:
    """Configuration for REST API clients."""

    def __init__(
        self,
        base_url: str,
        api_key: str,
        use_ssl: bool = True,
        cert_via_openssl: bool = False,
        retry: Retry = _DEFAULT_REST_RETRY,
        request_timeout: float | tuple[float, float] | None = DEFAULT_REST_TIMEOUT,
    ):
        """Initialize the REST configuration.

        Args:
            base_url: The base URL of the API.
            api_key: The API key for authentication.
            use_ssl: Whether to use HTTPS.
            cert_via_openssl: Whether to use OpenSSL for SSL/TLS.
            retry: The retry configuration for requests.
            request_timeout: Default timeout in seconds for requests that don't set their
                own; applies to connect and read. Pass a (connect, read) tuple to split
                them. Defaults to DEFAULT_REST_TIMEOUT; set to None to disable.
        """
        if not base_url.startswith("http"):
            # urljoin (used when executing requests) requires URL starting with http or https
            base_url = f"https://{base_url}" if use_ssl else f"http://{base_url}"
        self.base_url = base_url
        self.api_key = api_key
        self.use_ssl = use_ssl
        self.cert_via_openssl = cert_via_openssl
        self.retry = retry
        self.request_timeout = request_timeout

    def _to_sift_rest_config(self) -> SiftRestConfig:
        """Convert to a SiftRestConfig for backwards compatibility. Will be removed in the future.

        Returns:
            A SiftRestConfig.
        """
        return {
            "uri": self.base_url,
            "apikey": self.api_key,
            "retry": self.retry,
            "use_ssl": self.use_ssl,
            "cert_via_openssl": self.cert_via_openssl,
        }


class RestClient:
    """A client wrapper for REST APIs.

    This class provides a wrapper around sift_py/rest.py for making REST API calls.
    It handles authentication, retries, and error mapping.
    """

    def __init__(self, config: RestConfig):
        """Initialize the REST client.

        Args:
            config: The REST client configuration.
        """
        self._base_url = config.base_url
        self._config = config
        self._request_timeout = config.request_timeout
        self._client = self._create_client()

    def _create_client(self) -> _RestService:
        """Create a REST service with the configured settings. Using _RestService for backwards compatibility. Will be removed in the future.

        Returns:
            A configured REST service.
        """
        sift_rest_config = self._config._to_sift_rest_config()

        # Create a concrete implementation of _RestService
        class ConcreteRestService(_RestService):
            def __init__(self, rest_conf: SiftRestConfig):
                super().__init__(rest_conf)

        return ConcreteRestService(sift_rest_config)

    @property
    def base_url(self) -> str:
        """Get the base URL of the REST client.

        Returns:
            The base URL string.
        """
        return self._base_url

    def close(self) -> None:
        """Close the REST client session."""
        self._client._session.close()

    # Convenience methods for common HTTP methods
    def _execute(
        self,
        method: str,
        endpoint: str,
        headers: dict | None = None,
        data: dict | None = None,
        **kwargs,
    ) -> requests.Response:
        full_url = urljoin(self.base_url, endpoint)
        # Apply the default timeout unless the caller set one, so a stalled socket
        # fails instead of blocking forever.
        if "timeout" not in kwargs and self._request_timeout is not None:
            kwargs["timeout"] = self._request_timeout
        return self._client._session.request(method, full_url, headers=headers, data=data, **kwargs)

    def get(self, endpoint: str, headers: dict | None = None, **kwargs) -> requests.Response:
        """Execute a GET request.

        Args:
            endpoint: The API endpoint to call.
            headers: Additional headers to include in the request.
            **kwargs: Additional arguments to pass to the request.

        Returns:
            The HTTP response.
        """
        return self._execute("GET", endpoint=endpoint, headers=headers, **kwargs)

    def post(
        self, endpoint: str, headers: dict | None = None, data=None, **kwargs
    ) -> requests.Response:
        """Execute a POST request.

        Args:
            endpoint: The API endpoint to call.
            headers: Additional headers to include in the request.
            data: The data to send in the request body.
            **kwargs: Additional arguments to pass to the request.

        Returns:
            The HTTP response.
        """
        return self._execute("POST", endpoint=endpoint, headers=headers, data=data, **kwargs)

    def put(
        self, endpoint: str, headers: dict | None = None, data=None, **kwargs
    ) -> requests.Response:
        """Execute a PUT request.

        Args:
            endpoint: The API endpoint to call.
            headers: Additional headers to include in the request.
            data: The data to send in the request body.
            **kwargs: Additional arguments to pass to the request.

        Returns:
            The HTTP response.
        """
        return self._execute("PUT", endpoint=endpoint, headers=headers, data=data, **kwargs)

    def delete(self, endpoint: str, headers: dict | None = None, **kwargs) -> requests.Response:
        """Execute a DELETE request.

        Args:
            endpoint: The API endpoint to call.
            headers: Additional headers to include in the request.
            **kwargs: Additional arguments to pass to the request.

        Returns:
            The HTTP response.
        """
        return self._execute("DELETE", endpoint=endpoint, headers=headers, **kwargs)

    def patch(
        self, endpoint: str, headers: dict | None = None, data=None, **kwargs
    ) -> requests.Response:
        """Execute a PATCH request.

        Args:
            endpoint: The API endpoint to call.
            headers: Additional headers to include in the request.
            data: The data to send in the request body.
            **kwargs: Additional arguments to pass to the request.

        Returns:
            The HTTP response.
        """
        return self._execute("PATCH", endpoint=endpoint, headers=headers, data=data, **kwargs)
