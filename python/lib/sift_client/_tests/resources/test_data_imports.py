"""Unit tests for data import config models and helpers."""

from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING, cast
from unittest.mock import AsyncMock, MagicMock, patch

import pytest
from sift.common.type.v1.channel_config_pb2 import ChannelConfig as ChannelConfigProto
from sift.data_imports.v2.data_imports_pb2 import (
    DATA_IMPORT_STATUS_FAILED,
    DATA_IMPORT_STATUS_SUCCEEDED,
    DATA_IMPORT_STATUS_UNSPECIFIED,
    ParquetColumn,
    ParquetConfig,
    ParquetFlatDatasetConfig,
    ParquetSingleChannelPerRowConfig,
)
from sift.data_imports.v2.data_imports_pb2 import DataImport as DataImportProto
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetDataColumn as ParquetDataColumnProto,
)
from sift.data_imports.v2.data_imports_pb2 import (
    ParquetTimeColumn as ParquetTimeColumnProto,
)

from sift_client.resources import DataImportAPI, DataImportAPIAsync
from sift_client.resources.data_imports import (
    _infer_time_column,
    _parse_parquet_detect_response,
    _resolve_data_type_key,
)
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    CsvDataColumn,
    CsvImportConfig,
    CsvTimeColumn,
    DataImport,
    DataImportStatus,
    DataTypeKey,
    Hdf5DataColumn,
    Hdf5ImportConfig,
    McapComplexTypesImportMode,
    McapDataColumn,
    McapImportConfig,
    McapParseErrorPolicy,
    ParquetDataColumn,
    ParquetFlatDatasetImportConfig,
    ParquetSingleChannelConfig,
    ParquetSingleChannelPerRowImportConfig,
    ParquetTimeColumn,
    TdmsDataColumn,
    TdmsImportConfig,
    TimeFormat,
    UlogDataColumn,
    UlogImportConfig,
    UlogParseErrorPolicy,
)
from sift_client.sift_types.job import Job, JobStatus, JobType
from sift_client.sift_types.run import Run

