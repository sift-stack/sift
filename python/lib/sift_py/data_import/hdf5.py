import uuid
from collections import defaultdict
from contextlib import ExitStack
from pathlib import Path
from typing import Dict, List, Tuple, Union, cast

import numpy as np

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
    _prev_run_id: str

    def __init__(self, rest_conf: SiftRestConfig):
        self._csv_upload_service = CsvUploadService(rest_conf)
        self._prev_run_id = ""

    def upload(
        self,
        path: Union[str, Path],
        hdf5_config: Hdf5Config,
        show_progress: bool = True,
    ) -> DataImportService:
        """
        Uploads the HDF5 file pointed to by `path` using a custom HDF5 config.

        Args:
            path: The path to the HDF5 file.
            hdf5_config: The HDF5 config.
            show_progress: Whether to show the status bar or not.

        Returns:
            DataImportService used to get the status of the import
        """

        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_file():
            raise Exception(f"Provided path, '{path}', does not point to a regular file.")

        # Prefer to combine data into a single CSV for upload
        # Empty data points for the String data type however will be ingested as empty strings
        # This necessitates separate files for each string dataframe
        # Split up hdf5_config into separate configs. String data is split into separate configs. All other data is a single config
        split_configs = _split_hdf5_configs(hdf5_config)

        # NamedTemporaryFiles will delete upon exiting with block
        # ExitStack used to ensures all temp files stay open through upload, than are closed upon existing block or if program exits early
        with ExitStack() as stack:
            # First convert each csv file
            csv_items: List[Tuple[str, CsvConfig]] = []
            for config in split_configs:
                temp_file = stack.enter_context(NamedTemporaryFile(mode="w", suffix=".csv"))
                csv_config = _convert_to_csv_file(
                    path,
                    temp_file.name,
                    config,
                )
                csv_items.append((temp_file.name, csv_config))

            if not csv_items:
                raise Exception("No data found for upload during processing of file")

            # If a config defines a run_name and is split up, multiple runs will be created.
            # Instead, generate a run_id now, and use that instead of a run_name
            # Perform now instead of before the config split to avoid creating a run any problems arise before ready to upload
            # Active run_id copied to _prev_run_id for user reference
            if hdf5_config._hdf5_config.run_name != "":
                run_id = self._csv_upload_service._create_run(hdf5_config._hdf5_config.run_name)
                for _, csv_config in csv_items:
                    csv_config._csv_config.run_name = ""
                    csv_config._csv_config.run_id = run_id

                self._prev_run_id = run_id
            elif hdf5_config._hdf5_config.run_id != "":
                self._prev_run_id = hdf5_config._hdf5_config.run_id
            else:
                self._prev_run_id = ""

            # Upload each file
            import_service = None
            for filename, csv_config in csv_items:
                new_import_service = self._csv_upload_service.upload(
                    filename, csv_config, show_progress=show_progress
                )
                if import_service is None:
                    import_service = new_import_service
                else:
                    import_service.extend(new_import_service)

        if import_service is not None:
            return import_service
        else:
            raise Exception("No data uploaded by service")

    def get_previous_upload_run_id(self) -> str:
        """Return the run_id used in the previous upload"""
        return self._prev_run_id


