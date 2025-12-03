"""Pytest tests for the User Attributes API.

These tests demonstrate and validate the usage of the User Attributes API including:
- Basic user attribute key operations (create, get, list, update, archive)
- User attribute value operations (create single/batch, list, archive)
- Filtering and searching
- Error handling and edge cases
"""

from datetime import datetime, timezone

import pytest
from sift.user_attributes.v1.user_attributes_pb2 import UserAttributeValueType

from sift_client.resources import UserAttributesAPI, UserAttributesAPIAsync
from sift_client.sift_types import UserAttributeKey, UserAttributeValue

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    """Test that user_attributes API is properly registered on the client."""
    assert sift_client.user_attributes
    assert isinstance(sift_client.user_attributes, UserAttributesAPI)
    assert sift_client.async_.user_attributes
    assert isinstance(sift_client.async_.user_attributes, UserAttributesAPIAsync)


@pytest.fixture(scope="session")
def test_timestamp():
    """Setup a test timestamp for the session."""
    timestamp = datetime.now(timezone.utc)
    return timestamp


@pytest.fixture(scope="session")
def test_timestamp_str(test_timestamp):
    """Setup a test timestamp string for the session."""
    return test_timestamp.isoformat()


@pytest.fixture(scope="session")
def test_user_attribute_key(sift_client, test_timestamp_str):
    """Setup a test user attribute key for the session."""
    key = sift_client.user_attributes.create_key(
        name=f"test_dept_{test_timestamp_str}",
        description="Test department",
        value_type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
    )
    yield key
    # Cleanup: archive the key
    try:
        sift_client.user_attributes.archive_key(key.id_)
    except Exception:
        pass


class TestUserAttributeKeys:
    """Tests for User Attribute Keys API."""

    def test_create_key(self, sift_client, test_timestamp_str):
        """Test creating a user attribute key."""
        key = sift_client.user_attributes.create_key(
            name=f"test_create_{test_timestamp_str}",
            description="Test key",
            value_type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
        )

        assert isinstance(key, UserAttributeKey)
        assert key.id_ is not None
        assert key.name == f"test_create_{test_timestamp_str}"
        assert key.type == UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING

        # Cleanup
        sift_client.user_attributes.archive_key(key.id_)

    def test_get_key(self, sift_client, test_user_attribute_key):
        """Test getting a user attribute key by ID."""
        key = sift_client.user_attributes.get_key(test_user_attribute_key.id_)

        assert isinstance(key, UserAttributeKey)
        assert key.id_ == test_user_attribute_key.id_
        assert key.name == test_user_attribute_key.name

    def test_list_keys(self, sift_client, test_user_attribute_key):
        """Test listing user attribute keys."""
        keys = sift_client.user_attributes.list_keys(limit=10)

        assert isinstance(keys, list)
        assert len(keys) > 0
        assert all(isinstance(key, UserAttributeKey) for key in keys)

    def test_list_keys_with_filter(self, sift_client, test_user_attribute_key):
        """Test listing user attribute keys with filtering."""
        keys = sift_client.user_attributes.list_keys(
            name=test_user_attribute_key.name, limit=10
        )

        assert len(keys) >= 1
        assert keys[0].id_ == test_user_attribute_key.id_

    def test_update_key(self, sift_client, test_timestamp_str):
        """Test updating a user attribute key."""
        key = sift_client.user_attributes.create_key(
            name=f"test_update_{test_timestamp_str}",
            description="Original description",
            value_type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
        )

        updated_key = sift_client.user_attributes.update_key(
            key, {"name": f"test_updated_{test_timestamp_str}", "description": "Updated description"}
        )

        assert updated_key.name == f"test_updated_{test_timestamp_str}"
        assert updated_key.description == "Updated description"

        # Cleanup
        sift_client.user_attributes.archive_key(updated_key.id_)

    def test_archive_unarchive_key(self, sift_client, test_timestamp_str):
        """Test archiving and unarchiving a user attribute key."""
        key = sift_client.user_attributes.create_key(
            name=f"test_archive_{test_timestamp_str}",
            value_type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
        )

        # Archive
        sift_client.user_attributes.archive_key(key.id_)
        archived_key = sift_client.user_attributes.get_key(key.id_)
        assert archived_key.is_archived is True

        # Unarchive
        sift_client.user_attributes.unarchive_key(key.id_)
        unarchived_key = sift_client.user_attributes.get_key(key.id_)
        assert unarchived_key.is_archived is False

        # Cleanup
        sift_client.user_attributes.archive_key(key.id_)


