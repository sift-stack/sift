"""Domain types for resource attributes.

Resource attributes describe the Sift objects an access decision applies to. A resource
is the "what" in an access decision. The model has three tiers:

- ``ResourceAttributeKey`` defines an attribute (e.g. ``licenses``) and its value type.
- ``ResourceAttributeEnumValue`` is an allowed value for an ``ENUM``/``SET_OF_ENUM`` key.
- ``ResourceAttribute`` is a single assignment of a value to one resource.

The ``ResourceAttributeKey`` acts as the entry point: enum values and assignments are
managed through methods on a key instance.
"""

from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, Any

from pydantic import BaseModel
from sift.resource_attribute.v1 import resource_attribute_pb2 as ra_pb

from sift_client.sift_types._base import BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset
    from sift_client.sift_types.channel import Channel
    from sift_client.sift_types.run import Run


class ResourceAttributeKeyType(Enum):
    """Value type of a resource attribute key."""

    UNSPECIFIED = ra_pb.RESOURCE_ATTRIBUTE_KEY_TYPE_UNSPECIFIED
    ENUM = ra_pb.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM
    BOOLEAN = ra_pb.RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN
    NUMBER = ra_pb.RESOURCE_ATTRIBUTE_KEY_TYPE_NUMBER
    SET_OF_ENUM = ra_pb.RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM


class ResourceAttributeEntityType(Enum):
    """Kind of Sift resource a resource attribute can be assigned to."""

    UNSPECIFIED = ra_pb.RESOURCE_ATTRIBUTE_ENTITY_TYPE_UNSPECIFIED
    ASSET = ra_pb.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET
    CHANNEL = ra_pb.RESOURCE_ATTRIBUTE_ENTITY_TYPE_CHANNEL
    RUN = ra_pb.RESOURCE_ATTRIBUTE_ENTITY_TYPE_RUN


class ResourceAttributeEntity(BaseModel):
    """Identifies the supported resource a resource attribute is assigned to."""

    entity_id: str
    entity_type: ResourceAttributeEntityType

    @classmethod
    def for_asset(cls, entity_id: str) -> ResourceAttributeEntity:
        """Build an identifier for an asset ID."""
        return cls(entity_id=entity_id, entity_type=ResourceAttributeEntityType.ASSET)

    @classmethod
    def for_channel(cls, entity_id: str) -> ResourceAttributeEntity:
        """Build an identifier for a channel ID."""
        return cls(entity_id=entity_id, entity_type=ResourceAttributeEntityType.CHANNEL)

    @classmethod
    def for_run(cls, entity_id: str) -> ResourceAttributeEntity:
        """Build an identifier for a run ID."""
        return cls(entity_id=entity_id, entity_type=ResourceAttributeEntityType.RUN)

    @classmethod
    def _from_proto(cls, proto: ra_pb.ResourceAttributeEntityIdentifier) -> ResourceAttributeEntity:
        return cls(
            entity_id=proto.entity_id,
            entity_type=ResourceAttributeEntityType(proto.entity_type),
        )

    def _to_proto(self) -> ra_pb.ResourceAttributeEntityIdentifier:
        return ra_pb.ResourceAttributeEntityIdentifier(
            entity_id=self.entity_id,
            entity_type=self.entity_type.value,
        )


class ResourceAttributeEnumValue(
    BaseType[ra_pb.ResourceAttributeEnumValue, "ResourceAttributeEnumValue"]
):
    """An allowed value for an ``ENUM`` or ``SET_OF_ENUM`` resource attribute key."""

    key_id: str
    display_name: str
    description: str
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    archived_date: datetime | None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: ra_pb.ResourceAttributeEnumValue, sift_client: SiftClient | None = None
    ) -> ResourceAttributeEnumValue:
        return cls(
            proto=proto,
            id_=proto.resource_attribute_enum_value_id,
            key_id=proto.resource_attribute_key_id,
            display_name=proto.display_name,
            description=proto.description,
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

    def archive(self, *, replacement: ResourceAttributeEnumValue | str | None = None) -> int:
        """Archive this enum value, migrating existing assignments to a replacement.

        Args:
            replacement: Enum value (or ID) that existing assignments should be
                reassigned to. If None, assignments using this value are archived.

        Returns:
            The number of resource attribute assignments that were migrated.

        Note:
            Returns the migration count; it does not refresh this instance's
            ``is_archived``/``archived_date``. Re-fetch the enum value to observe those.
        """
        return self.client.access_control.resource_attributes.archive_enum_value(
            self, replacement=replacement
        )

    def unarchive(self) -> ResourceAttributeEnumValue:
        """Unarchive this enum value."""
        updated = self.client.access_control.resource_attributes.unarchive_enum_value(self)
        self._update(updated)
        return self

    def __str__(self) -> str:
        return self.display_name


class ResourceAttribute(BaseType[ra_pb.ResourceAttribute, "ResourceAttribute"]):
    """A single assignment of a resource attribute value to a supported resource."""

    organization_id: str
    key_id: str
    entity: ResourceAttributeEntity | None
    enum_value_id: str | None
    boolean_value: bool | None
    number_value: int | None
    key: ResourceAttributeKey | None
    enum_value: ResourceAttributeEnumValue | None
    created_date: datetime | None
    created_by_user_id: str
    archived_date: datetime | None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: ra_pb.ResourceAttribute, sift_client: SiftClient | None = None
    ) -> ResourceAttribute:
        which = proto.WhichOneof("value")
        return cls(
            proto=proto,
            id_=proto.resource_attribute_id,
            organization_id=proto.organization_id,
            key_id=proto.resource_attribute_key_id,
            entity=(
                ResourceAttributeEntity._from_proto(proto.entity)
                if proto.HasField("entity")
                else None
            ),
            enum_value_id=(
                proto.resource_attribute_enum_value_id
                if which == "resource_attribute_enum_value_id"
                else None
            ),
            boolean_value=proto.boolean_value if which == "boolean_value" else None,
            number_value=proto.number_value if which == "number_value" else None,
            key=(
                ResourceAttributeKey._from_proto(proto.key, sift_client)
                if proto.HasField("key")
                else None
            ),
            enum_value=(
                ResourceAttributeEnumValue._from_proto(proto.enum_value_details, sift_client)
                if proto.HasField("enum_value_details")
                else None
            ),
            created_date=(
                proto.created_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("created_date")
                else None
            ),
            created_by_user_id=proto.created_by_user_id,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            is_archived=proto.is_archived,
            _client=sift_client,
        )

    def _apply_client_to_instance(self, client: SiftClient) -> None:
        # Cascade to the nested key/enum_value so their convenience methods work too.
        super()._apply_client_to_instance(client)
        if self.key is not None:
            self.key._apply_client_to_instance(client)
        if self.enum_value is not None:
            self.enum_value._apply_client_to_instance(client)

    def archive(self) -> ResourceAttribute:
        """Archive this assignment."""
        self.client.access_control.resource_attributes.archive_assignments([self])
        self._update(
            self.client.access_control.resource_attributes.get_assignment(
                assignment_id=self._id_or_error
            )
        )
        return self

    def unarchive(self) -> ResourceAttribute:
        """Unarchive this assignment."""
        self.client.access_control.resource_attributes.unarchive_assignments([self])
        self._update(
            self.client.access_control.resource_attributes.get_assignment(
                assignment_id=self._id_or_error
            )
        )
        return self


