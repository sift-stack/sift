"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.notifications.v1.notifications_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class NotificationServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    ListNotifications: grpc.UnaryUnaryMultiCallable[
        sift.notifications.v1.notifications_pb2.ListNotificationsRequest,
        sift.notifications.v1.notifications_pb2.ListNotificationsResponse,
    ]
    """Retrieves notifications using an optional filter."""

    BatchUpdateNotifications: grpc.UnaryUnaryMultiCallable[
        sift.notifications.v1.notifications_pb2.BatchUpdateNotificationsRequest,
        sift.notifications.v1.notifications_pb2.BatchUpdateNotificationsResponse,
    ]
    """Batch updates a list of notifications using the list of fields specified in their respective `update_mask`s."""

class NotificationServiceAsyncStub:
    ListNotifications: grpc.aio.UnaryUnaryMultiCallable[
        sift.notifications.v1.notifications_pb2.ListNotificationsRequest,
        sift.notifications.v1.notifications_pb2.ListNotificationsResponse,
    ]
    """Retrieves notifications using an optional filter."""

    BatchUpdateNotifications: grpc.aio.UnaryUnaryMultiCallable[
        sift.notifications.v1.notifications_pb2.BatchUpdateNotificationsRequest,
        sift.notifications.v1.notifications_pb2.BatchUpdateNotificationsResponse,
    ]
    """Batch updates a list of notifications using the list of fields specified in their respective `update_mask`s."""

class NotificationServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def ListNotifications(
        self,
        request: sift.notifications.v1.notifications_pb2.ListNotificationsRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.notifications.v1.notifications_pb2.ListNotificationsResponse, collections.abc.Awaitable[sift.notifications.v1.notifications_pb2.ListNotificationsResponse]]:
        """Retrieves notifications using an optional filter."""

    @abc.abstractmethod
    def BatchUpdateNotifications(
        self,
        request: sift.notifications.v1.notifications_pb2.BatchUpdateNotificationsRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.notifications.v1.notifications_pb2.BatchUpdateNotificationsResponse, collections.abc.Awaitable[sift.notifications.v1.notifications_pb2.BatchUpdateNotificationsResponse]]:
        """Batch updates a list of notifications using the list of fields specified in their respective `update_mask`s."""

def add_NotificationServiceServicer_to_server(servicer: NotificationServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...