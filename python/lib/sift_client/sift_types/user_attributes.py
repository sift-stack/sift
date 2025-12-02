from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING

from sift.user_attributes.v1.user_attributes_pb2 import (
    CreateUserAttributeKeyRequest as CreateUserAttributeKeyRequestProto,
    CreateUserAttributeValueRequest as CreateUserAttributeValueRequestProto,
    UserAttributeKey as UserAttributeKeyProto,
    UserAttributeValue as UserAttributeValueProto,
    UserAttributeValueType,
)

from sift_client.sift_types._base import BaseType, ModelCreate, ModelUpdate

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class UserAttributeKey(BaseType[UserAttributeKeyProto, "UserAttributeKey"]):
    """Model representing a User Attribute Key."""

    name: str
    organization_id: str
    description: str | None
    type: int  # UserAttributeValueType enum value
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    archived_date: datetime | None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: UserAttributeKeyProto, sift_client: SiftClient | None = None
    ) -> UserAttributeKey:
        return cls(
            id_=proto.user_attribute_key_id,
            proto=proto,
            name=proto.name,
            organization_id=proto.organization_id,
            description=proto.description if proto.description else None,
            type=proto.type,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            modified_by_user_id=proto.modified_by_user_id,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            is_archived=proto.is_archived,
            _client=sift_client,
        )


class UserAttributeValue(BaseType[UserAttributeValueProto, "UserAttributeValue"]):
    """Model representing a User Attribute Value."""

    user_attribute_key_id: str
    user_id: str
    organization_id: str
    string_value: str | None
    number_value: float | None
    boolean_value: bool | None
    created_date: datetime
    created_by_user_id: str
    archived_date: datetime | None
    is_archived: bool
    # The full user attribute key is populated in responses
    key: UserAttributeKey | None

    @classmethod
    def _from_proto(
        cls, proto: UserAttributeValueProto, sift_client: SiftClient | None = None
    ) -> UserAttributeValue:
        return cls(
            id_=proto.user_attribute_value_id,
            proto=proto,
            user_attribute_key_id=proto.user_attribute_key_id,
            user_id=proto.user_id,
            organization_id=proto.organization_id,
            string_value=proto.string_value if proto.HasField("string_value") else None,
            number_value=proto.number_value if proto.HasField("number_value") else None,
            boolean_value=proto.boolean_value if proto.HasField("boolean_value") else None,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            is_archived=proto.is_archived,
            key=UserAttributeKey._from_proto(proto.key, sift_client) if proto.HasField("key") else None,
            _client=sift_client,
        )


class UserAttributeKeyCreate(ModelCreate[CreateUserAttributeKeyRequestProto]):
    """Create model for User Attribute Key."""

    name: str
    description: str | None = None
    type: int  # UserAttributeValueType enum value

    def _get_proto_class(self) -> type[CreateUserAttributeKeyRequestProto]:
        return CreateUserAttributeKeyRequestProto


class UserAttributeValueCreate(ModelCreate[CreateUserAttributeValueRequestProto]):
    """Create model for User Attribute Value."""

    user_attribute_key_id: str
    user_id: str
    string_value: str | None = None
    number_value: float | None = None
    boolean_value: bool | None = None

    def _get_proto_class(self) -> type[CreateUserAttributeValueRequestProto]:
        return CreateUserAttributeValueRequestProto


class UserAttributeKeyUpdate(ModelUpdate[UserAttributeKeyProto]):
    """Update model for User Attribute Key."""

    name: str | None = None
    description: str | None = None

    def _get_proto_class(self) -> type[UserAttributeKeyProto]:
        return UserAttributeKeyProto

    def _add_resource_id_to_proto(self, proto_msg: UserAttributeKeyProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.user_attribute_key_id = self._resource_id

