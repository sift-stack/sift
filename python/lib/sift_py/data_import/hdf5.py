import json
from contextlib import ExitStack
from pathlib import Path
from typing import List, TextIO, Tuple, Union, cast
from urllib.parse import urljoin

try:
    import h5py  # type: ignore
except ImportError as e:
    raise RuntimeError(
        "The h5py package is required to use the HDF5 upload service. "
        "Please include this dependency in your project by specifying `sift-stack-py[hdf5]`."
    ) from e

try:
    import polars as pl  # type: ignore
except ImportError as e:
    raise RuntimeError(
        "The polars package is required to use the HDF5 upload service. "
        "Please include this dependency in your project by specifying `sift-stack-py[hdf5]`."
    ) from e

from sift_py.data_import._config import Hdf5DataCfg
from sift_py.data_import.config import CsvConfig, Hdf5Config
from sift_py.data_import.csv import CsvUploadService
from sift_py.data_import.status import DataImportService
from sift_py.data_import.tempfile import NamedTemporaryFile
from sift_py.rest import SiftRestConfig


class Hdf5UploadService:
    """
    Service to upload HDF5 files.
    """

    _csv_upload_service: CsvUploadService

    def __init__(self, rest_conf: SiftRestConfig):
        self.RUN_PATH = "/api/v2/runs"
        self._csv_upload_service = CsvUploadService(rest_conf)

    def upload(
        self,
        path: Union[str, Path],
        hdf5_config: Hdf5Config,
        show_progress: bool = True,
    ) -> List[DataImportService]:
        """
        Uploads the HDF5 file pointed to by `path` using a custom HDF5 config.

        Args:
            path: The path to the HDF5 file.
            hdf5_config: The HDF5 config.
            show_progress: Whether to show the status bar or not.

        Returns:
            A list of DataImportServices used to get the status of the import.
        """

        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_file():
            raise Exception(f"Provided path, '{path}', does not point to a regular file.")

        # Prefer to combine data into a single CSV for upload
        # Empty data points for the String data type however will be ingested as empty strings
        # This necessitates separate files for each string dataframe
        # Split up hdf5_config into separate configs. String data is split into separate configs. All other data is a single config
        split_configs = _split_hdf5_configs(hdf5_config)

        # Ensures all temp files opened under stack.enter_context() will have __exit__ called as with a standard with statement
        with ExitStack() as stack:
            # First convert each csv file
            csv_items: List[Tuple[str, CsvConfig]] = []
            for config in split_configs:
                temp_file = stack.enter_context(NamedTemporaryFile(mode="wt", suffix=".csv"))
                csv_config = _convert_to_csv_file(
                    path,
                    temp_file,
                    config,
                )
                csv_items.append((temp_file.name, csv_config))

            # If a config defines a run_name and is split up, multiple runs will be created.
            # Instead, generate a run_id now, and use that instead of a run_name
            # Perform now instead of before the config split to avoid creating a run any problems arise before ready to upload
            if hdf5_config._hdf5_config.run_name != "":
                run_id = self._create_run(hdf5_config._hdf5_config.run_name)
                for _, csv_config in csv_items:
                    csv_config._csv_config.run_name = ""
                    csv_config._csv_config.run_id = run_id

            # Upload each file
            import_services = []
            for filename, csv_config in csv_items:
                import_services.append(
                    self._csv_upload_service.upload(
                        filename, csv_config, show_progress=show_progress
                    )
                )

        return import_services

    def _create_run(self, run_name: str) -> str:
        """Create a new run using the REST service, and return a run_id"""
        run_uri = urljoin(self._csv_upload_service._base_uri, self.RUN_PATH)

        # Since CSVUploadService is already a RestService, we can utilize that
        response = self._csv_upload_service._session.post(
            url=run_uri,
            headers={
                "Content-Encoding": "application/json",
            },
            data=json.dumps(
                {
                    "name": run_name,
                    "description": "",
                }
            ),
        )
        if response.status_code != 200:
            raise Exception(
                f"Run creation failed with status code {response.status_code}. {response.text}"
            )

        try:
            run_info = response.json()
        except (json.decoder.JSONDecodeError, KeyError):
            raise Exception(f"Invalid response: {response.text}")

        if "run" not in run_info:
            raise Exception("Response missing key: run")
        if "runId" not in run_info["run"]:
            raise Exception("Response missing key: runId")

        return run_info["run"]["runId"]


