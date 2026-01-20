"""Job types for the Sift API."""

from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, Union

from pydantic import BaseModel
from sift.jobs.v1.jobs_pb2 import Job as JobProto
from sift.jobs.v1.jobs_pb2 import JobDetails as JobDetailsProto
from sift.jobs.v1.jobs_pb2 import JobStatus as JobStatusProto
from sift.jobs.v1.jobs_pb2 import JobStatusDetails as JobStatusDetailsProto
from sift.jobs.v1.jobs_pb2 import JobType as JobTypeProto

from sift_client.sift_types._base import BaseType

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class JobType(str, Enum):
    """Type of job."""

    RULE_EVALUATION = "RULE_EVALUATION"
    DATA_IMPORT = "DATA_IMPORT"
    DATA_EXPORT = "DATA_EXPORT"

    def to_filter_str(self) -> str:
        """Convert to string representation."""
        return f"JOB_TYPE_{self.value}"

    def to_proto(self) -> int:
        """Convert to proto enum value."""
        mapping = {
            JobType.RULE_EVALUATION: JobTypeProto.JOB_TYPE_RULE_EVALUATION,
            JobType.DATA_IMPORT: JobTypeProto.JOB_TYPE_DATA_IMPORT,
            JobType.DATA_EXPORT: JobTypeProto.JOB_TYPE_DATA_EXPORT,
        }
        return mapping[self]

    @classmethod
    def from_proto(cls, proto_value: int) -> JobType:
        """Create from proto enum value."""
        mapping: dict[int, JobType] = {
            JobTypeProto.JOB_TYPE_RULE_EVALUATION: JobType.RULE_EVALUATION,
            JobTypeProto.JOB_TYPE_DATA_IMPORT: JobType.DATA_IMPORT,
            JobTypeProto.JOB_TYPE_DATA_EXPORT: JobType.DATA_EXPORT,
        }
        if proto_value not in mapping:
            raise ValueError(f"Unknown JobType proto value: {proto_value}")
        return mapping[proto_value]


class JobStatus(str, Enum):
    """Status of a job."""

    CREATED = "CREATED"
    RUNNING = "RUNNING"
    FINISHED = "FINISHED"
    FAILED = "FAILED"
    CANCELLED = "CANCELLED"
    CANCEL_REQUESTED = "CANCEL_REQUESTED"

    def to_filter_str(self) -> str:
        """Convert to string representation."""
        return f"JOB_STATUS_{self.value}"

    @classmethod
    def from_proto(cls, proto_value: int) -> JobStatus:
        """Create from proto enum value."""
        mapping: dict[int, JobStatus] = {
            JobStatusProto.JOB_STATUS_CREATED: JobStatus.CREATED,
            JobStatusProto.JOB_STATUS_RUNNING: JobStatus.RUNNING,
            JobStatusProto.JOB_STATUS_FINISHED: JobStatus.FINISHED,
            JobStatusProto.JOB_STATUS_FAILED: JobStatus.FAILED,
            JobStatusProto.JOB_STATUS_CANCELLED: JobStatus.CANCELLED,
            JobStatusProto.JOB_STATUS_CANCEL_REQUESTED: JobStatus.CANCEL_REQUESTED,
        }
        if proto_value not in mapping:
            raise ValueError(f"Unknown JobStatus proto value: {proto_value}")
        return mapping[proto_value]


class DataImportStatusDetails(BaseModel):
    """Status details for a data import job."""

    points_processed: int
    points_total: int


class DataExportStatusDetails(BaseModel):
    """Status details for a data export job."""

    error_message: str | None = None


class RuleEvaluationStatusDetails(BaseModel):
    """Status details for a rule evaluation job."""

    pass


def _job_status_details_from_proto(
    proto: JobStatusDetailsProto,
) -> JobStatusDetails | None:
    """Create JobStatusDetails from proto."""
    if not proto.HasField("status"):
        return None

    status_field = proto.WhichOneof("status")
    if status_field == "data_import":
        return DataImportStatusDetails(
            points_processed=proto.data_import.points_processed,
            points_total=proto.data_import.points_total,
        )
    elif status_field == "data_export":
        return DataExportStatusDetails(error_message=proto.data_export.error_message or None)
    elif status_field == "rule_evaluation":
        return RuleEvaluationStatusDetails()
    return None


