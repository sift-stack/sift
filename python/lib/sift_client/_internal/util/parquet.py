"""Detect the import configuration of flat dataset Parquet files.
Reads only the Parquet footer schema, flattens struct fields into one channel
per leaf, and picks the first top-level time typed column as the time column.
Files exported with embedded channel configs return that config directly.
"""

from __future__ import annotations

import json
import warnings
from datetime import datetime, timezone
from pathlib import Path

import pyarrow as pa
import pyarrow.parquet as pq
from google.protobuf import json_format
from google.protobuf.timestamp_pb2 import Timestamp as TimestampPb
from sift.common.type.v1.channel_config_pb2 import ChannelConfig as ChannelConfigProto

from sift_client._internal.util.file import extract_parquet_footer
from sift_client.resources.data_imports import _infer_time_column
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    ParquetDataColumn,
    ParquetFlatDatasetImportConfig,
    ParquetTimeColumn,
    TimeFormat,
    TimeFormatProto,
    _enum_types_from_proto,
)

# Separator used in column paths to address nested struct fields. Channel
# names replace it with "." since Sift forbids "|" in channel names.
PATH_SEPARATOR = "|"

# Arrow field metadata keys written when a file is exported with
# embed_channel_configs.
EMBEDDED_TIME_CONFIG_KEY = b"sift_time_config"
EMBEDDED_CHANNEL_CONFIG_KEY = b"sift_channel_config"

_TIMESTAMP_UNIT_TO_FORMAT: dict[str, TimeFormat] = {
    "s": TimeFormat.ABSOLUTE_UNIX_SECONDS,
    "ms": TimeFormat.ABSOLUTE_UNIX_MILLISECONDS,
    "us": TimeFormat.ABSOLUTE_UNIX_MICROSECONDS,
    "ns": TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
}

_RELATIVE_UNIT_TO_FORMAT: dict[str, TimeFormat] = {
    "s": TimeFormat.RELATIVE_SECONDS,
    "ms": TimeFormat.RELATIVE_MILLISECONDS,
    "us": TimeFormat.RELATIVE_MICROSECONDS,
    "ns": TimeFormat.RELATIVE_NANOSECONDS,
}


def _is_time_type(t: pa.DataType) -> bool:
    # Durations are excluded: the server's Parquet reader sees them as plain
    # int64 columns, so they are never time columns.
    return (
        pa.types.is_timestamp(t)
        or pa.types.is_time32(t)
        or pa.types.is_time64(t)
        or pa.types.is_date32(t)
        or pa.types.is_date64(t)
    )


def _is_list_type(t: pa.DataType) -> bool:
    # The server reads large lists as lists. Fixed size lists are unsupported.
    return pa.types.is_list(t) or pa.types.is_large_list(t)


def _arrow_to_sift_type(t: pa.DataType) -> ChannelDataType | None:
    """Map an Arrow type to the channel data type it is ingested as.

    Returns None for unsupported types and structs, which are flattened.
    Smaller ints widen to 32-bit and float16 widens to float.
    """
    if pa.types.is_boolean(t):
        return ChannelDataType.BOOL
    if pa.types.is_float16(t) or pa.types.is_float32(t):
        return ChannelDataType.FLOAT
    if pa.types.is_float64(t):
        return ChannelDataType.DOUBLE
    if pa.types.is_int8(t) or pa.types.is_int16(t) or pa.types.is_int32(t):
        return ChannelDataType.INT_32
    if pa.types.is_int64(t) or pa.types.is_duration(t):
        return ChannelDataType.INT_64
    if pa.types.is_uint8(t) or pa.types.is_uint16(t) or pa.types.is_uint32(t):
        return ChannelDataType.UINT_32
    if pa.types.is_uint64(t):
        return ChannelDataType.UINT_64
    if pa.types.is_string(t) or pa.types.is_large_string(t) or pa.types.is_string_view(t):
        return ChannelDataType.STRING
    if (
        pa.types.is_binary(t)
        or pa.types.is_large_binary(t)
        or pa.types.is_binary_view(t)
        or pa.types.is_fixed_size_binary(t)
    ):
        return ChannelDataType.BYTES
    # All time types are ingested as int64 unix nanoseconds.
    if _is_time_type(t):
        return ChannelDataType.INT_64
    # Ingested as BYTES by default. The import can also add a STRING channel
    # depending on complex_types_import_mode.
    if _is_list_type(t) or pa.types.is_map(t):
        return ChannelDataType.BYTES
    return None


