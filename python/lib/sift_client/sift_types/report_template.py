from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING, ClassVar

from pydantic import Field, model_validator
from sift.report_templates.v1.report_templates_pb2 import (
    CreateReportTemplateRequest,
    CreateReportTemplateRequestClientKeys,
    CreateReportTemplateRequestRuleIds,
)
from sift.report_templates.v1.report_templates_pb2 import (
    ReportTemplate as ReportTemplateProto,
)
from sift.report_templates.v1.report_templates_pb2 import (
    ReportTemplateRule as ReportTemplateRuleProto,
)
from sift.report_templates.v1.report_templates_pb2 import (
    ReportTemplateTag as ReportTemplateTagProto,
)

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)
from sift_client.sift_types.rule import Rule
from sift_client.sift_types.tag import Tag
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from google.protobuf import field_mask_pb2

    from sift_client.client import SiftClient


class ReportTemplateRule(BaseType[ReportTemplateRuleProto, "ReportTemplateRule"]):
    """ReportTemplateRule model representing a rule attached to a report template."""

    rule_id: str
    rule_version_id: str
    rule_version_number: int
    rule_client_key: str | None = None
    display_order: int

    @classmethod
    def _from_proto(
        cls, proto: ReportTemplateRuleProto, sift_client: SiftClient | None = None
    ) -> ReportTemplateRule:
        return cls(
            proto=proto,
            id_=proto.rule_id,
            rule_id=proto.rule_id,
            rule_version_id=proto.rule_version_id,
            rule_version_number=proto.rule_version_number,
            rule_client_key=proto.client_key if proto.client_key else None,
            display_order=proto.display_order,
            _client=sift_client,
        )

    def to_proto(self) -> ReportTemplateRuleProto:
        """Convert to protobuf message."""
        return ReportTemplateRuleProto(
            rule_id=self.rule_id or "",
            rule_version_id=self.rule_version_id or "",
            rule_version_number=self.rule_version_number,
            client_key=self.rule_client_key or "",
            display_order=self.display_order,
        )


class ReportTemplate(BaseType[ReportTemplateProto, "ReportTemplate"]):
    """ReportTemplate model representing a reusable set of rules for creating reports."""

    organization_id: str
    client_key: str | None = None
    name: str
    description: str | None = None
    created_by_user_id: str
    modified_by_user_id: str
    created_date: datetime
    modified_date: datetime
    rules: list[ReportTemplateRule]
    tags: list[str]
    metadata: dict[str, str | float | bool]
    archived_date: datetime | None = None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: ReportTemplateProto, sift_client: SiftClient | None = None
    ) -> ReportTemplate:
        return cls(
            proto=proto,
            id_=proto.report_template_id,
            organization_id=proto.organization_id,
            client_key=proto.client_key if proto.client_key else None,
            name=proto.name,
            description=proto.description if proto.HasField("description") else None,
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            # Rules are executed in display_order; sort so list position matches.
            rules=sorted(
                (ReportTemplateRule._from_proto(rule, sift_client) for rule in proto.rules),
                key=lambda rule: rule.display_order,
            ),
            tags=[tag.tag_name for tag in proto.tags],
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            archived_date=proto.archived_date.ToDatetime(tzinfo=timezone.utc)
            if proto.HasField("archived_date")
            else None,
            is_archived=proto.is_archived,
            _client=sift_client,
        )

    def to_proto(self) -> ReportTemplateProto:
        """Convert to protobuf message."""
        proto = ReportTemplateProto(
            report_template_id=self.id_ or "",
            organization_id=self.organization_id or "",
            client_key=self.client_key,
            name=self.name,
            description=self.description,
            created_by_user_id=self.created_by_user_id,
            modified_by_user_id=self.modified_by_user_id,
            rules=[rule.to_proto() for rule in self.rules],
            tags=[ReportTemplateTagProto(tag_name=tag) for tag in self.tags],
            metadata=metadata_dict_to_proto(self.metadata) if self.metadata else [],
            is_archived=self.is_archived,
        )
        proto.created_date.FromDatetime(self.created_date)
        proto.modified_date.FromDatetime(self.modified_date)
        if self.archived_date:
            proto.archived_date.FromDatetime(self.archived_date)
        return proto

    def update(self, update: ReportTemplateUpdate | dict) -> ReportTemplate:
        """Update the ReportTemplate.

        Args:
            update: Either a ReportTemplateUpdate instance or a dictionary of key-value pairs to update.
        """
        updated_template = self.client.reports.templates.update(report_template=self, update=update)
        self._update(updated_template)
        return self

    def archive(self) -> ReportTemplate:
        """Archive the ReportTemplate."""
        updated_template = self.client.reports.templates.archive(report_template=self)
        self._update(updated_template)
        return self

    def unarchive(self) -> ReportTemplate:
        """Unarchive the ReportTemplate."""
        updated_template = self.client.reports.templates.unarchive(report_template=self)
        self._update(updated_template)
        return self


