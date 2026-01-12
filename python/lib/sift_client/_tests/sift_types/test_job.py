"""Tests for sift_types.Job model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import Job
from sift_client.sift_types.job import (
    DataExportDetails,
    DataExportStatusDetails,
    DataImportDetails,
    DataImportStatusDetails,
    JobStatus,
    JobType,
    RuleEvaluationDetails,
    RuleEvaluationStatusDetails,
)


@pytest.fixture
def mock_job(mock_client):
    """Create a mock Job instance for testing."""
    job = Job(
        proto=MagicMock(),
        id_="test_job_id",
        organization_id="org1",
        created_by_user_id="user1",
        modified_by_user_id="user1",
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        started_date=datetime.now(timezone.utc),
        completed_date=None,
        job_type=JobType.DATA_IMPORT,
        job_status=JobStatus.RUNNING,
        job_status_details=DataImportStatusDetails(points_processed=100, points_total=1000),
        job_details=DataImportDetails(data_import_id="import123"),
    )
    job._apply_client_to_instance(mock_client)
    # Configure mock to return the job itself when refresh is called
    mock_client.jobs.get.return_value = job
    return job


@pytest.fixture
def mock_finished_job(mock_client):
    """Create a mock finished Job instance for testing."""
    job = Job(
        proto=MagicMock(),
        id_="test_finished_job_id",
        organization_id="org1",
        created_by_user_id="user1",
        modified_by_user_id="user1",
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        started_date=datetime.now(timezone.utc),
        completed_date=datetime.now(timezone.utc),
        job_type=JobType.DATA_IMPORT,
        job_status=JobStatus.FINISHED,
        job_status_details=DataImportStatusDetails(points_processed=1000, points_total=1000),
        job_details=DataImportDetails(data_import_id="import123"),
    )
    job._apply_client_to_instance(mock_client)
    # Configure mock to return the job itself when refresh is called
    mock_client.jobs.get.return_value = job
    return job


@pytest.fixture
def mock_failed_job(mock_client):
    """Create a mock failed Job instance for testing."""
    job = Job(
        proto=MagicMock(),
        id_="test_failed_job_id",
        organization_id="org1",
        created_by_user_id="user1",
        modified_by_user_id="user1",
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        started_date=datetime.now(timezone.utc),
        completed_date=datetime.now(timezone.utc),
        job_type=JobType.DATA_EXPORT,
        job_status=JobStatus.FAILED,
        job_status_details=DataExportStatusDetails(error_message="Export failed"),
        job_details=DataExportDetails(storage_key="exports/failed.csv"),
    )
    job._apply_client_to_instance(mock_client)
    # Configure mock to return the job itself when refresh is called
    mock_client.jobs.get.return_value = job
    return job


@pytest.fixture
def mock_cancelled_job(mock_client):
    """Create a mock cancelled Job instance for testing."""
    job = Job(
        proto=MagicMock(),
        id_="test_cancelled_job_id",
        organization_id="org1",
        created_by_user_id="user1",
        modified_by_user_id="user1",
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        started_date=None,
        completed_date=None,
        job_type=JobType.RULE_EVALUATION,
        job_status=JobStatus.CANCELLED,
        job_status_details=None,
        job_details=RuleEvaluationDetails(report_id="report123"),
    )
    job._apply_client_to_instance(mock_client)
    # Configure mock to return the job itself when refresh is called
    mock_client.jobs.get.return_value = job
    return job


class TestJobStatusProperties:
    """Unit tests for Job status property methods."""

    def test_is_in_progress_true_for_running_job(self, mock_job):
        """Test that is_in_progress returns True for RUNNING status."""
        assert mock_job.job_status == JobStatus.RUNNING
        assert mock_job.is_in_progress is True
        assert mock_job.is_finished is False
        assert mock_job.is_failed is False
        assert mock_job.is_cancelled is False

    def test_is_finished_true_for_finished_job(self, mock_finished_job):
        """Test that is_finished returns True for FINISHED status."""
        assert mock_finished_job.job_status == JobStatus.FINISHED
        assert mock_finished_job.is_finished is True
        assert mock_finished_job.is_in_progress is False
        assert mock_finished_job.is_failed is False
        assert mock_finished_job.is_cancelled is False

    def test_is_failed_true_for_failed_job(self, mock_failed_job):
        """Test that is_failed returns True for FAILED status."""
        assert mock_failed_job.job_status == JobStatus.FAILED
        assert mock_failed_job.is_failed is True
        assert mock_failed_job.is_in_progress is False
        assert mock_failed_job.is_finished is False
        assert mock_failed_job.is_cancelled is False

    def test_is_cancelled_true_for_cancelled_job(self, mock_cancelled_job):
        """Test that is_cancelled returns True for CANCELLED status."""
        assert mock_cancelled_job.job_status == JobStatus.CANCELLED
        assert mock_cancelled_job.is_cancelled is True
        assert mock_cancelled_job.is_in_progress is False
        assert mock_cancelled_job.is_finished is False
        assert mock_cancelled_job.is_failed is False


class TestJobInstanceMethods:
    """Unit tests for Job instance methods."""

    def test_refresh_calls_client_and_updates_self(self, mock_job, mock_client):
        """Test that refresh() calls client.jobs.get and calls _update."""
        refreshed_job = MagicMock()
        refreshed_job.job_status = JobStatus.FINISHED
        refreshed_job.completed_date = datetime.now(timezone.utc)
        mock_client.jobs.get.return_value = refreshed_job

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_job._update = mock_update

            # Call refresh
            result = mock_job.refresh()

            # Verify client method was called with correct job ID
            mock_client.jobs.get.assert_called_once_with(mock_job._id_or_error)
            # Verify _update was called with the returned job
            mock_update.assert_called_once_with(refreshed_job)
            # Verify it returns self
            assert result is mock_job

    def test_cancel_calls_client_and_refreshes(self, mock_job, mock_client):
        """Test that cancel() calls client.jobs.cancel and refreshes."""
        refreshed_job = MagicMock()
        refreshed_job.job_status = JobStatus.CANCELLED
        mock_client.jobs.get.return_value = refreshed_job

        # Mock the _update method
        with MagicMock() as mock_update:
            mock_job._update = mock_update

            # Call cancel
            mock_job.cancel()

            # Verify client.jobs.cancel was called with self
            mock_client.jobs.cancel.assert_called_once_with(mock_job)
            # Verify refresh was called (which calls get and _update)
            mock_client.jobs.get.assert_called_once_with(mock_job._id_or_error)
            mock_update.assert_called_once_with(refreshed_job)

    def test_retry_calls_client_and_updates_self(self, mock_failed_job, mock_client):
        """Test that retry() calls client.jobs.retry and calls _update."""
        retried_job = MagicMock()
        retried_job.job_status = JobStatus.CREATED
        mock_client.jobs.retry.return_value = retried_job

        # Mock the _update method to verify it's called
        with MagicMock() as mock_update:
            mock_failed_job._update = mock_update

            # Call retry
            result = mock_failed_job.retry()

            # Verify client method was called with self
            mock_client.jobs.retry.assert_called_once_with(mock_failed_job)
            # Verify _update was called with the returned job
            mock_update.assert_called_once_with(retried_job)
            # Verify it returns self
            assert result is mock_failed_job


class TestJobType:
    """Unit tests for JobType enum."""

    def test_to_filter_str(self):
        """Test that to_filter_str returns correct format."""
        assert JobType.DATA_IMPORT.to_filter_str() == "JOB_TYPE_DATA_IMPORT"
        assert JobType.DATA_EXPORT.to_filter_str() == "JOB_TYPE_DATA_EXPORT"
        assert JobType.RULE_EVALUATION.to_filter_str() == "JOB_TYPE_RULE_EVALUATION"


class TestJobStatus:
    """Unit tests for JobStatus enum."""

    def test_to_filter_str(self):
        """Test that to_filter_str returns correct format."""
        assert JobStatus.CREATED.to_filter_str() == "JOB_STATUS_CREATED"
        assert JobStatus.RUNNING.to_filter_str() == "JOB_STATUS_RUNNING"
        assert JobStatus.FINISHED.to_filter_str() == "JOB_STATUS_FINISHED"
        assert JobStatus.FAILED.to_filter_str() == "JOB_STATUS_FAILED"
        assert JobStatus.CANCELLED.to_filter_str() == "JOB_STATUS_CANCELLED"
        assert JobStatus.CANCEL_REQUESTED.to_filter_str() == "JOB_STATUS_CANCEL_REQUESTED"


class TestJobStatusDetails:
    """Unit tests for JobStatusDetails types."""

    def test_data_import_status_details(self):
        """Test DataImportStatusDetails for data import jobs."""
        details = DataImportStatusDetails(points_processed=500, points_total=1000)
        assert details.points_processed == 500
        assert details.points_total == 1000

    def test_data_export_status_details(self):
        """Test DataExportStatusDetails for data export jobs."""
        details = DataExportStatusDetails(error_message="Export failed due to timeout")
        assert details.error_message == "Export failed due to timeout"

    def test_data_export_status_details_empty_error(self):
        """Test DataExportStatusDetails with no error message."""
        details = DataExportStatusDetails()
        assert details.error_message is None

    def test_rule_evaluation_status_details(self):
        """Test RuleEvaluationStatusDetails for rule evaluation jobs."""
        details = RuleEvaluationStatusDetails()
        assert details is not None


class TestJobDetails:
    """Unit tests for JobDetails types."""

    def test_rule_evaluation_details(self):
        """Test RuleEvaluationDetails for rule evaluation jobs."""
        details = RuleEvaluationDetails(report_id="report123")
        assert details.report_id == "report123"

    def test_data_import_details(self):
        """Test DataImportDetails for data import jobs."""
        details = DataImportDetails(data_import_id="import456")
        assert details.data_import_id == "import456"

    def test_data_export_details(self):
        """Test DataExportDetails for data export jobs."""
        details = DataExportDetails(storage_key="exports/data.csv")
        assert details.storage_key == "exports/data.csv"
