from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.data_imports import DataImportsLowLevelClient
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.file import extract_parquet_footer, upload_file
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    DATA_TYPE_KEY_TO_PROTO,
    EXTENSION_TO_DATA_TYPE_KEY,
    CsvImportConfig,
    DataTypeKey,
    Hdf5ImportConfig,
    ImportConfig,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelPerRowImportConfig,
    ParquetTimeColumn,
    TdmsImportConfig,
    TimeFormat,
    UlogImportConfig,
)
from sift_client.sift_types.run import Run

if TYPE_CHECKING:
    from collections.abc import Iterable

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
        time_format: TimeFormat | None = None,
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
        ``detect_config`` (CSV, Parquet, HDF5, TDMS, and ULog).
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
            data_type: Explicit data type key. Required for formats with
                multiple supported layouts (Parquet, HDF5) where the file
                extension alone is ambiguous. Only used when ``config`` is
                not provided.
            time_format: Time format override. When provided, takes
                precedence over the format returned by detection. When
                omitted, the returned config uses the detected format if
                available, falling back to
                ``TimeFormat.ABSOLUTE_UNIX_NANOSECONDS``. Only used when
                ``config`` is not provided.
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
            config = await self.detect_config(
                file_path,
                data_type=data_type,
                time_format=time_format,
            )

        if asset is not None:
            config.asset_name = asset.name if isinstance(asset, Asset) else asset
        elif not config.asset_name:
            raise ValueError("'asset' is required when not set on the config.")
        if run is not None and run_name is not None:
            raise ValueError("'run' and 'run_name' are mutually exclusive.")
        if run is not None:
            config.run_id = run._id_or_error if isinstance(run, Run) else run
        elif run_name is not None:
            config.run_name = run_name
        elif not config.run_name and not config.run_id:
            config.run_name = path.name

        if isinstance(
            config, (ParquetFlatDatasetImportConfig, ParquetSingleChannelPerRowImportConfig)
        ):
            await _prepare_parquet_config(config, path)

        if show_progress is None:
            show_progress = self._show_progress()

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
            raise RuntimeError("Upload succeeded but server response did not include a job ID.")

        return await self.client.async_.jobs.get(job_id=job_id)

    async def get_run(self, data_import_id: str) -> Run:
        """Get the run associated with a data import.

        The ``data_import_id`` is available on the job returned by
        ``import_from_path`` via ``job.job_details.data_import_id``.
        For a more ergonomic approach, use ``job.get_import_run()``
        which calls this method internally.

        Args:
            data_import_id: The ID of the data import.

        Returns:
            The Run created by or associated with the import.

        Raises:
            ValueError: If the data import has no associated run.
        """
        response = await self._low_level_client.get(data_import_id)
        run_id = response.data_import.run_id
        if not run_id:
            raise ValueError("Data import does not have an associated run.")
        return await self.client.async_.runs.get(run_id=run_id)

    async def detect_config(
        self,
        file_path: str | Path,
        data_type: DataTypeKey | None = None,
        time_format: TimeFormat | None = None,
    ) -> ImportConfig:
        """Auto-detect import configuration from a file.

        Returns the detected configuration, inferring the file format from the
        extension when ``data_type`` is not provided. CSV and Parquet are
        detected by sending a sample of the file to the server's DetectConfig
        endpoint; TDMS, HDF5, and ULog are detected locally on the client.

        CSV, Parquet, HDF5, TDMS, and ULog files are supported for
        auto-detection.

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

        For file types with multiple supported layouts (Parquet, HDF5),
        ``data_type`` must be specified explicitly.

        Args:
            file_path: Path to the file to analyze.
            data_type: Explicit data type key. Required for formats with
                multiple supported layouts (Parquet, HDF5) where the file
                extension alone is ambiguous.
            time_format: Time format override. When provided, takes
                precedence over the format returned by detection. When
                omitted, the returned config uses the detected format if
                available, falling back to
                ``TimeFormat.ABSOLUTE_UNIX_NANOSECONDS``.

        Returns:
            The detected import config.

        Raises:
            FileNotFoundError: If the file does not exist.
            ValueError: If the file extension is unsupported, no supported
                configuration could be detected, or ``data_type`` was
                omitted for a file format that requires a variant.
        """
        path = Path(file_path)
        if not path.is_file():
            raise FileNotFoundError(f"File not found: {file_path}")

        data_type_key = _resolve_data_type_key(path.suffix.lower(), data_type)
        config = await self._detect_config_for_type(path, data_type_key)
        if time_format is not None:
            _apply_time_format(config, time_format)
        elif (
            not isinstance(config, (TdmsImportConfig, UlogImportConfig))
            and _get_time_format(config) is None
        ):
            _apply_time_format(config, TimeFormat.ABSOLUTE_UNIX_NANOSECONDS)
        return config

    async def _detect_config_for_type(
        self,
        path: Path,
        data_type_key: DataTypeKey,
    ) -> ImportConfig:
        if data_type_key in (
            DataTypeKey.HDF5_ONE_D,
            DataTypeKey.HDF5_TWO_D,
            DataTypeKey.HDF5_COMPOUND,
        ):
            try:
                from sift_client._internal.util.hdf5 import detect_hdf5_config
            except ImportError as e:
                raise RuntimeError(
                    "h5py is required for HDF5 import. "
                    "Install it via `pip install sift-stack-py[hdf5]`."
                ) from e
            return await run_sync_function(lambda: detect_hdf5_config(path, data_type_key))
        if data_type_key == DataTypeKey.TDMS:
            try:
                from sift_client._internal.util.tdms import detect_tdms_config
            except ImportError as e:
                raise RuntimeError(
                    "npTDMS is required for TDMS import. "
                    "Install it via `pip install sift-stack-py[tdms]`."
                ) from e
            return await run_sync_function(lambda: detect_tdms_config(path))
        if data_type_key == DataTypeKey.ULOG:
            try:
                from sift_client._internal.util.ulog import detect_ulog_config
            except ImportError as e:
                raise RuntimeError(
                    "pyulog is required for ULog import. "
                    "Install it via `pip install sift-stack-py[ulog]`."
                ) from e
            return await run_sync_function(lambda: detect_ulog_config(path))

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

        response = await self._low_level_client.detect_config(
            sample, DATA_TYPE_KEY_TO_PROTO[data_type_key]
        )

        if response.HasField("csv_config"):
            return _parse_csv_detect_response(response.csv_config)

        if response.HasField("parquet_config"):
            return _parse_parquet_detect_response(
                response.parquet_config, path.name, footer_offset, footer_length
            )

        raise ValueError(
            f"No supported configuration detected for '{path.name}'. "
            "Only CSV, Parquet, HDF5, TDMS, and ULog are supported by auto-detection."
        )


