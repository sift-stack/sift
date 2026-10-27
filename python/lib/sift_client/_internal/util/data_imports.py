"""Helpers for detecting and preparing data import configs."""

from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.time import to_timestamp_pb
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.file import extract_parquet_footer
from sift_client._internal.util.time_column import infer_time_column
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    EXTENSION_TO_DATA_TYPE_KEY,
    CsvImportConfig,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelPerRowImportConfig,
    ParquetTimeColumn,
    TimeFormat,
)

if TYPE_CHECKING:
    from datetime import datetime
    from pathlib import Path

    from sift_client.sift_types.data_import import DataTypeKey


_RELATIVE_FORMAT_VALUES = {f.value for f in TimeFormat if f.is_relative}


def set_detected_start_time(time_column, relative_start_time: datetime | None) -> None:
    """Set the start time on a detected relative time column proto.

    Without it, a relative format detected by the server fails config
    validation before ``relative_start_time`` can be applied.
    """
    if relative_start_time is not None and time_column.format in _RELATIVE_FORMAT_VALUES:
        time_column.relative_start_time.CopyFrom(to_timestamp_pb(relative_start_time))


def resolve_data_type_key(ext: str, data_type: DataTypeKey | None) -> DataTypeKey:
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


def parse_csv_detect_response(
    proto, filename: str, *, relative_start_time: datetime | None = None
) -> CsvImportConfig:
    """Parse a CSV DetectConfig response into a config."""
    if proto.time_column.column_number < 1:
        raise ValueError(
            f"No time column detected in '{filename}'. Build a CsvImportConfig with "
            "time_column set and pass it to import_from_path as config."
        )
    set_detected_start_time(proto.time_column, relative_start_time)
    csv_config = CsvImportConfig._from_proto(proto)
    time_col = csv_config.time_column.column
    csv_config.data_columns = [dc for dc in csv_config.data_columns if dc.column != time_col]
    return csv_config


def parse_parquet_detect_response(
    proto,
    filename: str,
    footer_offset: int,
    footer_length: int,
    *,
    relative_start_time: datetime | None = None,
) -> ParquetFlatDatasetImportConfig | ParquetSingleChannelPerRowImportConfig:
    """Parse a Parquet DetectConfig response into a config."""
    if proto.HasField("flat_dataset"):
        set_detected_start_time(proto.flat_dataset.time_column, relative_start_time)
        parquet_config = ParquetFlatDatasetImportConfig._from_proto(
            proto, footer_offset=footer_offset, footer_length=footer_length
        )
        time_path: str | None = parquet_config.time_column.path
        if not time_path:
            time_path = infer_time_column(
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
        set_detected_start_time(proto.single_channel_per_row.time_column, relative_start_time)
        scpr_config = ParquetSingleChannelPerRowImportConfig._from_proto(
            proto, footer_offset=footer_offset, footer_length=footer_length
        )
        if not scpr_config.time_column.path:
            time_path = infer_time_column(
                (col.column_config.name, ChannelDataType(col.column_config.data_type), col.path)
                for col in proto.single_channel_per_row.columns
            )
            if time_path is not None:
                scpr_config.time_column = ParquetTimeColumn(path=time_path)
        return scpr_config
    raise ValueError(f"Unsupported parquet layout in DetectConfig response for '{filename}'.")


async def prepare_parquet_config(
    config: ParquetFlatDatasetImportConfig | ParquetSingleChannelPerRowImportConfig,
    path: Path,
) -> None:
    """Populate parquet footer fields on the config if not already set."""
    if config.footer_offset == 0 and config.footer_length == 0:
        footer_bytes, footer_offset = await run_sync_function(lambda: extract_parquet_footer(path))
        config.footer_offset = footer_offset
        config.footer_length = len(footer_bytes)
