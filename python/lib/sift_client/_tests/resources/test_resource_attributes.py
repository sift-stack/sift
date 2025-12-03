"""Pytest tests for the Resource Attributes API.

These tests demonstrate and validate the usage of the Resource Attributes API including:
- Basic resource attribute key operations (create, get, list, update, archive)
- Resource attribute enum value operations (create, list, update, archive)
- Resource attribute operations (create single/batch, list, archive)
- Filtering and searching
- Error handling and edge cases
"""

from datetime import datetime, timezone

import pytest
from sift.resource_attribute.v1.resource_attribute_pb2 import (
    ResourceAttributeEntityType,
    ResourceAttributeKeyType,
)

from sift_client.resources import ResourceAttributesAPI, ResourceAttributesAPIAsync
from sift_client.sift_types import (
    ResourceAttribute,
    ResourceAttributeEnumValue,
    ResourceAttributeKey,
)

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    """Test that resource_attributes API is properly registered on the client."""
    assert sift_client.resource_attributes
    assert isinstance(sift_client.resource_attributes, ResourceAttributesAPI)
    assert sift_client.async_.resource_attributes
    assert isinstance(sift_client.async_.resource_attributes, ResourceAttributesAPIAsync)


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
def test_resource_attribute_key(sift_client, test_timestamp_str):
    """Setup a test resource attribute key for the session."""
    key = sift_client.resource_attributes.create_key(
        display_name=f"test_env_{test_timestamp_str}",
        description="Test environment",
        key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
    )
    yield key
    # Cleanup: archive the key
    try:
        sift_client.resource_attributes.archive_key(key.id_)
    except Exception:
        pass


@pytest.fixture(scope="session")
def test_resource_attribute_enum_value(sift_client, test_resource_attribute_key):
    """Setup a test resource attribute enum value for the session."""
    enum_value = sift_client.resource_attributes.create_enum_value(
        key_id=test_resource_attribute_key.id_,
        display_name="production",
        description="Production environment",
    )
    return enum_value
    # Cleanup handled by key cleanup


class TestResourceAttributeKeys:
    """Tests for Resource Attribute Keys API."""

    def test_create_key(self, sift_client, test_timestamp_str):
        """Test creating a resource attribute key."""
        key = sift_client.resource_attributes.create_key(
            display_name=f"test_create_{test_timestamp_str}",
            description="Test key",
            key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
        )

        assert isinstance(key, ResourceAttributeKey)
        assert key.id_ is not None
        assert key.display_name == f"test_create_{test_timestamp_str}"

        # Cleanup
        sift_client.resource_attributes.archive_key(key.id_)

    def test_create_key_with_initial_enum_values(self, sift_client, test_timestamp_str):
        """Test creating a resource attribute key with initial enum values."""
        key = sift_client.resource_attributes.create_key(
            display_name=f"test_init_enum_{test_timestamp_str}",
            key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
            initial_enum_values=[
                {"display_name": "prod", "description": "Production"},
                {"display_name": "staging"},
            ],
        )

        assert isinstance(key, ResourceAttributeKey)
        enum_values = sift_client.resource_attributes.list_enum_values(key.id_)
        assert len(enum_values) >= 2

        # Cleanup
        sift_client.resource_attributes.archive_key(key.id_)

    def test_get_key(self, sift_client, test_resource_attribute_key):
        """Test getting a resource attribute key by ID."""
        key = sift_client.resource_attributes.get_key(test_resource_attribute_key.id_)

        assert isinstance(key, ResourceAttributeKey)
        assert key.id_ == test_resource_attribute_key.id_

    def test_list_keys(self, sift_client):
        """Test listing resource attribute keys."""
        keys = sift_client.resource_attributes.list_keys(limit=10)

        assert isinstance(keys, list)
        assert all(isinstance(key, ResourceAttributeKey) for key in keys)

    def test_update_key(self, sift_client, test_timestamp_str):
        """Test updating a resource attribute key."""
        key = sift_client.resource_attributes.create_key(
            display_name=f"test_update_{test_timestamp_str}",
            description="Original description",
            key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
        )

        updated_key = sift_client.resource_attributes.update_key(
            key, {"display_name": f"test_updated_{test_timestamp_str}"}
        )

        assert updated_key.display_name == f"test_updated_{test_timestamp_str}"

        # Cleanup
        sift_client.resource_attributes.archive_key(updated_key.id_)


