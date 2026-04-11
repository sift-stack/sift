from __future__ import annotations

import asyncio
import tempfile
import time
import zipfile
from pathlib import Path
from typing import TYPE_CHECKING

from alive_progress import alive_bar  # type: ignore[import-untyped]

from sift_client._internal.low_level_wrappers.jobs import JobsLowLevelClient
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.file import download_file, extract_zip
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.job import DataExportStatusDetails, Job, JobStatus, JobType
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

    async def wait_until_complete(
        self,
        job: Job | str,
        *,
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
        show_progress: bool | None = None,
    ) -> Job:
        """Wait until the job is complete or the timeout is reached.

        Polls the job status at the given interval until the job is FINISHED,
        FAILED, or CANCELLED, returning the completed Job

        Args:
            job: The Job or job_id to wait for.
            polling_interval_secs: Seconds between status polls. Defaults to 5s.
            timeout_secs: Maximum seconds to wait. If None, polls indefinitely.
                Defaults to None (indefinite).
            show_progress: If True, display an animated progress spinner alongside
                the job status while polling. Defaults to True for sync, False
                for async. Use ``sift_client.config.show_progress = False`` to disable
                globally for sync.

        Returns:
            The Job in the completed state.
        """
        job_id = job._id_or_error if isinstance(job, Job) else job
        if show_progress is None:
            show_progress = self._show_progress()

        start = time.monotonic()
        with alive_bar(
            title=f"Job {job_id}: polling",
            bar=None,
            spinner_length=7,
            spinner="dots_waves",
            monitor=False,
            stats=False,
            disable=not show_progress,
        ) as bar:
            while True:
                job = await self.get(job_id)
                bar.title(f"Job {job_id} ({job.job_type.value.lower()}): {job.job_status.value}")
                bar()
                if job.job_status in (JobStatus.FINISHED, JobStatus.FAILED, JobStatus.CANCELLED):
                    return job
                if timeout_secs is not None and (time.monotonic() - start) >= timeout_secs:
                    raise TimeoutError(
                        f"Job {job_id} did not complete within {timeout_secs} seconds"
                    )
                await asyncio.sleep(polling_interval_secs)

    async def wait_and_download(
        self,
        job: Job | str,
        *,
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
        output_dir: str | Path | None = None,
        extract: bool = True,
        show_progress: bool | None = None,
    ) -> list[Path]:
        """Wait for a job to complete and download the result files.

        Polls the job status at the given interval until the job is FINISHED,
        FAILED, or CANCELLED, then downloads the result files.

        Args:
            job: The Job or job ID to wait for.
            polling_interval_secs: Seconds between status polls. Defaults to 5.
            timeout_secs: Maximum seconds to wait. If None, polls indefinitely.
            output_dir: Directory to save the downloaded files. If omitted, a
                temporary directory is created automatically.
            extract: If True (default) and the downloaded file is a zip,
                extract it and delete the archive, returning paths to the
                extracted files. Non-zip files are returned as-is regardless
                of this flag.
            show_progress: If True, display an animated progress spinner
                while waiting and a download progress bar. Defaults to True
                for sync, False for async. Use ``sift_client.config.show_progress = False``
                to disable globally for sync.

        Returns:
            List of paths to the downloaded/extracted files.

        Raises:
            RuntimeError: If the job fails or is cancelled.
            TimeoutError: If the job does not complete within timeout_secs.
        """
        job_id = job._id_or_error if isinstance(job, Job) else job
        if show_progress is None:
            show_progress = self._show_progress()

        completed_job = await self.wait_until_complete(
            job=job_id,
            polling_interval_secs=polling_interval_secs,
            timeout_secs=timeout_secs,
            show_progress=show_progress,
        )
        if completed_job.job_status == JobStatus.FAILED:
            if (
                isinstance(completed_job.job_status_details, DataExportStatusDetails)
                and completed_job.job_status_details.error_message
            ):
                raise RuntimeError(
                    f"Export job '{job_id}' failed. {completed_job.job_status_details.error_message}"
                )
            raise RuntimeError(f"Export job '{job_id}' failed.")
        if completed_job.job_status == JobStatus.CANCELLED:
            raise RuntimeError(f"Export job '{job_id}' was cancelled.")

        presigned_url = await self.client.async_.data_export._low_level_client.get_download_url(
            job_id=job_id
        )
        output_dir = (
            Path(output_dir)
            if output_dir is not None
            else Path(tempfile.mkdtemp(prefix="sift_export_"))
        )
        download_path = output_dir / job_id

        # Run the synchronous download in a thread pool to avoid blocking the event loop
        rest_client = self.client.rest_client
        await run_sync_function(
            lambda: download_file(
                presigned_url, download_path, rest_client=rest_client, show_progress=show_progress
            )
        )

        if not extract or not zipfile.is_zipfile(download_path):
            return [download_path]
        return extract_zip(download_path, output_dir)
