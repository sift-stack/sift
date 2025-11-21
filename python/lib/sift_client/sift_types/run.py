from __future__ import annotations

from datetime import datetime, timedelta, timezone
from typing import TYPE_CHECKING, ClassVar

from pydantic import model_validator
from sift.runs.v2.runs_pb2 import CreateRunRequest as CreateRunRequestProto
from sift.runs.v2.runs_pb2 import Run as RunProto

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)
from sift_client.sift_types._mixins.file_attachments import FileAttachmentsMixin
from sift_client.sift_types.tag import Tag
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_stream_bindings import RunFormPy

    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset


class Run(BaseType[RunProto, "Run"], FileAttachmentsMixin):
    """Run model representing a data collection run."""

    # Required fields
    name: str
    description: str
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    organization_id: str
    metadata: dict[str, str | float | bool]
    tags: list[str]
    asset_ids: list[str]
    is_adhoc: bool
    is_archived: bool

    # Optional fields
    start_time: datetime | None
    stop_time: datetime | None
    duration: timedelta | None
    default_report_id: str | None
    client_key: str | None
    archived_date: datetime | None

    @classmethod
    def _from_proto(cls, proto: RunProto, sift_client: SiftClient | None = None) -> Run:
        return cls(
            proto=proto,
            id_=proto.run_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            organization_id=proto.organization_id,
            start_time=(
                proto.start_time.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("start_time")
                else None
            ),
            stop_time=(
                proto.stop_time.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("stop_time")
                else None
            ),
            duration=(proto.duration.ToTimedelta() if proto.HasField("duration") else None),
            name=proto.name,
            description=proto.description,
            tags=list(proto.tags),
            default_report_id=proto.default_report_id,
            client_key=proto.client_key if proto.HasField("client_key") else None,
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            asset_ids=list(proto.asset_ids),
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            is_archived=proto.is_archived,
            is_adhoc=proto.is_adhoc,
            _client=sift_client,
        )

    @property
    def assets(self) -> list[Asset]:
        """Return all assets associated with this run."""
        if not self.asset_ids:
            return []
        return self.client.assets.list_(asset_ids=self.asset_ids)

    def archive(self) -> Run:
        """Archive the run."""
        updated_run = self.client.runs.archive(run=self)
        self._update(updated_run)
        return self

    def unarchive(self) -> Run:
        """Unarchive the run."""
        updated_run = self.client.runs.unarchive(run=self)
        self._update(updated_run)
        return self

    def update(self, update: RunUpdate | dict) -> Run:
        """Update the Run.

        Args:
            update: The update to apply to the run. See RunUpdate for more updatable fields.

        Returns:
            The updated run.
        """
        updated_run = self.client.runs.update(run=self, update=update)
        self._update(updated_run)
        return self

    def stop(self) -> Run:
        """Stop the run."""
        self.client.runs.stop(run=self)
        updated_run = self.client.runs.get(run_id=self.id_)
        self._update(updated_run)
        return self


class RunBase(ModelCreateUpdateBase):
    """Base class for Run create and update models with shared fields and validation."""

    description: str | None = None
    start_time: datetime | None = None
    stop_time: datetime | None = None
    tags: list[str] | list[Tag] | None = None
    metadata: dict[str, str | float | bool] | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "metadata": MappingHelper(
            proto_attr_path="metadata",
            update_field="metadata",
            converter=metadata_dict_to_proto,
        ),
        "tags": MappingHelper(
            proto_attr_path="tags",
            update_field="tags",
            converter=lambda tags: [tag.name if isinstance(tag, Tag) else tag for tag in tags],
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
    organization_id: str | None = None

    def _get_proto_class(self) -> type[CreateRunRequestProto]:
        return CreateRunRequestProto

    def _to_rust_form(self) -> RunFormPy:
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        from sift_stream_bindings import MetadataPy, MetadataValuePy, RunFormPy

        if self.client_key:
            client_key = self.client_key
        else:
            client_key = self.name

        if self.tags:
            tags = [tag.name if isinstance(tag, Tag) else tag for tag in self.tags]
        else:
            tags = None

        if self.metadata:
            metadata = []
            for key, value in self.metadata.items():
                metadata.append(MetadataPy(key=key, value=MetadataValuePy(value)))
        else:
            metadata = None

        return RunFormPy(
            name=self.name,
            client_key=client_key,
            description=self.description,
            tags=tags,
            metadata=metadata,
        )


class RunUpdate(RunBase, ModelUpdate[RunProto]):
    """Update model for Run."""

    name: str | None = None
    is_archived: bool | None = None

    def _get_proto_class(self) -> type[RunProto]:
        return RunProto

    def _add_resource_id_to_proto(self, proto_msg: RunProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.run_id = self._resource_id
