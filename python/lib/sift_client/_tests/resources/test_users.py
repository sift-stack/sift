"""Unit tests for the users high-level API. These mock the low-level client."""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.common.type.v1.user_pb2 import User as UserProto

from sift_client.resources.users import UsersAPIAsync
from sift_client.sift_types.user import User
from sift_client.util import cel_utils as cel


def _api() -> UsersAPIAsync:
    client = MagicMock()
    api = UsersAPIAsync(client)
    api._low_level_client = MagicMock()
    return api


def _user(user_id: str, name: str) -> User:
    return User._from_proto(UserProto(user_id=user_id, user_name=name))


class TestList:
    @pytest.mark.asyncio
    async def test_defaults_to_active_users(self):
        api = _api()
        api._low_level_client.list_all_active_users = AsyncMock(return_value=[])

        await api.list_()

        api._low_level_client.list_all_active_users.assert_awaited_once()

    @pytest.mark.asyncio
    async def test_include_inactive_uses_all_users_rpc(self):
        api = _api()
        api._low_level_client.list_all_users = AsyncMock(return_value=[])

        await api.list_(include_inactive=True)

        api._low_level_client.list_all_users.assert_awaited_once()

    @pytest.mark.asyncio
    async def test_names_become_cel_filter(self):
        api = _api()
        api._low_level_client.list_all_active_users = AsyncMock(return_value=[])

        await api.list_(names=["a@x.com", "b@x.com"])

        kwargs = api._low_level_client.list_all_active_users.call_args.kwargs
        assert kwargs["query_filter"] == cel.in_("name", ["a@x.com", "b@x.com"])

    @pytest.mark.asyncio
    async def test_include_inactive_with_organization_id_raises(self):
        api = _api()

        with pytest.raises(ValueError, match="organization_id"):
            await api.list_(include_inactive=True, organization_id="org1")


class TestFind:
    @pytest.mark.asyncio
    async def test_returns_none_when_no_match(self):
        api = _api()
        api._low_level_client.list_all_active_users = AsyncMock(return_value=[])

        assert await api.find(name="ghost@x.com") is None

    @pytest.mark.asyncio
    async def test_raises_on_multiple_matches(self):
        api = _api()
        api._low_level_client.list_all_active_users = AsyncMock(
            return_value=[_user("u1", "a@x.com"), _user("u2", "a@x.com")]
        )

        with pytest.raises(ValueError, match="Multiple"):
            await api.find(name="a@x.com")


class TestResolveIds:
    @pytest.mark.asyncio
    async def test_maps_emails_and_omits_missing(self):
        api = _api()
        api._low_level_client.list_all_users = AsyncMock(return_value=[_user("u1", "alice@x.com")])

        resolved = await api.resolve_ids(["alice@x.com", "ghost@x.com"])

        assert resolved == {"alice@x.com": "u1"}

    @pytest.mark.asyncio
    async def test_empty_input_makes_no_call(self):
        api = _api()
        api._low_level_client.list_all_users = AsyncMock()

        assert await api.resolve_ids([]) == {}
        api._low_level_client.list_all_users.assert_not_called()

    @pytest.mark.asyncio
    async def test_deduplicates_emails(self):
        api = _api()
        api._low_level_client.list_all_users = AsyncMock(return_value=[_user("u1", "alice@x.com")])

        await api.resolve_ids(["alice@x.com", "alice@x.com"])

        kwargs = api._low_level_client.list_all_users.call_args.kwargs
        assert kwargs["query_filter"] == cel.in_("name", ["alice@x.com"])

    @pytest.mark.asyncio
    async def test_no_fallback_call_when_all_emails_match_exactly(self):
        api = _api()
        api._low_level_client.list_all_users = AsyncMock(return_value=[_user("u1", "alice@x.com")])

        await api.resolve_ids(["alice@x.com"])

        assert api._low_level_client.list_all_users.await_count == 1

    @pytest.mark.asyncio
    async def test_resolves_case_insensitively_when_exact_match_misses(self):
        api = _api()
        # First call is the exact name filter (misses); the second lists all users
        # for the case-insensitive fallback.
        api._low_level_client.list_all_users = AsyncMock(
            side_effect=[[], [_user("u1", "alice@x.com")]]
        )

        resolved = await api.resolve_ids(["Alice@X.com"])

        assert resolved == {"Alice@X.com": "u1"}

    @pytest.mark.asyncio
    async def test_ambiguous_case_insensitive_match_raises(self):
        api = _api()
        api._low_level_client.list_all_users = AsyncMock(
            side_effect=[[], [_user("u1", "alice@x.com"), _user("u2", "Alice@x.com")]]
        )

        with pytest.raises(ValueError, match="Multiple users match"):
            await api.resolve_ids(["ALICE@X.COM"])


class TestGet:
    @pytest.mark.asyncio
    async def test_delegates_to_low_level_and_binds_client(self):
        api = _api()
        api._low_level_client.get_user = AsyncMock(return_value=_user("u1", "alice@x.com"))

        user = await api.get(user_id="u1")

        api._low_level_client.get_user.assert_awaited_once_with("u1")
        assert user.id_ == "u1"
        assert user._client is api.client
