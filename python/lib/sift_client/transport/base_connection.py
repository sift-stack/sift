from __future__ import annotations

from sift_client.transport.grpc_transport import GrpcConfig
from sift_client.transport.rest_transport import RestConfig


class SiftConnectionConfig:
    def __init__(
        self,
        grpc_url: str,
        rest_url: str,
        api_key: str,
        use_ssl: bool = False,
        cert_via_openssl: bool = False,
    ):
        self.api_key = api_key
        self.grpc_url = grpc_url
        self.rest_url = rest_url
        self.use_ssl = use_ssl
        self.cert_via_openssl = cert_via_openssl

    def get_grpc_config(self):
        return GrpcConfig(
            url=self.grpc_url,
            api_key=self.api_key,
            use_ssl=self.use_ssl,
            cert_via_openssl=self.cert_via_openssl,
        )

    def get_rest_config(self):
        return RestConfig(
            base_url=self.rest_url,
            api_key=self.api_key,
            use_ssl=self.use_ssl,
            cert_via_openssl=self.cert_via_openssl,
        )
