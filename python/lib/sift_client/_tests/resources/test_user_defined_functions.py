"""Pytest tests for the User Defined Functions API.

These tests demonstrate and validate the usage of the UserDefinedFunctionsAPI including:
- Basic function operations (get, list, find)
- Creation, updates, and archiving
- Versions, validation, dependents, and sync
"""

from datetime import datetime, timezone

import pytest

from sift_client import SiftClient
from sift_client.resources import (
    UserDefinedFunctionsAPI,
    UserDefinedFunctionsAPIAsync,
    UserDefinedFunctionVersionsAPI,
)
from sift_client.sift_types import UserDefinedFunction
from sift_client.sift_types.user_defined_function import (
    FunctionDataType,
    FunctionInput,
    UserDefinedFunctionCreate,
    UserDefinedFunctionUpdate,
)

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert isinstance(sift_client.user_defined_functions, UserDefinedFunctionsAPI)
    assert isinstance(sift_client.user_defined_functions.versions, UserDefinedFunctionVersionsAPI)
    assert isinstance(sift_client.async_.user_defined_functions, UserDefinedFunctionsAPIAsync)


@pytest.fixture(scope="session")
def test_timestamp_str():
    """A per-session suffix so function names stay unique across runs."""
    return datetime.now(timezone.utc).strftime("%Y%m%d%H%M%S")


@pytest.fixture
def functions_api_async(sift_client: SiftClient):
    """Get the async functions API instance."""
    return sift_client.async_.user_defined_functions


@pytest.fixture(scope="session")
def new_function(sift_client, test_timestamp_str):
    """Create a function for the session and archive it on teardown."""
    created = sift_client.user_defined_functions.create(
        UserDefinedFunctionCreate(
            name=f"test_fn_{test_timestamp_str}",
            description="Created by Sift Client pytest",
            expression="$1 * 2",
            function_inputs=[FunctionInput(identifier="$1")],
        )
    )
    yield created
    sift_client.user_defined_functions.archive(created)


class TestUserDefinedFunctions:
    """Tests for the User Defined Functions API."""

    def test_create(self, new_function, test_timestamp_str):
        """Test that create returns a fully populated function."""
        assert isinstance(new_function, UserDefinedFunction)
        assert new_function.id_ is not None
        assert new_function.name == f"test_fn_{test_timestamp_str}"
        assert new_function.expression == "$1 * 2"
        assert [i.identifier for i in new_function.function_inputs] == ["$1"]
        assert new_function.version == 1
        assert new_function.version_id
        assert new_function.is_archived is False

    def test_get(self, sift_client, new_function):
        """Test getting a function by ID."""
        fetched = sift_client.user_defined_functions.get(new_function._id_or_error)

        assert fetched.id_ == new_function.id_
        assert fetched.name == new_function.name

    def test_basic_list(self, sift_client, new_function):
        """Test basic function listing functionality."""
        functions = sift_client.user_defined_functions.list_(limit=5)

        assert isinstance(functions, list)
        for function in functions:
            assert isinstance(function, UserDefinedFunction)
            assert function.id_ is not None

    def test_list_with_name_filter(self, sift_client, new_function):
        """Test function listing with name filtering."""
        by_name = sift_client.user_defined_functions.list_(name=new_function.name)

        assert [f.id_ for f in by_name] == [new_function.id_]

    def test_list_with_id_filter(self, sift_client, new_function):
        """Test function listing filtered to specific IDs."""
        functions = sift_client.user_defined_functions.list_(
            function_ids=[new_function._id_or_error]
        )

        assert [f.id_ for f in functions] == [new_function.id_]

    def test_find(self, sift_client, new_function):
        """Test finding a single function."""
        found = sift_client.user_defined_functions.find(name=new_function.name)

        assert found is not None
        assert found.id_ == new_function.id_

    def test_find_nonexistent(self, sift_client):
        """Test finding a non-existent function returns None."""
        found = sift_client.user_defined_functions.find(
            name=f"nonexistent_fn_{datetime.now(timezone.utc).timestamp()}"
        )
        assert found is None

    def test_update_creates_a_version(self, sift_client, new_function):
        """Test that updating a function bumps its version."""
        updated = sift_client.user_defined_functions.update(
            new_function, UserDefinedFunctionUpdate(description="tweaked")
        )

        assert updated.description == "tweaked"
        assert updated.version > new_function.version
        # The name was not in the mask, so it is unchanged.
        assert updated.name == new_function.name

    def test_update_accepts_dict(self, sift_client, new_function):
        """Test that update accepts a plain dict."""
        updated = sift_client.user_defined_functions.update(
            new_function._id_or_error, {"description": "via dict"}
        )

        assert updated.description == "via dict"

    def test_archive_and_unarchive(self, sift_client, test_timestamp_str):
        """Test archiving and unarchiving a function."""
        function = sift_client.user_defined_functions.create(
            UserDefinedFunctionCreate(
                name=f"test_fn_archive_{test_timestamp_str}",
                expression="$1 + 1",
                function_inputs=[FunctionInput(identifier="$1")],
            )
        )

        archived = sift_client.user_defined_functions.archive(function)
        assert archived.is_archived is True

        # Archived functions are excluded from list_ by default.
        assert sift_client.user_defined_functions.find(name=function.name) is None
        assert (
            sift_client.user_defined_functions.find(name=function.name, include_archived=True).id_
            == function.id_
        )

        unarchived = sift_client.user_defined_functions.unarchive(function)
        assert unarchived.is_archived is False

        sift_client.user_defined_functions.archive(function)

    def test_instance_methods(self, sift_client, test_timestamp_str):
        """Test the update and archive methods on the instance itself."""
        function = sift_client.user_defined_functions.create(
            UserDefinedFunctionCreate(
                name=f"test_fn_instance_{test_timestamp_str}",
                expression="$1 - 1",
                function_inputs=[FunctionInput(identifier="$1")],
            )
        )

        function.update({"description": "from-instance"})
        assert function.description == "from-instance"

        function.archive()
        assert function.is_archived is True

    def test_validate_accepts_a_good_expression(self, sift_client):
        """Test validating an expression that compiles."""
        result = sift_client.user_defined_functions.validate_expression(
            "$1 * 2", [FunctionInput(identifier="$1")]
        )

        assert result.valid is True
        assert result.error is None

    def test_validate_rejects_a_bad_expression(self, sift_client):
        """Test validating an expression that does not compile."""
        result = sift_client.user_defined_functions.validate_expression(
            "$1 ** ** 2", [FunctionInput(identifier="$1")]
        )

        assert result.valid is False
        assert result.error

    def test_dependents_of_an_unused_function(self, sift_client, new_function):
        """Test that a function nothing uses reports no dependents."""
        dependents = sift_client.user_defined_functions.get_where_used(new_function)

        assert dependents.is_used is False

    @pytest.mark.asyncio
    async def test_async_list(self, functions_api_async, new_function):
        """Test the async API returns the same functions."""
        functions = await functions_api_async.list_(function_ids=[new_function._id_or_error])

        assert [f.id_ for f in functions] == [new_function.id_]


