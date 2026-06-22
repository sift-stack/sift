"""Tests for sift_types principal attribute models."""

import pytest
from sift.principal_attributes.v1 import principal_attributes_pb2 as pa

from sift_client.sift_types.principal_attribute import (
    PrincipalAttributeKey,
    PrincipalAttributeValue,
    PrincipalAttributeValueType,
    PrincipalType,
)


def _key_proto() -> pa.PrincipalAttributeKey:
    return pa.PrincipalAttributeKey(
        principal_attribute_key_id="pk1",
        organization_id="org1",
        display_name="licenses",
        description="license ids",
        type=pa.PRINCIPAL_ATTRIBUTE_VALUE_TYPE_SET_OF_ENUM,
        is_archived=False,
    )


class TestPrincipalAttributeKey:
    def test_from_proto_renames_type_to_value_type(self):
        key = PrincipalAttributeKey._from_proto(_key_proto())
        assert key.id_ == "pk1"
        assert key.value_type == PrincipalAttributeValueType.SET_OF_ENUM
        assert key.archived_date is None

    def test_str_is_display_name(self):
        assert str(PrincipalAttributeKey._from_proto(_key_proto())) == "licenses"


class TestPrincipalAttributeValue:
    def test_from_proto_maps_principal_and_flattens_oneof(self):
        proto = pa.PrincipalAttributeValue(
            principal_attribute_value_id="v1",
            principal_attribute_key_id="pk1",
            principal_id="u1",
            principal_type=pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER,
            principal_attribute_enum_value_id="ev1",
        )
        value = PrincipalAttributeValue._from_proto(proto)
        assert value.principal_id == "u1"
        assert value.principal_type == PrincipalType.USER
        assert value.enum_value_id == "ev1"
        assert value.number_value is None

    def test_archive_passes_principal_type_and_refreshes(self, mock_client):
        proto = pa.PrincipalAttributeValue(
            principal_attribute_value_id="v1",
            principal_attribute_key_id="pk1",
            principal_id="u1",
            principal_type=pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER,
            boolean_value=True,
        )
        value = PrincipalAttributeValue._from_proto(proto)
        value._apply_client_to_instance(mock_client)
        archived_proto = pa.PrincipalAttributeValue(
            principal_attribute_value_id="v1",
            principal_attribute_key_id="pk1",
            principal_id="u1",
            principal_type=pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER,
            boolean_value=True,
            is_archived=True,
        )
        mock_client.principal_attributes.get_value.return_value = (
            PrincipalAttributeValue._from_proto(archived_proto)
        )

        result = value.archive()

        mock_client.principal_attributes.archive_values.assert_called_once_with(
            [value], principal_type=PrincipalType.USER
        )
        assert result is value
        assert value.is_archived is True

    def test_apply_client_cascades_to_nested_key_and_enum_value(self, mock_client):
        proto = pa.PrincipalAttributeValue(
            principal_attribute_value_id="v1",
            principal_attribute_key_id="pk1",
            principal_id="u1",
            principal_type=pa.PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER,
            principal_attribute_enum_value_id="ev1",
            key=pa.PrincipalAttributeKey(principal_attribute_key_id="pk1", display_name="licenses"),
            enum_value_details=pa.PrincipalAttributeEnumValue(
                principal_attribute_enum_value_id="ev1", display_name="LIC_A"
            ),
        )
        value = PrincipalAttributeValue._from_proto(proto)
        value._apply_client_to_instance(mock_client)

        # Nested objects must also carry the client so their convenience methods work.
        assert value.key is not None
        assert value.key.client is mock_client
        assert value.enum_value is not None
        assert value.enum_value.client is mock_client


class TestPrincipalAttributeKeyConvenience:
    def test_assign_to_defaults_to_user(self, mock_client):
        key = PrincipalAttributeKey._from_proto(_key_proto())
        key._apply_client_to_instance(mock_client)
        mock_client.principal_attributes.assign.return_value = []

        key.assign_to(["u1"], value=["LIC_A"])

        mock_client.principal_attributes.assign.assert_called_once_with(
            key, ["u1"], value=["LIC_A"], principal_type=PrincipalType.USER
        )

    def test_client_required(self):
        key = PrincipalAttributeKey._from_proto(_key_proto())
        with pytest.raises(AttributeError, match="Sift client not set"):
            key.check_archive_impact()
