from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.remote_files import RemoteFilesLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.file_attachment import FileAttachment, FileAttachmentUpdate

if TYPE_CHECKING:
    from sift_client.client import SiftClient


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
        entity_type: str | None = None,
        entity_id: str | None = None,
        query_filter: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[FileAttachment]:
        """List file attachments with optional filtering.

        Args:
            entity_type: Filter by entity type (e.g., 'ENTITY_TYPE_ASSET', 'ENTITY_TYPE_RUN').
            entity_id: Filter by entity ID.
            query_filter: Optional CEL query filter.
            order_by: The field to order by.
            limit: Maximum number of results to return.
            page_size: Number of results per page.

        Returns:
            A list of FileAttachments.
        """
        # Build the filter
        filters = []
        if entity_type:
            filters.append(f'entity_type=="{entity_type}"')
        if entity_id:
            filters.append(f'entity_id=="{entity_id}"')
        if query_filter:
            filters.append(query_filter)

        combined_filter = " && ".join(filters) if filters else None

        file_attachments = await self._low_level_client.list_all_remote_files(
            query_filter=combined_filter,
            order_by=order_by,
            max_results=limit,
            page_size=page_size,
            sift_client=self.client,
        )
        return [self._apply_client_to_instance(fa) for fa in file_attachments]

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
        file_attachment_ids: list[str] = []
        if isinstance(file_attachments, FileAttachment):
            if file_attachments.id_ is not None:
                file_attachment_ids.append(file_attachments.id_)
            else:
                raise ValueError("FileAttachment ID is not set")
        elif isinstance(file_attachments, str):
            file_attachment_ids.append(file_attachments)
        elif isinstance(file_attachments, list):
            for file_attachment in file_attachments:
                if isinstance(file_attachment, FileAttachment):
                    if file_attachment.id_ is not None:
                        file_attachment_ids.append(file_attachment.id_)
                    else:
                        raise ValueError("FileAttachment ID is not set")
                elif isinstance(file_attachment, str):
                    file_attachment_ids.append(file_attachment)
                else:
                    raise ValueError(
                        "file_attachments must be a list of FileAttachment or list of str"
                    )
        await self._low_level_client.batch_delete_remote_files(remote_file_ids=file_attachment_ids)

    async def get_download_url(self, *, file_attachment: FileAttachment | str) -> str:
        """Get a download URL for a file attachment.

        Args:
            file_attachment: The FileAttachment or the ID of the file attachment.

        Returns:
            The download URL for the file attachment.
        """
        id_: str = ""
        if isinstance(file_attachment, FileAttachment):
            if file_attachment.id_ is not None:
                id_ = file_attachment.id_
            else:
                raise ValueError("FileAttachment ID is not set")
        elif isinstance(file_attachment, str):
            id_ = file_attachment
        else:
            raise ValueError("file_attachment must be a FileAttachment or a string")
        if id_ == "":
            raise ValueError("FileAttachment ID is not set")
        return await self._low_level_client.get_remote_file_download_url(
            remote_file_id=id_
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
