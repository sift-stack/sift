"""Tests for sift_types.resource_attribute models."""

from datetime import datetime, timezone

import pytest
from sift.resource_attribute.v1.resource_attribute_pb2 import (
    ResourceAttribute as ResourceAttributeProto,
)
from sift.resource_attribute.v1.resource_attribute_pb2 import (
    ResourceAttributeEntityIdentifier,
    ResourceAttributeEntityType,
    ResourceAttributeKeyType,
)
from sift.resource_attribute.v1.resource_attribute_pb2 import (
    ResourceAttributeEnumValue as ResourceAttributeEnumValueProto,
)
from sift.resource_attribute.v1.resource_attribute_pb2 import (
    ResourceAttributeKey as ResourceAttributeKeyProto,
)

from sift_client._internal.util.timestamp import to_pb_timestamp
from sift_client.sift_types.resource_attribute import (
    ResourceAttribute,
    ResourceAttributeCreate,
    ResourceAttributeEnumValue,
    ResourceAttributeEnumValueCreate,
    ResourceAttributeEnumValueUpdate,
    ResourceAttributeKey,
    ResourceAttributeKeyCreate,
    ResourceAttributeKeyUpdate,
)


@pytest.fixture
def mock_resource_attribute_key(mock_client):
    """Create a mock ResourceAttributeKey instance for testing."""
    now = datetime.now(timezone.utc)
    proto = ResourceAttributeKeyProto(
        resource_attribute_key_id="test_key_id",
        organization_id="test_org_id",
        display_name="environment",
        description="Deployment environment",
        type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
        created_date=to_pb_timestamp(now),
        created_by_user_id="user1",
        modified_date=to_pb_timestamp(now),
        modified_by_user_id="user1",
    )
    key = ResourceAttributeKey._from_proto(proto, mock_client)
    return key


@pytest.fixture
def mock_resource_attribute_enum_value(mock_client):
    """Create a mock ResourceAttributeEnumValue instance for testing."""
    now = datetime.now(timezone.utc)
    proto = ResourceAttributeEnumValueProto(
        resource_attribute_enum_value_id="test_enum_value_id",
        resource_attribute_key_id="test_key_id",
        display_name="production",
        description="Production environment",
        created_date=to_pb_timestamp(now),
        created_by_user_id="user1",
        modified_date=to_pb_timestamp(now),
        modified_by_user_id="user1",
    )
    enum_value = ResourceAttributeEnumValue._from_proto(proto, mock_client)
    return enum_value


@pytest.fixture
def mock_resource_attribute(mock_client):
    """Create a mock ResourceAttribute instance for testing."""
    now = datetime.now(timezone.utc)
    entity = ResourceAttributeEntityIdentifier(
        entity_id="asset123",
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
    )
    proto = ResourceAttributeProto(
        resource_attribute_id="test_attr_id",
        organization_id="test_org_id",
        entity=entity,
        resource_attribute_key_id="test_key_id",
        resource_attribute_enum_value_id="test_enum_value_id",
        created_date=to_pb_timestamp(now),
        created_by_user_id="user1",
    )
    attr = ResourceAttribute._from_proto(proto, mock_client)
    return attr


class TestResourceAttributeKeyCreate:
    """Unit tests for ResourceAttributeKeyCreate model."""

    def test_resource_attribute_key_create_basic(self):
        """Test basic ResourceAttributeKeyCreate instantiation."""
        create = ResourceAttributeKeyCreate(
            display_name="environment",
            type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
        )

        assert create.display_name == "environment"
        assert create.type == ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM

    def test_resource_attribute_key_create_with_initial_enum_values(self):
        """Test ResourceAttributeKeyCreate with initial enum values."""
        create = ResourceAttributeKeyCreate(
            display_name="environment",
            type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
            initial_enum_values=[
                {"display_name": "production", "description": "Prod env"},
                {"display_name": "staging"},
            ],
        )

        assert create.initial_enum_values is not None
        assert len(create.initial_enum_values) == 2
        assert create.initial_enum_values[0]["display_name"] == "production"

    def test_resource_attribute_key_create_to_proto(self):
        """Test that ResourceAttributeKeyCreate converts to proto correctly."""
        create = ResourceAttributeKeyCreate(
            display_name="environment",
            description="Deployment environment",
            type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
            initial_enum_values=[{"display_name": "production"}],
        )
        proto = create.to_proto()

        assert proto.display_name == "environment"
        assert proto.description == "Deployment environment"
        assert proto.type == ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM
        assert len(proto.initial_enum_values) == 1
        assert proto.initial_enum_values[0].display_name == "production"


