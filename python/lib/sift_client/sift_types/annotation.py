from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, ClassVar, Literal, cast

from pydantic import BaseModel, ConfigDict, Field, field_validator, model_validator
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
from sift_client.sift_types.asset import Asset  # noqa: TC001
from sift_client.sift_types.calculated_channel import CalculatedChannel
from sift_client.sift_types.channel import Channel  # noqa: TC001
from sift_client.sift_types.run import Run
from sift_client.sift_types.tag import Tag
from sift_client.sift_types.user import User
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
    """Review state of a data review annotation, named as Sift shows it.

    The proto spells these UNSPECIFIED, OPEN, FLAGGED, and RESOLVED.
    """

    UNSPECIFIED = AnnotationStateProto.ANNOTATION_STATE_UNSPECIFIED  # 0
    OPEN = AnnotationStateProto.ANNOTATION_STATE_OPEN  # 1
    FAILED = AnnotationStateProto.ANNOTATION_STATE_FLAGGED  # 2
    ACCEPTED = AnnotationStateProto.ANNOTATION_STATE_RESOLVED  # 3

    def to_filter_str(self) -> str:
        """Convert to the string used in CEL filters, which uses the proto spelling."""
        proto_name = {"FAILED": "FLAGGED", "ACCEPTED": "RESOLVED"}.get(self.name, self.name)
        return f"ANNOTATION_STATE_{proto_name}"


def _linked_channel_to_proto(channel: Channel | CalculatedChannel) -> AnnotationLinkedChannelProto:
    """Wrap a Channel or CalculatedChannel in the proto's oneof."""
    if isinstance(channel, CalculatedChannel):
        return AnnotationLinkedChannelProto(
            calculated_channel=AnnotationLinkedCalculatedChannel(
                calculated_channel_version_id=channel.version_id or ""
            )
        )
    return AnnotationLinkedChannelProto(
        channel=AnnotationLinkedChannelsChannel(channel_id=channel._id_or_error)
    )


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
    linked_channel_ids: list[str]
    linked_calculated_channel_version_ids: list[str]
    metadata: dict[str, str | float | bool]
    is_archived: bool
    pending: bool

    # Optional fields
    state: AnnotationState | None
    run_id: str | None
    assigned_to_user_id: str | None
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
            linked_channel_ids=[
                c.bit_field_element.channel_id
                if c.WhichOneof("type") == "bit_field_element"
                else c.channel.channel_id
                for c in proto.linked_channels
                if c.WhichOneof("type") != "calculated_channel"
            ],
            linked_calculated_channel_version_ids=[
                c.calculated_channel.calculated_channel_version_id
                for c in proto.linked_channels
                if c.WhichOneof("type") == "calculated_channel"
            ],
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            is_archived=proto.is_archived,
            pending=proto.pending,
            state=AnnotationState(proto.state) if proto.HasField("state") else None,
            run_id=proto.run_id if proto.HasField("run_id") else None,
            assigned_to_user_id=proto.assigned_to_user_id or None,
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

    @property
    def assets(self) -> list[Asset]:
        """Fetch the Assets this annotation is on."""
        return self.client.assets.list_(asset_ids=self.asset_ids) if self.asset_ids else []

    @property
    def linked_channels(self) -> list[Channel | CalculatedChannel]:
        """Fetch the Channels and CalculatedChannels this annotation is drawn on."""
        linked: list[Channel | CalculatedChannel] = []
        if self.linked_channel_ids:
            linked.extend(self.client.channels.list_(channel_ids=self.linked_channel_ids))
        if self.linked_calculated_channel_version_ids:
            quoted = ", ".join(f'"{v}"' for v in self.linked_calculated_channel_version_ids)
            linked.extend(
                self.client.calculated_channels.list_versions(
                    filter_query=f"calculated_channel_version_id in [{quoted}]"
                )
            )
        return linked

    @property
    def run(self) -> Run | None:
        """Fetch the Run this annotation belongs to."""
        return self.client.runs.get(run_id=self.run_id) if self.run_id else None

    @property
    def created_by(self) -> User:
        """Fetch the User that created this annotation."""
        return self.client.users.get(user_id=self.created_by_user_id)

    @property
    def modified_by(self) -> User:
        """Fetch the User that last modified this annotation."""
        return self.client.users.get(user_id=self.modified_by_user_id)

    @property
    def assigned_to(self) -> User | None:
        """Fetch the User this annotation is assigned to."""
        return (
            self.client.users.get(user_id=self.assigned_to_user_id)
            if self.assigned_to_user_id
            else None
        )

    def assign_to_user(self, user: str | User) -> Annotation:
        """Assign the annotation to a user for review.

        Args:
            user: The User or user ID to assign to.

        Returns:
            The updated annotation.
        """
        updated = self.client.annotations.assign_to_user(annotation=self, user=user)
        self._update(updated)
        return self

    def set_open(self) -> Annotation:
        """Set the review state to Open."""
        updated = self.client.annotations.set_open(annotation=self)
        self._update(updated)
        return self

    def set_failed(self) -> Annotation:
        """Set the review state to Failed."""
        updated = self.client.annotations.set_failed(annotation=self)
        self._update(updated)
        return self

    def set_accepted(self) -> Annotation:
        """Set the review state to Accepted."""
        updated = self.client.annotations.set_accepted(annotation=self)
        self._update(updated)
        return self

    def add_comment(self, text: str | list[AnnotationCommentElement]) -> AnnotationLog:
        """Add a comment to the annotation.

        Args:
            text: Plain text, or a list of elements to mix text with user mentions.

        Returns:
            The created AnnotationLog.
        """
        return self.client.annotations.logs.add_comment(self, text)