class ResourceAttributeKey(BaseType[ra_pb.ResourceAttributeKey, "ResourceAttributeKey"]):
    """A resource attribute key. Enum values and assignments are managed through it."""

    organization_id: str
    display_name: str
    description: str
    key_type: ResourceAttributeKeyType
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    archived_date: datetime | None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: ra_pb.ResourceAttributeKey, sift_client: SiftClient | None = None
    ) -> ResourceAttributeKey:
        return cls(
            proto=proto,
            id_=proto.resource_attribute_key_id,
            organization_id=proto.organization_id,
            display_name=proto.display_name,
            description=proto.description,
            key_type=ResourceAttributeKeyType(proto.type),
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

    def create_enum_value(
        self, display_name: str, *, description: str = ""
    ) -> ResourceAttributeEnumValue:
        """Create a single enum value for this key."""
        return self.client.access_control.resource_attributes.create_enum_value(
            self, display_name, description=description
        )

    def get_or_create_enum_values(self, names: list[str]) -> list[ResourceAttributeEnumValue]:
        """Get existing enum values by name, creating any that don't exist."""
        return self.client.access_control.resource_attributes.get_or_create_enum_values(self, names)

    def list_enum_values(
        self, *, include_archived: bool = False
    ) -> list[ResourceAttributeEnumValue]:
        """List the enum values defined for this key."""
        return self.client.access_control.resource_attributes.list_enum_values(
            self, include_archived=include_archived
        )

    def assign_to(
        self,
        resources: list[ResourceAttributeEntity | Asset | Channel | Run],
        *,
        value: Any,
    ) -> list[ResourceAttribute]:
        """Assign a value to one or more resources for this key.

        Args:
            resources: Resources to assign to. For currently supported resource types, pass
                ``Asset``, ``Channel``, or ``Run`` objects directly, or use
                ``ResourceAttributeEntity`` when you only have an ID.
            value: The value to assign. For ``SET_OF_ENUM`` keys, a list of enum values
                (or their IDs); for ``ENUM`` keys, a single enum value; for ``BOOLEAN``
                keys, a bool; for ``NUMBER`` keys, an int. For ``SET_OF_ENUM`` this
                replaces the full set on each resource.

        Returns:
            The created assignments.
        """
        return self.client.access_control.resource_attributes.assign(self, resources, value=value)

    def list_assignments(self, *, include_archived: bool = False) -> list[ResourceAttribute]:
        """List all assignments of this key."""
        return self.client.access_control.resource_attributes.list_assignments(
            key=self, include_archived=include_archived
        )

    def update(
        self, *, display_name: str | None = None, description: str | None = None
    ) -> ResourceAttributeKey:
        """Update this key's display name or description."""
        updated = self.client.access_control.resource_attributes.update_key(
            self, display_name=display_name, description=description
        )
        self._update(updated)
        return self

    def archive(self) -> ResourceAttributeKey:
        """Archive this key. Cascades to its enum values and assignments."""
        updated = self.client.access_control.resource_attributes.archive_key(self)
        self._update(updated)
        return self

    def unarchive(self) -> ResourceAttributeKey:
        """Unarchive this key."""
        updated = self.client.access_control.resource_attributes.unarchive_key(self)
        self._update(updated)
        return self

    def check_archive_impact(self) -> int:
        """Return the number of active assignments that archiving this key would affect."""
        return self.client.access_control.resource_attributes.check_key_archive_impact(self)

    def __str__(self) -> str:
        return self.display_name
