"""Pytest tests for the Jobs API.

These tests demonstrate and validate the usage of the Jobs API including:
- Basic job operations (get, list)
- Job filtering and searching
- Job cancellation and retry
- Error handling and edge cases
"""

from datetime import datetime, timedelta, timezone

import pytest
from grpc.aio import AioRpcError

from sift_client import SiftClient
from sift_client.resources import JobsAPI, JobsAPIAsync
from sift_client.sift_types import Job
from sift_client.sift_types.job import (
    DataExportDetails,
    DataImportDetails,
    DataImportStatusDetails,
    JobStatus,
    JobType,
    RuleEvaluationDetails,
)

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert sift_client.jobs
    assert isinstance(sift_client.jobs, JobsAPI)
    assert sift_client.async_.jobs
    assert isinstance(sift_client.async_.jobs, JobsAPIAsync)


@pytest.fixture
def jobs_api_async(sift_client: SiftClient):
    """Get the async jobs API instance."""
    return sift_client.async_.jobs


@pytest.fixture
def jobs_api_sync(sift_client: SiftClient):
    """Get the synchronous jobs API instance."""
    return sift_client.jobs


@pytest.fixture
def test_job(jobs_api_sync):
    """Get a test job to use in tests."""
    jobs = jobs_api_sync.list_(limit=1)
    assert jobs
    assert len(jobs) >= 1
    return jobs[0]


