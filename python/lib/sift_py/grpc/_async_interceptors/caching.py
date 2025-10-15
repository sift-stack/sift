"""Async gRPC caching interceptor for transparent local response caching.

This module provides an async caching interceptor that can be used to cache gRPC
unary-unary responses locally using diskcache. The cache is initialized at the
GrpcClient level and passed to the interceptor.

Note: Cache initialization is handled by GrpcClient, not by this interceptor.

Usage:
    # Cache is initialized at GrpcClient level
    cache = diskcache.Cache(".grpc_cache", size_limit=1024**3)
    
    # Create interceptor with cache instance
    cache_interceptor = CachingAsyncInterceptor(ttl=3600, cache_instance=cache)

    # Use with metadata to control caching:
    metadata = [
        ("use-cache", "true"),  # Enable caching for this call
        # ("force-refresh", "true"),  # Bypass cache and store fresh result
        # ("ignore-cache", "true"),  # Bypass cache without clearing
    ]
"""

from __future__ import annotations

import logging
from typing import Any

import diskcache
from google.protobuf import message, symbol_database
from grpc import aio as grpc_aio

from sift_py.grpc._async_interceptors.base import ClientAsyncInterceptor
from sift_py.grpc.cache import GrpcCache

logger = logging.getLogger(__name__)

class CachingAsyncInterceptor(ClientAsyncInterceptor):
    """Async interceptor that caches unary-unary gRPC responses locally.

    This interceptor uses a diskcache instance for persistent storage with TTL support.
    The cache instance must be provided during initialization (typically from GrpcClient).
    Cache keys are generated deterministically based on the gRPC method name
    and serialized request payload.

    Responses are serialized to bytes before caching to avoid pickling issues with
    async objects.

    Note: diskcache operations are synchronous, but the overhead is minimal
    for most use cases. For high-throughput scenarios, consider using an
    async-native cache backend.

    Attributes:
        _cache: The GrpcCache instance provided during initialization.
    """

    def __init__(
        self,
        cache: GrpcCache,
    ):
        """Initialize the async caching interceptor.

        Args:
            cache: Pre-initialized GrpcCache instance (required).
        """
        self.cache = cache
        self.symbol_db = symbol_database.Default()

    async def intercept(
        self,
        method: Any,
        request_or_iterator: Any,
        client_call_details: grpc_aio.ClientCallDetails,
    ) -> Any:
        """Intercept the async gRPC call and apply caching logic.

        Uses GrpcCache.resolve_cache_metadata() to determine caching behavior.

        Args:
            method: The continuation to call for the actual RPC.
            request_or_iterator: The request object or iterator.
            client_call_details: The call details including method name and metadata.

        Returns:
            The response from the cache or the actual RPC call.
        """
        # Resolve cache metadata to determine behavior
        cache_settings = self.cache.resolve_cache_metadata(client_call_details.metadata)

        # Generate cache key
        key = self.cache.key_from_proto_message(
            method_name=client_call_details.method, request=request_or_iterator
        )

        # Try to read from cache if allowed
        if cache_settings.use_cache and not cache_settings.force_refresh:
            try:
                cached_data = self.cache.get(key)
                if cached_data is not None:
                    logger.debug(f"Cache hit for `{key}`")
                    # Reconstruct the response
                    response = self._deserialize_response(cached_data)
                    if response is not None:
                        return response
                    else:
                        logger.warning(f"Failed to deserialize cached response for `{key}`")
            except diskcache.Timeout as e:
                logger.debug(f"Cache read timeout for `{key}`: {e}")
            except Exception as e:
                logger.warning(f"Failed to deserialize cached response for `{key}`: {e}")

        # Force refresh if requested
        if cache_settings.force_refresh:
            logger.debug(f"Forcing refresh for `{key}`")
            self.cache.delete(key)

        # Make the actual RPC call
        call = await method(request_or_iterator, client_call_details)

        # The call is a UnaryUnaryCall object, we need to await it to get the actual response
        response = await call

        # Cache the response if allowed
        if cache_settings.use_cache:
            try:
                # Serialize the protobuf response to bytes before caching
                cached_data = self._serialize_response(response)
                if cached_data is not None:
                    self.cache.set_with_default_ttl(key, cached_data, expire=cache_settings.custom_ttl)
                    logger.debug(f"Cached response for `{key}`")
            except diskcache.Timeout as e:
                logger.warning(f"Failed to cache response for `{key}`: {e}")

        return response

    @staticmethod
    def _serialize_response(response: message.Message) -> tuple[Any, bytes] | None:
        if isinstance(response, message.Message):
            return response.DESCRIPTOR.full_name, response.SerializeToString()
        else:
            logger.warning(f"Response is not a protobuf message: {type(response)}")
            return None

    def _deserialize_response(self, response: tuple[Any, bytes]) -> message.Message | None:
        response_type, data = response
        try:
            response_type_cls = self.symbol_db.GetSymbol(response_type)
            message = response_type_cls()
            message.ParseFromString(data)
            return message
        except Exception as e:
            logger.warning(f"Failed to deserialize response: {e}")
            return None

