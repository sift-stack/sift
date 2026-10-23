from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.data_imports import DataImportsLowLevelClient
from sift_client._internal.time import to_timestamp_nanos
from sift_client._internal.util.data_imports import (
    parse_csv_detect_response,
    parse_parquet_detect_response,
    prepare_parquet_config,
    resolve_data_type_key,
)
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.file import extract_parquet_footer, upload_file
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.data_import import (
    DATA_TYPE_KEY_TO_PROTO,
    DataImport,
    DataTypeKey,
    ImportConfig,
    McapImportConfig,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelPerRowImportConfig,
    TdmsImportConfig,
    TimeFormat,
    UlogImportConfig,
)
from sift_client.sift_types.run import Run
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from datetime import datetime

    import pandas as pd

    from sift_client.client import SiftClient
    from sift_client.sift_types.data_import import DataImportStatus
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
        relative_start_time: pd.Timestamp | datetime | str | None = None,
        run: Run | str | None = None,
        run_name: str | None = None,
        show_progress: bool | None = None,
        leave_run_open: bool = False,
    ) -> Job:
        """Import data from a local file.

        Creates a data import on the server, uploads the file, and returns
        a ``Job`` handle after uploading the file. The import processes
        server-side and typically completes shortly after upload. Use
        ``job.wait_until_complete()`` only if you need to confirm
        completion before proceeding.

        When ``config`` is omitted the file format is auto-detected via
        ``detect_config`` (CSV, Parquet, HDF5, TDMS, ULog, and MCAP).
        When ``asset`` is provided it overrides the config value;
        otherwise the config's ``asset_name`` is used.
        If neither ``run`` nor ``run_name`` is provided (and none is
        set on the config), ``run_name`` defaults to the filename.

        Examples:
            Import a CSV file with auto-detected config:

                job = client.data_import.import_from_path(
                    "data.csv",
                    asset=my_asset,
                )

            Auto-detect config, inspect and patch before importing:

                config = client.data_import.detect_config("data.csv")

                # Fix a column data type
                config["temperature"].data_type = ChannelDataType.FLOAT

                # Remove an unwanted column
                config.data_columns = [
                    dc for dc in config.data_columns if dc.name != "internal_id"
                ]

                job = client.data_import.import_from_path(
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
                the config is auto-detected via ``detect_config`` (for ULog
                and MCAP the detected channel list is dropped so every channel
                in the file is imported). You can call ``detect_config``
                yourself to inspect and modify the config before passing it
                here.
            data_type: Explicit data type key. Required for formats with
                multiple supported layouts (Parquet, HDF5) where the file
                extension alone is ambiguous. Only used when ``config`` is
                not provided.
            time_format: Time format override for CSV, Parquet, HDF5, and TDMS.
                Ignored for ULog and MCAP. When omitted, CSV, Parquet, and
                HDF5 use the detected format if available, otherwise
                ``TimeFormat.ABSOLUTE_UNIX_NANOSECONDS``. TDMS keeps its
                detected/default time handling. Only used when ``config`` is
                not provided.
            relative_start_time: Start time for a relative time format.
                A string is parsed with ``pd.Timestamp``; a string or
                ``pd.Timestamp`` keeps nanosecond precision.
                For ULog, this is the log start time and takes precedence
                over the log's GPS fix. For MCAP, set it only for logs whose
                ``log_time`` is not on the Unix epoch; ``log_time`` is then
                read as nanoseconds elapsed from this time. For other
                formats, it is required with a relative time format and
                ignored with a warning otherwise. Only used when ``config``
                is not provided.
            run: ``Run`` object or run ID string to import into an existing
                run. Mutually exclusive with ``run_name``.
            run_name: Name for a new run. Defaults to the filename if
                neither ``run`` nor ``run_name`` is set.
            show_progress: If True, display a progress spinner during upload.
                Defaults to True for sync, False for async.
            leave_run_open: If true, the import does not mark its run stopped,
                so run-stop automations do not fire. Set this when more data
                is coming to the run, then call ``Run.stop`` after the last
                import. Ignored when the import has no run. Defaults to false,
                which stops the run when the import finishes.

        Returns:
            A ``Job`` handle for the pending import. Call
            ``job.get_data_import()`` on it for the import's status, error
            message, and warnings, which are not on the job.

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
                relative_start_time=relative_start_time,
            )
            config._validate_relative_start_time()
            if isinstance(config, (UlogImportConfig, McapImportConfig)):
                # An empty channel list imports every channel
                config.data = []

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
            await prepare_parquet_config(config, path)

        if show_progress is None:
            show_progress = self._show_progress()

        data_import_id, upload_url = await self._low_level_client.create_from_upload(
            config, leave_run_open=leave_run_open
        )

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

        job = await self.client.async_.jobs.get(job_id=job_id)
        # The job's details are filled in server-side and may not be set yet.
        job._set_data_import_id(data_import_id)
        return job

    async def get(self, data_import_id: str) -> DataImport:
        """Get a data import by ID.

        The ``data_import_id`` is available on the job returned by
        ``import_from_path`` via ``job.job_details.data_import_id``.
        For a more ergonomic approach, use ``job.get_data_import()``
        which calls this method internally.

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
        # Self ids
        data_import_ids: list[str] | None = None,
        # Resource-specific filters
        source_url: str | None = None,
        source_url_contains: str | None = None,
        status: DataImportStatus | None = None,
        runs: list[Run | str] | None = None,
        # Common filters
        filter_query: str | None = None,
        # Ordering and pagination
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[DataImport]:
        """List data imports with optional filtering.

        The server only supports filtering on ``data_import_id``,
        ``source_url``, ``status``, and ``run_id``, and only supports
        ordering by ``created_date`` and ``modified_date``.

        Args:
            data_import_ids: Filter to data imports with any of these IDs.
            source_url: Filter to data imports with exactly this source url.
            source_url_contains: Filter to data imports whose source url
                contains this substring.
            status: Filter to data imports with this status.
            runs: Filter to data imports that ingested into any of these Runs
                or run IDs.
            filter_query: Explicit CEL query to filter data imports.
            order_by: Field and direction to order results by, e.g.
                ``"created_date desc"``. Defaults to oldest-first by
                ``created_date``.
            limit: Maximum number of data imports to return. If None, returns
                all matches.
            page_size: Number of results to fetch per request. Lower this if you hit gRPC
                message size limits on responses. If None, uses the server default.

        Returns:
            A list of DataImport objects that match the filter criteria.
        """
        filter_parts = [*self._build_common_cel_filters(filter_query=filter_query)]
        if data_import_ids:
            filter_parts.append(cel.in_("data_import_id", data_import_ids))
        if source_url:
            filter_parts.append(cel.equals("source_url", source_url))
        if source_url_contains:
            filter_parts.append(cel.contains("source_url", source_url_contains))
        if status:
            filter_parts.append(cel.equals("status", status.to_filter_str()))
        if runs:
            run_ids = [r._id_or_error if isinstance(r, Run) else r for r in runs]
            filter_parts.append(cel.in_("run_id", run_ids))

        query_filter = cel.and_(*filter_parts)

        data_imports = await self._low_level_client.list_all_data_imports(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(data_imports)

    async def find(self, **kwargs) -> DataImport | None:
        """Find a single data import matching the given query. Takes the same arguments as
        `list_`. If more than one data import is found, raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The DataImport found or None.
        """
        data_imports = await self.list_(**kwargs)
        if len(data_imports) > 1:
            raise ValueError("Multiple data imports found for query")
        elif len(data_imports) == 1:
            return data_imports[0]
        return None

    async def get_run(self, data_import_id: str) -> Run:
        """Get the run associated with a data import.

        The ``data_import_id`` is available on the job returned by
        ``import_from_path`` via ``job.job_details.data_import_id``.
        For a more ergonomic approach, use ``job.get_import_run()``
        which calls this method internally, or ``job.get_data_import()``
        followed by the import's ``run_id``.

        Args:
            data_import_id: The ID of the data import.

        Returns:
            The Run created by or associated with the import.

        Raises:
            ValueError: If the data import has no associated run.
        """
        data_import = await self._low_level_client.get(data_import_id)
        if not data_import.run_id:
            raise ValueError("Data import does not have an associated run.")
        return await self.client.async_.runs.get(run_id=data_import.run_id)

    async def detect_config(
        self,
        file_path: str | Path,
        data_type: DataTypeKey | None = None,
        time_format: TimeFormat | None = None,
        *,
        relative_start_time: pd.Timestamp | datetime | str | None = None,
    ) -> ImportConfig:
        """Auto-detect import configuration from a file.

        Returns the detected configuration, inferring the file format from the
        extension when ``data_type`` is not provided. CSV and Parquet are
        detected by sending a sample of the file to the server's DetectConfig
        endpoint; TDMS, HDF5, ULog, and MCAP are detected locally on the
        client.

        CSV, Parquet, HDF5, TDMS, ULog, and MCAP files are supported for
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

        For ULog files, ``data`` lists the channels pyulog decodes from the
        file. When imported, a non-empty ``data`` list restricts the import
        to exactly those channels; the import fails if a listed channel is
        not in the file. Clear ``data`` to import every channel.

        For MCAP files, ``data`` lists one channel per flattened field of each
        supported topic, without decoding messages. A variable-cardinality
        field is one entry; ``complex_types_import_mode`` on the config decides
        whether it imports as Arrow IPC bytes, a JSON string under
        ``<name>.json``, both, or neither. The same non-empty ``data``
        semantics as ULog apply. Topics that cannot be decoded are skipped with
        a warning; importing such a file fails unless
        ``McapParseErrorPolicy.IGNORE_ERROR`` is set.

        For file types with multiple supported layouts (Parquet, HDF5),
        ``data_type`` must be specified explicitly.

        Args:
            file_path: Path to the file to analyze.
            data_type: Explicit data type key. Required for formats with
                multiple supported layouts (Parquet, HDF5) where the file
                extension alone is ambiguous.
            time_format: Time format override for CSV, Parquet, HDF5, and TDMS.
                Ignored for ULog and MCAP. When omitted, CSV, Parquet, and
                HDF5 use the detected format if available, otherwise
                ``TimeFormat.ABSOLUTE_UNIX_NANOSECONDS``. TDMS keeps its
                detected/default time handling.
            relative_start_time: Start time for a relative time format.
                A string is parsed with ``pd.Timestamp``; a string or
                ``pd.Timestamp`` keeps nanosecond precision.
                For ULog, this is the log start time and takes precedence
                over the log's GPS fix. For MCAP, set it only for logs whose
                ``log_time`` is not on the Unix epoch; ``log_time`` is then
                read as nanoseconds elapsed from this time. For other
                formats, it is required with a relative time format and
                ignored with a warning otherwise.

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

        data_type_key = resolve_data_type_key(path.suffix.lower(), data_type)
        if isinstance(relative_start_time, str):
            relative_start_time = to_timestamp_nanos(relative_start_time)
        config = await self._detect_config_for_type(
            path, data_type_key, relative_start_time=relative_start_time
        )
        if time_format is not None:
            config._set_time_format(time_format)
        elif (
            not isinstance(config, (TdmsImportConfig, UlogImportConfig, McapImportConfig))
            and config._get_time_format() is None
        ):
            config._set_time_format(TimeFormat.ABSOLUTE_UNIX_NANOSECONDS)
        if relative_start_time is not None:
            config._apply_relative_start_time(relative_start_time)
        return config

    async def _detect_config_for_type(
        self,
        path: Path,
        data_type_key: DataTypeKey,
        *,
        relative_start_time: datetime | None = None,
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
        if data_type_key == DataTypeKey.MCAP:
            try:
                from sift_client._internal.util.mcap import detect_mcap_config
            except ImportError as e:
                raise RuntimeError(
                    "mcap and mcap-ros2-support are required for MCAP import. "
                    "Install them via `pip install sift-stack-py[mcap]`."
                ) from e
            return await run_sync_function(lambda: detect_mcap_config(path))
        if data_type_key == DataTypeKey.PARQUET_FLATDATASET:
            try:
                from sift_client._internal.util.parquet import detect_flat_parquet_config
            except ImportError as e:
                raise RuntimeError(
                    "pyarrow is required for Parquet import. "
                    "Install it via `pip install sift-stack-py[parquet]`."
                ) from e
            return await run_sync_function(
                lambda: detect_flat_parquet_config(path, relative_start_time=relative_start_time)
            )
        is_parquet = data_type_key == DataTypeKey.PARQUET_SINGLE_CHANNEL_PER_ROW
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
            return parse_csv_detect_response(
                response.csv_config, path.name, relative_start_time=relative_start_time
            )

        if response.HasField("parquet_config"):
            return parse_parquet_detect_response(
                response.parquet_config,
                path.name,
                footer_offset,
                footer_length,
                relative_start_time=relative_start_time,
            )

        raise ValueError(
            f"No supported configuration detected for '{path.name}'. "
            "Only CSV, Parquet, HDF5, TDMS, ULog, and MCAP are supported by auto-detection."
        )
