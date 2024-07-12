from abc import abstractmethod
from typing import Any, AsyncIterable, Callable, Iterable, TypeVar, Union

from grpc import aio as grpc_aio

CallType = TypeVar("CallType", bound=grpc_aio.Call)
Continuation = Callable[[grpc_aio.ClientCallDetails, Any], CallType]


class ClientAsyncInterceptor(
    grpc_aio.UnaryUnaryClientInterceptor,
    grpc_aio.UnaryStreamClientInterceptor,
    grpc_aio.StreamUnaryClientInterceptor,
    grpc_aio.StreamStreamClientInterceptor,
):
    @abstractmethod
    async def intercept(
        self,
        method: Callable,
        request_or_iterator: Any,
        client_call_details: grpc_aio.ClientCallDetails,
    ) -> Any:
        pass

    async def intercept_unary_unary(
        self,
        continuation: Continuation[grpc_aio.UnaryUnaryCall],
        client_call_details: grpc_aio.ClientCallDetails,
        request: Any,
    ):
        return await self.intercept(_async_swap_args(continuation), request, client_call_details)

    async def intercept_unary_stream(
        self,
        continuation: Continuation[grpc_aio.UnaryStreamCall],
        client_call_details: grpc_aio.ClientCallDetails,
        request: Any,
    ):
        return await self.intercept(_async_swap_args(continuation), request, client_call_details)

    async def intercept_stream_unary(
        self,
        continuation: Continuation[grpc_aio.StreamUnaryCall],
        client_call_details: grpc_aio.ClientCallDetails,
        request_iterator: Union[Iterable[Any], AsyncIterable[Any]],
    ):
        return await self.intercept(
            _async_swap_args(continuation), request_iterator, client_call_details
        )

    async def intercept_stream_stream(
        self,
        continuation: Continuation[grpc_aio.StreamStreamCall],
        client_call_details: grpc_aio.ClientCallDetails,
        request_iterator: Union[Iterable[Any], AsyncIterable[Any]],
    ):
        return await self.intercept(
            _async_swap_args(continuation), request_iterator, client_call_details
        )


def _async_swap_args(fn: Callable[[Any, Any], Any]) -> Callable[[Any, Any], Any]:
    """
    Continuations are typed in such a way that details are the first argument, and the request second.
    Code generated from protobuf however takes in the request first, then the details. Weird grpc library
    quirk. This utility just flips the arguments.
    """

    async def new_fn(x, y):
        return await fn(y, x)

    return new_fn
