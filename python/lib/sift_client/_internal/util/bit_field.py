"""Check bit field values before upload, so a wrong encoding fails early.

The import service reads a bit field value as raw bytes, a string of 0s and 1s,
or URL-safe base64, and fails the import on the first value it can't parse. It
only reports that after the whole file uploads. These checks read the
Parquet schema and a sample of values, and raise before the upload starts.
"""

from __future__ import annotations

import base64
import binascii
import csv
import re
from typing import TYPE_CHECKING, Iterable, Mapping

from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    CsvImportConfig,
    DataColumnBase,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelPerRowImportConfig,
)

if TYPE_CHECKING:
    from pathlib import Path

    from sift_client.sift_types.data_import import ImportConfig

SAMPLE_ROWS = 1000

_BITS = re.compile(r"[01]+\Z")
_BASE64_URL = re.compile(r"[A-Za-z0-9_-]*={0,2}\Z")

ACCEPTED_FORMATS = "raw bytes, a string of 0s and 1s like '00110101', or URL-safe base64"


def check_bit_field_values(config: ImportConfig, path: Path) -> None:
    """Raise if a ``BIT_FIELD`` column's values aren't in a format the import service reads."""
    if isinstance(config, CsvImportConfig):
        csv_columns = {
            dc.column - 1: dc
            for dc in config.data_columns
            if dc.data_type == ChannelDataType.BIT_FIELD
        }
        if csv_columns:
            _check_csv(path, config.first_data_row, csv_columns)
        return

    parquet_columns: dict[str, DataColumnBase]
    if isinstance(config, ParquetFlatDatasetImportConfig):
        parquet_columns = {
            dc.path: dc for dc in config.data_columns if dc.data_type == ChannelDataType.BIT_FIELD
        }
    elif isinstance(config, ParquetSingleChannelPerRowImportConfig):
        sc = config.single_channel
        parquet_columns = (
            {sc.data_path: sc}
            if sc is not None and sc.data_type == ChannelDataType.BIT_FIELD
            else {}
        )
    else:
        return
    if parquet_columns:
        try:
            from sift_client._internal.util.parquet import check_bit_field_columns
        except ImportError as e:
            raise RuntimeError(
                "pyarrow is required for Parquet import. "
                "Install it via `pip install sift-stack-py[parquet]`."
            ) from e
        check_bit_field_columns(path, parquet_columns)


def _is_bit_value(value: str) -> bool:
    """Whether the import service can parse ``value`` as a bit field."""
    if _BITS.match(value):
        return True
    if len(value) % 4 or not _BASE64_URL.match(value):
        return False
    try:
        base64.urlsafe_b64decode(value)
    except (binascii.Error, ValueError):
        return False
    return True


def _check_values(dc: DataColumnBase, values: Iterable[tuple[int, str]]) -> None:
    """Check sampled ``(row, value)`` pairs from one column. Empty values are skipped."""
    for row, value in values:
        if value == "":
            continue
        if not _is_bit_value(value):
            raise ValueError(
                f"bit field column '{dc.name}' has value {value!r} in row {row}, which isn't "
                f"{ACCEPTED_FORMATS}. To convert integers, use format(value, '08b') for an "
                "8-bit status word."
            )


def _check_csv(path: Path, first_data_row: int, columns: Mapping[int, DataColumnBase]) -> None:
    samples: dict[int, list[tuple[int, str]]] = {i: [] for i in columns}
    with open(path, newline="", encoding="utf-8") as f:
        for row_number, row in enumerate(csv.reader(f), start=1):
            if row_number < first_data_row:
                continue
            if row_number >= first_data_row + SAMPLE_ROWS:
                break
            for i in columns:
                samples[i].append((row_number, row[i] if i < len(row) else ""))
    for i, dc in columns.items():
        _check_values(dc, samples[i])
