"""Tests for sift_types resource attribute models."""

from datetime import datetime, timezone

import pytest
from sift.resource_attribute.v1 import resource_attribute_pb2 as ra

from sift_client.sift_types.resource_attribute import (
    ResourceAttribute,
    ResourceAttributeEntity,
    ResourceAttributeEntityType,
    ResourceAttributeEnumValue,
    ResourceAttributeKey,
    ResourceAttributeKeyType,
)


def _key_proto(*, is_archived: bool = False) -> ra.ResourceAttributeKey:
    return ra.ResourceAttributeKey(
        resource_attribute_key_id="k1",
        organization_id="org1",
        display_name="licenses",
        description="license ids",
        type=ra.RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM,
        is_archived=is_archived,
    )


class TestResourceAttributeEntity:
    def test_for_channel_builds_proto(self):
        entity = ResourceAttributeEntity.for_channel("ch1")
        proto = entity._to_proto()
        assert proto.entity_id == "ch1"
        assert proto.entity_type == ra.RESOURCE_ATTRIBUTE_ENTITY_TYPE_CHANNEL

    def test_from_proto_round_trips_entity_type(self):
        proto = ra.ResourceAttributeEntityIdentifier(
            entity_id="a1", entity_type=ra.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET
        )
        entity = ResourceAttributeEntity._from_proto(proto)
        assert entity.entity_id == "a1"
        assert entity.entity_type == ResourceAttributeEntityType.ASSET


class TestResourceAttributeKey:
    def test_from_proto_maps_fields_and_renames_type(self):
        key = ResourceAttributeKey._from_proto(_key_proto())
        assert key.id_ == "k1"
        assert key.display_name == "licenses"
        assert key.key_type == ResourceAttributeKeyType.SET_OF_ENUM
        assert key.is_archived is False
        assert key.archived_date is None

    def test_from_proto_parses_archived_date_when_set(self):
        proto = _key_proto(is_archived=True)
        proto.archived_date.FromDatetime(datetime(2026, 1, 1, tzinfo=timezone.utc))
        key = ResourceAttributeKey._from_proto(proto)
        assert key.is_archived is True
        assert key.archived_date == datetime(2026, 1, 1, tzinfo=timezone.utc)

    def test_str_is_display_name(self):
        assert str(ResourceAttributeKey._from_proto(_key_proto())) == "licenses"


class TestResourceAttribute:
    def test_from_proto_flattens_enum_value_oneof(self):
        proto = ra.ResourceAttribute(
            resource_attribute_id="a1",
            resource_attribute_key_id="k1",
            resource_attribute_enum_value_id="ev1",
            entity=ra.ResourceAttributeEntityIdentifier(
                entity_id="ch1", entity_type=ra.RESOURCE_ATTRIBUTE_ENTITY_TYPE_CHANNEL
            ),
        )
        attr = ResourceAttribute._from_proto(proto)
        assert attr.enum_value_id == "ev1"
        assert attr.boolean_value is None
        assert attr.number_value is None
        assert attr.entity is not None
        assert attr.entity.entity_type == ResourceAttributeEntityType.CHANNEL

    def test_from_proto_flattens_boolean_oneof(self):
        proto = ra.ResourceAttribute(
            resource_attribute_id="a1", resource_attribute_key_id="k1", boolean_value=True
        )
        attr = ResourceAttribute._from_proto(proto)
        assert attr.boolean_value is True
        assert attr.enum_value_id is None

    def test_from_proto_without_entity_is_none(self):
        proto = ra.ResourceAttribute(resource_attribute_id="a1", resource_attribute_key_id="k1")
        attr = ResourceAttribute._from_proto(proto)
        assert attr.entity is None

    def test_apply_client_cascades_to_nested_key_and_enum_value(self, mock_client):
        proto = ra.ResourceAttribute(
            resource_attribute_id="a1",
            resource_attribute_key_id="k1",
            resource_attribute_enum_value_id="ev1",
            key=ra.ResourceAttributeKey(resource_attribute_key_id="k1", display_name="licenses"),
            enum_value_details=ra.ResourceAttributeEnumValue(
                resource_attribute_enum_value_id="ev1", display_name="LIC_A"
            ),
        )
        attr = ResourceAttribute._from_proto(proto)
        attr._apply_client_to_instance(mock_client)

        # Nested objects must also carry the client so their convenience methods work.
        assert attr.key is not None
        assert attr.key.client is mock_client
        assert attr.enum_value is not None
        assert attr.enum_value.client is mock_client

    def test_archive_refreshes_and_returns_self(self, mock_client):
        proto = ra.ResourceAttribute(resource_attribute_id="a1", resource_attribute_key_id="k1")
        attr = ResourceAttribute._from_proto(proto)
        attr._apply_client_to_instance(mock_client)
        archived = ra.ResourceAttribute(
            resource_attribute_id="a1", resource_attribute_key_id="k1", is_archived=True
        )
        mock_client.access_control.resource_attributes.get_assignment.return_value = (
            ResourceAttribute._from_proto(archived)
        )

        result = attr.archive()

        mock_client.access_control.resource_attributes.archive_assignments.assert_called_once_with(
            [attr]
        )
        assert result is attr
        assert attr.is_archived is True


class TestResourceAttributeKeyConvenience:
    def test_archive_delegates_and_updates_in_place(self, mock_client):
        key = ResourceAttributeKey._from_proto(_key_proto())
        key._apply_client_to_instance(mock_client)
        archived = ResourceAttributeKey._from_proto(_key_proto(is_archived=True))
        mock_client.access_control.resource_attributes.archive_key.return_value = archived

        result = key.archive()

        mock_client.access_control.resource_attributes.archive_key.assert_called_once_with(key)
        assert result is key
        assert key.is_archived is True

    def test_assign_to_delegates(self, mock_client):
        key = ResourceAttributeKey._from_proto(_key_proto())
        key._apply_client_to_instance(mock_client)
        mock_client.access_control.resource_attributes.assign.return_value = ["sentinel"]

        resource = ResourceAttributeEntity.for_channel("ch1")

        result = key.assign_to([resource], value=["LIC_A"])

        mock_client.access_control.resource_attributes.assign.assert_called_once_with(
            key, [resource], value=["LIC_A"]
        )
        assert result == ["sentinel"]

    def test_check_archive_impact_delegates(self, mock_client):
        key = ResourceAttributeKey._from_proto(_key_proto())
        key._apply_client_to_instance(mock_client)
        mock_client.access_control.resource_attributes.check_key_archive_impact.return_value = 7

        assert key.check_archive_impact() == 7

    def test_client_required_for_convenience_methods(self):
        key = ResourceAttributeKey._from_proto(_key_proto())
        with pytest.raises(AttributeError, match="Sift client not set"):
            key.check_archive_impact()


class TestResourceAttributeEnumValue:
    def test_archive_returns_migrated_count(self, mock_client):
        proto = ra.ResourceAttributeEnumValue(
            resource_attribute_enum_value_id="ev1",
            resource_attribute_key_id="k1",
            display_name="LIC_A",
        )
        value = ResourceAttributeEnumValue._from_proto(proto)
        value._apply_client_to_instance(mock_client)
        mock_client.access_control.resource_attributes.archive_enum_value.return_value = 3

        migrated = value.archive(replacement="ev2")

        mock_client.access_control.resource_attributes.archive_enum_value.assert_called_once_with(
            value, replacement="ev2"
        )
        assert migrated == 3
