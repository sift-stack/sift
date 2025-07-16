from __future__ import annotations

from datetime import datetime
from typing import TYPE_CHECKING, Any, Type

from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CalculatedChannel as CalculatedChannelProto,
)
from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CalculatedChannelAbstractChannelReference,
)

from sift_client.types._base import BaseType, MappingHelper, ModelUpdate
from sift_client.types.channel import ChannelReference

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class CalculatedChannel(BaseType[CalculatedChannelProto, "CalculatedChannel"]):
    """
    Model of the Sift Calculated Channel.
    """

    id: str
    name: str
    description: str
    expression: str
    channel_references: list[ChannelReference]

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
    def is_archived(self):
        """Whether the calculated channel is archived."""
        return self.archived_date is not None and self.archived_date > datetime(1970, 1, 1)

    @property
    def created_by(self):
        raise NotImplementedError

    @property
    def modified_by(self):
        raise NotImplementedError

    def archive(self) -> CalculatedChannel:
        """Archive the calculated channel."""
        self.client.calculated_channels.archive(calculated_channel=self)
        return self

    def update(
        self,
        update: CalculatedChannelUpdate | dict,
        user_notes: str | None = None,
    ) -> CalculatedChannel:
        """
        Update the Calculated Channel.

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
            id=proto.calculated_channel_id,
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
                proto.archived_date.ToDatetime() if proto.HasField("archived_date") else None
            ),
            version_id=proto.version_id,
            version=proto.version,
            change_message=proto.change_message,
            user_notes=proto.user_notes,
            units=proto.units,
            asset_ids=proto.calculated_channel_configuration.asset_configuration.selection.asset_ids,  # type: ignore
            tag_ids=proto.calculated_channel_configuration.asset_configuration.selection.tag_ids,  # type: ignore
            all_assets=proto.calculated_channel_configuration.asset_configuration.all_assets,
            created_date=proto.created_date.ToDatetime(),
            modified_date=proto.modified_date.ToDatetime(),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            _client=sift_client,
        )


class CalculatedChannelUpdate(ModelUpdate[CalculatedChannelProto]):
    """
    Model of the Calculated Channel Fields that can be updated.
    """

    name: str | None = None
    description: str | None = None
    units: str | None = None
    expression: str | None = None
    # This is named expression_channel_references to match the protobuf field name for easier deserialization.
    expression_channel_references: list[ChannelReference] | None = None
    tag_ids: list[str] | None = None
    archived_date: datetime | None = None

    _to_proto_helpers = {
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
    }

    def __init__(self, **data: Any):
        super().__init__(**data)
        if any([self.expression, self.expression_channel_references]) and not all(
            [self.expression, self.expression_channel_references]
        ):
            raise ValueError("Expression and channel references must be set together")

    def _get_proto_class(self) -> Type[CalculatedChannelProto]:
        return CalculatedChannelProto

    def _add_resource_id_to_proto(self, proto_msg: CalculatedChannelProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.calculated_channel_id = self._resource_id