class DataImportDetails(BaseModel):
    """Details for a data import job."""

    data_import_id: str


class DataExportDetails(BaseModel):
    """Details for a data export job."""

    storage_key: str


class RuleEvaluationDetails(BaseModel):
    """Details for a rule evaluation job."""

    report_id: str


# Note: Using Union instead of | syntax for Python 3.9 compatibility at module level.
# While `from __future__ import annotations` allows | in type hints (they're strings),
# module-level type aliases are evaluated at runtime and require Union in Python <3.10.
JobStatusDetails = Union[
    DataImportStatusDetails, DataExportStatusDetails, RuleEvaluationStatusDetails
]
JobDetails = Union[DataImportDetails, DataExportDetails, RuleEvaluationDetails]


def _job_details_from_proto(proto: JobDetailsProto) -> JobDetails | None:
    """Create JobDetails from proto."""
    if not proto.HasField("details"):
        return None

    details_field = proto.WhichOneof("details")
    if details_field == "rule_evaluation":
        return RuleEvaluationDetails(report_id=proto.rule_evaluation.report_id)
    elif details_field == "data_import":
        return DataImportDetails(data_import_id=proto.data_import.data_import_id)
    elif details_field == "data_export":
        return DataExportDetails(storage_key=proto.data_export.storage_key)
    return None


class Job(BaseType[JobProto, "Job"]):
    """A job in the Sift system.

    Jobs represent long-running operations like data imports, rule evaluations, and data exports.
    """

    # Required fields
    organization_id: str
    created_by_user_id: str
    modified_by_user_id: str
    created_date: datetime
    modified_date: datetime
    job_type: JobType
    job_status: JobStatus

    # Optional fields
    started_date: datetime | None
    completed_date: datetime | None
    job_status_details: JobStatusDetails | None
    job_details: JobDetails | None

    @classmethod
    def _from_proto(cls, proto: JobProto, sift_client: SiftClient | None = None) -> Job:
        """Create from proto."""
        return cls(
            proto=proto,
            id_=proto.job_id,
            organization_id=proto.organization_id,
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            started_date=(
                proto.started_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("started_date")
                else None
            ),
            completed_date=(
                proto.completed_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("completed_date")
                else None
            ),
            job_type=JobType.from_proto(proto.job_type),
            job_status=JobStatus.from_proto(proto.job_status),
            job_status_details=(
                _job_status_details_from_proto(proto.job_status_details)
                if proto.HasField("job_status_details")
                else None
            ),
            job_details=(
                _job_details_from_proto(proto.job_details)
                if proto.HasField("job_details")
                else None
            ),
            _client=sift_client,
        )

    @property
    def is_in_progress(self) -> bool:
        """Return True if the job is in progress, False otherwise.

        A job is in progress if its status is RUNNING.
        """
        self.refresh()
        return self.job_status == JobStatus.RUNNING

    @property
    def is_failed(self) -> bool:
        """Return True if the job has failed, False otherwise.

        A job has failed if its status is FAILED.
        """
        self.refresh()
        return self.job_status == JobStatus.FAILED

    @property
    def is_finished(self) -> bool:
        """Return True if the job has finished, False otherwise.

        A job has finished if its status is FINISHED.
        """
        self.refresh()
        return self.job_status == JobStatus.FINISHED

    @property
    def is_cancelled(self) -> bool:
        """Return True if the job has been cancelled, False otherwise.

        A job has been cancelled if its status is CANCELLED.
        """
        self.refresh()
        return self.job_status == JobStatus.CANCELLED

    def refresh(self) -> Job:
        """Refresh this job with the latest data from the API.

        Returns:
            The updated Job object.
        """
        updated_job = self.client.jobs.get(self._id_or_error)
        self._update(updated_job)
        return self

    def cancel(self) -> None:
        """Cancel this job.

        If the job hasn't started yet, it will be cancelled immediately.
        Jobs that are already finished, failed, or cancelled are not affected.
        """
        self.client.jobs.cancel(self)
        self.refresh()

    def retry(self) -> Job:
        """Retry this job.

        Jobs that are finished, in progress, or in the process of being cancelled are not affected.

        Returns:
            The updated Job object.
        """
        updated_job = self.client.jobs.retry(self)
        self._update(updated_job)
        return self
