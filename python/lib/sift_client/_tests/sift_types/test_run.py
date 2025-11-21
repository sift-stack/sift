"""Tests for sift_types.Run model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import Run
from sift_client.sift_types.run import RunCreate, RunUpdate


class TestRunCreate:
    """Unit tests for RunCreate model - tests _to_proto_helpers and validators."""

    def test_metadata_converter(self):
        """Test that metadata is converted using _to_proto_helpers."""
        metadata = {"string_key": "value", "number_key": 3.14, "bool_key": True}
        create = RunCreate(name="test_run", metadata=metadata)
        proto = create.to_proto()

        assert len(proto.metadata) == 3

        # Convert list to dict for easier assertion
        metadata_dict = {md.key.name: md for md in proto.metadata}
        assert metadata_dict["string_key"].string_value == "value"
        assert metadata_dict["number_key"].number_value == 3.14
        assert metadata_dict["bool_key"].boolean_value is True

    def test_time_validator_start_before_stop(self):
        """Test time validator accepts start_time before stop_time."""
        start_time = datetime(2024, 1, 1, 12, 0, 0, tzinfo=timezone.utc)
        stop_time = datetime(2024, 1, 1, 13, 0, 0, tzinfo=timezone.utc)

        # Should not raise
        create = RunCreate(name="test_run", start_time=start_time, stop_time=stop_time)
        assert create.start_time == start_time
        assert create.stop_time == stop_time

    def test_time_validator_rejects_start_after_stop(self):
        """Test time validator rejects start_time after stop_time."""
        start_time = datetime(2024, 1, 1, 13, 0, 0, tzinfo=timezone.utc)
        stop_time = datetime(2024, 1, 1, 12, 0, 0, tzinfo=timezone.utc)

        with pytest.raises(ValueError, match="start_time must be before stop_time"):
            RunCreate(name="test_run", start_time=start_time, stop_time=stop_time)

    def test_time_validator_rejects_stop_without_start(self):
        """Test time validator rejects stop_time without start_time."""
        stop_time = datetime(2024, 1, 1, 13, 0, 0, tzinfo=timezone.utc)

        with pytest.raises(
            ValueError, match="start_time must be provided if stop_time is provided"
        ):
            RunCreate(name="test_run", stop_time=stop_time)


class TestRunUpdate:
    """Unit tests for RunUpdate model - tests _to_proto_helpers and validators."""

    def test_metadata_converter(self):
        """Test that metadata is converted using _to_proto_helpers."""
        metadata = {"key1": "value1", "key2": 42.5, "key3": False}
        update = RunUpdate(metadata=metadata)
        update.resource_id = "test_run_id"

        proto, mask = update.to_proto_with_mask()

        assert len(proto.metadata) == 3

        # Convert list to dict for easier assertion
        metadata_dict = {md.key.name: md for md in proto.metadata}
        assert metadata_dict["key1"].string_value == "value1"
        assert metadata_dict["key2"].number_value == 42.5
        assert metadata_dict["key3"].boolean_value is False
        assert "metadata" in mask.paths

    def test_time_validator_start_before_stop(self):
        """Test time validator accepts start_time before stop_time."""
        start_time = datetime(2024, 1, 1, 12, 0, 0, tzinfo=timezone.utc)
        stop_time = datetime(2024, 1, 1, 13, 0, 0, tzinfo=timezone.utc)

        # Should not raise
        update = RunUpdate(start_time=start_time, stop_time=stop_time)
        assert update.start_time == start_time
        assert update.stop_time == stop_time

    def test_time_validator_rejects_start_after_stop(self):
        """Test time validator rejects start_time after stop_time."""
        start_time = datetime(2024, 1, 1, 13, 0, 0, tzinfo=timezone.utc)
        stop_time = datetime(2024, 1, 1, 12, 0, 0, tzinfo=timezone.utc)

        with pytest.raises(ValueError, match="start_time must be before stop_time"):
            RunUpdate(start_time=start_time, stop_time=stop_time)

    def test_time_validator_rejects_stop_without_start(self):
        """Test time validator rejects stop_time without start_time."""
        stop_time = datetime(2024, 1, 1, 13, 0, 0, tzinfo=timezone.utc)

        with pytest.raises(
            ValueError, match="start_time must be provided if stop_time is provided"
        ):
            RunUpdate(stop_time=stop_time)


@pytest.fixture
def mock_run(mock_client):
    """Create a mock Run instance for testing."""
    run = Run(
        proto=MagicMock(),
        id_="test_run_id",
        name="test_run",
        description="test",
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        created_by_user_id="user1",
        modified_by_user_id="user1",
        organization_id="org1",
        metadata={},
        tags=[],
        asset_ids=["asset1", "asset2"],
        is_adhoc=False,
        is_archived=False,
        start_time=None,
        stop_time=None,
        duration=None,
        default_report_id=None,
        client_key=None,
        archived_date=None,
    )
    run._apply_client_to_instance(mock_client)
    return run


class TestRun:
    """Unit tests for Run model - tests properties and methods."""

    def test_assets_property_calls_client(self, mock_run, mock_client):
        """Test that assets property calls client.assets.list_ with correct parameters."""
        mock_client.assets.list_.return_value = []

        # Access assets property
        _ = mock_run.assets

        # Verify client method was called with correct asset_ids
        mock_client.assets.list_.assert_called_once_with(asset_ids=["asset1", "asset2"])

    def test_archive_calls_client_and_updates_self(self, mock_run, mock_client):
        """Test that archive() calls client.runs.archive and calls _update."""
        archived_run = MagicMock()
        archived_run.is_archived = True
        archived_run.archived_date = datetime.now(timezone.utc)
        mock_client.runs.archive.return_value = archived_run

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_run._update = mock_update

            # Call archive
            result = mock_run.archive()

            # Verify client method was called
            mock_client.runs.archive.assert_called_once_with(run=mock_run)
            # Verify _update was called with the returned run
            mock_update.assert_called_once_with(archived_run)
            # Verify it returns self
            assert result is mock_run

    def test_unarchive_calls_client_and_updates_self(self, mock_run, mock_client):
        """Test that unarchive() calls client.runs.unarchive and calls _update."""
        unarchived_run = MagicMock()
        unarchived_run.is_archived = False
        mock_client.runs.unarchive.return_value = unarchived_run

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_run._update = mock_update

            # Call unarchive
            result = mock_run.unarchive()

            # Verify client method was called
            mock_client.runs.unarchive.assert_called_once_with(run=mock_run)
            # Verify _update was called with the returned run
            mock_update.assert_called_once_with(unarchived_run)
            # Verify it returns self
            assert result is mock_run

    def test_update_calls_client_and_updates_self(self, mock_run, mock_client):
        """Test that update() calls client.runs.update and calls _update."""
        updated_run = MagicMock()
        updated_run.description = "Updated description"
        mock_client.runs.update.return_value = updated_run

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_run._update = mock_update

            # Call update
            update = RunUpdate(description="Updated description")
            result = mock_run.update(update)

            # Verify client method was called with correct parameters
            mock_client.runs.update.assert_called_once_with(run=mock_run, update=update)
            # Verify _update was called with the returned run
            mock_update.assert_called_once_with(updated_run)
            # Verify it returns self
            assert result is mock_run

    def test_attachments_property_fetches_files(self, mock_run, mock_client):
        """Test that attachments property fetches files from client.file_attachments API."""
        # Create mock remote files
        mock_remote_file = MagicMock()
        mock_remote_file.entity_id = mock_run.id_
        mock_remote_files = [mock_remote_file]

        # Mock the file_attachments API
        mock_client.file_attachments.list_.return_value = mock_remote_files

        # Access the attachments property (it's a property, not a method)
        result = mock_run.attachments

        # Verify file_attachments.list_ was called with correct parameters
        mock_client.file_attachments.list_.assert_called_once_with(
            entities=[mock_run],
        )

        # Verify result
        assert result == mock_remote_files

    def test_run_stop(self, mock_run, mock_client):
        """Test that stop() calls client.runs.stop and updates self."""
        stopped_run = MagicMock()
        mock_client.runs.get.return_value = stopped_run

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_run._update = mock_update
            result = mock_run.stop()
            mock_client.runs.stop.assert_called_once_with(run=mock_run)
            mock_update.assert_called_once_with(stopped_run)
            assert result is mock_run