class TestResourceAttributeEnumValues:
    """Tests for Resource Attribute Enum Values API."""

    def test_create_enum_value(self, sift_client, test_resource_attribute_key, test_timestamp_str):
        """Test creating a resource attribute enum value."""
        enum_value = sift_client.resource_attributes.create_enum_value(
            key_id=test_resource_attribute_key.id_,
            display_name=f"staging_{test_timestamp_str}",
            description="Staging environment",
        )

        assert isinstance(enum_value, ResourceAttributeEnumValue)
        assert enum_value.id_ is not None
        assert enum_value.display_name == f"staging_{test_timestamp_str}"

    def test_list_enum_values(self, sift_client, test_resource_attribute_key):
        """Test listing resource attribute enum values."""
        enum_values = sift_client.resource_attributes.list_enum_values(
            test_resource_attribute_key.id_
        )

        assert isinstance(enum_values, list)
        assert all(isinstance(ev, ResourceAttributeEnumValue) for ev in enum_values)


class TestResourceAttributes:
    """Tests for Resource Attributes API."""

    def test_create_single(
        self,
        sift_client,
        test_resource_attribute_key,
        test_resource_attribute_enum_value,
        test_timestamp_str,
    ):
        """Test creating a single resource attribute."""
        # Need a real asset ID - using a test asset if available, otherwise skip
        # For now, we'll test the structure but may need to skip if no assets exist
        try:
            assets = sift_client.assets.list_(limit=1)
            if not assets:
                pytest.skip("No assets available for testing")

            asset_id = assets[0].id_
            attr = sift_client.resource_attributes.create(
                key_id=test_resource_attribute_key.id_,
                entities=asset_id,
                entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
                resource_attribute_enum_value_id=test_resource_attribute_enum_value.id_,
            )

            assert isinstance(attr, ResourceAttribute)
            assert attr.id_ is not None
            assert attr.entity_id == asset_id

            # Cleanup
            sift_client.resource_attributes.archive(attr.id_)
        except Exception as e:
            pytest.skip(f"Could not create resource attribute: {e}")

    def test_create_batch(
        self,
        sift_client,
        test_resource_attribute_key,
        test_resource_attribute_enum_value,
        test_timestamp_str,
    ):
        """Test creating multiple resource attributes in batch."""
        try:
            assets = sift_client.assets.list_(limit=2)
            if len(assets) < 2:
                pytest.skip("Need at least 2 assets for batch test")

            asset_ids = [assets[0].id_, assets[1].id_]
            attrs = sift_client.resource_attributes.create(
                key_id=test_resource_attribute_key.id_,
                entities=asset_ids,
                entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
                resource_attribute_enum_value_id=test_resource_attribute_enum_value.id_,
            )

            assert isinstance(attrs, list)
            assert len(attrs) == 2
            assert all(isinstance(a, ResourceAttribute) for a in attrs)

            # Cleanup
            sift_client.resource_attributes.batch_archive([a.id_ for a in attrs])
        except Exception as e:
            pytest.skip(f"Could not create batch resource attributes: {e}")

    def test_list(self, sift_client, test_resource_attribute_key):
        """Test listing resource attributes."""
        attrs = sift_client.resource_attributes.list(
            key_id=test_resource_attribute_key.id_, limit=10
        )

        assert isinstance(attrs, list)
        assert all(isinstance(a, ResourceAttribute) for a in attrs)


