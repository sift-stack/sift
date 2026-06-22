"""Unit tests for the principal attributes high-level API.

These mock the low-level client and the REST client; they are not integration tests.
"""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.principal_attributes.v1 import principal_attributes_pb2 as pa

from sift_client.resources.principal_attributes import PrincipalAttributesAPIAsync
from sift_client.sift_types.principal_attribute import (
    PrincipalAttributeKey,
    PrincipalType,
)


def _api() -> PrincipalAttributesAPIAsync:
    client = MagicMock()
    api = PrincipalAttributesAPIAsync(client)
    api._low_level_client = MagicMock()
    return api


def _key() -> PrincipalAttributeKey:
    return PrincipalAttributeKey._from_proto(
        pa.PrincipalAttributeKey(
            principal_attribute_key_id="pk1",
            display_name="licenses",
            type=pa.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_SET_OF_ENUM,
        )
    )


def _users_response(users):
    response = MagicMock()
    response.raise_for_status = MagicMock()
    response.json = MagicMock(return_value={"users": users, "nextPageToken": ""})
    return response


class TestResolveUserIds:
    @pytest.mark.asyncio
    async def test_maps_email_to_user_id_via_rest(self):
        api = _api()
        api.client.rest_client.get = MagicMock(
            return_value=_users_response(
                [
                    {"userName": "alice@x.com", "userId": "u1"},
                    {"userName": "bob@x.com", "userId": "u2"},
                ]
            )
        )

        resolved = await api.resolve_user_ids(["alice@x.com"])

        assert resolved == {"alice@x.com": "u1"}

    @pytest.mark.asyncio
    async def test_resolve_user_id_raises_when_missing(self):
        api = _api()
        api.client.rest_client.get = MagicMock(return_value=_users_response([]))

        with pytest.raises(ValueError, match="No user found"):
            await api.resolve_user_id("ghost@x.com")


class TestAssign:
    @pytest.mark.asyncio
    async def test_resolves_emails_and_keeps_raw_ids(self):
        api = _api()
        api.client.rest_client.get = MagicMock(
            return_value=_users_response([{"userName": "alice@x.com", "userId": "u1"}])
        )
        api._low_level_client.batch_create_values = AsyncMock(return_value=[])

        await api.assign(_key(), ["alice@x.com", "raw_id"], value=["e_a"])

        kwargs = api._low_level_client.batch_create_values.call_args.kwargs
        assert kwargs["principal_ids"] == ["u1", "raw_id"]
        assert kwargs["principal_type"] == PrincipalType.USER.value
        assert kwargs["enum_value_ids"] == ["e_a"]

    @pytest.mark.asyncio
    async def test_unresolvable_email_raises(self):
        api = _api()
        api.client.rest_client.get = MagicMock(return_value=_users_response([]))

        with pytest.raises(ValueError, match="No user found"):
            await api.assign(_key(), ["ghost@x.com"], value=["e_a"])

    @pytest.mark.asyncio
    async def test_email_with_non_user_principal_type_raises(self):
        api = _api()
        with pytest.raises(ValueError, match="only supported for USER"):
            await api.assign(
                _key(), ["group@x.com"], value=["e_a"], principal_type=PrincipalType.USER_GROUP
            )


class TestListAssignmentsRouting:
    @pytest.mark.asyncio
    async def test_uses_key_values_rpc_when_key_given(self):
        api = _api()
        api._low_level_client.list_all_key_values = AsyncMock(return_value=[])
        api._low_level_client.list_all_values = AsyncMock(
            side_effect=AssertionError("should use key values")
        )

        await api.list_assignments(key=_key())

        api._low_level_client.list_all_key_values.assert_awaited_once()

    @pytest.mark.asyncio
    async def test_uses_all_values_rpc_without_key(self):
        api = _api()
        api._low_level_client.list_all_values = AsyncMock(return_value=[])

        await api.list_assignments()

        api._low_level_client.list_all_values.assert_awaited_once()