def _is_supported_type(t: pa.DataType) -> bool:
    """Arrow types supported for Parquet ingestion."""
    return pa.types.is_struct(t) or _arrow_to_sift_type(t) is not None


def _add_fields(columns: list[ParquetDataColumn], field: pa.Field, parent_path: str = "") -> None:
    """Append a column per leaf of ``field``, flattening structs.

    Nested paths are joined with ``PATH_SEPARATOR``; channel names use ``.``.
    Duplicate channel names are not rejected here.
    """
    full_path = f"{parent_path}{PATH_SEPARATOR}{field.name}" if parent_path else field.name

    if pa.types.is_struct(field.type):
        for i in range(field.type.num_fields):
            _add_fields(columns, field.type.field(i), full_path)
        return

    name = full_path.replace(PATH_SEPARATOR, ".")
    data_type = _arrow_to_sift_type(field.type)
    if data_type is None:
        raise ValueError(f"unsupported data type for column '{full_path}': {field.type}")
    columns.append(ParquetDataColumn(name=name, data_type=data_type, path=full_path))


def _detect_time_column(
    field: pa.Field, relative_start_time: datetime | None = None
) -> ParquetTimeColumn | None:
    """Return a time column for ``field`` if it has a time type.

    ``relative_start_time`` is only set on relative (time32/time64) columns.
    """
    t = field.type
    if not _is_time_type(t):
        return None

    if pa.types.is_time32(t) or pa.types.is_time64(t):
        return ParquetTimeColumn(
            path=field.name,
            format=_RELATIVE_UNIT_TO_FORMAT[t.unit],
            relative_start_time=relative_start_time,
        )
    if pa.types.is_timestamp(t):
        return ParquetTimeColumn(path=field.name, format=_TIMESTAMP_UNIT_TO_FORMAT[t.unit])
    # Dates are converted to unix nanoseconds.
    return ParquetTimeColumn(path=field.name, format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS)


def _has_embedded_config(schema: pa.Schema) -> bool:
    return any(
        field.metadata
        and (
            EMBEDDED_TIME_CONFIG_KEY in field.metadata
            or EMBEDDED_CHANNEL_CONFIG_KEY in field.metadata
        )
        for field in schema
    )


def _parse_embedded_time_column(path: str, raw: bytes) -> ParquetTimeColumn:
    """Parse the ``sift_time_config`` JSON written by the export service.

    Like the server, the format must be a ``TimeFormat`` enum name.
    """
    try:
        config = json.loads(raw)
    except json.JSONDecodeError as e:
        raise ValueError("failed to parse embedded time config") from e
    format_name = config.get("time_format", "") if isinstance(config, dict) else None
    if not isinstance(format_name, str):
        raise ValueError("failed to parse embedded time config")

    try:
        format_value = TimeFormatProto.Value(format_name)
    except ValueError as e:
        raise ValueError(f"invalid embedded time format: {format_name}") from e
    fmt = TimeFormat(format_value) if format_value else None

    relative_start_time = None
    raw_start = config.get("relative_start_time")
    if raw_start:
        start = TimestampPb()
        start.FromJsonString(raw_start)
        relative_start_time = start.ToDatetime(tzinfo=timezone.utc)

    return ParquetTimeColumn(path=path, format=fmt, relative_start_time=relative_start_time)


