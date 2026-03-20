from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.exports import ExportsLowLevelClient
from sift_client._internal.util.channels import resolve_calculated_channels
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.calculated_channel import CalculatedChannelCreate
from sift_client.sift_types.channel import Channel
from sift_client.sift_types.export import ExportOutputFormat  # noqa: TC001
from sift_client.sift_types.run import Run

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.calculated_channel import CalculatedChannel
    from sift_client.sift_types.job import Job


class DataExportAPIAsync(ResourceBase):
    """High-level API for exporting data from Sift."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the DataExportAPI.

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
        ``job.wait_and_download()`` to poll for completion and download the files.

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
        normalized_calc_channels: list[CalculatedChannel | CalculatedChannelCreate] | None = (
            [
                CalculatedChannelCreate.model_validate(cc) if isinstance(cc, dict) else cc
                for cc in calculated_channels
            ]
            if calculated_channels
            else None
        )
        resolved_calc_channels = await resolve_calculated_channels(
            normalized_calc_channels,
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
