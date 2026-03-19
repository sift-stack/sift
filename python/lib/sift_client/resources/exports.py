from __future__ import annotations

import asyncio
import tempfile
from pathlib import Path
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.exports import ExportsLowLevelClient
from sift_client._internal.util.channels import resolve_calculated_channels
from sift_client._internal.util.download import download_and_extract_zip
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.calculated_channel import CalculatedChannelCreate
from sift_client.sift_types.channel import Channel
from sift_client.sift_types.export import ExportOutputFormat  # noqa: TC001
from sift_client.sift_types.job import Job
from sift_client.sift_types.run import Run

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.calculated_channel import CalculatedChannel


class ExportsAPIAsync(ResourceBase):
    """High-level API for exporting data from Sift."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the ExportsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ExportsLowLevelClient(grpc_client=self.client.grpc_client)

    async def export(
        self,
        *,
        output_format: ExportOutputFormat,
        runs: list[str | Run] | None = None,
        assets: list[str | Asset] | None = None,
        start_time: datetime | None = None,
        stop_time: datetime | None = None,
        channels: list[str | Channel] | None = None,
        calculated_channels: list[CalculatedChannel | CalculatedChannelCreate | dict] | None = None,
        simplify_channel_names: bool = False,
        combine_runs: bool = False,
        split_export_by_asset: bool = False,
        split_export_by_run: bool = False,
    ) -> Job:
        """Export data from Sift.

        Initiates an export on the server and returns a Job handle. Use
        ``wait_and_download`` to poll for completion and download the files.

        There are three ways to scope the export, determined by which arguments
        are provided:

        1. **By runs** — provide ``runs``. The ``start_time``/``stop_time`` are
           optional (if omitted, the full time range of each run is used). If no
           ``channels`` or ``calculated_channels`` are provided, all channels
           from the runs' assets are included.

        2. **By assets** — provide ``assets``. Both ``start_time`` and
           ``stop_time`` are **required**. If no ``channels`` or
           ``calculated_channels`` are provided, all channels from the assets
           are included.

        3. **By time range only** — provide ``start_time`` and ``stop_time``
           without ``runs`` or ``assets``. At least one of ``channels`` or
           ``calculated_channels`` **must** be provided to scope the data.

        You cannot provide both ``runs`` and ``assets`` at the same time.

        Args:
            output_format: The file format for the export (CSV or Sun/WinPlot).
            runs: One or more Run objects or run IDs to export data from.
            assets: One or more Asset objects or asset IDs to export data from.
            start_time: Start of the time range to export. Required when using
                assets or time-range-only mode; optional when using runs.
            stop_time: End of the time range to export. Required when using
                assets or time-range-only mode; optional when using runs.
            channels: Channel objects or channel IDs to include. If omitted and
                runs or assets are provided, all channels are exported. Required
                (along with ``calculated_channels``) in time-range-only mode.
            calculated_channels: Calculated channels to include in the export.
                Accepts existing CalculatedChannel objects,
                CalculatedChannelCreate definitions, or dictionaries that
                will be converted to CalculatedChannelCreate via model_validate.
            simplify_channel_names: Remove text preceding last period in channel
                names, only if the resulting simplified name is unique.
            combine_runs: Identical channels within the same asset across
                multiple runs will be combined into a single column.
            split_export_by_asset: Split each asset into a separate file, with
                asset name removed from channel name display.
            split_export_by_run: Split each run into a separate file, with run
                name removed from channel name display.

        Returns:
            A Job handle for the pending export.
        """
        if runs and assets:
            raise ValueError("Provide either 'runs' or 'assets', not both.")
        if not runs and not assets and not start_time and not stop_time:
            raise ValueError("At least one of 'runs', 'assets', or a time range must be provided.")

        run_ids = [r._id_or_error if isinstance(r, Run) else r for r in runs] if runs else None
        asset_ids = (
            [a._id_or_error if isinstance(a, Asset) else a for a in assets] if assets else None
        )
        channel_ids = (
            [c._id_or_error if isinstance(c, Channel) else c for c in channels] if channels else []
        )
        if calculated_channels:
            calculated_channels = [
                CalculatedChannelCreate.model_validate(cc) if isinstance(cc, dict) else cc
                for cc in calculated_channels
            ]
        resolved_calc_channels = await resolve_calculated_channels(
            calculated_channels,
            channels_api=self.client.async_.channels,
        )

        job_id = await self._low_level_client.export_data(
            run_ids=run_ids,
            asset_ids=asset_ids,
            output_format=output_format,
            start_time=start_time,
            stop_time=stop_time,
            channel_ids=channel_ids,
            calculated_channels=resolved_calc_channels,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        return await self.client.async_.jobs.get(job_id=job_id)

    async def wait_and_download(
        self,
        *,
        job: Job | str,
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
        output_dir: str | Path | None = None,
        extract: bool = True,
    ) -> list[Path]:
        """Wait for an export job to complete and download the exported files.

        Polls the job status at the given interval until the job is FINISHED,
        FAILED, or CANCELLED, then downloads and extracts the exported data files.

        Args:
            job: The export Job or job ID to wait for.
            polling_interval_secs: Seconds between status polls. Defaults to 5.
            timeout_secs: Maximum seconds to wait. If None, polls indefinitely.
            output_dir: Directory to save the extracted files. If omitted, a
                temporary directory is created automatically.
            extract: If True (default), extract the zip and delete it,
                returning paths to the extracted files. If False, keep the
                zip file and return its path.

        Returns:
            List of paths to the extracted data files, or a single-element
            list containing the zip path if extract is False.

        Raises:
            RuntimeError: If the export job fails or is cancelled.
            TimeoutError: If the export job does not complete within timeout_secs.
        """
        from sift_client.sift_types.job import DataExportStatusDetails, JobStatus

        job_id = job._id_or_error if isinstance(job, Job) else job

        completed_job = await self.client.async_.jobs.wait_until_complete(
            job=job_id,
            polling_interval_secs=polling_interval_secs,
            timeout_secs=timeout_secs,
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

        presigned_url = await self._low_level_client.get_download_url(job_id=job_id)
        output_dir = (
            Path(output_dir)
            if output_dir is not None
            else Path(tempfile.mkdtemp(prefix="sift_export_"))
        )
        zip_path = output_dir / f"{job_id}.zip"

        # Run the synchronous download in a thread pool to avoid blocking the event loop
        rest_client = self.client.rest_client
        loop = asyncio.get_running_loop()
        extracted_files = await loop.run_in_executor(
            None,
            lambda: download_and_extract_zip(
                presigned_url, zip_path, output_dir, rest_client=rest_client, extract=extract
            ),
        )

        return extracted_files
