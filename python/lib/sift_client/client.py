from __future__ import annotations

from sift_client.errors import _sift_client_experimental_warning
from sift_client.resources import (
    AssetsAPI,
    AssetsAPIAsync,
    CalculatedChannelsAPI,
    CalculatedChannelsAPIAsync,
    PingAPI,
    PingAPIAsync,
    RulesAPI,
    RulesAPIAsync,
)
from sift_client.transport import (
    GrpcClient,
    GrpcConfig,
    RestClient,
    RestConfig,
    SiftConnectionConfig,
    WithGrpcClient,
    WithRestClient,
)

_sift_client_experimental_warning()


class SiftClient(
    WithGrpcClient,
    WithRestClient,
):
    def __init__(
        self,
        api_key: str | None = None,
        grpc_url: str | None = None,
        rest_url: str | None = None,
        connection_config: SiftConnectionConfig | None = None,
    ):
        if not (api_key and grpc_url and rest_url) and not connection_config:
            raise ValueError(
                "Either api_key, grpc_uri and rest_uri or connection_config must be provided to establish a connection."
            )

        if connection_config:
            grpc_client = GrpcClient(connection_config.get_grpc_config())
            rest_client = RestClient(connection_config.get_rest_config())
        elif api_key and grpc_url and rest_url:
            grpc_client = GrpcClient(GrpcConfig(grpc_url, api_key))
            rest_client = RestClient(RestConfig(rest_url, api_key))
        else:
            raise ValueError(
                "Invalid connection configuration. Please provide api_key, grpc_uri and rest_uri or a connection_config."
            )

        WithGrpcClient.__init__(self, grpc_client=grpc_client)
        WithRestClient.__init__(self, rest_client=rest_client)

        self.ping = PingAPI(self)
        self.ping_async = PingAPIAsync(self)
        self.assets = AssetsAPI(self)
        self.assets_async = AssetsAPIAsync(self)
        self.calculated_channels = CalculatedChannelsAPI(self)
        self.calculated_channels_async = CalculatedChannelsAPIAsync(self)
        self.rules = RulesAPI(self)
        self.rules_async = RulesAPIAsync(self)

    @property
    def grpc_client(self) -> GrpcClient:
        return self._grpc_client

    @property
    def rest_client(self) -> RestClient:
        return self._rest_client
