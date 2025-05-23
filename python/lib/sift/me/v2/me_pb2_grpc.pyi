"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.me.v2.me_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class MeServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    GetMe: grpc.UnaryUnaryMultiCallable[
        sift.me.v2.me_pb2.GetMeRequest,
        sift.me.v2.me_pb2.GetMeResponse,
    ]

class MeServiceAsyncStub:
    GetMe: grpc.aio.UnaryUnaryMultiCallable[
        sift.me.v2.me_pb2.GetMeRequest,
        sift.me.v2.me_pb2.GetMeResponse,
    ]

class MeServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def GetMe(
        self,
        request: sift.me.v2.me_pb2.GetMeRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.me.v2.me_pb2.GetMeResponse, collections.abc.Awaitable[sift.me.v2.me_pb2.GetMeResponse]]: ...

def add_MeServiceServicer_to_server(servicer: MeServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...
