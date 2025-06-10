from __future__ import annotations

from abc import ABC
from typing import Any, Callable

from sift_client.transport import GrpcClient, RestClient


class WithGrpcClient(ABC):
    _grpc_client: GrpcClient


class WithRestClient(ABC):
    _rest_client: RestClient


class LowLevelClientBase:
    _grpc_client: GrpcClient
    _rest_client: RestClient

    @staticmethod
    async def _handle_pagination(
        func: Callable,
        page_size: int = None,
        page_token: str = None,
        query_filter: str = None,
        order_by: str = None,
        max_results: int = None,
    ) -> Any:
        results = []
        if page_token is None:
            page_token = ""
        while True:
            if max_results is not None and len(results) >= max_results:
                break
            response, page_token = await func(
                page_size=page_size,
                page_token=page_token,
                query_filter=query_filter,
                order_by=order_by,
            )
            results.extend(response)
            if page_token == "":
                break
        return results
