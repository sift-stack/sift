"""Unit tests for the resource attributes high-level API.

These mock the low-level client and exercise the resource-layer orchestration
(get-or-create de-duplication, value/entity resolution, batching) across the
nested keys/enum_values/assignments sub-resources. They are not integration
tests and run without a backend.
"""

from types import SimpleNamespace
from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.resource_attribute.v1 import resource_attribute_pb2 as ra

from sift_client.resources.access_control.resource_attributes import ResourceAttributesAPIAsync
from sift_client.sift_types.resource_attribute import (
    ResourceAttributeEntity,
    ResourceAttributeEntityType,
    ResourceAttributeEnumValue,
    ResourceAttributeKey,
    ResourceAttributeKeyUpdate,
    ResourceAttributeValueType,
)


def _api() -> ResourceAttributesAPIAsync:
    client = MagicMock()
    api = ResourceAttributesAPIAsync(client)
    api.keys._low_level_client = MagicMock()
    api.enum_values._low_level_client = MagicMock()
    api.assignments._low_level_client = MagicMock()
    return api


def _key(value_type=ra.RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM) -> ResourceAttributeKey:
    return ResourceAttributeKey._from_proto(
        ra.ResourceAttributeKey(
            resource_attribute_key_id="k1", display_name="licenses", type=value_type
        )
    )


def _enum(eid: str, name: str) -> ResourceAttributeEnumValue:
    return ResourceAttributeEnumValue._from_proto(
        ra.ResourceAttributeEnumValue(
            resource_attribute_enum_value_id=eid, resource_attribute_key_id="k1", display_name=name
        )
    )


def _matched_resource(resource_id: str):
    return SimpleNamespace(_id_or_error=resource_id)


class TestKeysGetOrCreate:
    @pytest.mark.asyncio
    async def test_returns_existing_without_creating(self):
        api = _api()
        api.keys._low_level_client.list_all_keys = AsyncMock(return_value=[_key()])
        api.keys._low_level_client.create_key = AsyncMock(
            side_effect=AssertionError("must not create")
        )

        key = await api.keys.get_or_create("licenses", ResourceAttributeValueType.SET_OF_ENUM)

        assert key.id_ == "k1"

    @pytest.mark.asyncio
    async def test_creates_when_missing(self):
        api = _api()
        api.keys._low_level_client.list_all_keys = AsyncMock(return_value=[])
        api.keys._low_level_client.create_key = AsyncMock(return_value=_key())

        key = await api.keys.get_or_create("licenses", ResourceAttributeValueType.SET_OF_ENUM)

        api.keys._low_level_client.create_key.assert_awaited_once()
        assert key.id_ == "k1"


class TestEnumValuesGetOrCreate:
    @pytest.mark.asyncio
    async def test_only_creates_missing_and_preserves_order(self):
        api = _api()
        api.enum_values._low_level_client.list_all_enum_values = AsyncMock(
            return_value=[_enum("e_a", "LIC_A")]
        )
        api.enum_values._low_level_client.create_enum_value = AsyncMock(
            return_value=_enum("e_b", "LIC_B")
        )

        values = await api.enum_values.get_or_create(_key(), ["LIC_A", "LIC_B"])

        assert [v.display_name for v in values] == ["LIC_A", "LIC_B"]
        assert api.enum_values._low_level_client.create_enum_value.await_count == 1


