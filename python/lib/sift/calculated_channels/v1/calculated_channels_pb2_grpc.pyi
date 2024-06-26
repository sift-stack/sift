"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.calculated_channels.v1.calculated_channels_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class CalculatedChannelsServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    ListExpressionIdentifiers: grpc.UnaryUnaryMultiCallable[
        sift.calculated_channels.v1.calculated_channels_pb2.ListExpressionIdentifiersRequest,
        sift.calculated_channels.v1.calculated_channels_pb2.ListExpressionIdentifiersResponse,
    ]
    """Retrieves a list of valid identifiers that can be used as part of a calculated channel expression."""

    ValidateExpression: grpc.UnaryUnaryMultiCallable[
        sift.calculated_channels.v1.calculated_channels_pb2.ValidateExpressionRequest,
        sift.calculated_channels.v1.calculated_channels_pb2.ValidateExpressionResponse,
    ]
    """Used to validate whether or not an expression used for a calculated channel is valid."""

class CalculatedChannelsServiceAsyncStub:
    ListExpressionIdentifiers: grpc.aio.UnaryUnaryMultiCallable[
        sift.calculated_channels.v1.calculated_channels_pb2.ListExpressionIdentifiersRequest,
        sift.calculated_channels.v1.calculated_channels_pb2.ListExpressionIdentifiersResponse,
    ]
    """Retrieves a list of valid identifiers that can be used as part of a calculated channel expression."""

    ValidateExpression: grpc.aio.UnaryUnaryMultiCallable[
        sift.calculated_channels.v1.calculated_channels_pb2.ValidateExpressionRequest,
        sift.calculated_channels.v1.calculated_channels_pb2.ValidateExpressionResponse,
    ]
    """Used to validate whether or not an expression used for a calculated channel is valid."""

class CalculatedChannelsServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def ListExpressionIdentifiers(
        self,
        request: sift.calculated_channels.v1.calculated_channels_pb2.ListExpressionIdentifiersRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.calculated_channels.v1.calculated_channels_pb2.ListExpressionIdentifiersResponse, collections.abc.Awaitable[sift.calculated_channels.v1.calculated_channels_pb2.ListExpressionIdentifiersResponse]]:
        """Retrieves a list of valid identifiers that can be used as part of a calculated channel expression."""

    @abc.abstractmethod
    def ValidateExpression(
        self,
        request: sift.calculated_channels.v1.calculated_channels_pb2.ValidateExpressionRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.calculated_channels.v1.calculated_channels_pb2.ValidateExpressionResponse, collections.abc.Awaitable[sift.calculated_channels.v1.calculated_channels_pb2.ValidateExpressionResponse]]:
        """Used to validate whether or not an expression used for a calculated channel is valid."""

def add_CalculatedChannelsServiceServicer_to_server(servicer: CalculatedChannelsServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...