def _apply_time_format(config: ImportConfig, time_format: TimeFormat) -> None:
    """Set the time format on a detected config, dispatching by config type.
    CSV and Parquet store the format under ``time_column.format``; TDMS and
    HDF5 store it on ``time_format`` directly.
    """
    if isinstance(
        config,
        (CsvImportConfig, ParquetFlatDatasetImportConfig, ParquetSingleChannelPerRowImportConfig),
    ):
        config.time_column.format = time_format
    elif isinstance(config, (TdmsImportConfig, Hdf5ImportConfig)):
        config.time_format = time_format


def _get_time_format(config: ImportConfig) -> TimeFormat | None:
    """Read the current time format off a config, regardless of where it's stored."""
    if isinstance(
        config,
        (CsvImportConfig, ParquetFlatDatasetImportConfig, ParquetSingleChannelPerRowImportConfig),
    ):
        return config.time_column.format
    if isinstance(config, (TdmsImportConfig, Hdf5ImportConfig)):
        return config.time_format
    return None


def _resolve_data_type_key(ext: str, data_type: DataTypeKey | None) -> DataTypeKey:
    """Resolve the data type key from file extension and explicit override."""
    if data_type is not None:
        return data_type
    if ext in (".parquet", ".pqt"):
        raise ValueError(
            "Parquet files require 'data_type' to be specified. "
            "Use DataTypeKey.PARQUET_FLATDATASET or DataTypeKey.PARQUET_SINGLE_CHANNEL_PER_ROW."
        )
    if ext in (".h5", ".hdf5"):
        raise ValueError(
            "HDF5 files require 'data_type' to be specified. "
            "Use DataTypeKey.HDF5_ONE_D, DataTypeKey.HDF5_TWO_D, "
            "or DataTypeKey.HDF5_COMPOUND."
        )
    if ext not in EXTENSION_TO_DATA_TYPE_KEY:
        raise ValueError(
            f"Unsupported file extension '{ext}'. "
            f"Supported: {', '.join(sorted(EXTENSION_TO_DATA_TYPE_KEY))}. "
            "You can also specify 'data_type' explicitly using a DataTypeKey value."
        )
    return EXTENSION_TO_DATA_TYPE_KEY[ext]


