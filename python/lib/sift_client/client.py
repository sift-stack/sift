from __future__ import annotations

from sift_client.resources import PingAPI, PingAPIAsync, AssetsAPIAsync, AssetsAPI
from sift_client.transport import (
    GrpcClient,
    GrpcConfig,
    RestClient,
    RestConfig,
    SiftConnectionConfig,
    WithGrpcClient,
    WithRestClient,
)


class SiftClient(
    WithGrpcClient,
    WithRestClient,
):
    def __init__(
        self,
        api_key: str = None,
        grpc_url: str = None,
        rest_url: str = None,
        connection_config: SiftConnectionConfig = None,
    ):
        if not (api_key and grpc_url and rest_url) and not connection_config:
            raise ValueError(
                "Either api_key, grpc_uri and rest_uri or connection_config must be provided to establish a connection."
            )

        if connection_config:
            grpc_client = GrpcClient(connection_config.get_grpc_config())
            rest_client = RestClient(connection_config.get_rest_config())
        else:
            grpc_client =  GrpcClient(GrpcConfig(grpc_url, api_key))
            rest_client = RestClient(RestConfig(rest_url, api_key))

        WithGrpcClient.__init__(self, grpc_client=grpc_client)
        WithRestClient.__init__(self, rest_client=rest_client)

        self.ping = PingAPI(self)
        self.ping_async = PingAPIAsync(self)
        self.assets = AssetsAPI(self)
        self.assets_async = AssetsAPIAsync(self)

    @property
    def grpc_client(self) -> GrpcClient:
        return self._grpc_client

    @property
    def rest_client(self) -> RestClient:
        return self._rest_client
