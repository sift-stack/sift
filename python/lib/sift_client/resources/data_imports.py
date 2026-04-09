from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.data_imports import DataImportsLowLevelClient
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.file import (
    extract_parquet_footer,
    resolve_show_progress,
    upload_file,
)
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
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
from sift_client.sift_types.run import Run

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
        asset: Asset | str | None = None,
        config: ImportConfig | None = None,
        data_type: DataTypeKey | None = None,
        run: Run | str | None = None,
        run_name: str | None = None,
        show_progress: bool | None = None,
    ) -> Job:
        """Import data from a local file.

        Creates a data import on the server, uploads the file, and returns
        a ``Job`` handle after uploading the file. The import processes
        server-side and typically completes shortly after upload. Use
        ``job.wait_until_complete()`` only if you need to confirm
        completion before proceeding.

        When ``config`` is omitted the file format is auto-detected via
        ``detect_config`` (CSV and Parquet only). For other formats
        (TDMS, HDF5, CH10), ``config`` must be provided.
        When ``asset`` is provided it overrides the config value;
        otherwise the config's ``asset_name`` is used.
        If neither ``run`` nor ``run_name`` is provided (and none is
        set on the config), ``run_name`` defaults to the filename.

        Examples:
            Import a CSV file with auto-detected config:

                job = client.data_imports.import_from_path(
                    "data.csv",
                    asset=my_asset,
                )

            Auto-detect config, inspect and patch before importing:

                config = client.data_imports.detect_config("data.csv")

                # Fix a column data type
                config["temperature"].data_type = ChannelDataType.FLOAT

                # Remove an unwanted column
                config.data_columns = [
                    dc for dc in config.data_columns if dc.name != "internal_id"
                ]

                job = client.data_imports.import_from_path(
                    "data.csv",
                    asset=my_asset,
                    config=config,
                )

        Args:
            file_path: Path to the local file to import.
            asset: Asset object or asset name to import data into. Optional
                when ``config`` already has ``asset_name`` set.
            config: Import configuration describing the file format and column
                mapping. When provided, ``data_type`` is ignored. If omitted,
                the config is auto-detected via ``detect_config``. You can
                call ``detect_config`` yourself to inspect and modify the
                config before passing it here.
            data_type: Explicit data type key. Required for formats like
                Parquet where the extension alone is ambiguous. Only used
                when ``config`` is not provided.
            run: ``Run`` object or run ID string to import into an existing
                run. Mutually exclusive with ``run_name``.
            run_name: Name for a new run. Defaults to the filename if
                neither ``run`` nor ``run_name`` is set.
            show_progress: If True, display a progress spinner during upload.
                Defaults to True for sync, False for async.

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

        if asset is not None:
            config.asset_name = asset.name if isinstance(asset, Asset) else asset
        elif not config.asset_name:
            raise ValueError("'asset' is required when not set on the config.")
        if run is not None and run_name is not None:
            raise ValueError("'run' and 'run_name' are mutually exclusive.")
        if run is not None:
            if isinstance(config, Ch10ImportConfig):
                raise ValueError(
                    "'run' is not supported for Ch10ImportConfig. Use 'run_name' instead."
                )
            config.run_id = run._id_or_error if isinstance(run, Run) else run
        elif run_name is not None:
            config.run_name = run_name
        elif not config.run_name and (isinstance(config, Ch10ImportConfig) or not config.run_id):
            config.run_name = path.name

        if isinstance(
            config, (ParquetFlatDatasetImportConfig, ParquetSingleChannelPerRowImportConfig)
        ):
            await _prepare_parquet_config(config, path)

        if show_progress is None:
            show_progress = resolve_show_progress(is_sync=getattr(self, "_is_sync", False))

        _, upload_url = await self._low_level_client.create_from_upload(config)

        response = await run_sync_function(
            lambda: upload_file(
                upload_url,
                path,
                rest_client=self.client.rest_client,
                show_progress=show_progress,
            )
        )
        job_id = response.get("jobId")
        if not job_id:
            raise ValueError("Upload succeeded but server response did not include a job ID.")

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

        For CSV files, the server scans the first two rows for an optional
        JSON metadata row. Row 1 is checked first; row 2 is checked only
        if row 1 is not valid metadata. A row qualifies as metadata when
        every cell contains valid JSON that describes either a time column
        or a data column. When present, ``first_data_row`` in the returned
        config is set to the row after the metadata row.

        Each data column cell is a JSON ``ChannelConfig``::

            {"name": "speed", "units": "m/s", "dataType": "CHANNEL_DATA_TYPE_DOUBLE"}

        The time column cell is a JSON ``CsvTimeColumn``::

            {"format": "TIME_FORMAT_ABSOLUTE_RFC3339"}

        Enum type definitions and bit field elements can also be specified
        in the metadata row; they are applied server-side during import
        but are not included in the returned config.

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
            ValueError: If the file extension is unsupported or no
                supported configuration could be detected.
        """
        path = Path(file_path)
        if not path.is_file():
            raise FileNotFoundError(f"File not found: {file_path}")

        data_type_key = _resolve_data_type_key(path.suffix.lower(), data_type)

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
            return _parse_csv_detect_response(response.csv_config, path.name)

        if response.HasField("parquet_config"):
            return _parse_parquet_detect_response(
                response.parquet_config, path.name, footer_offset, footer_length
            )

        raise ValueError(
            f"No supported configuration detected for '{path.name}'. "
            "Auto-detection supports CSV and Parquet files. "
            "For other formats, provide a config manually."
        )


