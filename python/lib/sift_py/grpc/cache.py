"""Utilities for controlling gRPC response caching.

This module provides helper functions and constants for working with the gRPC
caching interceptor. Use these utilities to control caching behavior on a
per-request basis via metadata.

Example:
    from sift_py.grpc.cache import with_cache, with_force_refresh, clear_cache_for

    # Enable caching for a request
    metadata = with_cache()
    response = stub.GetData(request, metadata=metadata)

    # Force refresh (bypass cache and store fresh result)
    metadata = with_force_refresh()
    response = stub.GetData(request, metadata=metadata)

    # Clear cache for a specific request
    metadata = clear_cache_for()
    response = stub.GetData(request, metadata=metadata)
"""

from typing import List, Tuple

# Metadata keys for cache control
METADATA_USE_CACHE = "use-cache"
METADATA_FORCE_REFRESH = "force-refresh"
METADATA_CLEAR_CACHE = "clear-cache"


def with_cache(
    existing_metadata: List[Tuple[str, str]] | None = None,
) -> List[Tuple[str, str]]:
    """Add cache control metadata to enable caching for a request.

    Args:
        existing_metadata: Optional existing metadata to extend.

    Returns:
        Metadata list with cache enabled.

    Example:
        metadata = with_cache()
        response = stub.GetData(request, metadata=metadata)
    """
    metadata = list(existing_metadata) if existing_metadata else []
    metadata.append((METADATA_USE_CACHE, "true"))
    return metadata


def with_force_refresh(
    existing_metadata: List[Tuple[str, str]] | None = None,
) -> List[Tuple[str, str]]:
    """Add cache control metadata to force refresh (bypass cache and store fresh result).

    Args:
        existing_metadata: Optional existing metadata to extend.

    Returns:
        Metadata list with force refresh enabled.

    Example:
        metadata = with_force_refresh()
        response = stub.GetData(request, metadata=metadata)
    """
    metadata = list(existing_metadata) if existing_metadata else []
    metadata.append((METADATA_FORCE_REFRESH, "true"))
    metadata.append((METADATA_USE_CACHE, "true"))  # Also enable caching
    return metadata


def clear_cache_for(
    existing_metadata: List[Tuple[str, str]] | None = None,
) -> List[Tuple[str, str]]:
    """Add cache control metadata to clear the cache for a specific request.

    This will delete the cached entry before making the request.

    Args:
        existing_metadata: Optional existing metadata to extend.

    Returns:
        Metadata list with clear cache enabled.

    Example:
        metadata = clear_cache_for()
        response = stub.GetData(request, metadata=metadata)
    """
    metadata = list(existing_metadata) if existing_metadata else []
    metadata.append((METADATA_CLEAR_CACHE, "true"))
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
        if k not in (METADATA_USE_CACHE, METADATA_FORCE_REFRESH, METADATA_CLEAR_CACHE)
    ]
    return metadata
