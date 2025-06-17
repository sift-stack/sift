from datetime import datetime, timezone
from unittest import TestCase
from unittest.mock import MagicMock

from sift.assets.v1.assets_pb2 import (
    Asset,
    GetAssetResponse,
    ListAssetsResponse,
    UpdateAssetResponse,
)

from sift_py._internal.metadata import metadata_dict_to_pb
from sift_py.asset.config import AssetConfig
from sift_py.asset.service import AssetService
from sift_py.grpc.transport import SiftChannel


class TestAssetService(TestCase):
    """
    Tests for the AssetService class.

    Note: Most of these tests are useful purely for exercising code paths but importantly do not simulate the backend service so the returns are just the expected values.
    """

    def setUp(self):
        self.channel = MagicMock(spec=SiftChannel)
        self.service = AssetService(self.channel)
        self.asset_service_stub = self.service._asset_service_stub

    def test_get_asset_success(self):
        # Arrange
        asset_id = "test-asset-id"
        timestamp = datetime.now(timezone.utc)
        expected_asset = Asset(
            asset_id=asset_id,
            name="Test Asset",
            organization_id="test-org",
            created_by_user_id="test-user-id",
            modified_by_user_id="test-user-id",
            created_date=timestamp,
            modified_date=timestamp,
            tags=["tag1", "tag2"],
        )
        self.asset_service_stub.GetAsset.return_value = GetAssetResponse(asset=expected_asset)

        # Act
        result = self.service.get_asset(asset_id)

        # Assert
        self.assertIsInstance(result, AssetConfig)
        self.assertEqual(result, AssetConfig.from_asset(expected_asset))
        self.asset_service_stub.GetAsset.assert_called_once()

    def test_get_asset_not_found(self):
        # Arrange
        asset_id = "non-existent-asset"
        self.asset_service_stub.GetAsset.side_effect = Exception("Not found")

        # Act
        result = self.service.get_asset(asset_id)

        # Assert
        self.assertIsNone(result)
        self.asset_service_stub.GetAsset.assert_called_once()

    def test_delete_asset(self):
        # Arrange
        asset_id = "test-asset-id"

        # Act
        self.service.delete_asset(asset_id)

        # Assert
        self.asset_service_stub.DeleteAsset.assert_called_once()

    def test_list_assets_by_names(self):
        # Arrange
        names = ["Asset 1", "Asset 2"]
        timestamp = datetime.now(timezone.utc)
        expected_assets = [
            Asset(
                asset_id=f"asset-{i}",
                name=name,
                organization_id="test-org",
                created_date=timestamp,
                modified_date=timestamp,
                tags=[f"tag{i}"],
            )
            for i, name in enumerate(names)
        ]
        self.asset_service_stub.ListAssets.return_value = ListAssetsResponse(assets=expected_assets)

        # Act
        result = self.service.list_assets(names=names)

        # Assert
        self.assertEqual(len(result), 2)
        for i, config in enumerate(result):
            self.assertIsInstance(config, AssetConfig)
            self.assertEqual(config, AssetConfig.from_asset(expected_assets[i]))
        self.asset_service_stub.ListAssets.assert_called_once()

    def test_list_assets_by_ids(self):
        # Arrange
        ids = ["asset-1", "asset-2"]
        timestamp = datetime.now(timezone.utc)
        expected_assets = [
            Asset(
                asset_id=id,
                name=f"Asset {i}",
                created_date=timestamp,
                modified_date=timestamp,
                organization_id="test-org",
                tags=[f"tag{i}"],
            )
            for i, id in enumerate(ids)
        ]
        self.asset_service_stub.ListAssets.return_value = ListAssetsResponse(assets=expected_assets)

        # Act
        result = self.service.list_assets(ids=ids)

        # Assert
        self.assertEqual(len(result), 2)
        for i, config in enumerate(result):
            self.assertIsInstance(config, AssetConfig)
            self.assertEqual(config, AssetConfig.from_asset(expected_assets[i]))
        self.asset_service_stub.ListAssets.assert_called_once()

    def test_list_assets_empty(self):
        # Act
        result = self.service.list_assets()

        # Assert
        self.assertEqual(result, [])
        self.asset_service_stub.ListAssets.assert_not_called()

    def test_update_asset_full(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        asset = AssetConfig(
            asset_id="test-asset",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
            tags=["tag1", "tag2"],
            metadata={"key1": "value1", "key2": 42, "key3": True},
        )

        expected_asset = Asset(
            asset_id=asset.asset_id,
            name=asset.name,
            organization_id=asset.organization_id,
            created_date=timestamp,
            created_by_user_id=asset.created_by_user_id,
            modified_date=timestamp,
            modified_by_user_id=asset.modified_by_user_id,
            tags=asset.tags,
            metadata=metadata_dict_to_pb(asset.metadata),
        )
        self.asset_service_stub.UpdateAsset.return_value = UpdateAssetResponse(asset=expected_asset)

        # Act
        result = self.service.update_asset(asset)

        # Assert
        self.assertIsInstance(result, AssetConfig)
        self.assertEqual(result, AssetConfig.from_asset(expected_asset))
        self.asset_service_stub.UpdateAsset.assert_called_once()

    def test_update_asset_with_tags(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        asset = AssetConfig(
            asset_id="test-asset",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
            tags=["tag1", "tag2"],
            metadata={"key1": "value1"},
        )

        expected_asset = Asset(
            asset_id=asset.asset_id,
            name=asset.name,
            organization_id=asset.organization_id,
            created_date=timestamp,
            created_by_user_id=asset.created_by_user_id,
            modified_date=timestamp,
            modified_by_user_id=asset.modified_by_user_id,
            tags=asset.tags,
            metadata=metadata_dict_to_pb(asset.metadata),
        )
        self.asset_service_stub.UpdateAsset.return_value = UpdateAssetResponse(asset=expected_asset)

        # Act
        result = self.service.update_asset(asset, update_tags=True, update_metadata=False)

        # Assert
        self.assertIsInstance(result, AssetConfig)
        self.assertEqual(result, AssetConfig.from_asset(expected_asset))
        self.asset_service_stub.UpdateAsset.assert_called_once()

    def test_update_asset_tags_only(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        asset = AssetConfig(
            asset_id="test-asset",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
            tags=[],
            metadata={},
        )

        expected_asset = Asset(
            asset_id=asset.asset_id,
            name=asset.name,
            organization_id=asset.organization_id,
            created_date=timestamp,
            created_by_user_id=asset.created_by_user_id,
            modified_date=timestamp,
            modified_by_user_id=asset.modified_by_user_id,
            tags=[],
            metadata=[],
        )
        self.asset_service_stub.UpdateAsset.return_value = UpdateAssetResponse(asset=expected_asset)

        # Act
        result = self.service.update_asset(asset)

        # Assert
        self.assertIsInstance(result, AssetConfig)
        self.assertEqual(result, AssetConfig.from_asset(expected_asset))
        self.asset_service_stub.UpdateAsset.assert_called_once()

    def test_update_asset_metadata_only(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        asset = AssetConfig(
            asset_id="test-asset",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
            metadata={
                "string_value": "test",
                "number_value": 42,
                "float_value": 3.14,
                "bool_value": True,
                "zero_value": 0,
                "empty_string": "",
            },
        )

        expected_asset = Asset(
            asset_id=asset.asset_id,
            name=asset.name,
            organization_id=asset.organization_id,
            created_date=timestamp,
            created_by_user_id=asset.created_by_user_id,
            modified_date=timestamp,
            modified_by_user_id=asset.modified_by_user_id,
            metadata=metadata_dict_to_pb(asset.metadata),
        )
        self.asset_service_stub.UpdateAsset.return_value = UpdateAssetResponse(asset=expected_asset)

        # Act
        result = self.service.update_asset(asset, update_tags=False, update_metadata=True)

        # Assert
        self.assertIsInstance(result, AssetConfig)
        self.assertEqual(result, AssetConfig.from_asset(expected_asset))
        self.asset_service_stub.UpdateAsset.assert_called_once()
