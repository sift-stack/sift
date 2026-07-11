from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.users import UsersLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient
    from sift_client.sift_types.user import User


class UsersAPIAsync(ResourceBase):
    """High-level API for users.

    A user's ``name`` is their login name, typically their email address.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the UsersAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = UsersLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(self, *, user_id: str) -> User:
        """Get a user by ID.

        Args:
            user_id: The ID of the user to retrieve.

        Returns:
            The User.
        """
        user = await self._low_level_client.get_user(user_id)
        return self._apply_client_to_instance(user)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        include_inactive: bool = False,
        organization_id: str | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[User]:
        """List users with optional filtering.

        Args:
            name: Exact login name (typically the email address).
            names: Login names to filter by.
            name_contains: Substring match on the login name.
            name_regex: Regex match on the login name.
            include_inactive: If True, include inactive users.
            organization_id: Scope the search to this organization. Only supported when
                listing active users.
            filter_query: Explicit CEL query.
            order_by: Field and direction to order by.
            limit: Maximum number of users to return.
            page_size: Results to fetch per request.

        Returns:
            The matching users.
        """
        if include_inactive and organization_id is not None:
            raise ValueError("organization_id is only supported when listing active users.")

        filter_parts = self._build_name_cel_filters(
            name=name, names=names, name_contains=name_contains, name_regex=name_regex
        )
        if filter_query:
            filter_parts.append(filter_query)
        query_filter = cel.and_(*filter_parts) or None

        if include_inactive:
            users = await self._low_level_client.list_all_users(
                query_filter=query_filter,
                order_by=order_by,
                max_results=limit,
                **({"page_size": page_size} if page_size is not None else {}),
            )
        else:
            users = await self._low_level_client.list_all_active_users(
                query_filter=query_filter,
                order_by=order_by,
                organization_id=organization_id,
                max_results=limit,
                **({"page_size": page_size} if page_size is not None else {}),
            )
        return self._apply_client_to_instances(users)

    async def find(self, **kwargs) -> User | None:
        """Find a single user matching the query. Raises if more than one matches.

        Takes the same arguments as ``list_``.
        """
        users = await self.list_(**kwargs)
        if len(users) > 1:
            raise ValueError(f"Multiple ({len(users)}) users found for query")
        return users[0] if users else None

    async def resolve_ids(self, emails: list[str]) -> dict[str, str]:
        """Resolve user login emails (their user names) to user IDs.

        Matching is case-insensitive. Login names are stored and compared
        case-sensitively, so emails that miss on exact casing fall back to a
        case-insensitive match against the full user list. Inactive users are
        resolved too.

        Returns a mapping of email (as passed) to user ID for the emails that were
        found. Emails with no matching user are omitted.

        Args:
            emails: The login emails to resolve.

        Raises:
            ValueError: If an email matches multiple users case-insensitively.
        """
        wanted = list(dict.fromkeys(email for email in emails if email))
        if not wanted:
            return {}
        users = await self.list_(names=wanted, include_inactive=True)
        by_name = {user.name: user._id_or_error for user in users}
        resolved = {email: by_name[email] for email in wanted if email in by_name}

        missing = [email for email in wanted if email not in resolved]
        if missing:
            folded_to_email = {email.casefold(): email for email in missing}
            matches: dict[str, list[str]] = {}
            for user in await self.list_(include_inactive=True):
                email = folded_to_email.get(user.name.casefold())
                if email is not None:
                    matches.setdefault(email, []).append(user._id_or_error)
            for email, user_ids in matches.items():
                if len(set(user_ids)) > 1:
                    raise ValueError(f"Multiple users match email {email!r} case-insensitively.")
                resolved[email] = user_ids[0]

        return {email: resolved[email] for email in wanted if email in resolved}
