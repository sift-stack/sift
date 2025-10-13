"""Unit tests for sift_types._base module."""

from __future__ import annotations

from datetime import datetime, timezone
from typing import ClassVar
from unittest.mock import MagicMock

import pytest
from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CalculatedChannel as CalculatedChannelProto,
)
from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CreateCalculatedChannelRequest,
)

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelUpdate,
)


class SimpleCreateModel(ModelCreate[CreateCalculatedChannelRequest]):
    """Simple model for testing basic field mapping."""

    name: str
    description: str | None = None
    units: str | None = None

    def _get_proto_class(self) -> type[CreateCalculatedChannelRequest]:
        return CreateCalculatedChannelRequest


class NestedCreateModel(ModelCreate[CreateCalculatedChannelRequest]):
    """Model for testing nested field mapping with MappingHelper."""

    name: str
    description: str | None = None
    expression: str | None = None
    all_assets: bool | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "expression": MappingHelper(
            proto_attr_path="calculated_channel_configuration.query_configuration.sel.expression",
            update_field="query_configuration",
        ),
        "all_assets": MappingHelper(
            proto_attr_path="calculated_channel_configuration.asset_configuration.all_assets",
        ),
    }

    def _get_proto_class(self) -> type[CreateCalculatedChannelRequest]:
        return CreateCalculatedChannelRequest


