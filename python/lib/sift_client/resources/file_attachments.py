from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.remote_files import RemoteFilesLowLevelClient
from sift_client.resources._base import ResourceBase

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.file_attachment import FileAttachment, RemoteFileUpdate


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
        file_attachment: RemoteFileUpdate | dict,
    ) -> FileAttachment:
        """Update a file attachment.

        Args:
            file_attachment: The RemoteFileUpdate with fields to update.

        Returns:
            The updated FileAttachment.
        """
        from sift_client.sift_types.file_attachment import RemoteFileUpdate

        if isinstance(file_attachment, dict):
            file_attachment = RemoteFileUpdate.model_validate(file_attachment)

        updated = await self._low_level_client.update_remote_file(
            update=file_attachment,
            sift_client=self.client,
        )
        return self._apply_client_to_instance(updated)

    async def delete(self, *, file_attachment_id: str) -> None:
        """Delete a file attachment.

        Args:
            file_attachment_id: The ID of the file attachment to delete.
        """
        await self._low_level_client.delete_remote_file(remote_file_id=file_attachment_id)

    async def batch_delete(self, *, file_attachment_ids: list[str]) -> None:
        """Batch delete multiple file attachments.

        Args:
            file_attachment_ids: List of file attachment IDs to delete (up to 1000).
        """
        await self._low_level_client.batch_delete_remote_files(remote_file_ids=file_attachment_ids)

    async def get_download_url(self, *, file_attachment_id: str) -> str:
        """Get a download URL for a file attachment.

        Args:
            file_attachment_id: The ID of the file attachment.

        Returns:
            The download URL for the file attachment.
        """
        return await self._low_level_client.get_remote_file_download_url(
            remote_file_id=file_attachment_id
        )
