"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.ping.v1.ping_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class PingServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    Ping: grpc.UnaryUnaryMultiCallable[
        sift.ping.v1.ping_pb2.PingRequest,
        sift.ping.v1.ping_pb2.PingResponse,
    ]

class PingServiceAsyncStub:
    Ping: grpc.aio.UnaryUnaryMultiCallable[
        sift.ping.v1.ping_pb2.PingRequest,
        sift.ping.v1.ping_pb2.PingResponse,
    ]

class PingServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def Ping(
        self,
        request: sift.ping.v1.ping_pb2.PingRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.ping.v1.ping_pb2.PingResponse, collections.abc.Awaitable[sift.ping.v1.ping_pb2.PingResponse]]: ...

def add_PingServiceServicer_to_server(servicer: PingServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...
