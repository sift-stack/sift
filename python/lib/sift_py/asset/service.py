from __future__ import annotations

from datetime import datetime
from typing import Any, Dict, List, Optional, Union, cast

from google.protobuf.field_mask_pb2 import FieldMask
from sift.assets.v1.assets_pb2 import (
    Asset,
    DeleteAssetRequest,
    GetAssetRequest,
    GetAssetResponse,
    UpdateAssetRequest,
    UpdateAssetResponse,
)
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub

from sift_py._internal.metadata import wrap_metadata
from sift_py._internal.time import to_timestamp_pb
from sift_py.asset._internal.shared import list_assets_impl
from sift_py.grpc.transport import SiftChannel


class AssetService:
    """
    A service for managing assets. Allows for creating, updating, and retrieving assets in the Sift API.
    """

    _asset_service_stub: AssetServiceStub

    def __init__(self, channel: SiftChannel):
        self._asset_service_stub = AssetServiceStub(channel)

    def get_asset(self, asset_id: str) -> Optional[Asset]:
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
            return res.asset or None
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
    ) -> List[Asset]:
        """
        Lists assets in an organization.

        Args:
            names: Optional list of names to filter by.
            ids: Optional list of IDs to filter by.

        Returns:
            A list of assets matching the criteria.
        """
        return list_assets_impl(self._asset_service_stub, names, ids)

    def update_asset(
        self,
        asset: Asset,
        tags: Optional[List[str]] = None,
        metadata: Optional[Dict[str, Union[str, float, bool]]] = None,
    ) -> Asset:
        """
        Updates an existing asset.

        Args:
            asset: The asset to update.
            tags: Optional new list of tags for the asset.
            metadata: Optional new metadata for the asset.

        Returns:
            The updated Asset.
        """
        wrapped_metadata = wrap_metadata(metadata) if metadata else None

        update_map: Dict[str, Any] = {}
        new_tags = asset.tags
        new_metadata = asset.metadata
        if tags:
            update_map["tags"] = tags
            new_tags = tags
        if wrapped_metadata:
            update_map["metadata"] = wrapped_metadata
            new_metadata = wrapped_metadata

        updated_asset = Asset(
            asset_id=asset.asset_id,
            name=asset.name,
            organization_id=asset.organization_id,
            created_date=asset.created_date,
            created_by_user_id=asset.created_by_user_id,
            modified_date=to_timestamp_pb(
                datetime.now()
            ),  # This shouldn't need to be passed since they're set by backend but w/e.
            modified_by_user_id=asset.modified_by_user_id,
            tags=new_tags,
            metadata=new_metadata,
        )

        update_mask = FieldMask(paths=list(update_map.keys()))
        req = UpdateAssetRequest(
            asset=updated_asset,
            update_mask=update_mask,
        )
        res = cast(UpdateAssetResponse, self._asset_service_stub.UpdateAsset(req))
        return res.asset
