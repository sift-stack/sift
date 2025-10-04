from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING

from pydantic import ConfigDict
from sift.tags.v2.tags_pb2 import Tag as TagProto

from sift_client.sift_types._base import BaseType, ModelUpdate

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class TagUpdate(ModelUpdate[TagProto]):
    """Update model for Tag."""

    name: str | None = None

    def _get_proto_class(self) -> type[TagProto]:
        return TagProto

    def _add_resource_id_to_proto(self, proto_msg: TagProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.tag_id = self._resource_id


class Tag(BaseType[TagProto, "Tag"]):
    """Model of the Sift Tag."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str
    created_date: datetime
    created_by_user_id: str

    @classmethod
    def _from_proto(cls, proto: TagProto, sift_client: SiftClient | None = None) -> Tag:
        return cls(
            id_=proto.tag_id,
            name=proto.name,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            _client=sift_client,
        )

    def _to_proto(self) -> TagProto:
        """Convert to protobuf message."""
        proto = TagProto(
            tag_id=self.id_ or "",
            name=self.name,
            created_by_user_id=self.created_by_user_id,
            created_date=self.created_date,  # type: ignore
        )
        return proto
