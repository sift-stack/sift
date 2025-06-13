from __future__ import annotations

from sift_client.resources import PingAPI, PingAPIAsync
from sift_client.transport import (
    GrpcClient,
    GrpcConfig,
    RestClient,
    RestConfig,
    SiftConnectionConfig,
)


class SiftClient:
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
            self._grpc_client = GrpcClient(connection_config.get_grpc_config())
            self._rest_client = RestClient(connection_config.get_rest_config())
        else:
            self._grpc_client = GrpcClient(GrpcConfig(grpc_url, api_key))
            self._rest_client = RestClient(RestConfig(rest_url, api_key))

        self.ping = PingAPI(self._grpc_client)
        self.ping_async = PingAPIAsync(self._grpc_client)
        # self.assets = SyncAssetsAPI(self._grpc_client)
        self.assets_async = AssetsAPI(self._grpc_client)