if TYPE_CHECKING:
    from sift.common.type.v1.channel_data_type_pb2 import (
        ChannelDataType as ChannelDataTypeProto,
    )
    from sift.data_imports.v2.data_imports_pb2 import (
        DataImportStatus as DataImportStatusProto,
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
        time_column=ParquetTimeColumn(
            path="timestamp", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS
        ),
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

    def test_missing_time_format_raises_on_to_proto(self):
        """HDF5 timestamps aren't self-describing, so an unset time_format
        must fail loudly at upload rather than be silently defaulted.
        """
        config = Hdf5ImportConfig(asset_name="my_asset", data=[])
        assert config.time_format is None
        with pytest.raises(ValueError, match="time_format is required"):
            config._to_proto()


class TestUlogConfig:
    def _config(self):
        return UlogImportConfig(
            asset_name="my_asset",
            run_name="run1",
            data=[
                UlogDataColumn(
                    message_name="sensor_accel",
                    field_name="x",
                    name="sensor_accel_0.x",
                    data_type=ChannelDataType.FLOAT,
                ),
                UlogDataColumn(
                    message_name="vehicle_status",
                    field_name="nav_state",
                    name="nav_state",
                    data_type=ChannelDataType.UINT_32,
                    units="enum",
                    description="navigation state",
                ),
            ],
            info_keys=["ver_sw"],
            param_keys=["BAT1_CAPACITY"],
            parse_error_policy=UlogParseErrorPolicy.IGNORE_ERROR,
        )

    def test_to_proto(self):
        proto = self._config()._to_proto()
        assert proto.asset_name == "my_asset"
        assert proto.run_name == "run1"
        assert len(proto.data) == 2
        assert proto.data[0].message_name == "sensor_accel"
        assert proto.data[0].instance == 0
        assert proto.data[0].field_name == "x"
        assert proto.data[0].channel_config.name == "sensor_accel_0.x"
        assert proto.data[1].message_name == "vehicle_status"
        assert proto.data[1].field_name == "nav_state"
        assert proto.data[1].channel_config.name == "nav_state"
        assert proto.data[1].channel_config.units == "enum"
        assert list(proto.info_keys) == ["ver_sw"]
        assert list(proto.param_keys) == ["BAT1_CAPACITY"]

    def test_to_proto_defaults(self):
        """An empty config imports all channels; the default policy fails on error."""
        from sift.data_imports.v2.data_imports_pb2 import (
            ULOG_PARSE_ERROR_POLICY_FAIL_ON_ERROR,
        )

        proto = UlogImportConfig(asset_name="a")._to_proto()
        assert len(proto.data) == 0
        assert proto.run_id == ""
        assert proto.parse_error_policy == ULOG_PARSE_ERROR_POLICY_FAIL_ON_ERROR
        assert not proto.HasField("relative_start_time")

    def test_relative_start_time_round_trips(self):
        config = UlogImportConfig(
            asset_name="a",
            relative_start_time=datetime(2026, 1, 1, tzinfo=timezone.utc),
        )
        proto = config._to_proto()
        assert proto.HasField("relative_start_time")
        restored = UlogImportConfig._from_proto(proto)
        assert restored.relative_start_time == config.relative_start_time

    def test_from_proto_round_trip(self):
        config = self._config()
        restored = UlogImportConfig._from_proto(config._to_proto())
        assert restored.asset_name == config.asset_name
        assert restored.run_name == config.run_name
        assert restored.info_keys == config.info_keys
        assert restored.param_keys == config.param_keys
        assert restored.parse_error_policy == UlogParseErrorPolicy.IGNORE_ERROR
        assert len(restored.data) == 2
        assert restored.data[1].message_name == "vehicle_status"
        assert restored.data[1].instance == 0
        assert restored.data[1].field_name == "nav_state"
        assert restored.data[1].name == "nav_state"
        assert restored.data[1].data_type == ChannelDataType.UINT_32
        assert restored.data[1].units == "enum"

    def test_run_id_takes_precedence(self):
        proto = UlogImportConfig(asset_name="a", run_name="ignored", run_id="run_123")._to_proto()
        assert proto.run_id == "run_123"

    def test_channel_derived_from_selector(self):
        col = UlogDataColumn(
            message_name="sensor_accel",
            instance=1,
            field_name="x",
            data_type=ChannelDataType.FLOAT,
        )
        assert col.default_channel_name == "sensor_accel_1.x"

    def test_nonzero_instance_round_trips_through_proto(self):
        config = UlogImportConfig(
            asset_name="a",
            data=[
                UlogDataColumn(
                    message_name="sensor_accel",
                    instance=1,
                    field_name="x",
                    data_type=ChannelDataType.FLOAT,
                )
            ],
        )
        proto = config._to_proto()
        assert proto.data[0].instance == 1
        restored = UlogImportConfig._from_proto(proto)
        assert restored.data[0].instance == 1
        assert restored.data[0].name == "sensor_accel_1.x"

    def test_log_message_channel_is_message_name(self):
        col = UlogDataColumn(message_name="log_messages_5", data_type=ChannelDataType.STRING)
        assert col.default_channel_name == "log_messages_5"
        assert col.name == "log_messages_5"
        proto_data = UlogImportConfig(asset_name="a", data=[col])._to_proto().data[0]
        assert proto_data.message_name == "log_messages_5"
        assert proto_data.instance == 0
        assert proto_data.field_name == ""

    def test_name_defaults_to_channel(self):
        col = UlogDataColumn(
            message_name="sensor_accel", field_name="x", data_type=ChannelDataType.FLOAT
        )
        assert col.name == "sensor_accel_0.x"

    def test_explicit_name_overrides_channel(self):
        col = UlogDataColumn(
            message_name="vehicle_status",
            field_name="nav_state",
            name="nav_state",
            data_type=ChannelDataType.UINT_32,
        )
        assert col.name == "nav_state"

    def test_getitem(self):
        col = self._config()["nav_state"]
        assert col.default_channel_name == "vehicle_status_0.nav_state"

    def test_getitem_not_found(self):
        with pytest.raises(KeyError, match="nonexistent"):
            self._config()["nonexistent"]


class TestMcapConfig:
    def _config(self):
        return McapImportConfig(
            asset_name="my_asset",
            run_name="run1",
            data=[
                McapDataColumn(
                    topic="/imu/data",
                    field_path="orientation.x",
                    data_type=ChannelDataType.DOUBLE,
                ),
                McapDataColumn(
                    topic="/battery",
                    field_path="voltage",
                    name="battery_voltage",
                    data_type=ChannelDataType.FLOAT,
                    units="V",
                    description="pack voltage",
                ),
            ],
            metadata_records=["calibration"],
            parse_error_policy=McapParseErrorPolicy.IGNORE_ERROR,
            complex_types_import_mode=McapComplexTypesImportMode.STRING,
        )

    def test_to_proto(self):
        proto = self._config()._to_proto()
        assert proto.asset_name == "my_asset"
        assert proto.run_name == "run1"
        assert len(proto.data) == 2
        assert proto.data[0].topic == "/imu/data"
        assert proto.data[0].ros2.field_path == "orientation.x"
        assert proto.data[0].channel_config.name == "/imu/data.orientation.x"
        assert proto.data[1].topic == "/battery"
        assert proto.data[1].channel_config.name == "battery_voltage"
        assert proto.data[1].channel_config.units == "V"
        assert list(proto.metadata_records) == ["calibration"]

    def test_to_proto_defaults(self):
        """An empty config imports all channels; the default policy fails on
        error and imports complex fields as both bytes and JSON strings.
        """
        from sift.data_imports.v2.data_imports_pb2 import (
            MCAP_COMPLEX_TYPES_IMPORT_MODE_BOTH,
            MCAP_PARSE_ERROR_POLICY_FAIL_ON_ERROR,
        )

        proto = McapImportConfig(asset_name="a")._to_proto()
        assert len(proto.data) == 0
        assert proto.run_id == ""
        assert proto.parse_error_policy == MCAP_PARSE_ERROR_POLICY_FAIL_ON_ERROR
        assert proto.complex_types_import_mode == MCAP_COMPLEX_TYPES_IMPORT_MODE_BOTH
        assert not proto.HasField("relative_start_time")

    def test_relative_start_time_round_trips(self):
        config = McapImportConfig(
            asset_name="a",
            relative_start_time=datetime(2026, 1, 1, tzinfo=timezone.utc),
        )
        proto = config._to_proto()
        assert proto.HasField("relative_start_time")
        restored = McapImportConfig._from_proto(proto)
        assert restored.relative_start_time == config.relative_start_time

    def test_from_proto_round_trip(self):
        config = self._config()
        restored = McapImportConfig._from_proto(config._to_proto())
        assert restored.asset_name == config.asset_name
        assert restored.run_name == config.run_name
        assert restored.metadata_records == config.metadata_records
        assert restored.parse_error_policy == McapParseErrorPolicy.IGNORE_ERROR
        assert restored.complex_types_import_mode == McapComplexTypesImportMode.STRING
        assert len(restored.data) == 2
        assert restored.data[0].topic == "/imu/data"
        assert restored.data[0].field_path == "orientation.x"
        assert restored.data[0].name == "/imu/data.orientation.x"
        assert restored.data[1].name == "battery_voltage"
        assert restored.data[1].data_type == ChannelDataType.FLOAT
        assert restored.data[1].units == "V"
        assert restored.data[1].description == "pack voltage"

    def test_from_proto_unspecified_enums_fall_back_to_defaults(self):
        """UNSPECIFIED proto values mean FAIL_ON_ERROR and BOTH on the server."""
        from sift.data_imports.v2.data_imports_pb2 import McapConfig as McapConfigProto

        restored = McapImportConfig._from_proto(McapConfigProto(asset_name="a"))
        assert restored.parse_error_policy == McapParseErrorPolicy.FAIL_ON_ERROR
        assert restored.complex_types_import_mode == McapComplexTypesImportMode.BOTH

    def test_run_id_takes_precedence(self):
        proto = McapImportConfig(asset_name="a", run_name="ignored", run_id="run_123")._to_proto()
        assert proto.run_id == "run_123"

    def test_name_defaults_to_channel(self):
        col = McapDataColumn(
            topic="/imu/data", field_path="orientation.x", data_type=ChannelDataType.DOUBLE
        )
        assert col.default_channel_name == "/imu/data.orientation.x"
        assert col.name == "/imu/data.orientation.x"

    def test_explicit_name_overrides_channel(self):
        col = McapDataColumn(
            topic="/battery",
            field_path="voltage",
            name="battery_voltage",
            data_type=ChannelDataType.FLOAT,
        )
        assert col.name == "battery_voltage"

    def test_getitem(self):
        col = self._config()["battery_voltage"]
        assert col.field_path == "voltage"

    def test_getitem_not_found(self):
        with pytest.raises(KeyError, match="nonexistent"):
            self._config()["nonexistent"]


class TestImportFromPathClearsDetectedChannels:
    """Auto-detected ULog and MCAP configs import with an empty channel list
    so the server imports every channel instead of strictly filtering on a
    list that client detection may have misread.
    """

    async def _import(self, tmp_path, filename, detected):
        path = tmp_path / filename
        path.write_bytes(b"")

        api = DataImportAPIAsync(MagicMock())
        api.detect_config = AsyncMock(return_value=detected)
        api._low_level_client = MagicMock()
        api._low_level_client.create_from_upload = AsyncMock(return_value=("import_1", "url"))
        api.client.async_.jobs.get = AsyncMock(return_value="job")

        return await api.import_from_path(path, asset="my_asset", show_progress=False)

    @pytest.mark.asyncio
    async def test_mcap_data_cleared(self, tmp_path, monkeypatch):
        monkeypatch.setattr(
            "sift_client.resources.data_imports.upload_file", lambda *a, **k: {"jobId": "j1"}
        )
        detected = McapImportConfig(
            asset_name="",
            data=[McapDataColumn(topic="/imu", field_path="x", data_type=ChannelDataType.DOUBLE)],
        )

        job = await self._import(tmp_path, "log.mcap", detected)

        assert job == "job"
        assert detected.data == []
        assert detected.asset_name == "my_asset"

    @pytest.mark.asyncio
    async def test_ulog_data_cleared(self, tmp_path, monkeypatch):
        monkeypatch.setattr(
            "sift_client.resources.data_imports.upload_file", lambda *a, **k: {"jobId": "j1"}
        )
        detected = UlogImportConfig(
            asset_name="",
            data=[
                UlogDataColumn(
                    message_name="sensor_accel", field_name="x", data_type=ChannelDataType.FLOAT
                )
            ],
        )

        job = await self._import(tmp_path, "log.ulg", detected)

        assert job == "job"
        assert detected.data == []


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
            time_column=ParquetTimeColumn(path="ts", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
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
        col = ParquetTimeColumn(path="", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS)
        with pytest.raises(ValueError, match="path must be set"):
            col._to_proto()

    def test_missing_format_raises(self):
        """An unset format must fail loudly at upload rather than be silently
        defaulted; the precedence chain in detect_config/import_from_path is
        the supported way to populate it.
        """
        col = ParquetTimeColumn(path="timestamp")
        assert col.format is None
        with pytest.raises(ValueError, match="format must be set"):
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

    def test_ulog_extension_uses_map(self):
        assert _resolve_data_type_key(".ulg", None) == DataTypeKey.ULOG

    def test_mcap_extension_uses_map(self):
        assert _resolve_data_type_key(".mcap", None) == DataTypeKey.MCAP

    def test_explicit_data_type_overrides_extension(self):
        result = _resolve_data_type_key(".csv", DataTypeKey.TDMS)
        assert result == DataTypeKey.TDMS

    def test_unknown_extension_raises(self):
        with pytest.raises(ValueError, match="Unsupported file extension"):
            _resolve_data_type_key(".xyz", None)


class TestInferTimeColumn:
    def test_picks_canonical_skips_other_columns(self):
        path = _infer_time_column(
            [
                ("delta_time", ChannelDataType.INT_64, "delta_time"),
                ("voltage", ChannelDataType.DOUBLE, "voltage"),
                ("timestamp", ChannelDataType.INT_64, "timestamp"),
            ]
        )
        assert path == "timestamp"

    def test_accepts_uint64(self):
        path = _infer_time_column([("time", ChannelDataType.UINT_64, "time")])
        assert path == "time"

    def test_case_insensitive(self):
        path = _infer_time_column([("TimeStamp", ChannelDataType.INT_64, "TimeStamp")])
        assert path == "TimeStamp"

    def test_multiple_candidates_sorted_alphabetically(self):
        path = _infer_time_column(
            [
                ("timestamp", ChannelDataType.INT_64, "timestamp"),
                ("time", ChannelDataType.INT_64, "time"),
                ("ts", ChannelDataType.INT_64, "ts"),
            ]
        )
        assert path == "time"

    def test_returns_none_when_no_canonical_int_column(self):
        path = _infer_time_column(
            [
                ("timestamp", ChannelDataType.DOUBLE, "timestamp"),
                ("event_time", ChannelDataType.INT_64, "event_time"),
            ]
        )
        assert path is None


def _make_flat_dataset_response(
    time_path: str, data_columns: list[tuple[str, int]]
) -> ParquetConfig:
    return ParquetConfig(
        flat_dataset=ParquetFlatDatasetConfig(
            time_column=ParquetTimeColumnProto(path=time_path),
            data_columns=[
                ParquetDataColumnProto(
                    path=path,
                    channel_config=ChannelConfigProto(
                        name=path,
                        data_type=cast("ChannelDataTypeProto.ValueType", data_type),
                    ),
                )
                for path, data_type in data_columns
            ],
        )
    )


def _make_scpr_response(time_path: str, columns: list[tuple[str, int]]) -> ParquetConfig:
    return ParquetConfig(
        single_channel_per_row=ParquetSingleChannelPerRowConfig(
            time_column=ParquetTimeColumnProto(path=time_path),
            columns=[
                ParquetColumn(
                    path=path,
                    column_config=ChannelConfigProto(
                        name=path,
                        data_type=cast("ChannelDataTypeProto.ValueType", data_type),
                    ),
                )
                for path, data_type in columns
            ],
        )
    )


class TestParseParquetDetectResponseTimeFallback:
    def test_flat_dataset_infers_int64_time_column(self):
        proto = _make_flat_dataset_response(
            time_path="",
            data_columns=[
                ("voltage", ChannelDataType.DOUBLE.value),
                ("timestamp", ChannelDataType.INT_64.value),
                ("status", ChannelDataType.INT_32.value),
            ],
        )
        config = _parse_parquet_detect_response(proto, "file.parquet", 0, 0)
        assert isinstance(config, ParquetFlatDatasetImportConfig)
        assert config.time_column.path == "timestamp"
        assert [dc.path for dc in config.data_columns] == ["voltage", "status"]

    def test_flat_dataset_keeps_server_time_column_when_set(self):
        proto = _make_flat_dataset_response(
            time_path="server_ts",
            data_columns=[
                ("server_ts", ChannelDataType.INT_64.value),
                ("timestamp", ChannelDataType.INT_64.value),
                ("voltage", ChannelDataType.DOUBLE.value),
            ],
        )
        config = _parse_parquet_detect_response(proto, "file.parquet", 0, 0)
        assert config.time_column.path == "server_ts"
        assert [dc.path for dc in config.data_columns] == ["timestamp", "voltage"]

    def test_flat_dataset_no_int64_match_leaves_time_empty(self):
        proto = _make_flat_dataset_response(
            time_path="",
            data_columns=[("voltage", ChannelDataType.DOUBLE.value)],
        )
        config = _parse_parquet_detect_response(proto, "file.parquet", 0, 0)
        assert config.time_column.path == ""
        assert [dc.path for dc in config.data_columns] == ["voltage"]

    def test_scpr_infers_int64_time_column(self):
        proto = _make_scpr_response(
            time_path="",
            columns=[
                ("voltage", ChannelDataType.DOUBLE.value),
                ("timestamp", ChannelDataType.INT_64.value),
            ],
        )
        config = _parse_parquet_detect_response(proto, "file.parquet", 0, 0)
        assert isinstance(config, ParquetSingleChannelPerRowImportConfig)
        assert config.time_column.path == "timestamp"


def _data_import_proto(
    data_import_id: str = "data-import-1",
    status: DataImportStatusProto.ValueType = DATA_IMPORT_STATUS_SUCCEEDED,
) -> DataImportProto:
    proto = DataImportProto(
        data_import_id=data_import_id,
        source_url="https://example.com/data.csv",
        status=status,
        warning_messages=["skipped 3 records"],
        run_id="run-1",
        report_id="report-1",
        asset_id="asset-1",
    )
    proto.created_date.FromDatetime(datetime(2026, 1, 1, tzinfo=timezone.utc))
    proto.modified_date.FromDatetime(datetime(2026, 1, 2, tzinfo=timezone.utc))
    proto.data_start_time.FromDatetime(datetime(2025, 12, 31, tzinfo=timezone.utc))
    proto.data_stop_time.FromDatetime(datetime(2025, 12, 31, 1, tzinfo=timezone.utc))
    return proto


def _data_imports_api(mock_client) -> DataImportAPIAsync:
    api = DataImportAPIAsync(mock_client)
    api._low_level_client = MagicMock()
    return api


class TestDataImportStatus:
    def test_unspecified_maps_instead_of_raising(self):
        # A status the server never set must not break list_() parsing.
        assert (
            DataImportStatus.from_proto(DATA_IMPORT_STATUS_UNSPECIFIED)
            == DataImportStatus.UNSPECIFIED
        )

    def test_unknown_future_value_falls_back_to_unspecified(self):
        assert DataImportStatus.from_proto(100) == DataImportStatus.UNSPECIFIED


class TestDataImportFromProto:
    def test_all_fields(self):
        data_import = DataImport._from_proto(_data_import_proto())

        assert data_import.id_ == "data-import-1"
        assert data_import.source_url == "https://example.com/data.csv"
        assert data_import.status == DataImportStatus.SUCCEEDED
        assert data_import.error_message is None
        assert data_import.warning_messages == ["skipped 3 records"]
        assert data_import.created_date == datetime(2026, 1, 1, tzinfo=timezone.utc)
        assert data_import.modified_date == datetime(2026, 1, 2, tzinfo=timezone.utc)
        assert data_import.run_id == "run-1"
        assert data_import.report_id == "report-1"
        assert data_import.asset_id == "asset-1"
        assert data_import.data_start_time == datetime(2025, 12, 31, tzinfo=timezone.utc)
        assert data_import.data_stop_time == datetime(2025, 12, 31, 1, tzinfo=timezone.utc)

    def test_unset_optionals_are_none(self):
        proto = DataImportProto(
            data_import_id="data-import-2",
            status=DATA_IMPORT_STATUS_FAILED,
            error_message="boom",
        )
        data_import = DataImport._from_proto(proto)

        assert data_import.source_url is None
        assert data_import.error_message == "boom"
        assert data_import.warning_messages == []
        assert data_import.run_id is None
        assert data_import.report_id is None
        assert data_import.asset_id is None
        assert data_import.data_start_time is None
        assert data_import.data_stop_time is None


class TestDataImportsGet:
    @pytest.mark.asyncio
    async def test_returns_data_import(self, mock_client):
        api = _data_imports_api(mock_client)
        api._low_level_client.get = AsyncMock(
            return_value=DataImport._from_proto(_data_import_proto())
        )

        data_import = await api.get("data-import-1")

        api._low_level_client.get.assert_awaited_once_with("data-import-1")
        assert isinstance(data_import, DataImport)
        assert data_import.id_ == "data-import-1"
        assert data_import.client is mock_client


class TestDataImportsList:
    @pytest.mark.asyncio
    async def test_no_filters_sends_no_query(self, mock_client):
        api = _data_imports_api(mock_client)
        api._low_level_client.list_all_data_imports = AsyncMock(
            return_value=[DataImport._from_proto(_data_import_proto())]
        )

        data_imports = await api.list_()

        api._low_level_client.list_all_data_imports.assert_awaited_once_with(
            query_filter=None, order_by=None, max_results=None
        )
        assert data_imports[0].client is mock_client

    @pytest.mark.asyncio
    async def test_builds_cel_filter(self, mock_client):
        api = _data_imports_api(mock_client)
        api._low_level_client.list_all_data_imports = AsyncMock(return_value=[])

        await api.list_(
            data_import_ids=["di-1", "di-2"],
            source_url_contains="s3://bucket",
            status=DataImportStatus.FAILED,
            runs=["run-1"],
        )

        query_filter = api._low_level_client.list_all_data_imports.await_args.kwargs["query_filter"]
        assert query_filter == (
            "data_import_id in ['di-1','di-2'] && "
            "source_url.contains('s3://bucket') && "
            "status == 'DATA_IMPORT_STATUS_FAILED' && "
            "run_id in ['run-1']"
        )

    @pytest.mark.asyncio
    async def test_accepts_run_objects(self, mock_client):
        api = _data_imports_api(mock_client)
        api._low_level_client.list_all_data_imports = AsyncMock(return_value=[])
        run = MagicMock(spec=Run)
        run._id_or_error = "run-42"

        await api.list_(runs=[run])

        assert (
            api._low_level_client.list_all_data_imports.await_args.kwargs["query_filter"]
            == "run_id in ['run-42']"
        )

    @pytest.mark.asyncio
    async def test_forwards_pagination_and_ordering(self, mock_client):
        api = _data_imports_api(mock_client)
        api._low_level_client.list_all_data_imports = AsyncMock(return_value=[])

        await api.list_(order_by="created_date desc", limit=5, page_size=2)

        api._low_level_client.list_all_data_imports.assert_awaited_once_with(
            query_filter=None, order_by="created_date desc", max_results=5, page_size=2
        )


class TestDataImportsFind:
    @pytest.mark.asyncio
    async def test_raises_on_multiple_matches(self, mock_client):
        api = _data_imports_api(mock_client)
        api._low_level_client.list_all_data_imports = AsyncMock(
            return_value=[
                DataImport._from_proto(_data_import_proto("di-1")),
                DataImport._from_proto(_data_import_proto("di-2")),
            ]
        )

        with pytest.raises(ValueError, match="Multiple data imports found"):
            await api.find()


class TestDataImportsGetRun:
    @pytest.mark.asyncio
    async def test_resolves_run_id(self, mock_client):
        api = _data_imports_api(mock_client)
        api._low_level_client.get = AsyncMock(
            return_value=DataImport._from_proto(_data_import_proto())
        )
        run = MagicMock(spec=Run)
        mock_client.async_.runs.get = AsyncMock(return_value=run)

        assert await api.get_run("data-import-1") is run
        mock_client.async_.runs.get.assert_awaited_once_with(run_id="run-1")

    @pytest.mark.asyncio
    async def test_raises_without_run(self, mock_client):
        api = _data_imports_api(mock_client)
        proto = _data_import_proto()
        proto.ClearField("run_id")
        api._low_level_client.get = AsyncMock(return_value=DataImport._from_proto(proto))

        with pytest.raises(ValueError, match="does not have an associated run"):
            await api.get_run("data-import-1")


class TestImportFromPathDataImportId:
    """import_from_path must hand the returned job a way back to the import."""

    @staticmethod
    def _job(job_details):
        return Job(
            proto=MagicMock(),
            id_="job-1",
            organization_id="org-1",
            created_by_user_id="user-1",
            modified_by_user_id="user-1",
            created_date=datetime(2026, 1, 1, tzinfo=timezone.utc),
            modified_date=datetime(2026, 1, 1, tzinfo=timezone.utc),
            started_date=None,
            completed_date=None,
            job_type=JobType.DATA_IMPORT,
            job_status=JobStatus.RUNNING,
            job_status_details=None,
            job_details=job_details,
        )

    async def _run_import(self, mock_client, tmp_path):
        api = _data_imports_api(mock_client)
        api._low_level_client.create_from_upload = AsyncMock(
            return_value=("di-from-create", "https://upload.example/put")
        )
        job = self._job(job_details=None)
        job._apply_client_to_instance(mock_client)
        mock_client.async_.jobs.get = AsyncMock(return_value=job)

        csv_path = tmp_path / "data.csv"
        csv_path.write_text("time,value\n")
        config = CsvImportConfig(
            asset_name="test_asset",
            time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_RFC3339),
            data_columns=[CsvDataColumn(column=2, name="value", data_type=ChannelDataType.DOUBLE)],
        )

        with patch(
            "sift_client.resources.data_imports.upload_file",
            return_value={"jobId": "job-1"},
        ):
            return await api.import_from_path(csv_path, config=config, show_progress=False)

    @pytest.mark.asyncio
    async def test_records_id_when_job_details_missing(self, mock_client, tmp_path):
        """The create call's ID reaches the job even when the server hasn't set details."""
        job = await self._run_import(mock_client, tmp_path)

        job.get_data_import()

        mock_client.data_import.get.assert_called_once_with("di-from-create")

    @pytest.mark.asyncio
    async def test_missing_job_id_raises(self, mock_client, tmp_path):
        """A response without a job ID is still an error."""
        api = _data_imports_api(mock_client)
        api._low_level_client.create_from_upload = AsyncMock(
            return_value=("di-from-create", "https://upload.example/put")
        )
        csv_path = tmp_path / "data.csv"
        csv_path.write_text("time,value\n")
        config = CsvImportConfig(
            asset_name="test_asset",
            time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_RFC3339),
            data_columns=[CsvDataColumn(column=2, name="value", data_type=ChannelDataType.DOUBLE)],
        )

        with patch("sift_client.resources.data_imports.upload_file", return_value={}):
            with pytest.raises(RuntimeError, match="did not include a job ID"):
                await api.import_from_path(csv_path, config=config, show_progress=False)


