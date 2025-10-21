# ruff: noqa: N802

import logging
import tempfile
from concurrent import futures
from contextlib import contextmanager
from pathlib import Path
from typing import Any, Callable, Iterator, cast

import grpc
import pytest
from pytest_mock import MockFixture, MockType
from sift.data.v2.data_pb2 import GetDataRequest, GetDataResponse
from sift.data.v2.data_pb2_grpc import (
    DataServiceServicer,
    DataServiceStub,
    add_DataServiceServicer_to_server,
)

from sift_py._internal.test_util.server_interceptor import ServerInterceptor
from sift_py.grpc.cache import (
    GrpcCache,
    ignore_cache,
    with_cache,
    with_force_refresh,
    without_cache,
)
from sift_py.grpc.transport import SiftChannelConfig, use_sift_async_channel

# Enable debug logging for cache-related modules
logging.basicConfig(
    level=logging.DEBUG, format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
logging.getLogger("sift_py").setLevel(logging.DEBUG)


class DataService(DataServiceServicer):
    """Mock data service that returns a unique response each time."""

    call_count: int

    def __init__(self):
        self.call_count = 0

    def GetData(self, request: GetDataRequest, context: grpc.ServicerContext):
        self.call_count += 1
        # Return a unique token each time to verify caching
        return GetDataResponse(next_page_token=f"token-{self.call_count}")


class AuthInterceptor(ServerInterceptor):
    """Simple auth interceptor that checks for Bearer token."""

    def intercept(
        self,
        method: Callable,
        request_or_iterator: Any,
        context: grpc.ServicerContext,
        method_name: str,
    ) -> Any:
        authenticated = False
        for metadata in context.invocation_metadata():
            if metadata.key == "authorization":
                if metadata.value.startswith("Bearer "):
                    authenticated = True
                break

        if authenticated:
            return method(request_or_iterator, context)
        else:
            context.set_code(grpc.StatusCode.UNAUTHENTICATED)
            context.set_details("Invalid or missing API key")
            raise


@contextmanager
def server_with_service(mocker: MockFixture) -> Iterator[tuple[MockType, DataService, int]]:
    """Create a test server with a spy on the DataService.

    Returns:
        Tuple of (spy, data_service, port)
    """
    server = grpc.server(
        thread_pool=futures.ThreadPoolExecutor(max_workers=1),
        interceptors=[AuthInterceptor()],
    )

    data_service = DataService()
    spy = mocker.spy(data_service, "GetData")

    add_DataServiceServicer_to_server(data_service, server)
    # Use port 0 to let the OS assign an available port
    port = server.add_insecure_port("[::]:0")
    server.start()
    try:
        yield spy, data_service, port
    finally:
        server.stop(None)
        server.wait_for_termination()


def test_cache_helper_functions():
    """Test the cache metadata helper functions."""
    # Test with_cache
    metadata = with_cache()
    assert metadata == (("use-cache", "true"),)

    # Test with_cache with custom TTL
    metadata = with_cache(ttl=7200)
    assert metadata == (("use-cache", "true"), ("cache-ttl", "7200"))

    # Test with_force_refresh
    metadata = with_force_refresh()
    assert metadata == (("use-cache", "true"), ("force-refresh", "true"))

    # Test with_force_refresh with custom TTL
    metadata = with_force_refresh(ttl=3600)
    assert metadata == (("use-cache", "true"), ("force-refresh", "true"), ("cache-ttl", "3600"))

    # Test ignore_cache
    metadata = ignore_cache()
    assert metadata == ()

    # Test without_cache
    metadata = without_cache()
    assert metadata == ()


def test_grpc_cache_initialization():
    """Test GrpcCache initialization and configuration."""
    with tempfile.TemporaryDirectory() as tmpdir:
        cache_config = {
            "ttl": 1800,
            "cache_path": str(Path(tmpdir) / "test_cache"),
            "size_limit": 1024 * 1024,  # 1MB
            "clear_on_init": False,
        }

        cache = GrpcCache(cache_config)
        assert cache.default_ttl == 1800
        assert cache.cache_path == Path(tmpdir) / "test_cache"
        assert cache.size_limit == 1024 * 1024
        assert cache.cache_path.exists()

        # Test clear_on_init
        cache.set("test-key", "test-value")
        assert cache.get("test-key") == "test-value"

        cache_config["clear_on_init"] = True
        cache2 = GrpcCache(cache_config)
        assert cache2.get("test-key") is None


def test_cache_key_generation():
    """Test deterministic cache key generation."""
    request1 = GetDataRequest(page_size=100)
    request2 = GetDataRequest(page_size=100)
    request3 = GetDataRequest(page_size=200)

    key1 = GrpcCache.key_from_proto_message("/sift.data.v2.DataService/GetData", request1)
    key2 = GrpcCache.key_from_proto_message("/sift.data.v2.DataService/GetData", request2)
    key3 = GrpcCache.key_from_proto_message("/sift.data.v2.DataService/GetData", request3)

    # Same request should generate same key
    assert key1 == key2

    # Different request should generate different key
    assert key1 != key3

    # Keys should be SHA256 hashes (64 hex characters)
    assert len(key1) == 64
    assert all(c in "0123456789abcdef" for c in key1)


def test_cache_metadata_resolution():
    """Test cache metadata resolution logic."""
    # No metadata
    settings = GrpcCache.resolve_cache_metadata(None)
    assert settings.use_cache is False
    assert settings.force_refresh is False
    assert settings.custom_ttl is None

    # use-cache enabled
    settings = GrpcCache.resolve_cache_metadata((("use-cache", "true"),))
    assert settings.use_cache is True
    assert settings.force_refresh is False
    assert settings.custom_ttl is None

    # force-refresh enabled
    settings = GrpcCache.resolve_cache_metadata((("use-cache", "true"), ("force-refresh", "true")))
    assert settings.use_cache is True
    assert settings.force_refresh is True
    assert settings.custom_ttl is None

    # Custom TTL
    settings = GrpcCache.resolve_cache_metadata((("use-cache", "true"), ("cache-ttl", "7200")))
    assert settings.use_cache is True
    assert settings.force_refresh is False
    assert settings.custom_ttl == 7200

    # Invalid TTL (should be ignored)
    settings = GrpcCache.resolve_cache_metadata((("use-cache", "true"), ("cache-ttl", "invalid")))
    assert settings.use_cache is True
    assert settings.custom_ttl is None


@pytest.mark.asyncio
async def test_basic_caching(mocker: MockFixture):
    """Test basic cache hit and miss scenarios."""
    with tempfile.TemporaryDirectory() as tmpdir:
        with server_with_service(mocker) as (get_data_spy, data_service, port):
            config: SiftChannelConfig = {
                "uri": f"localhost:{port}",
                "apikey": "test-token",
                "use_ssl": False,
                "cache_config": {
                    "ttl": 3600,
                    "cache_path": str(Path(tmpdir) / "cache"),
                    "size_limit": 1024 * 1024,
                    "clear_on_init": True,
                },
            }
            cache = GrpcCache(config["cache_config"])

            async with use_sift_async_channel(config, cache=cache) as channel:
                stub = DataServiceStub(channel)
                request = GetDataRequest(page_size=100)

                # First call without cache - should hit server
                res1 = cast(GetDataResponse, await stub.GetData(request))
                assert res1.next_page_token == "token-1"
                assert data_service.call_count == 1

                # Second call without cache - should hit server again
                res2 = cast(GetDataResponse, await stub.GetData(request))
                assert res2.next_page_token == "token-2"
                assert data_service.call_count == 2

                # Third call WITH cache - should hit server
                res3 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res3.next_page_token == "token-3"
                assert data_service.call_count == 3

                # Fourth call WITH cache - should use cached response
                res4 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res4.next_page_token == "token-3"  # Same as res3!
                assert data_service.call_count == 3  # No new call

                # Fifth call WITH cache - should still use cached response
                res5 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res5.next_page_token == "token-3"
                assert data_service.call_count == 3


@pytest.mark.asyncio
async def test_force_refresh(mocker: MockFixture):
    """Test force refresh bypasses cache and updates it."""
    with tempfile.TemporaryDirectory() as tmpdir:
        with server_with_service(mocker) as (get_data_spy, data_service, port):
            config: SiftChannelConfig = {
                "uri": f"localhost:{port}",
                "apikey": "test-token",
                "use_ssl": False,
                "cache_config": {
                    "ttl": 3600,
                    "cache_path": str(Path(tmpdir) / "cache"),
                    "size_limit": 1024 * 1024,
                    "clear_on_init": True,
                },
            }
            cache = GrpcCache(config["cache_config"])

            async with use_sift_async_channel(config, cache=cache) as channel:
                stub = DataServiceStub(channel)
                request = GetDataRequest(page_size=100)

                # First call with cache
                res1 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res1.next_page_token == "token-1"
                assert data_service.call_count == 1

                # Second call with cache - should use cached
                res2 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res2.next_page_token == "token-1"
                assert data_service.call_count == 1

                # Force refresh - should hit server and update cache
                res3 = cast(
                    GetDataResponse, await stub.GetData(request, metadata=with_force_refresh())
                )
                assert res3.next_page_token == "token-2"
                assert data_service.call_count == 2

                # Next call with cache should use the refreshed value
                res4 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res4.next_page_token == "token-2"
                assert data_service.call_count == 2


@pytest.mark.asyncio
async def test_ignore_cache(mocker: MockFixture):
    """Test ignore_cache bypasses cache without updating it."""
    with tempfile.TemporaryDirectory() as tmpdir:
        with server_with_service(mocker) as (get_data_spy, data_service, port):
            config: SiftChannelConfig = {
                "uri": f"localhost:{port}",
                "apikey": "test-token",
                "use_ssl": False,
                "cache_config": {
                    "ttl": 3600,
                    "cache_path": str(Path(tmpdir) / "cache"),
                    "size_limit": 1024 * 1024,
                    "clear_on_init": True,
                },
            }
            cache = GrpcCache(config["cache_config"])

            async with use_sift_async_channel(config, cache=cache) as channel:
                stub = DataServiceStub(channel)
                request = GetDataRequest(page_size=100)

                # First call with cache
                res1 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res1.next_page_token == "token-1"
                assert data_service.call_count == 1

                # Call with ignore_cache - should hit server
                res2 = cast(GetDataResponse, await stub.GetData(request, metadata=ignore_cache()))
                assert res2.next_page_token == "token-2"
                assert data_service.call_count == 2

                # Call with cache again - should still have original cached value
                res3 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res3.next_page_token == "token-1"  # Original cached value
                assert data_service.call_count == 2


@pytest.mark.asyncio
async def test_different_requests_different_cache_keys(mocker: MockFixture):
    """Test that different requests use different cache entries."""
    with tempfile.TemporaryDirectory() as tmpdir:
        with server_with_service(mocker) as (get_data_spy, data_service, port):
            config: SiftChannelConfig = {
                "uri": f"localhost:{port}",
                "apikey": "test-token",
                "use_ssl": False,
                "cache_config": {
                    "ttl": 3600,
                    "cache_path": str(Path(tmpdir) / "cache"),
                    "size_limit": 1024 * 1024,
                    "clear_on_init": True,
                },
            }
            cache = GrpcCache(config["cache_config"])

            async with use_sift_async_channel(config, cache=cache) as channel:
                stub = DataServiceStub(channel)
                request1 = GetDataRequest(page_size=100)
                request2 = GetDataRequest(page_size=200)

                # First request with cache
                res1 = cast(GetDataResponse, await stub.GetData(request1, metadata=with_cache()))
                assert res1.next_page_token == "token-1"
                assert data_service.call_count == 1

                # Different request with cache - should hit server
                res2 = cast(GetDataResponse, await stub.GetData(request2, metadata=with_cache()))
                assert res2.next_page_token == "token-2"
                assert data_service.call_count == 2

                # First request again - should use cache
                res3 = cast(GetDataResponse, await stub.GetData(request1, metadata=with_cache()))
                assert res3.next_page_token == "token-1"
                assert data_service.call_count == 2

                # Second request again - should use cache
                res4 = cast(GetDataResponse, await stub.GetData(request2, metadata=with_cache()))
                assert res4.next_page_token == "token-2"
                assert data_service.call_count == 2


@pytest.mark.asyncio
async def test_cache_persists_across_channels(mocker: MockFixture):
    """Test that cache persists across different channel instances."""
    with tempfile.TemporaryDirectory() as tmpdir:
        cache_path = str(Path(tmpdir) / "cache")

        with server_with_service(mocker) as (get_data_spy, data_service, port):
            config: SiftChannelConfig = {
                "uri": f"localhost:{port}",
                "apikey": "test-token",
                "use_ssl": False,
                "cache_config": {
                    "ttl": 3600,
                    "cache_path": str(Path(tmpdir) / "cache"),
                    "size_limit": 1024 * 1024,
                    "clear_on_init": False,
                },
            }
            cache = GrpcCache(config["cache_config"])

            # First channel - populate cache
            async with use_sift_async_channel(config, cache=cache) as channel1:
                stub1 = DataServiceStub(channel1)
                request = GetDataRequest(page_size=100)
                res1 = cast(GetDataResponse, await stub1.GetData(request, metadata=with_cache()))
                assert res1.next_page_token == "token-1"
                assert data_service.call_count == 1

            # Second channel - should use cached value
            async with use_sift_async_channel(config, cache=cache) as channel2:
                stub2 = DataServiceStub(channel2)
                request = GetDataRequest(page_size=100)
                res2 = cast(GetDataResponse, await stub2.GetData(request, metadata=with_cache()))
                assert res2.next_page_token == "token-1"  # Same as first call
                assert data_service.call_count == 1  # No new server call


@pytest.mark.asyncio
async def test_custom_ttl(mocker: MockFixture):
    """Test custom TTL parameter."""
    with tempfile.TemporaryDirectory() as tmpdir:
        with server_with_service(mocker) as (get_data_spy, data_service, port):
            config: SiftChannelConfig = {
                "uri": f"localhost:{port}",
                "apikey": "test-token",
                "use_ssl": False,
                "cache_config": {
                    "ttl": 3600,
                    "cache_path": str(Path(tmpdir) / "cache"),
                    "size_limit": 1024 * 1024,
                    "clear_on_init": True,
                },
            }

            cache = GrpcCache(config["cache_config"])

            async with use_sift_async_channel(config, cache=cache) as channel:
                stub = DataServiceStub(channel)
                request = GetDataRequest(page_size=100)

                # Call with custom TTL
                res1 = cast(
                    GetDataResponse, await stub.GetData(request, metadata=with_cache(ttl=7200))
                )
                assert res1.next_page_token == "token-1"
                assert data_service.call_count == 1

                # Verify it's cached
                res2 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res2.next_page_token == "token-1"
                assert data_service.call_count == 1


@pytest.mark.asyncio
async def test_metadata_merging(mocker: MockFixture):
    """Test that cache metadata is properly merged with API key metadata."""
    with tempfile.TemporaryDirectory() as tmpdir:
        with server_with_service(mocker) as (get_data_spy, data_service, port):
            config: SiftChannelConfig = {
                "uri": f"localhost:{port}",
                "apikey": "test-token",
                "use_ssl": False,
                "cache_config": {
                    "ttl": 3600,
                    "cache_path": str(Path(tmpdir) / "cache"),
                    "size_limit": 1024 * 1024,
                    "clear_on_init": True,
                },
            }
            cache = GrpcCache(config["cache_config"])

            async with use_sift_async_channel(config, cache=cache) as channel:
                stub = DataServiceStub(channel)
                request = GetDataRequest(page_size=100)

                # This should work - cache metadata should be merged with auth metadata
                res = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res.next_page_token == "token-1"
                assert data_service.call_count == 1

                # Verify cache works
                res2 = cast(GetDataResponse, await stub.GetData(request, metadata=with_cache()))
                assert res2.next_page_token == "token-1"
                assert data_service.call_count == 1
