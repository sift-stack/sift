"""Tests for flat dataset Parquet config detection."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock

import pyarrow as pa
import pyarrow.parquet as pq
import pytest
from pydantic import ValidationError

from sift_client._internal.util.parquet import detect_flat_parquet_config
from sift_client.resources import DataImportAPIAsync
from sift_client.resources.data_imports import _apply_relative_start_time
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    CsvImportConfig,
    CsvTimeColumn,
    DataTypeKey,
    TimeFormat,
)


@pytest.fixture
def create_parquet_file(tmp_path):
    """Return a helper that writes a table to a Parquet file and returns its path."""
    file_path = tmp_path / "test.parquet"

    def _create(table: pa.Table):
        pq.write_table(table, file_path)
        return file_path

    return _create


def _columns(config) -> dict[str, tuple[str, ChannelDataType]]:
    return {dc.path: (dc.name, dc.data_type) for dc in config.data_columns}


def _with_time(arr: pa.Array) -> pa.Table:
    return pa.table({"value": pa.array([1.0, 2.0]), "t": arr})


def _embedded_field(name: str, arrow_type: pa.DataType, key: str, config: dict) -> pa.Field:
    return pa.field(name, arrow_type, metadata={key: json.dumps(config)})


def _embedded_table(fields: list[pa.Field]) -> pa.Table:
    return pa.table(
        {f.name: pa.array([1, 2]).cast(f.type) for f in fields}, schema=pa.schema(fields)
    )


_TIME_CONFIG = "sift_time_config"
_CHANNEL_CONFIG = "sift_channel_config"


def _time_field(name: str, time_format: str = "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS") -> pa.Field:
    return _embedded_field(name, pa.int64(), _TIME_CONFIG, {"time_format": time_format})


def _channel_field(name: str, data_type: str | None = "CHANNEL_DATA_TYPE_DOUBLE") -> pa.Field:
    config = {"name": name} if data_type is None else {"name": name, "data_type": data_type}
    return _embedded_field(name, pa.float64(), _CHANNEL_CONFIG, config)


class TestDataTypes:
    def test_scalar_types(self, create_parquet_file):
        table = pa.table(
            {
                "bool": pa.array([True]),
                "f16": pa.array([1.0], pa.float16()),
                "f32": pa.array([1.0], pa.float32()),
                "f64": pa.array([1.0], pa.float64()),
                "i8": pa.array([1], pa.int8()),
                "i16": pa.array([1], pa.int16()),
                "i32": pa.array([1], pa.int32()),
                "i64": pa.array([1], pa.int64()),
                "u8": pa.array([1], pa.uint8()),
                "u16": pa.array([1], pa.uint16()),
                "u32": pa.array([1], pa.uint32()),
                "u64": pa.array([1], pa.uint64()),
                "string": pa.array(["a"], pa.string()),
                "large_string": pa.array(["a"], pa.large_string()),
                "string_view": pa.array(["a"], pa.string_view()),
                "binary": pa.array([b"a"], pa.binary()),
                "large_binary": pa.array([b"a"], pa.large_binary()),
                "binary_view": pa.array([b"a"], pa.binary_view()),
                "fixed_binary": pa.array([b"ab"], pa.binary(2)),
            }
        )
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert {path: data_type for path, (_, data_type) in _columns(config).items()} == {
            "bool": ChannelDataType.BOOL,
            "f16": ChannelDataType.FLOAT,
            "f32": ChannelDataType.FLOAT,
            "f64": ChannelDataType.DOUBLE,
            "i8": ChannelDataType.INT_32,
            "i16": ChannelDataType.INT_32,
            "i32": ChannelDataType.INT_32,
            "i64": ChannelDataType.INT_64,
            "u8": ChannelDataType.UINT_32,
            "u16": ChannelDataType.UINT_32,
            "u32": ChannelDataType.UINT_32,
            "u64": ChannelDataType.UINT_64,
            "string": ChannelDataType.STRING,
            "large_string": ChannelDataType.STRING,
            "string_view": ChannelDataType.STRING,
            "binary": ChannelDataType.BYTES,
            "large_binary": ChannelDataType.BYTES,
            "binary_view": ChannelDataType.BYTES,
            "fixed_binary": ChannelDataType.BYTES,
        }

    def test_time_types_are_int64(self, create_parquet_file):
        # The absolute timestamp comes first so it is the time column; the
        # relative time64 column is then only a data column.
        table = pa.table(
            {
                "ts": pa.array([0], pa.timestamp("ns")),
                "d32": pa.array([0], pa.date32()),
                "t64": pa.array([0], pa.time64("ns")),
                "dur": pa.array([0], pa.duration("us")),
            }
        )
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert all(dc.data_type == ChannelDataType.INT_64 for dc in config.data_columns)

    def test_complex_types_are_bytes(self, create_parquet_file):
        table = pa.table(
            {
                "list": pa.array([[1, 2]]),
                "large_list": pa.array([[1, 2]], pa.large_list(pa.int64())),
                "map": pa.array([[("k", 1)]], pa.map_(pa.string(), pa.int64())),
                "list_of_struct": pa.array([[{"x": 1}]]),
            }
        )
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert _columns(config) == {
            "list": ("list", ChannelDataType.BYTES),
            "large_list": ("large_list", ChannelDataType.BYTES),
            "map": ("map", ChannelDataType.BYTES),
            "list_of_struct": ("list_of_struct", ChannelDataType.BYTES),
        }

    @pytest.mark.parametrize(
        ("column", "array"),
        [
            ("dictionary", pa.array(["a"]).dictionary_encode()),
            ("decimal", pa.array([1], pa.decimal128(10, 2))),
            ("null", pa.array([None], pa.null())),
            ("fixed_list", pa.array([[1, 2]], pa.list_(pa.int64(), 2))),
        ],
    )
    def test_unsupported_type_is_skipped_with_warning(self, create_parquet_file, column, array):
        table = pa.table({"value": pa.array([1.0]), column: array})

        with pytest.warns(UserWarning, match=f"column '{column}'"):
            config = detect_flat_parquet_config(create_parquet_file(table))

        assert list(_columns(config)) == ["value"]

    def test_unsupported_type_inside_struct_raises(self, create_parquet_file):
        struct = pa.StructArray.from_arrays([pa.array([1], pa.decimal128(10, 2))], names=["d"])
        table = pa.table({"s": struct})

        with pytest.raises(ValueError, match="unable to add field s"):
            detect_flat_parquet_config(create_parquet_file(table))


class TestStructs:
    def test_nested_fields_are_flattened(self, create_parquet_file):
        table = pa.table({"s": pa.array([{"a": 1, "b": {"c": "x"}}])})
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert _columns(config) == {
            "s|a": ("s.a", ChannelDataType.INT_64),
            "s|b|c": ("s.b.c", ChannelDataType.STRING),
        }

    def test_pipe_in_column_name_becomes_dot(self, create_parquet_file):
        table = pa.table({"x|y": pa.array([1.0])})
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert _columns(config) == {"x|y": ("x.y", ChannelDataType.DOUBLE)}

    def test_duplicate_channel_names_are_kept(self, create_parquet_file):
        """Matches the server: names that collide after flattening are not rejected."""
        table = pa.table({"a.b": pa.array([1]), "a": pa.array([{"b": 1}])})
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert _columns(config) == {
            "a.b": ("a.b", ChannelDataType.INT_64),
            "a|b": ("a.b", ChannelDataType.INT_64),
        }


class TestTimeColumn:
    @pytest.mark.parametrize(
        ("arrow_type", "expected"),
        [
            # Parquet has no second-resolution timestamp, so it is stored as ms.
            (pa.timestamp("s"), TimeFormat.ABSOLUTE_UNIX_MILLISECONDS),
            (pa.timestamp("ms"), TimeFormat.ABSOLUTE_UNIX_MILLISECONDS),
            (pa.timestamp("us"), TimeFormat.ABSOLUTE_UNIX_MICROSECONDS),
            (pa.timestamp("ns"), TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
            (pa.timestamp("us", tz="UTC"), TimeFormat.ABSOLUTE_UNIX_MICROSECONDS),
            (pa.date32(), TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
            (pa.date64(), TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
        ],
    )
    def test_absolute_format_from_type(self, create_parquet_file, arrow_type, expected):
        config = detect_flat_parquet_config(
            create_parquet_file(_with_time(pa.array([0, 1], arrow_type)))
        )

        assert config.time_column.path == "t"
        assert config.time_column.format == expected

    @pytest.mark.parametrize(
        "arrow_type", [pa.time32("s"), pa.time32("ms"), pa.time64("us"), pa.time64("ns")]
    )
    def test_relative_format_without_start_time_raises(self, create_parquet_file, arrow_type):
        """A relative time column has no start time in the file, which the config rejects."""
        path = create_parquet_file(_with_time(pa.array([0, 1], arrow_type)))

        with pytest.raises(ValidationError, match="relative_start_time"):
            detect_flat_parquet_config(path)

    @pytest.mark.parametrize(
        ("arrow_type", "expected"),
        [
            (pa.time32("ms"), TimeFormat.RELATIVE_MILLISECONDS),
            (pa.time64("us"), TimeFormat.RELATIVE_MICROSECONDS),
            (pa.time64("ns"), TimeFormat.RELATIVE_NANOSECONDS),
        ],
    )
    def test_relative_format_with_start_time(self, create_parquet_file, arrow_type, expected):
        start = datetime(2024, 1, 1, tzinfo=timezone.utc)
        path = create_parquet_file(_with_time(pa.array([0, 1], arrow_type)))

        config = detect_flat_parquet_config(path, relative_start_time=start)

        assert config.time_column.format == expected
        assert config.time_column.relative_start_time == start

    @pytest.mark.parametrize("arrow_type", [pa.timestamp("ns"), pa.date32()])
    def test_absolute_format_ignores_start_time(self, create_parquet_file, arrow_type):
        path = create_parquet_file(_with_time(pa.array([0, 1], arrow_type)))

        config = detect_flat_parquet_config(
            path, relative_start_time=datetime(2024, 1, 1, tzinfo=timezone.utc)
        )

        assert config.time_column.relative_start_time is None

    @pytest.mark.parametrize("unit", ["s", "ms", "us", "ns"])
    def test_duration_is_int64_data_not_time(self, create_parquet_file, unit):
        config = detect_flat_parquet_config(
            create_parquet_file(_with_time(pa.array([0, 1], pa.duration(unit))))
        )

        assert config.time_column.path == ""
        assert _columns(config)["t"] == ("t", ChannelDataType.INT_64)

    def test_first_time_column_wins(self, create_parquet_file):
        table = pa.table(
            {
                "t1": pa.array([0], pa.timestamp("ms")),
                "t2": pa.array([0], pa.timestamp("ns")),
            }
        )
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert config.time_column.path == "t1"

    def test_time_column_is_not_a_data_column(self, create_parquet_file):
        table = pa.table({"t": pa.array([0], pa.timestamp("ns")), "v": pa.array([1.0])})
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert list(_columns(config)) == ["v"]

    def test_no_time_column_leaves_path_empty(self, create_parquet_file):
        config = detect_flat_parquet_config(create_parquet_file(pa.table({"v": [1.0]})))

        assert config.time_column.path == ""
        assert config.time_column.format is None

    @pytest.mark.parametrize("arrow_type", [pa.int64(), pa.uint64()])
    def test_infers_named_integer_time_column(self, create_parquet_file, arrow_type):
        table = pa.table({"v": pa.array([1.0]), "TimeStamp": pa.array([0], arrow_type)})
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert config.time_column.path == "TimeStamp"
        assert config.time_column.format is None
        assert list(_columns(config)) == ["v"]

    def test_time_typed_column_wins_over_named_integer(self, create_parquet_file):
        table = pa.table(
            {"timestamp": pa.array([0], pa.int64()), "t": pa.array([0], pa.timestamp("ns"))}
        )
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert config.time_column.path == "t"
        assert list(_columns(config)) == ["timestamp"]

    @pytest.mark.parametrize(
        "column",
        [
            pytest.param({"count": pa.array([0], pa.int64())}, id="unmatched_name"),
            pytest.param({"timestamp": pa.array([0], pa.int32())}, id="int32"),
        ],
    )
    def test_does_not_infer_other_integer_columns(self, create_parquet_file, column):
        config = detect_flat_parquet_config(create_parquet_file(pa.table(column)))

        assert config.time_column.path == ""
        assert list(_columns(config)) == list(column)

    def test_nested_time_field_is_not_selected(self, create_parquet_file):
        struct = pa.StructArray.from_arrays([pa.array([0], pa.timestamp("ns"))], names=["t"])
        config = detect_flat_parquet_config(create_parquet_file(pa.table({"s": struct})))

        assert config.time_column.path == ""
        assert _columns(config) == {"s|t": ("s.t", ChannelDataType.INT_64)}


class TestEmbeddedConfig:
    def test_uses_embedded_configs(self, create_parquet_file):
        table = _embedded_table(
            [
                _embedded_field(
                    "time",
                    pa.int64(),
                    _TIME_CONFIG,
                    {
                        "time_format": "TIME_FORMAT_RELATIVE_SECONDS",
                        "relative_start_time": "2024-01-01T00:00:00Z",
                    },
                ),
                _embedded_field(
                    "x",
                    pa.float64(),
                    _CHANNEL_CONFIG,
                    {
                        "name": "x.renamed",
                        "data_type": "CHANNEL_DATA_TYPE_DOUBLE",
                        "units": "m",
                        "description": "position",
                    },
                ),
                _embedded_field(
                    "state",
                    pa.uint32(),
                    _CHANNEL_CONFIG,
                    {
                        "name": "state",
                        "data_type": "CHANNEL_DATA_TYPE_ENUM",
                        "enum_types": [{"name": "on", "key": 1}],
                    },
                ),
            ]
        )
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert config.time_column.path == "time"
        assert config.time_column.format == TimeFormat.RELATIVE_SECONDS
        assert config.time_column.relative_start_time == datetime(2024, 1, 1, tzinfo=timezone.utc)
        assert _columns(config) == {
            "x": ("x.renamed", ChannelDataType.DOUBLE),
            "state": ("state", ChannelDataType.ENUM),
        }
        assert config["x.renamed"].units == "m"
        assert config["x.renamed"].description == "position"
        assert config["state"].enum_types == {"on": 1}

    def test_camel_case_keys_and_numeric_enums(self, create_parquet_file):
        """Protobuf JSON allows lowerCamelCase field names and enum numbers."""
        table = _embedded_table(
            [
                _embedded_field(
                    "t",
                    pa.int64(),
                    _TIME_CONFIG,
                    {"time_format": "TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS"},
                ),
                _embedded_field(
                    "x",
                    pa.uint32(),
                    _CHANNEL_CONFIG,
                    {
                        "name": "x",
                        "dataType": ChannelDataType.ENUM.value,
                        "enumTypes": [{"name": "on", "key": 1}],
                    },
                ),
            ]
        )
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert config.time_column.format == TimeFormat.ABSOLUTE_UNIX_MILLISECONDS
        assert config["x"].data_type == ChannelDataType.ENUM
        assert config["x"].enum_types == {"on": 1}

    @pytest.mark.parametrize(
        ("fields", "match"),
        [
            pytest.param(
                [_time_field("t"), _channel_field("x", data_type="BOGUS")],
                "failed to parse embedded channel config for column x",
                id="unknown_channel_data_type",
            ),
            pytest.param(
                [
                    _time_field("t"),
                    _embedded_field(
                        "x",
                        pa.float64(),
                        _CHANNEL_CONFIG,
                        {"name": "x", "data_type": "CHANNEL_DATA_TYPE_DOUBLE", "bogus": 1},
                    ),
                ],
                "failed to parse embedded channel config for column x",
                id="unknown_channel_config_field",
            ),
            pytest.param(
                [_time_field("t"), _channel_field("x", data_type=None)],
                "missing data type .* column x",
                id="missing_channel_data_type",
            ),
            pytest.param(
                [_channel_field("x")],
                "no embedded time config found in parquet file",
                id="missing_time_config",
            ),
            pytest.param(
                [_time_field("t1"), _time_field("t2")],
                "multiple embedded time column configs",
                id="multiple_time_configs",
            ),
            pytest.param(
                [_time_field("t", time_format="BOGUS"), _channel_field("x", data_type=None)],
                "invalid embedded time format: BOGUS",
                id="invalid_time_format",
            ),
            pytest.param(
                [_embedded_field("t", pa.int64(), _TIME_CONFIG, {}), _channel_field("x")],
                "invalid embedded time format: $",
                id="missing_time_format",
            ),
            pytest.param(
                [
                    _embedded_field("t", pa.int64(), _TIME_CONFIG, {"time_format": 11}),
                    _channel_field("x"),
                ],
                "failed to parse embedded time config",
                id="numeric_time_format",
            ),
        ],
    )
    def test_invalid_embedded_config_raises(self, create_parquet_file, fields, match):
        with pytest.raises(ValueError, match=match):
            detect_flat_parquet_config(create_parquet_file(_embedded_table(fields)))

    def test_unconfigured_columns_are_ignored(self, create_parquet_file):
        table = _embedded_table(
            [_time_field("time"), _channel_field("x"), pa.field("plain", pa.float64())]
        )
        config = detect_flat_parquet_config(create_parquet_file(table))

        assert list(_columns(config)) == ["x"]


class TestFile:
    def test_footer_fields_populated(self, create_parquet_file):
        path = create_parquet_file(pa.table({"v": [1.0]}))
        config = detect_flat_parquet_config(path)

        assert config.footer_length > 0
        assert config.footer_offset + config.footer_length + 8 == path.stat().st_size

    def test_empty_schema(self, create_parquet_file):
        config = detect_flat_parquet_config(create_parquet_file(pa.table({})))

        assert config.data_columns == []
        assert config.time_column.path == ""

    def test_invalid_file_raises(self, tmp_path):
        path = tmp_path / "bad.parquet"
        path.write_bytes(b"not a parquet file")

        with pytest.raises(ValueError, match="Invalid Parquet file"):
            detect_flat_parquet_config(path)

    def test_truncated_file_raises(self, create_parquet_file):
        path = create_parquet_file(pa.table({"v": [1.0]}))
        data = path.read_bytes()
        # Keep the trailer but drop the start of the footer metadata.
        path.write_bytes(data[:4] + data[-8:])

        with pytest.raises(Exception):  # noqa: B017, PT011
            detect_flat_parquet_config(path)


class TestDetectConfigForType:
    @pytest.mark.asyncio
    async def test_detects_locally_and_drops_time_column(self, create_parquet_file):
        table = pa.table({"t": pa.array([0], pa.timestamp("ms")), "v": pa.array([1.0])})
        api = DataImportAPIAsync(MagicMock())
        api._low_level_client = MagicMock()
        api._low_level_client.detect_config = AsyncMock()

        config = await api._detect_config_for_type(
            create_parquet_file(table), DataTypeKey.PARQUET_FLATDATASET
        )

        api._low_level_client.detect_config.assert_not_called()
        assert config.time_column.path == "t"
        assert config.time_column.format == TimeFormat.ABSOLUTE_UNIX_MILLISECONDS
        assert [dc.path for dc in config.data_columns] == ["v"]

    @pytest.mark.asyncio
    async def test_infers_int64_time_column_by_name(self, create_parquet_file):
        table = pa.table({"timestamp": pa.array([0], pa.int64()), "v": pa.array([1.0])})
        api = DataImportAPIAsync(MagicMock())

        config = await api._detect_config_for_type(
            create_parquet_file(table), DataTypeKey.PARQUET_FLATDATASET
        )

        assert config.time_column.path == "timestamp"
        assert [dc.path for dc in config.data_columns] == ["v"]


class TestDetectConfigRelativeStartTime:
    START = datetime(2024, 1, 1, tzinfo=timezone.utc)

    @pytest.mark.asyncio
    async def test_relative_column_uses_start_time(self, create_parquet_file):
        path = create_parquet_file(_with_time(pa.array([0, 1], pa.time64("us"))))
        api = DataImportAPIAsync(MagicMock())

        config = await api.detect_config(
            path, data_type=DataTypeKey.PARQUET_FLATDATASET, relative_start_time=self.START
        )

        assert config.time_column.format == TimeFormat.RELATIVE_MICROSECONDS
        assert config.time_column.relative_start_time == self.START

    @pytest.mark.asyncio
    async def test_relative_time_format_override_uses_start_time(self, create_parquet_file):
        table = pa.table({"timestamp": pa.array([0, 1], pa.int64()), "v": pa.array([1.0, 2.0])})
        api = DataImportAPIAsync(MagicMock())

        config = await api.detect_config(
            create_parquet_file(table),
            data_type=DataTypeKey.PARQUET_FLATDATASET,
            time_format=TimeFormat.RELATIVE_SECONDS,
            relative_start_time=self.START,
        )

        assert config.time_column.path == "timestamp"
        assert config.time_column.format == TimeFormat.RELATIVE_SECONDS
        assert config.time_column.relative_start_time == self.START

    @pytest.mark.asyncio
    async def test_absolute_column_warns_and_ignores_start_time(self, create_parquet_file):
        path = create_parquet_file(_with_time(pa.array([0, 1], pa.timestamp("ns"))))
        api = DataImportAPIAsync(MagicMock())

        with pytest.warns(UserWarning, match="relative_start_time"):
            config = await api.detect_config(
                path, data_type=DataTypeKey.PARQUET_FLATDATASET, relative_start_time=self.START
            )

        assert config.time_column.relative_start_time is None

    def test_non_parquet_config_warns_and_ignores_start_time(self):
        config = CsvImportConfig(
            asset_name="a",
            time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_RFC3339),
            data_columns=[],
        )
        config.time_column.format = TimeFormat.RELATIVE_SECONDS

        with pytest.warns(UserWarning, match="not supported for CsvImportConfig"):
            _apply_relative_start_time(config, self.START)

        assert config.time_column.relative_start_time is None
