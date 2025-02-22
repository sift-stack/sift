"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.tags.v2.tags_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class TagServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    CreateTag: grpc.UnaryUnaryMultiCallable[
        sift.tags.v2.tags_pb2.CreateTagRequest,
        sift.tags.v2.tags_pb2.CreateTagResponse,
    ]
    """Create a tag."""

    ListTags: grpc.UnaryUnaryMultiCallable[
        sift.tags.v2.tags_pb2.ListTagsRequest,
        sift.tags.v2.tags_pb2.ListTagsResponse,
    ]
    """Retrieves tags using an optional filter."""

class TagServiceAsyncStub:
    CreateTag: grpc.aio.UnaryUnaryMultiCallable[
        sift.tags.v2.tags_pb2.CreateTagRequest,
        sift.tags.v2.tags_pb2.CreateTagResponse,
    ]
    """Create a tag."""

    ListTags: grpc.aio.UnaryUnaryMultiCallable[
        sift.tags.v2.tags_pb2.ListTagsRequest,
        sift.tags.v2.tags_pb2.ListTagsResponse,
    ]
    """Retrieves tags using an optional filter."""

class TagServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def CreateTag(
        self,
        request: sift.tags.v2.tags_pb2.CreateTagRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.tags.v2.tags_pb2.CreateTagResponse, collections.abc.Awaitable[sift.tags.v2.tags_pb2.CreateTagResponse]]:
        """Create a tag."""

    @abc.abstractmethod
    def ListTags(
        self,
        request: sift.tags.v2.tags_pb2.ListTagsRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.tags.v2.tags_pb2.ListTagsResponse, collections.abc.Awaitable[sift.tags.v2.tags_pb2.ListTagsResponse]]:
        """Retrieves tags using an optional filter."""

def add_TagServiceServicer_to_server(servicer: TagServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...
