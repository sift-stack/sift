from __future__ import annotations

from sift_client.transport import GrpcClient, RestClient


class APIWithGrpcClient:
    def __init__(self, grpc_client: GrpcClient):
        self._client = grpc_client


class APIWithRestClient:
    def __init__(self, rest_client: RestClient):
        self._rest_client = rest_client
