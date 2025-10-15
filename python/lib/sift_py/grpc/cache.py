"""Utilities for controlling gRPC response caching.

This module provides helper functions and constants for working with the gRPC
caching interceptor. Use these utilities to control caching behavior on a
per-request basis via metadata.

Example:
    from sift_py.grpc.cache import with_cache, with_force_refresh, ignore_cache

    # Enable caching for a request
    metadata = with_cache()
    response = stub.GetData(request, metadata=metadata)

    # Force refresh (bypass cache and store fresh result)
    metadata = with_force_refresh()
    response = stub.GetData(request, metadata=metadata)

    # Ignore cache without clearing
    metadata = ignore_cache()
    response = stub.GetData(request, metadata=metadata)
"""

from __future__ import annotations

import hashlib
import logging
from pathlib import Path
from typing import TYPE_CHECKING, Any, NamedTuple

import diskcache
from google.protobuf import json_format, message

if TYPE_CHECKING:


    from sift_py.grpc.transport import SiftCacheConfig

logger = logging.getLogger(__name__)


class CacheSettings(NamedTuple):
    """Resolved cache metadata from gRPC request."""

    use_cache: bool
    force_refresh: bool
    custom_ttl: float | None

# Metadata keys for cache control
METADATA_USE_CACHE = "use-cache"
METADATA_FORCE_REFRESH = "force-refresh"
METADATA_CACHE_TTL = "cache-ttl"


class GrpcCache(diskcache.Cache):
    """Subclass of diskcache.Cache for gRPC response caching."""

    def __init__(self, config: SiftCacheConfig):
        """Initialize the cache from configuration.

        Args:
            config: Cache configuration with ttl, cache_path, size_limit, clear_on_init.
        """
        self.default_ttl = config["ttl"]
        self.cache_path = Path(config["cache_path"])
        self.size_limit = config["size_limit"]

        # Create cache directory if it doesn't exist
        self.cache_path.mkdir(parents=True, exist_ok=True)

        # Initialize parent diskcache.Cache
        super().__init__(str(self.cache_path), size_limit=self.size_limit)

        # Clear cache if requested
        if config.get("clear_on_init", False):
            logger.debug(f"Clearing cache on initialization: {self.cache_path}")
            self.clear()

        logger.debug(
            f"Cache initialized at {self.cache_path.absolute()!r} "
            f"with size {self.volume() / (1024**2):.2f} MB"
        )

    def set_with_default_ttl(self, key: str, value: Any, expire: float | None = None, **kwargs) -> bool:
        expire_time = expire if expire is not None else self.default_ttl
        return super().set(key, value, expire=expire_time, **kwargs)

    @staticmethod
    def key_from_proto_message(method_name: str | bytes, request: message.Message) -> str:
        # Serialize the request to bytes
        request_json = json_format.MessageToJson(request).encode("utf-8")

        if isinstance(method_name, str):
            method_name = method_name.encode("utf-8")

        # Create a hash of method name + request
        hasher = hashlib.sha256()
        hasher.update(method_name)
        hasher.update(request_json)

        return hasher.hexdigest()

    @staticmethod
    def resolve_cache_metadata(
         metadata: tuple[tuple[str, str], ...] | None
    ) -> CacheSettings:
        """Extract and resolve cache-related metadata fields.

        Args:
            metadata: The gRPC request metadata tuple.

        Returns:
            CacheMetadata named tuple with resolved cache control fields:
            - use_cache: bool - Whether to use caching
            - force_refresh: bool - Whether to force refresh
            - ignore_cache: bool - Whether to ignore cache
            - custom_ttl: int | None - Custom TTL if specified
            - should_read: bool - Whether to read from cache
            - should_cache: bool - Whether to cache the response

        Example:
            cache_info = cache.resolve_cache_metadata(metadata)
            if cache_info.should_read:
                cached = cache.get(key)
            if cache_info.should_cache:
                cache.set_with_default_ttl(key, response, expire=cache_info.custom_ttl)
        """
        if not metadata:
            metadata_dict = {}
        else:
            # Handle both tuple and grpc.aio.Metadata types
            metadata_dict = {}
            for key, value in metadata:
                metadata_dict[key] = value

        use_cache = metadata_dict.get(METADATA_USE_CACHE, "false").lower() == "true"

        if not use_cache:
            return CacheSettings(use_cache=False, force_refresh=False, custom_ttl=None)

        force_refresh = metadata_dict.get(METADATA_FORCE_REFRESH, "false").lower() == "true"
        custom_ttl_str = metadata_dict.get(METADATA_CACHE_TTL)

        # Parse custom TTL if provided
        custom_ttl = None
        if custom_ttl_str:
            try:
                custom_ttl = int(custom_ttl_str)
            except ValueError:
                logger.warning(f"Invalid cache TTL value: {custom_ttl_str}, using default")

        return CacheSettings(
            use_cache=use_cache,
            force_refresh=force_refresh,
            custom_ttl=custom_ttl,
        )


def with_cache(ttl: int | None = None) -> tuple[tuple[str, str], ...]:
    """Enable caching for a gRPC request.
    
    Args:
        ttl: Optional custom TTL in seconds. If not provided, uses the default TTL.
    
    Returns:
        Metadata tuple to pass to the gRPC stub method.
    
    Example:
        metadata = with_cache()
        response = stub.GetData(request, metadata=metadata)
        
        # With custom TTL
        metadata = with_cache(ttl=7200)  # 2 hours
        response = stub.GetData(request, metadata=metadata)
    """
    metadata = [(METADATA_USE_CACHE, "true")]
    if ttl is not None:
        metadata.append((METADATA_CACHE_TTL, str(ttl)))
    return tuple(metadata)


def with_force_refresh(ttl: int | None = None) -> tuple[tuple[str, str], ...]:
    """Force refresh the cache for a gRPC request.
    
    Bypasses the cache, fetches fresh data from the server, and stores the result.
    
    Args:
        ttl: Optional custom TTL in seconds. If not provided, uses the default TTL.
    
    Returns:
        Metadata tuple to pass to the gRPC stub method.
    
    Example:
        metadata = with_force_refresh()
        response = stub.GetData(request, metadata=metadata)
    """
    metadata = [
        (METADATA_USE_CACHE, "true"),
        (METADATA_FORCE_REFRESH, "true"),
    ]
    if ttl is not None:
        metadata.append((METADATA_CACHE_TTL, str(ttl)))
    return tuple(metadata)
