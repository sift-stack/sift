"""Async gRPC caching interceptor for transparent local response caching.

This module provides an async caching interceptor that can be used to cache gRPC
unary-unary responses locally using diskcache. The cache is persistent across runs
and supports TTL expiration and per-request control via metadata.

Usage:
    from sift_py.grpc._async_interceptors.caching import CachingAsyncInterceptor

    # Create interceptor with 1 hour TTL
    cache_interceptor = CachingAsyncInterceptor(ttl=3600, cache_path=".grpc_cache")

    # Use with metadata to control caching:
    metadata = [
        ("use-cache", "true"),  # Enable caching for this call
        # ("force-refresh", "true"),  # Bypass cache and store fresh result
        # ("clear-cache", "true"),  # Delete cached entry before request
    ]
"""

from __future__ import annotations

import hashlib
import logging
from pathlib import Path
from typing import Any, Optional

from grpc import aio as grpc_aio

from sift_py.grpc._async_interceptors.base import ClientAsyncInterceptor

logger = logging.getLogger(__name__)

# Metadata keys for cache control
METADATA_USE_CACHE = "use-cache"
METADATA_FORCE_REFRESH = "force-refresh"
METADATA_CLEAR_CACHE = "clear-cache"


class CachingAsyncInterceptor(ClientAsyncInterceptor):
    """Async interceptor that caches unary-unary gRPC responses locally.

    This interceptor uses diskcache for persistent storage with TTL support.
    Cache keys are generated deterministically based on the gRPC method name
    and serialized request payload.

    Note: diskcache operations are synchronous, but the overhead is minimal
    for most use cases. For high-throughput scenarios, consider using an
    async-native cache backend.

    Attributes:
        ttl: Time-to-live for cached entries in seconds. Default is 3600 (1 hour).
        cache_path: Path to the cache directory. Default is ".grpc_cache".
        size_limit: Maximum size of the cache in bytes. Default is 1GB.
    """

    def __init__(
        self,
        ttl: int = 3600,
        cache_path: str = ".grpc_cache",
        size_limit: int = 1024 * 1024 * 1024,  # 1GB
    ):
        """Initialize the async caching interceptor.

        Args:
            ttl: Time-to-live for cached entries in seconds.
            cache_path: Path to the cache directory.
            size_limit: Maximum size of the cache in bytes.
        """
        try:
            import diskcache
        except ImportError:
            raise ImportError(
                "diskcache is required for caching. Install it with: pip install diskcache"
            )

        self.ttl = ttl
        self.cache_path = Path(cache_path)
        self.size_limit = size_limit

        # Create cache directory if it doesn't exist
        self.cache_path.mkdir(parents=True, exist_ok=True)

        # Initialize diskcache
        self._cache = diskcache.Cache(str(self.cache_path), size_limit=size_limit)

        logger.debug(
            f"Initialized CachingAsyncInterceptor with ttl={ttl}s, "
            f"cache_path={cache_path}, size_limit={size_limit} bytes"
        )

    async def intercept(
        self,
        method: Any,
        request_or_iterator: Any,
        client_call_details: grpc_aio.ClientCallDetails,
    ) -> Any:
        """Intercept the async gRPC call and apply caching logic.

        Args:
            method: The continuation to call for the actual RPC.
            request_or_iterator: The request object or iterator.
            client_call_details: The call details including method name and metadata.

        Returns:
            The response from the cache or the actual RPC call.
        """
        # Extract metadata flags
        metadata_dict = self._extract_metadata(client_call_details.metadata)
        use_cache = metadata_dict.get(METADATA_USE_CACHE, "false").lower() == "true"
        force_refresh = metadata_dict.get(METADATA_FORCE_REFRESH, "false").lower() == "true"
        clear_cache = metadata_dict.get(METADATA_CLEAR_CACHE, "false").lower() == "true"

        # If caching is not enabled, just pass through
        if not use_cache and not clear_cache and not force_refresh:
            return await method(request_or_iterator, client_call_details)

        # Generate cache key
        cache_key = self._generate_cache_key(client_call_details.method, request_or_iterator)

        # Handle clear-cache flag
        if clear_cache:
            logger.debug(f"Clearing cache for key: {cache_key}")
            self._cache.delete(cache_key)
            # Continue with the request after clearing

        # Handle force-refresh flag
        if force_refresh:
            logger.debug(f"Force refresh for key: {cache_key}")
            call = await method(request_or_iterator, client_call_details)
            # For async, we need to await the response
            response = await call
            # Cache the fresh result
            self._cache_response(cache_key, response)
            return response

        # Try to get from cache if use-cache is enabled
        if use_cache:
            cached_response = self._cache.get(cache_key)
            if cached_response is not None:
                logger.debug(f"Cache hit for key: {cache_key}")
                return cached_response

            logger.debug(f"Cache miss for key: {cache_key}")

        # Make the actual RPC call
        call = await method(request_or_iterator, client_call_details)
        response = await call

        # Cache the response if use-cache is enabled
        if use_cache:
            self._cache_response(cache_key, response)

        return response

    def _generate_cache_key(self, method_name: str, request: Any) -> str:
        """Generate a deterministic cache key from method name and request.

        Args:
            method_name: The gRPC method name.
            request: The request object.

        Returns:
            A SHA256 hash of the method name and serialized request.
        """
        try:
            # Serialize the request using protobuf's SerializeToString
            request_bytes = request.SerializeToString()
        except AttributeError:
            # If the request doesn't have SerializeToString, fall back to str
            logger.warning(
                f"Request for {method_name} doesn't have SerializeToString, using str() instead"
            )
            request_bytes = str(request).encode()

        # Create a deterministic hash
        key_material = method_name.encode() + request_bytes
        cache_key = hashlib.sha256(key_material).hexdigest()

        return cache_key

    def _cache_response(self, cache_key: str, response: Any) -> None:
        """Store a response in the cache with TTL.

        Args:
            cache_key: The cache key.
            response: The response to cache.
        """
        try:
            self._cache.set(cache_key, response, expire=self.ttl)
            logger.debug(f"Cached response for key: {cache_key} with TTL: {self.ttl}s")
        except Exception as e:
            logger.error(f"Failed to cache response for key {cache_key}: {e}")

    def _extract_metadata(self, metadata: Optional[tuple[tuple[str, str], ...]]) -> dict[str, str]:
        """Extract metadata into a dictionary.

        Args:
            metadata: The metadata tuple.

        Returns:
            A dictionary of metadata key-value pairs.
        """
        if metadata is None:
            return {}
        return dict(metadata)

    def clear_all(self) -> None:
        """Clear all cached entries."""
        logger.info("Clearing all cached entries")
        self._cache.clear()

    def close(self) -> None:
        """Close the cache and release resources."""
        logger.debug("Closing cache")
        self._cache.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()