class AnnotationBase(ModelCreateUpdateBase):
    """Base class for Annotation create and update models with shared fields and validation."""

    description: str | None = None
    tags: list[str] | list[Tag] | None = None
    linked_channels: (
        list[Channel] | list[CalculatedChannel] | list[Channel | CalculatedChannel] | None
    ) = Field(default=None, exclude=True)
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

    def linked_channels_to_proto(self) -> list[AnnotationLinkedChannelProto]:
        """Build the proto form of `linked_channels`."""
        return [_linked_channel_to_proto(c) for c in self.linked_channels or []]

    @model_validator(mode="after")
    def _validate_time_range(self):
        start = getattr(self, "start_time", None)
        end = getattr(self, "end_time", None)
        if start is not None and end is not None and start > end:
            raise ValueError("start_time must not be after end_time")
        return self


class AnnotationCreateBase(AnnotationBase, ModelCreate[CreateAnnotationRequestProto]):
    """Shared fields for creating an annotation.

    `assets` takes Assets or asset IDs, and `tags` takes Tags or tag names. Omit
    `assets` and the asset is taken from `linked_channels`.
    """

    name: str
    start_time: datetime
    end_time: datetime
    assets: list[str | Asset] | None = None
    """Assets or asset IDs. Resolved to names, which is what the proto takes."""
    run: str | Run | None = None
    organization_id: str | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        **AnnotationBase._to_proto_helpers,
        "run": MappingHelper(proto_attr_path="run_id", update_field="run_id"),
    }

    def _get_proto_class(self) -> type[CreateAnnotationRequestProto]:
        return CreateAnnotationRequestProto

    @field_validator("run", mode="after")
    @classmethod
    def _run_to_id(cls, value):
        return value._id_or_error if isinstance(value, Run) else value

    @model_validator(mode="after")
    def _mark_annotation_type_set(self):
        """Assign here, not as a field default.

        `to_proto` excludes unset fields, so a field default would send UNSPECIFIED.
        """
        self.annotation_type = self.annotation_type
        return self


class AnnotationCreate(AnnotationCreateBase):
    """Create a data review annotation, which carries a review state and an assignee."""

    annotation_type: Literal[AnnotationType.DATA_REVIEW] = AnnotationType.DATA_REVIEW
    state: AnnotationState | None = None
    assign_to_user: str | User | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        **AnnotationBase._to_proto_helpers,
        "assign_to_user": MappingHelper(
            proto_attr_path="assign_to_user_id",
            update_field="assign_to_user_id",
        ),
    }

    @field_validator("assign_to_user", mode="after")
    @classmethod
    def _user_to_id(cls, value):
        return value._id_or_error if isinstance(value, User) else value


class PhaseCreate(AnnotationCreateBase):
    """Create a phase annotation, which labels a segment of a run and has no state.

    `extra="forbid"` so that passing `state` raises rather than being dropped.
    """

    model_config = ConfigDict(extra="forbid")

    annotation_type: Literal[AnnotationType.PHASE] = AnnotationType.PHASE


class AnnotationUpdate(AnnotationBase, ModelUpdate[AnnotationProto]):
    """Update model for Annotation."""

    name: str | None = None
    start_time: datetime | None = None
    end_time: datetime | None = None
    assigned_to_user_id: str | None = None
    state: AnnotationState | None = None
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
    FAILED = AnnotationLogStateProto.ANNOTATION_LOG_STATE_FLAGGED  # 3
    ACCEPTED = AnnotationLogStateProto.ANNOTATION_LOG_STATE_RESOLVED  # 4


class AnnotationCommentElement(BaseModel):
    """One element of a comment body: literal text or a user mention.

    Attributes:
        text: The literal text, when this element is text.
        user_id: The mentioned user's ID, when this element is a mention.
    """

    text: str | None = None
    user_id: str | User | None = None

    @field_validator("user_id", mode="after")
    @classmethod
    def _user_to_id(cls, value):
        return value._id_or_error if isinstance(value, User) else value

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
            return cls(user_id=proto.user_mention.user_id)
        return cls(text=proto.text)

    def _to_proto(self) -> AnnotationCommentBodyElementProto:
        if self.user_id:
            return AnnotationCommentBodyElementProto(
                type=AnnotationCommentBodyElementTypeProto.ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION,
                user_mention=AnnotationCommentUserMentionProto(user_id=cast("str", self.user_id)),
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

    # Set on ASSIGNED logs
    assigned_to_user_id: str | None
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
            assigned_to_user_id=(
                proto.assigned.assigned_to_user_id if which == "assigned" else None
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
    def created_by(self) -> User:
        """Fetch the User that created this log entry."""
        return self.client.users.get(user_id=self.created_by_user_id)

    @property
    def assigned_to(self) -> User | None:
        """Fetch the User this log entry assigned the annotation to."""
        if not self.assigned_to_user_id:
            return None
        return self.client.users.get(user_id=self.assigned_to_user_id)

    @property
    def mentioned_users(self) -> list[User]:
        """Fetch the Users mentioned in this comment."""
        return [
            self.client.users.get(user_id=cast("str", e.user_id))
            for e in self.comment or []
            if e.user_id
        ]

    @property
    def text(self) -> str:
        """The comment body as plain text, with mentions rendered as user IDs."""
        if not self.comment:
            return ""
        return "".join(e.text or f"@{e.user_id}" for e in self.comment)