def _resolve_data_type_key(ext: str, data_type: DataTypeKey | None) -> DataTypeKey:
    """Resolve the data type key from file extension and explicit override."""
    if ext in (".parquet", ".pqt"):
        if data_type is None:
            raise ValueError(
                "Parquet files require 'data_type' to be specified. "
                "Use DataTypeKey.PARQUET_FLATDATASET or DataTypeKey.PARQUET_SINGLE_CHANNEL_PER_ROW."
            )
        return data_type
    if data_type is not None:
        return data_type
    if ext not in EXTENSION_TO_DATA_TYPE_KEY:
        raise ValueError(
            f"Unsupported file extension '{ext}'. "
            f"Supported: {', '.join(sorted(EXTENSION_TO_DATA_TYPE_KEY))}. "
            "You can also specify 'data_type' explicitly using a DataTypeKey value."
        )
    return EXTENSION_TO_DATA_TYPE_KEY[ext]


def _parse_csv_detect_response(proto, filename: str) -> CsvImportConfig:
    """Parse a CSV DetectConfig response into a config."""
    csv_config = CsvImportConfig._from_proto(proto)
    time_col = csv_config.time_column.column
    csv_config.data_columns = [dc for dc in csv_config.data_columns if dc.column != time_col]
    if not csv_config.data_columns:
        raise ValueError(f"No data columns detected in '{filename}'.")
    return csv_config


def _parse_parquet_detect_response(
    proto, filename: str, footer_offset: int, footer_length: int
) -> ParquetFlatDatasetImportConfig | ParquetSingleChannelPerRowImportConfig:
    """Parse a Parquet DetectConfig response into a config."""
    if proto.HasField("flat_dataset"):
        parquet_config = ParquetFlatDatasetImportConfig._from_proto(
            proto, footer_offset=footer_offset, footer_length=footer_length
        )
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
                f"No time column detected in '{filename}'. "
                "Set the time column manually on the config before importing."
            )
        if not parquet_config.data_columns:
            raise ValueError(f"No data columns detected in '{filename}'.")
        return parquet_config
    elif proto.HasField("single_channel_per_row"):
        return ParquetSingleChannelPerRowImportConfig._from_proto(
            proto, footer_offset=footer_offset, footer_length=footer_length
        )
    raise ValueError(f"Unsupported parquet layout in DetectConfig response for '{filename}'.")


async def _prepare_parquet_config(
    config: ParquetFlatDatasetImportConfig | ParquetSingleChannelPerRowImportConfig,
    path: Path,
) -> None:
    """Populate parquet footer fields on the config if not already set."""
    if config.footer_offset == 0 and config.footer_length == 0:
        footer_bytes, footer_offset = await run_sync_function(lambda: extract_parquet_footer(path))
        config.footer_offset = footer_offset
        config.footer_length = len(footer_bytes)
