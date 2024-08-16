from pathlib import Path
from typing import List, Optional, Union, cast

from sift.remote_files.v1.remote_files_pb2 import (
    BatchDeleteRemoteFilesRequest,
    GetRemoteFileDownloadUrlRequest,
    GetRemoteFileDownloadUrlResponse,
    GetRemoteFileRequest,
    GetRemoteFileResponse,
    ListRemoteFilesRequest,
    ListRemoteFilesResponse,
    RemoteFile,
)
from sift.remote_files.v1.remote_files_pb2_grpc import RemoteFileServiceStub

from sift_py.file_attachment._internal.download import download_remote_file
from sift_py.file_attachment._internal.upload import UploadService
from sift_py.file_attachment.entity import Entity
from sift_py.file_attachment.metadata import Metadata
from sift_py.grpc.transport import SiftChannel
from sift_py.rest import SiftRestConfig


class FileAttachmentService:
    """
    Service used to retrieve, upload, download, and delete file attachments. Seee `sift_py.file_attachment`
    for more information and examples on how to use this service.
    """

    _remote_file_service_stub: RemoteFileServiceStub
    _upload_service: UploadService

    def __init__(self, channel: SiftChannel, restconf: SiftRestConfig):
        self._remote_file_service_stub = RemoteFileServiceStub(channel)
        self._upload_service = UploadService(restconf)

    def retrieve_attachments(self, entity: Entity) -> List[RemoteFile]:
        """
        Retrieves all file attachments for the provided `entity`.
        """

        filter = f'entity_id=="{entity.entity_id}" && entity_type=="{entity.entity_type.value}"'
        page_size = 1_000
        next_page_token = ""

        remote_files: List[RemoteFile] = []

        while True:
            req = ListRemoteFilesRequest(
                filter=filter,
                page_size=page_size,
                page_token=next_page_token,
            )
            res = cast(ListRemoteFilesResponse, self._remote_file_service_stub.ListRemoteFiles(req))
            remote_files.extend(res.remote_files)
            next_page_token = res.next_page_token

            if not next_page_token:
                break

        return remote_files

    def upload_attachment(
        self,
        path: Union[str, Path],
        entity: Entity,
        metadata: Optional[Metadata],
        description: Optional[str] = None,
        organization_id: Optional[str] = None,
    ) -> RemoteFile:
        """
        Uploads a file pointed to by `path` and attaches it to the provided `entity`.

        - `path`: A path to the file to upload to Sift as a file attachment.
        - `entity`: The entity to attach the file to.
        - `metadata`: Optional metadata to include with the specific file.
        - `description`: An optional description to provide for the file attachment.
        - `organization_id`: Only required if your user belongs to multiple organizations.
        """
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

    def download_attachment(
        self,
        file: Union[RemoteFile, str],
        out: Optional[Union[str, Path]] = None,
    ) -> Path:
        """
        Downloads a file attachment and saves it locally.

        - `remote_file`: Could either be an instance of `RemoteFile` or the ID of the remote file to download.
        - `out`: If unspecified, then the file will be downloaded to the current working directory with the original name.
        """

        if isinstance(file, RemoteFile):
            remote_file = file
        else:
            req = GetRemoteFileRequest(remote_file_id=file)
            res = cast(GetRemoteFileResponse, self._remote_file_service_stub.GetRemoteFile(req))
            remote_file = res.remote_file

        output_file_path = (
            Path(out) if isinstance(out, str) else Path(remote_file.file_name).resolve()
        )

        download_url_req = GetRemoteFileDownloadUrlRequest(
            remote_file_id=remote_file.remote_file_id
        )
        download_url_res = cast(
            GetRemoteFileDownloadUrlResponse,
            self._remote_file_service_stub.GetRemoteFileDownloadUrl(download_url_req),
        )
        url = download_url_res.download_url

        download_remote_file(url, output_file_path)

        return output_file_path

    def delete_file_attachments(self, *to_delete: Union[str, RemoteFile]):
        """
        Deletes remote files given a set of arguments that could either be instances of `RemoteFile` or the ID
        of remote files to delete
        """
        remote_file_ids = [
            remote_file.remote_file_id if isinstance(remote_file, RemoteFile) else remote_file
            for remote_file in to_delete
        ]

        batch_size = 1_000
        for i in range(0, len(remote_file_ids), batch_size):
            batch = remote_file_ids[i : i + batch_size]
            self._remote_file_service_stub.BatchDeleteRemoteFiles(
                BatchDeleteRemoteFilesRequest(remote_file_ids=batch)
            )
