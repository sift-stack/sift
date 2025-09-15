from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING, ClassVar

from pydantic import ConfigDict, model_validator
from sift.runs.v2.runs_pb2 import CreateRunRequest as CreateRunRequestProto
from sift.runs.v2.runs_pb2 import Run as RunProto

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset


class Run(BaseType[RunProto, "Run"]):
    """Run model representing a data collection run."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str
    description: str
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    organization_id: str
    start_time: datetime | None = None
    stop_time: datetime | None = None
    tags: list[str] | None = None
    default_report_id: str | None = None
    client_key: str | None = None
    metadata: dict[str, str | float | bool]
    asset_ids: list[str] | None = None
    archived_date: datetime | None = None

    @classmethod
    def _from_proto(cls, proto: RunProto, sift_client: SiftClient | None = None) -> Run:
        return cls(
            id_=proto.run_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            organization_id=proto.organization_id,
            start_time=proto.start_time.ToDatetime(tzinfo=timezone.utc)
            if proto.HasField("start_time")
            else None,
            stop_time=proto.stop_time.ToDatetime(tzinfo=timezone.utc)
            if proto.HasField("stop_time")
            else None,
            name=proto.name,
            description=proto.description,
            tags=list(proto.tags),
            default_report_id=proto.default_report_id,
            client_key=proto.client_key if proto.HasField("client_key") else None,
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            asset_ids=list(proto.asset_ids),
            archived_date=proto.archived_date.ToDatetime()
            if proto.HasField("archived_date")
            else None,
            _client=sift_client,
        )

    def _to_proto(self) -> RunProto:
        """Convert to protobuf message."""
        proto = RunProto(
            run_id=self.id_ or "",
            created_date=self.created_date,  # type: ignore
            modified_date=self.modified_date,  # type: ignore
            created_by_user_id=self.created_by_user_id,
            modified_by_user_id=self.modified_by_user_id,
            organization_id=self.organization_id,
            is_pinned=False,
            name=self.name,
            description=self.description,
            tags=self.tags,
            metadata=metadata_dict_to_proto(self.metadata),
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

    @property
    def assets(self) -> list[Asset]:
        """Return all assets associated with this run."""
        if not self.asset_ids:
            return []
        return self.client.assets.list_(asset_ids=self.asset_ids)


class RunBase(ModelCreateUpdateBase):
    """Base class for Run create and update models with shared fields and validation."""

    description: str | None = None
    start_time: datetime | None = None
    stop_time: datetime | None = None
    tags: list[str] | None = None
    metadata: dict[str, str | float | bool] | None = None

    _to_proto_helpers: ClassVar = {
        "metadata": MappingHelper(
            proto_attr_path="metadata", update_field="metadata", converter=metadata_dict_to_proto
        ),
    }

    @model_validator(mode="after")
    def _validate_time_fields(self):
        """Validate time-related fields after initialization."""
        if self.stop_time is not None and self.start_time is None:
            raise ValueError("start_time must be provided if stop_time is provided")

        if self.start_time is not None and self.stop_time is not None:
            if self.start_time >= self.stop_time:
                raise ValueError("start_time must be before stop_time")

        return self


class RunCreate(RunBase, ModelCreate[CreateRunRequestProto]):
    """Create model for Run."""

    name: str
    client_key: str | None = None

    def _get_proto_class(self) -> type[CreateRunRequestProto]:
        return CreateRunRequestProto


class RunUpdate(RunBase, ModelUpdate[RunProto]):
    """Update model for Run."""

    name: str | None = None

    @model_validator(mode="after")
    def _validate_non_updatable_fields(self):
        """Validate that the fields that cannot be updated are not set."""
        if self.client_key is not None:
            raise ValueError("Cannot update client key")
        return self

    def _get_proto_class(self) -> type[RunProto]:
        return RunProto

    def _add_resource_id_to_proto(self, proto_msg: RunProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.run_id = self._resource_id
