from typing import Any, List, Tuple, cast

import grpc

from sift_py.grpc._interceptors.base import ClientInterceptor, Continuation
from sift_py.grpc._interceptors.context import ClientCallDetails

Metadata = List[Tuple[str, str]]


class MetadataInterceptor(ClientInterceptor):
    metadata: Metadata

    def __init__(self, metadata: Metadata):
        self.metadata = metadata

    def intercept(
        self,
        method: Continuation,
        request_or_iterator: Any,
        client_call_details: grpc.ClientCallDetails,
    ):
        details = cast(ClientCallDetails, client_call_details)

        new_details = ClientCallDetails(
            method=details.method,
            timeout=details.timeout,
            credentials=details.credentials,
            wait_for_ready=details.wait_for_ready,
            metadata=self.metadata,
        )

        return method(request_or_iterator, new_details)
