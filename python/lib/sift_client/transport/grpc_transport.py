"""Transport layer for gRPC communication.

This module provides a simple wrapper around sift_py/grpc/transport.py for making gRPC API calls.
It just stores the channel and the stubs, without any additional functionality.
"""

from __future__ import annotations

import asyncio
import atexit
import logging
import threading
from typing import Any

from sift_py.grpc.transport import (
    SiftChannelConfig,
    use_sift_async_channel,
)

# Configure logging
logger = logging.getLogger(__name__)


def _suppress_blocking_io(loop, context):
    """Suppress benign BlockingIOError from gRPC's PollerCompletionQueue.

    gRPC's internal poller uses non-blocking I/O. When no events are ready,
    it raises BlockingIOError (EAGAIN), which is expected and safe to ignore.
    Swallowing these prevents noisy, spurious error logs.
    """
    exc = context.get("exception")
    if isinstance(exc, BlockingIOError):
        return
    loop.default_exception_handler(context)


class GrpcConfig:
    """Configuration for gRPC API clients."""

    def __init__(
        self,
        url: str,
        api_key: str,
        use_ssl: bool = True,
        cert_via_openssl: bool = False,
        metadata: dict[str, str] | None = None,
    ):
        """Initialize the gRPC configuration.

        Args:
            url: The URI of the gRPC server.
            api_key: The API key for authentication.
            use_ssl: Whether to use SSL/TLS.
            cert_via_openssl: Whether to use OpenSSL for SSL/TLS.
            use_async: Whether to use async gRPC client.
            metadata: Additional metadata to include in all requests.
        """
        self.uri = url
        self.api_key = api_key
        self.use_ssl = use_ssl
        self.cert_via_openssl = cert_via_openssl
        self.metadata = metadata or {}

    def _to_sift_channel_config(self) -> SiftChannelConfig:
        """Convert to a SiftChannelConfig.

        Returns:
            A SiftChannelConfig.
        """
        return {
            "uri": self.uri,
            "apikey": self.api_key,
            "use_ssl": self.use_ssl,
            "cert_via_openssl": self.cert_via_openssl,
        }


class GrpcClient:
    """A simple wrapper around sift_py/grpc/transport.py for making gRPC API calls.

    This class just stores the channel and the stubs, without any additional functionality.
    """

    def __init__(self, config: GrpcConfig):
        """Initialize the gRPC client.

        Args:
            config: The gRPC client configuration.
        """
        self._config = config
        # map each asyncio loop to its async channel and stub dict
        self._channels_async: dict[asyncio.AbstractEventLoop, Any] = {}
        self._stubs_async_map: dict[asyncio.AbstractEventLoop, dict[type[Any], Any]] = {}
        # default loop for sync API
        self._default_loop = asyncio.new_event_loop()
        atexit.register(self.close_sync)
        # suppress benign EAGAIN (no-data) errors from gRPC poll on default loop
        self._default_loop.set_exception_handler(_suppress_blocking_io)

        # start default loop in background thread
        def _run_default_loop():
            asyncio.set_event_loop(self._default_loop)
            self._default_loop.run_forever()

        self._default_loop_thread = threading.Thread(
            target=_run_default_loop,
            daemon=True,
        )
        self._default_loop_thread.start()
        # init async channel on default loop via helper coroutine
        cfg = config._to_sift_channel_config()
        future = asyncio.run_coroutine_threadsafe(
            self._create_async_channel(cfg, config.metadata), self._default_loop
        )
        channel = future.result()
        self._channels_async[self._default_loop] = channel
        self._stubs_async_map[self._default_loop] = {}

    @property
    def default_loop(self) -> asyncio.AbstractEventLoop:
        """Return the default event loop used for synchronous API operations.

        Returns:
            The default asyncio event loop.
        """
        return self._default_loop

    def get_stub(self, stub_class: type[Any]) -> Any:
        """Get an async stub bound to the current event loop.
        Creates a channel and stub for this loop if needed.
        """
        try:
            loop = asyncio.get_running_loop()
            # suppress benign EAGAIN (no-data) errors from gRPC poll on this loop
            loop.set_exception_handler(_suppress_blocking_io)
        except RuntimeError:  # No running event loop
            loop = self._default_loop

        if loop not in self._channels_async:
            channel = use_sift_async_channel(
                self._config._to_sift_channel_config(), self._config.metadata
            )
            self._channels_async[loop] = channel
            self._stubs_async_map[loop] = {}

        stubs = self._stubs_async_map[loop]
        if stub_class not in stubs:
            stubs[stub_class] = stub_class(self._channels_async[loop])
        return stubs[stub_class]

    def close_sync(self):
        """Close the sync channel and all async channels."""
        try:
            for ch in self._channels_async.values():
                asyncio.run_coroutine_threadsafe(ch.close(), self._default_loop).result()
            self._default_loop.call_soon_threadsafe(self._default_loop.stop)
            self._default_loop_thread.join(timeout=1.0)
        except ValueError:
            ...

    async def close(self):
        """Close sync and async channels and stop the default loop."""
        for ch in self._channels_async.values():
            await ch.close()
        self._default_loop.call_soon_threadsafe(self._default_loop.stop)
        self._default_loop_thread.join(timeout=1.0)

    async def __aenter__(self):
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close_sync()

    async def _create_async_channel(
        self, cfg: SiftChannelConfig, metadata: dict[str, str] | None
    ) -> Any:
        """Helper to create async channel on default loop."""
        return use_sift_async_channel(cfg, metadata)
