from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING

from sift.remote_files.v1.remote_files_pb2 import EntityType
from sift.remote_files.v1.remote_files_pb2 import RemoteFile as RemoteFileProto

from sift_client.sift_types._base import BaseType, ModelUpdate

if TYPE_CHECKING:
    from pathlib import Path

    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset
    from sift_client.sift_types.run import Run
    from sift_client.sift_types.test_report import TestReport


class RemoteFileEntityType(Enum):
    """Enum for the entity type of a remote file."""

    UNSPECIFIED = EntityType.ENTITY_TYPE_UNSPECIFIED  # 0
    RUNS = EntityType.ENTITY_TYPE_RUN  # 1
    ANNOTATIONS = EntityType.ENTITY_TYPE_ANNOTATION  # 2
    ASSETS = EntityType.ENTITY_TYPE_ASSET  # 3
    ANNOTATION_LOGS = EntityType.ENTITY_TYPE_ANNOTATION_LOG  # 4
    TEST_REPORTS = EntityType.ENTITY_TYPE_TEST_REPORT  # 5

    @classmethod
    def from_str(cls, val: str) -> RemoteFileEntityType | None:
        """Convert string representation to RemoteFileEntityType."""
        if isinstance(val, str) and val.startswith("ENTITY_TYPE_"):
            for item in cls:
                if "ENTITY_TYPE_" + item.name == val:
                    return item

        return cls(int(val))

    def __str__(self) -> str:
        return "ENTITY_TYPE_" + self.name

    @staticmethod
    def from_api_format(val: str) -> RemoteFileEntityType | None:
        """Convert API format string to RemoteFileEntityType."""
        for item in RemoteFileEntityType:
            if "ENTITY_TYPE_" + item.name == val:
                return item
        return None

    @staticmethod
    def from_proto_value(proto_value: int) -> RemoteFileEntityType:
        """Convert protobuf int value to RemoteFileEntityType."""
        return RemoteFileEntityType(proto_value)


class FileAttachment(BaseType[RemoteFileProto, "FileAttachment"]):
    """Model of the Sift FileAttachment."""

    organization_id: str
    entity_id: str
    entity_type: RemoteFileEntityType
    file_name: str
    file_mime_type: str
    file_content_encoding: str
    storage_key: str
    file_size: int
    """Size of the file in bytes."""
    description: str
    created_by_user_id: str
    modified_by_user_id: str
    created_date: datetime
    modified_date: datetime

    @classmethod
    def _from_proto(
        cls, proto: RemoteFileProto, sift_client: SiftClient | None = None
    ) -> FileAttachment:
        return cls(
            id_=proto.remote_file_id,
            organization_id=proto.organization_id,
            entity_id=proto.entity_id,
            entity_type=RemoteFileEntityType.from_proto_value(proto.entity_type),
            file_name=proto.file_name,
            file_mime_type=proto.file_mime_type,
            file_content_encoding=proto.file_content_encoding,
            storage_key=proto.storage_key,
            file_size=proto.file_size,
            description=proto.description,
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            proto=proto,
            _client=sift_client,
        )

    @property
    def entity(self) -> Run | Asset | TestReport:
        """Get the entity that this file attachment is attached to."""
        if self.entity_type == RemoteFileEntityType.RUNS:
            return self.client.runs.get(run_id=self.entity_id)
        elif self.entity_type == RemoteFileEntityType.ASSETS:
            return self.client.assets.get(asset_id=self.entity_id)
        elif self.entity_type == RemoteFileEntityType.TEST_REPORTS:
            return self.client.test_results.get(test_report_id=self.entity_id)
        elif self.entity_type in (
            RemoteFileEntityType.ANNOTATIONS,
            RemoteFileEntityType.ANNOTATION_LOGS,
        ):
            raise NotImplementedError(
                f"Entity type {self.entity_type} is not yet supported for entity access"
            )
        else:
            raise Exception(f"Unknown remote file entity type: {self.entity_type}")

    def delete(self) -> None:
        """Delete the file attachment."""
        self.client.file_attachments.delete(file_attachments=self)

    def update(self, update: FileAttachmentUpdate | dict) -> FileAttachment:
        """Update the file attachment."""
        if isinstance(update, dict):
            update = FileAttachmentUpdate.model_validate(update)
        if self.id_ is None:
            raise ValueError("Remote file ID is not set")
        update.resource_id = self.id_

        updated_file_attachment = self.client.file_attachments.update(file_attachment=update)
        self._update(updated_file_attachment)
        return self

    def download_url(self) -> str:
        """Get the download URL for the file attachment."""
        if self.id_ is None:
            raise ValueError("file attachment ID is not set")
        return self.client.file_attachments.get_download_url(file_attachment=self)

    def download(self, output_path: str | Path) -> None:
        """Download the file attachment to a local path."""
        self.client.file_attachments.download(file_attachment=self, output_path=output_path)


class FileAttachmentUpdate(ModelUpdate[RemoteFileProto]):
    """Model of the FileAttachment fields that can be updated."""

    description: str | None = None

    def _get_proto_class(self) -> type[RemoteFileProto]:
        return RemoteFileProto

    def _add_resource_id_to_proto(self, proto_msg: RemoteFileProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.remote_file_id = self._resource_id
