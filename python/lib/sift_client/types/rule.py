from __future__ import annotations

from datetime import datetime
from enum import Enum
from typing import TYPE_CHECKING, List, Optional, Type

from pydantic import BaseModel, ConfigDict
from sift.annotations.v1.annotations_pb2 import AnnotationType
from sift.rules.v1.rules_pb2 import (
    ActionKind,
    CalculatedChannelConfig,
    ContextualChannels,
    RuleAssetConfiguration,
)
from sift.rules.v1.rules_pb2 import (
    RuleCondition as RuleConditionProto,
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
from sift_client.types.channel import ChannelConfig

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class Rule(BaseType[RuleProto, "Rule"]):
    """
    Model of the Sift Rule.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str
    description: str
    is_enabled: bool = True
    conditions: List[RuleCondition]  # TODO: Is this just versions?

    # Fields for creation
    expression: str | None = None
    action: RuleAction | None = None
    channel_references: List[ExpressionChannelReference] | None = None
    rule_client_key: str | None = None
    asset_names: List[str] | None = None
    contextual_channels: List[str] | None = None

    # Fields from proto
    rule_id: str | None = None
    created_date: datetime | None = None
    modified_date: datetime | None = None
    created_by_user_id: str | None = None
    modified_by_user_id: str | None = None
    _organization_id: str | None = None
    rule_version: RuleVersion | None = None
    client_key: str | None = None
    asset_configuration: RuleAssetConfiguration | None = None
    contextual_channels_proto: ContextualChannels | None = None
    deleted_date: datetime | None = None
    is_external: bool | None = None

    @property
    def is_deleted(self) -> bool:
        """Whether the rule is deleted."""
        return self.deleted_date is not None and self.deleted_date > datetime(1970, 1, 1)


    def update(self, update: RuleUpdate | dict) -> Rule:
        """
        Update the Rule.

        Args:
            update: Either a RuleUpdate instance or a dictionary of key-value pairs to update.
        """
        updated_rule = self.client.rules.update(rule=self, update=update)
        self._update(updated_rule)
        return self

    def delete(self) -> None:
        """Delete the rule."""
        self.client.rules.delete(rule=self)

    @classmethod
    def _from_proto(cls, proto: RuleProto, sift_client: SiftClient | None = None) -> Rule:
        return cls(
            rule_id=proto.rule_id,
            name=proto.name,
            description=proto.description,
            is_enabled=proto.is_enabled,
            created_date=proto.created_date.ToDatetime(),
            modified_date=proto.modified_date.ToDatetime(),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            organization_id=proto.organization_id,
            conditions=[RuleCondition._from_proto(c) for c in proto.conditions],
            rule_version=(
                RuleVersion._from_proto(proto.rule_version) if proto.rule_version else None
            ),
            client_key=proto.client_key if proto.client_key else None,
            asset_configuration=proto.asset_configuration,
            contextual_channels_proto=proto.contextual_channels,
            deleted_date=proto.deleted_date.ToDatetime() if proto.deleted_date else None,
            is_external=proto.is_external,
            _client=sift_client,
        )


class RuleUpdate(ModelUpdate[RuleProto]):
    """
    Model of the Rule fields that can be updated.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str | None = None
    description: str | None = None
    asset_id: str | None = None
    is_enabled: bool | None = None
    organization_id: str | None = None
    version_notes: str | None = None
    client_key: str | None = None
    asset_configuration: RuleAssetConfiguration | None = None
    contextual_channels: ContextualChannels | None = None
    is_external: bool | None = None

    def _get_proto_class(self) -> Type[RuleProto]:
        return RuleProto

    def _add_resource_id_to_proto(self, proto_msg: RuleProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.rule_id = self._resource_id


class RuleCondition(BaseModel):
    """
    Model of a Rule Condition.
    """

    model_config = ConfigDict(frozen=True, arbitrary_types_allowed=True)

    expression: str
    channel_references: List[ExpressionChannelReference]
    actions: List[RuleAction]

    rule_condition_id: str | None = None
    rule_id: str | None = None
    created_date: datetime | None = None
    modified_date: datetime | None = None
    created_by_user_id: str | None = None
    modified_by_user_id: str | None = None
    rule_condition_version_id: str | None = None

    @classmethod
    def _from_proto(cls, proto: RuleConditionProto) -> RuleCondition:
        return cls(
            expression=proto.expression,
            channel_references=[
                ExpressionChannelReference._from_proto(c)
                for c in proto.calculated_channel.channel_references
            ],
            actions=[RuleAction._from_proto(a) for a in proto.actions],
            rule_condition_id=proto.rule_condition_id,
            rule_id=proto.rule_id,
            created_date=proto.created_date.ToDatetime(),
            modified_date=proto.modified_date.ToDatetime(),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            rule_condition_version_id=proto.rule_condition_version_id,
        )


class RuleActionType(Enum):
    """Enum for rule action kinds."""

    UNSPECIFIED = ActionKind.ACTION_KIND_UNSPECIFIED  # 0
    NOTIFICATION = ActionKind.NOTIFICATION  # 1
    ANNOTATION = ActionKind.ANNOTATION  # 2
    WEBHOOK = ActionKind.WEBHOOK  # 3

    @classmethod
    def from_str(cls, val: str) -> Optional["RuleActionType"]:
        for item in cls:
            if "ACTION_KIND_" + item.name == val:
                return item
        return cls.UNSPECIFIED


class RuleAnnotationType(Enum):
    """Enum for rule annotation types."""

    UNSPECIFIED = AnnotationType.ANNOTATION_TYPE_UNSPECIFIED  # 0
    DATA_REVIEW = AnnotationType.ANNOTATION_TYPE_DATA_REVIEW  # 1
    PHASE = AnnotationType.ANNOTATION_TYPE_PHASE  # 2

    @classmethod
    def from_str(cls, val: str) -> Optional["RuleAnnotationType"]:
        for item in cls:
            if "ANNOTATION_TYPE_" + item.name == val:
                return item
        return cls.UNSPECIFIED


class RuleAction(BaseModel):
    """
    Model of a Rule Action.
    """

    model_config = ConfigDict(frozen=True, arbitrary_types_allowed=True)

    action_type: RuleActionType
    rule_action_id: str | None = None
    rule_condition_id: str | None = None
    created_date: datetime | None = None
    modified_date: datetime | None = None
    created_by_user_id: str | None = None
    modified_by_user_id: str | None = None
    rule_action_version_id: str | None = None
    annotation_type: RuleAnnotationType | None = None
    notification_recipients: List[str] | None = None  # List of user IDs to notify
    tags: List[str] | None = None
    assignee: str | None = None

    # TODO: move to top level Rule
    @classmethod
    def notification(cls, notify_recipients: List[str]) -> RuleAction:
        """Create a notification action.

        Args:
            notify_recipients: List of user IDs to notify.
        """
        return cls(
            action_type=RuleActionType.NOTIFICATION,
            notification_recipients=notify_recipients,
        )

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
        return cls(
            rule_action_id=proto.rule_action_id,
            rule_condition_id=proto.rule_condition_id,
            created_date=proto.created_date.ToDatetime(),
            modified_date=proto.modified_date.ToDatetime(),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            rule_action_version_id=proto.rule_action_version_id,
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
            action_type=RuleActionType.from_str(proto.action_type),
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


class ExpressionChannelReference(BaseModel):
    """
    `channel_identifier`: The channel identifier (e.g. '$1') used in the expression.
    `channel_name`: The channel name.
    """

    channel_identifier: str
    channel_name: str | None = None
    channel_config: ChannelConfig | None = None

    @classmethod
    def _from_proto(cls, proto: ChannelReferencesEntry) -> ExpressionChannelReference:
        return cls(
            channel_identifier=proto.key,
            channel_name=proto.value.name,
        )
