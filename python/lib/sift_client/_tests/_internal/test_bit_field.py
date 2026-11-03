"""Tests for checking bit field values before upload."""

from __future__ import annotations

import pyarrow as pa
import pyarrow.parquet as pq
import pytest

from sift_client._internal.util.bit_field import SAMPLE_ROWS, check_bit_field_values
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

ELEMENTS = [
    ChannelBitFieldElement(name="armed", index=0, bit_count=1),
    ChannelBitFieldElement(name="mode", index=4, bit_count=4),
]


def _csv_config(first_data_row=2):
    return CsvImportConfig(
        asset_name="a",
        first_data_row=first_data_row,
        time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_RFC3339),
        data_columns=[
            CsvDataColumn(
                column=2,
                name="status",
                data_type=ChannelDataType.BIT_FIELD,
                bit_field_elements=ELEMENTS,
            ),
            CsvDataColumn(column=3, name="x", data_type=ChannelDataType.DOUBLE),
        ],
    )


def _write_csv(path, values):
    path.write_text("time,status,x\n" + "".join(f"t,{v},1.5\n" for v in values))


class TestCsv:
    @pytest.mark.parametrize(
        "values",
        [
            pytest.param(["00110101", "00000000", "11111111"], id="bit-strings"),
            pytest.param(["0", "1", "1"], id="single-bits"),
            pytest.param(["NQ==", "AA==", "_w=="], id="base64"),
            pytest.param(["00110101", "", "00000001"], id="empty-cells"),
            pytest.param(["0", "1", "10", "11"], id="unpadded-bit-strings"),
            pytest.param(["1234", "2048"], id="digit-only-base64"),
        ],
    )
    def test_accepted_values_pass(self, tmp_path, values):
        path = tmp_path / "data.csv"
        _write_csv(path, values)

        check_bit_field_values(_csv_config(), path)

    def test_unparseable_value_raises_with_row(self, tmp_path):
        path = tmp_path / "data.csv"
        _write_csv(path, ["00110101", "53"])

        with pytest.raises(ValueError, match=r"value '53' in row 3"):
            check_bit_field_values(_csv_config(), path)

    def test_rows_before_first_data_row_are_skipped(self, tmp_path):
        path = tmp_path / "data.csv"
        path.write_text("units,raw,m\ntime,status,x\nt,00000101,1\n")

        check_bit_field_values(_csv_config(first_data_row=3), path)

    def test_only_the_sample_is_read(self, tmp_path):
        path = tmp_path / "data.csv"
        _write_csv(path, ["00000001"] * SAMPLE_ROWS + ["53"])

        check_bit_field_values(_csv_config(), path)

    def test_no_bit_field_columns_skips_reading_the_file(self, tmp_path):
        config = CsvImportConfig(
            asset_name="a",
            time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_RFC3339),
            data_columns=[CsvDataColumn(column=2, name="x", data_type=ChannelDataType.DOUBLE)],
        )

        check_bit_field_values(config, tmp_path / "missing.csv")


def _parquet_config(column="status"):
    return ParquetFlatDatasetImportConfig(
        asset_name="a",
        time_column=ParquetTimeColumn(path="t", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
        data_columns=[
            ParquetDataColumn(
                path=column,
                name="status",
                data_type=ChannelDataType.BIT_FIELD,
                bit_field_elements=ELEMENTS,
            ),
            ParquetDataColumn(path="x", name="x", data_type=ChannelDataType.DOUBLE),
        ],
    )


def _write_parquet(path, status: pa.Array):
    rows = len(status)
    pq.write_table(
        pa.table(
            {
                "t": pa.array(range(rows), pa.int64()),
                "status": status,
                "x": pa.array([0.0] * rows),
            }
        ),
        path,
    )


class TestParquet:
    @pytest.mark.parametrize(
        "status",
        [
            pytest.param(pa.array([b"\x35", None], pa.binary()), id="binary"),
            pytest.param(pa.array([b"\x35"], pa.large_binary()), id="large-binary"),
            pytest.param(pa.array([b"\x35"], pa.binary(1)), id="fixed-size-binary"),
            pytest.param(pa.array(["00110101", None], pa.string()), id="bit-string"),
            pytest.param(pa.array(["NQ=="], pa.large_string()), id="base64"),
        ],
    )
    def test_accepted_types_pass(self, tmp_path, status):
        path = tmp_path / "data.parquet"
        _write_parquet(path, status)

        check_bit_field_values(_parquet_config(), path)

    @pytest.mark.parametrize(
        ("status", "type_name"),
        [
            pytest.param(pa.array([53], pa.uint8()), "uint8", id="uint8"),
            pytest.param(pa.array([53], pa.int64()), "int64", id="int64"),
            pytest.param(pa.array([[53]], pa.list_(pa.uint8())), "list<", id="list"),
            pytest.param(pa.array([True], pa.bool_()), "bool", id="bool"),
        ],
    )
    def test_other_types_raise(self, tmp_path, status, type_name):
        path = tmp_path / "data.parquet"
        _write_parquet(path, status)

        with pytest.raises(ValueError, match=rf"'status' is {type_name}"):
            check_bit_field_values(_parquet_config(), path)

    def test_unparseable_string_raises(self, tmp_path):
        path = tmp_path / "data.parquet"
        _write_parquet(path, pa.array(["00110101", "not bits"], pa.string()))

        with pytest.raises(ValueError, match=r"value 'not bits' in row 2"):
            check_bit_field_values(_parquet_config(), path)

    def test_nested_field_type_is_checked(self, tmp_path):
        path = tmp_path / "data.parquet"
        pq.write_table(
            pa.table(
                {
                    "t": pa.array([0], pa.int64()),
                    "s": pa.array([{"status": 53}], pa.struct([("status", pa.uint8())])),
                    "x": pa.array([0.0]),
                }
            ),
            path,
        )

        with pytest.raises(ValueError, match="'status' is uint8"):
            check_bit_field_values(_parquet_config(column="s|status"), path)

    def test_nested_string_values_are_sampled(self, tmp_path):
        path = tmp_path / "data.parquet"
        pq.write_table(
            pa.table(
                {
                    "t": pa.array([0], pa.int64()),
                    "s": pa.array([{"status": "53"}], pa.struct([("status", pa.string())])),
                    "x": pa.array([0.0]),
                }
            ),
            path,
        )

        with pytest.raises(ValueError, match="value '53' in row 1"):
            check_bit_field_values(_parquet_config(column="s|status"), path)

    def test_missing_column_is_left_to_the_server(self, tmp_path):
        path = tmp_path / "data.parquet"
        _write_parquet(path, pa.array([b"\x35"], pa.binary()))

        check_bit_field_values(_parquet_config(column="absent"), path)

    def test_single_channel_per_row(self, tmp_path):
        path = tmp_path / "data.parquet"
        pq.write_table(
            pa.table({"t": pa.array([0], pa.int64()), "value": pa.array([2], pa.uint8())}), path
        )
        config = ParquetSingleChannelPerRowImportConfig(
            asset_name="a",
            time_column=ParquetTimeColumn(path="t", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
            single_channel=ParquetSingleChannelConfig(
                data_path="value",
                name="status",
                data_type=ChannelDataType.BIT_FIELD,
                bit_field_elements=ELEMENTS,
            ),
        )

        with pytest.raises(ValueError, match="'status' is uint8"):
            check_bit_field_values(config, path)