class ReportTemplateCreateUpdateBase(ModelCreateUpdateBase):
    """Base class for ReportTemplate create and update models with shared fields and validation."""

    rule_ids: list[str | Rule] | None = None
    rule_client_keys: list[str] | None = None

    @model_validator(mode="after")
    def _validate_rule_identifiers(self):
        if self.rule_ids is not None and self.rule_client_keys is not None:
            raise ValueError("Only one of rule_ids or rule_client_keys may be provided")
        return self


class ReportTemplateCreate(
    ReportTemplateCreateUpdateBase, ModelCreate[CreateReportTemplateRequest]
):
    """Model for creating a new ReportTemplate.

    Note:
        - Rules can be attached with either rule_ids or rule_client_keys, but not both.
        - The order of rule_ids determines each rule's display order on the template.
    """

    name: str
    client_key: str | None = None
    description: str | None = None
    tags: list[str | Tag] | None = None
    metadata: dict[str, str | float | bool] | None = None
    organization_id: str | None = None

    def _get_proto_class(self) -> type[CreateReportTemplateRequest]:
        return CreateReportTemplateRequest

    def to_proto(self) -> CreateReportTemplateRequest:
        """Convert to protobuf message with custom logic."""
        proto = CreateReportTemplateRequest(
            name=self.name,
            client_key=self.client_key,
            description=self.description,
            organization_id=self.organization_id or "",
            tag_names=[tag.name if isinstance(tag, Tag) else tag for tag in self.tags]
            if self.tags
            else [],
            metadata=metadata_dict_to_proto(self.metadata) if self.metadata else [],
        )
        if self.rule_ids:
            proto.rule_ids.CopyFrom(
                CreateReportTemplateRequestRuleIds(
                    rule_ids=[
                        rule._id_or_error if isinstance(rule, Rule) else rule
                        for rule in self.rule_ids
                    ]
                )
            )
        if self.rule_client_keys:
            proto.rule_client_keys.CopyFrom(
                CreateReportTemplateRequestClientKeys(rule_client_keys=self.rule_client_keys)
            )
        return proto


class ReportTemplateUpdate(ReportTemplateCreateUpdateBase, ModelUpdate[ReportTemplateProto]):
    """Model of the ReportTemplate fields that can be updated.

    Note:
        - Updating tags, rule_ids, or rule_client_keys replaces the full list on the
          template. Pass an empty list to clear tags. Rules cannot be cleared; a
          report template must always have at least one rule.
        - Only one of rule_ids or rule_client_keys may be provided.
    """

    name: str | None = None
    description: str | None = None
    is_archived: bool | None = None
    metadata: dict[str, str | float | bool] | None = None
    # Excluded from the generic proto mapping since these map to repeated proto
    # messages; converted in to_proto_with_mask instead.
    tags: list[str | Tag] | None = Field(default=None, exclude=True)
    rule_ids: list[str | Rule] | None = Field(default=None, exclude=True)
    rule_client_keys: list[str] | None = Field(default=None, exclude=True)

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "metadata": MappingHelper(
            proto_attr_path="metadata", update_field="metadata", converter=metadata_dict_to_proto
        ),
    }

    @model_validator(mode="after")
    def _validate_rules_not_cleared(self):
        if self.rule_ids == [] or self.rule_client_keys == []:
            raise ValueError(
                "A report template must have at least one rule; rule_ids or rule_client_keys cannot be empty"
            )
        return self

    def to_proto_with_mask(self) -> tuple[ReportTemplateProto, field_mask_pb2.FieldMask]:
        """Convert to proto with field mask, including the repeated message fields."""
        proto_msg, mask = super().to_proto_with_mask()
        if self.tags is not None:
            proto_msg.tags.extend(
                ReportTemplateTagProto(tag_name=tag.name if isinstance(tag, Tag) else tag)
                for tag in self.tags
            )
            mask.paths.append("tags")
        if self.rule_ids is not None:
            proto_msg.rules.extend(
                ReportTemplateRuleProto(
                    rule_id=rule._id_or_error if isinstance(rule, Rule) else rule
                )
                for rule in self.rule_ids
            )
            mask.paths.append("rules")
        if self.rule_client_keys is not None:
            proto_msg.rules.extend(
                ReportTemplateRuleProto(client_key=client_key)
                for client_key in self.rule_client_keys
            )
            mask.paths.append("rules")
        return proto_msg, mask

    def _get_proto_class(self) -> type[ReportTemplateProto]:
        return ReportTemplateProto

    def _add_resource_id_to_proto(self, proto_msg: ReportTemplateProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.report_template_id = self._resource_id
