from __future__ import annotations

from abc import ABC
from typing import TYPE_CHECKING, Any, TypeVar

from sift_client.sift_types.tag import Tag
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types._base import BaseType
    from sift_client.transport.base_connection import GrpcClient, RestClient
T = TypeVar("T", bound="BaseType")


class ResourceBase(ABC):
    _sift_client: SiftClient

    def __init__(self, sift_client: SiftClient):
        self._sift_client = sift_client

    @property
    def client(self) -> SiftClient:
        return self._sift_client

    @property
    def grpc_client(self) -> GrpcClient:
        return self.client.grpc_client

    @property
    def rest_client(self) -> RestClient:
        return self.client.rest_client

    def _apply_client_to_instance(self, instance: T) -> T:
        instance._apply_client_to_instance(self.client)
        return instance

    def _apply_client_to_instances(self, instances: list[T]) -> list[T]:
        return [self._apply_client_to_instance(i) for i in instances]

    # Common CEL filters used in resources
    def _build_name_cel_filters(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
    ) -> list[str]:
        filter_parts = []
        if name:
            filter_parts.append(cel.equals("name", name))
        if names:
            filter_parts.append(cel.in_("name", names))
        if name_contains:
            filter_parts.append(cel.contains("name", name_contains))
        if name_regex:
            filter_parts.append(cel.match("name", name_regex))
        return filter_parts

    def _build_time_cel_filters(
        self,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
    ) -> list[str]:
        filter_parts = []
        if created_after:
            filter_parts.append(cel.greater_than("created_date", created_after))
        if created_before:
            filter_parts.append(cel.less_than("created_date", created_before))
        if modified_after:
            filter_parts.append(cel.greater_than("modified_date", modified_after))
        if modified_before:
            filter_parts.append(cel.less_than("modified_date", modified_before))
        if created_by:
            if isinstance(created_by, str):
                filter_parts.append(cel.equals("created_by_user_id", created_by))
            else:
                raise NotImplementedError
        if modified_by:
            if isinstance(modified_by, str):
                filter_parts.append(cel.equals("modified_by_user_id", created_by))
            else:
                raise NotImplementedError
        return filter_parts

    def _build_metadata_cel_filters(
        self, metadata: list[Any] | dict[str, Any] | None = None
    ) -> list[str]:
        filter_parts = []
        if metadata:
            if isinstance(metadata, list):
                raise NotImplementedError
            if isinstance(metadata, dict):
                for key, value in metadata.items():
                    cast_value = value
                    if isinstance(value, str):
                        cast_value = f"'{value}'"
                    elif isinstance(value, (int, float)):
                        cast_value = f"double({value})"
                    filter_parts.append(cel.equals(f"metadata[{key}]", cast_value))
        return filter_parts

    def _build_tags_metadata_cel_filters(
        self,
        *,
        tag_names: list[Tag] | list[str] | None = None,
        tag_ids: list[Any] | list[str] | None = None,
        metadata: list[Any] | dict[str, Any] | None = None,
    ) -> list[str]:
        """Build CEL filters for tags and metadata.
        Note: Some resources only support filtering on tag_id but conceptually users are most likely to want to filter on tag names. Check the request proto when using this helper and consider using tag_names by default if supported as a filterable field by the request proto.

        Args:
            tag_names: Creates filters for tag names
            tag_ids: Creates filters for tag IDs
            metadata: Creates filters for metadata.

        Returns:
            A list of CEL filters.
        """
        filter_parts = []
        if tag_names:
            tag_names = [tag.name if isinstance(tag, Tag) else tag for tag in tag_names]
            filter_parts.append(cel.in_("tag_name", tag_names))
        if tag_ids:
            tag_ids = [tag._id_or_error if isinstance(tag, Tag) else tag for tag in tag_ids]
            filter_parts.append(cel.in_("tag_id", tag_ids))
        if metadata:
            filter_parts.extend(self._build_metadata_cel_filters(metadata))
        return filter_parts

    def _build_common_cel_filters(
        self,
        *,
        description_contains: str | None = None,
        include_archived: bool | None = None,
        filter_query: str | None = None,
    ) -> list[str]:
        filter_parts = []
        if description_contains:
            filter_parts.append(cel.contains("description", description_contains))
        if include_archived is not None and not include_archived:
            # By default, archived resources are included so only need to set if included_archived is explicitly false
            filter_parts.append(cel.equals("is_archived", False))
        if filter_query:
            filter_parts.append(filter_query)
        return filter_parts
