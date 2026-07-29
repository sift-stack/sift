"""Domain types for users.

A ``User`` identifies a person in a Sift organization. The user's ``name`` is their
login name, typically their email address.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from pydantic import BaseModel
from sift.common.type.v1.user_pb2 import User as UserProto

from sift_client.sift_types._base import BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class UserOrganization(BaseModel):
    """An organization a user belongs to."""

    organization_id: str
    organization_name: str
    is_abac_enabled: bool | None


class User(BaseType[UserProto, "User"]):
    """A Sift user.

    ``name`` is the user's login name, typically their email address.
    """

    name: str
    organizations: list[UserOrganization]

    @classmethod
    def _from_proto(cls, proto: UserProto, sift_client: SiftClient | None = None) -> User:
        return cls(
            proto=proto,
            id_=proto.user_id,
            name=proto.user_name,
            organizations=[
                UserOrganization(
                    organization_id=org.organization_id,
                    organization_name=org.organization_name,
                    is_abac_enabled=(
                        org.is_abac_enabled if org.HasField("is_abac_enabled") else None
                    ),
                )
                for org in proto.organizations
            ],
            _client=sift_client,
        )

    def __str__(self) -> str:
        return self.name
