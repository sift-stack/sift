"""Unit tests for data import config models and helpers."""

from datetime import datetime, timezone

import pytest

from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    EXTENSION_TO_DATA_TYPE_KEY,
    CsvDataColumn,
    CsvImportConfig,
    CsvTimeColumn,
    DataTypeKey,
    ParquetDataColumn,
    ParquetFlatDatasetImportConfig,
    ParquetTimeColumn,
    TimeFormat,
)


@pytest.fixture
def csv_config():
    return CsvImportConfig(
        asset_name="test_asset",
        run_name="test_run",
        time_column=CsvTimeColumn(
            column=1,
            format=TimeFormat.ABSOLUTE_RFC3339,
        ),
        data_columns=[
            CsvDataColumn(column=2, name="cpu_util", data_type=ChannelDataType.DOUBLE),
            CsvDataColumn(column=3, name="status_flags", data_type=ChannelDataType.INT_32),
            CsvDataColumn(column=4, name="temperature", data_type=ChannelDataType.FLOAT),
        ],
    )


@pytest.fixture
def parquet_config():
    return ParquetFlatDatasetImportConfig(
        asset_name="test_asset",
        run_name="test_run",
        time_column=ParquetTimeColumn(path="timestamp"),
        data_columns=[
            ParquetDataColumn(path="cpu_util", name="cpu_util", data_type=ChannelDataType.DOUBLE),
            ParquetDataColumn(
                path="status_flags", name="status_flags", data_type=ChannelDataType.INT_32
            ),
            ParquetDataColumn(
                path="temperature", name="temperature", data_type=ChannelDataType.FLOAT
            ),
        ],
    )


class TestCsvConfigMutability:
    def test_mutate_asset_name(self, csv_config):
        csv_config.asset_name = "new_asset"
        assert csv_config.asset_name == "new_asset"

    def test_mutate_run_name(self, csv_config):
        csv_config.run_name = "new_run"
        assert csv_config.run_name == "new_run"

    def test_mutate_column_data_type(self, csv_config):
        csv_config.data_columns[1].data_type = ChannelDataType.STRING
        assert csv_config.data_columns[1].data_type == ChannelDataType.STRING

    def test_mutate_column_name(self, csv_config):
        csv_config.data_columns[0].name = "cpu_utilization"
        assert csv_config.data_columns[0].name == "cpu_utilization"

    def test_append_column(self, csv_config):
        csv_config.data_columns.append(
            CsvDataColumn(column=5, name="pressure", data_type=ChannelDataType.DOUBLE)
        )
        assert len(csv_config.data_columns) == 4
        assert csv_config.data_columns[-1].name == "pressure"

    def test_remove_column(self, csv_config):
        csv_config.data_columns = [
            dc for dc in csv_config.data_columns if dc.name != "status_flags"
        ]
        assert len(csv_config.data_columns) == 2
        assert all(dc.name != "status_flags" for dc in csv_config.data_columns)


class TestParquetConfigMutability:
    def test_mutate_asset_name(self, parquet_config):
        parquet_config.asset_name = "new_asset"
        assert parquet_config.asset_name == "new_asset"

    def test_mutate_column_data_type(self, parquet_config):
        parquet_config.data_columns[1].data_type = ChannelDataType.STRING
        assert parquet_config.data_columns[1].data_type == ChannelDataType.STRING

    def test_append_column(self, parquet_config):
        parquet_config.data_columns.append(
            ParquetDataColumn(path="pressure", name="pressure", data_type=ChannelDataType.DOUBLE)
        )
        assert len(parquet_config.data_columns) == 4


class TestGetColumn:
    def test_csv_get_column(self, csv_config):
        col = csv_config.get_column("cpu_util")
        assert col.name == "cpu_util"
        assert col.data_type == ChannelDataType.DOUBLE

    def test_csv_get_column_not_found(self, csv_config):
        with pytest.raises(KeyError, match="nonexistent"):
            csv_config.get_column("nonexistent")

    def test_csv_get_column_mutate(self, csv_config):
        csv_config.get_column("status_flags").data_type = ChannelDataType.STRING
        assert csv_config.data_columns[1].data_type == ChannelDataType.STRING

    def test_parquet_get_column(self, parquet_config):
        col = parquet_config.get_column("temperature")
        assert col.name == "temperature"
        assert col.data_type == ChannelDataType.FLOAT

    def test_parquet_get_column_not_found(self, parquet_config):
        with pytest.raises(KeyError, match="nonexistent"):
            parquet_config.get_column("nonexistent")

    def test_parquet_get_column_mutate(self, parquet_config):
        parquet_config.get_column("cpu_util").name = "cpu_utilization"
        assert parquet_config.data_columns[0].name == "cpu_utilization"


class TestTimeColumnValidation:
    def test_csv_relative_time_requires_start_time(self):
        with pytest.raises(ValueError, match="relative_start_time"):
            CsvTimeColumn(
                column=1,
                format=TimeFormat.RELATIVE_NANOSECONDS,
            )

    def test_csv_relative_time_with_start_time(self):
        col = CsvTimeColumn(
            column=1,
            format=TimeFormat.RELATIVE_NANOSECONDS,
            relative_start_time=datetime(2026, 1, 1, tzinfo=timezone.utc),
        )
        assert col.relative_start_time is not None

    def test_parquet_relative_time_requires_start_time(self):
        with pytest.raises(ValueError, match="relative_start_time"):
            ParquetTimeColumn(
                path="timestamp",
                format=TimeFormat.RELATIVE_SECONDS,
            )

    def test_parquet_relative_time_with_start_time(self):
        col = ParquetTimeColumn(
            path="timestamp",
            format=TimeFormat.RELATIVE_SECONDS,
            relative_start_time=datetime(2026, 1, 1, tzinfo=timezone.utc),
        )
        assert col.relative_start_time is not None

    def test_absolute_time_does_not_require_start_time(self):
        col = CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_RFC3339)
        assert col.relative_start_time is None


class TestDataTypeKey:
    def test_csv_extension(self):
        assert EXTENSION_TO_DATA_TYPE_KEY[".csv"] == DataTypeKey.CSV

    def test_parquet_not_in_extension_map(self):
        assert ".parquet" not in EXTENSION_TO_DATA_TYPE_KEY

    def test_hdf5_extensions(self):
        assert EXTENSION_TO_DATA_TYPE_KEY[".h5"] == DataTypeKey.HDF5
        assert EXTENSION_TO_DATA_TYPE_KEY[".hdf5"] == DataTypeKey.HDF5


class TestRunPrecedence:
    def test_run_id_ignored_when_none(self, csv_config):
        csv_config.run_id = None
        csv_config.run_name = "my_run"
        proto = csv_config._to_proto()
        assert proto.run_name == "my_run"
        assert proto.run_id == ""

    def test_run_id_set(self, csv_config):
        csv_config.run_id = "run_123"
        csv_config.run_name = "ignored"
        proto = csv_config._to_proto()
        assert proto.run_id == "run_123"
