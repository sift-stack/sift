"""
Asset type for the Sift client.

This module provides a user-friendly representation of an Asset,
using standard Python data structures and types instead of the gRPC Asset class.
"""

from __future__ import annotations

from typing import Optional, Type, TYPE_CHECKING, Dict
from datetime import datetime

from sift.assets.v1.assets_pb2 import Asset as ProtoAsset

from sift_client.types.base import ModelUpdate, BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient

class AssetUpdate(ModelUpdate):
    archived_date: Optional[datetime | str] = None

    def _get_proto_class(self) -> Type[ProtoAsset]:
        return ProtoAsset


class Asset(BaseType):
    """
    A user-friendly representation of an Asset.

    This class provides a more Pythonic interface for working with assets,
    using standard Python data structures and types instead of the gRPC Asset class.
    """

    asset_id: str
    name: str
    organization_id: str
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    tags: list[str]
    archived_date: Optional[datetime]

    @property
    def tags(self):
        raise NotImplementedError

    @property
    def metadata(self):
        raise NotImplementedError

    @property
    def runs(self):
        raise NotImplementedError

    @property
    def channels(self):
        raise NotImplementedError

    @property
    def rules(self):
        raise NotImplementedError

    @property
    def annotations(self):
        raise NotImplementedError

    # TODO: update this asset
    def archive(self):
        return self.client.assets.archive(asset=self)

    @classmethod
    def _from_proto(cls, asset: ProtoAsset, sift_client: SiftClient = None) -> Asset:
        return cls(
            asset_id=asset.asset_id,
            name=asset.name,
            organization_id=asset.organization_id,
            created_date=asset.created_date.ToDatetime(),
            created_by_user_id=asset.created_by_user_id,
            modified_date=asset.modified_date.ToDatetime(),
            modified_by_user_id=asset.modified_by_user_id,
            _client=sift_client
        )

    def _get_proto_class(self) -> Type[ProtoAsset]:
        return ProtoAsset

