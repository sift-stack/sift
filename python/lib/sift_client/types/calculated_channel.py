from __future__ import annotations

from datetime import datetime
from typing import TYPE_CHECKING, List, Type

from pydantic import BaseModel, ConfigDict
from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CalculatedChannel as CalculatedChannelProto,
)

from sift_client.types._base import BaseType, ModelUpdate

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class CalculatedChannel(BaseType[CalculatedChannelProto, "CalculatedChannel"]):
    """
    Model of the Sift Calculated Channel.
    """

    calculated_channel_id: str
    organization_id: str
    client_key: str | None
    archived_date: datetime | None
    version_id: str
    version: int
    name: str
    description: str
    change_message: str
    user_notes: str
    units: str | None
    created_date: datetime
    modified_date: datetime
    calculated_channel_configuration: CalculatedChannelConfiguration
    created_by_user_id: str
    modified_by_user_id: str

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
        updated_calculated_channel = self.client.calculated_channels.archive(
            calculated_channel=self
        )
        self._update(updated_calculated_channel)
        return self

    def update(self, update: CalculatedChannelUpdate | dict) -> CalculatedChannel:
        """
        Update the Calculated Channel.

        Args:
            update: Either a CalculatedChannelUpdate instance or a dictionary of key-value pairs to update.

        """
        updated_calculated_channel = self.client.calculated_channels.update(
            calculated_channel=self, update=update
        )
        self._update(updated_calculated_channel)
        return self

    @classmethod
    def _from_proto(
        cls, proto: CalculatedChannelProto, sift_client: SiftClient | None = None
    ) -> CalculatedChannel:
        return cls(
            calculated_channel_id=proto.calculated_channel_id,
            organization_id=proto.organization_id,
            client_key=proto.client_key if proto.HasField("client_key") else None,
            archived_date=proto.archived_date.ToDatetime()
            if proto.HasField("archived_date")
            else None,
            version_id=proto.version_id,
            version=proto.version,
            name=proto.name,
            description=proto.description,
            change_message=proto.change_message,
            user_notes=proto.user_notes,
            units=proto.units if proto.HasField("units") else None,
            created_date=proto.created_date.ToDatetime(),
            modified_date=proto.modified_date.ToDatetime(),
            calculated_channel_configuration=CalculatedChannelConfiguration._from_proto(
                proto.calculated_channel_configuration
            ),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            _client=sift_client,
        )


class CalculatedChannelConfiguration(BaseModel):
    """
    Configuration for a calculated channel.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    asset_configuration: CalculatedChannelAssetConfiguration
    query_configuration: CalculatedChannelQueryConfiguration

    @classmethod
    def _from_proto(cls, proto) -> CalculatedChannelConfiguration:
        return cls(
            asset_configuration=CalculatedChannelAssetConfiguration._from_proto(
                proto.asset_configuration
            ),
            query_configuration=CalculatedChannelQueryConfiguration._from_proto(
                proto.query_configuration
            ),
        )


class CalculatedChannelAssetConfiguration(BaseModel):
    """
    Asset configuration for a calculated channel.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    all_assets: bool | None = None
    selection: CalculatedChannelAssetSelection | None = None

    @classmethod
    def _from_proto(cls, proto) -> CalculatedChannelAssetConfiguration:
        if proto.HasField("all_assets"):
            return cls(all_assets=proto.all_assets)
        elif proto.HasField("selection"):
            return cls(selection=CalculatedChannelAssetSelection._from_proto(proto.selection))
        else:
            raise ValueError("Asset configuration must have either all_assets or selection")


class CalculatedChannelAssetSelection(BaseModel):
    """
    Asset selection for a calculated channel.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    asset_ids: List[str]
    tag_ids: List[str]

    @classmethod
    def _from_proto(cls, proto) -> CalculatedChannelAssetSelection:
        return cls(
            asset_ids=list(proto.asset_ids),
            tag_ids=list(proto.tag_ids),
        )


class CalculatedChannelQueryConfiguration(BaseModel):
    """
    Query configuration for a calculated channel.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    expression: str
    expression_channel_references: List[CalculatedChannelAbstractChannelReference]

    @classmethod
    def _from_proto(cls, proto) -> CalculatedChannelQueryConfiguration:
        if proto.HasField("sel"):
            sel = proto.sel
            return cls(
                expression=sel.expression,
                expression_channel_references=[
                    CalculatedChannelAbstractChannelReference._from_proto(ref)
                    for ref in sel.expression_channel_references
                ],
            )
        else:
            raise ValueError("Query configuration must have a sel field")


class CalculatedChannelAbstractChannelReference(BaseModel):
    """
    Abstract channel reference for a calculated channel.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    channel_reference: str
    channel_identifier: str

    @classmethod
    def _from_proto(cls, proto) -> CalculatedChannelAbstractChannelReference:
        return cls(
            channel_reference=proto.channel_reference,
            channel_identifier=proto.channel_identifier,
        )


class CalculatedChannelUpdate(ModelUpdate[CalculatedChannelProto]):
    """
    Model of the Calculated Channel Fields that can be updated.
    """

    name: str | None = None
    description: str | None = None
    units: str | None = None
    expression: str | None = None
    asset_names: List[str] | None = None
    tag_names: List[str] | None = None
    all_assets: bool | None = None
    archived: bool | None = None

    def _get_proto_class(self) -> Type[CalculatedChannelProto]:
        return CalculatedChannelProto

    def _add_resource_id_to_proto(self, proto_msg: CalculatedChannelProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.calculated_channel_id = self._resource_id
