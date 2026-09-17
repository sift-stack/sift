from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, ClassVar

from pydantic import BaseModel, model_validator
from sift.annotation_logs.v1.annotation_logs_pb2 import (
    AnnotationCommentBodyElement as AnnotationCommentBodyElementProto,
)
from sift.annotation_logs.v1.annotation_logs_pb2 import (
    AnnotationCommentBodyElementType as AnnotationCommentBodyElementTypeProto,
)
from sift.annotation_logs.v1.annotation_logs_pb2 import (
    AnnotationCommentUserMention as AnnotationCommentUserMentionProto,
)
from sift.annotation_logs.v1.annotation_logs_pb2 import (
    AnnotationLogKind as AnnotationLogKindProto,
)
from sift.annotation_logs.v1.annotation_logs_pb2 import (
    AnnotationLogSearchResult as AnnotationLogProto,
)
from sift.annotation_logs.v1.annotation_logs_pb2 import (
    AnnotationLogState as AnnotationLogStateProto,
)
from sift.annotations.v1.annotations_pb2 import (
    Annotation as AnnotationProto,
)
from sift.annotations.v1.annotations_pb2 import (
    AnnotationLinkedCalculatedChannel,
    AnnotationLinkedChannelsBitFieldElement,
    AnnotationLinkedChannelsChannel,
)
from sift.annotations.v1.annotations_pb2 import (
    AnnotationLinkedChannel as AnnotationLinkedChannelProto,
)
from sift.annotations.v1.annotations_pb2 import (
    AnnotationState as AnnotationStateProto,
)
from sift.annotations.v1.annotations_pb2 import (
    AnnotationType as AnnotationTypeProto,
)
from sift.annotations.v1.annotations_pb2 import (
    CreateAnnotationRequest as CreateAnnotationRequestProto,
)

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)
from sift_client.sift_types.tag import Tag
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class AnnotationType(Enum):
    """Enum for annotation types."""

    UNSPECIFIED = AnnotationTypeProto.ANNOTATION_TYPE_UNSPECIFIED  # 0
    DATA_REVIEW = AnnotationTypeProto.ANNOTATION_TYPE_DATA_REVIEW  # 1
    PHASE = AnnotationTypeProto.ANNOTATION_TYPE_PHASE  # 2

    def to_filter_str(self) -> str:
        """Convert to the string used in CEL filters."""
        return f"ANNOTATION_TYPE_{self.name}"


class AnnotationState(Enum):
    """Enum for the review state of a data review annotation."""

    UNSPECIFIED = AnnotationStateProto.ANNOTATION_STATE_UNSPECIFIED  # 0
    OPEN = AnnotationStateProto.ANNOTATION_STATE_OPEN  # 1
    FLAGGED = AnnotationStateProto.ANNOTATION_STATE_FLAGGED  # 2
    RESOLVED = AnnotationStateProto.ANNOTATION_STATE_RESOLVED  # 3

    def to_filter_str(self) -> str:
        """Convert to the string used in CEL filters."""
        return f"ANNOTATION_STATE_{self.name}"


