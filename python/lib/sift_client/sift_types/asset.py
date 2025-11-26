from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING, ClassVar

from sift.assets.v1.assets_pb2 import Asset as AssetProto

from sift_client.sift_types._base import BaseType, MappingHelper, ModelUpdate
from sift_client.sift_types._mixins.file_attachments import FileAttachmentsMixin
from sift_client.sift_types.tag import Tag
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.channel import Channel
    from sift_client.sift_types.run import Run


class Asset(BaseType[AssetProto, "Asset"], FileAttachmentsMixin):
    """Model of the Sift Asset."""

    # Required fields
    name: str
    organization_id: str
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    tags: list[str | Tag]
    metadata: dict[str, str | float | bool]
    is_archived: bool

    # Optional fields
    archived_date: datetime | None

    @property
    def created_by(self):
        """Get the user that created this asset."""
        raise NotImplementedError

    @property
    def modified_by(self):
        """Get the user that modified this asset."""
        raise NotImplementedError

    @property
    def runs(self) -> list[Run]:
        """Get the runs associated with this asset."""
        return self.client.runs.list_(assets=[self])

    def channels(self, run: Run | str | None = None, limit: int | None = None) -> list[Channel]:
        """Get the channels for this asset."""
        return self.client.channels.list_(asset=self, run=run, limit=limit)

    @property
    def rules(self):
        """Get the rules that apply to this asset."""
        raise NotImplementedError

    @property
    def annotations(self):
        """Get the annotations for this asset."""
        raise NotImplementedError

    def archive(self, *, archive_runs: bool = False) -> Asset:
        """Archive the asset.

        Args:
            archive_runs: If True, archive all Runs associated with the Asset.
        """
        updated_asset = self.client.assets.archive(asset=self, archive_runs=archive_runs)
        self._update(updated_asset)
        return self

    def unarchive(self) -> Asset:
        """Unarchive the asset."""
        updated_asset = self.client.assets.unarchive(asset=self)
        self._update(updated_asset)
        return self

    def update(self, update: AssetUpdate | dict) -> Asset:
        """Update the Asset.

        Args:
            update: Either an AssetUpdate instance or a dictionary of key-value pairs to update.

        """
        updated_asset = self.client.assets.update(asset=self, update=update)
        self._update(updated_asset)
        return self

    @classmethod
    def _from_proto(cls, proto: AssetProto, sift_client: SiftClient | None = None) -> Asset:
        return cls(
            proto=proto,
            id_=proto.asset_id,
            name=proto.name,
            organization_id=proto.organization_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            modified_by_user_id=proto.modified_by_user_id,
            tags=list(proto.tags) if proto.tags else [],
            archived_date=proto.archived_date.ToDatetime(tzinfo=timezone.utc),
            is_archived=proto.is_archived,
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            _client=sift_client,
        )


class AssetUpdate(ModelUpdate[AssetProto]):
    """Model of the Asset Fields that can be updated."""

    tags: list[str | Tag] | None = None
    metadata: dict[str, str | float | bool] | None = None
    is_archived: bool | None = None

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

    def _get_proto_class(self) -> type[AssetProto]:
        return AssetProto

    def _add_resource_id_to_proto(self, proto_msg: AssetProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.asset_id = self._resource_id
