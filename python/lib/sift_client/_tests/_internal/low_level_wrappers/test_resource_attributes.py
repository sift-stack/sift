"""Tests for the resource attributes low-level wrapper."""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.resource_attribute.v1 import resource_attribute_pb2 as ra

from sift_client._internal.low_level_wrappers.resource_attributes import (
    ResourceAttributesLowLevelClient,
)
from sift_client.sift_types.resource_attribute import ResourceAttributeEntity


def _client_with_stub(stub: MagicMock) -> ResourceAttributesLowLevelClient:
    grpc = MagicMock()
    grpc.get_stub.return_value = stub
    return ResourceAttributesLowLevelClient(grpc)


class TestCreateKey:
    @pytest.mark.asyncio
    async def test_sends_type_and_returns_key(self):
        stub = MagicMock()
        stub.CreateResourceAttributeKey = AsyncMock(
            return_value=ra.CreateResourceAttributeKeyResponse(
                resource_attribute_key=ra.ResourceAttributeKey(
                    resource_attribute_key_id="k1", display_name="licenses"
                )
            )
        )
        client = _client_with_stub(stub)

        key = await client.create_key(
            display_name="licenses", key_type=ra.RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM
        )

        request = stub.CreateResourceAttributeKey.call_args[0][0]
        assert request.display_name == "licenses"
        assert request.type == ra.RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM
        assert key.id_ == "k1"


class TestUpdateKey:
    @pytest.mark.asyncio
    async def test_only_sets_provided_fields_in_mask(self):
        stub = MagicMock()
        stub.UpdateResourceAttributeKey = AsyncMock(
            return_value=ra.UpdateResourceAttributeKeyResponse(
                resource_attribute_key=ra.ResourceAttributeKey(resource_attribute_key_id="k1")
            )
        )
        client = _client_with_stub(stub)

        await client.update_key("k1", description="new desc")

        request = stub.UpdateResourceAttributeKey.call_args[0][0]
        assert list(request.update_mask.paths) == ["description"]
        assert request.description == "new desc"


class TestListAllKeys:
    @pytest.mark.asyncio
    async def test_follows_pagination(self):
        stub = MagicMock()
        stub.ListResourceAttributeKeys = AsyncMock(
            side_effect=[
                ra.ListResourceAttributeKeysResponse(
                    resource_attribute_keys=[
                        ra.ResourceAttributeKey(resource_attribute_key_id="k1")
                    ],
                    next_page_token="tok",
                ),
                ra.ListResourceAttributeKeysResponse(
                    resource_attribute_keys=[
                        ra.ResourceAttributeKey(resource_attribute_key_id="k2")
                    ],
                    next_page_token="",
                ),
            ]
        )
        client = _client_with_stub(stub)

        keys = await client.list_all_keys()

        assert [k.id_ for k in keys] == ["k1", "k2"]
        assert stub.ListResourceAttributeKeys.call_count == 2

    @pytest.mark.asyncio
    async def test_passes_filter_and_include_archived(self):
        stub = MagicMock()
        stub.ListResourceAttributeKeys = AsyncMock(
            return_value=ra.ListResourceAttributeKeysResponse(next_page_token="")
        )
        client = _client_with_stub(stub)

        await client.list_all_keys(query_filter='name == "licenses"', include_archived=True)

        request = stub.ListResourceAttributeKeys.call_args[0][0]
        assert request.filter == 'name == "licenses"'
        assert request.include_archived is True


class TestArchiveEnumValue:
    @pytest.mark.asyncio
    async def test_returns_migrated_count(self):
        stub = MagicMock()
        stub.ArchiveResourceAttributeEnumValue = AsyncMock(
            return_value=ra.ArchiveResourceAttributeEnumValueResponse(
                resource_attributes_migrated=4
            )
        )
        client = _client_with_stub(stub)

        migrated = await client.archive_enum_value("ev1", replacement_enum_value_id="ev2")

        request = stub.ArchiveResourceAttributeEnumValue.call_args[0][0]
        assert request.archived_enum_value_id == "ev1"
        assert request.replacement_enum_value_id == "ev2"
        assert migrated == 4


class TestBatchCreateResourceAttributes:
    @pytest.mark.asyncio
    async def test_uses_enum_value_id_list_for_set(self):
        stub = MagicMock()
        stub.BatchCreateResourceAttributes = AsyncMock(
            return_value=ra.BatchCreateResourceAttributesResponse(
                resource_attributes=[ra.ResourceAttribute(resource_attribute_id="a1")]
            )
        )
        client = _client_with_stub(stub)

        attrs = await client.batch_create_resource_attributes(
            key_id="k1",
            entities=[ResourceAttributeEntity.for_channel("ch1")],
            enum_value_ids=["ev1", "ev2"],
        )

        request = stub.BatchCreateResourceAttributes.call_args[0][0]
        assert list(request.resource_attribute_enum_value_ids.ids) == ["ev1", "ev2"]
        assert [e.entity_id for e in request.entities] == ["ch1"]
        assert [a.id_ for a in attrs] == ["a1"]

    @pytest.mark.asyncio
    async def test_uses_single_enum_value_id(self):
        stub = MagicMock()
        stub.BatchCreateResourceAttributes = AsyncMock(
            return_value=ra.BatchCreateResourceAttributesResponse()
        )
        client = _client_with_stub(stub)

        await client.batch_create_resource_attributes(
            key_id="k1",
            entities=[ResourceAttributeEntity.for_asset("a1")],
            enum_value_id="ev1",
        )

        request = stub.BatchCreateResourceAttributes.call_args[0][0]
        assert request.resource_attribute_enum_value_id == "ev1"
        assert not request.HasField("resource_attribute_enum_value_ids")
