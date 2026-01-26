from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING
from uuid import UUID

from sift.rules.v1.rules_pb2 import (
    ActionKind,
    AnnotationActionConfiguration,
    CalculatedChannelConfig,
    RuleActionConfiguration,
    UpdateActionRequest,
)

# Extract nested class.
ChannelReferencesEntry = CalculatedChannelConfig.ChannelReferencesEntry
del CalculatedChannelConfig

from sift.rules.v1.rules_pb2 import (
    CreateRuleRequest,
)
from sift.rules.v1.rules_pb2 import (
    Rule as RuleProto,
)
from sift.rules.v1.rules_pb2 import (
    RuleAction as RuleActionProto,
)
from sift.rules.v1.rules_pb2 import (
    RuleVersion as RuleVersionProto,
)

from sift_client.sift_types._base import BaseType, ModelCreate, ModelCreateUpdateBase, ModelUpdate
from sift_client.sift_types.channel import ChannelReference
from sift_client.sift_types.tag import Tag

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset


class Rule(BaseType[RuleProto, "Rule"]):
    """Model of the Sift Rule."""

    # Required fields
    name: str
    description: str
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    organization_id: str
    is_archived: bool
    is_external: bool
    evaluate_on_live_data: bool
    current_version_id: str

    # Optional fields
    expression: str | None
    channel_references: list[ChannelReference] | None
    action: RuleAction | None
    asset_ids: list[str] | None
    asset_tag_ids: list[str] | None
    contextual_channels: list[str] | None
    client_key: str | None
    rule_version: RuleVersion | None
    archived_date: datetime | None

    @property
    def assets(self) -> list[Asset]:
        """Get the assets that this rule applies to."""
        return self.client.assets.list_(asset_ids=self.asset_ids, tags=self.asset_tag_ids)

    @property
    def organization(self):
        """Get the organization that this rule belongs to."""
        raise NotImplementedError("Organization is not supported yet.")

    @property
    def created_by(self):
        """Get the user that created this rule."""
        raise NotImplementedError("Created by is not supported yet.")

    @property
    def modified_by(self):
        """Get the user that modified this rule."""
        raise NotImplementedError("Modified by is not supported yet.")

    @property
    def tags(self) -> list[Tag]:
        """Get the tags that this rule applies to."""
        return self.client.tags.list_(tag_ids=self.asset_tag_ids)

    def update(self, update: RuleUpdate | dict, version_notes: str | None = None) -> Rule:
        """Update the Rule.

        Args:
            update: Either a RuleUpdate instance or a dictionary of key-value pairs to update.
            version_notes: Notes associated with the change.
        """
        updated_rule = self.client.rules.update(
            rule=self, update=update, version_notes=version_notes
        )
        self._update(updated_rule)
        return self

    def archive(self) -> Rule:
        """Archive the rule."""
        updated_rule = self.client.rules.archive(rule=self)
        self._update(updated_rule)
        return self

    def unarchive(self) -> Rule:
        """Unarchive the rule."""
        updated_rule = self.client.rules.unarchive(rule=self)
        self._update(updated_rule)
        return self

    @classmethod
    def _from_proto(cls, proto: RuleProto, sift_client: SiftClient | None = None) -> Rule:
        expression = (
            proto.conditions[0].expression.calculated_channel.expression
            if proto.conditions
            else None
        )
        return cls(
            proto=proto,
            id_=proto.rule_id,
            name=proto.name,
            description=proto.description,
            expression=expression,
            channel_references=[
                ChannelReference(channel_reference=ref, channel_identifier=c.name)
                for ref, c in proto.conditions[
                    0
                ].expression.calculated_channel.channel_references.items()
            ],
            action=RuleAction._from_proto(proto.conditions[0].actions[0]),
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            organization_id=proto.organization_id,
            rule_version=(
                RuleVersion._from_proto(proto.rule_version) if proto.rule_version else None
            ),
            client_key=proto.client_key if proto.client_key else None,
            asset_ids=proto.asset_configuration.asset_ids,  # type: ignore
            asset_tag_ids=proto.asset_configuration.tag_ids,  # type: ignore
            contextual_channels=[c.name for c in proto.contextual_channels.channels],
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc) if proto.archived_date else None
            ),
            is_archived=proto.is_archived,
            is_external=proto.is_external,
            evaluate_on_live_data=proto.is_live_evaluation_enabled,
            current_version_id=proto.current_version_id,
            _client=sift_client,
        )


class RuleCreateUpdateBase(ModelCreateUpdateBase):
    """Base class for Rule create and update models with shared fields and validation."""

    organization_id: str | None = None
    client_key: str | None = None
    asset_ids: list[str] | None = None
    asset_tag_ids: list[str] | None = None
    contextual_channels: list[str] | None = None
    is_external: bool = False
    evaluate_on_live_data: bool = False


class RuleCreate(RuleCreateUpdateBase, ModelCreate[CreateRuleRequest]):
    """Model for creating a new Rule.

    Note:
    - asset_ids applies this rule to those assets.
    - asset_tag_ids applies this rule to assets with those tags.
    """

    name: str
    description: str
    expression: str
    channel_references: list[ChannelReference]
    action: RuleAction

    def _get_proto_class(self) -> type[CreateRuleRequest]:
        return CreateRuleRequest


