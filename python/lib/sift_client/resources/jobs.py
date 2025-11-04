from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.jobs import JobsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.job import Job

if TYPE_CHECKING:
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
        filter_query: str | None = None,
        organization_id: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Job]:
        """List jobs.

        Args:
            filter_query: CEL filter query (e.g., 'job_status == "FINISHED"').
            organization_id: Organization ID (required if user belongs to multiple orgs).
            order_by: How to order results (e.g., 'created_date desc').
            limit: Maximum number of results to return.

        Returns:
            A list of Job objects.

        """
        jobs = await self._low_level_client.list_all_jobs(
            query_filter=filter_query,
            organization_id=organization_id,
            order_by=order_by,
            max_results=limit,
        )
        return [self._apply_client_to_instance(job) for job in jobs]

    async def cancel(self, job: Job | str) -> None:
        """Cancel a job.

        If the job hasn't started yet, it will be cancelled immediately.
        Jobs that are already finished, failed, or cancelled are not affected.

        Args:
            job_id: The ID of the job to cancel.

        """
        job_id = job._id_or_error if isinstance(job, Job) else job
        await self._low_level_client.cancel_job(job_id)

    async def retry(self, job: Job | str) -> Job:
        """Retry a failed job.

        Jobs that are finished, in progress, or in the process of being cancelled are not affected.

        Args:
            job: The Job object or job ID to retry.

        Returns:
            The updated Job object.
        """
        job_id = job._id_or_error if isinstance(job, Job) else job
        updated_job = await self._low_level_client.retry_job(job_id)
        return self._apply_client_to_instance(updated_job)