class TestJobsAPIAsync:
    """Test suite for the async Jobs API functionality."""

    class TestGet:
        """Tests for the async get method."""

        @pytest.mark.asyncio
        async def test_get_by_id(self, jobs_api_async, test_job):
            """Test getting a specific job by ID."""
            retrieved_job = await jobs_api_async.get(test_job.id_)

            assert isinstance(retrieved_job, Job)
            assert retrieved_job.id_ == test_job.id_
            assert retrieved_job.job_type is not None
            assert retrieved_job.job_status is not None

    class TestList:
        """Tests for the async list_ method."""

        @pytest.mark.asyncio
        async def test_basic_list(self, jobs_api_async):
            """Test basic job listing functionality."""
            jobs = await jobs_api_async.list_(limit=5)

            assert isinstance(jobs, list)
            assert len(jobs) <= 5

            if jobs:
                job = jobs[0]
                assert isinstance(job, Job)
                assert job.id_ is not None
                assert job.job_type is not None
                assert job.job_status is not None
                assert job.organization_id is not None
                assert job.created_by_user_id is not None
                assert job.modified_by_user_id is not None
                assert job.created_date is not None
                assert job.modified_date is not None

        @pytest.mark.asyncio
        async def test_list_with_job_ids_filter(self, jobs_api_async):
            """Test job listing with job IDs filter."""
            all_jobs = await jobs_api_async.list_(limit=3)

            if all_jobs:
                job_ids = [j.id_ for j in all_jobs]
                filtered_jobs = await jobs_api_async.list_(job_ids=job_ids)

                assert isinstance(filtered_jobs, list)
                assert len(filtered_jobs) >= len(all_jobs)

                for job in filtered_jobs:
                    assert job.id_ in job_ids

        @pytest.mark.asyncio
        async def test_list_with_job_type_filter(self, jobs_api_async):
            """Test job listing with job type filter."""
            # Test with DATA_IMPORT type
            jobs = await jobs_api_async.list_(job_type=JobType.DATA_IMPORT, limit=5)

            assert isinstance(jobs, list)

            for job in jobs:
                assert job.job_type == JobType.DATA_IMPORT

        @pytest.mark.asyncio
        async def test_list_with_job_status_filter(self, jobs_api_async):
            """Test job listing with job status filter."""
            # Test with FINISHED status
            jobs = await jobs_api_async.list_(job_status=JobStatus.FINISHED, limit=5)

            assert isinstance(jobs, list)

            for job in jobs:
                assert job.job_status == JobStatus.FINISHED

        @pytest.mark.asyncio
        async def test_list_with_created_date_filters(self, jobs_api_async):
            """Test job listing with created date filters."""
            one_year_ago = datetime.now(timezone.utc) - timedelta(days=365)
            jobs = await jobs_api_async.list_(created_after=one_year_ago, limit=5)

            assert isinstance(jobs, list)

            for job in jobs:
                assert job.created_date >= one_year_ago

        @pytest.mark.asyncio
        async def test_list_with_modified_date_filters(self, jobs_api_async):
            """Test job listing with modified date filters."""
            one_year_ago = datetime.now(timezone.utc) - timedelta(days=365)
            jobs = await jobs_api_async.list_(modified_after=one_year_ago, limit=5)

            assert isinstance(jobs, list)

            for job in jobs:
                assert job.modified_date >= one_year_ago

        @pytest.mark.asyncio
        async def test_list_with_started_date_filters(self, jobs_api_async):
            """Test job listing with started date filters."""
            one_year_ago = datetime.now(timezone.utc) - timedelta(days=365)
            jobs = await jobs_api_async.list_(started_date_after=one_year_ago, limit=5)

            assert isinstance(jobs, list)

            for job in jobs:
                if job.started_date:
                    assert job.started_date >= one_year_ago

        @pytest.mark.asyncio
        async def test_list_with_completed_date_filters(self, jobs_api_async):
            """Test job listing with completed date filters."""
            one_year_ago = datetime.now(timezone.utc) - timedelta(days=365)
            jobs = await jobs_api_async.list_(completed_date_after=one_year_ago, limit=5)

            assert isinstance(jobs, list)

            for job in jobs:
                if job.completed_date:
                    assert job.completed_date >= one_year_ago

        @pytest.mark.asyncio
        async def test_list_with_limit(self, jobs_api_async):
            """Test job listing with different limits."""
            jobs_1 = await jobs_api_async.list_(limit=1)
            assert isinstance(jobs_1, list)
            assert len(jobs_1) <= 1

            jobs_3 = await jobs_api_async.list_(limit=3)
            assert isinstance(jobs_3, list)
            assert len(jobs_3) <= 3

        @pytest.mark.asyncio
        async def test_list_with_multiple_filters(self, jobs_api_async):
            """Test job listing with multiple filters combined."""
            one_month_ago = datetime.now(timezone.utc) - timedelta(days=30)
            jobs = await jobs_api_async.list_(
                job_type=JobType.DATA_IMPORT,
                job_status=JobStatus.FINISHED,
                created_after=one_month_ago,
                limit=5,
            )

            assert isinstance(jobs, list)

            for job in jobs:
                assert job.job_type == JobType.DATA_IMPORT
                assert job.job_status == JobStatus.FINISHED
                assert job.created_date >= one_month_ago

    class TestCancel:
        """Tests for the async cancel method."""

        @pytest.mark.asyncio
        async def test_cancel_finished_job_no_effect(self, jobs_api_async):
            """Test that cancelling a finished job has no effect."""
            # Find a finished job
            finished_jobs = await jobs_api_async.list_(job_status=JobStatus.FINISHED, limit=1)

            if finished_jobs:
                job = finished_jobs[0]
                original_status = job.job_status

                # Cancel should not raise an error but won't change status
                await jobs_api_async.cancel(job)

                # Verify status hasn't changed
                updated_job = await jobs_api_async.get(job.id_)
                assert updated_job.job_status == original_status

        @pytest.mark.asyncio
        async def test_cancel_with_job_id_string(self, jobs_api_async):
            """Test cancelling a job by passing job ID as string."""
            # Find a finished job to safely test with
            finished_jobs = await jobs_api_async.list_(job_status=JobStatus.FINISHED, limit=1)

            if finished_jobs:
                job = finished_jobs[0]
                # Cancel using job ID string (should not raise error)
                await jobs_api_async.cancel(job.id_)

    class TestRetry:
        """Tests for the async retry method."""

        @pytest.mark.asyncio
        async def test_retry_failed_job(self, jobs_api_async):
            """Test retrying a failed job."""
            # Find a failed job
            failed_jobs = await jobs_api_async.list_(job_status=JobStatus.FAILED, limit=1)

            if failed_jobs:
                job = failed_jobs[0]
                original_id = job.id_

                # Retry the job
                retried_job = await jobs_api_async.retry(job)

                # Verify we got a job back
                assert isinstance(retried_job, Job)
                assert retried_job.id_ == original_id
                # Status might be CREATED or RUNNING after retry
                assert retried_job.job_status in [
                    JobStatus.CREATED,
                    JobStatus.RUNNING,
                    JobStatus.FAILED,  # May fail again immediately
                ]

        @pytest.mark.asyncio
        async def test_retry_with_job_id_string(self, jobs_api_async):
            """Test retrying a job by passing job ID as string."""
            # Find a failed job
            failed_jobs = await jobs_api_async.list_(job_status=JobStatus.FAILED, limit=1)

            if failed_jobs:
                job = failed_jobs[0]

                # Retry using job ID string
                retried_job = await jobs_api_async.retry(job.id_)

                assert isinstance(retried_job, Job)
                assert retried_job.id_ == job.id_

        @pytest.mark.asyncio
        async def test_retry_finished_job_no_effect(self, jobs_api_async):
            """Test that retrying a finished job has no effect."""
            # Find a finished job
            finished_jobs = await jobs_api_async.list_(job_status=JobStatus.FINISHED, limit=1)

            if finished_jobs:
                job = finished_jobs[0]

                # Retry should not raise an error but won't change status
                with pytest.raises(AioRpcError, match="job cannot be retried"):
                    await jobs_api_async.retry(job)

    class TestJobProperties:
        """Tests for job property methods."""

        @pytest.mark.asyncio
        async def test_job_status_properties(self, jobs_api_async):
            """Test job status property methods."""
            # Test with different job statuses
            finished_jobs = await jobs_api_async.list_(job_status=JobStatus.FINISHED, limit=1)
            if finished_jobs:
                job = finished_jobs[0]
                assert job.is_finished is True
                assert job.is_failed is False
                assert job.is_cancelled is False
                assert job.is_in_progress is False

            failed_jobs = await jobs_api_async.list_(job_status=JobStatus.FAILED, limit=1)
            if failed_jobs:
                job = failed_jobs[0]
                assert job.is_failed is True
                assert job.is_finished is False
                assert job.is_cancelled is False
                assert job.is_in_progress is False

            running_jobs = await jobs_api_async.list_(job_status=JobStatus.RUNNING, limit=1)
            if running_jobs:
                job = running_jobs[0]
                assert job.is_in_progress is True
                assert job.is_finished is False
                assert job.is_failed is False
                assert job.is_cancelled is False

            cancelled_jobs = await jobs_api_async.list_(job_status=JobStatus.CANCELLED, limit=1)
            if cancelled_jobs:
                job = cancelled_jobs[0]
                assert job.is_cancelled is True
                assert job.is_finished is False
                assert job.is_failed is False
                assert job.is_in_progress is False

        @pytest.mark.asyncio
        async def test_job_details_by_type(self, jobs_api_async):
            """Test that job details are populated correctly based on job type."""
            # Test DATA_IMPORT jobs
            import_jobs = await jobs_api_async.list_(job_type=JobType.DATA_IMPORT, limit=1)
            if import_jobs:
                job = import_jobs[0]
                if job.job_details:
                    assert isinstance(job.job_details, DataImportDetails)
                    assert job.job_details.data_import_id is not None

            # Test RULE_EVALUATION jobs
            rule_eval_jobs = await jobs_api_async.list_(job_type=JobType.RULE_EVALUATION, limit=1)
            if rule_eval_jobs:
                job = rule_eval_jobs[0]
                if job.job_details:
                    assert isinstance(job.job_details, RuleEvaluationDetails)
                    assert job.job_details.report_id is not None

            # Test DATA_EXPORT jobs
            export_jobs = await jobs_api_async.list_(job_type=JobType.DATA_EXPORT, limit=1)
            if export_jobs:
                job = export_jobs[0]
                if job.job_details:
                    assert isinstance(job.job_details, DataExportDetails)
                    assert job.job_details.storage_key is not None

        @pytest.mark.asyncio
        async def test_job_status_details_for_data_import(self, jobs_api_async):
            """Test that status details are populated for data import jobs."""
            import_jobs = await jobs_api_async.list_(
                job_type=JobType.DATA_IMPORT,
                job_status=JobStatus.FINISHED,
                limit=5,
            )

            if import_jobs:
                # Find a job with status details
                for job in import_jobs:
                    if job.job_status_details:
                        assert isinstance(job.job_status_details, DataImportStatusDetails)
                        assert job.job_status_details.points_processed is not None
                        assert job.job_status_details.points_total is not None
                        break

    class TestJobInstanceMethods:
        """Tests for job instance methods."""

        @pytest.mark.asyncio
        async def test_job_refresh(self, jobs_api_async, test_job):
            """Test refreshing a job to get latest data."""
            # Get the job
            job = await jobs_api_async.get(test_job.id_)

            # Refresh should work without error
            # Note: We can't easily test that data actually changes
            # but we can verify the method works
            original_modified = job.modified_date
            refreshed_job = job.refresh()

            assert isinstance(refreshed_job, Job)
            assert refreshed_job.id_ == job.id_
            # Modified date should be the same or newer
            assert refreshed_job.modified_date >= original_modified


class TestJobsAPISync:
    """Test suite for the synchronous Jobs API functionality.

    Only includes a single test for basic sync generation. No specific sync behavior difference tests are needed.
    """

    class TestGet:
        """Tests for the sync get method."""

        def test_get_by_id(self, jobs_api_sync, test_job):
            """Test getting a specific job by ID synchronously."""
            retrieved_job = jobs_api_sync.get(test_job.id_)

            assert isinstance(retrieved_job, Job)
            assert retrieved_job.id_ == test_job.id_

    class TestList:
        """Tests for the sync list method."""

        def test_basic_list(self, jobs_api_sync):
            """Test basic synchronous job listing functionality."""
            jobs = jobs_api_sync.list_(limit=5)

            assert isinstance(jobs, list)
            assert len(jobs) <= 5

            if jobs:
                assert isinstance(jobs[0], Job)
