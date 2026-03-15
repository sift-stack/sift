from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.exports import ExportsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.channel import Channel
from sift_client.sift_types.export import ExportOutputFormat  # noqa: TC001
from sift_client.sift_types.job import Job
from sift_client.sift_types.run import Run

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.calculated_channel import CalculatedChannel, CalculatedChannelCreate


class ExportsAPIAsync(ResourceBase):
    """High-level API for exporting data from Sift.

    Provides three export methods based on how you want to scope the data:

    - ``export_by_run`` - Export data from one or more runs.
    - ``export_by_asset`` - Export data from one or more assets within a time range.
    - ``export_by_time_range`` - Export data within a time range (requires channels or calculated_channels).

    Each method initiates the export and returns a Job handle. Use ``wait_until_complete``
    to poll the job and retrieve the download URL.

    Example::

        from sift_client.sift_types.export import ExportOutputFormat

        # Export by run
        run = await client.async_.runs.get(run_id="run-id-1")
        job = await client.async_.exports.export_by_run(
            runs=[run],
            output_format=ExportOutputFormat.CSV,
        )
        url = await client.async_.exports.wait_until_complete(job=job)

        # Export by asset with time range
        asset = await client.async_.assets.get(asset_id="asset-id-1")
        job = await client.async_.exports.export_by_asset(
            assets=[asset],
            start_time=start,
            stop_time=stop,
            output_format=ExportOutputFormat.CSV,
        )
        url = await client.async_.exports.wait_until_complete(job=job)
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the ExportsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ExportsLowLevelClient(grpc_client=self._sift_client.grpc_client)

    async def export_by_run(
        self,
        *,
        runs: list[str | Run],
        output_format: ExportOutputFormat,
        start_time: datetime | None = None,
        stop_time: datetime | None = None,
        channels: list[str | Channel] | None = None,
        calculated_channels: list[CalculatedChannel | CalculatedChannelCreate] | None = None,
        use_legacy_format: bool = False,
        simplify_channel_names: bool = False,
        combine_runs: bool = False,
        split_export_by_asset: bool = False,
        split_export_by_run: bool = False,
    ) -> Job:
        """Export data scoped by one or more runs.

        Initiates the export on the server and returns a Job handle. Use
        ``wait_until_complete`` to poll for completion and get the download URL.

        If no start_time/stop_time are provided, the full time range of each run is used.
        If no channels or calculated_channels are provided, all channels from
        the run's assets are included.

        Args:
            runs: One or more Run objects or run IDs to export data from.
            output_format: The file format for the export (CSV or Sun/WinPlot).
            start_time: Optional start time to narrow the export within the run(s).
            stop_time: Optional stop time to narrow the export within the run(s).
            channels: Optional list of Channel objects or channel IDs to include. If omitted, all channels are exported.
            calculated_channels: Optional calculated channels to include in the export. Accepts existing CalculatedChannel objects or CalculatedChannelCreate definitions.
            use_legacy_format: Use legacy channel name display format: ``channel.name (assetName=... runName=... runId=...)``.
            simplify_channel_names: Remove text preceding last period in channel names, only if the resulting simplified name is unique.
            combine_runs: Identical channels within the same asset across multiple runs will be combined into a single column.
            split_export_by_asset: Split each asset into a separate file, with asset name removed from channel name display.
            split_export_by_run: Split each run into a separate file, with run name removed from channel name display.

        Returns:
            A Job handle for the pending export.
        """
        if not runs:
            raise ValueError("'runs' must be a non-empty list of run objects or run ids.")
        if any(not run for run in runs):
            raise ValueError("'runs' must not contain empty or null values.")
        if (start_time is None) != (stop_time is None):
            raise ValueError("'start_time' and 'stop_time' must both be provided or both omitted.")
        if start_time and stop_time and start_time >= stop_time:
            raise ValueError("'start_time' must be before 'stop_time'.")

        run_ids = [r._id_or_error if isinstance(r, Run) else r for r in runs]
        channel_ids = (
            [c._id_or_error if isinstance(c, Channel) else c for c in channels] if channels else []
        )

        job_id = await self._low_level_client.export_by_run(
            run_ids=run_ids,
            output_format=output_format,
            start_time=start_time,
            stop_time=stop_time,
            channel_ids=channel_ids,
            calculated_channels=calculated_channels,
            use_legacy_format=use_legacy_format,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        job = await self.client.async_.jobs.get(job_id=job_id)
        return self._apply_client_to_instance(job)

    async def export_by_asset(
        self,
        *,
        assets: list[str | Asset],
        start_time: datetime,
        stop_time: datetime,
        output_format: ExportOutputFormat,
        channels: list[str | Channel] | None = None,
        calculated_channels: list[CalculatedChannel | CalculatedChannelCreate] | None = None,
        use_legacy_format: bool = False,
        simplify_channel_names: bool = False,
        combine_runs: bool = False,
        split_export_by_asset: bool = False,
        split_export_by_run: bool = False,
    ) -> Job:
        """Export data scoped by one or more assets within a time range.

        Initiates the export on the server and returns a Job handle. Use
        ``wait_until_complete`` to poll for completion and get the download URL.

        Both start_time and stop_time are required. If no channels or
        calculated_channels are provided, all channels from the assets are included.

        Args:
            assets: One or more Asset objects or asset IDs to export data from.
            start_time: Start of the time range to export.
            stop_time: End of the time range to export.
            output_format: The file format for the export (CSV, Parquet, or Sun/WinPlot).
            channels: Optional list of Channel objects or channel IDs to include. If omitted, all channels are exported.
            calculated_channels: Optional calculated channels to include in the export. Accepts existing CalculatedChannel objects or CalculatedChannelCreate definitions.
            use_legacy_format: Use legacy channel name display format: ``channel.name (assetName=... runName=... runId=...)``.
            simplify_channel_names: Remove text preceding last period in channel names, only if the resulting simplified name is unique.
            combine_runs: Identical channels within the same asset across multiple runs will be combined into a single column.
            split_export_by_asset: Split each asset into a separate file, with asset name removed from channel name display.
            split_export_by_run: Split each run into a separate file, with run name removed from channel name display.

        Returns:
            A Job handle for the pending export.
        """
        if not assets:
            raise ValueError("'assets' must be a non-empty list of asset objects or asset IDs.")
        if any(not asset for asset in assets):
            raise ValueError("'assets' must not contain empty or null values.")
        if start_time >= stop_time:
            raise ValueError("'start_time' must be before 'stop_time'.")

        asset_ids = [a._id_or_error if isinstance(a, Asset) else a for a in assets]
        channel_ids = (
            [c._id_or_error if isinstance(c, Channel) else c for c in channels] if channels else []
        )

        job_id = await self._low_level_client.export_by_asset(
            asset_ids=asset_ids,
            start_time=start_time,
            stop_time=stop_time,
            output_format=output_format,
            channel_ids=channel_ids,
            calculated_channels=calculated_channels,
            use_legacy_format=use_legacy_format,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        job = await self.client.async_.jobs.get(job_id=job_id)
        return self._apply_client_to_instance(job)

    async def export_by_time_range(
        self,
        *,
        start_time: datetime,
        stop_time: datetime,
        output_format: ExportOutputFormat,
        channels: list[str | Channel] | None = None,
        calculated_channels: list[CalculatedChannel | CalculatedChannelCreate] | None = None,
        use_legacy_format: bool = False,
        simplify_channel_names: bool = False,
        combine_runs: bool = False,
        split_export_by_asset: bool = False,
        split_export_by_run: bool = False,
    ) -> Job:
        """Export data within a time range.

        Initiates the export on the server and returns a Job handle. Use
        ``wait_until_complete`` to poll for completion and get the download URL.

        Both start_time and stop_time are required. At least one of channels or
        calculated_channels **must** be provided to scope the data, since there
        are no runs or assets to infer channels from.

        Args:
            start_time: Start of the time range to export.
            stop_time: End of the time range to export.
            output_format: The file format for the export (CSV, Parquet, or Sun/WinPlot).
            channels: List of Channel objects or channel IDs to include in the export.
            calculated_channels: Calculated channels to include in the export. Accepts existing CalculatedChannel objects or CalculatedChannelCreate definitions.
            use_legacy_format: Use legacy channel name display format: ``channel.name (assetName=... runName=... runId=...)``.
            simplify_channel_names: Remove text preceding last period in channel names, only if the resulting simplified name is unique.
            combine_runs: Identical channels within the same asset across multiple runs will be combined into a single column.
            split_export_by_asset: Split each asset into a separate file, with asset name removed from channel name display.
            split_export_by_run: Split each run into a separate file, with run name removed from channel name display.

        Returns:
            A Job handle for the pending export.

        Raises:
            ValueError: If neither channels nor calculated_channels is provided.
        """
        if not channels and not calculated_channels:
            raise ValueError(
                "At least one of 'channels' or 'calculated_channels' must be provided "
                "when exporting by time range."
            )
        if start_time >= stop_time:
            raise ValueError("'start_time' must be before 'stop_time'.")

        channel_ids = (
            [c._id_or_error if isinstance(c, Channel) else c for c in channels] if channels else []
        )

        job_id = await self._low_level_client.export_by_time_range(
            start_time=start_time,
            stop_time=stop_time,
            output_format=output_format,
            channel_ids=channel_ids,
            calculated_channels=calculated_channels,
            use_legacy_format=use_legacy_format,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        job = await self.client.async_.jobs.get(job_id=job_id)
        return self._apply_client_to_instance(job)

    async def wait_until_complete(
        self,
        *,
        job: Job | str,
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
    ) -> str:
        """Wait for an export job to complete and return the download URL.

        Polls the job status at the given interval until the job is FINISHED,
        FAILED, or CANCELLED.

        Args:
            job: The export Job or job ID to wait for.
            polling_interval_secs: Seconds between status polls. Defaults to 5.
            timeout_secs: Maximum seconds to wait. If None, polls indefinitely.

        Returns:
            A presigned download URL for the exported zip file.

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
            reason = ""
            if (
                isinstance(completed_job.job_status_details, DataExportStatusDetails)
                and completed_job.job_status_details.error_message
            ):
                reason = f": {completed_job.job_status_details.error_message}"
            raise RuntimeError(f"Export job '{job_id}' failed: {reason}")
        if completed_job.job_status == JobStatus.CANCELLED:
            raise RuntimeError(f"Export job '{job_id}' was cancelled.")
        return await self._low_level_client.get_download_url(job_id=job_id)
