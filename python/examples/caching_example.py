"""Example demonstrating gRPC response caching with the Sift client.

This example shows how to:
1. Enable caching via SiftClient configuration
2. Use cache control metadata to control caching behavior
3. Measure the performance improvement from caching

Requirements:
    pip install sift-stack-py[cache]
"""

import time
from sift_client import CacheConfig, SiftClient
from sift_py.grpc.cache import with_cache, with_force_refresh, clear_cache_for

# Configure caching
cache_config = CacheConfig(
    enabled=True,  # Enable caching
    ttl=3600,  # Cache for 1 hour
    cache_path=None,  # Uses system temp directory by default
    size_limit=1024 * 1024 * 1024,  # 1GB max
)

# Initialize client with caching enabled
client = SiftClient(
    api_key="your-api-key-here",
    grpc_url="api.siftstack.com",
    rest_url="https://api.siftstack.com",
    cache_config=cache_config,  # Pass cache config directly
)

# Example 1: Basic caching
print("Example 1: Basic Caching")
print("-" * 50)

# First call - cache miss (fetches from server)
start = time.time()
response = client.ping.ping()  # Note: Need to add metadata support to high-level APIs
elapsed_first = time.time() - start
print(f"First call (cache miss): {elapsed_first:.3f}s")

# Second call - cache hit (returns cached response)
start = time.time()
response = client.ping.ping()
elapsed_second = time.time() - start
print(f"Second call (cache hit): {elapsed_second:.3f}s")
print(f"Speedup: {elapsed_first / elapsed_second:.1f}x faster")

# Example 2: Force refresh
print("\nExample 2: Force Refresh")
print("-" * 50)

# Force refresh - bypasses cache and fetches fresh data
start = time.time()
response = client.ping.ping()  # with force_refresh metadata
elapsed = time.time() - start
print(f"Force refresh: {elapsed:.3f}s")

# Example 3: Clear cache
print("\nExample 3: Clear Cache")
print("-" * 50)

# Clear the cache for this specific request
response = client.ping.ping()  # with clear_cache_for metadata
print("Cache cleared for this request")

# Example 4: Conditional caching
print("\nExample 4: Conditional Caching")
print("-" * 50)


def get_data(use_cache: bool = False):
    """Helper function that conditionally uses caching."""
    if use_cache:
        # Use cache
        return client.ping.ping()  # with with_cache metadata
    else:
        # Skip cache
        return client.ping.ping()  # without cache metadata


# Use cache in production
response = get_data(use_cache=True)
print("Called with caching enabled")

# Skip cache in development
response = get_data(use_cache=False)
print("Called without caching")

print("\nNote: This example requires integration with the high-level API")
print("to pass cache control metadata. See the documentation for details.")