class AnnotationLinkedChannel(BaseModel):
    """A channel an annotation points at.

    Set exactly one of `channel_id` or `calculated_channel_version_id`.
    `bit_field_element` requires `channel_id`.

    Attributes:
        channel_id: The ID of a regular channel.
        bit_field_element: The name of a bit field element on `channel_id`.
        calculated_channel_version_id: The version ID of a calculated channel.
    """

    channel_id: str | None = None
    bit_field_element: str | None = None
    calculated_channel_version_id: str | None = None

    @model_validator(mode="after")
    def _validate_exactly_one(self) -> AnnotationLinkedChannel:
        if self.bit_field_element and not self.channel_id:
            raise ValueError("bit_field_element requires channel_id")
        if bool(self.channel_id) == bool(self.calculated_channel_version_id):
            raise ValueError(
                "AnnotationLinkedChannel requires exactly one of channel_id or "
                "calculated_channel_version_id"
            )
        return self

    @classmethod
    def _from_proto(cls, proto: AnnotationLinkedChannelProto) -> AnnotationLinkedChannel:
        which = proto.WhichOneof("type")
        if which == "bit_field_element":
            return cls(
                channel_id=proto.bit_field_element.channel_id,
                bit_field_element=proto.bit_field_element.bit_field_name,
            )
        if which == "calculated_channel":
            return cls(
                calculated_channel_version_id=proto.calculated_channel.calculated_channel_version_id
            )
        return cls(channel_id=proto.channel.channel_id)

    def _to_proto(self) -> AnnotationLinkedChannelProto:
        if self.calculated_channel_version_id:
            return AnnotationLinkedChannelProto(
                calculated_channel=AnnotationLinkedCalculatedChannel(
                    calculated_channel_version_id=self.calculated_channel_version_id
                )
            )
        if self.bit_field_element:
            return AnnotationLinkedChannelProto(
                bit_field_element=AnnotationLinkedChannelsBitFieldElement(
                    channel_id=self.channel_id or "", bit_field_name=self.bit_field_element
                )
            )
        return AnnotationLinkedChannelProto(
            channel=AnnotationLinkedChannelsChannel(channel_id=self.channel_id or "")
        )


def _linked_channel_to_proto(**kwargs) -> AnnotationLinkedChannelProto:
    """Convert a linked channel dict (from model_dump) into its proto form."""
    return AnnotationLinkedChannel(**kwargs)._to_proto()


class Annotation(BaseType[AnnotationProto, "Annotation"]):
    """Annotation model representing a time range on one or more assets."""

    # Required fields
    name: str
    description: str
    start_time: datetime
    end_time: datetime
    annotation_type: AnnotationType
    organization_id: str
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    tags: list[str]
    asset_ids: list[str]
    linked_channels: list[AnnotationLinkedChannel]
    metadata: dict[str, str | float | bool]
    is_archived: bool
    pending: bool

    # Optional fields
    state: AnnotationState | None
    run_id: str | None
    assigned_to_user_id: str | None
    created_by_rule_condition_version_id: str | None
    legend_config: str | None
    archived_date: datetime | None

    @classmethod
    def _from_proto(
        cls, proto: AnnotationProto, sift_client: SiftClient | None = None
    ) -> Annotation:
        return cls(
            proto=proto,
            id_=proto.annotation_id,
            name=proto.name,
            description=proto.description,
            start_time=proto.start_time.ToDatetime(tzinfo=timezone.utc),
            end_time=proto.end_time.ToDatetime(tzinfo=timezone.utc),
            annotation_type=AnnotationType(proto.annotation_type),
            organization_id=proto.organization_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            tags=list(proto.tags),
            asset_ids=list(proto.asset_ids),
            linked_channels=[AnnotationLinkedChannel._from_proto(c) for c in proto.linked_channels],
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            is_archived=proto.is_archived,
            pending=proto.pending,
            state=AnnotationState(proto.state) if proto.HasField("state") else None,
            run_id=proto.run_id if proto.HasField("run_id") else None,
            assigned_to_user_id=proto.assigned_to_user_id or None,
            created_by_rule_condition_version_id=(
                proto.created_by_rule_condition_version_id
                if proto.HasField("created_by_rule_condition_version_id")
                else None
            ),
            legend_config=proto.legend_config if proto.HasField("legend_config") else None,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            _client=sift_client,
        )

    @property
    def logs(self) -> list[AnnotationLog]:
        """Return this annotation's history."""
        return self.client.annotations.logs.list_(annotation=self._id_or_error)

    def update(self, update: AnnotationUpdate | dict) -> Annotation:
        """Update the Annotation.

        Args:
            update: The update to apply. See AnnotationUpdate for updatable fields.

        Returns:
            The updated annotation.
        """
        updated = self.client.annotations.update(annotation=self, update=update)
        self._update(updated)
        return self

    def archive(self) -> Annotation:
        """Archive the annotation."""
        updated = self.client.annotations.archive(annotation=self)
        self._update(updated)
        return self

    def unarchive(self) -> Annotation:
        """Unarchive the annotation."""
        updated = self.client.annotations.unarchive(annotation=self)
        self._update(updated)
        return self

    def assign(self, user: str) -> Annotation:
        """Assign the annotation to a user for review.

        Args:
            user: The user ID to assign to.

        Returns:
            The updated annotation.
        """
        return self.update({"assigned_to_user_id": user})

    def resolve(self) -> Annotation:
        """Close out the review as resolved."""
        return self._set_state(AnnotationState.RESOLVED)

    def flag(self) -> Annotation:
        """Flag the review as needing attention."""
        return self._set_state(AnnotationState.FLAGGED)

    def reopen(self) -> Annotation:
        """Return the review to the open state."""
        return self._set_state(AnnotationState.OPEN)

    def _set_state(self, state: AnnotationState) -> Annotation:
        """Move to a review state, skipping the call if already there.

        The server rejects a redundant state change with INVALID_ARGUMENT, so calling
        `resolve` twice would fail without this.
        """
        if self.state is state:
            return self
        return self.update({"state": state})

    def comment(self, text: str | list[AnnotationCommentElement]) -> AnnotationLog:
        """Add a comment to the annotation.

        Args:
            text: Plain text, or a list of elements to mix text with user mentions.

        Returns:
            The created AnnotationLog.
        """
        return self.client.annotations.logs.comment(self, text)


