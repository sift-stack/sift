from __future__ import annotations

from datetime import datetime
from enum import Enum
from typing import TYPE_CHECKING, List, Optional, Type

from pydantic import BaseModel, ConfigDict
from sift.annotations.v1.annotations_pb2 import AnnotationType
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
    Rule as RuleProto,
)
from sift.rules.v1.rules_pb2 import (
    RuleAction as RuleActionProto,
)

from sift_client.types._base import BaseType, ModelUpdate
from sift_client.types.channel import ChannelReference

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class Rule(BaseType[RuleProto, "Rule"]):
    """
    Model of the Sift Rule.
    """

    name: str
    description: str
    is_enabled: bool = True
    expression: str | None = None
    channel_references: List[ChannelReference] | None = None
    action: RuleAction | None = None
    asset_ids: List[str] | None = None
    tag_ids: List[str] | None = None
    contextual_channels: List[str] | None = None
    client_key: str | None = None

    # Fields from proto
    rule_id: str | None = None
    created_date: datetime | None = None
    modified_date: datetime | None = None
    created_by_user_id: str | None = None
    modified_by_user_id: str | None = None
    organization_id: str | None = None
    rule_version: RuleVersion | None = None
    deleted_date: datetime | None = None
    is_external: bool | None = None

    @property
    def is_deleted(self) -> bool:
        """Whether the rule is deleted."""
        return self.deleted_date is not None and self.deleted_date > datetime(1970, 1, 1)

    def update(self, update: RuleUpdate | dict, version_notes: str | None = None) -> Rule:
        """
        Update the Rule.

        Args:
            update: Either a RuleUpdate instance or a dictionary of key-value pairs to update.
        """
        updated_rule = self.client.rules.update(
            rule=self, update=update, version_notes=version_notes
        )
        self._update(updated_rule)
        return self

    def delete(self) -> None:
        """Delete the rule."""
        self.client.rules.delete(rule=self)

    @classmethod
    def _from_proto(cls, proto: RuleProto, sift_client: SiftClient | None = None) -> Rule:
        expression = (
            proto.conditions[0].expression.calculated_channel.expression
            if proto.conditions
            else None
        )
        return cls(
            rule_id=proto.rule_id,
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
            is_enabled=proto.is_enabled,
            created_date=proto.created_date.ToDatetime(),
            modified_date=proto.modified_date.ToDatetime(),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            organization_id=proto.organization_id,
            rule_version=(
                RuleVersion._from_proto(proto.rule_version) if proto.rule_version else None
            ),
            client_key=proto.client_key if proto.client_key else None,
            asset_ids=proto.asset_configuration.asset_ids,  # type: ignore
            tag_ids=proto.asset_configuration.tag_ids,  # type: ignore
            contextual_channels=[c.name for c in proto.contextual_channels.channels],
            deleted_date=proto.deleted_date.ToDatetime() if proto.deleted_date else None,
            is_external=proto.is_external,
            _client=sift_client,
        )


class RuleUpdate(ModelUpdate[RuleProto]):
    """
    Model of the Rule fields that can be updated.
    """

    name: str | None = None
    description: str | None = None
    expression: str | None = None
    channel_references: List[ChannelReference] | None = None
    action: RuleAction | None = None
    asset_ids: List[str] | None = None
    tag_ids: List[str] | None = None
    organization_id: str | None = None
    client_key: str | None = None
    contextual_channels: List[str] | None = None
    is_external: bool | None = None

    def _get_proto_class(self) -> Type[RuleProto]:
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
    def from_str(cls, val: str) -> Optional["RuleActionType"]:
        if isinstance(val, str) and val.startswith("ACTION_KIND_"):
            for item in cls:
                if "ACTION_KIND_" + item.name == val:
                    return item

        return cls(int(val))


class RuleAnnotationType(Enum):
    """Enum for rule annotation types."""

    UNSPECIFIED = AnnotationType.ANNOTATION_TYPE_UNSPECIFIED  # 0
    DATA_REVIEW = AnnotationType.ANNOTATION_TYPE_DATA_REVIEW  # 1
    PHASE = AnnotationType.ANNOTATION_TYPE_PHASE  # 2

    @classmethod
    def from_str(cls, val: str) -> Optional["RuleAnnotationType"]:
        if isinstance(val, str) and val.startswith("ANNOTATION_TYPE_"):
            for item in cls:
                if "ANNOTATION_TYPE_" + item.name == val:
                    return item

        return cls(int(val))


class RuleAction(BaseModel):
    """
    Model of a Rule Action.
    """

    model_config = ConfigDict(frozen=True, arbitrary_types_allowed=True)

    action_type: RuleActionType
    condition_id: str | None = None
    created_date: datetime | None = None
    modified_date: datetime | None = None
    created_by_user_id: str | None = None
    modified_by_user_id: str | None = None
    version_id: str | None = None
    annotation_type: RuleAnnotationType | None = None
    tags: List[str] | None = None
    assignee: str | None = None

    @classmethod
    def annotation(
        cls, annotation_type: RuleAnnotationType, tags: List[str] = [], assignee: str | None = None
    ) -> RuleAction:
        """Create an annotation action.

        Args:
            annotation_type: Type of annotation to create.
            assignee: User ID to assign the annotation to.
            tags: List of tag IDs to add to the annotation.
        """
        return cls(
            action_type=RuleActionType.ANNOTATION,
            annotation_type=annotation_type,
            tags=tags,
            assignee=assignee,
        )

    @classmethod
    def _from_proto(cls, proto: RuleActionProto) -> RuleAction:
        action_type = RuleActionType(proto.action_type)
        return cls(
            condition_id=proto.rule_condition_id,
            created_date=proto.created_date.ToDatetime(),
            modified_date=proto.modified_date.ToDatetime(),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            version_id=proto.rule_action_version_id,
            tags=(
                list(proto.configuration.annotation.tag_ids)
                if proto.configuration.annotation.tag_ids
                else None
            ),
            assignee=(
                proto.configuration.annotation.assigned_to_user_id
                if proto.configuration.annotation.assigned_to_user_id
                else None
            ),
            action_type=action_type,
            annotation_type=RuleAnnotationType.from_str(
                proto.configuration.annotation.annotation_type  # type: ignore
            )
            if action_type == RuleActionType.ANNOTATION
            else None,
        )

    def to_update_proto(self) -> UpdateActionRequest:
        return UpdateActionRequest(
            action_type=self.action_type.value,
            configuration=RuleActionConfiguration(
                annotation=(
                    AnnotationActionConfiguration(
                        assigned_to_user_id=self.assignee,
                        tag_ids=self.tags,
                        annotation_type=self.annotation_type.value,  # type: ignore
                    )
                    if self.action_type == RuleActionType.ANNOTATION
                    else None
                ),
            ),
        )


class RuleVersion(BaseModel):
    """
    Model of a Rule Version.
    """

    model_config = ConfigDict(frozen=True)

    rule_id: str
    rule_version_id: str
    version: str
    created_date: datetime
    created_by_user_id: str
    version_notes: str
    generated_change_message: str
    deleted_date: datetime | None = None

    @classmethod
    def _from_proto(cls, proto) -> RuleVersion:
        return cls(
            rule_id=proto.rule_id,
            rule_version_id=proto.rule_version_id,
            version=proto.version,
            created_date=proto.created_date.ToDatetime(),
            created_by_user_id=proto.created_by_user_id,
            version_notes=proto.version_notes,
            generated_change_message=proto.generated_change_message,
            deleted_date=proto.deleted_date.ToDatetime() if proto.deleted_date else None,
        )
