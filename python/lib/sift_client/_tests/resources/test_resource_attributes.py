"""Unit tests for the resource attributes high-level API.

These mock the low-level client and exercise the resource-layer orchestration
(get-or-create de-duplication, value/entity resolution, batching). They are not
integration tests and run without a backend.
"""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.resource_attribute.v1 import resource_attribute_pb2 as ra

from sift_client.resources.access_control.resource_attributes import ResourceAttributesAPIAsync
from sift_client.sift_types.resource_attribute import (
    ResourceAttributeEntity,
    ResourceAttributeEntityType,
    ResourceAttributeEnumValue,
    ResourceAttributeKey,
    ResourceAttributeKeyType,
)


def _api() -> ResourceAttributesAPIAsync:
    client = MagicMock()
    api = ResourceAttributesAPIAsync(client)
    api._low_level_client = MagicMock()
    return api


def _key(key_type=ra.RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM) -> ResourceAttributeKey:
    return ResourceAttributeKey._from_proto(
        ra.ResourceAttributeKey(
            resource_attribute_key_id="k1", display_name="licenses", type=key_type
        )
    )


def _enum(eid: str, name: str) -> ResourceAttributeEnumValue:
    return ResourceAttributeEnumValue._from_proto(
        ra.ResourceAttributeEnumValue(
            resource_attribute_enum_value_id=eid, resource_attribute_key_id="k1", display_name=name
        )
    )


class TestGetOrCreateKey:
    @pytest.mark.asyncio
    async def test_returns_existing_without_creating(self):
        api = _api()
        api._low_level_client.list_all_keys = AsyncMock(return_value=[_key()])
        api._low_level_client.create_key = AsyncMock(side_effect=AssertionError("must not create"))

        key = await api.get_or_create_key("licenses", ResourceAttributeKeyType.SET_OF_ENUM)

        assert key.id_ == "k1"

    @pytest.mark.asyncio
    async def test_creates_when_missing(self):
        api = _api()
        api._low_level_client.list_all_keys = AsyncMock(return_value=[])
        api._low_level_client.create_key = AsyncMock(return_value=_key())

        key = await api.get_or_create_key("licenses", ResourceAttributeKeyType.SET_OF_ENUM)

        api._low_level_client.create_key.assert_awaited_once()
        assert key.id_ == "k1"


class TestGetOrCreateEnumValues:
    @pytest.mark.asyncio
    async def test_only_creates_missing_and_preserves_order(self):
        api = _api()
        api._low_level_client.list_all_enum_values = AsyncMock(return_value=[_enum("e_a", "LIC_A")])
        api._low_level_client.create_enum_value = AsyncMock(return_value=_enum("e_b", "LIC_B"))

        values = await api.get_or_create_enum_values(_key(), ["LIC_A", "LIC_B"])

        assert [v.display_name for v in values] == ["LIC_A", "LIC_B"]
        assert api._low_level_client.create_enum_value.await_count == 1


class TestAssignValueResolution:
    @pytest.mark.asyncio
    async def test_set_of_enum_uses_id_list(self):
        api = _api()
        api._low_level_client.batch_create_resource_attributes = AsyncMock(return_value=[])

        await api.assign(
            _key(ra.RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM),
            [ResourceAttributeEntity.for_channel("ch1")],
            value=[_enum("e_a", "LIC_A"), "e_b"],
        )

        kwargs = api._low_level_client.batch_create_resource_attributes.call_args.kwargs
        assert kwargs["enum_value_ids"] == ["e_a", "e_b"]

    @pytest.mark.asyncio
    async def test_boolean_key_requires_bool(self):
        api = _api()
        with pytest.raises(TypeError, match="BOOLEAN keys require a bool"):
            await api.assign(
                _key(ra.RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN),
                [ResourceAttributeEntity.for_channel("ch1")],
                value="not-a-bool",
            )

    @pytest.mark.asyncio
    async def test_enum_key_rejects_multiple_values(self):
        api = _api()
        with pytest.raises(ValueError, match="exactly one enum value"):
            await api.assign(
                _key(ra.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM),
                [ResourceAttributeEntity.for_channel("ch1")],
                value=["e_a", "e_b"],
            )

    @pytest.mark.asyncio
    async def test_resolves_domain_objects_to_entities(self):
        from sift_client.sift_types.asset import Asset

        api = _api()
        api._low_level_client.batch_create_resource_attributes = AsyncMock(return_value=[])
        asset = Asset._from_proto(_asset_proto())

        await api.assign(_key(), [asset], value=["e_a"])

        kwargs = api._low_level_client.batch_create_resource_attributes.call_args.kwargs
        entities = kwargs["entities"]
        assert entities[0].entity_type == ResourceAttributeEntity.for_asset("a1").entity_type
        assert entities[0].entity_id == "a1"

    @pytest.mark.asyncio
    async def test_rejects_resource_entity_types_outside_current_supported_targets(self):
        api = _api()
        api._low_level_client.batch_create_resource_attributes = AsyncMock(return_value=[])
        unsupported = ResourceAttributeEntity(
            entity_id="unknown",
            entity_type=ResourceAttributeEntityType.UNSPECIFIED,
        )

        with pytest.raises(ValueError, match="currently support assets, channels, and runs"):
            await api.assign(_key(), [unsupported], value=["e_a"])

        api._low_level_client.batch_create_resource_attributes.assert_not_called()

    @pytest.mark.asyncio
    async def test_rejects_resource_assignment_filters_outside_current_supported_targets(self):
        api = _api()
        api._low_level_client.list_all_resource_attributes_by_entity = AsyncMock(return_value=[])
        unsupported = ResourceAttributeEntity(
            entity_id="unknown",
            entity_type=ResourceAttributeEntityType.UNSPECIFIED,
        )

        with pytest.raises(ValueError, match="currently support assets, channels, and runs"):
            await api.list_assignments(resource=unsupported)

        api._low_level_client.list_all_resource_attributes_by_entity.assert_not_called()


def _asset_proto():
    from sift.assets.v1.assets_pb2 import Asset as AssetProto

    proto = AssetProto(asset_id="a1", name="asset")
    return proto
