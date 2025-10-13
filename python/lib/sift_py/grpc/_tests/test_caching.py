"""Tests for gRPC caching interceptor."""

import tempfile
from pathlib import Path
from unittest.mock import MagicMock, Mock

import grpc
import pytest

from sift_py.grpc._interceptors.caching import CachingInterceptor
from sift_py.grpc._interceptors.context import ClientCallDetails
from sift_py.grpc.cache import (
    clear_cache_for,
    with_cache,
    with_force_refresh,
    without_cache,
)


class MockRequest:
    """Mock protobuf request for testing."""

    def __init__(self, data: str):
        self.data = data

    def SerializeToString(self) -> bytes:
        return self.data.encode()


class MockResponse:
    """Mock protobuf response for testing."""

    def __init__(self, value: str):
        self.value = value


@pytest.fixture
def temp_cache_dir():
    """Create a temporary cache directory."""
    with tempfile.TemporaryDirectory() as tmpdir:
        yield tmpdir


@pytest.fixture
def interceptor(temp_cache_dir):
    """Create a caching interceptor with a temporary cache directory."""
    return CachingInterceptor(ttl=60, cache_path=temp_cache_dir)


def test_cache_miss_and_hit(interceptor):
    """Test that cache miss fetches from server and cache hit returns cached response."""
    # Setup
    request = MockRequest("test-data")
    response = MockResponse("test-response")
    method_name = "/test.Service/TestMethod"

    # Create mock continuation
    continuation = Mock(return_value=response)

    # Create call details with cache enabled
    call_details = ClientCallDetails(
        method=method_name,
        timeout=None,
        metadata=with_cache(),
        credentials=None,
        wait_for_ready=None,
    )

    # First call - cache miss
    result1 = interceptor.intercept(continuation, request, call_details)
    assert result1 == response
    assert continuation.call_count == 1

    # Second call - cache hit
    result2 = interceptor.intercept(continuation, request, call_details)
    assert result2 == response
    assert continuation.call_count == 1  # Should not call continuation again


def test_cache_disabled_by_default(interceptor):
    """Test that caching is disabled by default without metadata."""
    # Setup
    request = MockRequest("test-data")
    response = MockResponse("test-response")
    method_name = "/test.Service/TestMethod"

    # Create mock continuation
    continuation = Mock(return_value=response)

    # Create call details without cache metadata
    call_details = ClientCallDetails(
        method=method_name,
        timeout=None,
        metadata=None,
        credentials=None,
        wait_for_ready=None,
    )

    # First call
    result1 = interceptor.intercept(continuation, request, call_details)
    assert result1 == response
    assert continuation.call_count == 1

    # Second call - should call continuation again (no caching)
    result2 = interceptor.intercept(continuation, request, call_details)
    assert result2 == response
    assert continuation.call_count == 2


def test_force_refresh(interceptor):
    """Test that force refresh bypasses cache and stores fresh result."""
    # Setup
    request = MockRequest("test-data")
    response1 = MockResponse("response-1")
    response2 = MockResponse("response-2")
    method_name = "/test.Service/TestMethod"

    # Create mock continuation that returns different responses
    continuation = Mock(side_effect=[response1, response2])

    # First call with cache
    call_details = ClientCallDetails(
        method=method_name,
        timeout=None,
        metadata=with_cache(),
        credentials=None,
        wait_for_ready=None,
    )
    result1 = interceptor.intercept(continuation, request, call_details)
    assert result1 == response1
    assert continuation.call_count == 1

    # Second call with force refresh
    call_details_refresh = ClientCallDetails(
        method=method_name,
        timeout=None,
        metadata=with_force_refresh(),
        credentials=None,
        wait_for_ready=None,
    )
    result2 = interceptor.intercept(continuation, request, call_details_refresh)
    assert result2 == response2
    assert continuation.call_count == 2  # Should call continuation again


def test_clear_cache(interceptor):
    """Test that clear cache deletes the cached entry."""
    # Setup
    request = MockRequest("test-data")
    response1 = MockResponse("response-1")
    response2 = MockResponse("response-2")
    method_name = "/test.Service/TestMethod"

    # Create mock continuation
    continuation = Mock(side_effect=[response1, response2])

    # First call with cache
    call_details = ClientCallDetails(
        method=method_name,
        timeout=None,
        metadata=with_cache(),
        credentials=None,
        wait_for_ready=None,
    )
    result1 = interceptor.intercept(continuation, request, call_details)
    assert result1 == response1
    assert continuation.call_count == 1

    # Second call with clear cache
    call_details_clear = ClientCallDetails(
        method=method_name,
        timeout=None,
        metadata=clear_cache_for(),
        credentials=None,
        wait_for_ready=None,
    )
    result2 = interceptor.intercept(continuation, request, call_details_clear)
    assert result2 == response2
    assert continuation.call_count == 2  # Should call continuation again


