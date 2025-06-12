from __future__ import annotations

from typing import List, Optional, cast

from google.protobuf.field_mask_pb2 import FieldMask
from sift.assets.v1.assets_pb2 import (
    DeleteAssetRequest,
    GetAssetRequest,
    GetAssetResponse,
    UpdateAssetRequest,
    UpdateAssetResponse,
)
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub

from sift_py.asset._internal.shared import list_assets_impl
from sift_py.asset.config import AssetConfig
from sift_py.grpc.transport import SiftChannel


class AssetService:
    """
    A service for managing assets. Allows for creating, updating, and retrieving assets in the Sift API.
    """

    _asset_service_stub: AssetServiceStub

    def __init__(self, channel: SiftChannel):
        self._asset_service_stub = AssetServiceStub(channel)

    def get_asset(self, asset_id: str) -> Optional[AssetConfig]:
        """
        Retrieves an asset by its ID.

        Args:
            asset_id: The ID of the asset to retrieve.

        Returns:
            The Asset if found, None otherwise.
        """
        req = GetAssetRequest(asset_id=asset_id)
        try:
            res = cast(GetAssetResponse, self._asset_service_stub.GetAsset(req))
            return AssetConfig.from_asset(res.asset) if res.asset else None
        except:
            return None

    def delete_asset(self, asset_id: str) -> None:
        """
        Deletes an asset by its ID.
        """
        req = DeleteAssetRequest(asset_id=asset_id)
        self._asset_service_stub.DeleteAsset(req)

    def list_assets(
        self,
        names: Optional[List[str]] = None,
        ids: Optional[List[str]] = None,
    ) -> List[AssetConfig]:
        """
        Lists assets in an organization.

        Args:
            names: Optional list of names to filter by.
            ids: Optional list of IDs to filter by.

        Returns:
            A list of assets matching the criteria.
        """
        return [
            AssetConfig.from_asset(asset)
            for asset in list_assets_impl(self._asset_service_stub, names, ids)
        ]

    def update_asset(
        self, asset: AssetConfig, update_tags: bool = True, update_metadata: bool = True
    ) -> AssetConfig:
        """
        Updates an existing asset.

        Args:
            asset: The asset to update.
            update_tags: Whether to update the tags.
            update_metadata: Whether to update the metadata.

        Returns:
            The updated AssetConfig.
        """
        update_mask = []
        if update_tags:
            update_mask.append("tags")
        if update_metadata:
            update_mask.append("metadata")
        req = UpdateAssetRequest(
            asset=asset.to_asset(),
            update_mask=FieldMask(paths=update_mask),
        )
        res = cast(UpdateAssetResponse, self._asset_service_stub.UpdateAsset(req))

        return AssetConfig.from_asset(res.asset)
