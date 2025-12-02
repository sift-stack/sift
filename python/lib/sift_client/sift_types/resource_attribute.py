from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING

from sift.resource_attribute.v1.resource_attribute_pb2 import (
    CreateResourceAttributeEnumValueRequest as CreateResourceAttributeEnumValueRequestProto,
    CreateResourceAttributeKeyRequest as CreateResourceAttributeKeyRequestProto,
    CreateResourceAttributeRequest as CreateResourceAttributeRequestProto,
    ResourceAttribute as ResourceAttributeProto,
    ResourceAttributeEnumValue as ResourceAttributeEnumValueProto,
    ResourceAttributeEntityIdentifier,
    ResourceAttributeEntityType,
    ResourceAttributeKey as ResourceAttributeKeyProto,
    ResourceAttributeKeyType,
)

from sift_client.sift_types._base import BaseType, ModelCreate, ModelUpdate

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class ResourceAttributeKey(BaseType[ResourceAttributeKeyProto, "ResourceAttributeKey"]):
    """Model representing a Resource Attribute Key."""

    organization_id: str
    display_name: str
    description: str | None
    type: int  # ResourceAttributeKeyType enum value
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    archived_date: datetime | None

    @classmethod
    def _from_proto(
        cls, proto: ResourceAttributeKeyProto, sift_client: SiftClient | None = None
    ) -> ResourceAttributeKey:
        return cls(
            id_=proto.resource_attribute_key_id,
            proto=proto,
            organization_id=proto.organization_id,
            display_name=proto.display_name,
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
            _client=sift_client,
        )


class ResourceAttributeEnumValue(
    BaseType[ResourceAttributeEnumValueProto, "ResourceAttributeEnumValue"]
):
    """Model representing a Resource Attribute Enum Value."""

    resource_attribute_key_id: str
    display_name: str
    description: str | None
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    archived_date: datetime | None

    @classmethod
    def _from_proto(
        cls,
        proto: ResourceAttributeEnumValueProto,
        sift_client: SiftClient | None = None,
    ) -> ResourceAttributeEnumValue:
        return cls(
            id_=proto.resource_attribute_enum_value_id,
            proto=proto,
            resource_attribute_key_id=proto.resource_attribute_key_id,
            display_name=proto.display_name,
            description=proto.description if proto.description else None,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            modified_by_user_id=proto.modified_by_user_id,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            _client=sift_client,
        )


class ResourceAttribute(BaseType[ResourceAttributeProto, "ResourceAttribute"]):
    """Model representing a Resource Attribute assignment to an entity."""

    organization_id: str
    entity_id: str
    entity_type: int  # ResourceAttributeEntityType enum value
    resource_attribute_key_id: str
    resource_attribute_enum_value_id: str | None
    boolean_value: bool | None
    number_value: float | None
    created_date: datetime
    created_by_user_id: str
    archived_date: datetime | None
    # Populated in responses
    key: ResourceAttributeKey | None
    enum_value_details: ResourceAttributeEnumValue | None

    @classmethod
    def _from_proto(
        cls, proto: ResourceAttributeProto, sift_client: SiftClient | None = None
    ) -> ResourceAttribute:
        return cls(
            id_=proto.resource_attribute_id,
            proto=proto,
            organization_id=proto.organization_id,
            entity_id=proto.entity.entity_id,
            entity_type=proto.entity.entity_type,
            resource_attribute_key_id=proto.resource_attribute_key_id,
            resource_attribute_enum_value_id=(
                proto.resource_attribute_enum_value_id
                if proto.HasField("resource_attribute_enum_value_id")
                else None
            ),
            boolean_value=proto.boolean_value if proto.HasField("boolean_value") else None,
            number_value=proto.number_value if proto.HasField("number_value") else None,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            key=(
                ResourceAttributeKey._from_proto(proto.key, sift_client)
                if proto.HasField("key")
                else None
            ),
            enum_value_details=(
                ResourceAttributeEnumValue._from_proto(proto.enum_value_details, sift_client)
                if proto.HasField("enum_value_details")
                else None
            ),
            _client=sift_client,
        )


class ResourceAttributeKeyCreate(ModelCreate[CreateResourceAttributeKeyRequestProto]):
    """Create model for Resource Attribute Key."""

    display_name: str
    description: str | None = None
    type: int  # ResourceAttributeKeyType enum value
    initial_enum_values: list[dict] | None = None  # [{display_name: str, description: str}]

    def _get_proto_class(self) -> type[CreateResourceAttributeKeyRequestProto]:
        return CreateResourceAttributeKeyRequestProto

    def to_proto(self) -> CreateResourceAttributeKeyRequestProto:
        """Convert to proto, handling initial_enum_values."""
        proto = super().to_proto()
        if self.initial_enum_values:
            for enum_val in self.initial_enum_values:
                initial_enum_value = CreateResourceAttributeKeyRequestProto.InitialEnumValue(
                    display_name=enum_val["display_name"],
                    description=enum_val.get("description"),
                )
                proto.initial_enum_values.append(initial_enum_value)
        return proto


class ResourceAttributeEnumValueCreate(
    ModelCreate[CreateResourceAttributeEnumValueRequestProto]
):
    """Create model for Resource Attribute Enum Value."""

    resource_attribute_key_id: str
    display_name: str
    description: str | None = None

    def _get_proto_class(self) -> type[CreateResourceAttributeEnumValueRequestProto]:
        return CreateResourceAttributeEnumValueRequestProto


class ResourceAttributeCreate(ModelCreate[CreateResourceAttributeRequestProto]):
    """Create model for Resource Attribute."""

    entity_id: str
    entity_type: int  # ResourceAttributeEntityType enum value
    resource_attribute_key_id: str
    resource_attribute_enum_value_id: str | None = None
    boolean_value: bool | None = None
    number_value: float | None = None

    def _get_proto_class(self) -> type[CreateResourceAttributeRequestProto]:
        return CreateResourceAttributeRequestProto

    def to_proto(self) -> CreateResourceAttributeRequestProto:
        """Convert to proto, handling entity."""
        proto = super().to_proto()
        # Set entity
        proto.entity.entity_id = self.entity_id
        proto.entity.entity_type = self.entity_type
        return proto


class ResourceAttributeKeyUpdate(ModelUpdate[ResourceAttributeKeyProto]):
    """Update model for Resource Attribute Key."""

    display_name: str | None = None
    description: str | None = None

    def _get_proto_class(self) -> type[ResourceAttributeKeyProto]:
        return ResourceAttributeKeyProto

    def _add_resource_id_to_proto(self, proto_msg: ResourceAttributeKeyProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.resource_attribute_key_id = self._resource_id


class ResourceAttributeEnumValueUpdate(ModelUpdate[ResourceAttributeEnumValueProto]):
    """Update model for Resource Attribute Enum Value."""

    display_name: str | None = None
    description: str | None = None

    def _get_proto_class(self) -> type[ResourceAttributeEnumValueProto]:
        return ResourceAttributeEnumValueProto

    def _add_resource_id_to_proto(self, proto_msg: ResourceAttributeEnumValueProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.resource_attribute_enum_value_id = self._resource_id

