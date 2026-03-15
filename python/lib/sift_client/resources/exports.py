from __future__ import annotations

from typing import TYPE_CHECKING

from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CalculatedChannelAbstractChannelReference,
)
from sift.exports.v1.exports_pb2 import (
    AssetsAndTimeRange,
    CalculatedChannelConfig,
    ExportDataRequest,
    ExportOptions,
    RunsAndTimeRange,
    TimeRange,
)

from sift_client._internal.low_level_wrappers.exports import ExportsLowLevelClient
from sift_client._internal.util.timestamp import to_pb_timestamp
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.export import ExportOutputFormat  # noqa: TC001
from sift_client.sift_types.run import Run
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.channel import Channel
from sift_client.sift_types.calculated_channel import CalculatedChannel, CalculatedChannelCreate

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.client import SiftClient


def _build_calc_channels(
    calculated_channels: list[CalculatedChannel | CalculatedChannelCreate] | None,
) -> list[CalculatedChannelConfig]:
    if not calculated_channels:
        return []
    configs = []
    for cc in calculated_channels:
        if isinstance(cc, CalculatedChannelCreate):
            refs = cc.expression_channel_references or []
        else:
            refs = cc.channel_references
        configs.append(
            CalculatedChannelConfig(
                name=cc.name,
                expression=cc.expression,
                channel_references=[
                    CalculatedChannelAbstractChannelReference(
                        channel_reference=ref.channel_reference,
                        channel_identifier=ref.channel_identifier,
                    )
                    for ref in refs
                ],
                units=cc.units,
            )
        )
    return configs


