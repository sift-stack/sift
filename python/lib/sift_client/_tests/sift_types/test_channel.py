"""Tests for sift_types.Channel model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import Channel
from sift_client.sift_types.channel import ChannelDataType, ChannelReference, ChannelUpdate


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
        metadata={},
        active=True,
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

    def test_channel_reference_requires_one_target(self):
        """ChannelReference must specify exactly one of identifier or calculated_channel."""
        with pytest.raises(ValueError, match="exactly one"):
            ChannelReference(channel_reference="$1")
        with pytest.raises(ValueError, match="exactly one"):
            ChannelReference(
                channel_reference="$1",
                channel_identifier="ch",
                calculated_channel="v-id",
            )

    def test_channel_reference_accepts_version_id_string(self):
        """A plain version_id string is stored as-is."""
        ref = ChannelReference(channel_reference="$1", calculated_channel="v-abc")
        assert ref.calculated_channel == "v-abc"
        assert ref.channel_identifier is None

    def test_channel_reference_accepts_calculated_channel_object(self):
        """Passing a CalculatedChannel normalizes to its version_id string."""
        from sift_client.sift_types.calculated_channel import CalculatedChannel

        cc = CalculatedChannel(
            proto=MagicMock(),
            id_="cc-id",
            name="parent",
            description="",
            expression="$1",
            channel_references=[],
            is_archived=False,
            units=None,
            asset_ids=["asset-1"],
            tag_ids=None,
            all_assets=False,
            organization_id=None,
            client_key=None,
            archived_date=None,
            version_id="v-abc",
            version=1,
            change_message=None,
            user_notes=None,
            created_date=datetime.now(timezone.utc),
            modified_date=datetime.now(timezone.utc),
            created_by_user_id="u",
            modified_by_user_id="u",
        )

        ref = ChannelReference(channel_reference="$1", calculated_channel=cc)
        assert ref.calculated_channel == "v-abc"

    def test_channel_reference_rejects_calculated_channel_without_version_id(self):
        """A CalculatedChannel missing version_id is unusable as a reference."""
        from sift_client.sift_types.calculated_channel import CalculatedChannel

        cc = CalculatedChannel(
            proto=MagicMock(),
            id_="cc-id",
            name="parent",
            description="",
            expression="$1",
            channel_references=[],
            is_archived=False,
            units=None,
            asset_ids=["asset-1"],
            tag_ids=None,
            all_assets=False,
            organization_id=None,
            client_key=None,
            archived_date=None,
            version_id=None,
            version=None,
            change_message=None,
            user_notes=None,
            created_date=datetime.now(timezone.utc),
            modified_date=datetime.now(timezone.utc),
            created_by_user_id="u",
            modified_by_user_id="u",
        )

        with pytest.raises(ValueError, match="no version_id"):
            ChannelReference(channel_reference="$1", calculated_channel=cc)

    def test_channel_reference_from_proto_reads_version_id_oneof(self):
        """_from_proto picks calculated_channel when the proto oneof selects it."""
        from sift.calculated_channels.v2.calculated_channels_pb2 import (
            CalculatedChannelAbstractChannelReference,
        )

        proto = CalculatedChannelAbstractChannelReference(
            channel_reference="$1", calculated_channel_version_id="v-abc"
        )
        ref = ChannelReference._from_proto(proto)
        assert ref.calculated_channel == "v-abc"
        assert ref.channel_identifier is None

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

    def test_update_calls_client_and_updates_self(self, mock_channel, mock_client):
        """Test that update() calls client.channels.update and calls _update."""
        updated_channel = MagicMock()
        mock_client.channels.update.return_value = updated_channel

        with MagicMock() as mock_update:
            mock_channel._update = mock_update

            update = ChannelUpdate(description="new description")
            result = mock_channel.update(update)

            mock_client.channels.update.assert_called_once_with(channel=mock_channel, update=update)
            mock_update.assert_called_once_with(updated_channel)
            assert result is mock_channel

    def test_archive_sets_active_false_via_update(self, mock_channel, mock_client):
        """archive() delegates to update with active=False and updates in place."""
        updated = MagicMock()
        mock_client.channels.update.return_value = updated

        with MagicMock() as mock_update:
            mock_channel._update = mock_update
            result = mock_channel.archive()

            mock_client.channels.update.assert_called_once_with(
                channel=mock_channel, update={"active": False}
            )
            mock_update.assert_called_once_with(updated)
            assert result is mock_channel

    def test_unarchive_sets_active_true_via_update(self, mock_channel, mock_client):
        """unarchive() delegates to update with active=True and updates in place."""
        updated = MagicMock()
        mock_client.channels.update.return_value = updated

        with MagicMock() as mock_update:
            mock_channel._update = mock_update
            result = mock_channel.unarchive()

            mock_client.channels.update.assert_called_once_with(
                channel=mock_channel, update={"active": True}
            )
            mock_update.assert_called_once_with(updated)
            assert result is mock_channel


class TestChannelUpdate:
    """Channel-specific field wiring for ChannelUpdate.

    The generic ModelUpdate behavior (field-mask generation, unset/None exclusion,
    resource-id requirement, the metadata converter and MappingHelper path expansion)
    is already covered in test_base.py and test_asset.py. These tests only assert the
    parts unique to ChannelUpdate: which proto fields its fields map onto.
    """

    def test_fields_map_to_correct_proto_fields(self):
        """Each ChannelUpdate field targets its matching Channel proto field and mask path."""
        update = ChannelUpdate(
            description="new description",
            unit="volts",
            metadata={"source": "pytest"},
            active=False,
        )
        update.resource_id = "test_channel_id"

        proto, mask = update.to_proto_with_mask()

        assert proto.channel_id == "test_channel_id"
        # description/unit write the display override fields, the only ones the
        # server allows UpdateChannel to mutate.
        assert proto.display_description == "new description"
        assert proto.display_unit_id == "volts"
        assert {md.key.name: md.string_value for md in proto.metadata} == {"source": "pytest"}
        assert proto.active is False
        # The server's update mask accepts "display_units" (not "display_unit_id")
        # for the unit; see channel_service.go UpdateChannel.
        assert set(mask.paths) == {
            "display_description",
            "display_units",
            "metadata",
            "active",
        }
