from unittest import TestCase
from unittest.mock import MagicMock, patch
from datetime import datetime, timezone

from sift.assets.v1.assets_pb2 import (
    Asset,
    GetAssetResponse,
    ListAssetsResponse,
    UpdateAssetResponse,
)
from sift.metadata.v1.metadata_pb2 import MetadataKey, MetadataValue

from sift_py.asset.service import AssetService
from sift_py.grpc.transport import SiftChannel
from sift_py._internal.metadata import wrap_metadata


class TestAssetService(TestCase):
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
        self.assertEqual(result, expected_asset)
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
        self.assertEqual(result, expected_assets)
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
        self.assertEqual(result, expected_assets)
        self.asset_service_stub.ListAssets.assert_called_once()

    def test_list_assets_empty(self):
        # Act
        result = self.service.list_assets()

        # Assert
        self.assertEqual(result, [])
        self.asset_service_stub.ListAssets.assert_not_called()

    def test_update_asset(self):
        timestamp = datetime.now(timezone.utc)
        # Arrange
        asset = Asset(
            asset_id="test-asset",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            modified_date=timestamp,
        )
        new_tags = ["new-tag1", "new-tag2"]
        new_metadata = {"key1": "value1", "key2": 42, "key3": True}

        expected_asset = Asset(
            asset_id=asset.asset_id,
            name=asset.name,
            organization_id=asset.organization_id,
            created_date=timestamp,
            modified_date=timestamp,
            tags=new_tags,
        )
        self.asset_service_stub.UpdateAsset.return_value = UpdateAssetResponse(asset=expected_asset)

        # Act
        result = self.service.update_asset(
            asset=asset,
            tags=new_tags,
            metadata=new_metadata,
        )

        # Assert
        self.assertEqual(result, expected_asset)
        self.asset_service_stub.UpdateAsset.assert_called_once()


class TestMetadata(TestCase):
    def test_wrap_metadata(self):
        # Arrange
        metadata = {
            "string_key": "string_value",
            "number_key": 42,
            "float_key": 3.14,
            "bool_key": True,
        }

        # Act
        result = wrap_metadata(metadata)

        # Assert
        self.assertEqual(len(result), 4)

        # Check string metadata
        string_metadata = next(m for m in result if m.key.name == "string_key")
        self.assertEqual(string_metadata.string_value, "string_value")

        # Check number metadata
        number_metadata = next(m for m in result if m.key.name == "number_key")
        self.assertEqual(number_metadata.number_value, 42.0)

        # Check float metadata
        float_metadata = next(m for m in result if m.key.name == "float_key")
        self.assertEqual(float_metadata.number_value, 3.14)

        # Check boolean metadata
        bool_metadata = next(m for m in result if m.key.name == "bool_key")
        self.assertTrue(bool_metadata.boolean_value)

    def test_wrap_metadata_invalid_type(self):
        # Arrange
        metadata = {"invalid_key": [1, 2, 3]}  # List is not a supported type

        # Act & Assert
        with self.assertRaises(ValueError) as context:
            wrap_metadata(metadata)

        self.assertIn("Unsupported metadata value type", str(context.exception))
