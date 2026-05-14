"""Unit tests for data import config models and helpers."""

from datetime import datetime, timezone

import pytest

from sift_client.resources import DataImportAPI, DataImportAPIAsync
from sift_client.resources.data_imports import _resolve_data_type_key
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    CsvDataColumn,
    CsvImportConfig,
    CsvTimeColumn,
    DataTypeKey,
    Hdf5DataColumn,
    Hdf5ImportConfig,
    ParquetDataColumn,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelPerRowImportConfig,
    ParquetTimeColumn,
    TdmsImportConfig,
    TimeFormat,
)


@pytest.mark.integration
def test_client_binding(sift_client):
    assert sift_client.data_import
    assert isinstance(sift_client.data_import, DataImportAPI)
    assert sift_client.async_.data_import
    assert isinstance(sift_client.async_.data_import, DataImportAPIAsync)


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


class TestGetItem:
    def test_csv_getitem(self, csv_config):
        col = csv_config["cpu_util"]
        assert col.name == "cpu_util"
        assert col.data_type == ChannelDataType.DOUBLE

    def test_csv_getitem_not_found(self, csv_config):
        with pytest.raises(KeyError, match="nonexistent"):
            csv_config["nonexistent"]

    def test_csv_getitem_mutate(self, csv_config):
        csv_config["status_flags"].data_type = ChannelDataType.STRING
        assert csv_config.data_columns[1].data_type == ChannelDataType.STRING

    def test_parquet_getitem(self, parquet_config):
        col = parquet_config["temperature"]
        assert col.name == "temperature"
        assert col.data_type == ChannelDataType.FLOAT

    def test_parquet_getitem_not_found(self, parquet_config):
        with pytest.raises(KeyError, match="nonexistent"):
            parquet_config["nonexistent"]

    def test_parquet_getitem_mutate(self, parquet_config):
        parquet_config["cpu_util"].name = "cpu_utilization"
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


class TestTdmsConfig:
    def test_to_proto(self):
        config = TdmsImportConfig(
            asset_name="my_asset",
            run_name="run1",
            run_id="run_123",
            start_time_override=datetime(2026, 1, 1, tzinfo=timezone.utc),
            import_file_properties=True,
        )
        proto = config._to_proto()
        assert proto.asset_name == "my_asset"
        assert proto.run_id == "run_123"
        assert proto.import_file_properties is True
        assert proto.HasField("start_time_override")

    def test_to_proto_optional_fields_unset(self):
        config = TdmsImportConfig(asset_name="my_asset", run_name="run1")
        proto = config._to_proto()
        assert proto.run_name == "run1"
        assert proto.run_id == ""
        assert not proto.HasField("start_time_override")
        assert proto.import_file_properties is False

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


class TestCsvToProto:
    def test_to_proto(self, csv_config):
        proto = csv_config._to_proto()
        assert proto.asset_name == "test_asset"
        assert proto.run_name == "test_run"
        assert proto.first_data_row == 2
        assert proto.time_column.column_number == 1
        assert len(proto.data_columns) == 3
        assert proto.data_columns[2].name == "cpu_util"

    def test_from_proto_round_trip(self, csv_config):
        proto = csv_config._to_proto()
        restored = CsvImportConfig._from_proto(proto)
        assert restored.asset_name == csv_config.asset_name
        assert restored.run_name == csv_config.run_name
        assert restored.first_data_row == csv_config.first_data_row
        assert restored.time_column.column == csv_config.time_column.column
        assert len(restored.data_columns) == len(csv_config.data_columns)


class TestParquetToProto:
    def test_flat_dataset_to_proto(self, parquet_config):
        proto = parquet_config._to_proto()
        assert proto.asset_name == "test_asset"
        assert proto.HasField("flat_dataset")
        assert proto.flat_dataset.time_column.path == "timestamp"
        assert len(proto.flat_dataset.data_columns) == 3

    def test_flat_dataset_from_proto_round_trip(self, parquet_config):
        proto = parquet_config._to_proto()
        restored = ParquetFlatDatasetImportConfig._from_proto(proto)
        assert restored.asset_name == parquet_config.asset_name
        assert restored.time_column.path == parquet_config.time_column.path
        assert len(restored.data_columns) == len(parquet_config.data_columns)
        for orig, rest in zip(parquet_config.data_columns, restored.data_columns):
            assert orig.name == rest.name
            assert orig.data_type == rest.data_type

    def test_single_channel_per_row_from_proto_round_trip(self):
        from sift_client.sift_types.data_import import ParquetSingleChannelConfig

        config = ParquetSingleChannelPerRowImportConfig(
            asset_name="a",
            time_column=ParquetTimeColumn(path="ts"),
            single_channel=ParquetSingleChannelConfig(
                data_path="value",
                name="voltage",
                data_type=ChannelDataType.DOUBLE,
            ),
        )
        proto = config._to_proto()
        restored = ParquetSingleChannelPerRowImportConfig._from_proto(proto)
        assert restored.single_channel is not None
        assert restored.single_channel.name == "voltage"
        assert restored.single_channel.data_type == ChannelDataType.DOUBLE


class TestParquetTimeColumnToProto:
    def test_empty_path_raises(self):
        col = ParquetTimeColumn(path="")
        with pytest.raises(ValueError, match="path must be set"):
            col._to_proto()


class TestResolveDataTypeKey:
    def test_parquet_requires_data_type(self):
        with pytest.raises(ValueError, match="data_type"):
            _resolve_data_type_key(".parquet", None)

    def test_parquet_with_explicit_data_type(self):
        result = _resolve_data_type_key(".parquet", DataTypeKey.PARQUET_FLATDATASET)
        assert result == DataTypeKey.PARQUET_FLATDATASET

    def test_pqt_requires_data_type(self):
        with pytest.raises(ValueError, match="data_type"):
            _resolve_data_type_key(".pqt", None)

    def test_known_extension_uses_map(self):
        assert _resolve_data_type_key(".csv", None) == DataTypeKey.CSV

    def test_explicit_data_type_overrides_extension(self):
        result = _resolve_data_type_key(".csv", DataTypeKey.TDMS)
        assert result == DataTypeKey.TDMS

    def test_unknown_extension_raises(self):
        with pytest.raises(ValueError, match="Unsupported file extension"):
            _resolve_data_type_key(".xyz", None)