class TestResourceAttributeKeyUpdate:
    """Unit tests for ResourceAttributeKeyUpdate model."""

    def test_resource_attribute_key_update_basic(self):
        """Test basic ResourceAttributeKeyUpdate instantiation."""
        update = ResourceAttributeKeyUpdate(display_name="new_name")

        assert update.display_name == "new_name"
        assert update.description is None

    def test_resource_attribute_key_update_to_proto_with_mask(self):
        """Test that ResourceAttributeKeyUpdate converts to proto with field mask correctly."""
        update = ResourceAttributeKeyUpdate(display_name="new_name", description="new description")
        update.resource_id = "test_key_id"
        proto, mask = update.to_proto_with_mask()

        assert proto.resource_attribute_key_id == "test_key_id"
        assert proto.display_name == "new_name"
        assert proto.description == "new description"
        assert "display_name" in mask.paths
        assert "description" in mask.paths


class TestResourceAttributeKey:
    """Unit tests for ResourceAttributeKey model."""

    def test_resource_attribute_key_properties(self, mock_resource_attribute_key):
        """Test that ResourceAttributeKey properties are accessible."""
        assert mock_resource_attribute_key.id_ == "test_key_id"
        assert mock_resource_attribute_key.display_name == "environment"
        assert mock_resource_attribute_key.organization_id == "test_org_id"
        assert mock_resource_attribute_key.description == "Deployment environment"
        assert (
            mock_resource_attribute_key.type
            == ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM
        )
        assert mock_resource_attribute_key.created_by_user_id == "user1"
        assert mock_resource_attribute_key.created_date is not None
        assert mock_resource_attribute_key.created_date.tzinfo == timezone.utc

    def test_resource_attribute_key_without_client_raises_error(self):
        """Test that accessing client without setting it raises an error."""
        now = datetime.now(timezone.utc)
        proto = ResourceAttributeKeyProto(
            resource_attribute_key_id="test_key_id",
            organization_id="test_org_id",
            display_name="environment",
            type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
            created_date=to_pb_timestamp(now),
            created_by_user_id="user1",
            modified_date=to_pb_timestamp(now),
            modified_by_user_id="user1",
        )
        key = ResourceAttributeKey._from_proto(proto, None)

        with pytest.raises(AttributeError, match="Sift client not set"):
            _ = key.client


class TestResourceAttributeEnumValueCreate:
    """Unit tests for ResourceAttributeEnumValueCreate model."""

    def test_resource_attribute_enum_value_create_basic(self):
        """Test basic ResourceAttributeEnumValueCreate instantiation."""
        create = ResourceAttributeEnumValueCreate(
            resource_attribute_key_id="test_key_id", display_name="production"
        )

        assert create.resource_attribute_key_id == "test_key_id"
        assert create.display_name == "production"

    def test_resource_attribute_enum_value_create_to_proto(self):
        """Test that ResourceAttributeEnumValueCreate converts to proto correctly."""
        create = ResourceAttributeEnumValueCreate(
            resource_attribute_key_id="test_key_id",
            display_name="production",
            description="Production environment",
        )
        proto = create.to_proto()

        assert proto.resource_attribute_key_id == "test_key_id"
        assert proto.display_name == "production"
        assert proto.description == "Production environment"


class TestResourceAttributeEnumValueUpdate:
    """Unit tests for ResourceAttributeEnumValueUpdate model."""

    def test_resource_attribute_enum_value_update_to_proto_with_mask(self):
        """Test that ResourceAttributeEnumValueUpdate converts to proto with field mask correctly."""
        update = ResourceAttributeEnumValueUpdate(display_name="new_name")
        update.resource_id = "test_enum_value_id"
        proto, mask = update.to_proto_with_mask()

        assert proto.resource_attribute_enum_value_id == "test_enum_value_id"
        assert proto.display_name == "new_name"
        assert "display_name" in mask.paths