def _convert_to_csv_file(
    src_path: Union[str, Path],
    dst_file: TextIO,
    hdf5_config: Hdf5Config,
) -> CsvConfig:
    """Converts the HDF5 file to a temporary CSV on disk that we will upload.

    Args:
        src_path: The source path to the HDF5 file.
        dst_file: The output CSV file.
        hdf5_config: The HDF5 config.

    Returns:
        The CSV config for the import.
    """

    csv_cfg = _create_csv_config(hdf5_config)
    merged_df = _convert_hdf5_to_dataframes(src_path, hdf5_config)
    merged_df.write_csv(dst_file)

    return csv_cfg


def _convert_hdf5_to_dataframes(
    src_path: Union[str, Path], hdf5_config: Hdf5Config
) -> pl.DataFrame:
    """Convert the HDF5 file to a polars DataFrame.

    Args:
        src_path: The source path to the HDF5 file.
        hdf5_config: The HDF5 config.

    Returns:
        A polars DataFrame containing the data.
    """
    data_frames = []
    # Using swmr=True allows opening of HDF5 files written in SWMR mode which may have not been properly closed, but may be otherwise valid
    with h5py.File(src_path, "r", libver="latest", swmr=True) as h5f:
        for data_cfg in hdf5_config._hdf5_config.data:
            df = _extract_hdf5_data_to_dataframe(h5f, data_cfg)
            data_frames.append(df)

    # Merge polars dataframes, sort by timestamp, then write to a temp file
    # Could write csv without headers, but keeping for debugging purposes
    # If header removed, need to update 'first_data_row' in _create_csv_config to 1
    # Using join instead of concat to avoid issue if all data columns share a name
    merged_df = data_frames[0]
    if len(merged_df) > 1:
        for df in data_frames[1:]:
            merged_df = merged_df.join(df, on="timestamp", how="full", coalesce=True)
    merged_df = merged_df.sort("timestamp")
    return merged_df


def _extract_hdf5_data_to_dataframe(
    hdf5_file: h5py.File,
    hdf5_data_config: Hdf5DataCfg,
) -> pl.DataFrame:
    """Extract data from an hdf5_file to a polars DataFrame.

    Args:
        hdf5_file: HDF5 File
        hdf5_data_config: The HDF5 Data Config

    Returns:
        A two-column polars DataFrame containing the timestamps and values
    """

    if not hdf5_data_config.time_dataset in hdf5_file:
        raise Exception(f"HDF5 file does not contain dataset {hdf5_data_config.time_dataset}")
    time_dataset = cast(h5py.Dataset, hdf5_file[hdf5_data_config.time_dataset])
    if not hdf5_data_config.value_dataset in hdf5_file:
        raise Exception(f"HDF5 file does not contain dataset {hdf5_data_config.value_dataset}")
    value_dataset = cast(h5py.Dataset, hdf5_file[hdf5_data_config.value_dataset])

    # Convert the full time and value dataset to a dataframe
    # This will make it easier to work with any nested columns from a numpy structured array
    df_time = pl.DataFrame(time_dataset[:])
    df_value = pl.DataFrame(value_dataset[:])
    time_col = hdf5_data_config.time_column - 1
    val_col = hdf5_data_config.value_column - 1

    if df_time.shape[1] <= time_col:
        raise Exception(
            f"{hdf5_data_config.name}: time_column={hdf5_data_config.time_column} out of range for {hdf5_data_config.time_dataset}"
        )
    if df_value.shape[1] <= val_col:
        raise Exception(
            f"{hdf5_data_config.name}: value_column={hdf5_data_config.value_column} out of range for {hdf5_data_config.value_dataset}"
        )

    time_series = df_time[df_time.columns[time_col]]
    value_series = df_value[df_value.columns[val_col]]

    if len(time_series) != len(value_series):
        raise Exception(
            f"{hdf5_data_config.name}: time and value columns have different lengths ({len(time_series)} vs {len(value_series)})"
        )

    # HDF5 string data may come in as binary, so convert
    if time_series.dtype == pl.Binary:
        time_series = time_series.cast(pl.String)
    if value_series.dtype == pl.Binary:
        value_series = value_series.cast(pl.String)

    return pl.DataFrame(data={"timestamp": time_series, hdf5_data_config.name: value_series})


