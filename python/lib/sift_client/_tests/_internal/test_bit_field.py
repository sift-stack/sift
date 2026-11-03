"""Tests for converting integer bit field columns before upload."""

from __future__ import annotations

import os
import shutil
import tempfile

import pyarrow as pa
import pyarrow.parquet as pq
import pytest

from sift_client._internal.util.bit_field import convert_integer_bit_fields
from sift_client._internal.util.file import extract_parquet_footer
from sift_client.sift_types.channel import ChannelBitFieldElement, ChannelDataType
from sift_client.sift_types.data_import import (
    CsvDataColumn,
    CsvImportConfig,
    CsvTimeColumn,
    ParquetDataColumn,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelConfig,
    ParquetSingleChannelPerRowImportConfig,
    ParquetTimeColumn,
    TimeFormat,
)

ONE_BYTE = [
    ChannelBitFieldElement(name="armed", index=0, bit_count=1),
    ChannelBitFieldElement(name="mode", index=4, bit_count=4),
]
TWO_BYTES = [
    ChannelBitFieldElement(name="low", index=0, bit_count=8),
    ChannelBitFieldElement(name="high", index=8, bit_count=8),
]
CHANNEL_CONFIG_KEY = b"sift_channel_config"


@pytest.fixture
def converted_dirs():
    """Remove the temporary directories conversions create."""
    dirs = []
    yield dirs
    for d in dirs:
        shutil.rmtree(d)


def _convert(config, path, converted_dirs):
    result = convert_integer_bit_fields(config, path)
    assert result is not None
    converted_dirs.append(result[1].parent)
    return result


