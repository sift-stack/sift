"""Pytest tests for the Tags API.

These tests demonstrate and validate the usage of the Tags API including:
- Basic tag operations (list, find)
- Tag filtering and searching
- Tag creation and find_or_create
- Error handling and edge cases
"""

from datetime import datetime, timezone

import pytest

from sift_client import SiftClient
from sift_client.resources import TagsAPI, TagsAPIAsync
from sift_client.sift_types import Tag

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert sift_client.tags
    assert isinstance(sift_client.tags, TagsAPI)
    assert sift_client.async_.tags
    assert isinstance(sift_client.async_.tags, TagsAPIAsync)


@pytest.fixture
def tags_api_async(sift_client: SiftClient):
    """Get the async tags API instance."""
    return sift_client.async_.tags


@pytest.fixture
def tags_api_sync(sift_client: SiftClient):
    """Get the synchronous tags API instance."""
    return sift_client.tags


@pytest.fixture
def test_tag(tags_api_sync):
    """Get an existing tag for testing."""
    tags = tags_api_sync.list_(limit=1)
    if not tags:
        # Create a tag if none exists
        tag = tags_api_sync.create(f"test_tag_{datetime.now(timezone.utc).isoformat()}")
        return tag
    return tags[0]


class TestTagsAPIAsync:
    """Test suite for the async Tags API functionality."""

    class TestList:
        """Tests for the async list method."""

        @pytest.mark.asyncio
        async def test_basic_list(self, tags_api_async):
            """Test basic tag listing functionality."""
            tags = await tags_api_async.list_(limit=5)

            # Verify we get a list
            assert isinstance(tags, list)

            # If we have tags, verify their structure
            if tags:
                tag = tags[0]
                assert isinstance(tag, Tag)
                assert tag.id_ is not None
                assert tag.name is not None

        @pytest.mark.asyncio
        async def test_list_with_name_filter(self, tags_api_async):
            """Test tag listing with name filtering."""
            # Create a test tag with a unique name
            test_tag_name = f"test_tag_exact_{datetime.now(timezone.utc).timestamp()}"
            created_tag = await tags_api_async.create(test_tag_name)

            try:
                # Filter by exact name
                filtered_tags = await tags_api_async.list_(name=test_tag_name)

                # Should find exactly one tag with this name
                assert isinstance(filtered_tags, list)
                assert len(filtered_tags) == 1
                assert filtered_tags[0].name == test_tag_name
                assert filtered_tags[0].id_ == created_tag.id_
            finally:
                # Note: No archive method for tags, they persist
                pass

        @pytest.mark.asyncio
        async def test_list_with_name_contains_filter(self, tags_api_async):
            """Test tag listing with name contains filtering."""
            # Create test tags with a common substring
            test_prefix = f"test_contains_{datetime.now(timezone.utc).timestamp()}"
            created_tag1 = await tags_api_async.create(f"{test_prefix}_alpha")
            created_tag2 = await tags_api_async.create(f"{test_prefix}_beta")

            try:
                # Filter by substring
                filtered_tags = await tags_api_async.list_(name_contains=test_prefix)

                assert isinstance(filtered_tags, list)
                assert len(filtered_tags) >= 2

                # Verify all returned tags contain the substring
                tag_names = [tag.name for tag in filtered_tags]
                assert created_tag1.name in tag_names
                assert created_tag2.name in tag_names

                for tag in filtered_tags:
                    assert test_prefix in tag.name
            finally:
                pass

        @pytest.mark.asyncio
        async def test_list_with_name_regex_filter(self, tags_api_async):
            """Test tag listing with regex filtering."""
            # Create test tags
            test_prefix = f"test_regex_{datetime.now(timezone.utc).timestamp()}"
            created_tag1 = await tags_api_async.create(f"{test_prefix}_123")
            created_tag2 = await tags_api_async.create(f"{test_prefix}_456")

            try:
                # Filter with regex pattern for tags ending in digits
                filtered_tags = await tags_api_async.list_(name_regex=f"{test_prefix}_\\d+")

                assert isinstance(filtered_tags, list)
                assert len(filtered_tags) >= 2

                # Verify the tags match
                tag_names = [tag.name for tag in filtered_tags]
                assert created_tag1.name in tag_names
                assert created_tag2.name in tag_names
            finally:
                pass

        @pytest.mark.asyncio
        async def test_list_with_names_filter(self, tags_api_async):
            """Test tag listing with multiple names filter."""
            # Create test tags
            test_prefix = f"test_names_{datetime.now(timezone.utc).timestamp()}"
            tag1_name = f"{test_prefix}_one"
            tag2_name = f"{test_prefix}_two"
            created_tag1 = await tags_api_async.create(tag1_name)
            created_tag2 = await tags_api_async.create(tag2_name)

            try:
                # Filter by list of names
                filtered_tags = await tags_api_async.list_(names=[tag1_name, tag2_name])

                assert isinstance(filtered_tags, list)
                assert len(filtered_tags) == 2

                tag_names = {tag.name for tag in filtered_tags}
                assert tag_names == {tag1_name, tag2_name}

                # Verify the created tags are in the results
                assert created_tag1.id_ in {tag.id_ for tag in filtered_tags}
                assert created_tag2.id_ in {tag.id_ for tag in filtered_tags}
            finally:
                pass

        @pytest.mark.asyncio
        async def test_list_with_tag_ids_filter(self, tags_api_async):
            """Test tag listing with tag IDs filter."""
            # Create test tags
            test_prefix = f"test_ids_{datetime.now(timezone.utc).timestamp()}"
            created_tag1 = await tags_api_async.create(f"{test_prefix}_1")
            created_tag2 = await tags_api_async.create(f"{test_prefix}_2")

            try:
                # Filter by tag IDs
                filtered_tags = await tags_api_async.list_(
                    tag_ids=[created_tag1.id_, created_tag2.id_]
                )

                assert isinstance(filtered_tags, list)
                assert len(filtered_tags) == 2

                tag_ids = {tag.id_ for tag in filtered_tags}
                assert tag_ids == {created_tag1.id_, created_tag2.id_}
            finally:
                pass

        @pytest.mark.asyncio
        async def test_list_with_limit(self, tags_api_async):
            """Test tag listing with different limits."""
            # Test with limit of 1
            tags_1 = await tags_api_async.list_(limit=1)
            assert isinstance(tags_1, list)
            assert len(tags_1) <= 1

            # Test with limit of 3
            tags_3 = await tags_api_async.list_(limit=3)
            assert isinstance(tags_3, list)
            assert len(tags_3) <= 3

        @pytest.mark.asyncio
        async def test_list_include_archived(self, tags_api_async):
            """Test tag listing with archived tags included."""
            # Test without archived tags (default)
            tags_active = await tags_api_async.list_(limit=5, include_archived=False)
            assert isinstance(tags_active, list)

            # Test with archived tags included
            tags_all = await tags_api_async.list_(limit=5, include_archived=True)
            assert isinstance(tags_all, list)

            # Should have at least as many tags when including archived
            assert len(tags_all) >= len(tags_active)

    class TestFind:
        """Tests for the async find method."""

        @pytest.mark.asyncio
        async def test_find_tag(self, tags_api_async):
            """Test finding a single tag."""
            # Create a test tag
            test_tag_name = f"test_find_{datetime.now(timezone.utc).timestamp()}"
            created_tag = await tags_api_async.create(test_tag_name)

            try:
                # Find the tag by name
                found_tag = await tags_api_async.find(name=test_tag_name)

                assert found_tag is not None
                assert found_tag.id_ == created_tag.id_
                assert found_tag.name == test_tag_name
            finally:
                pass

        @pytest.mark.asyncio
        async def test_find_nonexistent_tag(self, tags_api_async):
            """Test finding a non-existent tag returns None."""
            found_tag = await tags_api_async.find(
                name=f"nonexistent_tag_{datetime.now(timezone.utc).timestamp()}"
            )
            assert found_tag is None

        @pytest.mark.asyncio
        async def test_find_multiple_raises_error(self, tags_api_async):
            """Test finding multiple tags raises an error."""
            # Create multiple tags with similar names
            test_prefix = f"test_multi_{datetime.now(timezone.utc).timestamp()}"
            created_tag1 = await tags_api_async.create(f"{test_prefix}_1")
            created_tag2 = await tags_api_async.create(f"{test_prefix}_2")

            try:
                # Verify both tags exist
                assert created_tag1.id_ is not None
                assert created_tag2.id_ is not None

                # Try to find with a filter that matches multiple tags
                with pytest.raises(ValueError, match="Multiple tags found"):
                    await tags_api_async.find(name_contains=test_prefix)
            finally:
                pass

    class TestCreate:
        """Tests for the async create method."""

        @pytest.mark.asyncio
        async def test_create_basic_tag(self, tags_api_async):
            """Test creating a basic tag."""
            tag_name = f"test_create_{datetime.now(timezone.utc).timestamp()}"

            created_tag = await tags_api_async.create(tag_name)

            # Verify the tag was created
            assert created_tag is not None
            assert isinstance(created_tag, Tag)
            assert created_tag.id_ is not None
            assert created_tag.name == tag_name
            assert created_tag.created_date is not None

        @pytest.mark.asyncio
        async def test_create_tag_is_persisted(self, tags_api_async):
            """Test that a created tag can be retrieved."""
            tag_name = f"test_persist_{datetime.now(timezone.utc).timestamp()}"

            created_tag = await tags_api_async.create(tag_name)

            # Retrieve the tag
            found_tag = await tags_api_async.find(name=tag_name)

            assert found_tag is not None
            assert found_tag.id_ == created_tag.id_
            assert found_tag.name == tag_name

    class TestFindOrCreate:
        """Tests for the async find_or_create method."""

        @pytest.mark.asyncio
        async def test_find_or_create_existing_tags(self, tags_api_async):
            """Test find_or_create with existing tags."""
            # Create test tags
            test_prefix = f"test_foc_exist_{datetime.now(timezone.utc).timestamp()}"
            tag1_name = f"{test_prefix}_1"
            tag2_name = f"{test_prefix}_2"

            tag1 = await tags_api_async.create(tag1_name)
            tag2 = await tags_api_async.create(tag2_name)

            try:
                # Find or create the existing tags
                result_tags = await tags_api_async.find_or_create([tag1_name, tag2_name])

                assert len(result_tags) == 2
                result_ids = {tag.id_ for tag in result_tags}
                assert tag1.id_ in result_ids
                assert tag2.id_ in result_ids
            finally:
                pass

        @pytest.mark.asyncio
        async def test_find_or_create_new_tags(self, tags_api_async):
            """Test find_or_create with new tags."""
            test_prefix = f"test_foc_new_{datetime.now(timezone.utc).timestamp()}"
            tag1_name = f"{test_prefix}_1"
            tag2_name = f"{test_prefix}_2"

            # Find or create tags that don't exist
            result_tags = await tags_api_async.find_or_create([tag1_name, tag2_name])

            assert len(result_tags) == 2
            result_names = {tag.name for tag in result_tags}
            assert result_names == {tag1_name, tag2_name}

            # Verify all tags have IDs (were created)
            for tag in result_tags:
                assert tag.id_ is not None

        @pytest.mark.asyncio
        async def test_find_or_create_mixed(self, tags_api_async):
            """Test find_or_create with a mix of existing and new tags."""
            test_prefix = f"test_foc_mixed_{datetime.now(timezone.utc).timestamp()}"
            existing_name = f"{test_prefix}_existing"
            new_name = f"{test_prefix}_new"

            # Create one tag
            existing_tag = await tags_api_async.create(existing_name)

            try:
                # Find or create with one existing and one new
                result_tags = await tags_api_async.find_or_create([existing_name, new_name])

                assert len(result_tags) == 2
                result_names = {tag.name for tag in result_tags}
                assert result_names == {existing_name, new_name}

                # Verify the existing tag has the same ID
                existing_result = next(tag for tag in result_tags if tag.name == existing_name)
                assert existing_result.id_ == existing_tag.id_

                # Verify the new tag was created
                new_result = next(tag for tag in result_tags if tag.name == new_name)
                assert new_result.id_ is not None
            finally:
                pass

    class TestUpdate:
        """Tests for the async update method."""

        @pytest.mark.asyncio
        async def test_update_not_implemented(self, tags_api_async):
            """Test that update raises NotImplementedError."""
            # Create a test tag
            tag_name = f"test_update_{datetime.now(timezone.utc).timestamp()}"
            created_tag = await tags_api_async.create(tag_name)

            # Try to update (should raise NotImplementedError)
            with pytest.raises(NotImplementedError, match="not supported"):
                await tags_api_async.update(created_tag, {"name": "new_name"})


