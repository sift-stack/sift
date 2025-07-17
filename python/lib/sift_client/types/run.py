from __future__ import annotations

from datetime import datetime
from typing import TYPE_CHECKING, List, Type

from pydantic import ConfigDict
from sift.runs.v2.runs_pb2 import Run as RunProto

from sift_client.types._base import BaseType, MappingHelper, ModelUpdate
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class RunUpdate(ModelUpdate[RunProto]):
    """
    Update model for Run.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str | None = None
    description: str | None = None
    start_time: datetime | None = None
    stop_time: datetime | None = None
    is_pinned: bool | None = None
    client_key: str | None = None
    tags: List[str] | None = None
    metadata: dict[str, str | float | bool] | None = None

    _to_proto_helpers = {
        "metadata": MappingHelper(
            proto_attr_path="metadata", update_field="metadata", converter=metadata_dict_to_proto
        ),
    }

    def _get_proto_class(self) -> Type[RunProto]:
        return RunProto

    def _add_resource_id_to_proto(self, proto_msg: RunProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.run_id = self._resource_id


class Run(BaseType[RunProto, "Run"]):
    """
    Run model representing a data collection run.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    id: str
    name: str
    description: str
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    organization_id: str
    start_time: datetime | None = None
    stop_time: datetime | None = None
    tags: List[str] | None = None
    default_report_id: str | None = None
    client_key: str | None = None
    metadata: dict[str, str | float | bool]
    asset_ids: List[str] | None = None
    archived_date: datetime | None = None

    @classmethod
    def _from_proto(cls, proto: RunProto, sift_client: SiftClient | None = None) -> Run:
        return cls(
            id=proto.run_id,
            created_date=proto.created_date.ToDatetime(),
            modified_date=proto.modified_date.ToDatetime(),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            organization_id=proto.organization_id,
            start_time=proto.start_time.ToDatetime() if proto.HasField("start_time") else None,
            stop_time=proto.stop_time.ToDatetime() if proto.HasField("stop_time") else None,
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

    def to_proto(self) -> RunProto:
        """
        Convert to protobuf message.
        """
        proto = RunProto(
            run_id=self.id,
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

    def assets(self):
        """
        Return all assets associated with this run.
        """
        if not hasattr(self, "client") or self.client is None:
            raise RuntimeError("Run is not bound to a client instance.")
        if not self.asset_ids:
            return []
        return self.client.assets.list_(asset_ids=self.asset_ids)

    def stop(self):
        """
        Stop the run.
        """
        if not hasattr(self, "client") or self.client is None:
            raise RuntimeError("Run is not bound to a client instance.")
        self.client.runs.stop_run(self.id)
