from __future__ import annotations

import asyncio
import logging
import time
from pathlib import Path
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.data_imports import DataImportsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.data_import import (
    EXTENSION_TO_DATA_TYPE_KEY,
    CsvImportConfig,
    DataImport,
    DataImportStatus,
)
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from sift_client._internal.low_level_wrappers.data_imports import ImportConfig
    from sift_client.client import SiftClient

logger = logging.getLogger(__name__)

_DETECT_CONFIG_SAMPLE_SIZE = 65_536  # 64 KiB


class DataImportAPIAsync(ResourceBase):
    """High-level API for importing data into Sift.

    Supports importing data from local files or remote URLs. Returns a
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
            rest_client=self.client.rest_client,
        )

    async def import_from_path(
        self,
        *,
        file_path: str | Path,
        config: ImportConfig | None = None,
        asset_name: str | None = None,
        run_name: str | None = None,
        run_id: str | None = None,
    ) -> DataImport:
        """Import data from a local file.

        Creates a data import on the server and uploads the file to the
        returned presigned URL. Returns a :class:`DataImport` that can be
        polled for status via ``data_import.refresh()``.

        When ``config`` is omitted the file format is auto-detected via
        :meth:`detect_config` and a :class:`CsvImportConfig` is built using
        the provided ``asset_name`` and optional ``run_name`` / ``run_id``.

        Args:
            file_path: Path to the local file to import.
            config: Import configuration describing the file format and column
                mapping. When provided, ``asset_name``, ``run_name``, and
                ``run_id`` are ignored.
            asset_name: Name of the asset to import into. Required when
                ``config`` is not provided.
            run_name: Optional run name. Only used when ``config`` is not
                provided.
            run_id: Optional existing run ID. Only used when ``config`` is not
                provided.

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
                raise ValueError(
                    "Either 'config' or 'asset_name' must be provided."
                )
            detected = await self.detect_config(file_path)
            config = detected.model_copy(
                update={
                    "asset_name": asset_name,
                    "run_name": run_name,
                    "run_id": run_id,
                }
            )

        data_import_id, upload_url = await self._low_level_client.create_from_upload(config)
        logger.info("Created data import %s", data_import_id)

        await self._low_level_client.upload_file(upload_url, path)
        logger.info("Uploaded file to presigned URL for import %s", data_import_id)

        data_import = await self._low_level_client.get(data_import_id)
        return self._apply_client_to_instance(data_import)

    async def import_from_url(
        self,
        *,
        url: str,
        config: ImportConfig,
    ) -> DataImport:
        """Import data from a remote URL (HTTP or S3).

        Returns a :class:`DataImport` that can be polled for status via
        ``data_import.refresh()``.

        Args:
            url: The URL to import from.
            config: Import configuration describing the file format and column
                mapping.

        Returns:
            A :class:`DataImport` representing the import operation.
        """
        data_import_id = await self._low_level_client.create_from_url(url, config)
        logger.info("Created URL-based data import %s", data_import_id)

        data_import = await self._low_level_client.get(data_import_id)
        return self._apply_client_to_instance(data_import)

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

        Only works for URL-based imports in a failed state.

        Args:
            data_import: The DataImport or data_import_id to retry.
        """
        data_import_id = (
            data_import._id_or_error if isinstance(data_import, DataImport) else data_import
        )
        await self._low_level_client.retry(data_import_id)

    async def detect_config(self, file_path: str | Path) -> ImportConfig:
        """Auto-detect import configuration from a file.

        Reads a sample of the file, sends it to the server's DetectConfig
        endpoint, and returns the detected configuration. The file format
        is inferred from the file extension. You can inspect and modify the
        result before passing it to :meth:`import_from_path`.

        Supported extensions: .csv, .parquet, .tdms, .ch10, .ch11, .h5, .hdf5

        Args:
            file_path: Path to the file to analyze.

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

        ext = path.suffix.lower()
        data_type_key = EXTENSION_TO_DATA_TYPE_KEY.get(ext)
        if data_type_key is None:
            raise ValueError(
                f"Unsupported file extension '{ext}'. "
                f"Supported: {', '.join(sorted(EXTENSION_TO_DATA_TYPE_KEY))}"
            )

        with open(path, "rb") as f:
            sample = f.read(_DETECT_CONFIG_SAMPLE_SIZE)

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

        # TODO: Add parquet_config and hdf5_config once their config types are added.

        raise ValueError("Server returned an empty DetectConfig response.")

    async def wait_until_complete(
        self,
        data_import: str | DataImport,
        *,
        polling_interval_secs: int = 5,
        timeout_secs: int | None = None,
    ) -> DataImport:
        """Wait until a data import reaches a terminal state.

        Polls the import status at the given interval until the import is
        SUCCEEDED or FAILED, returning the completed DataImport.

        Args:
            data_import: The DataImport or data_import_id to wait for.
            polling_interval_secs: Seconds between status polls. Defaults to 5s.
            timeout_secs: Maximum seconds to wait. If None, polls indefinitely.
                Defaults to None (indefinite).

        Returns:
            The DataImport in its terminal state.
        """
        data_import_id = (
            data_import._id_or_error if isinstance(data_import, DataImport) else data_import
        )

        start = time.monotonic()
        while True:
            result = await self.get(data_import_id)
            if result.is_complete:
                return result
            if timeout_secs is not None and (time.monotonic() - start) >= timeout_secs:
                raise TimeoutError(
                    f"Data import '{data_import_id}' did not complete "
                    f"within {timeout_secs} seconds."
                )
            await asyncio.sleep(polling_interval_secs)
