from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.tags import TagsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

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
        filter_query: str | None = None,
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
            filter_query: Explicit CEL query to filter tags.
            order_by: How to order the retrieved tags.
            limit: How many tags to retrieve. If None, retrieves all matches.

        Returns:
            A list of Tags that matches the filter.
        """
        # Build CEL filter
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_common_cel_filters(
                filter_query=filter_query,
            ),
        ]

        if tag_ids:
            filter_parts.append(cel.in_("tag_id", tag_ids))

        query_filter = cel.and_(*filter_parts) if filter_parts else None

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

    async def find_or_create(self, names: list[str]) -> list[Tag]:
        """Find tags by name or create them if they don't exist.

        Args:
            names: List of tag names to find or create.

        Returns:
            List of Tags that were found or created.
        """
        tags = await self.list_(names=names)
        existing_tag_names = {tag.name for tag in tags}
        for name in names:
            if name not in existing_tag_names:
                tags.append(await self.create(name))
        return tags

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
