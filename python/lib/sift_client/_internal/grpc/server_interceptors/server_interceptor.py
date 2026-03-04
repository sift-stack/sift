import abc
from typing import Any, Callable, Optional, Tuple, cast

import grpc


class ServerInterceptor(grpc.ServerInterceptor, metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def intercept(
        self,
        method: Callable,
        request_or_iterator: Any,
        context: grpc.ServicerContext,
        method_name: str,
    ) -> Any:
        return method(request_or_iterator, context)

    def intercept_service(self, continuation, handler_call_details):
        next_handler = continuation(handler_call_details)
        if next_handler is None:
            return

        handler_factory, next_handler_method = _get_factory_and_method(next_handler)

        def invoke_intercept_method(request_or_iterator, context):
            method_name = handler_call_details.method
            return self.intercept(
                next_handler_method,
                request_or_iterator,
                context,
                method_name,
            )

        return handler_factory(
            invoke_intercept_method,
            request_deserializer=next_handler.request_deserializer,
            response_serializer=next_handler.response_serializer,
        )


class _RpcHandler(grpc.RpcMethodHandler):
    unary_unary: Optional[Callable]
    unary_stream: Optional[Callable]
    stream_unary: Optional[Callable]
    stream_stream: Optional[Callable]


def _get_factory_and_method(
    rpc_handler: grpc.RpcMethodHandler,
) -> Tuple[Callable, Callable]:
    handler = cast(_RpcHandler, rpc_handler)

    if handler.unary_unary:
        return grpc.unary_unary_rpc_method_handler, handler.unary_unary
    elif handler.unary_stream:
        return grpc.unary_stream_rpc_method_handler, handler.unary_stream
    elif handler.stream_unary:
        return grpc.stream_unary_rpc_method_handler, handler.stream_unary
    elif handler.stream_stream:
        return grpc.stream_stream_rpc_method_handler, handler.stream_stream
    else:
        raise Exception("Unreachable")
