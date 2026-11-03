"""Convert integer bit field columns to an encoding the import service accepts.

The import service reads bit field values as raw bytes, a string of 0s and 1s, or
base64, and rejects integers. Status words are usually stored as integers, so when
a ``BIT_FIELD`` column holds integers the client uploads a converted copy of the
file instead. The original file and config are never changed.
"""

from __future__ import annotations

import csv
import re
import shutil
import tempfile
from pathlib import Path
from typing import TYPE_CHECKING, Iterable, Mapping, Union

from sift_client._internal.util.file import extract_parquet_footer
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    CsvImportConfig,
    DataColumnBase,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelPerRowImportConfig,
)

if TYPE_CHECKING:
    import pyarrow as pa

    from sift_client.sift_types.data_import import ImportConfig

ParquetImportConfig = Union[ParquetFlatDatasetImportConfig, ParquetSingleChannelPerRowImportConfig]

# Matches the separator and metadata key in ``_internal/util/parquet.py``, which
# imports pyarrow at module load and so can't be imported here unconditionally.
_PATH_SEPARATOR = "|"
_EMBEDDED_CHANNEL_CONFIG_KEY = b"sift_channel_config"

_DECIMAL = re.compile(r"-?[0-9]+\Z")
_BITS = re.compile(r"[01]+\Z")


def convert_integer_bit_fields(
    config: ImportConfig, path: Path
) -> tuple[ImportConfig, Path] | None:
    """Return a config and a converted copy of ``path`` to upload instead of the originals.

    Returns ``None`` when no ``BIT_FIELD`` column holds integers, so the file is
    uploaded as is. The converted copy is written to a new temporary directory,
    which the caller removes after the upload.
    """
    if isinstance(config, CsvImportConfig):
        columns = {
            dc.column - 1: dc
            for dc in config.data_columns
            if dc.data_type == ChannelDataType.BIT_FIELD
        }
        if not columns:
            return None
        _validate(columns.values())
        to_convert = _csv_columns_to_convert(path, config.first_data_row, columns)
        if not to_convert:
            return None
        return config, _write_converted(path, lambda out: _write_csv(path, out, config, to_convert))

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
        return None
    if not parquet_columns:
        return None
    _validate(parquet_columns.values())
    return _convert_parquet(config, path, parquet_columns)


def _validate(columns: Iterable[DataColumnBase]) -> None:
    # Catch a bad layout here instead of failing on an empty or invalid element
    # list partway through a conversion.
    for dc in columns:
        dc._validate_channel_type_fields()


def _bits_used(dc: DataColumnBase) -> int:
    assert dc.bit_field_elements
    return max(el.index + el.bit_count for el in dc.bit_field_elements)


def _write_converted(path: Path, write) -> Path:
    out = Path(tempfile.mkdtemp(prefix="sift-import-")) / path.name
    try:
        write(out)
    except BaseException:
        shutil.rmtree(out.parent, ignore_errors=True)
        raise
    return out


def _csv_columns_to_convert(
    path: Path, first_data_row: int, columns: Mapping[int, DataColumnBase]
) -> dict[int, int]:
    """Return the byte width to convert at for each CSV column that holds integers.

    A column is converted when every value is a decimal integer and at least one
    isn't also a valid bit string. Bit strings and base64 are sent unchanged.
    """
    all_decimal = dict.fromkeys(columns, True)
    has_integer = dict.fromkeys(columns, False)
    bit_widths: dict[int, set[int]] = {i: set() for i in columns}
    with open(path, newline="", encoding="utf-8") as f:
        for row_number, row in enumerate(csv.reader(f), start=1):
            if row_number < first_data_row:
                continue
            for i in columns:
                cell = row[i].strip() if i < len(row) else ""
                if not cell:
                    continue
                if not _DECIMAL.match(cell):
                    all_decimal[i] = False
                elif _BITS.match(cell):
                    bit_widths[i].add(len(cell))
                else:
                    has_integer[i] = True

    to_convert = {}
    for i, dc in columns.items():
        if not all_decimal[i]:
            continue
        if has_integer[i]:
            to_convert[i] = (_bits_used(dc) + 7) // 8
        elif len(bit_widths[i]) > 1:
            raise ValueError(
                f"can't tell whether bit field column '{dc.name}' holds integers or bit "
                "strings: it only has 0s and 1s, in values of different lengths, so a value "
                "like '10' reads as either 10 or 2. Write the column as fixed-width bit "
                "strings, like '00001010', before importing."
            )
    return to_convert


def _csv_cell(cell: str, width: int, dc: DataColumnBase, row_number: int) -> str:
    if not cell.strip():
        return cell
    value = int(cell)
    bits = width * 8
    if value < 0:
        value += 1 << bits
    if not 0 <= value < 1 << bits:
        raise ValueError(
            f"value {cell.strip()} in row {row_number} of bit field column '{dc.name}' "
            f"doesn't fit in the {bits} bits its elements span."
        )
    return format(value, f"0{bits}b")


