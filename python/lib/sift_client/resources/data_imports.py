from __future__ import annotations

import asyncio
import time
from pathlib import Path
from typing import TYPE_CHECKING

from alive_progress import alive_bar  # type: ignore[import-untyped]

import sift_client as _sift_client_module
from sift_client._internal.low_level_wrappers.data_imports import DataImportsLowLevelClient
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.file import upload_file
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.data_import import (
    EXTENSION_TO_DATA_TYPE_KEY,
    CsvImportConfig,
    DataImport,
    DataImportStatus,
    DataTypeKey,
)
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from sift_client._internal.low_level_wrappers.data_imports import ImportConfig
    from sift_client.client import SiftClient


class DataImportAPIAsync(ResourceBase):
    """High-level API for importing data into Sift.

    Supports importing data from local files. Returns a
    `DataImport` object that can be polled for status.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the DataImportAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = DataImportsLowLevelClient(
            grpc_client=self.client.grpc_client,
        )

    async def import_from_path(
        self,
        *,
        file_path: str | Path,
        config: ImportConfig | None = None,
        data_type: DataTypeKey | None = None,
        asset_name: str | None = None,
        run_name: str | None = None,
        run_id: str | None = None,
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
        show_progress: bool | None = None,
    ) -> DataImport:
        """Import data from a local file.

        Creates a data import on the server, uploads the file, and waits
        for the import to complete. Returns the completed :class:`DataImport`.

        When ``config`` is omitted the file format is auto-detected via
        :meth:`detect_config` and a :class:`CsvImportConfig` is built using
        the provided ``asset_name`` and optional ``run_name`` / ``run_id``.

        Args:
            file_path: Path to the local file to import.
            config: Import configuration describing the file format and column
                mapping. When provided, ``asset_name``, ``run_name``,
                ``run_id``, and ``data_type`` are ignored.
            data_type: Explicit data type key. Required for formats like
                Parquet where the extension alone is ambiguous. Only used
                when ``config`` is not provided.
            asset_name: Name of the asset to import into. Required when
                ``config`` is not provided.
            run_name: Optional run name. Only used when ``config`` is not
                provided.
            run_id: Optional existing run ID. Only used when ``config`` is not
                provided.
            polling_interval_secs: Seconds between status polls. Defaults to 5s.
            timeout_secs: Maximum seconds to wait. If None, polls indefinitely.
            show_progress: If True, display a progress spinner while waiting
                for the import to complete. Defaults to True for sync, False
                for async.

        Returns:
            A :class:`DataImport` representing the import operation.

        Raises:
            FileNotFoundError: If the file does not exist.
            ValueError: If neither ``config`` nor ``asset_name`` is provided.
        """
        path = Path(file_path)
        if not path.is_file():
            raise FileNotFoundError(f"File not found: {file_path}")

        if config is None:
            if asset_name is None:
                raise ValueError("Either 'config' or 'asset_name' must be provided.")
            detected = await self.detect_config(file_path, data_type=data_type)
            config = detected.model_copy(
                update={
                    "asset_name": asset_name,
                    "run_name": run_name if run_name or run_id else path.stem,
                    "run_id": run_id,
                }
            )
        data_import_id, upload_url = await self._low_level_client.create_from_upload(config)

        await run_sync_function(
            lambda: upload_file(upload_url, path, rest_client=self.client.rest_client)
        )
        # job_id = response["job_id"]

        return await self.wait_until_complete(
            data_import_id,
            polling_interval_secs=polling_interval_secs,
            timeout_secs=timeout_secs,
            show_progress=show_progress,
        )

    async def get(self, data_import_id: str) -> DataImport:
        """Get a data import by ID.

        Args:
            data_import_id: The ID of the data import.

        Returns:
            The DataImport.
        """
        data_import = await self._low_level_client.get(data_import_id)
        return self._apply_client_to_instance(data_import)

    async def list_(
        self,
        *,
        data_import_ids: list[str] | None = None,
        status: DataImportStatus | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[DataImport]:
        """List data imports with optional filtering.

        Args:
            data_import_ids: Filter to imports with any of these IDs.
            status: Filter to imports with this status.
            filter_query: Explicit CEL filter string.
            order_by: Ordering string (e.g. "created_date desc").
            limit: Maximum number of imports to return. If None, returns all.

        Returns:
            A list of DataImport objects matching the filter criteria.
        """
        filter_parts = []
        if data_import_ids:
            filter_parts.append(cel.in_("data_import_id", data_import_ids))
        if status is not None:
            filter_parts.append(cel.equals("status", str(status.value)))
        if filter_query:
            filter_parts.append(filter_query)
        query_filter = cel.and_(*filter_parts)

        data_imports = await self._low_level_client.list_all(
            query_filter=query_filter or "",
            order_by=order_by or "",
            max_results=limit,
        )
        return self._apply_client_to_instances(data_imports)

    async def retry(self, data_import: str | DataImport) -> None:
        """Retry a failed data import.

        Args:
            data_import: The DataImport or data_import_id to retry.
        """
        data_import_id = (
            data_import._id_or_error if isinstance(data_import, DataImport) else data_import
        )
        await self._low_level_client.retry(data_import_id)

    async def detect_config(
        self,
        file_path: str | Path,
        data_type: DataTypeKey | None = None,
    ) -> ImportConfig:
        """Auto-detect import configuration from a file.

        Reads a sample of the file, sends it to the server's DetectConfig
        endpoint, and returns the detected configuration. The file format
        is inferred from the file extension when ``data_type`` is not
        provided.

        For file types with multiple layouts (e.g. Parquet), ``data_type``
        must be specified explicitly.

        Args:
            file_path: Path to the file to analyze.
            data_type: Explicit data type key. Required for formats like
                Parquet where the extension alone is ambiguous.

        Returns:
            The detected import config.

        Raises:
            FileNotFoundError: If the file does not exist.
            ValueError: If the file extension is unsupported or detection
                returns no config.
        """
        path = Path(file_path)
        if not path.is_file():
            raise FileNotFoundError(f"File not found: {file_path}")

        if data_type is not None:
            data_type_key = data_type
        else:
            ext = path.suffix.lower()
            data_type_key = EXTENSION_TO_DATA_TYPE_KEY.get(ext)
            if data_type_key is None:
                raise ValueError(
                    f"Unsupported file extension '{ext}'. "
                    f"Supported: {', '.join(sorted(EXTENSION_TO_DATA_TYPE_KEY))}. "
                    f"For other formats (e.g. Parquet), pass 'data_type' explicitly."
                )

        def _read_sample() -> bytes:
            with open(path, "rb") as f:
                return f.read(65_536)  # 64 KiB

        sample = await run_sync_function(_read_sample)

        response = await self._low_level_client.detect_config(sample, data_type_key.value)

        if response.HasField("csv_config"):
            config = CsvImportConfig._from_proto(response.csv_config)
            # The server's DetectConfig may include the time column in
            # data_columns, but CreateDataImportFromUpload rejects that
            # overlap. Filter it out so the config is import-ready.
            time_col = config.time_column.column
            filtered = [dc for dc in config.data_columns if dc.column != time_col]
            if len(filtered) != len(config.data_columns):
                config = config.model_copy(update={"data_columns": filtered})
            return config

        # TODO: Add other file format configs

        raise ValueError("Server returned an empty DetectConfig response.")

    async def wait_until_complete(
        self,
        data_import: str | DataImport,
        *,
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
        show_progress: bool | None = None,
    ) -> DataImport:
        """Wait until a data import reaches a terminal state.

        Polls the import status at the given interval until the import is
        SUCCEEDED or FAILED, returning the completed DataImport.

        Args:
            data_import: The DataImport or data_import_id to wait for.
            polling_interval_secs: Seconds between status polls. Defaults to 5s.
            timeout_secs: Maximum seconds to wait. If None, polls indefinitely.
                Defaults to None (indefinite).
            show_progress: If True, display an animated progress spinner alongside
                the import status while polling. Defaults to True for sync, False
                for async. Use ``sift_client.config.show_progress = False`` to disable
                globally for sync.

        Returns:
            The DataImport in its terminal state.
        """
        data_import_id = (
            data_import._id_or_error if isinstance(data_import, DataImport) else data_import
        )
        if show_progress is None:
            global_setting = _sift_client_module.config.show_progress
            if global_setting is not None:
                show_progress = global_setting
            elif getattr(self, "_is_sync", False):
                show_progress = True
            else:
                show_progress = False

        start = time.monotonic()
        with alive_bar(
            title=f"Data Import ID {data_import_id}: polling",
            bar=None,
            spinner_length=7,
            spinner="dots_waves",
            monitor=False,
            stats=False,
            disable=not show_progress,
        ) as bar:
            while True:
                result = await self.get(data_import_id)
                bar.title(f"Data Import ID {data_import_id}: {result.status.name}")
                bar()
                if result.is_complete:
                    return result
                if timeout_secs is not None and (time.monotonic() - start) >= timeout_secs:
                    raise TimeoutError(
                        f"Data import '{data_import_id}' did not complete "
                        f"within {timeout_secs} seconds."
                    )
                await asyncio.sleep(polling_interval_secs)
