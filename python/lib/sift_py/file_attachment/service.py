from typing import Optional, cast
from sift.remote_files.v1.remote_files_pb2_grpc import RemoteFileServiceStub
from sift_py.file_attachment._internal.upload import UploadService
from sift_py.file_attachment.entity import Entity
from sift_py.file_attachment.metadata import Metadata
from sift_py.grpc.transport import SiftChannel
from sift.remote_files.v1.remote_files_pb2 import (
    RemoteFile,
    GetRemoteFileRequest,
    GetRemoteFileResponse,
)
from sift_py.rest import SiftRestConfig


class FileAttachmentService:
    _remote_file_service_stub: RemoteFileServiceStub
    _upload_service: UploadService

    def __init__(self, channel: SiftChannel, restconf: SiftRestConfig):
        self._remote_file_service_stub = RemoteFileServiceStub(channel)
        self._upload_service = UploadService(restconf)

    def upload_attachment(
        self,
        path: str,
        entity: Entity,
        metadata: Metadata,
        description: Optional[str] = None,
        organization_id: Optional[str] = None,
    ) -> RemoteFile:
        remote_file_id = self._upload_service.upload_attachment(
            path,
            entity,
            metadata,
            description,
            organization_id,
        )
        req = GetRemoteFileRequest(remote_file_id=remote_file_id)
        res = cast(GetRemoteFileResponse, self._remote_file_service_stub.GetRemoteFile(req))
        return res.remote_file
