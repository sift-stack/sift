"""Unit tests for data import config models and helpers."""

from datetime import datetime, timezone

import pytest

from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    EXTENSION_TO_DATA_TYPE_KEY,
    Ch10ImportConfig,
    CsvDataColumn,
    CsvImportConfig,
    CsvTimeColumn,
    DataTypeKey,
    Hdf5DataColumn,
    Hdf5ImportConfig,
    ParquetDataColumn,
    ParquetFlatDatasetImportConfig,
    ParquetTimeColumn,
    TdmsImportConfig,
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


class TestDetectConfigValidation:
    """Tests for validation checks applied after detect_config."""

    def test_csv_no_data_columns_raises(self):
        """If all columns are filtered out, detect_config should raise."""
        config = CsvImportConfig(
            asset_name="",
            time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_RFC3339),
            data_columns=[],
        )
        assert not config.data_columns

    def test_parquet_empty_time_column_path(self):
        """An empty time column path indicates detection failed."""
        config = ParquetFlatDatasetImportConfig(
            asset_name="",
            time_column=ParquetTimeColumn(path=""),
            data_columns=[
                ParquetDataColumn(
                    path="cpu_util", name="cpu_util", data_type=ChannelDataType.DOUBLE
                ),
            ],
        )
        assert not config.time_column.path

    def test_parquet_no_data_columns(self):
        """A config with no data columns indicates detection found nothing useful."""
        config = ParquetFlatDatasetImportConfig(
            asset_name="",
            time_column=ParquetTimeColumn(path="timestamp"),
            data_columns=[],
        )
        assert not config.data_columns

    def test_parquet_integer_time_column_fallback(self):
        """An integer column starting with 'time' should be usable as the time column."""
        config = ParquetFlatDatasetImportConfig(
            asset_name="",
            time_column=ParquetTimeColumn(path=""),
            data_columns=[
                ParquetDataColumn(path="time_ns", name="time_ns", data_type=ChannelDataType.INT_64),
                ParquetDataColumn(
                    path="cpu_util", name="cpu_util", data_type=ChannelDataType.DOUBLE
                ),
            ],
        )
        _integer_types = {
            ChannelDataType.INT_32,
            ChannelDataType.INT_64,
            ChannelDataType.UINT_32,
            ChannelDataType.UINT_64,
        }
        match = None
        for dc in config.data_columns:
            if dc.data_type in _integer_types and dc.name.lower().startswith("time"):
                match = dc
                break
        assert match is not None
        assert match.path == "time_ns"


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


class TestCh10Config:
    def test_to_proto(self):
        config = Ch10ImportConfig(asset_name="my_asset", run_name="run1", scale_values=True)
        proto = config._to_proto()
        assert proto.asset_name == "my_asset"
        assert proto.run_name == "run1"
        assert proto.scale_values is True

    def test_to_proto_defaults(self):
        config = Ch10ImportConfig(asset_name="my_asset")
        proto = config._to_proto()
        assert proto.run_name == ""
        assert proto.scale_values is False

    def test_run_id_inherited_but_unused(self):
        config = Ch10ImportConfig(asset_name="my_asset")
        assert config.run_id is None


class TestTdmsConfig:
    def test_to_proto(self):
        config = TdmsImportConfig(
            asset_name="my_asset",
            run_name="run1",
            run_id="run_123",
            start_time_override=datetime(2026, 1, 1, tzinfo=timezone.utc),
            file_size=12345,
        )
        proto = config._to_proto()
        assert proto.asset_name == "my_asset"
        assert proto.run_id == "run_123"
        assert proto.file_size == 12345
        assert proto.HasField("start_time_override")

    def test_to_proto_optional_fields_unset(self):
        config = TdmsImportConfig(asset_name="my_asset", run_name="run1")
        proto = config._to_proto()
        assert proto.run_name == "run1"
        assert proto.run_id == ""
        assert not proto.HasField("start_time_override")
        assert proto.file_size == 0

    def test_run_id_takes_precedence(self):
        config = TdmsImportConfig(asset_name="a", run_name="ignored", run_id="run_123")
        proto = config._to_proto()
        assert proto.run_id == "run_123"


