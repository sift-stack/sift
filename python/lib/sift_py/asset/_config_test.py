from datetime import datetime, timezone
from unittest import TestCase

from sift.assets.v1.assets_pb2 import Asset

from sift_py._internal.metadata import metadata_dict_to_pb
from sift_py.asset.config import AssetConfig


class TestAssetConfig(TestCase):
    def test_from_asset_basic(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        asset = Asset(
            asset_id="test-asset-id",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
        )

        # Act
        config = AssetConfig.from_asset(asset)

        # Assert
        self.assertEqual(config.asset_id, "test-asset-id")
        self.assertEqual(config.name, "Test Asset")
        self.assertEqual(config.organization_id, "test-org")
        self.assertEqual(config.created_by_user_id, "creator-123")
        self.assertEqual(config.modified_by_user_id, "modifier-456")
        self.assertEqual(config.created_date, timestamp)
        self.assertEqual(config.modified_date, timestamp)
        self.assertEqual(config.tags, [])
        self.assertEqual(config.metadata, {})

    def test_from_asset_with_optional_fields(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        metadata = {"string_key": "string_value", "number_key": 42, "bool_key": True}
        asset = Asset(
            asset_id="test-asset-id",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
            tags=["tag1", "tag2"],
            metadata=metadata_dict_to_pb(metadata),
        )

        # Act
        config = AssetConfig.from_asset(asset)

        # Assert
        self.assertEqual(config.asset_id, "test-asset-id")
        self.assertEqual(config.name, "Test Asset")
        self.assertEqual(config.organization_id, "test-org")
        self.assertEqual(config.created_by_user_id, "creator-123")
        self.assertEqual(config.modified_by_user_id, "modifier-456")
        self.assertEqual(config.created_date, timestamp)
        self.assertEqual(config.modified_date, timestamp)
        self.assertEqual(config.tags, ["tag1", "tag2"])
        self.assertEqual(config.metadata, metadata)

    def test_from_asset_with_empty_fields(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        asset = Asset(
            asset_id="test-asset-id",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
            tags=[],  # Empty tags
            metadata=[],  # Empty metadata
        )

        # Act
        config = AssetConfig.from_asset(asset)

        # Assert
        self.assertEqual(config.asset_id, "test-asset-id")
        self.assertEqual(config.name, "Test Asset")
        self.assertEqual(config.organization_id, "test-org")
        self.assertEqual(config.created_by_user_id, "creator-123")
        self.assertEqual(config.modified_by_user_id, "modifier-456")
        self.assertEqual(config.created_date, timestamp)
        self.assertEqual(config.modified_date, timestamp)
        self.assertEqual(config.tags, [])  # Empty list should be preserved
        self.assertEqual(config.metadata, {})  # Empty metadata should be converted to empty dict

    def test_to_asset_basic(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        config = AssetConfig(
            asset_id="test-asset-id",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
        )

        # Act
        asset = config.to_asset()

        # Assert
        self.assertEqual(asset.asset_id, "test-asset-id")
        self.assertEqual(asset.name, "Test Asset")
        self.assertEqual(asset.organization_id, "test-org")
        self.assertEqual(asset.created_by_user_id, "creator-123")
        self.assertEqual(asset.modified_by_user_id, "modifier-456")
        self.assertEqual(asset.created_date.seconds, int(timestamp.timestamp()))
        self.assertEqual(asset.modified_date.seconds, int(timestamp.timestamp()))
        self.assertEqual(list(asset.tags), [])  # Empty list for no tags
        self.assertEqual(list(asset.metadata), [])  # Empty list for no metadata

    def test_to_asset_with_optional_fields(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        metadata = {"string_key": "string_value", "number_key": 42, "bool_key": True}
        config = AssetConfig(
            asset_id="test-asset-id",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
            tags=["tag1", "tag2"],
            metadata=metadata,
        )

        # Act
        asset = config.to_asset()

        # Assert
        self.assertEqual(asset.asset_id, "test-asset-id")
        self.assertEqual(asset.name, "Test Asset")
        self.assertEqual(asset.organization_id, "test-org")
        self.assertEqual(asset.created_by_user_id, "creator-123")
        self.assertEqual(asset.modified_by_user_id, "modifier-456")
        self.assertEqual(asset.created_date.seconds, int(timestamp.timestamp()))
        self.assertEqual(asset.modified_date.seconds, int(timestamp.timestamp()))
        self.assertEqual(list(asset.tags), ["tag1", "tag2"])

        # Verify metadata conversion
        metadata_dict = {md.key.name: md for md in asset.metadata}
        self.assertEqual(metadata_dict["string_key"].string_value, "string_value")
        self.assertEqual(metadata_dict["number_key"].number_value, 42.0)
        self.assertTrue(metadata_dict["bool_key"].boolean_value)

    def test_roundtrip_conversion(self):
        # Arrange
        timestamp = datetime.now(timezone.utc)
        metadata = {"string_key": "string_value", "number_key": 42, "bool_key": True}
        original_asset = Asset(
            asset_id="test-asset-id",
            name="Test Asset",
            organization_id="test-org",
            created_date=timestamp,
            created_by_user_id="creator-123",
            modified_date=timestamp,
            modified_by_user_id="modifier-456",
            tags=["tag1", "tag2"],
            metadata=metadata_dict_to_pb(metadata),
        )

        # Act
        config = AssetConfig.from_asset(original_asset)
        roundtrip_asset = config.to_asset()

        # Assert
        self.assertEqual(roundtrip_asset.asset_id, original_asset.asset_id)
        self.assertEqual(roundtrip_asset.name, original_asset.name)
        self.assertEqual(roundtrip_asset.organization_id, original_asset.organization_id)
        self.assertEqual(roundtrip_asset.created_by_user_id, original_asset.created_by_user_id)
        self.assertEqual(roundtrip_asset.modified_by_user_id, original_asset.modified_by_user_id)
        self.assertEqual(roundtrip_asset.created_date.seconds, original_asset.created_date.seconds)
        self.assertEqual(
            roundtrip_asset.modified_date.seconds, original_asset.modified_date.seconds
        )
        self.assertEqual(list(roundtrip_asset.tags), list(original_asset.tags))

        # Compare metadata
        original_metadata = {md.key.name: md for md in original_asset.metadata}
        roundtrip_metadata = {md.key.name: md for md in roundtrip_asset.metadata}
        self.assertEqual(len(original_metadata), len(roundtrip_metadata))
        for key in original_metadata:
            self.assertEqual(
                original_metadata[key].WhichOneof("value"),
                roundtrip_metadata[key].WhichOneof("value"),
            )
            if original_metadata[key].HasField("string_value"):
                self.assertEqual(
                    original_metadata[key].string_value, roundtrip_metadata[key].string_value
                )
            elif original_metadata[key].HasField("number_value"):
                self.assertEqual(
                    original_metadata[key].number_value, roundtrip_metadata[key].number_value
                )
            elif original_metadata[key].HasField("boolean_value"):
                self.assertEqual(
                    original_metadata[key].boolean_value, roundtrip_metadata[key].boolean_value
                )
