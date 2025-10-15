"""Pytest tests for the Tags API.

These tests demonstrate and validate the usage of the Tags API including:
- Basic tag operations (list, find)
- Tag filtering and searching
- Tag creation and find_or_create
- Error handling and edge cases
"""

import re
from datetime import datetime, timezone

import pytest

from sift_client.resources import TagsAPI, TagsAPIAsync
from sift_client.sift_types import Tag

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert sift_client.tags
    assert isinstance(sift_client.tags, TagsAPI)
    assert sift_client.async_.tags
    assert isinstance(sift_client.async_.tags, TagsAPIAsync)


@pytest.fixture(scope="session")
def test_timestamp():
    """Setup a test tag for the session."""
    timestamp = datetime.now(timezone.utc)
    return timestamp


@pytest.fixture(scope="session")
def test_timestamp_str(test_timestamp):
    """Setup a test tag for the session."""
    return test_timestamp.isoformat()


@pytest.fixture(scope="session")
def test_tags(sift_client, test_timestamp_str):
    """Setup test tags for the session."""
    tag1 = sift_client.tags.create(f"test_tag1_{test_timestamp_str}")
    tag2 = sift_client.tags.create(f"test_tag2_{test_timestamp_str}")
    return tag1, tag2
    # Would like to archive the tags, but this is not supported by the API


class TestTags:
    """Tests for the Tags API."""

    def test_basic_list(self, sift_client, test_tags, test_timestamp_str):
        """Test basic tag listing functionality."""
        tags = sift_client.tags.list_(limit=5)

        # Verify we get a list
        assert isinstance(tags, list)

        # If we have tags, verify their structure
        tag = tags[0]
        assert isinstance(tag, Tag)
        assert tag.id_ is not None
        assert tag.name is not None

    def test_list_with_name_filter(self, sift_client, test_tags, test_timestamp_str):
        """Test tag listing with name filtering."""
        # Create a test tag with a unique name
        name_filter = f"test_tag1_{test_timestamp_str}"
        name_filter_contains = f"tag1_{test_timestamp_str}"
        name_filter_regex = re.compile(rf".*_tag.+_{re.escape(test_timestamp_str)}")

        filtered_tags = sift_client.tags.list_(name=name_filter)
        filtered_tags_contains = sift_client.tags.list_(name_contains=name_filter_contains)
        filtered_tags_regex = sift_client.tags.list_(name_regex=name_filter_regex)
        # Should find exactly one tag with this name
        assert isinstance(filtered_tags, list)
        assert len(filtered_tags) == 1
        assert filtered_tags[0].name == name_filter
        assert filtered_tags_contains[0].name == test_tags[0].name
        assert filtered_tags_regex[0].name == test_tags[0].name
        assert filtered_tags[0].id_ == test_tags[0].id_
        assert filtered_tags_contains[0].id_ == test_tags[0].id_
        assert filtered_tags_regex[0].id_ == test_tags[0].id_

    def test_find_tag(self, sift_client, test_tags, test_timestamp_str):
        """Test finding a single tag. Excercises find and list_ limit functionality."""
        # Create a test tag
        test_tag_name = f"test_tag1_{test_timestamp_str}"

        found_tag = sift_client.tags.find(name=test_tag_name)

        assert found_tag is not None
        assert found_tag.id_ == test_tags[0].id_

    def test_find_nonexistent_tag(self, sift_client):
        """Test finding a non-existent tag returns None."""
        found_tag = sift_client.tags.find(
            name=f"nonexistent_tag_{datetime.now(timezone.utc).timestamp()}"
        )
        assert found_tag is None

    def test_find_multiple_raises_error(self, sift_client, test_timestamp_str, test_tags):
        """Test finding multiple tags raises an error."""
        # Create multiple tags with similar names
        name_filter_regex = re.compile(rf".*_tag(1|2)_{re.escape(test_timestamp_str)}")
        with pytest.raises(ValueError, match="Multiple tags found"):
            _ = sift_client.tags.find(name_regex=name_filter_regex)

    def test_find_or_create_existing_tags(self, sift_client, test_timestamp_str, test_tags):
        """Test find_or_create with existing tags."""
        # Find or create the existing tags
        existing_tag_names = [tag.name for tag in test_tags]
        result_tags = sift_client.tags.find_or_create(existing_tag_names)

        assert len(result_tags) == 2
        result_ids = {tag.id_ for tag in result_tags}
        assert test_tags[0].id_ in result_ids
        assert test_tags[1].id_ in result_ids

    def test_find_or_create_new_tags(self, sift_client, test_timestamp_str, test_tags):
        """Test find_or_create with new tags."""
        new_tag_name = f"test_find_or_create_new_{test_timestamp_str}"

        # Find or create tags that don't exist
        existing_tag_names = [tag.name for tag in test_tags]
        result_tags = sift_client.tags.find_or_create({*existing_tag_names, new_tag_name})

        assert len(result_tags) == len(existing_tag_names) + 1
        result_names = {tag.name for tag in result_tags}
        assert result_names == {*existing_tag_names, new_tag_name}

        # Verify all tags have IDs (were created)
        for tag in result_tags:
            assert tag.id_ is not None

    def test_update_not_implemented(self, sift_client, test_tags):
        """Test that update raises NotImplementedError."""
        # Try to update (should raise NotImplementedError)
        with pytest.raises(NotImplementedError, match="not supported"):
            sift_client.tags.update(test_tags[0], {"name": "new_name"})