def test_different_requests_different_cache_keys(interceptor):
    """Test that different requests generate different cache keys."""
    # Setup
    request1 = MockRequest("data-1")
    request2 = MockRequest("data-2")
    response1 = MockResponse("response-1")
    response2 = MockResponse("response-2")
    method_name = "/test.Service/TestMethod"

    # Create mock continuation
    continuation = Mock(side_effect=[response1, response2])

    # First call with request1
    call_details = ClientCallDetails(
        method=method_name,
        timeout=None,
        metadata=with_cache(),
        credentials=None,
        wait_for_ready=None,
    )
    result1 = interceptor.intercept(continuation, request1, call_details)
    assert result1 == response1
    assert continuation.call_count == 1

    # Second call with request2 (different request)
    result2 = interceptor.intercept(continuation, request2, call_details)
    assert result2 == response2
    assert continuation.call_count == 2  # Should call continuation for different request


def test_cache_key_generation(interceptor):
    """Test that cache key generation is deterministic."""
    request = MockRequest("test-data")
    method_name = "/test.Service/TestMethod"

    key1 = interceptor._generate_cache_key(method_name, request)
    key2 = interceptor._generate_cache_key(method_name, request)

    assert key1 == key2
    assert len(key1) == 64  # SHA256 hex digest length


def test_without_cache_helper():
    """Test the without_cache helper function."""
    metadata = without_cache()
    assert len(metadata) == 0

    # Test with existing metadata
    existing = [("key", "value")]
    metadata = without_cache(existing)
    assert ("key", "value") in metadata
    assert ("use-cache", "true") not in metadata


def test_with_cache_helper():
    """Test the with_cache helper function."""
    metadata = with_cache()
    assert ("use-cache", "true") in metadata


def test_with_force_refresh_helper():
    """Test the with_force_refresh helper function."""
    metadata = with_force_refresh()
    assert ("force-refresh", "true") in metadata
    assert ("use-cache", "true") in metadata


def test_clear_cache_for_helper():
    """Test the clear_cache_for helper function."""
    metadata = clear_cache_for()
    assert ("clear-cache", "true") in metadata


def test_clear_all(interceptor):
    """Test clearing all cached entries."""
    # Setup
    request = MockRequest("test-data")
    response = MockResponse("test-response")
    method_name = "/test.Service/TestMethod"

    # Create mock continuation
    continuation = Mock(return_value=response)

    # Create call details with cache enabled
    call_details = ClientCallDetails(
        method=method_name,
        timeout=None,
        metadata=with_cache(),
        credentials=None,
        wait_for_ready=None,
    )

    # First call - cache miss
    result1 = interceptor.intercept(continuation, request, call_details)
    assert result1 == response
    assert continuation.call_count == 1

    # Clear all cache
    interceptor.clear_all()

    # Second call - should be cache miss again
    result2 = interceptor.intercept(continuation, request, call_details)
    assert result2 == response
    assert continuation.call_count == 2


def test_context_manager(temp_cache_dir):
    """Test that the interceptor works as a context manager."""
    with CachingInterceptor(ttl=60, cache_path=temp_cache_dir) as interceptor:
        assert interceptor is not None
        # Cache should be usable within the context
        request = MockRequest("test")
        key = interceptor._generate_cache_key("/test", request)
        assert key is not None


def test_cache_persistence(temp_cache_dir):
    """Test that cache persists across interceptor instances."""
    request = MockRequest("test-data")
    response = MockResponse("test-response")
    method_name = "/test.Service/TestMethod"

    # Create first interceptor and cache a response
    interceptor1 = CachingInterceptor(ttl=60, cache_path=temp_cache_dir)
    continuation = Mock(return_value=response)
    call_details = ClientCallDetails(
        method=method_name,
        timeout=None,
        metadata=with_cache(),
        credentials=None,
        wait_for_ready=None,
    )
    result1 = interceptor1.intercept(continuation, request, call_details)
    assert result1 == response
    assert continuation.call_count == 1
    interceptor1.close()

    # Create second interceptor with same cache path
    interceptor2 = CachingInterceptor(ttl=60, cache_path=temp_cache_dir)
    continuation2 = Mock(return_value=response)
    result2 = interceptor2.intercept(continuation2, request, call_details)
    assert result2 == response
    assert continuation2.call_count == 0  # Should use cached response
    interceptor2.close()


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
