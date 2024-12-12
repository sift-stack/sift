from typing import List, cast

from sift.common.type.v1.user_pb2 import User
from sift.users.v2.users_pb2 import ListActiveUsersRequest, ListActiveUsersResponse
from sift.users.v2.users_pb2_grpc import UserServiceStub


def get_active_users(
    user_service: UserServiceStub,
    filter: str,
    page_size: int = 1_000,
    page_token: str = "",
) -> List[User]:
    """
    Get active users from the user service with the given filter.
    The filter must be a CEL expression.
    """
    users_pb: List[User] = []

    req = ListActiveUsersRequest(
        filter=filter,
        page_size=page_size,
        page_token=page_token,
    )
    res = cast(ListActiveUsersResponse, user_service.ListActiveUsers(req))
    users_pb.extend(res.users)
    next_page_token = res.next_page_token

    while len(next_page_token) > 0:
        req = ListActiveUsersRequest(
            filter=filter,
            page_size=page_size,
            page_token=page_token,
        )
        res = cast(ListActiveUsersResponse, user_service.ListActiveUsers(req))
        users_pb.extend(res.users)
        next_page_token = res.next_page_token

    return users_pb