def _parse_embedded_data_column(path: str, raw: bytes) -> ParquetDataColumn:
    """Parse the ``sift_channel_config`` JSON (a ChannelConfig) written by the export service.

    Like the server, unknown fields and enum values are rejected.
    """
    try:
        config = json_format.Parse(raw, ChannelConfigProto())
        data_type = ChannelDataType(config.data_type) if config.data_type else None
    except (json_format.ParseError, ValueError) as e:
        raise ValueError(f"failed to parse embedded channel config for column {path}") from e
    if data_type is None:
        raise ValueError(f"missing data type in embedded channel config for column {path}")
    return ParquetDataColumn(
        path=path,
        name=config.name,
        data_type=data_type,
        units=config.units,
        description=config.description,
        enum_types=_enum_types_from_proto(config),
    )


def _embedded_flat_dataset_config(
    schema: pa.Schema,
) -> tuple[ParquetTimeColumn, list[ParquetDataColumn]]:
    """Build the time and data columns from sift-exported configs embedded in field metadata."""
    time_column: ParquetTimeColumn | None = None
    data_columns: list[ParquetDataColumn] = []

    for field in schema:
        metadata = field.metadata or {}

        raw_time = metadata.get(EMBEDDED_TIME_CONFIG_KEY)
        if raw_time is not None:
            if time_column is not None:
                raise ValueError("found multiple embedded time column configs")
            time_column = _parse_embedded_time_column(field.name, raw_time)
            continue

        raw_channel = metadata.get(EMBEDDED_CHANNEL_CONFIG_KEY)
        if raw_channel is not None:
            data_columns.append(_parse_embedded_data_column(field.name, raw_channel))

    if time_column is None:
        raise ValueError("no embedded time config found in parquet file")
    if not data_columns:
        raise ValueError("no embedded channel configs found in parquet file")
    return time_column, data_columns


def detect_flat_parquet_config(
    file_path: str | Path, relative_start_time: datetime | None = None
) -> ParquetFlatDatasetImportConfig:
    """Detect the flat dataset import configuration of a Parquet file.

    Every supported column except the time column becomes a data column.
    Struct fields are flattened. The first top-level time typed column is used
    as the time column. If none exists, an INT64 or UINT64 column named ``ts``,
    ``timestamp``, or ``time`` is used instead. Otherwise ``time_column.path``
    is empty.

    Args:
        file_path: The path to the Parquet file.
        relative_start_time: Start time for a relative (time32/time64) time
            column. Required when the detected time column is relative.
            Ignored for absolute time columns and embedded configs.

    Returns:
        The detected configuration with footer offset and length populated.

    Raises:
        ValueError: If the file is not a valid Parquet file, a struct holds an
            unsupported type, or an embedded config is malformed.
    """
    path = Path(file_path)
    footer_bytes, footer_offset = extract_parquet_footer(path)
    schema = pq.read_schema(path)

    if _has_embedded_config(schema):
        time_column, data_columns = _embedded_flat_dataset_config(schema)
    else:
        time_column = ParquetTimeColumn(path="")
        data_columns = []
        for field in schema:
            if not _is_supported_type(field.type):
                warnings.warn(
                    f"skipping unsupported data type for column '{field.name}': {field.type}",
                    stacklevel=2,
                )
                continue

            try:
                _add_fields(data_columns, field)
            except ValueError as e:
                raise ValueError(f"unable to add field {field.name}: {e}") from e

            # Select the first encountered time column, if it exists.
            if not time_column.path:
                detected = _detect_time_column(field, relative_start_time)
                if detected is not None:
                    time_column = detected

        if not time_column.path:
            inferred = _infer_time_column((dc.name, dc.data_type, dc.path) for dc in data_columns)
            if inferred:
                time_column = ParquetTimeColumn(path=inferred)
        data_columns = [dc for dc in data_columns if dc.path != time_column.path]

    return ParquetFlatDatasetImportConfig(
        asset_name="",
        time_column=time_column,
        data_columns=data_columns,
        footer_offset=footer_offset,
        footer_length=len(footer_bytes),
    )