class RuleUpdate(RuleCreateUpdateBase, ModelUpdate[RuleProto]):
    """Model of the Rule fields that can be updated.

    Note:
        - assets applies this rule to those assets.
        - asset_tags applies this rule to assets with those tags.
        - contextual_channels are shown by UI to give context when viewing an annotation, but are not part of rule evaluation.
    """

    name: str | None = None
    description: str | None = None
    expression: str | None = None
    channel_references: list[ChannelReference] | None = None
    action: RuleAction | None = None
    is_archived: bool | None = None

    def _get_proto_class(self) -> type[RuleProto]:
        return RuleProto

    def _add_resource_id_to_proto(self, proto_msg: RuleProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.rule_id = self._resource_id


class RuleActionType(Enum):
    """Enum for rule action kinds."""

    UNSPECIFIED = ActionKind.ACTION_KIND_UNSPECIFIED  # 0
    ANNOTATION = ActionKind.ANNOTATION  # 1
    WEBHOOK = ActionKind.WEBHOOK  # 2

    @classmethod
    def from_str(cls, val: str) -> RuleActionType | None:
        """Convert string representation to RuleActionType.

        Args:
            val: String representation of RuleActionType.

        Returns:
            RuleActionType if conversion is successful, None otherwise.
        """
        if isinstance(val, str) and val.startswith("ACTION_KIND_"):
            for item in cls:
                if "ACTION_KIND_" + item.name == val:
                    return item

        return cls(int(val))


class RuleAnnotationType(Enum):
    """Enum for rule annotation types."""

    UNSPECIFIED = 0
    DATA_REVIEW = 1
    PHASE = 2

    @classmethod
    def from_str(cls, val: str) -> RuleAnnotationType | None:
        """Convert string representation to RuleAnnotationType.

        Args:
            val: String representation of RuleAnnotationType.

        Returns:
            RuleAnnotationType if conversion is successful, None otherwise.
        """
        if isinstance(val, str) and val.startswith("ANNOTATION_TYPE_"):
            for item in cls:
                if "ANNOTATION_TYPE_" + item.name == val:
                    return item

        return cls(int(val))


class RuleAction(BaseType[RuleActionProto, "RuleAction"]):
    """Model of a Rule Action."""

    action_type: RuleActionType
    condition_id: str | None = None
    created_date: datetime | None = None
    modified_date: datetime | None = None
    created_by_user_id: str | None = None
    modified_by_user_id: str | None = None
    version_id: str | None = None
    annotation_type: RuleAnnotationType | None = None
    tags_ids: list[str] | None = None
    default_assignee_user: str | None = None

    @classmethod
    def annotation(
        cls,
        annotation_type: RuleAnnotationType,
        tags: list[str | Tag],
        default_assignee_user: str | None = None,
    ) -> RuleAction:
        """Create an annotation action.

        Args:
            annotation_type: Type of annotation to create.
            default_assignee_user: User ID to assign the annotation to.
            tags: List of tags or tag IDs to add to the annotation.
        """
        validated_tags = (
            [str(UUID(tag.id_)) if isinstance(tag, Tag) else str(UUID(tag)) for tag in tags]
            if tags
            else None
        )

        return cls(
            action_type=RuleActionType.ANNOTATION,
            annotation_type=annotation_type,
            tags_ids=validated_tags,
            default_assignee_user=default_assignee_user,
        )

    @classmethod
    def _from_proto(
        cls, proto: RuleActionProto, sift_client: SiftClient | None = None
    ) -> RuleAction:
        action_type = RuleActionType(proto.action_type)
        return cls(
            proto=proto,
            condition_id=proto.rule_condition_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            version_id=proto.rule_action_version_id,
            tags_ids=(
                list(proto.configuration.annotation.tag_ids)
                if proto.configuration.annotation.tag_ids
                else None
            ),
            default_assignee_user=(
                proto.configuration.annotation.assigned_to_user_id
                if proto.configuration.annotation.assigned_to_user_id
                else None
            ),
            action_type=action_type,
            annotation_type=(
                RuleAnnotationType.from_str(
                    proto.configuration.annotation.annotation_type  # type: ignore
                )
                if action_type == RuleActionType.ANNOTATION
                else None
            ),
            _client=sift_client,
        )

    def _to_update_request(self) -> UpdateActionRequest:
        tags_ids = [str(UUID(tag)) for tag in self.tags_ids] if self.tags_ids else None
        return UpdateActionRequest(
            action_type=self.action_type.value,
            configuration=RuleActionConfiguration(
                annotation=(
                    AnnotationActionConfiguration(
                        assigned_to_user_id=self.default_assignee_user,
                        tag_ids=tags_ids,
                        annotation_type=self.annotation_type.value,  # type: ignore
                    )
                    if self.action_type == RuleActionType.ANNOTATION
                    else None
                ),
            ),
        )

    @property
    def tags(self) -> list[Tag]:
        """Get the tags that this rule action applies to."""
        return self.client.tags.list_(tag_ids=self.tags_ids) if self.tags_ids else []


class RuleVersion(BaseType[RuleVersionProto, "RuleVersion"]):
    """Model of a Rule Version."""

    rule_id: str
    rule_version_id: str
    version: str
    created_date: datetime
    created_by_user_id: str
    version_notes: str
    generated_change_message: str
    archived_date: datetime | None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: RuleVersionProto, sift_client: SiftClient | None = None
    ) -> RuleVersion:
        return cls(
            proto=proto,
            rule_id=proto.rule_id,
            rule_version_id=proto.rule_version_id,
            version=proto.version,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            version_notes=proto.version_notes,
            generated_change_message=proto.generated_change_message,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc) if proto.archived_date else None
            ),
            is_archived=proto.is_archived,
            _client=sift_client,
        )
