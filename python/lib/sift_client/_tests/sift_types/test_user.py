"""Tests for the sift_types user model."""

from sift.common.type.v1.organization_pb2 import Organization
from sift.common.type.v1.user_pb2 import User as UserProto

from sift_client.sift_types.user import User


def _user_proto() -> UserProto:
    return UserProto(
        user_id="u1",
        user_name="alice@example.com",
        organizations=[
            Organization(organization_id="org1", organization_name="Acme", is_abac_enabled=True)
        ],
    )


class TestUser:
    def test_from_proto_maps_id_and_name(self):
        user = User._from_proto(_user_proto())

        assert user.id_ == "u1"
        assert user.name == "alice@example.com"

    def test_from_proto_maps_organizations(self):
        user = User._from_proto(_user_proto())

        assert len(user.organizations) == 1
        org = user.organizations[0]
        assert org.organization_id == "org1"
        assert org.organization_name == "Acme"
        assert org.is_abac_enabled is True

    def test_from_proto_unset_abac_flag_is_none(self):
        proto = UserProto(
            user_id="u1",
            user_name="alice@example.com",
            organizations=[Organization(organization_id="org1", organization_name="Acme")],
        )

        user = User._from_proto(proto)

        assert user.organizations[0].is_abac_enabled is None

    def test_str_is_name(self):
        assert str(User._from_proto(_user_proto())) == "alice@example.com"