def _parquet_config(path, column="status", elements=ONE_BYTE):
    footer, offset = extract_parquet_footer(path)
    return ParquetFlatDatasetImportConfig(
        asset_name="a",
        time_column=ParquetTimeColumn(path="t", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
        data_columns=[
            ParquetDataColumn(
                path=column,
                name=column,
                data_type=ChannelDataType.BIT_FIELD,
                bit_field_elements=elements,
            ),
            ParquetDataColumn(path="x", name="x", data_type=ChannelDataType.DOUBLE),
        ],
        footer_offset=offset,
        footer_length=len(footer),
    )


def _write_parquet(path, status: pa.Array, **kwargs):
    rows = len(status)
    pq.write_table(
        pa.table(
            {
                "t": pa.array(range(rows), pa.int64()),
                "status": status,
                "x": pa.array([float(i) for i in range(rows)]),
            }
        ),
        path,
        **kwargs,
    )


class TestParquet:
    @pytest.mark.parametrize(
        ("arrow_type", "values", "expected"),
        [
            pytest.param(pa.uint8(), [53, 0, 255], [b"\x35", b"\x00", b"\xff"], id="uint8"),
            pytest.param(pa.int8(), [-1, 5], [b"\xff", b"\x05"], id="int8-twos-complement"),
            pytest.param(
                pa.uint16(), [0x1234, 1], [b"\x34\x12", b"\x01\x00"], id="uint16-little-endian"
            ),
            pytest.param(pa.int64(), [-2], [b"\xfe" + b"\xff" * 7], id="int64"),
        ],
    )
    def test_integers_become_little_endian_bytes(
        self, tmp_path, converted_dirs, arrow_type, values, expected
    ):
        path = tmp_path / "data.parquet"
        _write_parquet(path, pa.array(values, arrow_type))

        _, out = _convert(_parquet_config(path), path, converted_dirs)
        table = pq.read_table(out)
        assert table.schema.field("status").type == pa.binary()
        assert table.column("status").to_pylist() == expected
        assert table.column("x").to_pylist() == pq.read_table(path).column("x").to_pylist()

    def test_nulls_are_kept(self, tmp_path, converted_dirs):
        path = tmp_path / "data.parquet"
        _write_parquet(path, pa.array([1, None, 3], pa.uint8()))

        _, out = _convert(_parquet_config(path), path, converted_dirs)

        assert pq.read_table(out).column("status").to_pylist() == [b"\x01", None, b"\x03"]

    def test_footer_is_recomputed_on_a_copy_of_the_config(self, tmp_path, converted_dirs):
        path = tmp_path / "data.parquet"
        _write_parquet(path, pa.array([1, 2, 3], pa.uint8()))
        config = _parquet_config(path)
        original = (config.footer_offset, config.footer_length)

        upload_config, out = _convert(config, path, converted_dirs)

        footer, offset = extract_parquet_footer(out)
        assert (upload_config.footer_offset, upload_config.footer_length) == (offset, len(footer))
        assert (config.footer_offset, config.footer_length) == original
        assert out.name == path.name
        assert out.parent != path.parent

    def test_layout_and_metadata_are_kept(self, tmp_path, converted_dirs):
        path = tmp_path / "data.parquet"
        schema = pa.schema(
            [
                pa.field("t", pa.int64()),
                pa.field(
                    "status",
                    pa.uint8(),
                    metadata={CHANNEL_CONFIG_KEY: b"{}", b"keep": b"yes"},
                ),
                pa.field("x", pa.float64()),
            ],
            metadata={b"origin": b"test"},
        )
        table = pa.table(
            {"t": list(range(30)), "status": [i % 256 for i in range(30)], "x": [0.0] * 30},
            schema=schema,
        )
        pq.write_table(table, path, row_group_size=10, compression="zstd")

        _, out = _convert(_parquet_config(path), path, converted_dirs)

        converted = pq.ParquetFile(out)
        assert converted.num_row_groups == 3
        assert {
            converted.metadata.row_group(0).column(c).compression
            for c in range(converted.metadata.num_columns)
        } == {"ZSTD"}
        assert converted.schema_arrow.metadata == {b"origin": b"test"}
        # The stale embedded channel config is dropped; other field metadata stays.
        assert converted.schema_arrow.field("status").metadata == {b"keep": b"yes"}

    @pytest.mark.parametrize(
        "status",
        [
            pytest.param(pa.array([b"\x01"], pa.binary()), id="binary"),
            pytest.param(pa.array(["AQ=="], pa.string()), id="base64-string"),
        ],
    )
    def test_non_integer_columns_are_uploaded_as_is(self, tmp_path, status):
        path = tmp_path / "data.parquet"
        _write_parquet(path, status)

        assert convert_integer_bit_fields(_parquet_config(path), path) is None

    def test_no_bit_field_columns_skips_reading_the_file(self, tmp_path):
        path = tmp_path / "not-parquet.parquet"
        path.write_bytes(b"not a parquet file")
        config = ParquetFlatDatasetImportConfig(
            asset_name="a",
            time_column=ParquetTimeColumn(path="t", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
            data_columns=[ParquetDataColumn(path="x", name="x", data_type=ChannelDataType.DOUBLE)],
        )

        assert convert_integer_bit_fields(config, path) is None

    def test_elements_wider_than_the_integer_raise(self, tmp_path):
        path = tmp_path / "data.parquet"
        _write_parquet(path, pa.array([1], pa.uint8()))

        with pytest.raises(ValueError, match="span 16 bits"):
            convert_integer_bit_fields(_parquet_config(path, elements=TWO_BYTES), path)

    def test_nested_integer_column_raises(self, tmp_path):
        path = tmp_path / "data.parquet"
        pq.write_table(
            pa.table(
                {
                    "t": pa.array([0], pa.int64()),
                    "s": pa.array([{"status": 1}], pa.struct([("status", pa.uint8())])),
                    "x": pa.array([0.0]),
                }
            ),
            path,
        )

        with pytest.raises(ValueError, match="nested integer"):
            convert_integer_bit_fields(_parquet_config(path, column="s|status"), path)

    def test_unreadable_file_raises(self, tmp_path):
        path = tmp_path / "data.parquet"
        path.write_bytes(b"not a parquet file")

        with pytest.raises(ValueError, match="failed to read data.parquet"):
            convert_integer_bit_fields(_parquet_config_without_footer(), path)

    def test_single_channel_per_row(self, tmp_path, converted_dirs):
        path = tmp_path / "data.parquet"
        pq.write_table(
            pa.table({"t": pa.array([0, 1], pa.int64()), "value": pa.array([2, 3], pa.uint8())}),
            path,
        )
        config = ParquetSingleChannelPerRowImportConfig(
            asset_name="a",
            time_column=ParquetTimeColumn(path="t", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
            single_channel=ParquetSingleChannelConfig(
                data_path="value",
                name="status",
                data_type=ChannelDataType.BIT_FIELD,
                bit_field_elements=ONE_BYTE,
            ),
        )

        _, out = _convert(config, path, converted_dirs)

        assert pq.read_table(out).column("value").to_pylist() == [b"\x02", b"\x03"]


def _parquet_config_without_footer():
    return ParquetFlatDatasetImportConfig(
        asset_name="a",
        time_column=ParquetTimeColumn(path="t", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
        data_columns=[
            ParquetDataColumn(
                path="status",
                name="status",
                data_type=ChannelDataType.BIT_FIELD,
                bit_field_elements=ONE_BYTE,
            )
        ],
    )


def _csv_config(elements=ONE_BYTE, first_data_row=2):
    return CsvImportConfig(
        asset_name="a",
        first_data_row=first_data_row,
        time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_RFC3339),
        data_columns=[
            CsvDataColumn(
                column=2,
                name="status",
                data_type=ChannelDataType.BIT_FIELD,
                bit_field_elements=elements,
            ),
            CsvDataColumn(column=3, name="x", data_type=ChannelDataType.DOUBLE),
        ],
    )


class TestCsv:
    def test_integers_become_fixed_width_bit_strings(self, tmp_path, converted_dirs):
        path = tmp_path / "data.csv"
        path.write_text("time,status,x\nt0,53,1.5\nt1,0,2\nt2,,3\nt3,-1,4\n")

        config, out = _convert(_csv_config(), path, converted_dirs)

        assert out.read_text() == (
            "time,status,x\nt0,00110101,1.5\nt1,00000000,2\nt2,,3\nt3,11111111,4\n"
        )
        assert path.read_text().startswith("time,status,x\nt0,53,")

    def test_width_follows_the_elements(self, tmp_path, converted_dirs):
        path = tmp_path / "data.csv"
        path.write_text("time,status,x\nt0,258,1\n")

        _, out = _convert(_csv_config(elements=TWO_BYTES), path, converted_dirs)

        assert out.read_text() == "time,status,x\nt0,0000000100000010,1\n"

    def test_rows_before_the_first_data_row_are_kept(self, tmp_path, converted_dirs):
        path = tmp_path / "data.csv"
        path.write_text("units,raw,m\ntime,status,x\nt0,5,1\n")

        _, out = _convert(_csv_config(first_data_row=3), path, converted_dirs)

        assert out.read_text() == "units,raw,m\ntime,status,x\nt0,00000101,1\n"

    @pytest.mark.parametrize(
        "values",
        [
            pytest.param(["00110101", "00000000"], id="bit-strings"),
            pytest.param(["0", "1", "1"], id="single-bits"),
            pytest.param(["NQ==", "AA=="], id="base64"),
        ],
    )
    def test_encoded_values_are_uploaded_as_is(self, tmp_path, values):
        path = tmp_path / "data.csv"
        path.write_text("time,status,x\n" + "".join(f"t,{v},1\n" for v in values))

        assert convert_integer_bit_fields(_csv_config(), path) is None

    def test_ambiguous_bit_strings_raise(self, tmp_path):
        path = tmp_path / "data.csv"
        path.write_text("time,status,x\nt0,0,1\nt1,10,1\nt2,11,1\n")

        with pytest.raises(ValueError, match="can't tell whether"):
            convert_integer_bit_fields(_csv_config(), path)

    def test_value_too_large_raises(self, tmp_path):
        path = tmp_path / "data.csv"
        path.write_text("time,status,x\nt0,256,1\n")

        with pytest.raises(ValueError, match="value 256 in row 2"):
            convert_integer_bit_fields(_csv_config(), path)

    def test_failed_conversion_leaves_no_temp_dir(self, tmp_path, monkeypatch):
        created = []
        real_mkdtemp = tempfile.mkdtemp

        def tracking_mkdtemp(*args, **kwargs):
            d = real_mkdtemp(*args, **kwargs)
            created.append(d)
            return d

        monkeypatch.setattr(tempfile, "mkdtemp", tracking_mkdtemp)
        path = tmp_path / "data.csv"
        path.write_text("time,status,x\nt0,5,1\nt1,300,1\n")

        with pytest.raises(ValueError, match="value 300 in row 3"):
            convert_integer_bit_fields(_csv_config(), path)

        assert created
        assert not any(os.path.exists(d) for d in created)
