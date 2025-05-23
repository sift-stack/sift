"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.protobuf_descriptors.v2.protobuf_descriptors_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class ProtobufDescriptorServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    AddProtobufDescriptor: grpc.UnaryUnaryMultiCallable[
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.AddProtobufDescriptorRequest,
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.AddProtobufDescriptorResponse,
    ]
    """Used to register a protobuf message to be ingested."""

    CheckProtobufDescriptorCompatibility: grpc.UnaryUnaryMultiCallable[
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.CheckProtobufDescriptorCompatibilityRequest,
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.CheckProtobufDescriptorCompatibilityResponse,
    ]
    """Used to check if a protobuf descriptor is compatible with the existing descriptors."""

    DeleteProtobufDescriptors: grpc.UnaryUnaryMultiCallable[
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.DeleteProtobufDescriptorsRequest,
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.DeleteProtobufDescriptorsResponse,
    ]
    """Delete protobuf descriptors of that match the provided `namespace` and `message_type_full_name`."""

    ListProtobufDescriptors: grpc.UnaryUnaryMultiCallable[
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.ListProtobufDescriptorsRequest,
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.ListProtobufDescriptorsResponse,
    ]
    """Retrieve protobuf descriptors using an optional filter."""

class ProtobufDescriptorServiceAsyncStub:
    AddProtobufDescriptor: grpc.aio.UnaryUnaryMultiCallable[
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.AddProtobufDescriptorRequest,
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.AddProtobufDescriptorResponse,
    ]
    """Used to register a protobuf message to be ingested."""

    CheckProtobufDescriptorCompatibility: grpc.aio.UnaryUnaryMultiCallable[
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.CheckProtobufDescriptorCompatibilityRequest,
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.CheckProtobufDescriptorCompatibilityResponse,
    ]
    """Used to check if a protobuf descriptor is compatible with the existing descriptors."""

    DeleteProtobufDescriptors: grpc.aio.UnaryUnaryMultiCallable[
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.DeleteProtobufDescriptorsRequest,
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.DeleteProtobufDescriptorsResponse,
    ]
    """Delete protobuf descriptors of that match the provided `namespace` and `message_type_full_name`."""

    ListProtobufDescriptors: grpc.aio.UnaryUnaryMultiCallable[
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.ListProtobufDescriptorsRequest,
        sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.ListProtobufDescriptorsResponse,
    ]
    """Retrieve protobuf descriptors using an optional filter."""

class ProtobufDescriptorServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def AddProtobufDescriptor(
        self,
        request: sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.AddProtobufDescriptorRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.AddProtobufDescriptorResponse, collections.abc.Awaitable[sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.AddProtobufDescriptorResponse]]:
        """Used to register a protobuf message to be ingested."""

    @abc.abstractmethod
    def CheckProtobufDescriptorCompatibility(
        self,
        request: sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.CheckProtobufDescriptorCompatibilityRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.CheckProtobufDescriptorCompatibilityResponse, collections.abc.Awaitable[sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.CheckProtobufDescriptorCompatibilityResponse]]:
        """Used to check if a protobuf descriptor is compatible with the existing descriptors."""

    @abc.abstractmethod
    def DeleteProtobufDescriptors(
        self,
        request: sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.DeleteProtobufDescriptorsRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.DeleteProtobufDescriptorsResponse, collections.abc.Awaitable[sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.DeleteProtobufDescriptorsResponse]]:
        """Delete protobuf descriptors of that match the provided `namespace` and `message_type_full_name`."""

    @abc.abstractmethod
    def ListProtobufDescriptors(
        self,
        request: sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.ListProtobufDescriptorsRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.ListProtobufDescriptorsResponse, collections.abc.Awaitable[sift.protobuf_descriptors.v2.protobuf_descriptors_pb2.ListProtobufDescriptorsResponse]]:
        """Retrieve protobuf descriptors using an optional filter."""

def add_ProtobufDescriptorServiceServicer_to_server(servicer: ProtobufDescriptorServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...
