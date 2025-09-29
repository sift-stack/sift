"""Pytest tests for the Runs API.

These tests demonstrate and validate the usage of the Runs API including:
- Basic run operations (get, list, find)
- Run filtering and searching
- Run creation, updates, and archiving
- Error handling and edge cases
"""

import os

import pytest

from sift_client import SiftClient
from sift_client.resources import RunsAPI, RunsAPIAsync
from sift_client.sift_types import Run

pytestmark = pytest.mark.integration


@pytest.fixture(scope="session")
def sift_client() -> SiftClient:
    """Create a SiftClient instance for testing."""
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_API_KEY", "")

    return SiftClient(
        api_key=api_key,
        grpc_url=grpc_url,
        rest_url=rest_url,
    )


def test_client_binding(sift_client):
    assert sift_client.runs
    assert isinstance(sift_client.runs, RunsAPI)
    assert sift_client.async_.runs
    assert isinstance(sift_client.async_.runs, RunsAPIAsync)


@pytest.fixture
def runs_api_async(sift_client: SiftClient):
    """Get the async runs API instance."""
    return sift_client.async_.runs


@pytest.fixture
def runs_api_sync(sift_client: SiftClient):
    """Get the synchronous runs API instance."""
    return sift_client.runs


@pytest.fixture
def test_run(runs_api_sync):
    runs = runs_api_sync.list_(limit=1)
    assert runs
    assert len(runs) >= 1
    return runs[0]


class TestRunsAPIAsync:
    """Test suite for the async Runs API functionality."""

    class TestList:
        """Tests for the async list method."""

        @pytest.mark.asyncio
        async def test_basic_list(self, runs_api_async):
            """Test basic run listing functionality."""
            runs = await runs_api_async.list_(limit=5)

            # Verify we get a list
            assert isinstance(runs, list)
            assert runs

            # If we have runs, verify their structure
            run = runs[0]
            assert isinstance(run, Run)
            assert run.id_ is not None
            assert run.name is not None

        @pytest.mark.asyncio
        async def test_list_with_name_filter(self, runs_api_async):
            """Test run listing with name filtering."""
            # First get some runs to work with
            all_runs = await runs_api_async.list_(limit=10)

            if all_runs:
                # Use the first run's name for filtering
                test_run_name = all_runs[0].name
                filtered_runs = await runs_api_async.list_(name=test_run_name)

                # Should find at least one run with exact name match
                assert isinstance(filtered_runs, list)
                assert len(filtered_runs) >= 1

                # All returned runs should have the exact name
                for run in filtered_runs:
                    assert run.name == test_run_name

        @pytest.mark.asyncio
        async def test_list_with_name_contains_filter(self, runs_api_async):
            """Test run listing with name contains filtering."""
            # Test with a common substring that might exist in run names
            runs = await runs_api_async.list_(name_contains="test", limit=5)

            assert isinstance(runs, list)

            # If we found runs, verify they contain the substring
            for run in runs:
                assert "test" in run.name.lower()

        @pytest.mark.asyncio
        async def test_list_with_limit(self, runs_api_async):
            """Test run listing with different limits."""
            # Test with limit of 1
            runs_1 = await runs_api_async.list_(limit=1)
            assert isinstance(runs_1, list)
            assert len(runs_1) <= 1

            # Test with limit of 3
            runs_3 = await runs_api_async.list_(limit=3)
            assert isinstance(runs_3, list)
            assert len(runs_3) <= 3

        @pytest.mark.asyncio
        async def test_list_include_archived(self, runs_api_async):
            """Test run listing with archived runs included."""
            # Test without archived runs (default)
            runs_active = await runs_api_async.list_(limit=5, include_archived=False)
            assert isinstance(runs_active, list)

            # Test with archived runs included
            runs_all = await runs_api_async.list_(limit=5, include_archived=True)
            assert isinstance(runs_all, list)

            # Should have at least as many runs when including archived
            assert len(runs_all) >= len(runs_active)

    class TestGet:
        """Tests for the async get method."""

        @pytest.mark.asyncio
        async def test_get_by_id(self, runs_api_async, test_run):
            """Test getting a specific run by ID."""
            retrieved_run = await runs_api_async.get(run_id=test_run.id_)

            assert retrieved_run is not None
            assert retrieved_run.id_ == test_run.id_

        # TODO: test for client key
        @pytest.mark.asyncio
        async def test_get_by_id_with_client_key(self, runs_api_async, test_run):
            """Test getting a specific run by client key."""
            assert test_run.client_key is not None
            retrieved_run = await runs_api_async.get(client_key=test_run.client_key)

            assert retrieved_run is not None
            assert retrieved_run.id_ == test_run.id_

        @pytest.mark.asyncio
        async def test_get_without_params_raises_error(self, runs_api_async):
            """Test that getting a run without parameters raises an error."""
            with pytest.raises(ValueError, match="must be provided"):
                await runs_api_async.get()

        @pytest.mark.asyncio
        async def test_get_nonexistent_run_raises_error(self, runs_api_async):
            """Test that getting a non-existent run raises an error."""
            with pytest.raises(ValueError, match="not found"):
                await runs_api_async.get(client_key="nonexistent-run-name-12345")

    class TestFind:
        """Tests for the async find method."""

        @pytest.mark.asyncio
        async def test_find_run(self, runs_api_async, test_run):
            """Test finding a single run."""
            # Find the same run by name
            found_run = await runs_api_async.find(name=test_run.name)

            assert found_run is not None
            assert found_run.id_ == test_run.id_

        @pytest.mark.asyncio
        async def test_find_nonexistent_run(self, runs_api_async):
            """Test finding a non-existent run returns None."""
            found_run = await runs_api_async.find(name="nonexistent-run-name-12345")
            assert found_run is None

        @pytest.mark.asyncio
        async def test_find_multiple_raises_error(self, runs_api_async):
            """Test finding multiple runs raises an error."""
            with pytest.raises(ValueError, match="Multiple"):
                await runs_api_async.find(name_contains="a")


class TestRunsAPISync:
    """Test suite for the synchronous Runs API functionality.

    Only includes a single test for basic sync generation. No specific sync behavior difference tests are needed.
    """

    class TestList:
        """Tests for the sync list method."""

        def test_basic_list(self, runs_api_sync):
            """Test basic synchronous run listing functionality."""
            runs = runs_api_sync.list_(limit=5)

            # Verify we get a list
            assert isinstance(runs, list)
            assert runs
            assert isinstance(runs[0], Run)
