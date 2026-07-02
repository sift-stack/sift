"""Domain types for principal attributes.

Principal attributes describe the users or groups an access decision applies to. A
principal is the "who" in an access decision, such as a user or user group. The
model mirrors resource attributes with three tiers:

- ``PrincipalAttributeKey`` defines an attribute and its value type.
- ``PrincipalAttributeEnumValue`` is an allowed value for an ``ENUM``/``SET_OF_ENUM`` key.
- ``PrincipalAttributeValue`` is a single assignment of a value to one principal.

The ``PrincipalAttributeKey`` acts as the entry point: enum values and assignments are
managed through methods on a key instance.
"""

from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, Any

from sift.principal_attributes.v1 import principal_attributes_pb2 as pa_pb

from sift_client.sift_types._base import BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class PrincipalAttributeValueType(Enum):
    """Value type of a principal attribute key."""

    UNSPECIFIED = pa_pb.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_UNSPECIFIED
    ENUM = pa_pb.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_ENUM
    BOOLEAN = pa_pb.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_BOOLEAN
    NUMBER = pa_pb.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_NUMBER
    SET_OF_ENUM = pa_pb.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_SET_OF_ENUM


class PrincipalType(Enum):
    """Kind of principal a principal attribute can be assigned to."""

    UNSPECIFIED = pa_pb.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_UNSPECIFIED
    USER = pa_pb.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER
    USER_GROUP = pa_pb.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER_GROUP


