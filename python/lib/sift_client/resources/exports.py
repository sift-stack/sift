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
from sift_client.sift_types.export import ExportCalculatedChannel, ExportOutputFormat  # noqa: TC001

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.client import SiftClient


def _build_calc_configs(
    calculated_channel_configs: list[ExportCalculatedChannel] | None,
) -> list[CalculatedChannelConfig] | None:
    """Convert CalculatedChannel Pydantic models to proto CalculatedChannelConfig messages."""
    if not calculated_channel_configs:
        return None
    return [
        CalculatedChannelConfig(
            name=cc.name,
            expression=cc.expression,
            channel_references=[
                CalculatedChannelAbstractChannelReference(
                    channel_reference=ref.channel_reference,
                    channel_identifier=ref.channel_identifier,
                )
                for ref in cc.channel_references
            ],
            units=cc.units,
        )
        for cc in calculated_channel_configs
    ]


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
        run_ids: list[str],
        output_format: ExportOutputFormat,
        start_time: datetime | None = None,
        stop_time: datetime | None = None,
        channel_ids: list[str] | None = None,
        calculated_channel_configs: list[ExportCalculatedChannel] | None = None,
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
            output_format: The file format for the export (CSV or SUN).
            start_time: Optional start time to narrow the export within the run(s).
            stop_time: Optional stop time to narrow the export within the run(s).
            channel_ids: Optional list of channel IDs to include. If omitted, all channels are exported.
            calculated_channel_configs: Optional inline calculated channels to include in the export.
            use_legacy_format: Use legacy key-value metadata format for channel headers.
            simplify_channel_names: Remove the component part of channel names if unique in the export.
            combine_runs: Combine channels from the same asset across different runs into a single column.
            split_export_by_asset: Split each asset into its own export file.
            split_export_by_run: Split each run into its own export file.
            polling_interval_secs: Seconds between status polls for async exports. Defaults to 5.
            timeout_secs: Maximum seconds to wait for async exports. None means wait indefinitely.

        Returns:
            A presigned download URL for the exported zip file.

        Raises:
            TimeoutError: If the export job does not complete within timeout_secs.
        """
        if not run_ids:
            raise ValueError("'run_ids' must be a non-empty list of run IDs.")
        if any(not run_id for run_id in run_ids):
            raise ValueError("'run_ids' must not contain empty or null values.")
        if (start_time is None) != (stop_time is None):
            raise ValueError("'start_time' and 'stop_time' must both be provided or both omitted.")
        if start_time and stop_time and start_time >= stop_time:
            raise ValueError("'start_time' must be before 'stop_time'.")

        runs_and_time_range = RunsAndTimeRange(run_ids=run_ids)
        if start_time:
            runs_and_time_range.start_time.CopyFrom(to_pb_timestamp(start_time))
        if stop_time:
            runs_and_time_range.stop_time.CopyFrom(to_pb_timestamp(stop_time))

        calc_configs = _build_calc_configs(calculated_channel_configs)

        export_options = ExportOptions(
            use_legacy_format=use_legacy_format,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        request = ExportDataRequest(
            runs_and_time_range=runs_and_time_range,
            output_format=output_format.value,
            export_options=export_options,
            channel_ids=channel_ids or [],
            calculated_channel_configs=calc_configs or [],
        )

        response = await self._low_level_client.export_data(request=request)

        if response.presigned_url:
            return response.presigned_url
        return await self._await_download_url(
            job_id=response.job_id,
            polling_interval_secs=polling_interval_secs,
            timeout_secs=timeout_secs,
        )

    async def export_by_asset(
        self,
        *,
        asset_ids: list[str],
        start_time: datetime,
        stop_time: datetime,
        output_format: ExportOutputFormat,
        channel_ids: list[str] | None = None,
        calculated_channel_configs: list[ExportCalculatedChannel] | None = None,
        use_legacy_format: bool = False,
        simplify_channel_names: bool = False,
        combine_runs: bool = False,
        split_export_by_asset: bool = False,
        split_export_by_run: bool = False,
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
    ) -> str:
        """Export data scoped by one or more assets within a time range.

        Both start_time and stop_time are required. If no channel_ids or
        calculated_channel_configs are provided, all channels from the assets are included.

        Args:
            asset_ids: One or more asset IDs to export data from.
            start_time: Start of the time range to export.
            stop_time: End of the time range to export.
            output_format: The file format for the export (CSV or SUN).
            channel_ids: Optional list of channel IDs to include. If omitted, all channels are exported.
            calculated_channel_configs: Optional inline calculated channels to include in the export.
            use_legacy_format: Use legacy key-value metadata format for channel headers.
            simplify_channel_names: Remove the component part of channel names if unique in the export.
            combine_runs: Combine channels from the same asset across different runs into a single column.
            split_export_by_asset: Split each asset into its own export file.
            split_export_by_run: Split each run into its own export file.
            polling_interval_secs: Seconds between status polls for async exports. Defaults to 5.
            timeout_secs: Maximum seconds to wait for async exports. None means wait indefinitely.

        Returns:
            A presigned download URL for the exported zip file.

        Raises:
            TimeoutError: If the export job does not complete within timeout_secs.
        """
        if not asset_ids:
            raise ValueError("'asset_ids' must be a non-empty list of asset IDs.")
        if any(not asset_id for asset_id in asset_ids):
            raise ValueError("'asset_ids' must not contain empty or null values.")
        if start_time >= stop_time:
            raise ValueError("'start_time' must be before 'stop_time'.")

        assets_and_time_range = AssetsAndTimeRange(asset_ids=asset_ids)
        assets_and_time_range.start_time.CopyFrom(to_pb_timestamp(start_time))
        assets_and_time_range.stop_time.CopyFrom(to_pb_timestamp(stop_time))

        calc_configs = _build_calc_configs(calculated_channel_configs)

        export_options = ExportOptions(
            use_legacy_format=use_legacy_format,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        request = ExportDataRequest(
            assets_and_time_range=assets_and_time_range,
            channel_ids=channel_ids or [],
            calculated_channel_configs=calc_configs or [],
            output_format=output_format.value,
            export_options=export_options,
        )

        response = await self._low_level_client.export_data(request=request)

        if response.presigned_url:
            return response.presigned_url
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
        channel_ids: list[str] | None = None,
        calculated_channel_configs: list[ExportCalculatedChannel] | None = None,
        use_legacy_format: bool = False,
        simplify_channel_names: bool = False,
        combine_runs: bool = False,
        split_export_by_asset: bool = False,
        split_export_by_run: bool = False,
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
    ) -> str:
        """Export data within a time range.

        Both start_time and stop_time are required. At least one of channel_ids or
        calculated_channel_configs **must** be provided to scope the data, since there
        are no runs or assets to infer channels from.

        Args:
            start_time: Start of the time range to export.
            stop_time: End of the time range to export.
            output_format: The file format for the export (CSV or SUN).
            channel_ids: List of channel IDs to include in the export.
            calculated_channel_configs: Inline calculated channels to include in the export.
            use_legacy_format: Use legacy key-value metadata format for channel headers.
            simplify_channel_names: Remove the component part of channel names if unique in the export.
            combine_runs: Combine channels from the same asset across different runs into a single column.
            split_export_by_asset: Split each asset into its own export file.
            split_export_by_run: Split each run into its own export file.
            polling_interval_secs: Seconds between status polls for async exports. Defaults to 5.
            timeout_secs: Maximum seconds to wait for async exports. None means wait indefinitely.

        Returns:
            A presigned download URL for the exported zip file.

        Raises:
            ValueError: If neither channel_ids nor calculated_channel_configs is provided.
            TimeoutError: If the export job does not complete within timeout_secs.
        """
        if not channel_ids and not calculated_channel_configs:
            raise ValueError(
                "At least one of 'channel_ids' or 'calculated_channel_configs' must be provided "
                "when exporting by time range."
            )
        if start_time >= stop_time:
            raise ValueError("'start_time' must be before 'stop_time'.")

        time_range = TimeRange()
        time_range.start_time.CopyFrom(to_pb_timestamp(start_time))
        time_range.stop_time.CopyFrom(to_pb_timestamp(stop_time))

        calc_configs = _build_calc_configs(calculated_channel_configs)

        export_options = ExportOptions(
            use_legacy_format=use_legacy_format,
            simplify_channel_names=simplify_channel_names,
            combine_runs=combine_runs,
            split_export_by_asset=split_export_by_asset,
            split_export_by_run=split_export_by_run,
        )

        request = ExportDataRequest(
            time_range=time_range,
            channel_ids=channel_ids or [],
            calculated_channel_configs=calc_configs or [],
            output_format=output_format.value,
            export_options=export_options,
        )

        response = await self._low_level_client.export_data(request=request)

        if response.presigned_url:
            return response.presigned_url
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
            raise RuntimeError(f"Export job '{job_id}' failed{reason}")
        if job.job_status == JobStatus.CANCELLED:
            raise RuntimeError(f"Export job '{job_id}' was cancelled.")
        return await self._low_level_client.get_download_url(job_id=job_id)
