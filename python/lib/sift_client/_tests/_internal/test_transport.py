"""Tests for URL normalization in GrpcConfig and RestConfig."""

from sift_client.transport.grpc_transport import GrpcConfig
from sift_client.transport.rest_transport import RestConfig


class TestGrpcConfigUrl:
    def test_adds_https_when_missing(self):
        config = GrpcConfig(url="grpc.sift.com", api_key="api")
        assert config.uri == "https://grpc.sift.com"

    def test_adds_http_when_missing_local(self):
        config = GrpcConfig(url="grpc.sift.com", api_key="api", use_ssl=False)
        assert config.uri == "http://grpc.sift.com"

    def test_url_keeps_https(self):
        config = GrpcConfig(url="https://grpc.sift.com", api_key="api")
        assert config.uri == "https://grpc.sift.com"

    def test_url_keeps_http(self):
        config = GrpcConfig(url="http://grpc.sift.com", api_key="api", use_ssl=False)
        assert config.uri == "http://grpc.sift.com"


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
