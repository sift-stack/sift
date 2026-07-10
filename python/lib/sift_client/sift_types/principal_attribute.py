"""Domain types for principal attributes.

Principal attributes describe the users or groups an access decision applies to. A
principal is the "who" in an access decision, such as a user or user group. The
model mirrors resource attributes with three tiers:

- ``PrincipalAttributeKey`` defines an attribute and its value type.
- ``PrincipalAttributeEnumValue`` is an allowed value for an ``ENUM``/``SET_OF_ENUM`` key.
- ``PrincipalAttributeAssignment`` is a single assignment of a value to one principal.

The ``PrincipalAttributeKey`` acts as the entry point: enum values and assignments are
managed through methods on a key instance.
"""

from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, Union

from pydantic import BaseModel
from sift.principal_attributes.v1 import principal_attributes_pb2 as pa_pb

from sift_client.sift_types._base import BaseType, ModelUpdate
from sift_client.sift_types.user import User

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class PrincipalAttributeValueType(Enum):
    """Value type of a principal attribute key."""

    ENUM = pa_pb.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_ENUM
    BOOLEAN = pa_pb.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_BOOLEAN
    NUMBER = pa_pb.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_NUMBER
    SET_OF_ENUM = pa_pb.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_SET_OF_ENUM


class PrincipalType(Enum):
    """Kind of principal a principal attribute can be assigned to."""

    USER = pa_pb.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER
    USER_GROUP = pa_pb.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER_GROUP


class PrincipalRef(BaseModel):
    """Typed reference to a principal, naming both the ID and the kind of principal."""

    principal_id: str
    principal_type: PrincipalType

    @classmethod
    def user(cls, user: str | User) -> PrincipalRef:
        """Reference a user by ID, email address, or ``User`` object."""
        if isinstance(user, User):
            return cls(principal_id=user._id_or_error, principal_type=PrincipalType.USER)
        return cls(principal_id=user, principal_type=PrincipalType.USER)

    @classmethod
    def user_group(cls, user_group_id: str) -> PrincipalRef:
        """Reference a user group by ID."""
        return cls(principal_id=user_group_id, principal_type=PrincipalType.USER_GROUP)


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
        return self.client.access_control.principal_attributes.enum_values.archive(
            self, replacement=replacement
        )

    def unarchive(self) -> PrincipalAttributeEnumValue:
        """Unarchive this enum value."""
        updated = self.client.access_control.principal_attributes.enum_values.unarchive(self)
        self._update(updated)
        return self

    def __str__(self) -> str:
        return self.display_name


# Accepted value shapes for assign: a list of enum values (or their IDs) for
# SET_OF_ENUM keys, a single enum value (or its ID) for ENUM keys, a bool for
# BOOLEAN keys, or an int for NUMBER keys.
PrincipalAttributeValueLike = Union[
    bool, int, str, PrincipalAttributeEnumValue, "list[PrincipalAttributeEnumValue | str]"
]


