from abc import abstractmethod
from typing import Any, Callable, Iterator

import grpc

Continuation = Callable[[grpc.ClientCallDetails, Any], Any]


class ClientInterceptor(
    grpc.StreamStreamClientInterceptor,
    grpc.StreamUnaryClientInterceptor,
    grpc.UnaryStreamClientInterceptor,
    grpc.UnaryUnaryClientInterceptor,
):
    @abstractmethod
    def intercept(
        self,
        method: Continuation,
        request_or_iterator: Any,
        client_call_details: grpc.ClientCallDetails,
    ):
        pass

    def intercept_unary_unary(
        self,
        continuation: Continuation,
        client_call_details: grpc.ClientCallDetails,
        request: Any,
    ):
        return self.intercept(_swap_args(continuation), request, client_call_details)

    def intercept_stream_unary(
        self,
        continuation: Continuation,
        client_call_details: grpc.ClientCallDetails,
        request_iterator: Iterator[Any],
    ):
        return self.intercept(_swap_args(continuation), request_iterator, client_call_details)

    def intercept_unary_stream(
        self,
        continuation: Continuation,
        client_call_details: grpc.ClientCallDetails,
        request: Any,
    ):
        return self.intercept(_swap_args(continuation), request, client_call_details)

    def intercept_stream_stream(
        self,
        continuation: Continuation,
        client_call_details: grpc.ClientCallDetails,
        request_iterator: Iterator[Any],
    ):
        return self.intercept(_swap_args(continuation), request_iterator, client_call_details)


def _swap_args(fn: Callable[[Any, Any], Any]) -> Callable[[Any, Any], Any]:
    def new_fn(x, y):
        return fn(y, x)

    return new_fn
