"""Transport layer for gRPC communication.

This module provides a simple wrapper around sift_py/grpc/transport.py for making gRPC API calls.
It just stores the channel and the stubs, without any additional functionality.
"""

from __future__ import annotations

import asyncio
import atexit
import concurrent.futures
import logging
import threading
from typing import Any
from urllib.parse import urlparse

from sift_client._internal.grpc_transport.transport import (
    DEFAULT_REQUEST_TIMEOUT_SECONDS,
    SiftChannelConfig,
    use_sift_async_channel,
)

# Configure logging
logger = logging.getLogger(__name__)

# How far the blocking sync deadline sits above the per-RPC deadline. The gRPC
# deadline should fire first and cancel the request; the sync backstop only trips
# if that never happens.
_SYNC_CALL_TIMEOUT_MARGIN_SECONDS = 15.0


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
        request_timeout: float | None = DEFAULT_REQUEST_TIMEOUT_SECONDS,
    ):
        """Initialize the gRPC configuration.

        Args:
            url: The URI of the gRPC server.
            api_key: The API key for authentication.
            use_ssl: Whether to use SSL/TLS.
            cert_via_openssl: Whether to use OpenSSL for SSL/TLS.
            use_async: Whether to use async gRPC client.
            metadata: Additional metadata to include in all requests.
            request_timeout: Default deadline in seconds applied to unary RPCs that don't set
                their own. Defaults to 60s. Set to None to disable the default deadline.
        """
        parsed_url = urlparse(url)
        normalized_url = url
        if not parsed_url.netloc and parsed_url.scheme not in ("http", "https"):
            # missing netloc means no '://' separator and will prepend the scheme
            normalized_url = f"https://{url}" if use_ssl else f"http://{url}"
            parsed_url = urlparse(normalized_url)
        if parsed_url.scheme not in ("http", "https") or not parsed_url.netloc:
            raise ValueError(
                f"Invalid connection URL '{url}'. Expected format: 'http[s]://hostname[:port]'."
            )
        self.uri = normalized_url
        self.api_key = api_key
        self.use_ssl = use_ssl
        self.cert_via_openssl = cert_via_openssl
        self.metadata = metadata or {}
        self.request_timeout = request_timeout

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
            "request_timeout": self.request_timeout,
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
        # Guards close() / close_sync() against running twice. The atexit
        # handler always fires, so an explicit close must leave it a no-op.
        self._closed = False
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
        # Tracks whether the default loop is accepting work. Set False the moment
        # a close begins so a concurrent sync call sees it immediately, before the
        # loop has actually stopped.
        self._loop_running = True

    @property
    def default_loop(self) -> asyncio.AbstractEventLoop:
        """Return the default event loop used for synchronous API operations.

        Returns:
            The default asyncio event loop.
        """
        return self._default_loop

    @property
    def is_loop_running(self) -> bool:
        """Whether the default loop is accepting synchronous API work.

        False once a close has begun, so callers can fail fast instead of
        scheduling a coroutine onto a loop that will never run it.
        """
        return self._loop_running and self._default_loop.is_running()

    @property
    def sync_call_timeout(self) -> float | None:
        """Deadline in seconds for a blocking sync API call, or None if disabled.

        Sits above the per-RPC deadline by a margin so the gRPC deadline fires
        first and cancels the in-flight request; this is only a backstop for the
        case where the RPC deadline never trips.
        """
        request_timeout = self._config.request_timeout
        if request_timeout is None:
            return None
        return request_timeout + _SYNC_CALL_TIMEOUT_MARGIN_SECONDS

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
        """Close the sync channel and all async channels. Idempotent."""
        if self._closed:
            return
        self._closed = True
        self._loop_running = False
        try:
            # Only drive the loop if it's still running; submitting a coroutine
            # to a stopped loop never resolves and would hang on .result().
            if self._default_loop.is_running():
                for ch in self._channels_async.values():
                    asyncio.run_coroutine_threadsafe(ch.close(), self._default_loop).result(
                        timeout=5.0
                    )
                self._default_loop.call_soon_threadsafe(self._default_loop.stop)
            self._default_loop_thread.join(timeout=1.0)
        except (ValueError, RuntimeError, concurrent.futures.TimeoutError):
            ...
        finally:
            self._release_channels()

    async def close(self):
        """Close sync and async channels and stop the default loop. Idempotent."""
        if self._closed:
            return
        self._closed = True
        self._loop_running = False
        for ch in self._channels_async.values():
            await ch.close()
        self._default_loop.call_soon_threadsafe(self._default_loop.stop)
        self._default_loop_thread.join(timeout=1.0)
        self._release_channels()

    def _release_channels(self):
        """Drop references to the closed channels and stubs.

        The gRPC C-core defers a channel's resource release until the Python
        object is destroyed, not merely closed. Holding the channels in these
        maps keeps them alive until interpreter finalization, which races the
        C-core's own exit-time shutdown ("grpc_wait_for_shutdown_with_timeout()
        timed out"). Clearing the maps lets the channels be collected promptly.
        """
        self._channels_async.clear()
        self._stubs_async_map.clear()

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
