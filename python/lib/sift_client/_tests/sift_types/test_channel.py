"""Tests for sift_types.Channel model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import Channel
from sift_client.sift_types.channel import ChannelDataType


@pytest.fixture
def mock_channel(mock_client):
    """Create a mock Channel instance for testing."""
    channel = Channel(
        proto=MagicMock(),
        id_="test_channel_id",
        name="test_channel",
        data_type=ChannelDataType.DOUBLE,
        description="test description",
        unit="m/s",
        bit_field_elements=[],
        enum_types={},
        asset_id="test_asset_id",
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        created_by_user_id="user1",
        modified_by_user_id="user1",
    )
    channel._apply_client_to_instance(mock_client)
    return channel


class TestChannel:
    """Unit tests for Channel model - tests properties and methods."""

    def test_asset_property_calls_client(self, mock_channel, mock_client):
        """Test that asset property calls client.assets.get with correct parameters."""
        mock_asset = MagicMock()
        mock_client.assets.get.return_value = mock_asset

        # Access asset property
        result = mock_channel.asset

        # Verify client method was called with correct asset_id
        mock_client.assets.get.assert_called_once_with(asset_id="test_asset_id")
        assert result is mock_asset

    def test_runs_property_calls_asset_runs(self, mock_channel, mock_client):
        """Test that runs property calls asset.runs."""
        mock_asset = MagicMock()
        mock_runs = [MagicMock(), MagicMock()]
        mock_asset.runs = mock_runs
        mock_client.assets.get.return_value = mock_asset

        # Access runs property
        result = mock_channel.runs

        # Verify it returns the asset's runs
        assert result == mock_runs

    def test_data_method_calls_get_data(self, mock_channel, mock_client):
        """Test that data() method calls client.channels.get_data with correct parameters."""
        mock_data = {"test_channel": MagicMock()}
        mock_client.channels.get_data.return_value = mock_data

        # Call data method
        result = mock_channel.data(
            run_id="run123",
            start_time=datetime(2024, 1, 1, tzinfo=timezone.utc),
            end_time=datetime(2024, 1, 2, tzinfo=timezone.utc),
            limit=100,
        )

        # Verify client method was called with correct parameters
        mock_client.channels.get_data.assert_called_once_with(
            channels=[mock_channel],
            run="run123",
            start_time=datetime(2024, 1, 1, tzinfo=timezone.utc),
            end_time=datetime(2024, 1, 2, tzinfo=timezone.utc),
            limit=100,
        )
        assert result == mock_data

    def test_data_method_as_arrow(self, mock_channel, mock_client):
        """Test that data() method calls get_data_as_arrow when as_arrow=True."""
        mock_data = {"test_channel": MagicMock()}
        mock_client.channels.get_data_as_arrow.return_value = mock_data

        # Call data method with as_arrow=True
        result = mock_channel.data(
            run_id="run123",
            as_arrow=True,
        )

        # Verify get_data_as_arrow was called instead of get_data
        mock_client.channels.get_data_as_arrow.assert_called_once_with(
            channels=[mock_channel],
            run="run123",
            start_time=None,
            end_time=None,
            limit=None,
        )
        mock_client.channels.get_data.assert_not_called()
        assert result == mock_data

    def test_data_method_with_minimal_params(self, mock_channel, mock_client):
        """Test that data() method works with minimal parameters."""
        mock_data = {"test_channel": MagicMock()}
        mock_client.channels.get_data.return_value = mock_data

        # Call data method with no parameters
        result = mock_channel.data()

        # Verify client method was called with None values
        mock_client.channels.get_data.assert_called_once_with(
            channels=[mock_channel],
            run=None,
            start_time=None,
            end_time=None,
            limit=None,
        )
        assert result == mock_data
