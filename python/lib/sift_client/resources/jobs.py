from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.jobs import JobsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.job import Job, JobStatus, JobType
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.client import SiftClient


class JobsAPIAsync(ResourceBase):
    """High-level API for interacting with jobs.

    This class provides a Pythonic interface for managing jobs in Sift.
    Jobs represent long-running operations like data imports, rule evaluations, and data exports.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the JobsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = JobsLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(self, job_id: str) -> Job:
        """Get a job by ID.

        Args:
            job_id: The ID of the job to retrieve.

        Returns:
            The Job object.
        """
        job = await self._low_level_client.get_job(job_id)
        return self._apply_client_to_instance(job)

    async def list_(
        self,
        *,
        # Self ids
        job_ids: list[str] | None = None,
        # Created/modified ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        # Created/modified users
        created_by_user_id: str | None = None,
        modified_by_user_id: str | None = None,
        # Resource-specific filters
        job_type: JobType | None = None,
        job_status: JobStatus | None = None,
        started_date_after: datetime | None = None,
        started_date_before: datetime | None = None,
        completed_date_after: datetime | None = None,
        completed_date_before: datetime | None = None,
        # Common filters
        organization_id: str | None = None,
        filter_query: str | None = None,
        # Ordering and pagination
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Job]:
        """List jobs with optional filtering.

        Args:
            job_ids: Filter to jobs with any of these IDs.
            created_after: Filter to jobs created after this datetime.
            created_before: Filter to jobs created before this datetime.
            modified_after: Filter to jobs modified after this datetime.
            modified_before: Filter to jobs modified before this datetime.
            created_by_user_id: Filter to jobs created by this user ID.
            modified_by_user_id: Filter to jobs last modified by this user ID.
            job_type: Filter to jobs with this type.
            job_status: Filter to jobs with this status.
            started_date_after: Filter to jobs started after this datetime.
            started_date_before: Filter to jobs started before this datetime.
            completed_date_after: Filter to jobs completed after this datetime.
            completed_date_before: Filter to jobs completed before this datetime.
            organization_id: Organization ID. Required if your user belongs to multiple organizations.
            filter_query: Explicit CEL query to filter jobs. If provided, other filter arguments are ignored.
            order_by: Field and direction to order results by.
            limit: Maximum number of jobs to return. If None, returns all matches.

        Returns:
            A list of Job objects that match the filter criteria.
        """
        filter_parts = [
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by_user_id,
                modified_by=modified_by_user_id,
            ),
            *self._build_common_cel_filters(filter_query=filter_query),
        ]
        if job_ids:
            filter_parts.append(cel.in_("job_id", job_ids))
        if job_status:
            filter_parts.append(cel.equals("job_status", job_status.to_filter_str()))
        if job_type:
            filter_parts.append(cel.equals("job_type", job_type.to_filter_str()))
        if started_date_after:
            filter_parts.append(cel.greater_than("started_date", started_date_after))
        if started_date_before:
            filter_parts.append(cel.less_than("started_date", started_date_before))
        if completed_date_after:
            filter_parts.append(cel.greater_than("completed_date", completed_date_after))
        if completed_date_before:
            filter_parts.append(cel.less_than("completed_date", completed_date_before))

        query_filter = cel.and_(*filter_parts)

        jobs = await self._low_level_client.list_all_jobs(
            query_filter=query_filter or None,
            organization_id=organization_id,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(jobs)

    async def cancel(self, job: Job | str) -> None:
        """Cancel a job.

        If the job hasn't started yet, it will be cancelled immediately.
        Jobs that are already finished, failed, or cancelled are not affected.

        Args:
            job: The Job or ID of the job to cancel.

        """
        job_id = job._id_or_error if isinstance(job, Job) else job
        await self._low_level_client.cancel_job(job_id)

    async def retry(self, job: Job | str) -> Job:
        """Retry a failed job.

        Jobs that are finished, in progress, or in the process of being cancelled are not affected.

        Args:
            job: The Job or ID of the job to retry.

        Returns:
            The updated Job object.
        """
        job_id = job._id_or_error if isinstance(job, Job) else job
        updated_job = await self._low_level_client.retry_job(job_id)
        return self._apply_client_to_instance(updated_job)
