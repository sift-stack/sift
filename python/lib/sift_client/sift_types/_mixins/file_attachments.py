from __future__ import annotations

from typing import TYPE_CHECKING, Any, ClassVar

if TYPE_CHECKING:
    from pathlib import Path

    from sift_client.sift_types._base import BaseTypeProtocol
    from sift_client.sift_types.file_attachment import FileAttachment


class FileAttachmentsMixin:
    """Mixin for sift_types that support file attachments (remote files).

    This mixin assumes the class also inherits from BaseType, which provides:
    - id_: str | None
    - client: SiftClient property
    """

    # Mapping of class names to entity types (REST API format)
    _ENTITY_TYPE_MAP: ClassVar[dict[str, str]] = {
        "Asset": "assets",
        "Run": "runs",
        "TestReport": "test_reports",
        "TestStep": "test_steps",
    }

    @staticmethod
    def check_is_supported_entity_type(cls):
        """Check if the entity type is supported for file attachments.

        Returns:
            True if the entity type is supported, False otherwise.
        """
        if not cls.__class__.__name__ in FileAttachmentsMixin._ENTITY_TYPE_MAP:
            raise ValueError(f"{cls.__name__} does not support file attachments")

    def _get_entity_type_name(self) -> str:
        """Get the entity type string.

        Returns:
            The entity type string (e.g., 'assets', 'runs', 'test_reports')

        Raises:
            ValueError: If the class name is not in the entity type mapping.
        """
        class_name = self.__class__.__name__
        entity_type = FileAttachmentsMixin._ENTITY_TYPE_MAP.get(self.__class__.__name__)

        if not entity_type:
            raise ValueError(
                f"{class_name} is not configured for attachments. "
                f"Add it to FileAttachmentsMixin._ENTITY_TYPE_MAP"
            )

        return entity_type

    @property
    def attachments(self: BaseTypeProtocol) -> list[FileAttachment]:
        """Get all file attachments for this entity.

        Returns:
            A list of FileAttachments associated with this entity.
        """
        FileAttachmentsMixin.check_is_supported_entity_type(self)
        return self.client.file_attachments.list_(
            entities=[self],  # type: ignore
        )

    def delete_attachment(
        self: BaseTypeProtocol,
        file_attachment: list[FileAttachment | str] | FileAttachment | str,
    ) -> None:
        """Delete one or more file attachments.

        Args:
            file_attachment: A single FileAttachment or list of FileAttachments to delete.
        """
        self.client.file_attachments.delete(file_attachments=file_attachment)

    def upload_attachment(
        self: BaseTypeProtocol,
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
        FileAttachmentsMixin.check_is_supported_entity_type(self)
        return self.client.file_attachments.upload(
            path=path,
            entity=self,  # type: ignore
            metadata=metadata,
            description=description,
            organization_id=organization_id,
        )
