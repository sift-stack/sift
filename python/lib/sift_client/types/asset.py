"""
Asset type for the Sift client.

This module provides a user-friendly representation of an Asset,
using standard Python data structures and types instead of the gRPC Asset class.
"""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime

from google.protobuf.timestamp_pb2 import Timestamp
from sift.assets.v1.assets_pb2 import Asset as GrpcAsset


@dataclass
class Asset:
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

    @classmethod
    def from_grpc(cls, asset: GrpcAsset) -> Asset:
        """
        Create an Asset from a gRPC Asset.

        Args:
            asset: The gRPC Asset.

        Returns:
            An Asset.
        """
        return cls(
            asset_id=asset.asset_id,
            name=asset.name,
            organization_id=asset.organization_id,
            created_date=asset.created_date.ToDatetime(),
            created_by_user_id=asset.created_by_user_id,
            modified_date=asset.modified_date.ToDatetime(),
            modified_by_user_id=asset.modified_by_user_id,
            tags=list(asset.tags),
        )

    def to_grpc(self) -> GrpcAsset:
        """
        Convert an Asset to a gRPC Asset.

        Returns:
            A gRPC Asset.
        """
        created_date = Timestamp()
        created_date.FromDatetime(self.created_date)

        modified_date = Timestamp()
        modified_date.FromDatetime(self.modified_date)

        return GrpcAsset(
            asset_id=self.asset_id,
            name=self.name,
            organization_id=self.organization_id,
            created_date=created_date,
            created_by_user_id=self.created_by_user_id,
            modified_date=modified_date,
            modified_by_user_id=self.modified_by_user_id,
            tags=self.tags,
        )

    @property
    def runs(self): ...
