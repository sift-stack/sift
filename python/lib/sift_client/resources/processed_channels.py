from __future__ import annotations

import asyncio
import tempfile
import time
import uuid
import warnings
from pathlib import Path
from typing import TYPE_CHECKING, Any, Mapping

from sift_client._internal.low_level_wrappers.processed_channels import (
    ProcessedChannelsLowLevelClient,
)
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.file import upload_file
from sift_client._internal.util.processed_channels import (
    default_code,
    write_processed_channel_parquet,
)
from sift_client._internal.util.progress import alive_bar
from sift_client.errors import SiftExperimentalWarning
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.processed_channel import (
    ProcessedChannelCode,
    ProcessedChannelInput,
    ProcessedChannelUpload,
    ProcessedChannelUploadError,
    ProcessedChannelWrite,
    ProcessedChannelWriteStatus,
)
from sift_client.sift_types.run import Run
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import pandas as pd

    from sift_client.client import SiftClient
    from sift_client.sift_types.channel import Channel


def warn_experimental(*, stacklevel: int = 1) -> None:
    """Warn that processed channels are experimental, at ``stacklevel`` frames above the caller.

    The sync client runs each call on its event loop thread, where the user's code isn't on the
    stack, so the sync client warns when ``client.processed_channel`` is accessed instead.
    """
    warnings.warn(
        "Processed channels are experimental and may change without notice.",
        SiftExperimentalWarning,
        stacklevel=stacklevel + 1,
    )