class TestUserDefinedFunctionVersions:
    """Tests for the nested versions API."""

    def test_list_versions(self, sift_client, new_function):
        """Test listing a function's versions."""
        versions = sift_client.user_defined_functions.versions.list_(
            user_defined_function=new_function
        )

        assert versions
        assert all(v.id_ == new_function.id_ for v in versions)

    def test_get_version(self, sift_client, new_function):
        """Test getting one version by ID."""
        version = sift_client.user_defined_functions.versions.get(new_function.version_id)

        assert version.version_id == new_function.version_id

    def test_versions_property(self, sift_client, new_function):
        """Test the versions property on the instance."""
        assert new_function.versions


class TestUserDefinedFunctionSync:
    """Tests for sync, which creates or updates by name."""

    def test_sync_creates_then_updates(self, sift_client, test_timestamp_str):
        """Test that sync creates a missing function and updates an existing one."""
        name = f"test_fn_sync_{test_timestamp_str}"

        created = sift_client.user_defined_functions.sync(
            [
                UserDefinedFunctionCreate(
                    name=name,
                    expression="$1 * 2",
                    function_inputs=[FunctionInput(identifier="$1")],
                )
            ]
        )
        assert len(created) == 1
        assert created[0].name == name
        first_version = created[0].version

        updated = sift_client.user_defined_functions.sync(
            [
                UserDefinedFunctionCreate(
                    name=name,
                    expression="$1 * 2",
                    function_inputs=[FunctionInput(identifier="$1")],
                    description="second pass",
                )
            ]
        )
        assert updated[0].id_ == created[0].id_
        assert updated[0].version > first_version
        assert updated[0].description == "second pass"

        sift_client.user_defined_functions.archive(updated[0])

    def test_sync_is_ordered(self, sift_client, test_timestamp_str):
        """Test that sync returns results in the order given."""
        names = [f"test_fn_sync{i}_{test_timestamp_str}" for i in range(2)]

        results = sift_client.user_defined_functions.sync(
            [
                UserDefinedFunctionCreate(
                    name=n, expression="$1 + 1", function_inputs=[FunctionInput(identifier="$1")]
                )
                for n in names
            ]
        )

        assert [r.name for r in results] == names
        for result in results:
            sift_client.user_defined_functions.archive(result)


class TestFunctionDataTypes:
    """Tests that non-numeric input types round trip."""

    def test_string_input(self, sift_client, test_timestamp_str):
        """Test a function with a string input."""
        function = sift_client.user_defined_functions.create(
            UserDefinedFunctionCreate(
                name=f"test_fn_string_{test_timestamp_str}",
                expression="$1",
                function_inputs=[FunctionInput(identifier="$1", data_type=FunctionDataType.STRING)],
            )
        )

        assert function.function_inputs[0].data_type is FunctionDataType.STRING
        sift_client.user_defined_functions.archive(function)
