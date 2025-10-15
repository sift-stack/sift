from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, ClassVar

from pydantic import ConfigDict
from sift.reports.v1.reports_pb2 import Report as ReportProto
from sift.reports.v1.reports_pb2 import ReportRuleSummary as ReportRuleSummaryProto
from sift.reports.v1.reports_pb2 import ReportTag as ReportTagProto

from sift_client.sift_types._base import BaseType, MappingHelper, ModelUpdate
from sift_client.sift_types.tag import Tag
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class ReportRuleStatus(Enum):
    """Report rule status."""

    UNSPECIFIED = 0
    CREATED = 1
    LIVE = 2
    FINISHED = 3
    FAILED = 4
    CANCELED = 5
    ERROR = 6


class ReportRuleSummary(BaseType[ReportRuleSummaryProto, "ReportRuleSummary"]):
    """ReportRuleSummary model representing a rule summary within a report."""

    rule_id: str
    rule_client_key: str | None = None
    rule_version_id: str
    rule_version_number: int
    report_rule_version_id: str
    num_open: int
    num_failed: int
    num_passed: int
    status: ReportRuleStatus
    created_date: datetime
    modified_date: datetime
    asset_id: str
    deleted_date: datetime | None = None

    @classmethod
    def _from_proto(
        cls, proto: ReportRuleSummaryProto, sift_client: SiftClient | None = None
    ) -> ReportRuleSummary:
        return cls(
            id_=proto.report_rule_version_id,
            rule_id=proto.rule_id,
            rule_client_key=proto.rule_client_key,
            rule_version_id=proto.rule_version_id,
            rule_version_number=proto.rule_version_number,
            report_rule_version_id=proto.report_rule_version_id,
            num_open=proto.num_open,
            num_failed=proto.num_failed,
            num_passed=proto.num_passed,
            status=ReportRuleStatus(proto.status),
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            asset_id=proto.asset_id,
            deleted_date=proto.deleted_date.ToDatetime(tzinfo=timezone.utc)
            if proto.HasField("deleted_date")
            else None,
            _client=sift_client,
        )

    def to_proto(self) -> ReportRuleSummaryProto:
        """Convert to protobuf message."""
        proto = ReportRuleSummaryProto(
            rule_id=self.rule_id or "",
            rule_client_key=self.rule_client_key or "",
            rule_version_id=self.rule_version_id or "",
            rule_version_number=self.rule_version_number,
            report_rule_version_id=self.report_rule_version_id or "",
            num_open=self.num_open,
            num_failed=self.num_failed,
            num_passed=self.num_passed,
            status=self.status.value,  # type: ignore
            asset_id=self.asset_id,
        )
        proto.created_date.FromDatetime(self.created_date)
        proto.modified_date.FromDatetime(self.modified_date)
        if self.deleted_date:
            proto.deleted_date.FromDatetime(self.deleted_date)
        return proto


class Report(BaseType[ReportProto, "Report"]):
    """Report model representing a data analysis report."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    report_template_id: str
    run_id: str
    organization_id: str
    name: str
    description: str
    created_by_user_id: str
    modified_by_user_id: str
    created_date: datetime
    modified_date: datetime
    summaries: list[ReportRuleSummary]
    tags: list[str]
    rerun_from_report_id: str | None = None
    metadata: dict[str, str | float | bool]
    job_id: str
    archived_date: datetime | None = None
    is_archived: bool

    @classmethod
    def _from_proto(cls, proto: ReportProto, sift_client: SiftClient | None = None) -> Report:
        return cls(
            id_=proto.report_id,
            report_template_id=proto.report_template_id,
            run_id=proto.run_id,
            organization_id=proto.organization_id,
            name=proto.name,
            description=proto.description,
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            summaries=[
                ReportRuleSummary._from_proto(summary, sift_client) for summary in proto.summaries
            ],
            tags=[tag.tag_name for tag in proto.tags],
            rerun_from_report_id=proto.rerun_from_report_id,
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            job_id=proto.job_id,
            archived_date=proto.archived_date.ToDatetime(tzinfo=timezone.utc)
            if proto.HasField("archived_date")
            else None,
            is_archived=proto.is_archived,
            _client=sift_client,
        )

    def to_proto(self) -> ReportProto:
        """Convert to protobuf message."""
        proto = ReportProto(
            report_id=self.id_ or "",
            run_id=self.run_id or "",
            organization_id=self.organization_id or "",
            created_by_user_id=self.created_by_user_id,
            modified_by_user_id=self.modified_by_user_id,
            name=self.name,
            description=self.description,
            report_template_id=self.report_template_id or "",
            tags=[ReportTagProto(tag_name=tag) for tag in self.tags],
            summaries=[summary.to_proto() for summary in self.summaries],
            job_id=self.job_id or "",
            is_archived=self.is_archived,
        )
        proto.created_date.FromDatetime(self.created_date)
        proto.modified_date.FromDatetime(self.modified_date)
        if self.archived_date:
            proto.archived_date.FromDatetime(self.archived_date)
        return proto

    def archive(self) -> Report:
        """Archive the Report."""
        updated_report = self.client.reports.archive(report=self)
        self._update(updated_report)
        return self

    def unarchive(self) -> Report:
        """Unarchive the Report."""
        updated_report = self.client.reports.unarchive(report=self)
        self._update(updated_report)
        return self


class ReportUpdate(ModelUpdate[ReportProto]):
    """Model of the Report fields that can be updated."""

    is_archived: bool | None = None
    metadata: dict[str, str | float | bool] | None = None
    tags: list[str | Tag] | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "metadata": MappingHelper(
            proto_attr_path="metadata", update_field="metadata", converter=metadata_dict_to_proto
        ),
        "tags": MappingHelper(
            proto_attr_path="tags",
            update_field="tags",
            converter=lambda tags: [tag.name if isinstance(tag, Tag) else tag for tag in tags],
        ),
    }

    def _get_proto_class(self) -> type[ReportProto]:
        return ReportProto

    def _add_resource_id_to_proto(self, proto_msg: ReportProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.report_id = self._resource_id