class PrincipalAttributeAssignment(
    BaseType[pa_pb.PrincipalAttributeValue, "PrincipalAttributeAssignment"]
):
    """A single assignment of a principal attribute value to a principal."""

    organization_id: str
    key_id: str
    principal_id: str
    principal_type: PrincipalType
    enum_value_id: str | None
    boolean_value: bool | None
    number_value: int | None
    key: PrincipalAttributeKey | None
    """Full key details. Always set in responses."""
    enum_value: PrincipalAttributeEnumValue | None
    """Full enum value details. Only set for enum values; ``value`` falls back to ``enum_value_id``."""
    created_date: datetime | None
    created_by_user_id: str
    archived_date: datetime | None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: pa_pb.PrincipalAttributeValue, sift_client: SiftClient | None = None
    ) -> PrincipalAttributeAssignment:
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

    @property
    def value(self) -> PrincipalAttributeEnumValue | str | bool | int | None:
        """The assigned value.

        The enum value for ``ENUM``/``SET_OF_ENUM`` keys (or its ID when details were
        not returned), a bool for ``BOOLEAN`` keys, or an int for ``NUMBER`` keys.
        """
        if self.enum_value_id is not None:
            return self.enum_value if self.enum_value is not None else self.enum_value_id
        if self.boolean_value is not None:
            return self.boolean_value
        return self.number_value

    def archive(self) -> PrincipalAttributeAssignment:
        """Archive this assignment."""
        self.client.access_control.principal_attributes.assignments.archive(
            [self], principal_type=self.principal_type
        )
        self._update(
            self.client.access_control.principal_attributes.assignments.get(
                assignment_id=self._id_or_error, principal_type=self.principal_type
            )
        )
        return self

    def unarchive(self) -> PrincipalAttributeAssignment:
        """Unarchive this assignment."""
        self.client.access_control.principal_attributes.assignments.unarchive(
            [self], principal_type=self.principal_type
        )
        self._update(
            self.client.access_control.principal_attributes.assignments.get(
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
        return self.client.access_control.principal_attributes.enum_values.create(
            self, display_name, description=description
        )

    def get_or_create_enum_values(self, names: list[str]) -> list[PrincipalAttributeEnumValue]:
        """Get existing enum values by name, creating any that don't exist."""
        return self.client.access_control.principal_attributes.enum_values.get_or_create(
            self, names
        )

    def list_enum_values(
        self, *, include_archived: bool = False
    ) -> list[PrincipalAttributeEnumValue]:
        """List the enum values defined for this key."""
        return self.client.access_control.principal_attributes.enum_values.list_(
            self, include_archived=include_archived
        )

    def assign_to(
        self,
        principals: list[PrincipalRef | User | str],
        *,
        value: PrincipalAttributeValueLike,
    ) -> list[PrincipalAttributeAssignment]:
        """Assign a value to one or more principals for this key.

        Args:
            principals: Principals to assign to. Pass ``PrincipalRef.user(...)`` /
                ``PrincipalRef.user_group(...)`` references, ``User`` objects, or user
                email addresses (resolved to user IDs automatically).
            value: The value to assign. For ``SET_OF_ENUM`` keys, a list of enum values
                (or their IDs); for ``ENUM`` keys, a single enum value; for ``BOOLEAN``
                keys, a bool; for ``NUMBER`` keys, an int. For ``SET_OF_ENUM`` this
                replaces the full set on each principal.

        Returns:
            The created assignments.
        """
        return self.client.access_control.principal_attributes.assignments.create(
            self, principals, value=value
        )

    def list_assignments(
        self, *, principal_type: PrincipalType = PrincipalType.USER, include_archived: bool = False
    ) -> list[PrincipalAttributeAssignment]:
        """List all assignments of this key for the given principal type."""
        return self.client.access_control.principal_attributes.assignments.list_(
            key=self, principal_type=principal_type, include_archived=include_archived
        )

    def update(self, update: PrincipalAttributeKeyUpdate | dict) -> PrincipalAttributeKey:
        """Update this key.

        Args:
            update: Either a PrincipalAttributeKeyUpdate instance or a dict of fields to update.
        """
        updated = self.client.access_control.principal_attributes.keys.update(self, update=update)
        self._update(updated)
        return self

    def archive(self) -> PrincipalAttributeKey:
        """Archive this key. Cascades to its enum values and assignments."""
        updated = self.client.access_control.principal_attributes.keys.archive(self)
        self._update(updated)
        return self

    def unarchive(self) -> PrincipalAttributeKey:
        """Unarchive this key."""
        updated = self.client.access_control.principal_attributes.keys.unarchive(self)
        self._update(updated)
        return self

    def check_archive_impact(self) -> int:
        """Return the number of active assignments that archiving this key would affect."""
        return self.client.access_control.principal_attributes.keys.check_archive_impact(self)

    def __str__(self) -> str:
        return self.display_name


class PrincipalAttributeKeyUpdate(ModelUpdate[pa_pb.UpdatePrincipalAttributeKeyRequest]):
    """Model of the PrincipalAttributeKey fields that can be updated."""

    display_name: str | None = None
    description: str | None = None

    def _get_proto_class(self) -> type[pa_pb.UpdatePrincipalAttributeKeyRequest]:
        return pa_pb.UpdatePrincipalAttributeKeyRequest

    def _add_resource_id_to_proto(self, proto_msg: pa_pb.UpdatePrincipalAttributeKeyRequest):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.principal_attribute_key_id = self._resource_id