class TestHdf5Config:
    def test_to_proto(self):
        config = Hdf5ImportConfig(
            asset_name="my_asset",
            run_name="run1",
            time_format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
            data=[
                Hdf5DataColumn(
                    time_dataset="/time",
                    value_dataset="/voltage",
                    name="voltage",
                    data_type=ChannelDataType.DOUBLE,
                    units="V",
                    description="Voltage reading",
                ),
            ],
        )
        proto = config._to_proto()
        assert proto.asset_name == "my_asset"
        assert len(proto.data) == 1
        assert proto.data[0].time_dataset == "/time"
        assert proto.data[0].value_dataset == "/voltage"
        assert proto.data[0].channel_config.name == "voltage"
        assert proto.data[0].channel_config.units == "V"
        assert proto.data[0].channel_config.description == "Voltage reading"

    def test_to_proto_compound_fields(self):
        config = Hdf5ImportConfig(
            asset_name="my_asset",
            time_format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
            data=[
                Hdf5DataColumn(
                    time_dataset="/data",
                    value_dataset="/data",
                    name="current",
                    data_type=ChannelDataType.FLOAT,
                    time_field="ts",
                    value_field="val",
                ),
            ],
        )
        proto = config._to_proto()
        assert proto.data[0].time_field == "ts"
        assert proto.data[0].value_field == "val"

    def test_to_proto_compound_fields_unset(self):
        config = Hdf5ImportConfig(
            asset_name="my_asset",
            time_format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
            data=[
                Hdf5DataColumn(
                    time_dataset="/time",
                    value_dataset="/voltage",
                    name="voltage",
                    data_type=ChannelDataType.DOUBLE,
                ),
            ],
        )
        proto = config._to_proto()
        assert not proto.data[0].HasField("time_field")
        assert not proto.data[0].HasField("value_field")

    def test_to_proto_multiple_datasets(self):
        config = Hdf5ImportConfig(
            asset_name="my_asset",
            time_format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
            data=[
                Hdf5DataColumn(
                    time_dataset="/time",
                    value_dataset="/voltage",
                    name="voltage",
                    data_type=ChannelDataType.DOUBLE,
                ),
                Hdf5DataColumn(
                    time_dataset="/time",
                    value_dataset="/current",
                    value_index=1,
                    name="current",
                    data_type=ChannelDataType.FLOAT,
                ),
            ],
        )
        proto = config._to_proto()
        assert len(proto.data) == 2
        assert proto.data[1].value_dataset == "/current"
        assert proto.data[1].value_index == 1

    def test_relative_time_requires_start_time(self):
        with pytest.raises(ValueError, match="relative_start_time"):
            Hdf5ImportConfig(
                asset_name="my_asset",
                time_format=TimeFormat.RELATIVE_SECONDS,
                data=[],
            )

    def test_relative_time_with_start_time(self):
        config = Hdf5ImportConfig(
            asset_name="my_asset",
            time_format=TimeFormat.RELATIVE_SECONDS,
            relative_start_time=datetime(2026, 1, 1, tzinfo=timezone.utc),
            data=[],
        )
        proto = config._to_proto()
        assert proto.HasField("relative_start_time")

    def test_absolute_time_no_start_time_required(self):
        config = Hdf5ImportConfig(
            asset_name="my_asset",
            time_format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
            data=[],
        )
        assert config.relative_start_time is None
        proto = config._to_proto()
        assert not proto.HasField("relative_start_time")


class TestExtensionMap:
    def test_tdms_extension(self):
        assert EXTENSION_TO_DATA_TYPE_KEY[".tdms"] == DataTypeKey.TDMS

    def test_ch10_extension(self):
        assert EXTENSION_TO_DATA_TYPE_KEY[".ch10"] == DataTypeKey.CH10
