from __future__ import annotations

from typing import TYPE_CHECKING, Any, ClassVar, Protocol

if TYPE_CHECKING:
    from pathlib import Path

    from sift_client.client import SiftClient
    from sift_client.sift_types.file_attachment import FileAttachment


class _SupportsFileAttachments(Protocol):
    """Protocol for types that support file attachments."""

    @property
    def client(self) -> SiftClient: ...

    @property
    def id_(self) -> str | None: ...


class FileAttachmentsMixin:
    """Mixin for sift_types that support file attachments (remote files).

    This mixin assumes the class also inherits from BaseType, which provides:
    - id_: str | None
    - client: SiftClient property

    The entity type is automatically determined from the class name:
    - Asset -> assets
    - Run -> runs
    - TestReport -> test_reports
    """

    # Mapping of class names to entity types (REST API format)
    _ENTITY_TYPE_MAP: ClassVar[dict[str, str]] = {
        "Asset": "assets",
        "Run": "runs",
        "TestReport": "test_reports",
    }

    def _get_entity_type_name(self) -> str:
        """Get the entity type string.

        Returns:
            The entity type string (e.g., 'assets', 'runs', 'test_reports')

        Raises:
            ValueError: If the class name is not in the entity type mapping.
        """
        class_name = self.__class__.__name__
        entity_type = self._ENTITY_TYPE_MAP.get(class_name)

        if not entity_type:
            raise ValueError(
                f"{class_name} is not configured for attachments. "
                f"Add it to FileAttachmentsMixin._ENTITY_TYPE_MAP"
            )

        return entity_type

    @property
    def attachments(self: _SupportsFileAttachments) -> list[FileAttachment]:
        """Get all file attachments for this entity.

        Returns:
            A list of FileAttachments associated with this entity.
        """
        from sift_client.sift_types.asset import Asset
        from sift_client.sift_types.run import Run
        from sift_client.sift_types.test_report import TestReport

        if not isinstance(self, (Asset, Run, TestReport)):
            raise ValueError("Entity is not a valid entity type")
        return self.client.file_attachments.list_(
            entities=[self],
        )

    def delete_attachment(
        self: _SupportsFileAttachments,
        file_attachment: list[FileAttachment | str] | FileAttachment | str,
    ) -> None:
        """Delete one or more file attachments.

        Args:
            file_attachment: A single FileAttachment or list of FileAttachments to delete.
        """
        self.client.file_attachments.delete(file_attachments=file_attachment)

    def upload_attachment(
        self: _SupportsFileAttachments,
        path: str | Path,
        metadata: dict[str, Any] | None = None,
        description: str | None = None,
        organization_id: str | None = None,
    ) -> FileAttachment:
        """Upload a file attachment to a remote file.

        Args:
            path: The path to the file to upload.
            metadata: Optional metadata for the file (e.g., video/image metadata).
            description: Optional description of the file.
            organization_id: Optional organization ID.

        Returns:
            The uploaded FileAttachment.
        """
        from sift_client.sift_types.asset import Asset
        from sift_client.sift_types.run import Run
        from sift_client.sift_types.test_report import TestReport

        if not isinstance(self, (Asset, Run, TestReport)):
            raise ValueError("Entity is not a valid entity type")
        return self.client.file_attachments.upload(
            path=path,
            entity=self,
            metadata=metadata,
            description=description,
            organization_id=organization_id,
        )