def _write_csv(src: Path, out: Path, config: CsvImportConfig, to_convert: dict[int, int]) -> None:
    columns = {dc.column - 1: dc for dc in config.data_columns}
    with open(src, newline="", encoding="utf-8") as fin, open(
        out, "w", newline="", encoding="utf-8"
    ) as fout:
        writer = csv.writer(fout, lineterminator="\n")
        for row_number, row in enumerate(csv.reader(fin), start=1):
            if row_number >= config.first_data_row:
                for i, width in to_convert.items():
                    if i < len(row):
                        row[i] = _csv_cell(row[i], width, columns[i], row_number)
            writer.writerow(row)


def _convert_parquet(
    config: ParquetImportConfig, path: Path, columns: Mapping[str, DataColumnBase]
) -> tuple[ImportConfig, Path] | None:
    try:
        import pyarrow as pa
        import pyarrow.parquet as pq
    except ImportError as e:
        raise RuntimeError(
            "pyarrow is required for Parquet import. "
            "Install it via `pip install sift-stack-py[parquet]`."
        ) from e

    try:
        pf = pq.ParquetFile(path)
    except Exception as e:
        raise ValueError(
            f"failed to read {path.name} to check its bit field columns. Encrypted "
            "Parquet files aren't supported; store bit field columns as binary instead."
        ) from e
    schema = pf.schema_arrow

    to_convert: dict[int, int] = {}
    for column_path, dc in columns.items():
        index = schema.get_field_index(column_path)
        if index == -1:
            leaf = _nested_leaf_type(schema, column_path)
            if leaf is not None and pa.types.is_integer(leaf):
                raise ValueError(
                    f"bit field column '{dc.name}' is a nested integer field, which isn't "
                    "converted automatically. Store it as binary before importing."
                )
            continue
        field_type = schema.field(index).type
        if not pa.types.is_integer(field_type):
            continue
        if _bits_used(dc) > field_type.bit_width:
            raise ValueError(
                f"bit field elements of '{dc.name}' span {_bits_used(dc)} bits, but the "
                f"column is {field_type} ({field_type.bit_width} bits)."
            )
        to_convert[index] = field_type.bit_width // 8
    if not to_convert:
        return None

    new_schema = schema
    for index in to_convert:
        old = schema.field(index)
        # An embedded config still describes the old integer type. The config sent
        # with the import is authoritative, so drop it from the converted column.
        metadata = {
            k: v for k, v in (old.metadata or {}).items() if k != _EMBEDDED_CHANNEL_CONFIG_KEY
        }
        new_schema = new_schema.set(
            index, pa.field(old.name, pa.binary(), nullable=old.nullable, metadata=metadata or None)
        )

    codecs = (
        {
            pf.metadata.row_group(0).column(c).compression.lower()
            for c in range(pf.metadata.num_columns)
        }
        if pf.num_row_groups
        else set()
    )
    compression = codecs.pop() if len(codecs) == 1 else "snappy"

    def write(out: Path) -> None:
        # Write each source row group as its own row group so the layout, and the
        # footer that describes it, stays the same apart from the converted columns.
        with pq.ParquetWriter(out, new_schema, compression=compression) as writer:
            for rg in range(pf.num_row_groups):
                table = pf.read_row_group(rg)
                for index, width in to_convert.items():
                    table = table.set_column(
                        index,
                        new_schema.field(index),
                        _integers_to_bytes(table.column(index), width),
                    )
                writer.write_table(
                    table.replace_schema_metadata(new_schema.metadata),
                    row_group_size=max(table.num_rows, 1),
                )

    out = _write_converted(path, write)
    upload_config = config.model_copy(deep=True)
    # Footer values from detect_config point into the original file.
    footer_bytes, footer_offset = extract_parquet_footer(out)
    upload_config.footer_offset = footer_offset
    upload_config.footer_length = len(footer_bytes)
    return upload_config, out


def _nested_leaf_type(schema: pa.Schema, column_path: str) -> pa.DataType | None:
    import pyarrow as pa

    parts = column_path.split(_PATH_SEPARATOR)
    index = schema.get_field_index(parts[0])
    if index == -1:
        return None
    field_type = schema.field(index).type
    for part in parts[1:]:
        if not pa.types.is_struct(field_type):
            return None
        child = field_type.get_field_index(part)
        if child == -1:
            return None
        field_type = field_type.field(child).type
    return field_type


def _integers_to_bytes(column: pa.ChunkedArray, width: int) -> pa.Array:
    """Encode integers as little-endian bytes, ``width`` bytes each, keeping nulls."""
    import numpy as np
    import pyarrow as pa
    import pyarrow.compute as pc

    values = column.combine_chunks()
    # Signed values keep their two's complement bit pattern.
    raw = pc.fill_null(values, 0).to_numpy(zero_copy_only=False)
    little_endian = np.ascontiguousarray(raw, dtype=raw.dtype.newbyteorder("<"))
    encoded = pa.FixedSizeBinaryArray.from_buffers(
        pa.binary(width), len(little_endian), [None, pa.py_buffer(little_endian.tobytes())]
    ).cast(pa.binary())
    if values.null_count:
        # pyarrow's stubs don't declare its generated compute functions.
        encoded = pc.if_else(values.is_valid(), encoded, pa.scalar(None, pa.binary()))  # type: ignore[attr-defined]
    return encoded