def _convert_to_csv_file(
    src_path: Union[str, Path],
    dst_file: str,
    hdf5_config: Hdf5Config,
) -> CsvConfig:
    """Converts the HDF5 file to a temporary CSV on disk that we will upload.

    Args:
        src_path: The source path to the HDF5 file.
        dst_file: The output CSV file path.
        hdf5_config: The HDF5 config.

    Returns:
        The CSV config for the import.
    """

    merged_df = _convert_hdf5_to_dataframes(src_path, hdf5_config)
    csv_cfg = _create_csv_config(hdf5_config, merged_df)
    # polars write_csv requires a path, not a TextIO to work in windows
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
    # Group data configs by matching time arrays to optimize downstream data processing
    data_cfg_ts_map: Dict[Tuple[str, int], List[Hdf5DataCfg]] = defaultdict(list)
    for data_cfg in hdf5_config._hdf5_config.data:
        map_tuple = (data_cfg.time_dataset, data_cfg.time_column)
        data_cfg_ts_map[map_tuple].append(data_cfg)

    data_frames = []
    # Using swmr=True allows opening of HDF5 files written in SWMR mode which may have not been properly closed, but may be otherwise valid
    with h5py.File(src_path, "r", libver="latest", swmr=True) as h5f:
        for (time_path, time_col), data_cfgs in data_cfg_ts_map.items():
            df = _extract_hdf5_data_to_dataframe(h5f, time_path, time_col, data_cfgs)
            data_frames.append(df)

    # Merge polars dataframes by joining pairs, then merging those pairs until one dataframe remains
    # More optimized than joining one by one
    # pl.concat(data_frames, how="align") in practice can lead to a fatal crash with larger files
    # https://github.com/pola-rs/polars/issues/14591
    while len(data_frames) > 1:
        next_round = []
        for i in range(0, len(data_frames), 2):
            if i + 1 < len(data_frames):
                df1 = data_frames[i]
                df2 = data_frames[i + 1]
                merged = _merge_timeseries_dataframes(df1, df2)
                next_round.append(merged)
            else:
                next_round.append(data_frames[i])
        data_frames = next_round
    merged_df = data_frames[0].sort("timestamp")
    return merged_df


def _merge_timeseries_dataframes(df1: pl.DataFrame, df2: pl.DataFrame) -> pl.DataFrame:
    """Merge two timeseries dataframes together. Handles duplicate channels"""

    df1_channels = [col for col in df1.columns if col != "timestamp"]
    df2_channels = [col for col in df2.columns if col != "timestamp"]
    dup_channels = set(df1_channels) & set(df2_channels)

    if dup_channels:
        # Create a unique id to mark duplicate channels
        uid = uuid.uuid4()

        df2_renamed = df2.clone()
        for col in dup_channels:
            df2_renamed = df2_renamed.rename({col: f"{col}_{uid}"})

        merged_df = df1.join(df2_renamed, on="timestamp", how="full", coalesce=True)

        # Merge duplicate column data
        for col in dup_channels:
            temp_col_name = f"{col}_{uid}"
            merged_df = merged_df.with_columns(
                pl.coalesce([pl.col(col), pl.col(temp_col_name)]).alias(col)
            ).drop(temp_col_name)

    else:
        merged_df = df1.join(df2, on="timestamp", how="full", coalesce=True)

    return merged_df