class SimpleUpdateModel(ModelUpdate[CalculatedChannelProto]):
    """Simple model for testing update with field masks."""

    name: str | None = None
    description: str | None = None
    units: str | None = None

    def _get_proto_class(self) -> type[CalculatedChannelProto]:
        return CalculatedChannelProto

    def _add_resource_id_to_proto(self, proto_msg: CalculatedChannelProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.calculated_channel_id = self._resource_id


class TestModelCreate:
    """Tests for ModelCreate base class."""

    def test_simple_create_with_all_fields(self):
        """Test creating a proto with all fields set."""
        model = SimpleCreateModel(
            name="test_name", description="test_description", units="test_units"
        )
        proto = model.to_proto()

        assert proto.name == "test_name"
        assert proto.description == "test_description"
        assert proto.units == "test_units"

    def test_simple_create_with_none_fields_excluded(self):
        """Test that None fields are excluded from proto."""
        model = SimpleCreateModel(name="test_name", description=None, units=None)
        proto = model.to_proto()

        assert proto.name == "test_name"
        # Proto should not have description or units set
        assert proto.description == ""  # Proto default for string
        assert proto.units == ""  # Proto default for string

    def test_simple_create_with_unset_fields_excluded(self):
        """Test that unset fields are excluded from proto."""
        model = SimpleCreateModel(name="test_name")
        proto = model.to_proto()

        assert proto.name == "test_name"
        # Proto should not have description or units set
        assert proto.description == ""  # Proto default for string
        assert proto.units == ""  # Proto default for string

    def test_nested_create_with_mapping_helper(self):
        """Test creating a proto with nested field mapping."""
        model = NestedCreateModel(
            name="test_name",
            description="test_description",
            expression="$1 + $2",
            all_assets=True,
        )
        proto = model.to_proto()

        assert proto.name == "test_name"
        assert proto.description == "test_description"
        # Check nested fields
        assert (
            proto.calculated_channel_configuration.query_configuration.sel.expression == "$1 + $2"
        )
        assert proto.calculated_channel_configuration.asset_configuration.all_assets is True

    def test_nested_create_with_none_nested_fields(self):
        """Test that None values in nested fields are excluded."""
        model = NestedCreateModel(
            name="test_name",
            description="test_description",
            expression=None,
            all_assets=None,
        )
        proto = model.to_proto()

        assert proto.name == "test_name"
        assert proto.description == "test_description"
        # Nested fields should not be set
        assert (
            proto.calculated_channel_configuration.query_configuration.sel.expression == ""
        )  # Proto default
        assert (
            proto.calculated_channel_configuration.asset_configuration.all_assets is False
        )  # Proto default for bool

    def test_nested_create_with_unset_nested_fields(self):
        """Test that unset nested fields are excluded."""
        model = NestedCreateModel(name="test_name", description="test_description")
        proto = model.to_proto()

        assert proto.name == "test_name"
        assert proto.description == "test_description"
        # Nested fields should not be set
        assert (
            proto.calculated_channel_configuration.query_configuration.sel.expression == ""
        )  # Proto default
        assert (
            proto.calculated_channel_configuration.asset_configuration.all_assets is False
        )  # Proto default for bool

    def test_mixed_none_and_set_fields(self):
        """Test model with mix of None, unset, and set fields."""
        model = NestedCreateModel(
            name="test_name",
            description=None,  # Explicitly None
            expression="$1 + $2",  # Set
            # all_assets is unset
        )
        proto = model.to_proto()

        assert proto.name == "test_name"
        assert proto.description == ""  # None excluded, proto default
        assert (
            proto.calculated_channel_configuration.query_configuration.sel.expression == "$1 + $2"
        )
        assert (
            proto.calculated_channel_configuration.asset_configuration.all_assets is False
        )  # Unset, proto default


class TestModelUpdate:
    """Tests for ModelUpdate base class."""

    def test_simple_update_with_field_mask(self):
        """Test updating a proto with field mask."""
        model = SimpleUpdateModel(name="new_name", description="new_description")
        model.resource_id = "test_id"

        proto, mask = model.to_proto_with_mask()

        assert proto.calculated_channel_id == "test_id"
        assert proto.name == "new_name"
        assert proto.description == "new_description"
        assert set(mask.paths) == {"name", "description"}

    def test_update_with_none_value_excluded(self):
        """Test that explicitly setting a field to None excludes it in the mask."""
        model = SimpleUpdateModel(name="new_name", description=None)
        model.resource_id = "test_id"

        proto, mask = model.to_proto_with_mask()

        assert proto.calculated_channel_id == "test_id"
        assert proto.name == "new_name"

        assert "description" not in mask.paths
        assert "name" in mask.paths

    def test_update_with_unset_fields_excluded(self):
        """Test that unset fields are excluded from the mask."""
        model = SimpleUpdateModel(name="new_name")
        model.resource_id = "test_id"

        proto, mask = model.to_proto_with_mask()

        assert proto.calculated_channel_id == "test_id"
        assert proto.name == "new_name"
        # Only name should be in the mask
        assert mask.paths == ["name"]

    def test_update_requires_resource_id(self):
        """Test that update fails without resource_id."""
        model = SimpleUpdateModel(name="new_name")

        with pytest.raises(ValueError, match="Resource ID must be set"):
            model.to_proto_with_mask()


class TestMappingHelper:
    """Tests for MappingHelper functionality."""

    def test_mapping_helper_basic(self):
        """Test basic MappingHelper creation."""
        helper = MappingHelper(proto_attr_path="field.nested.path")
        assert helper.proto_attr_path == "field.nested.path"
        assert helper.update_field is None
        assert helper.converter is None

    def test_mapping_helper_with_update_field(self):
        """Test MappingHelper with update_field."""
        helper = MappingHelper(proto_attr_path="field.nested.path", update_field="field")
        assert helper.proto_attr_path == "field.nested.path"
        assert helper.update_field == "field"

    def test_mapping_helper_with_converter(self):
        """Test MappingHelper with converter function."""

        def converter(x):
            return x.upper()

        helper = MappingHelper(proto_attr_path="field.path", converter=converter)
        assert helper.converter is not None
        assert helper.converter("test") == "TEST"


class TestEdgeCases:
    """Tests for edge cases and regression prevention."""

    def test_empty_model_create(self):
        """Test creating with only required fields."""
        model = SimpleCreateModel(name="test")
        proto = model.to_proto()
        assert proto.name == "test"

    def test_nested_path_expansion(self):
        """Test that nested paths are properly expanded."""
        model = NestedCreateModel(name="test", expression="$1 + $2")
        proto = model.to_proto()

        # Verify the nested structure was created
        assert proto.HasField("calculated_channel_configuration")
        assert proto.calculated_channel_configuration.HasField("query_configuration")
        assert proto.calculated_channel_configuration.query_configuration.HasField("sel")
        assert (
            proto.calculated_channel_configuration.query_configuration.sel.expression == "$1 + $2"
        )

    def test_multiple_nested_fields_same_parent(self):
        """Test multiple nested fields that share a parent path."""
        model = NestedCreateModel(name="test", expression="$1 + $2", all_assets=True)
        proto = model.to_proto()

        # Both fields should be set under calculated_channel_configuration
        assert (
            proto.calculated_channel_configuration.query_configuration.sel.expression == "$1 + $2"
        )
        assert proto.calculated_channel_configuration.asset_configuration.all_assets is True

    def test_validation_error_on_invalid_helper_field(self):
        """Test that MappingHelper validation catches mismatched fields."""

        class InvalidModel(ModelCreate[CreateCalculatedChannelRequest]):
            name: str

            _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
                "nonexistent_field": MappingHelper(proto_attr_path="some.path"),
            }

            def _get_proto_class(self):
                return CreateCalculatedChannelRequest

        with pytest.raises(ValueError, match="MappingHelper created for"):
            # This should raise during __init__
            InvalidModel(name="test")


