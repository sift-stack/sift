from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.remote_files import RemoteFilesLowLevelClient
from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset
    from sift_client.sift_types.file_attachment import (
        FileAttachment,
        FileAttachmentUpdate,
        RemoteFileEntityType,
    )
    from sift_client.sift_types.run import Run
    from sift_client.sift_types.test_report import TestReport


class FileAttachmentsAPIAsync(ResourceBase):
    """High-level API for interacting with file attachments (remote files).

    This class provides a Pythonic interface for managing file attachments
    on Sift entities like runs, assets, and test reports.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the FileAttachmentsAPIAsync.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = RemoteFilesLowLevelClient(grpc_client=self.client.grpc_client)
        self._upload_client = UploadLowLevelClient(rest_client=self.client.rest_client)
        self.greeting = "Hello, World!"

    async def get(self, *, file_attachment_id: str) -> FileAttachment:
        """Get a file attachment by ID.

        Args:
            file_attachment_id: The ID of the file attachment to retrieve.

        Returns:
            The FileAttachment.
        """
        file_attachment = await self._low_level_client.get_remote_file(
            remote_file_id=file_attachment_id,
            sift_client=self.client,
        )
        return self._apply_client_to_instance(file_attachment)

    async def list_(
        self,
        *,
        entity: Run | Asset | TestReport | None = None,
        remote_file_id: str | None = None,
        file_name: str | None = None,
        entity_type: RemoteFileEntityType | None = None,
        entity_id: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[FileAttachment]:
        """List file attachments with optional filtering.

        Args:
            entity: Filter by entity (Run, Asset, or TestReport).
            remote_file_id: Filter by remote file ID.
            file_name: Filter by file name.
            entity_type: Filter by entity type enum value (e.g., 1 for Run, 3 for Asset, 5 for TestReport).
            entity_id: Filter by entity ID.
            order_by: The field to order by.
            limit: Maximum number of results to return.
            page_size: Number of results per page.

        Returns:
            A list of FileAttachments.
        """
        # Build filter parts
        filter_parts = []

        if entity is not None:
            filter_parts.append(cel.equals("entity_id", entity._id_or_error))
            filter_parts.append(cel.equals("entity_type", entity._get_entity_type_name()))
        else:
            if entity_id:
                filter_parts.append(cel.equals("entity_id", entity_id))
            if entity_type:
                filter_parts.append(cel.equals("entity_type", entity_type))
        if remote_file_id:
            filter_parts.append(cel.equals("remote_file_id", remote_file_id))
        if file_name:
            filter_parts.append(cel.equals("file_name", file_name))

        query_filter = cel.and_(*filter_parts)

        file_attachments = await self._low_level_client.list_all_remote_files(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            page_size=page_size,
            sift_client=self.client,
        )
        return self._apply_client_to_instances(file_attachments)

    async def update(
        self,
        *,
        file_attachment: FileAttachmentUpdate | dict,
    ) -> FileAttachment:
        """Update a file attachment.

        Args:
            file_attachment: The FileAttachmentUpdate with fields to update.

        Returns:
            The updated FileAttachment.
        """
        from sift_client.sift_types.file_attachment import FileAttachmentUpdate

        if isinstance(file_attachment, dict):
            file_attachment = FileAttachmentUpdate.model_validate(file_attachment)

        updated = await self._low_level_client.update_remote_file(
            update=file_attachment,
            sift_client=self.client,
        )
        return self._apply_client_to_instance(updated)

    async def delete(
        self, *, file_attachments: list[FileAttachment | str] | FileAttachment | str
    ) -> None:
        """Batch delete multiple file attachments.

        Args:
            file_attachments: List of FileAttachments or the IDs of the file attachments to delete (up to 1000).
        """
        from sift_client.sift_types.file_attachment import FileAttachment

        file_attachment_ids: list[str] = []
        if isinstance(file_attachments, FileAttachment):
            file_attachment_ids.append(file_attachments._id_or_error)
        elif isinstance(file_attachments, str):
            file_attachment_ids.append(file_attachments)
        elif isinstance(file_attachments, list):
            for file_attachment in file_attachments:
                if isinstance(file_attachment, FileAttachment):
                    file_attachment_ids.append(file_attachment._id_or_error)
                elif isinstance(file_attachment, str):
                    file_attachment_ids.append(file_attachment)
                else:
                    raise ValueError(
                        "file_attachments must be a list of FileAttachment or list of str"
                    )
        else:
            raise ValueError(
                "file_attachments must be a FileAttachment, a string, or a list of FileAttachment or strings"
            )
        await self._low_level_client.batch_delete_remote_files(remote_file_ids=file_attachment_ids)

    async def get_download_url(self, *, file_attachment: FileAttachment | str) -> str:
        """Get a download URL for a file attachment.

        Args:
            file_attachment: The FileAttachment or the ID of the file attachment.

        Returns:
            The download URL for the file attachment.
        """
        from sift_client.sift_types.file_attachment import FileAttachment

        attachment_id = (
            file_attachment._id_or_error
            if isinstance(file_attachment, FileAttachment)
            else file_attachment
        )
        return await self._low_level_client.get_remote_file_download_url(
            remote_file_id=attachment_id
        )

    async def download(
        self, *, file_attachment: FileAttachment | str, output_path: str | Path
    ) -> None:
        """Download a file attachment to a local path.

        Args:
            file_attachment: The FileAttachment or the ID of the file attachment to download.
            output_path: The path to download the file attachment to.
        """
        from sift_py.file_attachment._internal.download import download_remote_file

        download_url = await self.get_download_url(file_attachment=file_attachment)
        download_remote_file(download_url, Path(output_path))

    async def upload(
        self,
        *,
        path: str | Path,
        entity: Asset | Run | TestReport,
        metadata: dict[str, Any] | None = None,
        description: str | None = None,
        organization_id: str | None = None,
    ) -> FileAttachment:
        """Upload a file attachment to a remote file.

        Args:
            path: The path to the file to upload.
            entity: The entity that the file is attached to.
            metadata: Optional metadata for the file (e.g., video/image metadata).
            description: Optional description of the file.
            organization_id: Optional organization ID.

        Returns:
            The uploaded FileAttachment.
        """
        remote_file_id = await self._upload_client.upload_attachment(
            path=path,
            entity_id=entity._id_or_error,
            entity_type=entity._get_entity_type_name(),
            metadata=metadata,
            description=description,
            organization_id=organization_id,
        )
        # Should be able to remove await
        return await self.get(file_attachment_id=remote_file_id)
