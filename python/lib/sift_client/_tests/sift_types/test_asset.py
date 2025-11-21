"""Tests for sift_types.Asset model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import Asset
from sift_client.sift_types.asset import AssetUpdate


class TestAssetUpdate:
    """Unit tests for AssetUpdate model - tests _to_proto_helpers."""

    def test_metadata_converter(self):
        """Test that metadata is converted using _to_proto_helpers."""
        metadata = {"key1": "value1", "key2": 42.5, "key3": True}
        update = AssetUpdate(metadata=metadata)
        update.resource_id = "test_asset_id"

        proto, mask = update.to_proto_with_mask()

        assert proto.asset_id == "test_asset_id"
        # Verify metadata was converted using the helper (returns a list)
        assert len(proto.metadata) == 3

        # Find each metadata value in the list
        metadata_dict = {md.key.name: md for md in proto.metadata}
        assert metadata_dict["key1"].string_value == "value1"
        assert metadata_dict["key2"].number_value == 42.5
        assert metadata_dict["key3"].boolean_value is True
        assert "metadata" in mask.paths


@pytest.fixture
def mock_asset(mock_client):
    """Create a mock Asset instance for testing."""
    asset = Asset(
        proto=MagicMock(),
        id_="test_asset_id",
        name="test_asset",
        organization_id="org1",
        created_date=datetime.now(timezone.utc),
        created_by_user_id="user1",
        modified_date=datetime.now(timezone.utc),
        modified_by_user_id="user1",
        tags=[],
        metadata={},
        is_archived=False,
        archived_date=None,
    )
    asset._apply_client_to_instance(mock_client)
    return asset


class TestAsset:
    """Unit tests for Asset model - tests properties and methods."""

    def test_runs_property_calls_client(self, mock_asset, mock_client):
        """Test that runs property calls client.runs.list_ with correct parameters."""
        mock_client.runs.list_.return_value = []

        # Access runs property
        _ = mock_asset.runs

        # Verify client method was called with correct asset
        mock_client.runs.list_.assert_called_once_with(assets=[mock_asset])

    def test_channels_method_calls_client(self, mock_asset, mock_client):
        """Test that channels() method calls client.channels.list_ with correct parameters."""
        mock_client.channels.list_.return_value = []

        # Call channels method
        _ = mock_asset.channels(limit=5)

        # Verify client method was called with correct parameters
        mock_client.channels.list_.assert_called_once_with(asset=mock_asset, run=None, limit=5)

    def test_channels_method_with_run_filter(self, mock_asset, mock_client):
        """Test that channels() method passes run filter to client."""
        mock_client.channels.list_.return_value = []
        mock_run = MagicMock()

        # Call channels method with run filter
        _ = mock_asset.channels(run=mock_run, limit=10)

        # Verify client method was called with run parameter
        mock_client.channels.list_.assert_called_once_with(asset=mock_asset, run=mock_run, limit=10)

    def test_archive_calls_client_and_updates_self(self, mock_asset, mock_client):
        """Test that archive() calls client.assets.archive and calls _update."""
        archived_asset = MagicMock()
        archived_asset.is_archived = True
        archived_asset.archived_date = datetime.now(timezone.utc)
        mock_client.assets.archive.return_value = archived_asset

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_asset._update = mock_update

            # Call archive
            result = mock_asset.archive(archive_runs=False)

            # Verify client method was called
            mock_client.assets.archive.assert_called_once_with(asset=mock_asset, archive_runs=False)
            # Verify _update was called with the returned asset
            mock_update.assert_called_once_with(archived_asset)
            # Verify it returns self
            assert result is mock_asset

    def test_archive_with_runs(self, mock_asset, mock_client):
        """Test that archive() passes archive_runs parameter correctly."""
        archived_asset = MagicMock()
        mock_client.assets.archive.return_value = archived_asset

        # Mock the _update method
        with MagicMock() as mock_update:
            mock_asset._update = mock_update

            # Call archive with archive_runs=True
            mock_asset.archive(archive_runs=True)

            # Verify client method was called with archive_runs=True
            mock_client.assets.archive.assert_called_once_with(asset=mock_asset, archive_runs=True)

    def test_unarchive_calls_client_and_updates_self(self, mock_asset, mock_client):
        """Test that unarchive() calls client.assets.unarchive and calls _update."""
        unarchived_asset = MagicMock()
        unarchived_asset.is_archived = False
        mock_client.assets.unarchive.return_value = unarchived_asset

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_asset._update = mock_update

            # Call unarchive
            result = mock_asset.unarchive()

            # Verify client method was called
            mock_client.assets.unarchive.assert_called_once_with(asset=mock_asset)
            # Verify _update was called with the returned asset
            mock_update.assert_called_once_with(unarchived_asset)
            # Verify it returns self
            assert result is mock_asset

    def test_update_calls_client_and_updates_self(self, mock_asset, mock_client):
        """Test that update() calls client.assets.update and calls _update."""
        updated_asset = MagicMock()
        updated_asset.tags = ["updated"]
        mock_client.assets.update.return_value = updated_asset

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_asset._update = mock_update

            # Call update
            update = AssetUpdate(tags=["updated"])
            result = mock_asset.update(update)

            # Verify client method was called with correct parameters
            mock_client.assets.update.assert_called_once_with(asset=mock_asset, update=update)
            # Verify _update was called with the returned asset
            mock_update.assert_called_once_with(updated_asset)
            # Verify it returns self
            assert result is mock_asset

    def test_attachments_property_fetches_files(self, mock_asset, mock_client):
        """Test that attachments property fetches files from client.file_attachments API."""
        # Create mock remote files
        mock_remote_file = MagicMock()
        mock_remote_file.entity_id = mock_asset.id_
        mock_remote_files = [mock_remote_file]

        # Mock the file_attachments API
        mock_client.file_attachments.list_.return_value = mock_remote_files

        # Access the attachments property (it's a property, not a method)
        result = mock_asset.attachments

        # Verify file_attachments.list_ was called with correct parameters
        mock_client.file_attachments.list_.assert_called_once_with(
            entities=[mock_asset],
        )

        # Verify result
        assert result == mock_remote_files
