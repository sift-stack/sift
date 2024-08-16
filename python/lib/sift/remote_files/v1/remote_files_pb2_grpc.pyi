"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import abc
import collections.abc
import grpc
import grpc.aio
import sift.remote_files.v1.remote_files_pb2
import typing

_T = typing.TypeVar("_T")

class _MaybeAsyncIterator(collections.abc.AsyncIterator[_T], collections.abc.Iterator[_T], metaclass=abc.ABCMeta): ...

class _ServicerContext(grpc.ServicerContext, grpc.aio.ServicerContext):  # type: ignore[misc, type-arg]
    ...

class RemoteFileServiceStub:
    def __init__(self, channel: typing.Union[grpc.Channel, grpc.aio.Channel]) -> None: ...
    GetRemoteFile: grpc.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.GetRemoteFileRequest,
        sift.remote_files.v1.remote_files_pb2.GetRemoteFileResponse,
    ]
    """Retrieve a remote file."""

    CreateRemoteFile: grpc.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.CreateRemoteFileRequest,
        sift.remote_files.v1.remote_files_pb2.CreateRemoteFileResponse,
    ]
    """Create a remote file."""

    ListRemoteFiles: grpc.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.ListRemoteFilesRequest,
        sift.remote_files.v1.remote_files_pb2.ListRemoteFilesResponse,
    ]
    """List remote files."""

    UpdateRemoteFile: grpc.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.UpdateRemoteFileRequest,
        sift.remote_files.v1.remote_files_pb2.UpdateRemoteFileResponse,
    ]
    """Updates an existing remote file using using the list of fields specified in `update_mask`."""

    DeleteRemoteFile: grpc.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.DeleteRemoteFileRequest,
        sift.remote_files.v1.remote_files_pb2.DeleteRemoteFileResponse,
    ]
    """Delete a remote file."""

    BatchDeleteRemoteFiles: grpc.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.BatchDeleteRemoteFilesRequest,
        sift.remote_files.v1.remote_files_pb2.BatchDeleteRemoteFilesResponse,
    ]
    """Batch deletes remote files. Each batch is limited to 1000 records."""

    GetRemoteFileDownloadUrl: grpc.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.GetRemoteFileDownloadUrlRequest,
        sift.remote_files.v1.remote_files_pb2.GetRemoteFileDownloadUrlResponse,
    ]
    """Gets a download URL for the remote file."""

class RemoteFileServiceAsyncStub:
    GetRemoteFile: grpc.aio.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.GetRemoteFileRequest,
        sift.remote_files.v1.remote_files_pb2.GetRemoteFileResponse,
    ]
    """Retrieve a remote file."""

    CreateRemoteFile: grpc.aio.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.CreateRemoteFileRequest,
        sift.remote_files.v1.remote_files_pb2.CreateRemoteFileResponse,
    ]
    """Create a remote file."""

    ListRemoteFiles: grpc.aio.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.ListRemoteFilesRequest,
        sift.remote_files.v1.remote_files_pb2.ListRemoteFilesResponse,
    ]
    """List remote files."""

    UpdateRemoteFile: grpc.aio.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.UpdateRemoteFileRequest,
        sift.remote_files.v1.remote_files_pb2.UpdateRemoteFileResponse,
    ]
    """Updates an existing remote file using using the list of fields specified in `update_mask`."""

    DeleteRemoteFile: grpc.aio.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.DeleteRemoteFileRequest,
        sift.remote_files.v1.remote_files_pb2.DeleteRemoteFileResponse,
    ]
    """Delete a remote file."""

    BatchDeleteRemoteFiles: grpc.aio.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.BatchDeleteRemoteFilesRequest,
        sift.remote_files.v1.remote_files_pb2.BatchDeleteRemoteFilesResponse,
    ]
    """Batch deletes remote files. Each batch is limited to 1000 records."""

    GetRemoteFileDownloadUrl: grpc.aio.UnaryUnaryMultiCallable[
        sift.remote_files.v1.remote_files_pb2.GetRemoteFileDownloadUrlRequest,
        sift.remote_files.v1.remote_files_pb2.GetRemoteFileDownloadUrlResponse,
    ]
    """Gets a download URL for the remote file."""

class RemoteFileServiceServicer(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def GetRemoteFile(
        self,
        request: sift.remote_files.v1.remote_files_pb2.GetRemoteFileRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.remote_files.v1.remote_files_pb2.GetRemoteFileResponse, collections.abc.Awaitable[sift.remote_files.v1.remote_files_pb2.GetRemoteFileResponse]]:
        """Retrieve a remote file."""

    @abc.abstractmethod
    def CreateRemoteFile(
        self,
        request: sift.remote_files.v1.remote_files_pb2.CreateRemoteFileRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.remote_files.v1.remote_files_pb2.CreateRemoteFileResponse, collections.abc.Awaitable[sift.remote_files.v1.remote_files_pb2.CreateRemoteFileResponse]]:
        """Create a remote file."""

    @abc.abstractmethod
    def ListRemoteFiles(
        self,
        request: sift.remote_files.v1.remote_files_pb2.ListRemoteFilesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.remote_files.v1.remote_files_pb2.ListRemoteFilesResponse, collections.abc.Awaitable[sift.remote_files.v1.remote_files_pb2.ListRemoteFilesResponse]]:
        """List remote files."""

    @abc.abstractmethod
    def UpdateRemoteFile(
        self,
        request: sift.remote_files.v1.remote_files_pb2.UpdateRemoteFileRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.remote_files.v1.remote_files_pb2.UpdateRemoteFileResponse, collections.abc.Awaitable[sift.remote_files.v1.remote_files_pb2.UpdateRemoteFileResponse]]:
        """Updates an existing remote file using using the list of fields specified in `update_mask`."""

    @abc.abstractmethod
    def DeleteRemoteFile(
        self,
        request: sift.remote_files.v1.remote_files_pb2.DeleteRemoteFileRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.remote_files.v1.remote_files_pb2.DeleteRemoteFileResponse, collections.abc.Awaitable[sift.remote_files.v1.remote_files_pb2.DeleteRemoteFileResponse]]:
        """Delete a remote file."""

    @abc.abstractmethod
    def BatchDeleteRemoteFiles(
        self,
        request: sift.remote_files.v1.remote_files_pb2.BatchDeleteRemoteFilesRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.remote_files.v1.remote_files_pb2.BatchDeleteRemoteFilesResponse, collections.abc.Awaitable[sift.remote_files.v1.remote_files_pb2.BatchDeleteRemoteFilesResponse]]:
        """Batch deletes remote files. Each batch is limited to 1000 records."""

    @abc.abstractmethod
    def GetRemoteFileDownloadUrl(
        self,
        request: sift.remote_files.v1.remote_files_pb2.GetRemoteFileDownloadUrlRequest,
        context: _ServicerContext,
    ) -> typing.Union[sift.remote_files.v1.remote_files_pb2.GetRemoteFileDownloadUrlResponse, collections.abc.Awaitable[sift.remote_files.v1.remote_files_pb2.GetRemoteFileDownloadUrlResponse]]:
        """Gets a download URL for the remote file."""

def add_RemoteFileServiceServicer_to_server(servicer: RemoteFileServiceServicer, server: typing.Union[grpc.Server, grpc.aio.Server]) -> None: ...