class TestResourceAttributeEnumValue:
    """Unit tests for ResourceAttributeEnumValue model."""

    def test_resource_attribute_enum_value_properties(self, mock_resource_attribute_enum_value):
        """Test that ResourceAttributeEnumValue properties are accessible."""
        assert mock_resource_attribute_enum_value.id_ == "test_enum_value_id"
        assert mock_resource_attribute_enum_value.resource_attribute_key_id == "test_key_id"
        assert mock_resource_attribute_enum_value.display_name == "production"
        assert mock_resource_attribute_enum_value.description == "Production environment"
        assert mock_resource_attribute_enum_value.created_by_user_id == "user1"
        assert mock_resource_attribute_enum_value.created_date is not None
        assert mock_resource_attribute_enum_value.created_date.tzinfo == timezone.utc


class TestResourceAttributeCreate:
    """Unit tests for ResourceAttributeCreate model."""

    def test_resource_attribute_create_enum_value(self):
        """Test ResourceAttributeCreate with enum value."""
        create = ResourceAttributeCreate(
            entity_id="asset123",
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
            resource_attribute_key_id="test_key_id",
            resource_attribute_enum_value_id="test_enum_value_id",
        )

        assert create.entity_id == "asset123"
        assert (
            create.entity_type == ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET
        )
        assert create.resource_attribute_enum_value_id == "test_enum_value_id"

    def test_resource_attribute_create_boolean_value(self):
        """Test ResourceAttributeCreate with boolean value."""
        create = ResourceAttributeCreate(
            entity_id="asset123",
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
            resource_attribute_key_id="test_key_id",
            boolean_value=True,
        )

        assert create.boolean_value is True

    def test_resource_attribute_create_to_proto(self):
        """Test that ResourceAttributeCreate converts to proto correctly."""
        create = ResourceAttributeCreate(
            entity_id="asset123",
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
            resource_attribute_key_id="test_key_id",
            resource_attribute_enum_value_id="test_enum_value_id",
        )
        proto = create.to_proto()

        assert proto.entity.entity_id == "asset123"
        assert (
            proto.entity.entity_type
            == ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET
        )
        assert proto.resource_attribute_key_id == "test_key_id"
        assert proto.resource_attribute_enum_value_id == "test_enum_value_id"


class TestResourceAttribute:
    """Unit tests for ResourceAttribute model."""

    def test_resource_attribute_properties(self, mock_resource_attribute):
        """Test that ResourceAttribute properties are accessible."""
        assert mock_resource_attribute.id_ == "test_attr_id"
        assert mock_resource_attribute.entity_id == "asset123"
        assert (
            mock_resource_attribute.entity_type
            == ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET
        )
        assert mock_resource_attribute.resource_attribute_key_id == "test_key_id"
        assert mock_resource_attribute.resource_attribute_enum_value_id == "test_enum_value_id"
        assert mock_resource_attribute.created_by_user_id == "user1"
        assert mock_resource_attribute.created_date is not None
        assert mock_resource_attribute.created_date.tzinfo == timezone.utc

    def test_resource_attribute_from_proto_boolean_value(self, mock_client):
        """Test ResourceAttribute creation from proto with boolean value."""
        now = datetime.now(timezone.utc)
        entity = ResourceAttributeEntityIdentifier(
            entity_id="asset123",
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        )
        proto = ResourceAttributeProto(
            resource_attribute_id="test_attr_id",
            organization_id="test_org_id",
            entity=entity,
            resource_attribute_key_id="test_key_id",
            boolean_value=True,
            created_date=to_pb_timestamp(now),
            created_by_user_id="user1",
        )

        attr = ResourceAttribute._from_proto(proto, mock_client)

        assert attr.boolean_value is True
        assert attr.resource_attribute_enum_value_id is None
        assert attr.number_value is None

    def test_resource_attribute_without_client_raises_error(self):
        """Test that accessing client without setting it raises an error."""
        now = datetime.now(timezone.utc)
        entity = ResourceAttributeEntityIdentifier(
            entity_id="asset123",
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        )
        proto = ResourceAttributeProto(
            resource_attribute_id="test_attr_id",
            organization_id="test_org_id",
            entity=entity,
            resource_attribute_key_id="test_key_id",
            resource_attribute_enum_value_id="test_enum_value_id",
            created_date=to_pb_timestamp(now),
            created_by_user_id="user1",
        )
        attr = ResourceAttribute._from_proto(proto, None)

        with pytest.raises(AttributeError, match="Sift client not set"):
            _ = attr.client
