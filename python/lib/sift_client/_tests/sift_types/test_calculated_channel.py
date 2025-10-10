"""Tests for sift_types.CalculatedChannel model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import CalculatedChannel
from sift_client.sift_types.calculated_channel import (
    CalculatedChannelUpdate,
)
from sift_client.sift_types.channel import ChannelReference


class TestCalculatedChannelBase:
    """Unit tests for CalculatedChannelBase - tests _to_proto_helpers and validators shared by Create and Update."""

    def test_metadata_converter(self):
        """Test that metadata is converted using _to_proto_helpers."""
        metadata = {"key1": "value1", "key2": 42.5, "key3": False}
        update = CalculatedChannelUpdate(metadata=metadata)
        update.resource_id = "test_calc_channel_id"

        proto, mask = update.to_proto_with_mask()

        assert len(proto.metadata) == 3

        # Convert list to dict for easier assertion
        metadata_dict = {md.key.name: md for md in proto.metadata}
        assert metadata_dict["key1"].string_value == "value1"
        assert metadata_dict["key2"].number_value == 42.5
        assert metadata_dict["key3"].boolean_value is False
        assert "metadata" in mask.paths

    def test_expression_helper(self):
        """Test that expression is mapped to nested proto path."""
        update = CalculatedChannelUpdate(
            expression="$1 + $2",
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="channel1"),
                ChannelReference(channel_reference="$2", channel_identifier="channel2"),
            ],
        )
        update.resource_id = "test_calc_channel_id"

        proto, mask = update.to_proto_with_mask()

        # Verify expression is set in nested path
        assert (
            proto.calculated_channel_configuration.query_configuration.sel.expression == "$1 + $2"
        )
        assert "query_configuration" in mask.paths

    def test_expression_channel_references_helper(self):
        """Test that expression_channel_references are converted and mapped."""
        update = CalculatedChannelUpdate(
            expression="$1 + $2",
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="channel1"),
                ChannelReference(channel_reference="$2", channel_identifier="channel2"),
            ],
        )
        update.resource_id = "test_calc_channel_id"

        proto, mask = update.to_proto_with_mask()

        # Verify channel references are converted
        refs = proto.calculated_channel_configuration.query_configuration.sel.expression_channel_references
        assert len(refs) == 2
        assert refs[0].channel_reference == "$1"
        assert refs[0].channel_identifier == "channel1"
        assert refs[1].channel_reference == "$2"
        assert refs[1].channel_identifier == "channel2"
        assert "query_configuration" in mask.paths

    def test_tag_ids_helper(self):
        """Test that tag_ids are mapped to nested proto path."""
        update = CalculatedChannelUpdate(tag_ids=["tag1", "tag2"])
        update.resource_id = "test_calc_channel_id"

        proto, mask = update.to_proto_with_mask()

        # Verify tag_ids are set in nested path
        assert list(
            proto.calculated_channel_configuration.asset_configuration.selection.tag_ids
        ) == ["tag1", "tag2"]
        assert "asset_configuration" in mask.paths

    def test_asset_ids_helper(self):
        """Test that asset_ids are mapped to nested proto path."""
        update = CalculatedChannelUpdate(asset_ids=["asset1", "asset2"])
        update.resource_id = "test_calc_channel_id"

        proto, mask = update.to_proto_with_mask()

        # Verify asset_ids are set in nested path
        assert list(
            proto.calculated_channel_configuration.asset_configuration.selection.asset_ids
        ) == ["asset1", "asset2"]
        assert "asset_configuration" in mask.paths

    def test_all_assets_helper(self):
        """Test that all_assets is mapped to nested proto path."""
        update = CalculatedChannelUpdate(all_assets=True)
        update.resource_id = "test_calc_channel_id"

        proto, mask = update.to_proto_with_mask()

        # Verify all_assets is set in nested path
        assert proto.calculated_channel_configuration.asset_configuration.all_assets is True
        # Verify update_field is in mask (same as tag_ids and asset_ids)
        assert "asset_configuration" in mask.paths

    def test_asset_configuration_validator_rejects_all_assets_with_asset_ids(self):
        """Test validator rejects all_assets=True with asset_ids."""
        with pytest.raises(
            ValueError,
            match="Cannot specify both all_assets=True and asset_ids/tag_ids",
        ):
            CalculatedChannelUpdate(
                all_assets=True,
                asset_ids=["asset1"],
            )

    def test_asset_configuration_validator_rejects_all_assets_with_tag_ids(self):
        """Test validator rejects all_assets=True with tag_ids."""
        with pytest.raises(
            ValueError,
            match="Cannot specify both all_assets=True and asset_ids/tag_ids",
        ):
            CalculatedChannelUpdate(
                all_assets=True,
                tag_ids=["tag1"],
            )

    def test_expression_validator_rejects_expression_without_references(self):
        """Test validator rejects expression without channel references."""
        with pytest.raises(
            ValueError, match="Expression and channel references must be set together"
        ):
            CalculatedChannelUpdate(expression="$1 + $2")

    def test_expression_validator_rejects_references_without_expression(self):
        """Test validator rejects channel references without expression."""
        with pytest.raises(
            ValueError, match="Expression and channel references must be set together"
        ):
            CalculatedChannelUpdate(
                expression_channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier="channel1"),
                ],
            )

    def test_expression_validator_accepts_both_set(self):
        """Test validator accepts expression and channel references together."""
        # Should not raise
        update = CalculatedChannelUpdate(
            expression="$1 + $2",
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="channel1"),
                ChannelReference(channel_reference="$2", channel_identifier="channel2"),
            ],
        )
        assert update.expression == "$1 + $2"
        assert len(update.expression_channel_references) == 2


@pytest.fixture
def mock_calculated_channel(mock_client):
    """Create a mock CalculatedChannel instance for testing."""
    calc_channel = CalculatedChannel(
        proto=MagicMock(),
        id_="test_calc_channel_id",
        name="test_calc_channel",
        description="test description",
        expression="$1 + $2",
        channel_references=[
            ChannelReference(channel_reference="$1", channel_identifier="channel1"),
            ChannelReference(channel_reference="$2", channel_identifier="channel2"),
        ],
        is_archived=False,
        units=None,
        asset_ids=None,
        tag_ids=None,
        all_assets=True,
        organization_id="org1",
        client_key=None,
        archived_date=None,
        version_id="v1",
        version=1,
        change_message=None,
        user_notes=None,
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        created_by_user_id="user1",
        modified_by_user_id="user1",
    )
    calc_channel._apply_client_to_instance(mock_client)
    return calc_channel


class TestCalculatedChannel:
    """Unit tests for CalculatedChannel model - tests properties and methods."""

    def test_archive_calls_client_and_updates_self(self, mock_calculated_channel, mock_client):
        """Test that archive() calls client.calculated_channels.archive and calls _update."""
        archived_calc_channel = MagicMock()
        archived_calc_channel.is_archived = True
        mock_client.calculated_channels.archive.return_value = archived_calc_channel

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_calculated_channel._update = mock_update

            # Call archive
            result = mock_calculated_channel.archive()

            # Verify client method was called
            mock_client.calculated_channels.archive.assert_called_once_with(
                calculated_channel=mock_calculated_channel
            )
            # Verify _update was called with the returned calculated channel
            mock_update.assert_called_once_with(archived_calc_channel)
            # Verify it returns self
            assert result is mock_calculated_channel

    def test_unarchive_calls_client_and_updates_self(self, mock_calculated_channel, mock_client):
        """Test that unarchive() calls client.calculated_channels.unarchive and calls _update."""
        unarchived_calc_channel = MagicMock()
        unarchived_calc_channel.is_archived = False
        mock_client.calculated_channels.unarchive.return_value = unarchived_calc_channel

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_calculated_channel._update = mock_update

            # Call unarchive
            result = mock_calculated_channel.unarchive()

            # Verify client method was called
            mock_client.calculated_channels.unarchive.assert_called_once_with(
                calculated_channel=mock_calculated_channel
            )
            # Verify _update was called with the returned calculated channel
            mock_update.assert_called_once_with(unarchived_calc_channel)
            # Verify it returns self
            assert result is mock_calculated_channel

    def test_update_calls_client_and_updates_self(self, mock_calculated_channel, mock_client):
        """Test that update() calls client.calculated_channels.update and calls _update."""
        updated_calc_channel = MagicMock()
        updated_calc_channel.description = "Updated description"
        mock_client.calculated_channels.update.return_value = updated_calc_channel

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_calculated_channel._update = mock_update

            # Call update
            update = CalculatedChannelUpdate(description="Updated description")
            result = mock_calculated_channel.update(update)

            # Verify client method was called with correct parameters
            mock_client.calculated_channels.update.assert_called_once_with(
                calculated_channel=mock_calculated_channel,
                update=update,
                user_notes=None,
            )
            # Verify _update was called with the returned calculated channel
            mock_update.assert_called_once_with(updated_calc_channel)
            # Verify it returns self
            assert result is mock_calculated_channel

    def test_update_with_user_notes(self, mock_calculated_channel, mock_client):
        """Test that update() passes user_notes parameter correctly."""
        updated_calc_channel = MagicMock()
        mock_client.calculated_channels.update.return_value = updated_calc_channel

        # Mock the _update method
        with MagicMock() as mock_update:
            mock_calculated_channel._update = mock_update

            # Call update with user_notes
            update = CalculatedChannelUpdate(description="Updated")
            mock_calculated_channel.update(update, user_notes="Test notes")

            # Verify client method was called with user_notes
            mock_client.calculated_channels.update.assert_called_once_with(
                calculated_channel=mock_calculated_channel,
                update=update,
                user_notes="Test notes",
            )