class AnnotationBase(ModelCreateUpdateBase):
    """Base class for Annotation create and update models with shared fields and validation."""

    description: str | None = None
    tags: list[str] | list[Tag] | None = None
    linked_channels: list[AnnotationLinkedChannel] | None = None
    state: AnnotationState | None = None
    legend_config: str | None = None
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
        "linked_channels": MappingHelper(
            proto_attr_path="linked_channels",
            update_field="linked_channels",
            converter=_linked_channel_to_proto,  # type: ignore[arg-type]
        ),
    }

    @model_validator(mode="after")
    def _validate_time_range(self):
        start = getattr(self, "start_time", None)
        end = getattr(self, "end_time", None)
        if start is not None and end is not None and start > end:
            raise ValueError("start_time must not be after end_time")
        return self


class AnnotationCreate(AnnotationBase, ModelCreate[CreateAnnotationRequestProto]):
    """Create model for Annotation.

    Note that `assets` takes asset names, not IDs, and `tags` takes tag names.
    """

    name: str
    start_time: datetime
    end_time: datetime
    annotation_type: AnnotationType = AnnotationType.DATA_REVIEW
    assets: list[str] | None = None
    run_id: str | None = None
    assign_to_user_id: str | None = None
    organization_id: str | None = None

    def _get_proto_class(self) -> type[CreateAnnotationRequestProto]:
        return CreateAnnotationRequestProto

    @model_validator(mode="after")
    def _validate_state(self):
        """Phase annotations have no review state; the server rejects one."""
        if self.annotation_type is AnnotationType.PHASE and self.state is not None:
            raise ValueError("state must be unset when annotation_type is PHASE")
        return self

    @model_validator(mode="after")
    def _mark_annotation_type_set(self):
        """Assign here, not as a field default.

        `to_proto` excludes unset fields, so a field default would send UNSPECIFIED.
        """
        self.annotation_type = self.annotation_type
        return self


