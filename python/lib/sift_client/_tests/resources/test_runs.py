"""Pytest tests for the Runs API.

These tests demonstrate and validate the usage of the Runs API including:
- Basic run operations (get, list, find)
- Run filtering and searching
- Run creation, updates, and archiving
- Error handling and edge cases
"""

from datetime import datetime, timedelta, timezone

import pytest
from grpc.aio import AioRpcError

from sift_client import SiftClient
from sift_client.resources import RunsAPI, RunsAPIAsync
from sift_client.sift_types import Run
from sift_client.sift_types.run import RunCreate, RunUpdate

pytestmark = pytest.mark.integration


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


@pytest.fixture
def new_run(runs_api_sync, ci_pytest_tag):
    """Create a test run for update tests."""
    run_name = f"test_run_update_{datetime.now(timezone.utc).isoformat()}"
    description = "Test run created by Sift Client pytest"
    created_run = runs_api_sync.create(
        RunCreate(
            name=run_name,
            description=description,
            tags=[ci_pytest_tag.name],
        )
    )
    return created_run


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

        # TODO: test run-specific filters

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
        # @pytest.mark.asyncio
        # async def test_get_by_id_with_client_key(self, runs_api_async, test_run):
        #     """Test getting a specific run by client key."""
        #     assert test_run.client_key is not None
        #     retrieved_run = await runs_api_async.get(client_key=test_run.client_key)
        #
        #     assert retrieved_run is not None
        #     assert retrieved_run.id_ == test_run.id_

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
            found_run = await runs_api_async.find(
                name=test_run.name,
                created_after=test_run.created_date - timedelta(seconds=10),
                created_before=test_run.created_date + timedelta(seconds=10),
            )

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

    class TestCreate:
        """Tests for the async create method."""

        @pytest.mark.asyncio
        async def test_create_basic_run(self, runs_api_async):
            """Test creating a basic run with minimal fields."""
            run_name = f"test_run_create_{datetime.now(timezone.utc).isoformat()}"
            description = "Test run created by Sift Client pytest"
            run_create = RunCreate(
                name=run_name,
                description=description,
                tags=["sift-client-pytest"],
            )

            created_run = await runs_api_async.create(run_create)

            try:
                # Verify the run was created
                assert created_run is not None
                assert isinstance(created_run, Run)
                assert created_run.id_ is not None
                assert created_run.name == run_name
                assert created_run.description == description
                assert created_run.created_date is not None
                assert created_run.modified_date is not None
            finally:
                # Clean up: archive the test run
                await runs_api_async.archive(created_run)

        @pytest.mark.asyncio
        async def test_create_run_with_all_fields(self, runs_api_async):
            """Test creating a run with all optional fields."""
            run_name = f"test_run_full_{datetime.now(timezone.utc).isoformat()}"
            description = "Test run created by Sift Client pytest"
            start_time = datetime.now(timezone.utc) - timedelta(hours=1)
            stop_time = datetime.now(timezone.utc)

            run_create = RunCreate(
                name=run_name,
                description=description,
                client_key=f"client_key_{datetime.now(timezone.utc).timestamp()}",
                start_time=start_time,
                stop_time=stop_time,
                tags=["test", "pytest", "integration", "sift-client-pytest"],
                metadata={"pytest_type": "integration"},
            )

            created_run = await runs_api_async.create(run_create)

            try:
                # Verify all fields
                assert created_run.name == run_name
                assert created_run.description == description
                assert created_run.client_key is not None
                assert created_run.start_time is not None
                assert created_run.stop_time is not None
                assert created_run.tags == [
                    "test",
                    "pytest",
                    "integration",
                    "sift-client-pytest",
                ]
                assert created_run.metadata["pytest_type"] == "integration"

            finally:
                # Clean up
                await runs_api_async.archive(created_run)

        @pytest.mark.asyncio
        async def test_create_run_with_dict(self, runs_api_async):
            """Test creating a run using a dictionary instead of RunCreate object."""
            run_name = f"test_run_dict_{datetime.now(timezone.utc).isoformat()}"
            description = "Test run created by Sift Client pytest"

            run_dict = {
                "name": run_name,
                "description": description,
                "tags": ["sift-client-pytest"],
            }

            created_run = await runs_api_async.create(run_dict)

            try:
                assert created_run.name == run_name
                assert created_run.description == description
                assert created_run.tags == ["sift-client-pytest"]
            finally:
                await runs_api_async.archive(created_run)

    class TestUpdate:
        """Tests for the async update method."""

        @pytest.mark.asyncio
        async def test_update_run_description(self, runs_api_async, new_run):
            """Test updating a run's description."""
            try:
                # Update the description
                update = RunUpdate(description="Updated description")
                updated_run = await runs_api_async.update(new_run, update)

                # Verify the update
                assert updated_run.id_ == new_run.id_
                assert updated_run.description == "Updated description"
                assert updated_run.name == new_run.name  # Name should remain unchanged
            finally:
                await runs_api_async.archive(new_run.id_)

        @pytest.mark.asyncio
        async def test_update_run_name(self, runs_api_async, new_run):
            """Test updating a run's name."""
            try:
                # Update the name
                new_name = f"updated_{new_run.name}"
                update = RunUpdate(name=new_name)
                updated_run = await runs_api_async.update(new_run, update)

                # Verify the update
                assert updated_run.name == new_name
                assert updated_run.id_ == new_run.id_
            finally:
                await runs_api_async.archive(new_run.id_)

        @pytest.mark.asyncio
        async def test_update_run_tags_and_metadata(self, runs_api_async, new_run):
            """Test updating a run's tags and metadata."""
            try:
                # Update tags and metadata
                update = RunUpdate(
                    tags=["updated", "new-tag", "sift-client-pytest"],
                    metadata={"test_key": "test_value", "number": 42.5, "flag": True},
                )
                updated_run = await runs_api_async.update(new_run, update)

                # Verify the updates
                assert set(updated_run.tags) == {
                    "updated",
                    "new-tag",
                    "sift-client-pytest",
                }
                assert updated_run.metadata["test_key"] == "test_value"
                assert updated_run.metadata["number"] == 42.5
                assert updated_run.metadata["flag"] is True
            finally:
                await runs_api_async.archive(new_run.id_)

        @pytest.mark.asyncio
        async def test_update_run_times(self, runs_api_async, new_run):
            """Test updating a run's start and stop times."""
            try:
                # Update with start and stop times
                start_time = datetime.now(timezone.utc) - timedelta(hours=2)
                stop_time = datetime.now(timezone.utc) - timedelta(hours=1)
                update = RunUpdate(start_time=start_time, stop_time=stop_time)
                updated_run = await runs_api_async.update(new_run, update)

                # Verify the times were set
                assert updated_run.start_time is not None
                assert updated_run.stop_time is not None
                # Allow for small time differences due to serialization
                assert abs((updated_run.start_time - start_time).total_seconds()) < 1
                assert abs((updated_run.stop_time - stop_time).total_seconds()) < 1
            finally:
                await runs_api_async.archive(new_run.id_)

        @pytest.mark.asyncio
        async def test_update_with_dict(self, runs_api_async, new_run):
            """Test updating a run using a dictionary instead of RunUpdate object."""
            try:
                # Update using dict
                update_dict = {"description": "Updated via dict"}
                updated_run = await runs_api_async.update(new_run, update_dict)

                assert updated_run.description == "Updated via dict"
            finally:
                await runs_api_async.archive(new_run.id_)

        @pytest.mark.asyncio
        async def test_update_with_run_id_string(self, runs_api_async, new_run):
            """Test updating a run by passing run ID as string."""
            try:
                # Update using run ID string
                update = RunUpdate(description="Updated via ID string")
                updated_run = await runs_api_async.update(new_run.id_, update)

                assert updated_run.id_ == new_run.id_
                assert updated_run.description == "Updated via ID string"
            finally:
                await runs_api_async.archive(new_run.id_)

    class TestArchive:
        """Tests for the async archive method."""

        @pytest.mark.asyncio
        async def test_archive_run(self, runs_api_async, new_run):
            """Test archiving a run."""
            run = await runs_api_async.archive(new_run)

            assert isinstance(run, Run)
            assert run.id_ == new_run.id_
            assert run.is_archived is True

            # Verify it's archived by checking it doesn't appear in normal list
            runs_without_archived = await runs_api_async.list_(
                name=new_run.name, include_archived=False
            )
            assert len(runs_without_archived) == 0

            # Verify it appears when including archived
            runs_with_archived = await runs_api_async.list_(
                name=new_run.name, include_archived=True
            )
            assert len(runs_with_archived) == 1
            assert runs_with_archived[0].id_ == new_run.id_
            assert runs_with_archived[0].archived_date is not None

        @pytest.mark.asyncio
        async def test_archive_with_run_id_string(self, runs_api_async, new_run):
            """Test archiving a run by passing run ID as string."""
            # Archive using run ID string
            run = await runs_api_async.archive(new_run.id_)

            assert isinstance(run, Run)
            assert run.id_ == new_run.id_
            assert run.is_archived is True

        @pytest.mark.asyncio
        async def test_get_archived_run_by_id(self, runs_api_async, new_run):
            """Test that we can still get an archived run by ID."""
            # Archive the test run
            run = await runs_api_async.archive(new_run)

            assert isinstance(run, Run)
            assert run.id_ == new_run.id_
            assert run.is_archived is True

    class TestStop:
        """Tests for the async stop method."""

        @pytest.mark.asyncio
        async def test_stop_run(self, runs_api_async, new_run):
            """Test stopping a run."""
            try:
                # Stop the run
                stopped_run = await runs_api_async.stop(new_run)

                # Verify the run was stopped
                assert isinstance(stopped_run, Run)
                assert stopped_run.id_ == new_run.id_
                assert stopped_run.stop_time is not None
            finally:
                await runs_api_async.archive(new_run.id_)

        @pytest.mark.asyncio
        async def test_stop_run_with_id_string(self, runs_api_async, new_run):
            """Test stopping a run by passing run ID as string."""
            try:
                # Stop using run ID string
                stopped_run = await runs_api_async.stop(new_run.id_)

                # Verify the run was stopped
                assert isinstance(stopped_run, Run)
                assert stopped_run.id_ == new_run.id_
                assert stopped_run.stop_time is not None
            finally:
                await runs_api_async.archive(new_run.id_)

        @pytest.mark.asyncio
        async def test_stop_run_with_start_time(self, runs_api_async, new_run):
            """Test stopping a run that has a start time."""
            try:
                # Set start time first
                start_time = datetime.now(timezone.utc) - timedelta(hours=1)
                update = RunUpdate(start_time=start_time)
                await runs_api_async.update(new_run, update)

                # Stop the run
                stopped_run = await runs_api_async.stop(new_run)

                # Verify the run was stopped and times are valid
                assert stopped_run.stop_time is not None
                assert stopped_run.start_time is not None
                assert stopped_run.stop_time > stopped_run.start_time
            finally:
                await runs_api_async.archive(new_run.id_)

    class TestAssetAssociation:
        """Tests for the async asset association methods."""

        @pytest.mark.asyncio
        async def test_create_adhoc_run_all(
            self, runs_api_async, sift_client, test_tag, ci_pytest_tag
        ):
            """Test creating an adhoc run with associated assets."""
            run_name = f"test_adhoc_run_assets_{datetime.now(timezone.utc).isoformat()}"

            start_time = datetime.now(timezone.utc) - timedelta(hours=2)
            stop_time = datetime.now(timezone.utc) - timedelta(hours=1)
            # Get some assets to associate
            assets = await sift_client.async_.assets.list_(limit=2)
            assert len(assets) == 2
            tags = [test_tag, ci_pytest_tag]

            run_create = RunCreate(
                name=run_name,
                description="Test adhoc run",
                start_time=start_time,
                stop_time=stop_time,
                tags=tags,
                metadata={"test_key": "test_value", "number": 42.5, "flag": True},
            )
            created_run = await runs_api_async.create(
                run_create, assets=assets, associate_new_data=False
            )

            try:
                assert created_run.name == run_name
                assert created_run.is_adhoc is True
                assert created_run.asset_ids is not None
                assert len(created_run.asset_ids) >= len(assets)
                # Verify all requested assets are in the run's asset_ids
                for asset in assets:
                    assert asset.id_ in created_run.asset_ids
                assert created_run.metadata is not None
                assert created_run.metadata["test_key"] == "test_value"
                assert created_run.metadata["number"] == 42.5
                assert created_run.metadata["flag"] is True
                assert set(created_run.tags) == {tag.name for tag in tags}
            finally:
                await runs_api_async.archive(created_run)

        @pytest.mark.asyncio
        async def test_create_adhoc_run_missing_assets(self, runs_api_async):
            """Test creating an adhoc run with missing assets."""
            run_name = f"test_adhoc_run_missing_assets_{datetime.now(timezone.utc).isoformat()}"
            run_create = RunCreate(
                name=run_name,
                start_time=datetime.now(timezone.utc),
                stop_time=datetime.now(timezone.utc) + timedelta(seconds=11),
            )
            with pytest.raises(
                AioRpcError,
                match='invalid argument: invalid input syntax for type uuid: "asset-name-not-id"',
            ):
                await runs_api_async.create(
                    run_create, assets=["asset-name-not-id"], associate_new_data=False
                )


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