def _extract_hdf5_data_to_dataframe(
    hdf5_file: h5py.File,
    time_path: str,
    time_col: int,
    hdf5_data_configs: List[Hdf5DataCfg],
) -> pl.DataFrame:
    """Extract data from an hdf5_file to a polars DataFrame.

    Args:
        hdf5_file: HDF5 File
        time_path: HDF5 time array path
        time_col: HDF5 time array col (1-indexed)
        hdf5_data_config: List of HDF5 Data Configs being extracted

    Returns:
        A multi-column polars DataFrame containing the timestamps and associated channels
    """

    if not time_path in hdf5_file:
        raise Exception(f"HDF5 file does not contain dataset {time_path}")
    time_dataset = cast(h5py.Dataset, hdf5_file[time_path])
    df_time = pl.DataFrame(time_dataset[:])
    time_idx = time_col - 1

    if df_time.shape[1] <= time_idx:
        raise Exception(f"{time_path}: time_column={time_col} out of range")
    time_series = df_time[df_time.columns[time_idx]]

    # HDF5 string data may come in as binary, so convert
    if time_series.dtype == pl.Binary:
        time_series = time_series.cast(pl.String)

    data_frame = pl.DataFrame(data={"timestamp": time_series})

    for hdf5_data_config in hdf5_data_configs:
        if not hdf5_data_config.value_dataset in hdf5_file:
            raise Exception(f"HDF5 file does not contain dataset {hdf5_data_config.value_dataset}")

        # Should always be true due to calling code
        assert time_path == hdf5_data_config.time_dataset, (
            f"Working time dataset {time_path} does not match data cfg defined dataset {hdf5_data_config.time_dataset}"
        )
        assert time_col == hdf5_data_config.time_column, (
            f"Working time col {time_col} does not match data cfg defined col {hdf5_data_config.time_column}"
        )

        value_dataset = cast(h5py.Dataset, hdf5_file[hdf5_data_config.value_dataset])

        # Convert the full value dataset to a dataframe
        # This will make it easier to work with any nested columns from a numpy structured array
        df_value = pl.DataFrame(value_dataset[:])
        val_idx = hdf5_data_config.value_column - 1

        if df_value.shape[1] <= val_idx:
            raise Exception(
                f"{hdf5_data_config.name}: value_column={hdf5_data_config.value_column} out of range for {hdf5_data_config.value_dataset}"
            )
        value_series = df_value[df_value.columns[val_idx]]

        if len(time_series) != len(value_series):
            raise Exception(
                f"{hdf5_data_config.name}: time and value columns have different lengths ({len(time_series)} vs {len(value_series)})"
            )

        # HDF5 string data may come in as binary, so convert
        if value_series.dtype == pl.Binary:
            value_series = value_series.cast(pl.String)

        # Handle signed enums
        # TODO: Remove once properly handled upon ingestion
        if hdf5_data_config.data_type == "CHANNEL_DATA_TYPE_ENUM" and any(
            [enum_type.is_signed for enum_type in hdf5_data_config.enum_types]
        ):
            value_series = _convert_signed_enums(hdf5_data_config, value_series)

        data_frame = data_frame.with_columns(value_series.alias(hdf5_data_config.name))

    return data_frame


def _convert_signed_enums(data_cfg: Hdf5DataCfg, data: pl.Series) -> pl.Series:
    """
    Convert signed enums to unsigned ints for ingestion
    Ignores keys >= 0, such as those which may have been converted previously by the user
    Will raise an exception if casting will cause a collision with an existing key
    Or otherwise cannot cast signed negative int to a uint32
    """
    cur_enum_keys = set([enum_type.key for enum_type in data_cfg.enum_types])

    for enum_type in data_cfg.enum_types:
        if not enum_type.is_signed or enum_type.key >= 0:
            continue
        if enum_type.key < -2_147_483_648:
            raise Exception(
                f"{data_cfg.name}: Cannot convert key {enum_type.key} to uint32 due to being below valid int32 range"
            )
        unsigned_key = enum_type.key + (1 << 32)
        if unsigned_key in cur_enum_keys:
            raise Exception(
                f"{data_cfg.name}: Converting key {enum_type.key} to unsigned int collides with existing key {unsigned_key}"
            )
        enum_type.key = unsigned_key

    # Numpy astype will wrap negative values
    return pl.Series(data.to_numpy().astype(np.uint32))


def _create_csv_config(hdf5_config: Hdf5Config, merged_df: pl.DataFrame) -> CsvConfig:
    """Construct a CsvConfig from a Hdf5Config

    Args:
        hdf5_config: The HDF5 config
        merged_df: The merged dataFrame of data

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

    # Map each data config to its channel name
    config_map = {d_cfg.name: d_cfg for d_cfg in hdf5_config._hdf5_config.data}

    assert merged_df.columns[0] == "timestamp", (
        f"Unexpected merged DataFrame layout. Expected first column to be timestamp, not {merged_df.columns[0]}"
    )

    data_columns = {}
    for idx, channel_name in enumerate(merged_df.columns[1:]):
        data_cfg = config_map[channel_name]
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
