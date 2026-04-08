from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.data_imports import DataImportsLowLevelClient
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.file import extract_parquet_footer, upload_file
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    EXTENSION_TO_DATA_TYPE_KEY,
    Ch10ImportConfig,
    CsvImportConfig,
    DataTypeKey,
    ImportConfig,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelPerRowImportConfig,
    ParquetTimeColumn,
)

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.job import Job


class DataImportAPIAsync(ResourceBase):
    """High-level API for importing data into Sift."""

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
        file_path: str | Path,
        *,
        asset_name: str | None = None,
        config: ImportConfig | None = None,
        data_type: DataTypeKey | None = None,
        run_name: str | None = None,
        run_id: str | None = None,
    ) -> Job:
        """Import data from a local file.

        Creates a data import on the server, uploads the file, and returns
        a ``Job`` handle. Use ``job.wait_until_complete()`` to poll for
        completion if needed.

        When ``config`` is omitted the file format is auto-detected via
        ``detect_config`` (CSV and Parquet only). For other formats
        (TDMS, HDF5, CH10), ``config`` must be provided.
        When ``asset_name`` is provided it overrides
        the config value; otherwise the config's ``asset_name`` is used.
        If neither ``run_name`` nor ``run_id`` is provided
        (and none is set on the config), ``run_name`` defaults to the
        filename.

        Args:
            file_path: Path to the local file to import.
            asset_name: Name of the asset to import data into. Optional
                when ``config`` already has ``asset_name`` set.
            config: Import configuration describing the file format and column
                mapping. When provided, ``data_type`` is ignored.
            data_type: Explicit data type key. Required for formats like
                Parquet where the extension alone is ambiguous. Only used
                when ``config`` is not provided.
            run_name: Run name to use. Overrides any value on the config.
                Defaults to the filename if neither ``run_name`` nor
                ``run_id`` is set.
            run_id: Existing run ID to use. Overrides any value on the config.

        Returns:
            A ``Job`` handle for the pending import.

        Raises:
            FileNotFoundError: If the file does not exist.
        """
        path = Path(file_path)
        if not path.is_file():
            raise FileNotFoundError(f"File not found: {file_path}")

        if config is None:
            config = await self.detect_config(file_path, data_type=data_type)

        if asset_name is not None:
            config.asset_name = asset_name
        elif not config.asset_name:
            raise ValueError("'asset_name' is required when not set on the config.")
        if run_id is not None:
            if isinstance(config, Ch10ImportConfig):
                raise ValueError(
                    "'run_id' is not supported for Ch10ImportConfig. Use 'run_name' instead."
                )
            config.run_id = run_id
        elif run_name is not None:
            config.run_name = run_name
        elif not config.run_name and (isinstance(config, Ch10ImportConfig) or not config.run_id):
            config.run_name = path.name

        if isinstance(
            config, (ParquetFlatDatasetImportConfig, ParquetSingleChannelPerRowImportConfig)
        ):
            if config.footer_offset == 0 and config.footer_length == 0:
                footer_bytes, footer_offset = await run_sync_function(
                    lambda: extract_parquet_footer(path)
                )
                config.footer_offset = footer_offset
                config.footer_length = len(footer_bytes)

        _, upload_url = await self._low_level_client.create_from_upload(config)

        response = await run_sync_function(
            lambda: upload_file(upload_url, path, rest_client=self.client.rest_client)
        )
        job_id = response["jobId"]

        return await self.client.async_.jobs.get(job_id=job_id)

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

        Only CSV and Parquet files are currently supported for auto-detection.
        For other formats (TDMS, HDF5, CH10), create the config manually
        using ``TdmsImportConfig``, ``Hdf5ImportConfig``, or
        ``Ch10ImportConfig``.

        For CSV files, the server can parse an optional JSON metadata row
        that auto-populates channel names, units, descriptions, data types,
        and enum definitions. Each cell in the row is a JSON object
        describing that column. When present, ``first_data_row`` in the
        returned config will be set to the row after the metadata row.
        Note that enum type definitions are applied server-side during
        import but are not included in the returned config.

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

        ext = path.suffix.lower()
        if ext in (".parquet", ".pqt"):
            if data_type is None:
                raise ValueError(
                    "Parquet files require 'data_type' to be specified. "
                    "Use DataTypeKey.PARQUET_FLATDATASET or DataTypeKey.PARQUET_SINGLE_CHANNEL_PER_ROW."
                )
            data_type_key = data_type
        elif data_type is not None:
            data_type_key = data_type
        else:
            if ext not in EXTENSION_TO_DATA_TYPE_KEY:
                raise ValueError(
                    f"Unsupported file extension '{ext}'. "
                    f"Supported: {', '.join(sorted(EXTENSION_TO_DATA_TYPE_KEY))}"
                )
            data_type_key = EXTENSION_TO_DATA_TYPE_KEY[ext]

        is_parquet = data_type_key in (
            DataTypeKey.PARQUET_FLATDATASET,
            DataTypeKey.PARQUET_SINGLE_CHANNEL_PER_ROW,
        )

        footer_offset = 0
        footer_length = 0

        if is_parquet:
            footer_bytes, footer_offset = await run_sync_function(
                lambda: extract_parquet_footer(path)
            )
            sample = footer_bytes
            footer_length = len(footer_bytes)
        else:

            def _read_sample() -> bytes:
                with open(path, "rb") as f:
                    return f.read(1048576)  # 1MiB

            sample = await run_sync_function(_read_sample)

        response = await self._low_level_client.detect_config(sample, data_type_key.value)

        if response.HasField("csv_config"):
            csv_config = CsvImportConfig._from_proto(response.csv_config)
            # Filter out the time column from data_columns to avoid overlap.
            time_col = csv_config.time_column.column
            csv_config.data_columns = [
                dc for dc in csv_config.data_columns if dc.column != time_col
            ]
            if not csv_config.data_columns:
                raise ValueError(f"No data columns detected in '{path.name}'.")
            return csv_config

        if response.HasField("parquet_config"):
            proto = response.parquet_config
            if proto.HasField("flat_dataset"):
                parquet_config = ParquetFlatDatasetImportConfig._from_proto(
                    proto, footer_offset=footer_offset, footer_length=footer_length
                )
                # Filter out the time column from data_columns to avoid overlap.
                time_path = parquet_config.time_column.path
                if time_path:
                    parquet_config.data_columns = [
                        dc for dc in parquet_config.data_columns if dc.path != time_path
                    ]
                else:
                    # The backend only detects arrow timestamp types. Fall back to
                    # an integer column whose name starts with "time".
                    _integer_types = {
                        ChannelDataType.INT_32,
                        ChannelDataType.INT_64,
                        ChannelDataType.UINT_32,
                        ChannelDataType.UINT_64,
                    }
                    match = None
                    for dc in parquet_config.data_columns:
                        if dc.data_type in _integer_types and dc.name.lower().startswith("time"):
                            match = dc
                            break
                    if match is not None:
                        parquet_config.time_column = ParquetTimeColumn(path=match.path)
                        parquet_config.data_columns = [
                            c for c in parquet_config.data_columns if c.path != match.path
                        ]
                if not parquet_config.time_column.path:
                    raise ValueError(
                        f"No time column detected in '{path.name}'. "
                        "Set the time column manually on the config before importing."
                    )
                if not parquet_config.data_columns:
                    raise ValueError(f"No data columns detected in '{path.name}'.")
                return parquet_config
            elif proto.HasField("single_channel_per_row"):
                return ParquetSingleChannelPerRowImportConfig._from_proto(
                    proto, footer_offset=footer_offset, footer_length=footer_length
                )

        raise ValueError("Server returned an empty DetectConfig response.")