class TestTagsAPISync:
    """Test suite for the synchronous Tags API functionality.

    Only includes a single test for basic sync generation. No specific sync behavior difference tests are needed.
    """

    class TestList:
        """Tests for the sync list method."""

        def test_basic_list(self, tags_api_sync):
            """Test basic synchronous tag listing functionality."""
            tags = tags_api_sync.list_(limit=5)

            # Verify we get a list
            assert isinstance(tags, list)

            # If we have tags, verify their structure
            if tags:
                assert isinstance(tags[0], Tag)

    class TestCreate:
        """Tests for the sync create method."""

        def test_create_basic_tag(self, tags_api_sync):
            """Test creating a basic tag synchronously."""
            tag_name = f"test_sync_create_{datetime.now(timezone.utc).timestamp()}"

            created_tag = tags_api_sync.create(tag_name)

            # Verify the tag was created
            assert created_tag is not None
            assert isinstance(created_tag, Tag)
            assert created_tag.id_ is not None
            assert created_tag.name == tag_name

    class TestFindOrCreate:
        """Tests for the sync find_or_create method."""

        def test_find_or_create(self, tags_api_sync):
            """Test synchronous find_or_create."""
            test_prefix = f"test_sync_foc_{datetime.now(timezone.utc).timestamp()}"
            tag1_name = f"{test_prefix}_1"
            tag2_name = f"{test_prefix}_2"

            # Find or create tags
            result_tags = tags_api_sync.find_or_create([tag1_name, tag2_name])

            assert len(result_tags) == 2
            result_names = {tag.name for tag in result_tags}
            assert result_names == {tag1_name, tag2_name}
