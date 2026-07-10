"""Unit tests for the principal attributes high-level API.

These mock the low-level client and the users API, and exercise the nested
keys/enum_values/assignments sub-resources. They are not integration tests.
"""

from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.principal_attributes.v1 import principal_attributes_pb2 as pa

from sift_client.resources.access_control.principal_attributes import PrincipalAttributesAPIAsync
from sift_client.sift_types.principal_attribute import (
    PrincipalAttributeKey,
    PrincipalAttributeKeyUpdate,
    PrincipalRef,
    PrincipalType,
)
from sift_client.util import cel_utils as cel


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

        await api.assignments.create("pk1", [PrincipalRef.user("u1")], value=["e_a"])

        api.assignments._low_level_client.get_key.assert_awaited_once_with("pk1")
        kwargs = api.assignments._low_level_client.batch_create_values.call_args.kwargs
        assert kwargs["key_id"] == "pk1"
        assert kwargs["principal_ids"] == ["u1"]
        assert kwargs["principal_type"] == PrincipalType.USER.value
        assert kwargs["enum_value_ids"] == ["e_a"]

    @pytest.mark.asyncio
    async def test_resolves_emails_via_users_api_and_keeps_raw_ids(self):
        api = _api()
        api.client.async_.users.resolve_ids = AsyncMock(return_value={"alice@x.com": "u1"})
        api.assignments._low_level_client.batch_create_values = AsyncMock(return_value=[])

        await api.assignments.create(
            _key(), ["alice@x.com", PrincipalRef.user("raw_id")], value=["e_a"]
        )

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
    async def test_email_in_user_group_ref_raises(self):
        api = _api()
        with pytest.raises(ValueError, match="only supported for USER"):
            await api.assignments.create(
                _key(), [PrincipalRef.user_group("group@x.com")], value=["e_a"]
            )

    @pytest.mark.asyncio
    async def test_bare_principal_id_raises_type_error(self):
        api = _api()
        api.assignments._low_level_client.batch_create_values = AsyncMock()

        with pytest.raises(TypeError, match="Cannot resolve principal"):
            await api.assignments.create(_key(), ["u1"], value=["e_a"])

        api.assignments._low_level_client.batch_create_values.assert_not_called()

    @pytest.mark.asyncio
    async def test_mixed_principal_types_split_into_one_rpc_per_type(self):
        api = _api()
        api.assignments._low_level_client.batch_create_values = AsyncMock(return_value=[])

        await api.assignments.create(
            _key(),
            [PrincipalRef.user("u1"), PrincipalRef.user_group("g1"), PrincipalRef.user("u2")],
            value=["e_a"],
        )

        calls = api.assignments._low_level_client.batch_create_values.call_args_list
        assert len(calls) == 2
        assert calls[0].kwargs["principal_ids"] == ["u1", "u2"]
        assert calls[0].kwargs["principal_type"] == PrincipalType.USER.value
        assert calls[1].kwargs["principal_ids"] == ["g1"]
        assert calls[1].kwargs["principal_type"] == PrincipalType.USER_GROUP.value


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
        kwargs = api.assignments._low_level_client.list_all_values.call_args.kwargs
        assert kwargs["principal_type"] == PrincipalType.USER.value

    @pytest.mark.asyncio
    async def test_typed_principal_scopes_rpc_to_its_type(self):
        api = _api()
        api.assignments._low_level_client.list_all_values = AsyncMock(return_value=[])

        await api.assignments.list_(principal=PrincipalRef.user_group("g1"))

        kwargs = api.assignments._low_level_client.list_all_values.call_args.kwargs
        assert kwargs["principal_type"] == PrincipalType.USER_GROUP.value
        assert kwargs["query_filter"] == cel.equals("principal_id", "g1")

    @pytest.mark.asyncio
    async def test_conflicting_principal_type_raises(self):
        api = _api()
        api.assignments._low_level_client.list_all_values = AsyncMock(return_value=[])

        with pytest.raises(ValueError, match="conflicts"):
            await api.assignments.list_(
                principal=PrincipalRef.user("u1"), principal_type=PrincipalType.USER_GROUP
            )

        api.assignments._low_level_client.list_all_values.assert_not_called()


class TestListCommonFilters:
    @pytest.mark.asyncio
    async def test_keys_time_and_user_filters_compose(self):
        api = _api()
        api.keys._low_level_client.list_all_keys = AsyncMock(return_value=[])
        before = datetime(2026, 2, 1, tzinfo=timezone.utc)

        await api.keys.list_(modified_before=before, created_by="u1", description_contains="lic")

        query = api.keys._low_level_client.list_all_keys.call_args.kwargs["query_filter"]
        assert cel.less_than("modified_date", before) in query
        assert cel.equals("created_by_user_id", "u1") in query
        assert cel.contains("description", "lic") in query

    @pytest.mark.asyncio
    async def test_enum_values_description_filter_composes(self):
        api = _api()
        api.enum_values._low_level_client.list_all_enum_values = AsyncMock(return_value=[])

        await api.enum_values.list_(_key(), description_contains="lic")

        query = api.enum_values._low_level_client.list_all_enum_values.call_args.kwargs[
            "query_filter"
        ]
        assert cel.contains("description", "lic") in query

    @pytest.mark.asyncio
    async def test_assignments_created_filter_composes(self):
        api = _api()
        api.assignments._low_level_client.list_all_values = AsyncMock(return_value=[])
        after = datetime(2026, 1, 1, tzinfo=timezone.utc)

        await api.assignments.list_(created_after=after)

        kwargs = api.assignments._low_level_client.list_all_values.call_args.kwargs
        assert cel.greater_than("created_date", after) in kwargs["query_filter"]


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
