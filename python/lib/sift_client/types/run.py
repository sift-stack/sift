from __future__ import annotations

from datetime import datetime
from typing import List, Optional

from google.protobuf.field_mask_pb2 import FieldMask
from pydantic import BaseModel, ConfigDict
from sift.metadata.v1.metadata_pb2 import MetadataValue as MetadataValueProto
from sift.runs.v2.runs_pb2 import Run as RunProto
from sift_client.types._base import BaseType
from sift_client.types.metadata import MetadataValue


class RunUpdate(BaseModel):
    """
    Update model for Run.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: Optional[str] = None
    description: Optional[str] = None
    start_time: Optional[datetime] = None
    stop_time: Optional[datetime] = None
    is_pinned: Optional[bool] = None
    client_key: Optional[str] = None
    tags: Optional[List[str]] = None
    metadata: Optional[List[MetadataValue]] = None

    def to_update_proto(self) -> tuple[RunProto, FieldMask]:
        """
        Convert to protobuf update request.
        """
        run_proto = RunProto()
        field_mask = FieldMask()

        if self.name is not None:
            run_proto.name = self.name
            field_mask.paths.append("name")

        if self.description is not None:
            run_proto.description = self.description
            field_mask.paths.append("description")

        if self.start_time is not None:
            run_proto.start_time.FromDatetime(self.start_time)
            field_mask.paths.append("start_time")

        if self.stop_time is not None:
            run_proto.stop_time.FromDatetime(self.stop_time)
            field_mask.paths.append("stop_time")

        if self.is_pinned is not None:
            run_proto.is_pinned = self.is_pinned
            field_mask.paths.append("is_pinned")

        if self.client_key is not None:
            run_proto.client_key = self.client_key
            field_mask.paths.append("client_key")

        if self.tags is not None:
            run_proto.tags.extend(self.tags)
            field_mask.paths.append("tags")

        if self.metadata is not None:
            run_proto.metadata.extend([meta.to_proto() for meta in self.metadata])
            field_mask.paths.append("metadata")

        return run_proto, field_mask


class Run(BaseType[RunProto, "Run"]):
    """
    Run model representing a data collection run.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    id: str
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    organization_id: str
    start_time: Optional[datetime] = None
    stop_time: Optional[datetime] = None
    is_pinned: bool
    name: str
    description: str
    tags: List[str]
    default_report_id: Optional[str] = None
    client_key: Optional[str] = None
    metadata: List[MetadataValue]
    asset_ids: List[str]
    archived_date: Optional[datetime] = None

    @classmethod
    def _from_proto(cls, message: RunProto) -> Run:
        return cls(
            id=message.run_id,
            created_date=message.created_date.ToDatetime(),
            modified_date=message.modified_date.ToDatetime(),
            created_by_user_id=message.created_by_user_id,
            modified_by_user_id=message.modified_by_user_id,
            organization_id=message.organization_id,
            start_time=message.start_time.ToDatetime() if message.HasField("start_time") else None,
            stop_time=message.stop_time.ToDatetime() if message.HasField("stop_time") else None,
            is_pinned=message.is_pinned,
            name=message.name,
            description=message.description,
            tags=list(message.tags),
            default_report_id=message.default_report_id,
            client_key=message.client_key if message.HasField("client_key") else None,
            metadata=[MetadataValue._from_proto(meta) for meta in message.metadata],
            asset_ids=list(message.asset_ids),
            archived_date=message.archived_date.ToDatetime()
            if message.HasField("archived_date")
            else None,
        )

    def to_proto(self) -> RunProto:
        """
        Convert to protobuf message.
        """
        proto = RunProto(
            run_id=self.id,
            created_date=self.created_date,
            modified_date=self.modified_date,
            created_by_user_id=self.created_by_user_id,
            modified_by_user_id=self.modified_by_user_id,
            organization_id=self.organization_id,
            is_pinned=self.is_pinned,
            name=self.name,
            description=self.description,
            tags=self.tags,
            metadata=[meta.to_proto() for meta in self.metadata],
            asset_ids=self.asset_ids,
        )

        if self.start_time is not None:
            proto.start_time.FromDatetime(self.start_time)

        if self.stop_time is not None:
            proto.stop_time.FromDatetime(self.stop_time)

        if self.default_report_id is not None:
            proto.default_report_id = self.default_report_id

        if self.client_key is not None:
            proto.client_key = self.client_key

        if self.archived_date is not None:
            proto.archived_date.FromDatetime(self.archived_date)

        return proto

    def assets(self):
        """
        Return all assets associated with this run.
        """
        if not hasattr(self, "client") or self.client is None:
            raise RuntimeError("Run is not bound to a client instance.")
        if not self.asset_ids:
            return []
        # If there are many asset_ids, this could be optimized with a batch_get if available
        return [self.client.assets.get(asset_id=aid) for aid in self.asset_ids]