class TestUserAttributeValues:
    """Tests for User Attribute Values API."""

    def test_create_value_single(self, sift_client, test_user_attribute_key, test_timestamp_str):
        """Test creating a single user attribute value."""
        value = sift_client.user_attributes.create_value(
            key_id=test_user_attribute_key.id_,
            user_ids=f"user_{test_timestamp_str}",
            string_value="Engineering",
        )

        assert isinstance(value, UserAttributeValue)
        assert value.id_ is not None
        assert value.user_id == f"user_{test_timestamp_str}"
        assert value.string_value == "Engineering"

        # Cleanup
        sift_client.user_attributes.archive_value(value.id_)

    def test_create_value_batch(self, sift_client, test_user_attribute_key, test_timestamp_str):
        """Test creating multiple user attribute values in batch."""
        user_ids = [f"user1_{test_timestamp_str}", f"user2_{test_timestamp_str}"]
        values = sift_client.user_attributes.create_value(
            key_id=test_user_attribute_key.id_,
            user_ids=user_ids,
            string_value="Engineering",
        )

        assert isinstance(values, list)
        assert len(values) == 2
        assert all(isinstance(v, UserAttributeValue) for v in values)
        assert {v.user_id for v in values} == set(user_ids)

        # Cleanup
        sift_client.user_attributes.batch_archive_values([v.id_ for v in values])

    def test_get_value(self, sift_client, test_user_attribute_key, test_timestamp_str):
        """Test getting a user attribute value by ID."""
        # Create a value first
        value = sift_client.user_attributes.create_value(
            key_id=test_user_attribute_key.id_,
            user_ids=f"user_get_{test_timestamp_str}",
            string_value="Engineering",
        )

        retrieved_value = sift_client.user_attributes.get_value(value.id_)

        assert isinstance(retrieved_value, UserAttributeValue)
        assert retrieved_value.id_ == value.id_
        assert retrieved_value.user_id == f"user_get_{test_timestamp_str}"

        # Cleanup
        sift_client.user_attributes.archive_value(value.id_)

    def test_list_values(self, sift_client, test_user_attribute_key, test_timestamp_str):
        """Test listing user attribute values."""
        # Create a value first
        value = sift_client.user_attributes.create_value(
            key_id=test_user_attribute_key.id_,
            user_ids=f"user_list_{test_timestamp_str}",
            string_value="Engineering",
        )

        values = sift_client.user_attributes.list_values(key_id=test_user_attribute_key.id_)

        assert isinstance(values, list)
        assert len(values) > 0
        assert any(v.id_ == value.id_ for v in values)

        # Cleanup
        sift_client.user_attributes.archive_value(value.id_)

    def test_archive_unarchive_value(self, sift_client, test_user_attribute_key, test_timestamp_str):
        """Test archiving and unarchiving a user attribute value."""
        value = sift_client.user_attributes.create_value(
            key_id=test_user_attribute_key.id_,
            user_ids=f"user_archive_{test_timestamp_str}",
            string_value="Engineering",
        )

        # Archive
        sift_client.user_attributes.archive_value(value.id_)
        archived_value = sift_client.user_attributes.get_value(value.id_)
        assert archived_value.is_archived is True

        # Unarchive
        sift_client.user_attributes.unarchive_value(value.id_)
        unarchived_value = sift_client.user_attributes.get_value(value.id_)
        assert unarchived_value.is_archived is False

        # Cleanup
        sift_client.user_attributes.archive_value(value.id_)

