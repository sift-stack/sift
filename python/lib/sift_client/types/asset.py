from __future__ import annotations

from datetime import datetime
from typing import TYPE_CHECKING, Type

from sift.assets.v1.assets_pb2 import Asset as AssetProto

from sift_client.types._base import BaseType, MappingHelper, ModelUpdate
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class Asset(BaseType[AssetProto, "Asset"]):
    """
    Model of the Sift Asset.
    """

    id: str
    name: str
    organization_id: str
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    tags: list[str]
    metadata: dict[str, str | float | bool]
    archived_date: datetime | None

    @property
    def is_archived(self):
        """Whether the asset is archived."""
        # TODO: clean up this logic when gRPC returns a null.
        return self.archived_date is not None and self.archived_date > datetime(1970, 1, 1)

    @property
    def created_by(self):
        raise NotImplementedError

    @property
    def modified_by(self):
        raise NotImplementedError

    def runs(self, limit: int | None = None):
        return self.client.runs.list(asset_id=self.id, limit=limit)

    @property
    def rules(self):
        raise NotImplementedError

    @property
    def annotations(self):
        raise NotImplementedError

    def archive(self, *, archive_runs: bool = False) -> Asset:
        """Archive the asset.

        Args:
            archive_runs: If True, archive all Runs associated with the Asset.
        """
        updated_asset = self.client.assets.archive(asset=self, archive_runs=archive_runs)
        self._update(updated_asset)
        return self

    def update(self, update: AssetUpdate | dict) -> Asset:
        """
        Update the Asset.

        Args:
            update: Either an AssetUpdate instance or a dictionary of key-value pairs to update.

        """
        updated_asset = self.client.assets.update(asset=self, update=update)
        self._update(updated_asset)
        return self

    @classmethod
    def _from_proto(cls, proto: AssetProto, sift_client: SiftClient | None = None) -> Asset:
        return cls(
            id=proto.asset_id,
            name=proto.name,
            organization_id=proto.organization_id,
            created_date=proto.created_date.ToDatetime(),
            created_by_user_id=proto.created_by_user_id,
            modified_date=proto.modified_date.ToDatetime(),
            modified_by_user_id=proto.modified_by_user_id,
            tags=list(proto.tags) if proto.tags else [],
            archived_date=proto.archived_date.ToDatetime(),
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            _client=sift_client,
        )


class AssetUpdate(ModelUpdate[AssetProto]):
    """
    Model of the Asset Fields that can be updated.
    """

    tags: list[str] | None = None
    archived_date: datetime | str | None = None
    metadata: dict[str, str | float | bool] | None = None

    _to_proto_helpers = {
        "metadata": MappingHelper(
            proto_attr_path="metadata", update_field="metadata", converter=metadata_dict_to_proto
        ),
    }

    def _get_proto_class(self) -> Type[AssetProto]:
        return AssetProto

    def _add_resource_id_to_proto(self, proto_msg: AssetProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.asset_id = self._resource_id