ENUMS = {"IDLE": 0, "ARMED": 1}


def _enum_dict(channel_config: ChannelConfigProto) -> dict[str, int]:
    return {e.name: e.key for e in channel_config.enum_types}


class TestEnumTypes:
    def test_csv_round_trip(self):
        config = CsvImportConfig(
            asset_name="a",
            time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_RFC3339),
            data_columns=[
                CsvDataColumn(
                    column=2, name="state", data_type=ChannelDataType.ENUM, enum_types=ENUMS
                )
            ],
        )
        proto = config._to_proto()
        assert _enum_dict(proto.data_columns[2]) == ENUMS
        assert CsvImportConfig._from_proto(proto)["state"].enum_types == ENUMS

    def test_parquet_flat_dataset_round_trip(self):
        config = ParquetFlatDatasetImportConfig(
            asset_name="a",
            time_column=ParquetTimeColumn(path="ts", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
            data_columns=[
                ParquetDataColumn(
                    path="state", name="state", data_type=ChannelDataType.ENUM, enum_types=ENUMS
                )
            ],
        )
        proto = config._to_proto()
        assert _enum_dict(proto.flat_dataset.data_columns[0].channel_config) == ENUMS
        round_tripped = ParquetFlatDatasetImportConfig._from_proto(proto)
        assert round_tripped["state"].enum_types == ENUMS

    def test_parquet_single_channel_per_row_round_trip(self):
        config = ParquetSingleChannelPerRowImportConfig(
            asset_name="a",
            time_column=ParquetTimeColumn(path="ts", format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
            single_channel=ParquetSingleChannelConfig(
                data_path="value", name="state", data_type=ChannelDataType.ENUM, enum_types=ENUMS
            ),
        )
        proto = config._to_proto()
        assert _enum_dict(proto.single_channel_per_row.single_channel.channel) == ENUMS
        round_tripped = ParquetSingleChannelPerRowImportConfig._from_proto(proto)
        assert round_tripped.single_channel is not None
        assert round_tripped.single_channel.enum_types == ENUMS

    def test_tdms_round_trip(self):
        config = TdmsImportConfig(
            asset_name="a",
            data=[
                TdmsDataColumn(
                    group_name="g",
                    channel_name="state",
                    name="state",
                    data_type=ChannelDataType.ENUM,
                    enum_types=ENUMS,
                )
            ],
        )
        proto = config._to_proto()
        assert _enum_dict(proto.data[0].channel_config) == ENUMS
        assert TdmsImportConfig._from_proto(proto)["state"].enum_types == ENUMS

    def test_hdf5_to_proto(self):
        config = Hdf5ImportConfig(
            asset_name="a",
            time_format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS,
            data=[
                Hdf5DataColumn(
                    time_dataset="/time",
                    value_dataset="/state",
                    name="state",
                    data_type=ChannelDataType.ENUM,
                    enum_types=ENUMS,
                )
            ],
        )
        proto = config._to_proto()
        assert _enum_dict(proto.data[0].channel_config) == ENUMS

    def test_ulog_round_trip(self):
        config = UlogImportConfig(
            asset_name="a",
            data=[
                UlogDataColumn(
                    message_name="vehicle_status",
                    field_name="arming_state",
                    data_type=ChannelDataType.ENUM,
                    enum_types=ENUMS,
                )
            ],
        )
        proto = config._to_proto()
        assert _enum_dict(proto.data[0].channel_config) == ENUMS
        round_tripped = UlogImportConfig._from_proto(proto)
        assert round_tripped["vehicle_status_0.arming_state"].enum_types == ENUMS

    def test_enum_types_require_enum_data_type(self):
        with pytest.raises(ValueError, match="ENUM"):
            CsvDataColumn(
                column=2, name="state", data_type=ChannelDataType.DOUBLE, enum_types=ENUMS
            )

    def test_enum_data_type_requires_enum_types(self):
        with pytest.raises(ValueError, match="requires 'enum_types'"):
            CsvDataColumn(column=2, name="state", data_type=ChannelDataType.ENUM)

    def test_enum_types_reject_duplicate_keys(self):
        with pytest.raises(ValueError, match="[Dd]uplicate"):
            CsvDataColumn(
                column=2,
                name="state",
                data_type=ChannelDataType.ENUM,
                enum_types={"IDLE": 0, "ARMED": 0},
            )

    def test_no_enum_types_from_proto_is_none(self, csv_config):
        round_tripped = CsvImportConfig._from_proto(csv_config._to_proto())
        assert round_tripped["cpu_util"].enum_types is None
