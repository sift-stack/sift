from __future__ import annotations

from typing import TYPE_CHECKING, ClassVar

if TYPE_CHECKING:
    from sift_client.sift_types._base import BaseType
    from sift_client.sift_types.file_attachment import FileAttachment


class AttachmentsMixin:
    """Mixin for sift_types that support file attachments (remote files).

    This mixin assumes the class also inherits from BaseType, which provides:
    - id_: str | None
    - client: SiftClient property

    The entity type is automatically determined from the class name:
    - Asset -> ENTITY_TYPE_ASSET
    - Run -> ENTITY_TYPE_RUN
    - TestReport -> ENTITY_TYPE_TEST_REPORT
    """

    # Mapping of class names to entity types
    _ENTITY_TYPE_MAP: ClassVar[dict[str, str]] = {
        "Asset": "ENTITY_TYPE_ASSET",
        "Run": "ENTITY_TYPE_RUN",
        "TestReport": "ENTITY_TYPE_TEST_REPORT",
    }

    def _get_entity_type_name(self) -> str:
        """Get the entity type for filtering based on the class name.

        Returns:
            The entity type string (e.g., 'ENTITY_TYPE_ASSET', 'ENTITY_TYPE_RUN')

        Raises:
            ValueError: If the class name is not in the entity type mapping.
        """
        class_name = self.__class__.__name__
        entity_type = self._ENTITY_TYPE_MAP.get(class_name)

        if not entity_type:
            raise ValueError(
                f"{class_name} is not configured for attachments. "
                f"Add it to AttachmentsMixin._ENTITY_TYPE_MAP"
            )

        return entity_type

    @property
    def attachments(self: BaseType) -> list[FileAttachment]:
        # Type ignore because mixin assumes BaseType attributes (client, id_)
        return self.client.file_attachments.list(
            entity_type=self._get_entity_type_name(),
            entity_id=self.id_,
        )

    def add_attachment(
        self: BaseType, file_attachment: FileAttachment | list[FileAttachment]
    ) -> FileAttachment:
        return self.client.file_attachments.add(
            entity_type=self._get_entity_type_name(),
            entity=self.id_,
            file_attachment=file_attachment,
        )

    def delete_attachment(
        self: BaseType, file_attachment: FileAttachment | list[FileAttachment]
    ) -> FileAttachment:
        return self.client.file_attachments.add(
            entity_type=self._get_entity_type_name(),
            entity=self.id_,
            file_attachment=file_attachment,
        )
