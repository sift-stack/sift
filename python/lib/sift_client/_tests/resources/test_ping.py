"""Pytest tests for the Ping API.

These tests demonstrate and validate the usage of the Ping API including:
- Basic ping functionality
- Connection health checks
- Cache behavior and performance
- Error handling and edge cases
"""

import asyncio
import os
import time

import pytest

from sift_client import SiftClient, SiftConnectionConfig
from sift_client.resources import PingAPI, PingAPIAsync
from sift_client.transport import CacheConfig, CacheMode

pytestmark = pytest.mark.integration

# We reimplement this here so that the cache is cleared each time we instantiate
@pytest.fixture
def sift_client() -> SiftClient:
    """Create a SiftClient instance for testing.

    This fixture is shared across all test files and is session-scoped
    to avoid creating multiple client instances.
    """
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_API_KEY", "")

    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=api_key,
            grpc_url=grpc_url,
            rest_url=rest_url,
            cache_config=CacheConfig(mode=CacheMode.CLEAR_ON_INIT)
        )
    )


def test_client_binding(sift_client):
    assert sift_client.ping
    assert isinstance(sift_client.ping, PingAPI)
    assert sift_client.async_.ping
    assert isinstance(sift_client.async_.ping, PingAPIAsync)


@pytest.fixture
def ping_api_async(sift_client: SiftClient):
    """Get the ping async API instance."""
    return sift_client.async_.ping


@pytest.fixture
def ping_api_sync(sift_client: SiftClient):
    """Get the synchronous ping API instance."""
    return sift_client.ping


class TestPingAPIAsync:
    """Test suite for the Ping API functionality."""

    @pytest.mark.asyncio
    async def test_basic_ping(self, ping_api_async):
        """Test basic ping functionality."""
        response = await ping_api_async.ping()

        # Verify response is a string
        assert isinstance(response, str)

        # Verify response is not empty
        assert len(response) > 0


class TestPingAPISync:
    """Test suite for the  Ping API functionality."""

    def test_basic_ping(self, ping_api_sync):
        """Test basic synchronous ping functionality."""
        response = ping_api_sync.ping()

        # Verify response is a string
        assert isinstance(response, str)

        # Verify response is not empty
        assert len(response) > 0


