"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.channels.v3.channels_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class ChannelServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    GetChannel: grpc.UnaryUnaryMultiCallable[
        sift.channels.v3.channels_pb2.GetChannelRequest,
        sift.channels.v3.channels_pb2.GetChannelResponse,
    ]
    """Retrieve a channel"""

    ListChannels: grpc.UnaryUnaryMultiCallable[
        sift.channels.v3.channels_pb2.ListChannelsRequest,
        sift.channels.v3.channels_pb2.ListChannelsResponse,
    ]
    """Retrieve channels using an optional filter."""

class ChannelServiceAsyncStub:
    GetChannel: grpc.aio.UnaryUnaryMultiCallable[
        sift.channels.v3.channels_pb2.GetChannelRequest,
        sift.channels.v3.channels_pb2.GetChannelResponse,
    ]
    """Retrieve a channel"""

    ListChannels: grpc.aio.UnaryUnaryMultiCallable[
        sift.channels.v3.channels_pb2.ListChannelsRequest,
        sift.channels.v3.channels_pb2.ListChannelsResponse,
    ]
    """Retrieve channels using an optional filter."""

class ChannelServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def GetChannel(
        self,
        request: sift.channels.v3.channels_pb2.GetChannelRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.channels.v3.channels_pb2.GetChannelResponse, collections.abc.Awaitable[sift.channels.v3.channels_pb2.GetChannelResponse]]:
        """Retrieve a channel"""

    @abc.abstractmethod
    def ListChannels(
        self,
        request: sift.channels.v3.channels_pb2.ListChannelsRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.channels.v3.channels_pb2.ListChannelsResponse, collections.abc.Awaitable[sift.channels.v3.channels_pb2.ListChannelsResponse]]:
        """Retrieve channels using an optional filter."""

def add_ChannelServiceServicer_to_server(servicer: ChannelServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...
