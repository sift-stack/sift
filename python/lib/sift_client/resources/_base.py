from __future__ import annotations

from abc import ABC
from typing import TYPE_CHECKING, Any, TypeVar

from sift_client.errors import _sift_client_experimental_warning
from sift_client.util import cel_utils as cel

_sift_client_experimental_warning()

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
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
    ) -> list[str]:
        filter_parts = []
        if name:
            filter_parts.append(cel.equals("name", name))
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

    def _build_tags_metadata_cel_filters(
        self, tags: list[Any] | list[str] | None = None, metadata: list[Any] | None = None
    ) -> list[str]:
        filter_parts = []
        if tags:
            if all(isinstance(tag, str) for tag in tags):
                filter_parts.append(cel.in_("tag_name", tags))
            else:
                raise NotImplementedError
        if metadata:
            raise NotImplementedError
        return filter_parts

    def _build_common_cel_filters(
        self,
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
    ) -> list[str]:
        filter_parts = []
        if description_contains:
            filter_parts.append(cel.contains("description", description_contains))
        if not include_archived:
            filter_parts.append(cel.equals("is_archived", False))
        if filter_query:
            filter_parts.append(filter_query)
        return filter_parts