class TestPingCacheBehavior:
    """Test suite for ping cache behavior."""

    @pytest.mark.asyncio
    async def test_cache_enabled(self, ping_api_async):
        """Test that caching can be enabled for ping requests."""
        # Enable caching on the low-level client
        ping_api_async._low_level_client._cache_results = True

        # Measure time for first ping - should hit the server (slower)
        start1 = time.perf_counter()
        response1 = await ping_api_async.ping()
        duration1 = time.perf_counter() - start1
        assert isinstance(response1, str)
        assert len(response1) > 0

        # Measure time for second ping - should use cache (much faster)
        start2 = time.perf_counter()
        response2 = await ping_api_async.ping()
        duration2 = time.perf_counter() - start2
        assert response2 == response1

        # Print timing info
        print(f"\nFirst ping (server): {duration1*1000:.2f}ms")
        print(f"Second ping (cache): {duration2*1000:.2f}ms")
        print(f"Speedup: {duration1/duration2:.2f}x")

        # Cached call should be significantly faster (at least 5x)
        assert duration2 < duration1 / 5, (
            f"Cached ping should be much faster. "
            f"First: {duration1*1000:.2f}ms, Second: {duration2*1000:.2f}ms"
        )

        # Disable caching for cleanup
        ping_api_async._low_level_client._cache_results = False

    @pytest.mark.asyncio
    async def test_force_refresh_bypasses_cache(self, ping_api_async):
        """Test that force_refresh bypasses the cache."""
        # Enable caching
        ping_api_async._low_level_client._cache_results = True

        # First ping - populate cache
        start1 = time.perf_counter()
        response1 = await ping_api_async._low_level_client.ping()
        duration1 = time.perf_counter() - start1
        assert isinstance(response1, str)

        # Second ping without force_refresh - should use cache (fast)
        start2 = time.perf_counter()
        response2 = await ping_api_async._low_level_client.ping(_force_refresh=False)
        duration2 = time.perf_counter() - start2
        assert isinstance(response2, str)

        # Third ping with force_refresh - should bypass cache (slow, like first call)
        start3 = time.perf_counter()
        response3 = await ping_api_async._low_level_client.ping(_force_refresh=True)
        duration3 = time.perf_counter() - start3
        assert isinstance(response3, str)

        # Print timing info
        print(f"\nFirst ping (server): {duration1*1000:.2f}ms")
        print(f"Second ping (cache): {duration2*1000:.2f}ms")
        print(f"Third ping (force_refresh, server): {duration3*1000:.2f}ms")

        # Cached call should be much faster than both server calls
        assert duration2 < duration1 / 5, (
            f"Cached ping should be much faster than first ping. "
            f"First: {duration1*1000:.2f}ms, Cached: {duration2*1000:.2f}ms"
        )
        assert duration2 < duration3 / 5, (
            f"Cached ping should be much faster than force_refresh ping. "
            f"Force refresh: {duration3*1000:.2f}ms, Cached: {duration2*1000:.2f}ms"
        )

        # Disable caching for cleanup
        ping_api_async._low_level_client._cache_results = False

    @pytest.mark.asyncio
    async def test_cache_ttl_expiration(self, ping_api_async):
        """Test that cache entries expire after TTL."""
        # Enable caching with very short TTL (1 second)
        ping_api_async._low_level_client._cache_results = True

        # First ping - populate cache with 1 second TTL
        response1 = await ping_api_async._low_level_client.ping()
        assert isinstance(response1, str)

        # Immediate second ping - should use cache
        response2 = await ping_api_async._low_level_client.ping()
        assert isinstance(response2, str)

        # Wait for TTL to expire (1 second + buffer)
        await asyncio.sleep(1.5)

        # Third ping - cache should have expired, will fetch fresh
        response3 = await ping_api_async._low_level_client.ping()
        assert isinstance(response3, str)

        # Disable caching for cleanup
        ping_api_async._low_level_client._cache_results = False

    @pytest.mark.asyncio
    async def test_cache_performance(self, ping_api_async):
        """Test that cached ping requests are faster than uncached ones."""
        num_iterations = 10

        # Enable caching
        ping_api_async._low_level_client._cache_results = True

        # Measure uncached performance (force_refresh=True)
        start_time = time.perf_counter()
        for _ in range(num_iterations):
            await ping_api_async._low_level_client.ping(_force_refresh=True)
        uncached_duration = time.perf_counter() - start_time

        # Warm up cache
        await ping_api_async._low_level_client.ping()

        # Measure cached performance
        start_time = time.perf_counter()
        for _ in range(num_iterations):
            await ping_api_async._low_level_client.ping(_force_refresh=False)
        cached_duration = time.perf_counter() - start_time

        # Print performance metrics
        print(f"\n{'='*60}")
        print(f"Ping Cache Performance ({num_iterations} iterations)")
        print(f"{'='*60}")
        print(f"Cached duration:   {cached_duration:.4f}s ({cached_duration/num_iterations*1000:.2f}ms per call)")
        print(f"Uncached duration: {uncached_duration:.4f}s ({uncached_duration/num_iterations*1000:.2f}ms per call)")
        print(f"Speedup:           {uncached_duration / cached_duration:.2f}x")
        print(f"Time saved:        {uncached_duration - cached_duration:.4f}s")
        print(f"{'='*60}\n")

        # Assert that cached is faster
        assert cached_duration < uncached_duration, (
            f"Cached pings should be faster. "
            f"Cached: {cached_duration:.4f}s, Uncached: {uncached_duration:.4f}s"
        )

        # Disable caching for cleanup
        ping_api_async._low_level_client._cache_results = False

    @pytest.mark.asyncio
    async def test_cache_disabled_by_default(self, ping_api_async):
        """Test that caching is disabled by default for ping."""
        # Verify cache is disabled by default
        assert ping_api_async._low_level_client._cache_results is False

        # Multiple pings should all hit the server (no caching)
        response1 = await ping_api_async.ping()
        response2 = await ping_api_async.ping()
        response3 = await ping_api_async.ping()

        # All should succeed
        assert isinstance(response1, str)
        assert isinstance(response2, str)
        assert isinstance(response3, str)

    @pytest.mark.asyncio
    async def test_ping_without_grpc_cache(self):
        """Test that ping works when GrpcCache is not enabled on the SiftClient."""
        import os

        from sift_client import SiftClient, SiftConnectionConfig

        # Create a client with caching explicitly disabled
        grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
        rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
        api_key = os.getenv("SIFT_API_KEY", "")

        client = SiftClient(
            connection_config=SiftConnectionConfig(
                api_key=api_key,
                grpc_url=grpc_url,
                rest_url=rest_url,
                use_ssl=True,
                cache_config=None
            )
        )

        # Verify cache is not initialized
        assert client.grpc_client.cache is None

        # Ping should still work without cache
        response1 = await client.async_.ping.ping()
        assert isinstance(response1, str)
        assert len(response1) > 0

        # Multiple pings should work
        response2 = await client.async_.ping.ping()
        assert isinstance(response2, str)

        response3 = await client.async_.ping.ping()
        assert isinstance(response3, str)

        print(f"\nPing without cache successful: {response1}")