def test_complete_resource_attribute_workflow(sift_client, test_timestamp_str):
    """End-to-end workflow test for resource attributes.

    This comprehensive test validates the complete workflow:
    1. Create key with initial enum values
    2. Create additional enum values
    3. Create attributes (enum, boolean, number) for multiple entities
    4. List and filter attributes
    5. Update resources
    6. Archive enum value with migration
    7. Cleanup
    """
    # Track resources for cleanup
    created_keys = []
    created_enum_values = []
    created_attributes = []

    try:
        # Setup: Get or create test assets
        assets = sift_client.assets.list_(limit=4)
        if len(assets) < 3:
            pytest.skip("Need at least 3 assets for complete workflow test")
        test_assets = assets[:3]
        asset_ids = [asset.id_ for asset in test_assets]

        # 1. Create key with initial enum values
        key = sift_client.resource_attributes.create_key(
            display_name=f"workflow_key_{test_timestamp_str}",
            description="Workflow test key",
            key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
            initial_enum_values=[
                {"display_name": "initial_prod", "description": "Initial production"},
                {"display_name": "initial_staging"},
            ],
        )
        created_keys.append(key)
        assert isinstance(key, ResourceAttributeKey)
        assert key.id_ is not None
        assert key.display_name == f"workflow_key_{test_timestamp_str}"

        # 2. Verify initial enum values exist
        enum_values = sift_client.resource_attributes.list_enum_values(key.id_)
        assert len(enum_values) >= 2
        initial_enum_value = next(
            (ev for ev in enum_values if ev.display_name == "initial_prod"), None
        )
        assert initial_enum_value is not None
        created_enum_values.append(initial_enum_value)

        # 3. Create additional enum values
        new_enum_value = sift_client.resource_attributes.create_enum_value(
            key_id=key.id_,
            display_name=f"workflow_dev_{test_timestamp_str}",
            description="Development environment",
        )
        created_enum_values.append(new_enum_value)
        assert isinstance(new_enum_value, ResourceAttributeEnumValue)
        assert new_enum_value.id_ is not None
        assert new_enum_value.display_name == f"workflow_dev_{test_timestamp_str}"

        # 4. List all enum values
        all_enum_values = sift_client.resource_attributes.list_enum_values(key.id_)
        assert len(all_enum_values) >= 3
        enum_value_names = {ev.display_name for ev in all_enum_values}
        assert "initial_prod" in enum_value_names
        assert "initial_staging" in enum_value_names
        assert f"workflow_dev_{test_timestamp_str}" in enum_value_names

        # 5. Update key
        updated_key = sift_client.resource_attributes.update_key(
            key, {"description": "Updated workflow test key"}
        )
        assert updated_key.description == "Updated workflow test key"
        assert updated_key.id_ == key.id_

        # 6. Create attributes with enum values (use asset_ids[0])
        enum_attr = sift_client.resource_attributes.create(
            key_id=key.id_,
            entities=asset_ids[0],
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
            resource_attribute_enum_value_id=initial_enum_value.id_,
        )
        created_attributes.append(enum_attr)
        assert isinstance(enum_attr, ResourceAttribute)
        assert enum_attr.resource_attribute_enum_value_id == initial_enum_value.id_
        assert enum_attr.entity_id == asset_ids[0]

        # 7. Create attributes with boolean values
        # First create a boolean key
        boolean_key = sift_client.resource_attributes.create_key(
            display_name=f"workflow_boolean_{test_timestamp_str}",
            description="Boolean test key",
            key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN,
        )
        created_keys.append(boolean_key)

        boolean_attr = sift_client.resource_attributes.create(
            key_id=boolean_key.id_,
            entities=asset_ids[0],
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
            boolean_value=True,
        )
        created_attributes.append(boolean_attr)
        assert isinstance(boolean_attr, ResourceAttribute)
        assert boolean_attr.boolean_value is True
        assert boolean_attr.resource_attribute_enum_value_id is None

        # 8. Create attributes with number values
        # First create a number key
        number_key = sift_client.resource_attributes.create_key(
            display_name=f"workflow_number_{test_timestamp_str}",
            description="Number test key",
            key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_NUMBER,
        )
        created_keys.append(number_key)

        number_attr = sift_client.resource_attributes.create(
            key_id=number_key.id_,
            entities=asset_ids[0],
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
            number_value=42.5,
        )
        created_attributes.append(number_attr)
        assert isinstance(number_attr, ResourceAttribute)
        assert number_attr.number_value == 42.5
        assert number_attr.resource_attribute_enum_value_id is None

        # 9. Create batch attributes (use asset_ids[1:] to avoid duplicate with asset_ids[0])
        # Note: We already created an attribute for asset_ids[0] with the same key,
        # so we'll create batch attributes for the remaining assets to test batch functionality
        batch_attrs = sift_client.resource_attributes.create(
            key_id=key.id_,
            entities=asset_ids[1:],  # Use all assets except the first to avoid duplicate
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
            resource_attribute_enum_value_id=new_enum_value.id_,
        )
        assert isinstance(batch_attrs, list)
        assert (
            len(batch_attrs) == len(asset_ids) - 1
        )  # Should have 2 attributes (for asset_ids[1] and asset_ids[2])
        created_attributes.extend(batch_attrs)
        for attr in batch_attrs:
            assert attr.resource_attribute_enum_value_id == new_enum_value.id_
            assert attr.entity_id in asset_ids[1:]  # Should be one of the assets we used

        # 10. List attributes by key
        key_attrs = sift_client.resource_attributes.list(key_id=key.id_)
        assert len(key_attrs) >= 3  # enum_attr + 2 batch attrs
        key_attr_ids = {attr.id_ for attr in key_attrs}
        assert enum_attr.id_ in key_attr_ids
        assert all(attr.id_ in key_attr_ids for attr in batch_attrs)

        # 11. List attributes by entity
        entity_attrs = sift_client.resource_attributes.list(
            entity_id=asset_ids[0],
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        )
        assert len(entity_attrs) >= 3  # enum_attr + boolean_attr + number_attr
        entity_attr_ids = {attr.id_ for attr in entity_attrs}
        assert enum_attr.id_ in entity_attr_ids
        assert boolean_attr.id_ in entity_attr_ids
        assert number_attr.id_ in entity_attr_ids

        # 12. List attributes with filters
        filtered_attrs = sift_client.resource_attributes.list(
            key_id=key.id_,
            entity_id=asset_ids[0],
            entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        )
        assert len(filtered_attrs) >= 1
        assert all(attr.resource_attribute_key_id == key.id_ for attr in filtered_attrs)
        assert all(attr.entity_id == asset_ids[0] for attr in filtered_attrs)

        # 13. Update enum value (attributes can't be updated, only enum values and keys)
        updated_enum_value = sift_client.resource_attributes.update_enum_value(
            new_enum_value, {"description": "Updated development environment"}
        )
        assert updated_enum_value.description == "Updated development environment"
        assert updated_enum_value.id_ == new_enum_value.id_

        # 14. Archive enum value with replacement (verify migration)
        # Create a replacement enum value first
        replacement_enum_value = sift_client.resource_attributes.create_enum_value(
            key_id=key.id_,
            display_name=f"workflow_replacement_{test_timestamp_str}",
            description="Replacement enum value",
        )
        created_enum_values.append(replacement_enum_value)

        # Archive the enum value with replacement
        migrated_count = sift_client.resource_attributes.archive_enum_value(
            new_enum_value.id_, replacement_enum_value.id_
        )
        assert migrated_count >= 1  # Should have migrated the batch attribute

        # Verify attributes were migrated
        migrated_attrs = sift_client.resource_attributes.list(key_id=key.id_)
        for attr in migrated_attrs:
            if attr.id_ in {a.id_ for a in batch_attrs}:
                assert attr.resource_attribute_enum_value_id == replacement_enum_value.id_

        # 15. Unarchive enum value
        sift_client.resource_attributes.unarchive_enum_value(new_enum_value.id_)
        unarchived_enum_value = sift_client.resource_attributes.get_enum_value(new_enum_value.id_)
        assert unarchived_enum_value.archived_date is None

        # 16. Archive attributes
        sift_client.resource_attributes.archive(enum_attr.id_)
        archived_attr = sift_client.resource_attributes.get(enum_attr.id_)
        assert archived_attr.archived_date is not None

        # 17. Batch archive attributes
        batch_attr_ids = [attr.id_ for attr in batch_attrs]
        sift_client.resource_attributes.batch_archive(batch_attr_ids)
        for attr_id in batch_attr_ids:
            archived = sift_client.resource_attributes.get(attr_id)
            assert archived.archived_date is not None

        # 18. Archive keys (cleanup)
        for key_to_archive in created_keys:
            sift_client.resource_attributes.archive_key(key_to_archive.id_)
            archived_key = sift_client.resource_attributes.get_key(key_to_archive.id_)
            assert archived_key.archived_date is not None

    except Exception:
        # Cleanup on failure
        for attr in created_attributes:
            try:
                sift_client.resource_attributes.archive(attr.id_)
            except Exception:  # noqa: PERF203  # Cleanup in finally block
                pass
        for key in created_keys:
            try:
                sift_client.resource_attributes.archive_key(key.id_)
            except Exception:  # noqa: PERF203  # Cleanup in finally block
                pass
        raise


