from __future__ import annotations

from typing import Any, Callable, List, Tuple, cast

from grpc import aio as grpc_aio

from sift_py.grpc._async_interceptors.base import ClientAsyncInterceptor

Metadata = List[Tuple[str, str]]


class MetadataAsyncInterceptor(ClientAsyncInterceptor):
    metadata: Metadata

    """
    Interceptor to add metadata to all async unary and streaming RPCs
    """

    def __init__(self, metadata: Metadata):
        self.metadata = metadata

    async def intercept(
        self,
        method: Callable,
        request_or_iterator: Any,
        client_call_details: grpc_aio.ClientCallDetails,
    ):
        call_details = cast(grpc_aio.ClientCallDetails, client_call_details)
        new_details = grpc_aio.ClientCallDetails(
            call_details.method,
            call_details.timeout,
            self.metadata,
            call_details.credentials,
            call_details.wait_for_ready,
        )
        return await method(request_or_iterator, new_details)
