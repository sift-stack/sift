"""Tests for the principal attributes low-level wrapper."""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.principal_attributes.v1 import principal_attributes_pb2 as pa

from sift_client._internal.low_level_wrappers.principal_attributes import (
    PrincipalAttributesLowLevelClient,
)


def _client_with_stub(stub: MagicMock) -> PrincipalAttributesLowLevelClient:
    grpc = MagicMock()
    grpc.get_stub.return_value = stub
    return PrincipalAttributesLowLevelClient(grpc)


class TestArchiveKey:
    @pytest.mark.asyncio
    async def test_uses_batch_archive_with_single_id(self):
        # Principal keys only expose a batch archive RPC; archive_key wraps a single id.
        stub = MagicMock()
        stub.ArchivePrincipalAttributeKeys = AsyncMock(
            return_value=pa.ArchivePrincipalAttributeKeysResponse()
        )
        client = _client_with_stub(stub)

        await client.archive_key("pk1")

        request = stub.ArchivePrincipalAttributeKeys.call_args[0][0]
        assert list(request.principal_attribute_key_ids) == ["pk1"]


class TestCheckKeyArchiveImpact:
    @pytest.mark.asyncio
    async def test_sums_user_and_group_counts(self):
        stub = MagicMock()
        stub.CheckPrincipalAttributeKeyArchiveImpact = AsyncMock(
            return_value=pa.CheckPrincipalAttributeKeyArchiveImpactResponse(
                active_user_principal_attribute_value_count=5,
                active_user_group_principal_attribute_value_count=2,
            )
        )
        client = _client_with_stub(stub)

        assert await client.check_key_archive_impact("pk1") == 7


class TestBatchCreateValues:
    @pytest.mark.asyncio
    async def test_sends_principal_type_and_enum_id_list(self):
        stub = MagicMock()
        stub.BatchCreatePrincipalAttributeValue = AsyncMock(
            return_value=pa.BatchCreatePrincipalAttributeValueResponse(
                principal_attribute_values=[
                    pa.PrincipalAttributeValue(principal_attribute_value_id="v1")
                ]
            )
        )
        client = _client_with_stub(stub)

        values = await client.batch_create_values(
            key_id="pk1",
            principal_ids=["u1", "u2"],
            principal_type=pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER,
            enum_value_ids=["ev1"],
        )

        request = stub.BatchCreatePrincipalAttributeValue.call_args[0][0]
        assert list(request.principal_ids) == ["u1", "u2"]
        assert request.principal_type == pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER
        assert list(request.principal_attribute_enum_value_ids.ids) == ["ev1"]
        assert [v.id_ for v in values] == ["v1"]


class TestArchiveValues:
    @pytest.mark.asyncio
    async def test_requires_principal_type(self):
        stub = MagicMock()
        stub.ArchivePrincipalAttributeValues = AsyncMock(
            return_value=pa.ArchivePrincipalAttributeValuesResponse()
        )
        client = _client_with_stub(stub)

        await client.archive_values(
            ["v1", "v2"], principal_type=pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER
        )

        request = stub.ArchivePrincipalAttributeValues.call_args[0][0]
        assert list(request.principal_attribute_value_ids) == ["v1", "v2"]
        assert request.principal_type == pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER


class TestListAllKeyValues:
    @pytest.mark.asyncio
    async def test_passes_key_and_principal_type(self):
        stub = MagicMock()
        stub.ListPrincipalAttributeKeyValues = AsyncMock(
            return_value=pa.ListPrincipalAttributeKeyValuesResponse(next_page_token="")
        )
        client = _client_with_stub(stub)

        await client.list_all_key_values(
            key_id="pk1", principal_type=pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER
        )

        request = stub.ListPrincipalAttributeKeyValues.call_args[0][0]
        assert request.principal_attribute_key_id == "pk1"
        assert request.principal_type == pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER
