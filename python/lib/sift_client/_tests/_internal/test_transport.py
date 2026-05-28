"""Tests for URL normalization in GrpcConfig and RestConfig."""

import threading

import pytest

from sift_client.transport.grpc_transport import GrpcClient, GrpcConfig
from sift_client.transport.rest_transport import RestConfig


class TestGrpcConfigUrl:
    def test_adds_https_when_missing(self):
        config = GrpcConfig(url="grpc.sift.com", api_key="api")
        assert config.uri == "https://grpc.sift.com"

    def test_adds_https_on_localhost(self):
        config = GrpcConfig(url="localhost:50051", api_key="api", use_ssl=False)
        assert config.uri == "http://localhost:50051"

    def test_adds_https_on_ip(self):
        conifg = GrpcConfig(url="129.10.1.1", api_key="api")
        assert conifg.uri == "https://129.10.1.1"

    def test_adds_https_on_ipv6(self):
        config = GrpcConfig(url="[::]:8080", api_key="api")
        assert config.uri == "https://[::]:8080"

    def test_adds_http_when_missing_local(self):
        config = GrpcConfig(url="grpc.sift.com", api_key="api", use_ssl=False)
        assert config.uri == "http://grpc.sift.com"

    def test_url_keeps_https(self):
        config = GrpcConfig(url="https://grpc.sift.com", api_key="api")
        assert config.uri == "https://grpc.sift.com"

    def test_url_keeps_http(self):
        config = GrpcConfig(url="http://grpc.sift.com", api_key="api", use_ssl=False)
        assert config.uri == "http://grpc.sift.com"

    def test_raises_on_invalid_url(self):
        with pytest.raises(ValueError, match="Invalid connection URL"):
            GrpcConfig(url="htp://localhost:8080", api_key="api")

    def test_raise_on_invalid_url2(self):
        with pytest.raises(ValueError, match="Invalid connection URL"):
            GrpcConfig(url="https:/localhost:50051", api_key="api")

    def test_raise_on_missing_url(self):
        with pytest.raises(ValueError, match="Invalid connection URL"):
            GrpcConfig(url="", api_key="api")


class TestGrpcClientClose:
    """Lifecycle of GrpcClient.close_sync().

    Constructing a GrpcClient builds a real (lazy, undialed) channel against an
    unresolvable host and spins up the background event-loop thread; no RPC is
    made, so these run offline.
    """

    @staticmethod
    def _client() -> GrpcClient:
        return GrpcClient(GrpcConfig(url="disabled.invalid:0", api_key="api", use_ssl=False))

    def test_close_sync_releases_channels(self):
        # The channel maps must be cleared so the gRPC C-core can destroy the
        # channels before its own exit-time shutdown, avoiding the
        # "grpc_wait_for_shutdown_with_timeout() timed out" message.
        client = self._client()
        assert client._channels_async  # channel created on the default loop
        client.close_sync()
        assert client._closed is True
        assert client._channels_async == {}
        assert client._stubs_async_map == {}
        assert not client._default_loop_thread.is_alive()

    def test_close_sync_is_idempotent(self):
        # The atexit handler always fires after an explicit close (or the
        # context manager's __exit__). The second call must be a no-op, not hang
        # submitting a coroutine to the already-stopped loop.
        client = self._client()
        client.close_sync()

        finished = threading.Event()

        def _second_close():
            client.close_sync()
            finished.set()

        thread = threading.Thread(target=_second_close)
        thread.start()
        thread.join(timeout=5.0)
        assert finished.is_set(), "second close_sync() hung on the stopped loop"


class TestRestConfigUrl:
    def test_adds_https_when_missing(self):
        config = RestConfig(base_url="rest.sift.com", api_key="api")
        assert config.base_url == "https://rest.sift.com"

    def test_add_http_when_missing_local(self):
        config = RestConfig(base_url="rest.sift.com", api_key="api", use_ssl=False)
        assert config.base_url == "http://rest.sift.com"

    def test_url_keeps_https(self):
        config = RestConfig(base_url="https://rest.sift.com", api_key="api")
        assert config.base_url == "https://rest.sift.com"

    def test_url_keeps_http(self):
        config = RestConfig(base_url="http://rest.sift.com", api_key="api", use_ssl=False)
        assert config.base_url == "http://rest.sift.com"
