from __future__ import annotations

from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.remote_files import RemoteFilesLowLevelClient
from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from pathlib import Path

    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset
    from sift_client.sift_types.file_attachment import (
        FileAttachment,
        FileAttachmentUpdate,
        RemoteFileEntityType,
    )
    from sift_client.sift_types.run import Run
    from sift_client.sift_types.test_report import TestReport, TestStep


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

    def _build_name_cel_filters(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
    ) -> list[str]:
        """Override base implementation to use 'file_name' field instead of 'name'."""
        filter_parts = []
        if name:
            filter_parts.append(cel.equals("file_name", name))
        if names:
            filter_parts.append(cel.in_("file_name", names))
        if name_contains:
            filter_parts.append(cel.contains("file_name", name_contains))
        if name_regex:
            filter_parts.append(cel.match("file_name", name_regex))
        return filter_parts

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
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        remote_file_ids: list[str] | None = None,
        # created/modified ranges TODO: Add to backend
        # created_after: datetime | None = None,
        # created_before: datetime | None = None,
        # modified_after: datetime | None = None,
        # modified_before: datetime | None = None,
        # created/modified users TODO: Add to backend
        # created_by: Any | str | None = None,
        # modified_by: Any | str | None = None,
        # metadata TODO: Add to backend
        # metadata: list[Any] | None = None,
        # file specific
        entities: list[Run | Asset | TestReport | TestStep] | None = None,
        entity_type: RemoteFileEntityType | None = None,
        entity_ids: list[str] | None = None,
        # common filters
        description_contains: str | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[FileAttachment]:
        """List file attachments with optional filtering.

        Args:
            name: Exact name of the file attachment.
            names: List of file attachment names to filter by.
            name_contains: Partial name of the file attachment.
            name_regex: Regular expression to filter file attachments by name.
            remote_file_ids: Filter to file attachments with any of these IDs.
            entities: Filter to file attachments associated with these entities.
            entity_type: Filter to file attachments associated with this entity type.
            entity_ids: Filter to file attachments associated with these entity IDs.
            description_contains: Partial description of the file attachment.
            filter_query: Explicit CEL query to filter file attachments.
            order_by: Field and direction to order results by. Note: Not supported by the backend, but it is here for API consistency.
            limit: Maximum number of file attachments to return. If None, returns all matches.

        Returns:
            A list of FileAttachment objects that match the filter criteria.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            # *self._build_time_cel_filters(
            #     created_after=created_after,
            #     created_before=created_before,
            #     modified_after=modified_after,
            #     modified_before=modified_before,
            #     created_by=created_by,
            #     modified_by=modified_by,
            # ),
            # *self._build_tags_metadata_cel_filters(metadata=metadata),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                filter_query=filter_query,
            ),
        ]

        if not entity_ids:
            entity_ids = []
        if entities:
            entity_ids += [entity._id_or_error for entity in entities]

        if entity_ids:
            filter_parts.append(cel.in_("entity_id", entity_ids))
        if entity_type:
            filter_parts.append(cel.equals("entity_type", entity_type.name.lower()))
        if remote_file_ids:
            filter_parts.append(cel.in_("remote_file_id", remote_file_ids))

        query_filter = cel.and_(*filter_parts)

        file_attachments = await self._low_level_client.list_all_remote_files(
            query_filter=query_filter or None,
            max_results=limit,
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
        if isinstance(file_attachment, str):
            file_attachment = await self.get(file_attachment_id=file_attachment)
        content = await self._low_level_client.download_remote_file(file_attachment=file_attachment)
        with open(output_path, "wb") as f:
            f.write(content)

    async def upload(
        self,
        *,
        path: str | Path,
        entity: Asset | Run | TestReport | TestStep,
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