class TestBaseType:
    """Tests for BaseType base class."""

    def test_base_type_concrete_implementation(self):
        """Test creating a concrete BaseType implementation."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str
            created_date: datetime | None = None

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name, _client=sift_client)

        model = TestModel(name="test", id_="test_id")
        assert model.name == "test"
        assert model.id_ == "test_id"

    def test_id_or_error_with_id(self):
        """Test _id_or_error property when ID is set."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name)

        model = TestModel(name="test", id_="test_id_123")
        assert model._id_or_error == "test_id_123"

    def test_id_or_error_without_id(self):
        """Test _id_or_error property raises when ID is not set."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name)

        model = TestModel(name="test")
        with pytest.raises(ValueError, match="ID is not set"):
            _ = model._id_or_error

    def test_client_property_without_client(self):
        """Test client property raises when client is not set."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name)

        model = TestModel(name="test")
        with pytest.raises(AttributeError, match="Sift client not set"):
            _ = model.client

    def test_apply_client_to_instance(self):
        """Test _apply_client_to_instance method."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name)

        model = TestModel(name="test")
        assert model._client is None

        mock_client = MagicMock()
        model._apply_client_to_instance(mock_client)
        assert model._client is mock_client
        assert model.client is mock_client

    def test_update_method(self):
        """Test _update method updates fields from another instance."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str
            description: str | None = None
            version: int | None = None

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(
                    name=proto.name,
                    description=proto.description,
                    version=proto.version,
                    proto=proto,
                )

        # Create original model
        original = TestModel(name="original", description="old desc", version=1, id_="id1")

        # Create updated model
        mock_proto = MagicMock()
        updated = TestModel(
            name="updated",
            description="new desc",
            version=2,
            id_="id1",
            proto=mock_proto,
        )

        # Update original with updated values
        result = original._update(updated)

        assert result is original  # Returns self
        assert original.name == "updated"
        assert original.description == "new desc"
        assert original.version == 2
        assert original.proto is mock_proto

    def test_validate_timezones_with_valid_datetime(self):
        """Test timezone validation passes with timezone-aware datetime."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str
            created_date: datetime | None = None

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name, created_date=proto.created_date)

        # Should not raise
        model = TestModel(name="test", created_date=datetime.now(timezone.utc))
        assert model.created_date.tzinfo is not None

    def test_validate_timezones_with_naive_datetime(self):
        """Test timezone validation fails with naive datetime."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str
            created_date: datetime

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name, created_date=proto.created_date)

        # Should raise validation error
        with pytest.raises(ValueError, match="must have timezone information"):
            TestModel(name="test", created_date=datetime.now())  # noqa: DTZ005

    def test_validate_timezones_with_none_datetime(self):
        """Test timezone validation passes when datetime is None."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str
            created_date: datetime | None = None

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name)

        # Should not raise
        model = TestModel(name="test", created_date=None)
        assert model.created_date is None

    def test_proto_field_excluded_from_dump(self):
        """Test that proto field is excluded from model_dump."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name, proto=proto)

        mock_proto = MagicMock()
        model = TestModel(name="test", proto=mock_proto)

        dumped = model.model_dump()
        assert "proto" not in dumped
        assert "name" in dumped

    def test_frozen_model_config(self):
        """Test that BaseType models are frozen."""

        class TestModel(BaseType[CalculatedChannelProto, "TestModel"]):
            name: str

            @classmethod
            def _from_proto(cls, proto, sift_client=None):
                return cls(name=proto.name)

        model = TestModel(name="test")

        # Should not be able to modify frozen model
        with pytest.raises(Exception):  # noqa: B017, PT011
            model.name = "new_name"


