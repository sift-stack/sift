"""
Internal Module: This module contains implementation details and is not intended for external use.

This module is concerned with defining interceptors for unary and streaming RPCs. Any sub-class
of `ClientInterceptor` will be invoked for all types of RPCs: unary-unary, unary-stream, stream-unary,
and stream-stream. To create interceptors for particular kinds of RPCs you'll need to create a sub-class
for the particular types of interceptors found in the base `grpc` module.
"""

from __future__ import annotations

from typing import Any, Callable, List, Tuple, cast

import grpc
from grpc_interceptor import ClientCallDetails, ClientInterceptor

Metadata = List[Tuple[str, str]]


class MetadataInterceptor(ClientInterceptor):
    metadata: Metadata

    """
    Interceptor to add metadata to all unary and streaming RPCs
    """

    def __init__(self, metadata: Metadata):
        self.metadata = metadata

    def intercept(
        self,
        method: Callable,
        request_or_iterator: Any,
        call_details: grpc.ClientCallDetails,
    ):
        call_details = cast(ClientCallDetails, call_details)
        new_details = ClientCallDetails(
            call_details.method,
            call_details.timeout,
            self.metadata,
            call_details.credentials,
            call_details.wait_for_ready,
            call_details.compression,
        )

        return method(request_or_iterator, new_details)