def _parse_csv_detect_response(proto) -> CsvImportConfig:
    """Parse a CSV DetectConfig response into a config."""
    csv_config = CsvImportConfig._from_proto(proto)
    time_col = csv_config.time_column.column
    csv_config.data_columns = [dc for dc in csv_config.data_columns if dc.column != time_col]
    return csv_config


_TIME_COLUMN_NAMES: frozenset[str] = frozenset({"ts", "timestamp", "time"})
_TIME_COLUMN_TYPES: frozenset[ChannelDataType] = frozenset(
    {ChannelDataType.INT_64, ChannelDataType.UINT_64}
)


def _infer_time_column(
    columns: Iterable[tuple[str, ChannelDataType, str]],
) -> str | None:
    """Pick a likely time column when the server couldn't identify one.

    Returns the path of an INT64 or UINT64 column whose name
    (case-insensitive) matches one of ``ts``, ``timestamp``, or ``time``.
    Returns None otherwise.
    """
    data_columns = sorted(columns, key=lambda c: c[0].lower())
    for name, data_type, path in data_columns:
        if data_type in _TIME_COLUMN_TYPES and name.lower() in _TIME_COLUMN_NAMES:
            return path
    return None


def _parse_parquet_detect_response(
    proto, filename: str, footer_offset: int, footer_length: int
) -> ParquetFlatDatasetImportConfig | ParquetSingleChannelPerRowImportConfig:
    """Parse a Parquet DetectConfig response into a config."""
    if proto.HasField("flat_dataset"):
        parquet_config = ParquetFlatDatasetImportConfig._from_proto(
            proto, footer_offset=footer_offset, footer_length=footer_length
        )
        time_path: str | None = parquet_config.time_column.path
        if not time_path:
            time_path = _infer_time_column(
                (dc.name, dc.data_type, dc.path) for dc in parquet_config.data_columns
            )
            if time_path:
                parquet_config.time_column = ParquetTimeColumn(path=time_path)
        if time_path:
            parquet_config.data_columns = [
                dc for dc in parquet_config.data_columns if dc.path != time_path
            ]
        return parquet_config
    elif proto.HasField("single_channel_per_row"):
        scpr_config = ParquetSingleChannelPerRowImportConfig._from_proto(
            proto, footer_offset=footer_offset, footer_length=footer_length
        )
        if not scpr_config.time_column.path:
            time_path = _infer_time_column(
                (col.column_config.name, ChannelDataType(col.column_config.data_type), col.path)
                for col in proto.single_channel_per_row.columns
            )
            if time_path is not None:
                scpr_config.time_column = ParquetTimeColumn(path=time_path)
        return scpr_config
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
