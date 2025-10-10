from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING, ClassVar

from pydantic import model_validator
from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CalculatedChannel as CalculatedChannelProto,
)
from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CalculatedChannelAbstractChannelReference,
    CreateCalculatedChannelRequest,
)

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)
from sift_client.sift_types.channel import ChannelReference
from sift_client.util.metadata import metadata_dict_to_proto

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class CalculatedChannel(BaseType[CalculatedChannelProto, "CalculatedChannel"]):
    """Model of the Sift Calculated Channel."""

    name: str
    description: str
    expression: str
    channel_references: list[ChannelReference]
    is_archived: bool

    units: str | None
    asset_ids: list[str] | None
    tag_ids: list[str] | None
    all_assets: bool | None
    organization_id: str | None
    client_key: str | None
    archived_date: datetime | None
    version_id: str | None
    version: int | None
    change_message: str | None
    user_notes: str | None
    created_date: datetime | None
    modified_date: datetime | None
    created_by_user_id: str | None
    modified_by_user_id: str | None

    @property
    def created_by(self):
        """Get the user that created this calculated channel."""
        raise NotImplementedError

    @property
    def modified_by(self):
        """Get the user that modified this calculated channel."""
        raise NotImplementedError

    def archive(self) -> CalculatedChannel:
        """Archive the calculated channel."""
        updated_calculated_channel = self.client.calculated_channels.archive(
            calculated_channel=self
        )
        self._update(updated_calculated_channel)
        return self

    def unarchive(self) -> CalculatedChannel:
        """Unarchive the calculated channel."""
        updated_calculated_channel = self.client.calculated_channels.unarchive(
            calculated_channel=self
        )
        self._update(updated_calculated_channel)
        return self

    def update(
        self,
        update: CalculatedChannelUpdate | dict,
        user_notes: str | None = None,
    ) -> CalculatedChannel:
        """Update the Calculated Channel.

        Args:
            update: The update to apply to the calculated channel. See CalculatedChannelUpdate for more updatable fields.
            user_notes: The user notes to apply to the calculated channel.

        Returns:
            The updated calculated channel.
        """
        updated_calculated_channel = self.client.calculated_channels.update(
            calculated_channel=self, update=update, user_notes=user_notes
        )
        self._update(updated_calculated_channel)
        return self

    @classmethod
    def _from_proto(
        cls, proto: CalculatedChannelProto, sift_client: SiftClient | None = None
    ) -> CalculatedChannel:
        return cls(
            proto=proto,
            id_=proto.calculated_channel_id,
            name=proto.name,
            description=proto.description,
            expression=proto.calculated_channel_configuration.query_configuration.sel.expression,
            channel_references=[
                ChannelReference(
                    channel_reference=ref_proto.channel_reference,
                    channel_identifier=ref_proto.channel_identifier,
                )
                for ref_proto in proto.calculated_channel_configuration.query_configuration.sel.expression_channel_references
            ],
            organization_id=proto.organization_id,
            client_key=proto.client_key,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            is_archived=proto.is_archived,
            version_id=proto.version_id,
            version=proto.version,
            change_message=proto.change_message,
            user_notes=proto.user_notes,
            units=proto.units,
            asset_ids=proto.calculated_channel_configuration.asset_configuration.selection.asset_ids,  # type: ignore
            tag_ids=proto.calculated_channel_configuration.asset_configuration.selection.tag_ids,  # type: ignore
            all_assets=proto.calculated_channel_configuration.asset_configuration.all_assets,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            _client=sift_client,
        )


class CalculatedChannelBase(ModelCreateUpdateBase):
    """Base class for CalculatedChannel create and update models with shared fields and validation."""

    description: str | None = None
    units: str | None = None

    expression: str | None = None
    # This is named expression_channel_references to match the protobuf field name for easier deserialization.
    expression_channel_references: list[ChannelReference] | None = None

    # Scoping of the calculated channel.
    tag_ids: list[str] | None = None
    asset_ids: list[str] | None = None
    all_assets: bool | None = None

    metadata: dict[str, str | float | bool] | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "expression": MappingHelper(
            proto_attr_path="calculated_channel_configuration.query_configuration.sel.expression",
            update_field="query_configuration",
        ),
        "expression_channel_references": MappingHelper(
            proto_attr_path="calculated_channel_configuration.query_configuration.sel.expression_channel_references",
            update_field="query_configuration",
            converter=CalculatedChannelAbstractChannelReference,
        ),
        "tag_ids": MappingHelper(
            proto_attr_path="calculated_channel_configuration.asset_configuration.selection.tag_ids",
            update_field="asset_configuration",
        ),
        "asset_ids": MappingHelper(
            proto_attr_path="calculated_channel_configuration.asset_configuration.selection.asset_ids",
            update_field="asset_configuration",
        ),
        "all_assets": MappingHelper(
            proto_attr_path="calculated_channel_configuration.asset_configuration.all_assets",
            update_field="asset_configuration",
        ),
        "metadata": MappingHelper(
            proto_attr_path="metadata",
            update_field="metadata",
            converter=metadata_dict_to_proto,
        ),
    }

    @model_validator(mode="after")
    def _validate_asset_configuration(self):
        """Validate that either all_assets is True or at least one of tag_ids or asset_ids is provided, but not both."""
        if self.all_assets is not None and self.all_assets and (self.asset_ids or self.tag_ids):
            raise ValueError("Cannot specify both all_assets=True and asset_ids/tag_ids")
        return self

    @model_validator(mode="after")
    def _validate_expression_and_channel_references(self):
        """Validate that expression and expression_channel_references are set together."""
        if any([self.expression, self.expression_channel_references]) and not all(
            [self.expression, self.expression_channel_references]
        ):
            raise ValueError("Expression and channel references must be set together")
        return self


class CalculatedChannelCreate(CalculatedChannelBase, ModelCreate[CreateCalculatedChannelRequest]):
    """Create model for a Calculated Channel."""

    name: str
    user_notes: str | None = None
    client_key: str | None = None

    def _get_proto_class(self) -> type[CreateCalculatedChannelRequest]:
        return CreateCalculatedChannelRequest


class CalculatedChannelUpdate(CalculatedChannelBase, ModelUpdate[CalculatedChannelProto]):
    """Update model for a Calculated Channel."""

    name: str | None = None
    is_archived: bool | None = None

    def _get_proto_class(self) -> type[CalculatedChannelProto]:
        return CalculatedChannelProto

    def _add_resource_id_to_proto(self, proto_msg: CalculatedChannelProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.calculated_channel_id = self._resource_id
