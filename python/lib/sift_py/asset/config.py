from __future__ import annotations

from datetime import datetime, timezone
from typing import Dict, List, Optional, Union

from pydantic.dataclasses import dataclass
from sift.assets.v1.assets_pb2 import Asset

from sift_py._internal.metadata import metadata_dict_to_pb, metadata_pb_to_dict
from sift_py._internal.time import to_timestamp_pb


@dataclass
class AssetConfig:
    """
    Thin wrapper class for an Asset that can be created from an Asset protobuf object.
    This provides a more Python-friendly interface than the generated protobuf object.
    """

    asset_id: str
    name: str
    organization_id: str
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    tags: Optional[List[str]] = None
    metadata: Optional[Dict[str, Union[str, float, bool]]] = None

    @classmethod
    def from_asset(cls, asset: Asset) -> AssetConfig:
        """
        Creates an AssetConfig from an Asset protobuf object.

        Args:
            asset: The Asset protobuf object to convert.

        Returns:
            An AssetConfig instance with the data from the Asset.
        """
        return cls(
            asset_id=asset.asset_id,
            name=asset.name,
            organization_id=asset.organization_id,
            created_date=datetime.fromtimestamp(
                asset.created_date.ToMicroseconds() / 1000000, tz=timezone.utc
            ),
            created_by_user_id=asset.created_by_user_id,
            modified_date=datetime.fromtimestamp(
                asset.modified_date.ToMicroseconds() / 1000000, tz=timezone.utc
            ),
            modified_by_user_id=asset.modified_by_user_id,
            tags=list(asset.tags),
            metadata=metadata_pb_to_dict(list(asset.metadata)),
        )

    def to_asset(self) -> Asset:
        """
        Converts this AssetConfig to an Asset protobuf object.

        Returns:
            An Asset protobuf object with the data from this config.
        """
        return Asset(
            asset_id=self.asset_id,
            name=self.name,
            organization_id=self.organization_id,
            created_date=to_timestamp_pb(self.created_date),
            created_by_user_id=self.created_by_user_id,
            modified_date=to_timestamp_pb(self.modified_date),
            modified_by_user_id=self.modified_by_user_id,
            tags=self.tags if self.tags else [],
            metadata=metadata_dict_to_pb(self.metadata) if self.metadata else [],
        )
