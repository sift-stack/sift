"""Unit tests for the principal attributes high-level API.

These mock the low-level client and the users API, and exercise the nested
keys/enum_values/assignments sub-resources. They are not integration tests.
"""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.principal_attributes.v1 import principal_attributes_pb2 as pa

from sift_client.resources.access_control.principal_attributes import PrincipalAttributesAPIAsync
from sift_client.sift_types.principal_attribute import (
    PrincipalAttributeKey,
    PrincipalAttributeKeyUpdate,
    PrincipalType,
)


def _api() -> PrincipalAttributesAPIAsync:
    client = MagicMock()
    api = PrincipalAttributesAPIAsync(client)
    api.keys._low_level_client = MagicMock()
    api.enum_values._low_level_client = MagicMock()
    api.assignments._low_level_client = MagicMock()
    return api


def _key() -> PrincipalAttributeKey:
    return PrincipalAttributeKey._from_proto(
        pa.PrincipalAttributeKey(
            principal_attribute_key_id="pk1",
            display_name="licenses",
            type=pa.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_SET_OF_ENUM,
        )
    )


class TestAssignmentsCreate:
    @pytest.mark.asyncio
    async def test_fetches_key_when_assigning_by_key_id(self):
        api = _api()
        api.assignments._low_level_client.get_key = AsyncMock(return_value=_key())
        api.assignments._low_level_client.batch_create_values = AsyncMock(return_value=[])

        await api.assignments.create("pk1", ["u1"], value=["e_a"])

        api.assignments._low_level_client.get_key.assert_awaited_once_with("pk1")
        kwargs = api.assignments._low_level_client.batch_create_values.call_args.kwargs
        assert kwargs["key_id"] == "pk1"
        assert kwargs["principal_ids"] == ["u1"]
        assert kwargs["enum_value_ids"] == ["e_a"]

    @pytest.mark.asyncio
    async def test_resolves_emails_via_users_api_and_keeps_raw_ids(self):
        api = _api()
        api.client.async_.users.resolve_ids = AsyncMock(return_value={"alice@x.com": "u1"})
        api.assignments._low_level_client.batch_create_values = AsyncMock(return_value=[])

        await api.assignments.create(_key(), ["alice@x.com", "raw_id"], value=["e_a"])

        api.client.async_.users.resolve_ids.assert_awaited_once_with(["alice@x.com"])
        kwargs = api.assignments._low_level_client.batch_create_values.call_args.kwargs
        assert kwargs["principal_ids"] == ["u1", "raw_id"]
        assert kwargs["principal_type"] == PrincipalType.USER.value
        assert kwargs["enum_value_ids"] == ["e_a"]

    @pytest.mark.asyncio
    async def test_unresolvable_email_raises(self):
        api = _api()
        api.client.async_.users.resolve_ids = AsyncMock(return_value={})

        with pytest.raises(ValueError, match="No user found"):
            await api.assignments.create(_key(), ["ghost@x.com"], value=["e_a"])

    @pytest.mark.asyncio
    async def test_email_with_non_user_principal_type_raises(self):
        api = _api()
        with pytest.raises(ValueError, match="only supported for USER"):
            await api.assignments.create(
                _key(), ["group@x.com"], value=["e_a"], principal_type=PrincipalType.USER_GROUP
            )


class TestAssignmentsListRouting:
    @pytest.mark.asyncio
    async def test_uses_key_values_rpc_when_key_given(self):
        api = _api()
        api.assignments._low_level_client.list_all_key_values = AsyncMock(return_value=[])
        api.assignments._low_level_client.list_all_values = AsyncMock(
            side_effect=AssertionError("should use key values")
        )

        await api.assignments.list_(key=_key())

        api.assignments._low_level_client.list_all_key_values.assert_awaited_once()

    @pytest.mark.asyncio
    async def test_uses_all_values_rpc_without_key(self):
        api = _api()
        api.assignments._low_level_client.list_all_values = AsyncMock(return_value=[])

        await api.assignments.list_()

        api.assignments._low_level_client.list_all_values.assert_awaited_once()


class TestKeysUpdate:
    @pytest.mark.asyncio
    async def test_dict_update_is_validated_and_carries_key_id(self):
        api = _api()
        api.keys._low_level_client.update_key = AsyncMock(return_value=_key())

        await api.keys.update("pk1", {"display_name": "new name"})

        update = api.keys._low_level_client.update_key.call_args.kwargs["update"]
        assert isinstance(update, PrincipalAttributeKeyUpdate)
        assert update.display_name == "new name"
        assert update.resource_id == "pk1"


class TestNestedWiring:
    def test_sub_resources_share_the_parent_client(self):
        api = _api()
        assert api.keys.client is api.client
        assert api.enum_values.client is api.client
        assert api.assignments.client is api.client
