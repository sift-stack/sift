"""Tests for sift_types.Rule model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import Rule
from sift_client.sift_types.channel import ChannelReference
from sift_client.sift_types.rule import (
    RuleAction,
    RuleActionType,
    RuleAnnotationType,
    RuleUpdate,
)


@pytest.fixture
def mock_rule(mock_client):
    """Create a mock Rule instance for testing."""
    rule = Rule(
        proto=MagicMock(),
        id_="test_rule_id",
        name="test_rule",
        description="test description",
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        created_by_user_id="user1",
        modified_by_user_id="user1",
        organization_id="org1",
        is_archived=False,
        is_external=False,
        expression="$1 > 100",
        channel_references=[
            ChannelReference(channel_reference="$1", channel_identifier="channel1"),
        ],
        action=RuleAction(
            action_type=RuleActionType.ANNOTATION,
            annotation_type=RuleAnnotationType.DATA_REVIEW,
            tags_ids=["tag1"],
        ),
        asset_ids=["asset1", "asset2"],
        asset_tag_ids=["tag1"],
        contextual_channels=["channel2"],
        client_key=None,
        rule_version=None,
        archived_date=None,
        evaluate_on_live_data=False,
        current_version_id="test_version_id",
    )
    rule._apply_client_to_instance(mock_client)
    return rule


class TestRule:
    """Unit tests for Rule model - tests properties and methods."""

    def test_assets_property_calls_client(self, mock_rule, mock_client):
        """Test that assets property calls client.assets.list_ with correct parameters."""
        mock_client.assets.list_.return_value = []

        # Access assets property
        _ = mock_rule.assets

        # Verify client method was called with correct parameters
        mock_client.assets.list_.assert_called_once_with(
            asset_ids=["asset1", "asset2"], tags=["tag1"]
        )

    def test_update_calls_client_and_updates_self(self, mock_rule, mock_client):
        """Test that update() calls client.rules.update and calls _update."""
        updated_rule = MagicMock()
        updated_rule.description = "Updated description"
        mock_client.rules.update.return_value = updated_rule

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_rule._update = mock_update

            # Call update
            update = RuleUpdate(description="Updated description")
            result = mock_rule.update(update)

            # Verify client method was called with correct parameters
            mock_client.rules.update.assert_called_once_with(
                rule=mock_rule, update=update, version_notes=None
            )
            # Verify _update was called with the returned rule
            mock_update.assert_called_once_with(updated_rule)
            # Verify it returns self
            assert result is mock_rule

    def test_update_with_version_notes(self, mock_rule, mock_client):
        """Test that update() passes version_notes parameter correctly."""
        updated_rule = MagicMock()
        mock_client.rules.update.return_value = updated_rule

        # Mock the _update method
        with MagicMock() as mock_update:
            mock_rule._update = mock_update

            # Call update with version_notes
            update = RuleUpdate(description="Updated")
            mock_rule.update(update, version_notes="Test notes")

            # Verify client method was called with version_notes
            mock_client.rules.update.assert_called_once_with(
                rule=mock_rule, update=update, version_notes="Test notes"
            )

    def test_archive_calls_client_and_updates_self(self, mock_rule, mock_client):
        """Test that archive() calls client.rules.archive and calls _update."""
        archived_rule = MagicMock()
        archived_rule.is_archived = True
        mock_client.rules.archive.return_value = archived_rule

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_rule._update = mock_update

            # Call archive
            result = mock_rule.archive()

            # Verify client method was called
            mock_client.rules.archive.assert_called_once_with(rule=mock_rule)
            # Verify _update was called with the returned rule
            mock_update.assert_called_once_with(archived_rule)
            # Verify it returns self
            assert result is mock_rule

    def test_unarchive_calls_client_and_updates_self(self, mock_rule, mock_client):
        """Test that unarchive() calls client.rules.unarchive and calls _update."""
        unarchived_rule = MagicMock()
        unarchived_rule.is_archived = False
        mock_client.rules.unarchive.return_value = unarchived_rule

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_rule._update = mock_update

            # Call unarchive
            result = mock_rule.unarchive()

            # Verify client method was called
            mock_client.rules.unarchive.assert_called_once_with(rule=mock_rule)
            # Verify _update was called with the returned rule
            mock_update.assert_called_once_with(unarchived_rule)
            # Verify it returns self
            assert result is mock_rule