class ExportsAPIAsync(ResourceBase):
    """High-level API for exporting data from Sift.

    Provides three export methods based on how you want to scope the data:

    - ``export_by_run`` - Export data from one or more runs.
    - ``export_by_asset`` - Export data from one or more assets within a time range.
    - ``export_by_time_range`` - Export data within a time range (requires channel_ids or calculated_channel_configs).

    Each method handles the full export lifecycle: initiating the export, polling for
    completion (if async), and returning the download URL.

    Example::

        from sift_client.sift_types.export import ExportOutputFormat

        # Export by run
        url = await client.async_.exports.export_by_run(
            run_ids=["run-id-1"],
            output_format=ExportOutputFormat.CSV,
        )

        # Export by asset with time range
        url = await client.async_.exports.export_by_asset(
            asset_ids=["asset-id-1"],
            start_time=start,
            stop_time=stop,
            output_format=ExportOutputFormat.CSV,
        )
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
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
    ) -> str:
        """Export data scoped by one or more runs.

        If no start_time/stop_time are provided, the full time range of each run is used.
        If no channel_ids or calculated_channel_configs are provided, all channels from
        the run's assets are included.

        Args:
            run_ids: One or more run IDs to export data from.
            output_format: The file format for the export (CSV or Sun/WinPlot).
            start_time: Optional start time to narrow the export within the run(s).
            stop_time: Optional stop time to narrow the export within the run(s).
            channel_ids: Optional list of channel IDs to include. If omitted, all channels are exported.
            calculated_channel_configs: Optional inline calculated channels to include in the export.
            use_legacy_format: Use legacy channel name display format: ``channel.name (assetName=... runName=... runId=...)``.
            simplify_channel_names: Remove text preceding last period in channel names, only if the resulting simplified name is unique.
            combine_runs: Identical channels within the same asset across multiple runs will be combined into a single column.
            split_export_by_asset: Split each asset into a separate file, with asset name removed from channel name display.
            split_export_by_run: Split each run into a separate file, with run name removed from channel name display.
            polling_interval_secs: Seconds between status polls for async exports. Defaults to 5.
            timeout_secs: Maximum seconds to wait for async exports. None means wait indefinitely.

        Returns:
            A presigned download URL for the exported zip file.

        Raises:
            TimeoutError: If the export job does not complete within timeout_secs.
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

        runs_and_time_range = RunsAndTimeRange(run_ids=run_ids)
        if start_time:
            runs_and_time_range.start_time.CopyFrom(to_pb_timestamp(start_time))
        if stop_time:
            runs_and_time_range.stop_time.CopyFrom(to_pb_timestamp(stop_time))

        export_options = ExportOptions(
            use_legacy_format=use_legacy_format,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        channel_ids = (
            [c._id_or_error if isinstance(c, Channel) else c for c in channels] if channels else []
        )

        request = ExportDataRequest(
            runs_and_time_range=runs_and_time_range,
            output_format=output_format.value,
            export_options=export_options,
            channel_ids=channel_ids,
            calculated_channel_configs=_build_calc_channels(calculated_channels),
        )

        response = await self._low_level_client.export_data(request=request)

        return await self._await_download_url(
            job_id=response.job_id,
            polling_interval_secs=polling_interval_secs,
            timeout_secs=timeout_secs,
        )

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
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
    ) -> str:
        """Export data scoped by one or more assets within a time range.

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
            polling_interval_secs: Seconds between status polls for async exports. Defaults to 5.
            timeout_secs: Maximum seconds to wait for async exports. None means wait indefinitely.

        Returns:
            A presigned download URL for the exported zip file.

        Raises:
            TimeoutError: If the export job does not complete within timeout_secs.
        """
        if not assets:
            raise ValueError("'assets' must be a non-empty list of asset objects or asset IDs.")
        if any(not asset for asset in assets):
            raise ValueError("'assets' must not contain empty or null values.")
        if start_time >= stop_time:
            raise ValueError("'start_time' must be before 'stop_time'.")

        asset_ids = [a._id_or_error if isinstance(a, Asset) else a for a in assets]

        assets_and_time_range = AssetsAndTimeRange(asset_ids=asset_ids)
        assets_and_time_range.start_time.CopyFrom(to_pb_timestamp(start_time))
        assets_and_time_range.stop_time.CopyFrom(to_pb_timestamp(stop_time))

        export_options = ExportOptions(
            use_legacy_format=use_legacy_format,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        channel_ids = (
            [c._id_or_error if isinstance(c, Channel) else c for c in channels] if channels else []
        )

        request = ExportDataRequest(
            assets_and_time_range=assets_and_time_range,
            channel_ids=channel_ids,
            calculated_channel_configs=_build_calc_channels(calculated_channels),
            output_format=output_format.value,
            export_options=export_options,
        )

        response = await self._low_level_client.export_data(request=request)

        return await self._await_download_url(
            job_id=response.job_id,
            polling_interval_secs=polling_interval_secs,
            timeout_secs=timeout_secs,
        )

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
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
    ) -> str:
        """Export data within a time range.

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
            polling_interval_secs: Seconds between status polls for async exports. Defaults to 5.
            timeout_secs: Maximum seconds to wait for async exports. None means wait indefinitely.

        Returns:
            A presigned download URL for the exported zip file.

        Raises:
            ValueError: If neither channels nor calculated_channels is provided.
            TimeoutError: If the export job does not complete within timeout_secs.
        """
        if not channels and not calculated_channels:
            raise ValueError(
                "At least one of 'channels' or 'calculated_channels' must be provided "
                "when exporting by time range."
            )
        if start_time >= stop_time:
            raise ValueError("'start_time' must be before 'stop_time'.")

        time_range = TimeRange()
        time_range.start_time.CopyFrom(to_pb_timestamp(start_time))
        time_range.stop_time.CopyFrom(to_pb_timestamp(stop_time))

        export_options = ExportOptions(
            use_legacy_format=use_legacy_format,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        channel_ids = (
            [c._id_or_error if isinstance(c, Channel) else c for c in channels] if channels else []
        )

        request = ExportDataRequest(
            time_range=time_range,
            channel_ids=channel_ids,
            calculated_channel_configs=_build_calc_channels(calculated_channels),
            output_format=output_format.value,
            export_options=export_options,
        )

        response = await self._low_level_client.export_data(request=request)

        return await self._await_download_url(
            job_id=response.job_id,
            polling_interval_secs=polling_interval_secs,
            timeout_secs=timeout_secs,
        )

    async def _await_download_url(
        self, job_id: str, polling_interval_secs: int = 5, timeout_secs: int | None = None
    ) -> str:
        """Poll a background export job until complete, then return the download URL."""
        from sift_client.sift_types.job import DataExportStatusDetails, JobStatus

        job = await self.client.async_.jobs.wait_until_complete(
            job=job_id, polling_interval_secs=polling_interval_secs, timeout_secs=timeout_secs
        )
        if job.job_status == JobStatus.FAILED:
            reason = ""
            if (
                isinstance(job.job_status_details, DataExportStatusDetails)
                and job.job_status_details.error_message
            ):
                reason = f": {job.job_status_details.error_message}"
            raise RuntimeError(f"Export job '{job_id}' failed {reason}")
        if job.job_status == JobStatus.CANCELLED:
            raise RuntimeError(f"Export job '{job_id}' was cancelled.")
        return await self._low_level_client.get_download_url(job_id=job_id)