class PrincipalAttributeEnumValue(
    BaseType[pa_pb.PrincipalAttributeEnumValue, "PrincipalAttributeEnumValue"]
):
    """An allowed value for an ``ENUM`` or ``SET_OF_ENUM`` principal attribute key."""

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
        cls, proto: pa_pb.PrincipalAttributeEnumValue, sift_client: SiftClient | None = None
    ) -> PrincipalAttributeEnumValue:
        return cls(
            proto=proto,
            id_=proto.principal_attribute_enum_value_id,
            key_id=proto.principal_attribute_key_id,
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

    def archive(self, *, replacement: PrincipalAttributeEnumValue | str | None = None) -> int:
        """Archive this enum value, migrating existing assignments to a replacement.

        Args:
            replacement: Enum value (or ID) that existing assignments should be
                reassigned to. If None, assignments using this value are archived.

        Returns:
            The number of principal attribute values that were migrated.

        Note:
            Returns the migration count; it does not refresh this instance's
            ``is_archived``/``archived_date``. Re-fetch the enum value to observe those.
        """
        return self.client.access_control.principal_attributes.archive_enum_value(
            self, replacement=replacement
        )

    def unarchive(self) -> PrincipalAttributeEnumValue:
        """Unarchive this enum value."""
        updated = self.client.access_control.principal_attributes.unarchive_enum_value(self)
        self._update(updated)
        return self

    def __str__(self) -> str:
        return self.display_name


class PrincipalAttributeValue(BaseType[pa_pb.PrincipalAttributeValue, "PrincipalAttributeValue"]):
    """A single assignment of a principal attribute value to a principal."""

    organization_id: str
    key_id: str
    principal_id: str
    principal_type: PrincipalType
    enum_value_id: str | None
    boolean_value: bool | None
    number_value: int | None
    key: PrincipalAttributeKey | None
    enum_value: PrincipalAttributeEnumValue | None
    created_date: datetime | None
    created_by_user_id: str
    archived_date: datetime | None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: pa_pb.PrincipalAttributeValue, sift_client: SiftClient | None = None
    ) -> PrincipalAttributeValue:
        which = proto.WhichOneof("value")
        return cls(
            proto=proto,
            id_=proto.principal_attribute_value_id,
            organization_id=proto.organization_id,
            key_id=proto.principal_attribute_key_id,
            principal_id=proto.principal_id,
            principal_type=PrincipalType(proto.principal_type),
            enum_value_id=(
                proto.principal_attribute_enum_value_id
                if which == "principal_attribute_enum_value_id"
                else None
            ),
            boolean_value=proto.boolean_value if which == "boolean_value" else None,
            number_value=proto.number_value if which == "number_value" else None,
            key=(
                PrincipalAttributeKey._from_proto(proto.key, sift_client)
                if proto.HasField("key")
                else None
            ),
            enum_value=(
                PrincipalAttributeEnumValue._from_proto(proto.enum_value_details, sift_client)
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

    def archive(self) -> PrincipalAttributeValue:
        """Archive this assignment."""
        self.client.access_control.principal_attributes.archive_assignments(
            [self], principal_type=self.principal_type
        )
        self._update(
            self.client.access_control.principal_attributes.get_assignment(
                assignment_id=self._id_or_error, principal_type=self.principal_type
            )
        )
        return self

    def unarchive(self) -> PrincipalAttributeValue:
        """Unarchive this assignment."""
        self.client.access_control.principal_attributes.unarchive_assignments(
            [self], principal_type=self.principal_type
        )
        self._update(
            self.client.access_control.principal_attributes.get_assignment(
                assignment_id=self._id_or_error, principal_type=self.principal_type
            )
        )
        return self


class PrincipalAttributeKey(BaseType[pa_pb.PrincipalAttributeKey, "PrincipalAttributeKey"]):
    """A principal attribute key. Enum values and assignments are managed through it."""

    organization_id: str
    display_name: str
    description: str
    value_type: PrincipalAttributeValueType
    created_date: datetime
    created_by_user_id: str
    modified_date: datetime
    modified_by_user_id: str
    archived_date: datetime | None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: pa_pb.PrincipalAttributeKey, sift_client: SiftClient | None = None
    ) -> PrincipalAttributeKey:
        return cls(
            proto=proto,
            id_=proto.principal_attribute_key_id,
            organization_id=proto.organization_id,
            display_name=proto.display_name,
            description=proto.description,
            value_type=PrincipalAttributeValueType(proto.type),
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
    ) -> PrincipalAttributeEnumValue:
        """Create a single enum value for this key."""
        return self.client.access_control.principal_attributes.create_enum_value(
            self, display_name, description=description
        )

    def get_or_create_enum_values(self, names: list[str]) -> list[PrincipalAttributeEnumValue]:
        """Get existing enum values by name, creating any that don't exist."""
        return self.client.access_control.principal_attributes.get_or_create_enum_values(
            self, names
        )

    def list_enum_values(
        self, *, include_archived: bool = False
    ) -> list[PrincipalAttributeEnumValue]:
        """List the enum values defined for this key."""
        return self.client.access_control.principal_attributes.list_enum_values(
            self, include_archived=include_archived
        )

    def assign_to(
        self,
        principals: list[str],
        *,
        value: Any,
        principal_type: PrincipalType = PrincipalType.USER,
    ) -> list[PrincipalAttributeValue]:
        """Assign a value to one or more principals for this key.

        Args:
            principals: Principal IDs to assign to. For ``USER`` principals, entries
                containing ``@`` are treated as email addresses and resolved to user IDs.
            value: The value to assign. For ``SET_OF_ENUM`` keys, a list of enum values
                (or their IDs); for ``ENUM`` keys, a single enum value; for ``BOOLEAN``
                keys, a bool; for ``NUMBER`` keys, an int. For ``SET_OF_ENUM`` this
                replaces the full set on each principal.
            principal_type: The kind of principal being assigned to. Defaults to ``USER``. Use
                ``PrincipalType.USER_GROUP`` when assigning to user groups.

        Returns:
            The created assignments.
        """
        return self.client.access_control.principal_attributes.assign(
            self, principals, value=value, principal_type=principal_type
        )

    def list_assignments(
        self, *, principal_type: PrincipalType = PrincipalType.USER, include_archived: bool = False
    ) -> list[PrincipalAttributeValue]:
        """List all assignments of this key for the given principal type."""
        return self.client.access_control.principal_attributes.list_assignments(
            key=self, principal_type=principal_type, include_archived=include_archived
        )

    def update(
        self, *, display_name: str | None = None, description: str | None = None
    ) -> PrincipalAttributeKey:
        """Update this key's display name or description."""
        updated = self.client.access_control.principal_attributes.update_key(
            self, display_name=display_name, description=description
        )
        self._update(updated)
        return self

    def archive(self) -> PrincipalAttributeKey:
        """Archive this key. Cascades to its enum values and assignments."""
        updated = self.client.access_control.principal_attributes.archive_key(self)
        self._update(updated)
        return self

    def unarchive(self) -> PrincipalAttributeKey:
        """Unarchive this key."""
        updated = self.client.access_control.principal_attributes.unarchive_key(self)
        self._update(updated)
        return self

    def check_archive_impact(self) -> int:
        """Return the number of active assignments that archiving this key would affect."""
        return self.client.access_control.principal_attributes.check_key_archive_impact(self)

    def __str__(self) -> str:
        return self.display_name