class AnnotationUpdate(AnnotationBase, ModelUpdate[AnnotationProto]):
    """Update model for Annotation."""

    name: str | None = None
    start_time: datetime | None = None
    end_time: datetime | None = None
    assigned_to_user_id: str | None = None
    is_archived: bool | None = None

    def _get_proto_class(self) -> type[AnnotationProto]:
        return AnnotationProto

    def _add_resource_id_to_proto(self, proto_msg: AnnotationProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.annotation_id = self._resource_id


class AnnotationLogKind(Enum):
    """Enum for the kind of entry in an annotation's history."""

    UNSPECIFIED = AnnotationLogKindProto.ANNOTATION_LOG_KIND_UNSPECIFIED  # 0
    COMMENT = AnnotationLogKindProto.ANNOTATION_LOG_KIND_COMMENT  # 1
    STATE_UPDATE = AnnotationLogKindProto.ANNOTATION_LOG_KIND_STATE_UPDATE  # 2
    ASSIGNED = AnnotationLogKindProto.ANNOTATION_LOG_KIND_ASSIGNED  # 3

    def to_filter_str(self) -> str:
        """Convert to the string used in CEL filters.

        Log kinds filter on the bare name, unlike annotation type and state which
        take the full enum value name.
        """
        return self.name


class AnnotationLogState(Enum):
    """Enum for the state recorded by a state update log."""

    UNSPECIFIED = AnnotationLogStateProto.ANNOTATION_LOG_STATE_UNSPECIFIED  # 0
    CREATED = AnnotationLogStateProto.ANNOTATION_LOG_STATE_CREATED  # 1
    OPEN = AnnotationLogStateProto.ANNOTATION_LOG_STATE_OPEN  # 2
    FLAGGED = AnnotationLogStateProto.ANNOTATION_LOG_STATE_FLAGGED  # 3
    RESOLVED = AnnotationLogStateProto.ANNOTATION_LOG_STATE_RESOLVED  # 4


class AnnotationCommentElement(BaseModel):
    """One element of a comment body: literal text or a user mention.

    Attributes:
        text: The literal text, when this element is text.
        user_id: The mentioned user's ID, when this element is a mention.
        user_email: The mentioned user's email, when this element is a mention.
    """

    text: str | None = None
    user_id: str | None = None
    user_email: str | None = None

    @model_validator(mode="after")
    def _validate_exactly_one(self) -> AnnotationCommentElement:
        if bool(self.text) == bool(self.user_id):
            raise ValueError("AnnotationCommentElement requires exactly one of text or user_id")
        return self

    @classmethod
    def _from_proto(cls, proto: AnnotationCommentBodyElementProto) -> AnnotationCommentElement:
        if (
            proto.type
            == AnnotationCommentBodyElementTypeProto.ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION
        ):
            return cls(user_id=proto.user_mention.user_id, user_email=proto.user_mention.user_email)
        return cls(text=proto.text)

    def _to_proto(self) -> AnnotationCommentBodyElementProto:
        if self.user_id:
            return AnnotationCommentBodyElementProto(
                type=AnnotationCommentBodyElementTypeProto.ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION,
                user_mention=AnnotationCommentUserMentionProto(
                    user_id=self.user_id, user_email=self.user_email or ""
                ),
            )
        return AnnotationCommentBodyElementProto(
            type=AnnotationCommentBodyElementTypeProto.ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT,
            text=self.text or "",
        )


class AnnotationLog(BaseType[AnnotationLogProto, "AnnotationLog"]):
    """One entry in an annotation's history: an assignment, a state change, or a comment."""

    annotation_id: str
    kind: AnnotationLogKind
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    created_by_user_name: str

    # Set on ASSIGNED logs
    assigned_to_user_id: str | None
    assigned_to_user_email: str | None
    # Set on STATE_UPDATE logs
    state: AnnotationLogState | None
    # Set on COMMENT logs
    comment: list[AnnotationCommentElement] | None

    @classmethod
    def _from_proto(
        cls, proto: AnnotationLogProto, sift_client: SiftClient | None = None
    ) -> AnnotationLog:
        which = proto.WhichOneof("properties")
        return cls(
            proto=proto,
            id_=proto.annotation_log_id,
            annotation_id=proto.annotation_id,
            kind=AnnotationLogKind(proto.kind),
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            created_by_user_name=proto.created_by_user_name,
            assigned_to_user_id=(
                proto.assigned.assigned_to_user_id if which == "assigned" else None
            ),
            assigned_to_user_email=(
                proto.assigned.assigned_to_user_email if which == "assigned" else None
            ),
            state=(
                AnnotationLogState(proto.state_update.state) if which == "state_update" else None
            ),
            comment=(
                [AnnotationCommentElement._from_proto(e) for e in proto.comment.body]
                if which == "comment"
                else None
            ),
            _client=sift_client,
        )

    @property
    def text(self) -> str:
        """The comment body as plain text, with mentions rendered as emails."""
        if not self.comment:
            return ""
        return "".join(e.text or f"@{e.user_email or e.user_id}" for e in self.comment)