class ProcessedChannelsAPIAsync(ResourceBase):
    """High-level API for processed channels.

    A processed channel is a channel whose data your code writes. Each upload is a new write to
    one run. Reads return the run's latest complete write, and every earlier write stays readable
    by ID, so rerunning your code against a run replaces what readers see without losing history.

    Processed channels are experimental. The API behind them is unstable and can change without
    notice, and the feature must be enabled for your organization.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the ProcessedChannelsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ProcessedChannelsLowLevelClient(
            rest_client=self.client.rest_client,
        )

    async def upload(
        self,
        df: pd.DataFrame,
        *,
        asset: Asset | str,
        run: Run | str,
        units: Mapping[str, str] | None = None,
        time_column: str | None = None,
        code: ProcessedChannelCode | str | None = None,
        inputs: list[ProcessedChannelInput] | None = None,
        metadata: dict[str, str | float | bool] | None = None,
        wait: bool = True,
        polling_interval_secs: float = 2,
        timeout_secs: float | None = None,
        show_progress: bool | None = None,
    ) -> ProcessedChannelUpload:
        """Write a DataFrame to a run as processed channels.

        Each column other than the time becomes one channel on the asset, named after the column,
        with its data type taken from the column's type. A channel is created on its first upload
        and reused after that. Every upload is a new write: once it completes, reads of the
        channel on this run return it, and earlier writes stay readable by ID.

        By default the call blocks until the data is ingested and readable. Pass ``wait=False``
        to return as soon as the file is uploaded, then call ``wait_until_complete()`` on the
        result when you need the data.

        Examples:
            Write two channels and wait until they're readable:

                df = pd.DataFrame(
                    {"battery_health": [0.98, 0.97], "battery_health_sigma": [0.01, 0.02]},
                    index=pd.to_datetime(["2026-09-29T13:00:00Z", "2026-09-29T13:00:01Z"]),
                )
                client.processed_channel.upload(
                    df, asset="satellite-01", run=run, units={"battery_health": "pct"}
                )

            Start an upload, keep working, and wait later:

                upload = client.processed_channel.upload(df, asset=asset, run=run, wait=False)
                ...
                upload.wait_until_complete()

        Args:
            df: The data. Every column other than the time is a channel. Columns must hold
                numbers, booleans, strings or bytes.
            asset: The asset that owns the channels: an ``Asset``, an asset ID or an asset name.
            run: The run to write to: a ``Run`` or a run ID. It must belong to ``asset``, and may
                be ad hoc. A write never changes an ad hoc run's bounds, and fails if any point is
                outside them.
            units: Units by channel name, for example ``{"battery_health": "pct"}``.
            time_column: A datetime column holding each row's time. Defaults to the DataFrame's
                index, which must then be a ``DatetimeIndex``. Times without a time zone are
                read as UTC.
            code: The code that produced the data, as a ``ProcessedChannelCode`` or a ref string.
                Writes made by the same code with the same channels share a fingerprint, which is
                how Sift tells when a run was written by older code. Defaults to the running
                script's path and git commit, with a hash of the script. In a notebook, pass it
                yourself, since there is no script to identify.
            inputs: The channels the code read to produce the data.
            metadata: Key-value pairs that describe the write, for example its parameters.
            wait: Block until every write is complete or failed. Defaults to True.
            polling_interval_secs: Seconds between status polls while waiting.
            timeout_secs: Maximum seconds to wait. If None, waits indefinitely.
            show_progress: Display progress during the upload and the wait. Defaults to True for
                sync, False for async.

        Returns:
            The upload, with one write per channel in column order. When ``wait`` is True, every
            write is complete.

        Raises:
            ValueError: If the DataFrame or arguments can't be written as processed channels.
            ProcessedChannelUploadError: If ``wait`` is True and any write failed.
            TimeoutError: If ``wait`` is True and the writes don't finish within
                ``timeout_secs``.
            ProcessedChannelRequestError: If the server refuses the upload, for example because
                processed channels aren't enabled for the organization, or a channel name is
                already used by ingested data on the asset.
        """
        if not getattr(self, "_is_sync", False):
            warn_experimental(stacklevel=2)
        if show_progress is None:
            show_progress = self._show_progress()

        asset_id = await self._resolve_asset_id(asset)
        run_id = run._id_or_error if isinstance(run, Run) else run
        if not run_id:
            raise ValueError("'run' is required: every processed channel write targets one run.")
        resolved_code: ProcessedChannelCode
        if isinstance(code, str):
            resolved_code = ProcessedChannelCode(ref=code)
        elif code is None:
            resolved_code = await run_sync_function(default_code)
        else:
            resolved_code = code

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "processed_channels.parquet"
            parquet, footer_offset, footer_length = await run_sync_function(
                lambda: write_processed_channel_parquet(
                    df, path, time_column=time_column, units=units
                )
            )
            upload_url, data_import_id, writes = await self._low_level_client.create(
                asset_id=asset_id,
                run_id=run_id,
                parquet=parquet,
                footer_offset=footer_offset,
                footer_length=footer_length,
                code=resolved_code,
                inputs=inputs,
                metadata=metadata,
            )
            response = await run_sync_function(
                lambda: upload_file(
                    upload_url,
                    path,
                    rest_client=self.client.rest_client,
                    show_progress=show_progress,
                )
            )

        upload = ProcessedChannelUpload(
            data_import_id=data_import_id,
            job_id=response.get("jobId") or None,
            writes=writes,
        )
        upload._apply_client_to_instance(self.client)
        if not wait:
            return upload
        return await self.wait_until_complete(
            upload,
            polling_interval_secs=polling_interval_secs,
            timeout_secs=timeout_secs,
            show_progress=show_progress,
        )

    async def wait_until_complete(
        self,
        upload: ProcessedChannelUpload,
        *,
        polling_interval_secs: float = 2,
        timeout_secs: float | None = None,
        raise_on_failure: bool = True,
        show_progress: bool | None = None,
    ) -> ProcessedChannelUpload:
        """Wait until every write of an upload is complete or failed.

        A write fails when its file is rejected, its import errors, or it isn't uploaded before
        the server's stale threshold.

        Args:
            upload: The upload returned by ``upload(..., wait=False)``.
            polling_interval_secs: Seconds between status polls.
            timeout_secs: Maximum seconds to wait. If None, waits indefinitely.
            raise_on_failure: Raise ``ProcessedChannelUploadError`` if any write failed.
                Defaults to True.
            show_progress: Display a spinner while polling. Defaults to True for sync, False for
                async.

        Returns:
            The upload, with its writes refreshed.

        Raises:
            ProcessedChannelUploadError: If ``raise_on_failure`` is True and any write failed.
            TimeoutError: If the writes don't finish within ``timeout_secs``.
        """
        if show_progress is None:
            show_progress = self._show_progress()
        write_ids = [w.id_ for w in upload.writes]
        query_filter = cel.in_("processed_channel_write_id", write_ids)

        start = time.monotonic()
        with alive_bar(
            title=f"Processed channels ({upload.data_import_id}): polling",
            bar=None,
            spinner_length=7,
            spinner="dots_waves",
            monitor=False,
            stats=False,
            disable=not show_progress,
        ) as bar:
            while True:
                found = {
                    w.id_: w
                    for w in await self._low_level_client.list_all_writes(query_filter=query_filter)
                }
                writes = [found.get(w.id_, w) for w in upload.writes]
                pending = sum(1 for w in writes if not w.status.is_terminal)
                bar.title(
                    f"Processed channels ({upload.data_import_id}): "
                    f"{len(writes) - pending}/{len(writes)} writes finished"
                )
                bar()
                if pending == 0:
                    break
                if timeout_secs is not None and (time.monotonic() - start) >= timeout_secs:
                    raise TimeoutError(
                        f"Processed channel writes for data import {upload.data_import_id} "
                        f"did not finish within {timeout_secs} seconds"
                    )
                await asyncio.sleep(polling_interval_secs)

        finished = ProcessedChannelUpload(
            data_import_id=upload.data_import_id, job_id=upload.job_id, writes=writes
        )
        finished._apply_client_to_instance(self.client)
        if raise_on_failure and finished.failed_writes:
            raise ProcessedChannelUploadError(finished)
        return finished

    async def get_write(self, processed_channel_write_id: str) -> ProcessedChannelWrite:
        """Get a processed channel write by ID.

        Args:
            processed_channel_write_id: The ID of the write.

        Returns:
            The write.
        """
        return await self._low_level_client.get_write(processed_channel_write_id)

    async def list_writes(
        self,
        *,
        processed_channel_write_ids: list[str] | None = None,
        channels: list[Channel | str] | None = None,
        assets: list[Asset | str] | None = None,
        runs: list[Run | str] | None = None,
        status: ProcessedChannelWriteStatus | None = None,
        is_current: bool | None = None,
        fingerprint: str | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[ProcessedChannelWrite]:
        """List processed channel writes, newest first.

        Examples:
            The write each run currently reads for a channel:

                writes = client.processed_channel.list_writes(
                    channels=[channel], is_current=True
                )

        Args:
            processed_channel_write_ids: Filter to these writes.
            channels: Filter to writes of these channels, as ``Channel`` objects or IDs.
            assets: Filter to writes on these assets, as ``Asset`` objects or IDs.
            runs: Filter to writes to these runs, as ``Run`` objects or IDs.
            status: Filter to writes with this status.
            is_current: Filter to writes that are (or aren't) the one reads return for their
                channel and run.
            fingerprint: Filter to writes with this fingerprint.
            filter_query: A CEL filter, ANDed with the other filters. Fields:
                ``processed_channel_write_id``, ``channel_id``, ``asset_id``, ``run_id``,
                ``sequence``, ``fingerprint``, ``canvas_execution_id``, ``report_id``,
                ``status``, ``source_kind`` and ``is_current``.
            order_by: How to order the results. Defaults to ``created_date desc``.
            limit: Maximum number of writes to return.

        Returns:
            The matching writes.
        """
        filter_parts: list[str] = []
        if processed_channel_write_ids:
            filter_parts.append(cel.in_("processed_channel_write_id", processed_channel_write_ids))
        if channels:
            filter_parts.append(cel.in_("channel_id", [_id_of(c) for c in channels]))
        if assets:
            filter_parts.append(cel.in_("asset_id", [_id_of(a) for a in assets]))
        if runs:
            filter_parts.append(cel.in_("run_id", [_id_of(r) for r in runs]))
        if status is not None:
            filter_parts.append(cel.equals("status", status.to_filter_str()))
        if is_current is not None:
            filter_parts.append(cel.equals("is_current", is_current))
        if fingerprint:
            filter_parts.append(cel.equals("fingerprint", fingerprint))
        if filter_query:
            filter_parts.append(cel.parens(filter_query))
        return await self._low_level_client.list_all_writes(
            query_filter=cel.and_(*filter_parts) if filter_parts else None,
            order_by=order_by,
            max_results=limit,
        )

    async def _resolve_asset_id(self, asset: Asset | str) -> str:
        if isinstance(asset, Asset):
            return asset._id_or_error
        if not asset:
            raise ValueError("'asset' is required.")
        if _is_uuid(asset):
            return asset
        found = await self.client.async_.assets.get(name=asset)
        return found._id_or_error


def _id_of(value: Any) -> str:
    return value if isinstance(value, str) else value._id_or_error


def _is_uuid(value: str) -> bool:
    try:
        uuid.UUID(value)
    except ValueError:
        return False
    return True
