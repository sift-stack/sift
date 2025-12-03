"""Tests for sift_types.user_attributes models."""

from datetime import datetime, timezone

import pytest
from sift.user_attributes.v1.user_attributes_pb2 import (
    UserAttributeKey as UserAttributeKeyProto,
)
from sift.user_attributes.v1.user_attributes_pb2 import (
    UserAttributeValue as UserAttributeValueProto,
)
from sift.user_attributes.v1.user_attributes_pb2 import (
    UserAttributeValueType,
)

from sift_client._internal.util.timestamp import to_pb_timestamp
from sift_client.sift_types.user_attributes import (
    UserAttributeKey,
    UserAttributeKeyCreate,
    UserAttributeKeyUpdate,
    UserAttributeValue,
    UserAttributeValueCreate,
)


@pytest.fixture
def mock_user_attribute_key(mock_client):
    """Create a mock UserAttributeKey instance for testing."""
    now = datetime.now(timezone.utc)
    proto = UserAttributeKeyProto(
        user_attribute_key_id="test_key_id",
        organization_id="test_org_id",
        name="department",
        description="User department",
        type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
        created_date=to_pb_timestamp(now),
        created_by_user_id="user1",
        modified_date=to_pb_timestamp(now),
        modified_by_user_id="user1",
        is_archived=False,
    )
    key = UserAttributeKey._from_proto(proto, mock_client)
    return key


@pytest.fixture
def mock_user_attribute_value(mock_client):
    """Create a mock UserAttributeValue instance for testing."""
    now = datetime.now(timezone.utc)
    proto = UserAttributeValueProto(
        user_attribute_value_id="test_value_id",
        user_attribute_key_id="test_key_id",
        user_id="user123",
        organization_id="test_org_id",
        string_value="Engineering",
        created_date=to_pb_timestamp(now),
        created_by_user_id="user1",
        is_archived=False,
    )
    # Set the key field
    key_proto = UserAttributeKeyProto(
        user_attribute_key_id="test_key_id",
        organization_id="test_org_id",
        name="department",
        type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
        created_date=to_pb_timestamp(now),
        created_by_user_id="user1",
        modified_date=to_pb_timestamp(now),
        modified_by_user_id="user1",
        is_archived=False,
    )
    proto.key.CopyFrom(key_proto)
    value = UserAttributeValue._from_proto(proto, mock_client)
    return value


class TestUserAttributeKeyCreate:
    """Unit tests for UserAttributeKeyCreate model."""

    def test_user_attribute_key_create_basic(self):
        """Test basic UserAttributeKeyCreate instantiation."""
        create = UserAttributeKeyCreate(
            name="department", type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING
        )

        assert create.name == "department"
        assert create.type == UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING

    def test_user_attribute_key_create_with_description(self):
        """Test UserAttributeKeyCreate with description."""
        create = UserAttributeKeyCreate(
            name="department",
            description="User department",
            type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
        )

        assert create.name == "department"
        assert create.description == "User department"

    def test_user_attribute_key_create_to_proto(self):
        """Test that UserAttributeKeyCreate converts to proto correctly."""
        create = UserAttributeKeyCreate(
            name="department",
            description="User department",
            type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
        )
        proto = create.to_proto()

        assert proto.name == "department"
        assert proto.description == "User department"
        assert proto.type == UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING


class TestUserAttributeKeyUpdate:
    """Unit tests for UserAttributeKeyUpdate model."""

    def test_user_attribute_key_update_basic(self):
        """Test basic UserAttributeKeyUpdate instantiation."""
        update = UserAttributeKeyUpdate(name="new_name")

        assert update.name == "new_name"
        assert update.description is None

    def test_user_attribute_key_update_to_proto_with_mask(self):
        """Test that UserAttributeKeyUpdate converts to proto with field mask correctly."""
        update = UserAttributeKeyUpdate(name="new_name", description="new description")
        update.resource_id = "test_key_id"
        proto, mask = update.to_proto_with_mask()

        assert proto.user_attribute_key_id == "test_key_id"
        assert proto.name == "new_name"
        assert proto.description == "new description"
        assert "name" in mask.paths
        assert "description" in mask.paths


class TestUserAttributeKey:
    """Unit tests for UserAttributeKey model."""

    def test_user_attribute_key_properties(self, mock_user_attribute_key):
        """Test that UserAttributeKey properties are accessible."""
        assert mock_user_attribute_key.id_ == "test_key_id"
        assert mock_user_attribute_key.name == "department"
        assert mock_user_attribute_key.organization_id == "test_org_id"
        assert mock_user_attribute_key.description == "User department"
        assert (
            mock_user_attribute_key.type == UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING
        )
        assert mock_user_attribute_key.created_by_user_id == "user1"
        assert mock_user_attribute_key.created_date is not None
        assert mock_user_attribute_key.created_date.tzinfo == timezone.utc
        assert mock_user_attribute_key.is_archived is False

    def test_user_attribute_key_from_proto(self, mock_client):
        """Test UserAttributeKey creation from proto."""
        now = datetime.now(timezone.utc)
        proto = UserAttributeKeyProto(
            user_attribute_key_id="test_key_id",
            organization_id="test_org_id",
            name="department",
            type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
            created_date=to_pb_timestamp(now),
            created_by_user_id="user1",
            modified_date=to_pb_timestamp(now),
            modified_by_user_id="user1",
            is_archived=False,
        )

        key = UserAttributeKey._from_proto(proto, mock_client)

        assert key.id_ == "test_key_id"
        assert key.name == "department"
        assert key.organization_id == "test_org_id"

    def test_user_attribute_key_without_client_raises_error(self):
        """Test that accessing client without setting it raises an error."""
        now = datetime.now(timezone.utc)
        proto = UserAttributeKeyProto(
            user_attribute_key_id="test_key_id",
            organization_id="test_org_id",
            name="department",
            type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
            created_date=to_pb_timestamp(now),
            created_by_user_id="user1",
            modified_date=to_pb_timestamp(now),
            modified_by_user_id="user1",
            is_archived=False,
        )
        key = UserAttributeKey._from_proto(proto, None)

        with pytest.raises(AttributeError, match="Sift client not set"):
            _ = key.client