def _create_csv_config(hdf5_config: Hdf5Config) -> CsvConfig:
    """Construct a CsvConfig from a Hdf5Config

    Args:
        hdf5_path: Path to the HDF5 file
        hdf5_config: The HDF5 config

    Returns:
        The CSV config.
    """

    csv_config_dict = {
        "asset_name": hdf5_config._hdf5_config.asset_name,
        "run_name": hdf5_config._hdf5_config.run_name,
        "run_id": hdf5_config._hdf5_config.run_id,
        "first_data_row": 2,  # Row 1 is headers
        "time_column": {
            "format": hdf5_config._hdf5_config.time.format,
            "column_number": 1,
            "relative_start_time": hdf5_config._hdf5_config.time.relative_start_time,
        },
    }

    data_columns = {}
    for idx, data_cfg in enumerate(hdf5_config._hdf5_config.data):
        col_num = idx + 2  # 1-indexed and col 1 is time col
        data_columns[col_num] = {
            "name": data_cfg.name,
            "data_type": data_cfg.data_type,
            "units": data_cfg.units,
            "description": data_cfg.description,
            "enum_types": data_cfg.enum_types,
            "bit_field_elements": data_cfg.bit_field_elements,
        }

    csv_config_dict["data_columns"] = data_columns

    return CsvConfig(csv_config_dict)


def _split_hdf5_configs(hdf5_config: Hdf5Config) -> List[Hdf5Config]:
    """
    Split up hdf5_config into separate configs used to generate each CSV file
    Needed as string channels cannot be merged without creating empty string data points in the app

    Args:
        hdf5_config: The HDF5 config.

    Returns:
        List of HDF5Configs for later CSV conversion
    """

    # Combined config for non string types
    non_string_config_dict = {
        "asset_name": hdf5_config._hdf5_config.asset_name,
        "run_name": hdf5_config._hdf5_config.run_name,
        "run_id": hdf5_config._hdf5_config.run_id,
        "time": hdf5_config._hdf5_config.time,
        "data": [
            data_cfg
            for data_cfg in hdf5_config._hdf5_config.data
            if data_cfg.data_type != "CHANNEL_DATA_TYPE_STRING"
        ],
    }

    filtered_hdf5_configs = []

    # Avoid adding combined config if no non-string data present
    if non_string_config_dict["data"]:
        filtered_hdf5_configs.append(Hdf5Config(non_string_config_dict))

    for data_cfg in hdf5_config._hdf5_config.data:
        if data_cfg.data_type != "CHANNEL_DATA_TYPE_STRING":
            continue
        string_config = Hdf5Config(
            {
                "asset_name": hdf5_config._hdf5_config.asset_name,
                "run_name": hdf5_config._hdf5_config.run_name,
                "run_id": hdf5_config._hdf5_config.run_id,
                "time": hdf5_config._hdf5_config.time,
                "data": [data_cfg],
            }
        )
        filtered_hdf5_configs.append(string_config)

    return filtered_hdf5_configs
