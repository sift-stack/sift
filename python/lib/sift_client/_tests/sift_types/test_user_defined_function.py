"""Tests for sift_types.UserDefinedFunction model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import UserDefinedFunction
from sift_client.sift_types.user_defined_function import (
    FunctionDataType,
    FunctionInput,
    FunctionUsage,
    UserDefinedFunctionCreate,
    UserDefinedFunctionUpdate,
    UserDefinedFunctionVersion,
)

NOW = datetime(2026, 1, 1, tzinfo=timezone.utc)


class TestFunctionInput:
    """Unit tests for FunctionInput."""

    def test_defaults(self):
        given = FunctionInput(identifier="$1")

        assert given.data_type is FunctionDataType.NUMERIC
        assert given.scalar is False

    def test_round_trip(self):
        from sift.common.type.v1.user_defined_functions_pb2 import FunctionInput as Proto

        original = FunctionInput(identifier="$1", data_type=FunctionDataType.STRING, scalar=True)
        proto = Proto(identifier="$1", data_type=FunctionDataType.STRING.value, constant=True)

        assert FunctionInput._from_proto(proto) == original


class TestUserDefinedFunctionCreate:
    """Unit tests for UserDefinedFunctionCreate - tests _to_proto_helpers."""

    def test_minimal_create(self):
        proto = UserDefinedFunctionCreate(
            name="double", expression="$1 * 2", function_inputs=[FunctionInput(identifier="$1")]
        ).to_proto()

        assert proto.name == "double"
        assert proto.expression == "$1 * 2"

    def test_function_inputs_converter(self):
        proto = UserDefinedFunctionCreate(
            name="double",
            expression="$1 * 2",
            function_inputs=[
                FunctionInput(identifier="$1"),
                FunctionInput(identifier="$2", data_type=FunctionDataType.STRING, scalar=True),
            ],
        ).to_proto()

        assert len(proto.function_inputs) == 2
        assert proto.function_inputs[0].identifier == "$1"
        assert proto.function_inputs[0].data_type == FunctionDataType.NUMERIC.value
        assert proto.function_inputs[1].data_type == FunctionDataType.STRING.value
        assert proto.function_inputs[1].constant is True

    def test_metadata_converter(self):
        proto = UserDefinedFunctionCreate(
            name="double",
            expression="$1 * 2",
            function_inputs=[FunctionInput(identifier="$1")],
            metadata={"owner": "ops", "n": 2.0},
        ).to_proto()

        by_key = {m.key.name: m for m in proto.metadata}
        assert by_key["owner"].string_value == "ops"
        assert by_key["n"].number_value == 2.0


class TestUserDefinedFunctionUpdate:
    """Unit tests for UserDefinedFunctionUpdate - tests field masks."""

    def test_update_mask_only_includes_set_fields(self):
        update = UserDefinedFunctionUpdate(expression="$1 * 3", description="tripled")
        update.resource_id = "fn-1"

        proto, mask = update.to_proto_with_mask()

        assert proto.user_defined_function_id == "fn-1"
        assert proto.expression == "$1 * 3"
        assert set(mask.paths) == {"expression", "description"}

    def test_archive_update(self):
        update = UserDefinedFunctionUpdate(is_archived=True)
        update.resource_id = "fn-1"

        proto, mask = update.to_proto_with_mask()

        assert proto.is_archived is True
        assert mask.paths == ["is_archived"]

    def test_requires_resource_id(self):
        with pytest.raises(ValueError, match="Resource ID must be set"):
            UserDefinedFunctionUpdate(expression="$1").to_proto_with_mask()


class TestFunctionUsage:
    """Unit tests for FunctionUsage."""

    def test_any_is_false_when_empty(self):
        assert FunctionUsage().is_used is False

    def test_any_is_true_with_one_rule(self):
        usage = FunctionUsage.model_construct(rules=[object()])

        assert usage.is_used is True


@pytest.fixture
def mock_function(mock_client):
    """Create a mock UserDefinedFunction instance for testing."""
    latest = UserDefinedFunctionVersion(
        proto=MagicMock(),
        id_="ver-1",
        user_defined_function_id="fn-1",
        version=1,
        expression="$1 * 2",
        change_message="created",
        change_notes="",
        function_inputs=[FunctionInput(identifier="$1")],
        dependency_version_ids=[],
        created_date=NOW,
        created_by_user_id="user1",
        output_type=FunctionDataType.NUMERIC,
    )
    function = UserDefinedFunction(
        proto=MagicMock(),
        id_="fn-1",
        name="double",
        description="doubles the input",
        metadata={},
        created_date=NOW,
        modified_date=NOW,
        created_by_user_id="user1",
        modified_by_user_id="user1",
        is_archived=False,
        latest_version=latest,
        archived_date=None,
    )
    function._apply_client_to_instance(mock_client)
    return function


class TestUserDefinedFunction:
    """Unit tests for UserDefinedFunction model - tests properties and methods."""

    def test_versions_property_calls_client(self, mock_function, mock_client):
        mock_client.user_defined_functions.versions.list_.return_value = []

        _ = mock_function.versions

        mock_client.user_defined_functions.versions.list_.assert_called_once_with(
            user_defined_function=mock_function, order_by="version desc"
        )

    def test_update_calls_client_and_updates_self(self, mock_function, mock_client):
        updated = MagicMock()
        mock_client.user_defined_functions.update.return_value = updated

        with MagicMock() as mock_update:
            mock_function._update = mock_update

            update = UserDefinedFunctionUpdate(expression="$1 * 3")
            result = mock_function.update(update)

            mock_client.user_defined_functions.update.assert_called_once_with(
                user_defined_function=mock_function, update=update
            )
            mock_update.assert_called_once_with(updated)
            assert result is mock_function

    def test_archive_calls_client(self, mock_function, mock_client):
        mock_client.user_defined_functions.archive.return_value = MagicMock()

        with MagicMock() as mock_update:
            mock_function._update = mock_update
            result = mock_function.archive()

            mock_client.user_defined_functions.archive.assert_called_once_with(
                user_defined_function=mock_function
            )
            assert result is mock_function

    def test_unarchive_calls_client(self, mock_function, mock_client):
        mock_client.user_defined_functions.unarchive.return_value = MagicMock()

        with MagicMock() as mock_update:
            mock_function._update = mock_update
            result = mock_function.unarchive()

            mock_client.user_defined_functions.unarchive.assert_called_once_with(
                user_defined_function=mock_function
            )
            assert result is mock_function