class TestAssignmentsCreateValueResolution:
    @pytest.mark.asyncio
    async def test_set_of_enum_uses_id_list(self):
        api = _api()
        api.assignments._low_level_client.batch_create_resource_attributes = AsyncMock(
            return_value=[]
        )

        await api.assignments.create(
            _key(ra.RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM),
            [ResourceAttributeEntity.for_channel("ch1")],
            value=[_enum("e_a", "LIC_A"), "e_b"],
        )

        kwargs = api.assignments._low_level_client.batch_create_resource_attributes.call_args.kwargs
        assert kwargs["enum_value_ids"] == ["e_a", "e_b"]

    @pytest.mark.asyncio
    async def test_boolean_key_requires_bool(self):
        api = _api()
        with pytest.raises(TypeError, match="BOOLEAN keys require a bool"):
            await api.assignments.create(
                _key(ra.RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN),
                [ResourceAttributeEntity.for_channel("ch1")],
                value="not-a-bool",
            )

    @pytest.mark.asyncio
    async def test_enum_key_rejects_multiple_values(self):
        api = _api()
        with pytest.raises(ValueError, match="exactly one enum value"):
            await api.assignments.create(
                _key(ra.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM),
                [ResourceAttributeEntity.for_channel("ch1")],
                value=["e_a", "e_b"],
            )

    @pytest.mark.asyncio
    async def test_resolves_domain_objects_to_entities(self):
        from sift_client.sift_types.asset import Asset

        api = _api()
        api.assignments._low_level_client.batch_create_resource_attributes = AsyncMock(
            return_value=[]
        )
        asset = Asset._from_proto(_asset_proto())

        await api.assignments.create(_key(), [asset], value=["e_a"])

        kwargs = api.assignments._low_level_client.batch_create_resource_attributes.call_args.kwargs
        entities = kwargs["entities"]
        assert entities[0].entity_type == ResourceAttributeEntity.for_asset("a1").entity_type
        assert entities[0].entity_id == "a1"

    @pytest.mark.asyncio
    async def test_fetches_key_when_assigning_by_key_id(self):
        api = _api()
        api.assignments._low_level_client.get_key = AsyncMock(return_value=_key())
        api.assignments._low_level_client.batch_create_resource_attributes = AsyncMock(
            return_value=[]
        )

        await api.assignments.create(
            "k1", [ResourceAttributeEntity.for_channel("ch1")], value=["e_a"]
        )

        api.assignments._low_level_client.get_key.assert_awaited_once_with("k1")
        kwargs = api.assignments._low_level_client.batch_create_resource_attributes.call_args.kwargs
        assert kwargs["key_id"] == "k1"
        assert kwargs["enum_value_ids"] == ["e_a"]

    @pytest.mark.asyncio
    async def test_resolves_resource_ids_to_supported_entity_type(self):
        api = _api()
        api.assignments._low_level_client.batch_create_resource_attributes = AsyncMock(
            return_value=[]
        )
        api.client.async_.assets.list_ = AsyncMock(return_value=[])
        api.client.async_.channels.list_ = AsyncMock(return_value=[_matched_resource("ch1")])
        api.client.async_.runs.list_ = AsyncMock(return_value=[])

        await api.assignments.create(_key(), ["ch1", "ch1"], value=["e_a"])

        api.client.async_.assets.list_.assert_awaited_once_with(
            asset_ids=["ch1"], include_archived=True
        )
        api.client.async_.channels.list_.assert_awaited_once_with(channel_ids=["ch1"])
        api.client.async_.runs.list_.assert_awaited_once_with(
            run_ids=["ch1"], include_archived=True
        )
        kwargs = api.assignments._low_level_client.batch_create_resource_attributes.call_args.kwargs
        entities = kwargs["entities"]
        assert entities[0].entity_type == ResourceAttributeEntityType.CHANNEL
        assert entities[0].entity_id == "ch1"
        assert entities[1].entity_type == ResourceAttributeEntityType.CHANNEL
        assert entities[1].entity_id == "ch1"

    @pytest.mark.asyncio
    async def test_rejects_unknown_resource_ids(self):
        api = _api()
        api.assignments._low_level_client.batch_create_resource_attributes = AsyncMock(
            return_value=[]
        )
        api.client.async_.assets.list_ = AsyncMock(return_value=[])
        api.client.async_.channels.list_ = AsyncMock(return_value=[])
        api.client.async_.runs.list_ = AsyncMock(return_value=[])

        with pytest.raises(ValueError, match="Could not resolve resource ID"):
            await api.assignments.create(_key(), ["missing"], value=["e_a"])

        api.assignments._low_level_client.batch_create_resource_attributes.assert_not_called()

    @pytest.mark.asyncio
    async def test_rejects_resource_entity_types_outside_current_supported_targets(self):
        api = _api()
        api.assignments._low_level_client.batch_create_resource_attributes = AsyncMock(
            return_value=[]
        )
        unsupported = ResourceAttributeEntity(
            entity_id="unknown",
            entity_type=ResourceAttributeEntityType.UNSPECIFIED,
        )

        with pytest.raises(ValueError, match="currently support assets, channels, and runs"):
            await api.assignments.create(_key(), [unsupported], value=["e_a"])

        api.assignments._low_level_client.batch_create_resource_attributes.assert_not_called()

    @pytest.mark.asyncio
    async def test_rejects_resource_assignment_filters_outside_current_supported_targets(self):
        api = _api()
        api.assignments._low_level_client.list_all_resource_attributes_by_entity = AsyncMock(
            return_value=[]
        )
        unsupported = ResourceAttributeEntity(
            entity_id="unknown",
            entity_type=ResourceAttributeEntityType.UNSPECIFIED,
        )

        with pytest.raises(ValueError, match="currently support assets, channels, and runs"):
            await api.assignments.list_(resource=unsupported)

        api.assignments._low_level_client.list_all_resource_attributes_by_entity.assert_not_called()


def _asset_proto():
    from sift.assets.v1.assets_pb2 import Asset as AssetProto

    proto = AssetProto(asset_id="a1", name="asset")
    return proto


class TestKeysUpdate:
    @pytest.mark.asyncio
    async def test_dict_update_is_validated_and_carries_key_id(self):
        api = _api()
        api.keys._low_level_client.update_key = AsyncMock(return_value=_key())

        await api.keys.update("k1", {"display_name": "new name"})

        update = api.keys._low_level_client.update_key.call_args.kwargs["update"]
        assert isinstance(update, ResourceAttributeKeyUpdate)
        assert update.display_name == "new name"
        assert update.resource_id == "k1"

    @pytest.mark.asyncio
    async def test_model_update_takes_id_from_key_object(self):
        api = _api()
        api.keys._low_level_client.update_key = AsyncMock(return_value=_key())

        await api.keys.update(_key(), ResourceAttributeKeyUpdate(description="d"))

        update = api.keys._low_level_client.update_key.call_args.kwargs["update"]
        assert update.resource_id == "k1"
        assert update.description == "d"


class TestNestedWiring:
    def test_sub_resources_share_the_parent_client(self):
        api = _api()
        assert api.keys.client is api.client
        assert api.enum_values.client is api.client
        assert api.assignments.client is api.client
