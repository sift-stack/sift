from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING, ClassVar

from pydantic import BaseModel, Field
from sift.campaigns.v1.campaigns_pb2 import (
    Campaign as CampaignProto,
)
from sift.campaigns.v1.campaigns_pb2 import (
    CampaignReport as CampaignReportProto,
)
from sift.campaigns.v1.campaigns_pb2 import (
    CreateCampaignRequest as CreateCampaignRequestProto,
)

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)
from sift_client.sift_types.report import Report  # noqa: TC001
from sift_client.sift_types.tag import Tag
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.run import Run


class RuleStatistics(BaseModel):
    """How a report's rules came out.

    Attributes:
        annotations: Annotations across the report's rules.
        passed: Rules that never triggered.
        accepted: Rules whose annotations are all accepted.
        failed: Rules with any failed annotation.
        open: Rules with open annotations and none failed.
    """

    annotations: int = 0
    passed: int = 0
    accepted: int = 0
    failed: int = 0
    open: int = 0


class CampaignReportSummary(BaseModel):
    """A report in a campaign, with a rollup of its rule outcomes.

    Read only. The counts are populated only when the campaign was fetched with summaries.

    Attributes:
        report_id: The report this entry refers to.
        report_name: The report's name.
        rule_statistics: The rollup of the report's rule outcomes.
    """

    report_id: str
    report_name: str = ""
    rule_statistics: RuleStatistics = RuleStatistics()

    @classmethod
    def _from_proto(cls, proto: CampaignReportProto) -> CampaignReportSummary:
        return cls(
            report_id=proto.report_id,
            report_name=proto.report_name,
            rule_statistics=RuleStatistics(
                annotations=proto.num_annotations,
                passed=proto.num_passed_rules,
                accepted=proto.num_accepted_rules,
                failed=proto.num_failed_rules,
                open=proto.num_open_rules,
            ),
        )


class Campaign(BaseType[CampaignProto, "Campaign"]):
    """A campaign, which is a named list of reports.

    Runs are attached through the reports they generate, not directly.
    """

    # Required fields
    name: str
    organization_id: str
    report_summaries: list[CampaignReportSummary]
    tags: list[str]
    metadata: dict[str, str | float | bool]
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    is_archived: bool
    reports_include_summaries: bool

    # Optional fields
    description: str | None
    client_key: str | None
    created_from_campaign_id: str | None
    archived_date: datetime | None

    @classmethod
    def _from_proto(cls, proto: CampaignProto, sift_client: SiftClient | None = None) -> Campaign:
        return cls(
            proto=proto,
            id_=proto.campaign_id,
            name=proto.name,
            organization_id=proto.organization_id,
            report_summaries=sorted(
                (CampaignReportSummary._from_proto(r) for r in proto.reports),
                key=lambda r: r.report_id,
            ),
            tags=[t.name or t.tag_id for t in proto.tags],
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            is_archived=proto.is_archived,
            reports_include_summaries=proto.reports_include_summaries,
            description=proto.description if proto.HasField("description") else None,
            client_key=proto.client_key if proto.HasField("client_key") else None,
            created_from_campaign_id=(
                proto.created_from_campaign_id
                if proto.HasField("created_from_campaign_id")
                else None
            ),
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            _client=sift_client,
        )

    @property
    def reports(self) -> list[Report]:
        """Fetch the full Reports in this campaign."""
        if not self.report_summaries:
            return []
        return self.client.reports.list_(report_ids=[r.report_id for r in self.report_summaries])

    @property
    def runs(self) -> list[Run]:
        """Fetch the Runs behind this campaign's reports. Reports with no run are skipped."""
        run_ids = [r.run_id for r in self.reports if r.run_id]
        if not run_ids:
            return []
        return self.client.runs.list_(run_ids=run_ids)

    def add_reports(self, reports: list[Report] | list[str]) -> Campaign:
        """Add reports to the campaign, keeping the ones already there."""
        updated = self.client.campaigns.add_reports(campaign=self, reports=reports)
        self._update(updated)
        return self

    def add_runs(self, runs: list[Run] | list[str]) -> Campaign:
        """Add runs to the campaign through the reports they generated."""
        updated = self.client.campaigns.add_runs(campaign=self, runs=runs)
        self._update(updated)
        return self

    def update(self, update: CampaignUpdate | dict) -> Campaign:
        """Update the Campaign.

        Args:
            update: The update to apply. See CampaignUpdate for updatable fields.

        Returns:
            The updated campaign.
        """
        updated = self.client.campaigns.update(campaign=self, update=update)
        self._update(updated)
        return self

    def archive(self) -> Campaign:
        """Archive the campaign."""
        updated = self.client.campaigns.archive(campaign=self)
        self._update(updated)
        return self

    def unarchive(self) -> Campaign:
        """Unarchive the campaign."""
        updated = self.client.campaigns.unarchive(campaign=self)
        self._update(updated)
        return self


def tag_names(tags: list[str] | list[Tag] | None) -> list[str]:
    """Reduce Tags or tag names to plain names."""
    return [tag.name if isinstance(tag, Tag) else tag for tag in tags or []]


def report_ids(reports: list[Report] | list[str] | None) -> list[str]:
    """Reduce Reports or report IDs to plain IDs."""
    return [r if isinstance(r, str) else r._id_or_error for r in reports or []]


class CampaignBase(ModelCreateUpdateBase):
    """Base class for Campaign create and update models."""

    description: str | None = None
    tags: list[str] | list[Tag] | None = Field(default=None, exclude=True)
    metadata: dict[str, str | float | bool] | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "metadata": MappingHelper(
            proto_attr_path="metadata",
            update_field="metadata",
            converter=metadata_dict_to_proto,
        ),
    }


class CampaignCreate(CampaignBase, ModelCreate[CreateCampaignRequestProto]):
    """Create model for Campaign.

    Pass `client_key` to make the campaign addressable by your own identifier.
    """

    name: str
    client_key: str | None = None
    organization_id: str | None = None

    def _get_proto_class(self) -> type[CreateCampaignRequestProto]:
        return CreateCampaignRequestProto


class CampaignUpdate(CampaignBase, ModelUpdate[CampaignProto]):
    """Update model for Campaign.

    `reports` replaces the campaign's report list. Prefer `campaigns.add_reports` or
    `campaigns.add_runs`, which read the current list first. `tags` is unordered, so it
    may come back in a different order.
    """

    name: str | None = None
    reports: list[Report] | list[str] | None = Field(default=None, exclude=True)
    is_archived: bool | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        **CampaignBase._to_proto_helpers,
    }

    def _get_proto_class(self) -> type[CampaignProto]:
        return CampaignProto

    def _add_resource_id_to_proto(self, proto_msg: CampaignProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.campaign_id = self._resource_id
