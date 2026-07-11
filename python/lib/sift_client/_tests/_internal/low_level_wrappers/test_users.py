"""Tests for the users low-level wrapper."""

from unittest.mock import AsyncMock, MagicMock

import pytest
from sift.common.type.v1.user_pb2 import User as UserProto
from sift.users.v2 import users_pb2

from sift_client._internal.low_level_wrappers.users import UsersLowLevelClient


def _client_with_stub(stub: MagicMock) -> UsersLowLevelClient:
    grpc = MagicMock()
    grpc.get_stub.return_value = stub
    return UsersLowLevelClient(grpc)


class TestGetUser:
    @pytest.mark.asyncio
    async def test_returns_user(self):
        stub = MagicMock()
        stub.GetUser = AsyncMock(
            return_value=users_pb2.GetUserResponse(
                user=UserProto(user_id="u1", user_name="alice@example.com")
            )
        )
        client = _client_with_stub(stub)

        user = await client.get_user("u1")

        request = stub.GetUser.call_args[0][0]
        assert request.user_id == "u1"
        assert user.id_ == "u1"
        assert user.name == "alice@example.com"


class TestListAllActiveUsers:
    @pytest.mark.asyncio
    async def test_follows_pagination(self):
        stub = MagicMock()
        stub.ListActiveUsers = AsyncMock(
            side_effect=[
                users_pb2.ListActiveUsersResponse(
                    users=[UserProto(user_id="u1", user_name="a@x.com")],
                    next_page_token="tok",
                ),
                users_pb2.ListActiveUsersResponse(
                    users=[UserProto(user_id="u2", user_name="b@x.com")],
                    next_page_token="",
                ),
            ]
        )
        client = _client_with_stub(stub)

        users = await client.list_all_active_users()

        assert [u.id_ for u in users] == ["u1", "u2"]
        assert stub.ListActiveUsers.call_count == 2

    @pytest.mark.asyncio
    async def test_passes_filter_and_organization_id(self):
        stub = MagicMock()
        stub.ListActiveUsers = AsyncMock(
            return_value=users_pb2.ListActiveUsersResponse(next_page_token="")
        )
        client = _client_with_stub(stub)

        await client.list_all_active_users(query_filter='name == "a@x.com"', organization_id="org1")

        request = stub.ListActiveUsers.call_args[0][0]
        assert request.filter == 'name == "a@x.com"'
        assert request.organization_id == "org1"


class TestListAllUsers:
    @pytest.mark.asyncio
    async def test_uses_list_users_rpc(self):
        stub = MagicMock()
        stub.ListUsers = AsyncMock(return_value=users_pb2.ListUsersResponse(next_page_token=""))
        client = _client_with_stub(stub)

        await client.list_all_users(query_filter='user_id == "u1"')

        request = stub.ListUsers.call_args[0][0]
        assert request.filter == 'user_id == "u1"'