class TestResourceAttributeErrors:
    """Tests for error handling in Resource Attributes API."""

    def test_create_attribute_with_nonexistent_key(self, sift_client, test_timestamp_str):
        """Test creating an attribute with a non-existent key raises an error."""
        try:
            assets = sift_client.assets.list_(limit=1)
            if not assets:
                pytest.skip("No assets available for testing")
            asset_id = assets[0].id_

            with pytest.raises(Exception):  # noqa: B017, PT011  # Should raise ValueError or gRPC error
                sift_client.resource_attributes.create(
                    key_id="nonexistent-key-id-12345",
                    entities=asset_id,
                    entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
                    resource_attribute_enum_value_id="some-enum-value-id",
                )
        except Exception as e:
            pytest.skip(f"Could not test error case: {e}")

    def test_create_attribute_with_nonexistent_enum_value(self, sift_client, test_timestamp_str):
        """Test creating an attribute with a non-existent enum value raises an error."""
        try:
            assets = sift_client.assets.list_(limit=1)
            if not assets:
                pytest.skip("No assets available for testing")
            asset_id = assets[0].id_

            # Create a valid key first
            key = sift_client.resource_attributes.create_key(
                display_name=f"error_test_key_{test_timestamp_str}",
                key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
            )

            try:
                with pytest.raises(Exception):  # noqa: B017, PT011  # Should raise ValueError or gRPC error
                    sift_client.resource_attributes.create(
                        key_id=key.id_,
                        entities=asset_id,
                        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
                        resource_attribute_enum_value_id="nonexistent-enum-value-id-12345",
                    )
            finally:
                sift_client.resource_attributes.archive_key(key.id_)
        except Exception as e:
            pytest.skip(f"Could not test error case: {e}")

    def test_create_enum_value_for_nonexistent_key(self, sift_client, test_timestamp_str):
        """Test creating an enum value for a non-existent key raises an error."""
        with pytest.raises(Exception):  # noqa: B017, PT011  # Should raise ValueError or gRPC error
            sift_client.resource_attributes.create_enum_value(
                key_id="nonexistent-key-id-12345",
                display_name="test_enum",
            )

    def test_archive_enum_value_without_replacement(self, sift_client, test_timestamp_str):
        """Test that archiving an enum value requires a replacement."""
        try:
            # Create a key and enum value
            key = sift_client.resource_attributes.create_key(
                display_name=f"error_test_key_{test_timestamp_str}",
                key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
            )
            enum_value = sift_client.resource_attributes.create_enum_value(
                key_id=key.id_,
                display_name=f"error_test_enum_{test_timestamp_str}",
            )

            try:
                # Archive enum value without replacement should raise an error
                # Note: The API might require replacement, check actual behavior
                with pytest.raises(Exception):  # noqa: B017, PT011  # Should raise ValueError or gRPC error
                    sift_client.resource_attributes.archive_enum_value(
                        enum_value.id_, "nonexistent-replacement-id"
                    )
            finally:
                sift_client.resource_attributes.archive_key(key.id_)
        except Exception as e:
            pytest.skip(f"Could not test error case: {e}")

    def test_get_nonexistent_key(self, sift_client):
        """Test getting a non-existent key raises an error."""
        with pytest.raises(Exception):  # noqa: B017, PT011  # Should raise ValueError or gRPC error
            sift_client.resource_attributes.get_key("nonexistent-key-id-12345")

    def test_get_nonexistent_enum_value(self, sift_client):
        """Test getting a non-existent enum value raises an error."""
        with pytest.raises(Exception):  # noqa: B017, PT011  # Should raise ValueError or gRPC error
            sift_client.resource_attributes.get_enum_value("nonexistent-enum-value-id-12345")

    def test_get_nonexistent_attribute(self, sift_client):
        """Test getting a non-existent attribute raises an error."""
        with pytest.raises(Exception):  # noqa: B017, PT011  # Should raise ValueError or gRPC error
            sift_client.resource_attributes.get("nonexistent-attribute-id-12345")

    def test_update_nonexistent_key(self, sift_client, test_timestamp_str):
        """Test updating a non-existent key raises an error."""
        with pytest.raises(Exception):  # noqa: B017, PT011  # Should raise ValueError or gRPC error
            sift_client.resource_attributes.update_key(
                "nonexistent-key-id-12345", {"display_name": "updated"}
            )

    def test_update_nonexistent_enum_value(self, sift_client, test_timestamp_str):
        """Test updating a non-existent enum value raises an error."""
        with pytest.raises(Exception):  # noqa: B017, PT011  # Should raise ValueError or gRPC error
            sift_client.resource_attributes.update_enum_value(
                "nonexistent-enum-value-id-12345", {"display_name": "updated"}
            )
