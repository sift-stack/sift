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

from typing import List, Tuple

# Metadata keys for cache control
METADATA_USE_CACHE = "use-cache"
METADATA_FORCE_REFRESH = "force-refresh"
METADATA_IGNORE_CACHE = "ignore-cache"
METADATA_CACHE_TTL = "cache-ttl"


def with_cache(
    existing_metadata: List[Tuple[str, str]] | None = None,
    ttl: int | None = None,
) -> List[Tuple[str, str]]:
    """Add cache control metadata to enable caching for a request.

    Args:
        existing_metadata: Optional existing metadata to extend.
        ttl: Optional custom TTL in seconds for this specific request.

    Returns:
        Metadata list with cache enabled.

    Example:
        # Use default TTL
        metadata = with_cache()
        response = stub.GetData(request, metadata=metadata)

        # Use custom TTL (5 minutes)
        metadata = with_cache(ttl=300)
        response = stub.GetData(request, metadata=metadata)
    """
    metadata = list(existing_metadata) if existing_metadata else []
    metadata.append((METADATA_USE_CACHE, "true"))
    if ttl is not None:
        metadata.append((METADATA_CACHE_TTL, str(ttl)))
    return metadata


def with_force_refresh(
    existing_metadata: List[Tuple[str, str]] | None = None,
    ttl: int | None = None,
) -> List[Tuple[str, str]]:
    """Add cache control metadata to force refresh (bypass cache and store fresh result).

    Args:
        existing_metadata: Optional existing metadata to extend.
        ttl: Optional custom TTL in seconds for the refreshed entry.

    Returns:
        Metadata list with force refresh enabled.

    Example:
        # Force refresh with default TTL
        metadata = with_force_refresh()
        response = stub.GetData(request, metadata=metadata)

        # Force refresh with custom TTL
        metadata = with_force_refresh(ttl=600)
        response = stub.GetData(request, metadata=metadata)
    """
    metadata = list(existing_metadata) if existing_metadata else []
    metadata.append((METADATA_FORCE_REFRESH, "true"))
    metadata.append((METADATA_USE_CACHE, "true"))  # Also enable caching
    if ttl is not None:
        metadata.append((METADATA_CACHE_TTL, str(ttl)))
    return metadata


def ignore_cache(
    existing_metadata: List[Tuple[str, str]] | None = None,
) -> List[Tuple[str, str]]:
    """Add metadata to ignore cache for this request without clearing it.

    This is useful when you want to bypass the cache for a specific call
    but don't want to clear the cached entry.

    Args:
        existing_metadata: Optional existing metadata to extend.

    Returns:
        Metadata list with ignore cache flag.

    Example:
        metadata = ignore_cache()
        response = stub.GetData(request, metadata=metadata)
    """
    metadata = list(existing_metadata) if existing_metadata else []
    metadata.append((METADATA_IGNORE_CACHE, "true"))
    return metadata


def without_cache(
    existing_metadata: List[Tuple[str, str]] | None = None,
) -> List[Tuple[str, str]]:
    """Explicitly disable caching for a request.

    This is the default behavior, so this function is mainly for clarity.

    Args:
        existing_metadata: Optional existing metadata to extend.

    Returns:
        Metadata list without cache flags.

    Example:
        metadata = without_cache()
        response = stub.GetData(request, metadata=metadata)
    """
    metadata = list(existing_metadata) if existing_metadata else []
    # Remove any cache-related metadata
    metadata = [
        (k, v)
        for k, v in metadata
        if k not in (METADATA_USE_CACHE, METADATA_FORCE_REFRESH, METADATA_IGNORE_CACHE, METADATA_CACHE_TTL)
    ]
    return metadata