class TestBuildProtoAndPaths:
    """Tests specifically for _build_proto_and_paths method."""

    def test_build_proto_simple_fields(self):
        """Test building proto with simple scalar fields."""
        model = SimpleCreateModel(name="test", description="desc")
        proto = CreateCalculatedChannelRequest()

        paths = model._build_proto_and_paths(proto, {"name": "test", "description": "desc"})

        assert proto.name == "test"
        assert proto.description == "desc"
        assert set(paths) == {"name", "description"}

    def test_build_proto_with_prefix(self):
        """Test building proto with path prefix."""
        model = SimpleCreateModel(name="test")
        proto = CreateCalculatedChannelRequest()

        paths = model._build_proto_and_paths(proto, {"name": "test"}, prefix="parent")

        assert proto.name == "test"
        assert paths == ["parent.name"]

    def test_build_proto_with_nested_dict(self):
        """Test building proto with nested dictionary through normal flow."""
        # This tests that the MappingHelper properly expands nested paths
        model = NestedCreateModel(name="test", expression="$1 + $2")
        proto = model.to_proto()

        # Verify the nested structure was created correctly
        assert (
            proto.calculated_channel_configuration.query_configuration.sel.expression == "$1 + $2"
        )
        assert proto.name == "test"

    def test_build_proto_with_submessage_dict(self):
        """Test building proto when data contains a dict for a submessage field."""

        class SubmessageModel(ModelCreate[CreateCalculatedChannelRequest]):
            name: str

            def _get_proto_class(self):
                return CreateCalculatedChannelRequest

        model = SubmessageModel(name="test")
        proto = CreateCalculatedChannelRequest()

        # Test that we can build nested structures by passing dict data
        # This simulates what happens when processing nested proto messages
        data = {"name": "test"}
        paths = model._build_proto_and_paths(proto, data)

        assert proto.name == "test"
        assert "name" in paths

    def test_build_proto_with_mapping_helper_update_field(self):
        """Test that mapping helper's update_field is added to paths."""
        model = NestedCreateModel(name="test", expression="$1 + $2")
        proto = CreateCalculatedChannelRequest()

        data = {"name": "test", "expression": "$1 + $2"}
        paths = model._build_proto_and_paths(proto, data)

        assert "query_configuration" in paths
        assert "name" in paths
        assert (
            proto.calculated_channel_configuration.query_configuration.sel.expression == "$1 + $2"
        )

    def test_build_proto_error_on_invalid_field(self):
        """Test that setting an invalid field raises TypeError."""
        model = SimpleCreateModel(name="test")
        proto = CreateCalculatedChannelRequest()

        with pytest.raises(TypeError, match="Can't set"):
            model._build_proto_and_paths(proto, {"nonexistent_field": "value"})

    def test_build_proto_already_setting_path_override(self):
        """Test that already_setting_path_override skips helper processing."""
        model = NestedCreateModel(name="test")
        proto = CreateCalculatedChannelRequest()

        # When already_setting_path_override=True, it should skip the helper
        # and try to process the field directly
        data = {"name": "test"}
        paths = model._build_proto_and_paths(proto, data, already_setting_path_override=True)

        assert proto.name == "test"
        assert "name" in paths
