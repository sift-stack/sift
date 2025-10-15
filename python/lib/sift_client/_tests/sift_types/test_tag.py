"""Tests for sift_types.Tag model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types.tag import Tag, TagCreate


@pytest.fixture
def mock_tag(mock_client):
    """Create a mock Tag instance for testing."""
    tag = Tag(
        proto=MagicMock(),
        id_="test_tag_id",
        name="test_tag",
        created_date=datetime.now(timezone.utc),
        created_by_user_id="user1",
    )
    tag._apply_client_to_instance(mock_client)
    return tag


class TestTagCreate:
    """Unit tests for TagCreate model."""

    def test_tag_create_basic(self):
        """Test basic TagCreate instantiation."""
        create = TagCreate(name="test_tag")

        assert create.name == "test_tag"

    def test_tag_create_to_proto(self):
        """Test that TagCreate converts to proto correctly."""
        create = TagCreate(name="test_tag")
        proto = create.to_proto()

        assert proto.name == "test_tag"


class TestTag:
    """Unit tests for Tag model - tests properties and methods."""

    def test_tag_properties(self, mock_tag):
        """Test that Tag properties are accessible."""
        assert mock_tag.id_ == "test_tag_id"
        assert mock_tag.name == "test_tag"
        assert mock_tag.created_by_user_id == "user1"
        assert mock_tag.created_date is not None
        assert mock_tag.created_date.tzinfo == timezone.utc

    def test_tag_str(self, mock_tag):
        """Test Tag string representation."""
        assert str(mock_tag) == "test_tag"

    def test_tag_to_proto(self, mock_tag):
        """Test that Tag can be converted to proto."""
        proto = mock_tag._to_proto()

        assert proto.tag_id == "test_tag_id"
        assert proto.name == "test_tag"
        assert proto.created_by_user_id == "user1"

    def test_tag_without_client_raises_error(self):
        """Test that accessing client without setting it raises an error."""
        tag = Tag(
            id_="test_tag_id",
            name="test_tag",
            created_date=datetime.now(timezone.utc),
            created_by_user_id="user1",
        )

        with pytest.raises(AttributeError, match="Sift client not set"):
            _ = tag.client