class TestUserAttributeValueCreate:
    """Unit tests for UserAttributeValueCreate model."""

    def test_user_attribute_value_create_string(self):
        """Test UserAttributeValueCreate with string value."""
        create = UserAttributeValueCreate(
            user_attribute_key_id="test_key_id",
            user_id="user123",
            string_value="Engineering",
        )

        assert create.user_attribute_key_id == "test_key_id"
        assert create.user_id == "user123"
        assert create.string_value == "Engineering"

    def test_user_attribute_value_create_number(self):
        """Test UserAttributeValueCreate with number value."""
        create = UserAttributeValueCreate(
            user_attribute_key_id="test_key_id", user_id="user123", number_value=42.5
        )

        assert create.number_value == 42.5

    def test_user_attribute_value_create_boolean(self):
        """Test UserAttributeValueCreate with boolean value."""
        create = UserAttributeValueCreate(
            user_attribute_key_id="test_key_id", user_id="user123", boolean_value=True
        )

        assert create.boolean_value is True

    def test_user_attribute_value_create_to_proto(self):
        """Test that UserAttributeValueCreate converts to proto correctly."""
        create = UserAttributeValueCreate(
            user_attribute_key_id="test_key_id",
            user_id="user123",
            string_value="Engineering",
        )
        proto = create.to_proto()

        assert proto.user_attribute_key_id == "test_key_id"
        assert proto.user_id == "user123"
        assert proto.string_value == "Engineering"


class TestUserAttributeValue:
    """Unit tests for UserAttributeValue model."""

    def test_user_attribute_value_properties(self, mock_user_attribute_value):
        """Test that UserAttributeValue properties are accessible."""
        assert mock_user_attribute_value.id_ == "test_value_id"
        assert mock_user_attribute_value.user_attribute_key_id == "test_key_id"
        assert mock_user_attribute_value.user_id == "user123"
        assert mock_user_attribute_value.organization_id == "test_org_id"
        assert mock_user_attribute_value.string_value == "Engineering"
        assert mock_user_attribute_value.created_by_user_id == "user1"
        assert mock_user_attribute_value.created_date is not None
        assert mock_user_attribute_value.created_date.tzinfo == timezone.utc
        assert mock_user_attribute_value.is_archived is False
        assert mock_user_attribute_value.key is not None

    def test_user_attribute_value_from_proto_string(self, mock_client):
        """Test UserAttributeValue creation from proto with string value."""
        now = datetime.now(timezone.utc)
        proto = UserAttributeValueProto(
            user_attribute_value_id="test_value_id",
            user_attribute_key_id="test_key_id",
            user_id="user123",
            organization_id="test_org_id",
            string_value="Engineering",
            created_date=to_pb_timestamp(now),
            created_by_user_id="user1",
            is_archived=False,
        )

        value = UserAttributeValue._from_proto(proto, mock_client)

        assert value.id_ == "test_value_id"
        assert value.string_value == "Engineering"
        assert value.number_value is None
        assert value.boolean_value is None

    def test_user_attribute_value_from_proto_number(self, mock_client):
        """Test UserAttributeValue creation from proto with number value."""
        now = datetime.now(timezone.utc)
        proto = UserAttributeValueProto(
            user_attribute_value_id="test_value_id",
            user_attribute_key_id="test_key_id",
            user_id="user123",
            organization_id="test_org_id",
            number_value=42.5,
            created_date=to_pb_timestamp(now),
            created_by_user_id="user1",
            is_archived=False,
        )

        value = UserAttributeValue._from_proto(proto, mock_client)

        assert value.number_value == 42.5
        assert value.string_value is None
        assert value.boolean_value is None

    def test_user_attribute_value_without_client_raises_error(self):
        """Test that accessing client without setting it raises an error."""
        now = datetime.now(timezone.utc)
        proto = UserAttributeValueProto(
            user_attribute_value_id="test_value_id",
            user_attribute_key_id="test_key_id",
            user_id="user123",
            organization_id="test_org_id",
            string_value="Engineering",
            created_date=to_pb_timestamp(now),
            created_by_user_id="user1",
            is_archived=False,
        )
        value = UserAttributeValue._from_proto(proto, None)

        with pytest.raises(AttributeError, match="Sift client not set"):
            _ = value.client
