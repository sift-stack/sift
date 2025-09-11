from __future__ import annotations

import re
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.tags import TagsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.util.cel_utils import contains, equals, in_, match

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.tag import Tag, TagUpdate

class TagsAPIAsync(ResourceBase):
    """High-level API for interacting with tags."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the TagsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = TagsLowLevelClient(grpc_client=self.client.grpc_client)

    async def list_(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        names: list[str] | None = None,
        tag_ids: list[str] | None = None,
        created_by_user_id: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Tag]:
        """List tags with optional filtering.

        Args:
            name: Exact name of the tag.
            name_contains: Partial name of the tag.
            name_regex: Regular expression string to filter tags by name.
            names: List of tag names to filter by.
            tag_ids: List of tag IDs to filter by.
            created_by_user_id: User ID who created the tag.
            order_by: How to order the retrieved tags.
            limit: How many tags to retrieve. If None, retrieves all matches.

        Returns:
            A list of Tags that matches the filter.
        """
        # Build CEL filter
        filter_parts = []

        if name:
            filter_parts.append(equals("name", name))
        elif name_contains:
            filter_parts.append(contains("name", name_contains))
        elif name_regex:
            if isinstance(name_regex, re.Pattern):
                name_regex = name_regex.pattern
            filter_parts.append(match("name", name_regex))  # type: ignore

        if names:
            filter_parts.append(in_("name", names))
        if tag_ids:
            filter_parts.append(in_("tag_id", tag_ids))

        if created_by_user_id:
            filter_parts.append(equals("created_by_user_id", created_by_user_id))

        query_filter = " && ".join(filter_parts) if filter_parts else None

        tags = await self._low_level_client.list_all_tags(
            query_filter=query_filter,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(tags)

    async def find(self, **kwargs) -> Tag | None:
        """Find a single tag matching the given query. Takes the same arguments as `list`. If more than one tag is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Tag found or None.
        """
        tags = await self.list_(**kwargs)
        if len(tags) > 1:
            raise ValueError("Multiple tags found for query")
        elif len(tags) == 1:
            return tags[0]
        return None

    async def create(self, name: str) -> Tag:
        """Create a new tag.

        Args:
            name: The name of the tag.

        Returns:
            The created Tag.
        """
        created_tag = await self._low_level_client.create_tag(name=name)
        return self._apply_client_to_instance(created_tag)

    async def update(self, tag: str | Tag, update: TagUpdate | dict) -> Tag:
        """Update a Tag.

        Args:
            tag: The Tag or tag ID to update.
            update: Updates to apply to the Tag.

        Returns:
            The updated Tag.

        Note:
            The tags API doesn't have an update method in the proto,
            so this would need to be implemented if the API supports it.
        """
        # Note: The tags API doesn't have an update method in the proto,
        # so this would need to be implemented if the API supports it
        raise NotImplementedError("Tag updates are not supported by the current API")
