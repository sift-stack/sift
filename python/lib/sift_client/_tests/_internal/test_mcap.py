"""Tests for MCAP channel detection."""

from __future__ import annotations

import pytest
from mcap.records import Channel, Schema
from mcap.writer import Writer

from sift_client._internal.util.mcap import (
    UnsupportedTopicError,
    detect_mcap_config,
    detect_mcap_topics,
    expand_message_fields,
    parse_schema_defs,
)
from sift_client.sift_types.channel import ChannelDataType


def _schema(data: str, schema_id: int = 1, name: str = "test_msgs/msg/Test") -> Schema:
    return Schema(id=schema_id, data=data.encode(), encoding="ros2msg", name=name)


def _channel(topic: str, schema_id: int = 1, channel_id: int = 1, encoding: str = "cdr") -> Channel:
    return Channel(
        id=channel_id, topic=topic, message_encoding=encoding, metadata={}, schema_id=schema_id
    )


def _leaves(data: str, name: str = "test_msgs/msg/Test"):
    root, msgdefs = parse_schema_defs(_schema(data, name=name))
    return expand_message_fields(root, msgdefs)


IMU_SCHEMA = """geometry_msgs/Vector3 gyro
float64 temp
================================================================================
MSG: geometry_msgs/Vector3
float64 x
float64 y
float64 z
"""


def _write_mcap(path, schemas_and_topics: list[tuple[str, str, str]]) -> None:
    """Write an MCAP file with one channel per (schema_name, schema_text, topic)."""
    with open(path, "wb") as f:
        writer = Writer(f)
        writer.start()
        for schema_name, schema_text, topic in schemas_and_topics:
            schema_id = writer.register_schema(
                name=schema_name, encoding="ros2msg", data=schema_text.encode()
            )
            writer.register_channel(topic=topic, message_encoding="cdr", schema_id=schema_id)
        writer.finish()


class TestExpandMessageFields:
    def test_scalars(self):
        assert _leaves("float64 x\nuint32 seq\nstring status\n") == [
            ("x", "scalar", "float64"),
            ("seq", "scalar", "uint32"),
            ("status", "scalar", "string"),
        ]

    def test_nested_message_uses_dotted_paths(self):
        leaves = _leaves(IMU_SCHEMA)
        assert [leaf.field_path for leaf in leaves] == ["gyro.x", "gyro.y", "gyro.z", "temp"]

    def test_fixed_array_expands_per_element(self):
        assert [leaf.field_path for leaf in _leaves("float32[3] accel\n")] == [
            "accel[0]",
            "accel[1]",
            "accel[2]",
        ]

    def test_variable_array_is_one_complex_leaf(self):
        assert _leaves("int32[] samples\n") == [("samples", "complex", None)]

    def test_bounded_array_is_one_complex_leaf(self):
        assert _leaves("int32[<=4] samples\n") == [("samples", "complex", None)]

    def test_variable_array_of_messages_is_one_complex_leaf(self):
        schema = (
            "geometry_msgs/Vector3[] path\n"
            + "=" * 80
            + "\nMSG: geometry_msgs/Vector3\nfloat64 x\nfloat64 y\nfloat64 z\n"
        )
        assert _leaves(schema) == [("path", "complex", None)]

    def test_fixed_array_of_messages_expands_per_element(self):
        schema = (
            "geometry_msgs/Vector3[2] corners\n"
            + "=" * 80
            + "\nMSG: geometry_msgs/Vector3\nfloat64 x\nfloat64 y\nfloat64 z\n"
        )
        assert [leaf.field_path for leaf in _leaves(schema)] == [
            "corners[0].x",
            "corners[0].y",
            "corners[0].z",
            "corners[1].x",
            "corners[1].y",
            "corners[1].z",
        ]

    def test_time_and_duration_collapse_to_int64(self):
        leaves = _leaves("builtin_interfaces/Time stamp\nbuiltin_interfaces/Duration elapsed\n")
        assert [leaf.field_path for leaf in leaves] == ["stamp", "elapsed"]
        assert all(leaf.sift_type() == ChannelDataType.INT_64 for leaf in leaves)

    def test_constants_are_not_fields(self):
        assert _leaves("int32 STATUS_OK=0\nint32 status\n") == [
            ("status", "scalar", "int32"),
        ]

    def test_maps_every_ros2_scalar_type(self):
        # Narrow ints widen to 32-bit; byte and char are unsigned 8-bit in ROS 2.
        definition = "\n".join(
            f"{ros_type} f_{ros_type}"
            for ros_type in (
                "bool",
                "int8",
                "int16",
                "int32",
                "int64",
                "uint8",
                "uint16",
                "uint32",
                "uint64",
                "byte",
                "char",
                "float32",
                "float64",
                "string",
            )
        )
        assert [leaf.sift_type() for leaf in _leaves(definition)] == [
            ChannelDataType.BOOL,
            ChannelDataType.INT_32,
            ChannelDataType.INT_32,
            ChannelDataType.INT_32,
            ChannelDataType.INT_64,
            ChannelDataType.UINT_32,
            ChannelDataType.UINT_32,
            ChannelDataType.UINT_32,
            ChannelDataType.UINT_64,
            ChannelDataType.UINT_32,
            ChannelDataType.UINT_32,
            ChannelDataType.FLOAT,
            ChannelDataType.DOUBLE,
            ChannelDataType.STRING,
        ]

    def test_wstring_raises_unsupported(self):
        with pytest.raises(UnsupportedTopicError, match="wstring"):
            _leaves("wstring label\n")

    def test_variable_array_of_wstring_raises(self):
        # The importer decodes every element of a complex leaf, so an
        # undecodable element type makes the whole topic unsupported.
        with pytest.raises(UnsupportedTopicError, match="wstring"):
            _leaves("wstring[] labels\n")

    def test_variable_array_of_messages_with_wstring_raises(self):
        schema = "pkg/Bad[] items\n" + "=" * 80 + "\nMSG: pkg/Bad\nwstring label\n"
        with pytest.raises(UnsupportedTopicError, match="wstring"):
            _leaves(schema)

    def test_nesting_beyond_max_depth_raises(self):
        # A chain of 40 nested message types exceeds MAX_FIELD_DEPTH (32).
        parts = ["pkg/M0 child\n"]
        parts.extend("=" * 80 + f"\nMSG: pkg/M{i}\npkg/M{i + 1} child\n" for i in range(40))
        parts.append("=" * 80 + "\nMSG: pkg/M40\nfloat64 x\n")
        with pytest.raises(UnsupportedTopicError, match="nests deeper"):
            _leaves("".join(parts))

    def test_unknown_nested_type_raises(self):
        with pytest.raises(UnsupportedTopicError, match="unknown type"):
            _leaves("other_msgs/Missing part\n")

    def test_root_message_missing_raises(self):
        with pytest.raises(UnsupportedTopicError, match="root message"):
            parse_schema_defs(
                Schema(
                    id=1,
                    data=b"MSG: other_msgs/Other\nfloat64 x\n",
                    encoding="ros2msg",
                    name="test_msgs/msg/Test",
                )
            )


class TestDetectMcapTopics:
    def test_supported_topic_yields_leaves(self):
        warnings: list[str] = []
        topics = detect_mcap_topics({1: _schema(IMU_SCHEMA)}, [_channel("/imu")], warnings)
        assert [t.topic for t in topics] == ["/imu"]
        assert [leaf.field_path for leaf in topics[0].leaves] == [
            "gyro.x",
            "gyro.y",
            "gyro.z",
            "temp",
        ]
        assert warnings == []

    def test_non_cdr_encoding_skipped_with_warning(self):
        warnings: list[str] = []
        topics = detect_mcap_topics(
            {1: _schema(IMU_SCHEMA)}, [_channel("/imu", encoding="json")], warnings
        )
        assert topics == []
        assert any("only cdr is supported" in w for w in warnings)

    def test_non_ros2msg_schema_skipped_with_warning(self):
        schema = Schema(id=1, data=b"{}", encoding="jsonschema", name="Test")
        warnings: list[str] = []
        assert detect_mcap_topics({1: schema}, [_channel("/diag")], warnings) == []
        assert any("only ros2msg is supported" in w for w in warnings)

    def test_missing_schema_skipped_with_warning(self):
        warnings: list[str] = []
        assert detect_mcap_topics({}, [_channel("/imu")], warnings) == []
        assert any("no schema" in w for w in warnings)

    def test_case_colliding_topics_keep_first(self):
        schemas = {1: _schema(IMU_SCHEMA)}
        channels = [
            _channel("/imu", channel_id=1),
            _channel("/IMU", channel_id=2),
        ]
        warnings: list[str] = []
        topics = detect_mcap_topics(schemas, channels, warnings)
        assert [t.topic for t in topics] == ["/imu"]
        assert any("by case only" in w for w in warnings)

    def test_same_topic_channels_merge_when_identical(self):
        schemas = {1: _schema(IMU_SCHEMA)}
        channels = [_channel("/imu", channel_id=1), _channel("/imu", channel_id=2)]
        warnings: list[str] = []
        topics = detect_mcap_topics(schemas, channels, warnings)
        assert [t.topic for t in topics] == ["/imu"]
        assert warnings == []

    def test_same_topic_mismatched_schemas_skipped(self):
        schemas = {
            1: _schema(IMU_SCHEMA, schema_id=1),
            2: _schema("float64 other\n", schema_id=2),
        }
        channels = [
            _channel("/imu", schema_id=1, channel_id=1),
            _channel("/imu", schema_id=2, channel_id=2),
        ]
        warnings: list[str] = []
        assert detect_mcap_topics(schemas, channels, warnings) == []
        assert any("mismatched schemas" in w for w in warnings)


class TestDetectMcapConfig:
    def test_detects_channel_per_field(self, tmp_path):
        path = tmp_path / "log.mcap"
        _write_mcap(path, [("sensors/msg/Imu", IMU_SCHEMA, "/imu/data")])

        config = detect_mcap_config(path, asset_name="robot")
        assert config.asset_name == "robot"
        assert len(config.data) == 4
        channels = {(d.topic, d.field_path, d.name, d.data_type) for d in config.data}
        assert channels == {
            ("/imu/data", "gyro.x", "/imu/data.gyro.x", ChannelDataType.DOUBLE),
            ("/imu/data", "gyro.y", "/imu/data.gyro.y", ChannelDataType.DOUBLE),
            ("/imu/data", "gyro.z", "/imu/data.gyro.z", ChannelDataType.DOUBLE),
            ("/imu/data", "temp", "/imu/data.temp", ChannelDataType.DOUBLE),
        }

    def test_complex_field_expands_to_bytes_and_json(self, tmp_path):
        path = tmp_path / "log.mcap"
        _write_mcap(path, [("test_msgs/msg/Samples", "int32[] samples\n", "/samples")])

        config = detect_mcap_config(path)
        assert [(d.field_path, d.name, d.data_type) for d in config.data] == [
            ("samples", "/samples.samples", ChannelDataType.BYTES),
            ("samples", "/samples.samples.json", ChannelDataType.STRING),
        ]

    def test_unsupported_topic_warns_and_keeps_supported(self, tmp_path):
        path = tmp_path / "log.mcap"
        with open(path, "wb") as f:
            writer = Writer(f)
            writer.start()
            imu = writer.register_schema(
                name="sensors/msg/Imu", encoding="ros2msg", data=IMU_SCHEMA.encode()
            )
            writer.register_channel(topic="/imu", message_encoding="cdr", schema_id=imu)
            diag = writer.register_schema(name="diag", encoding="jsonschema", data=b"{}")
            writer.register_channel(topic="/diag", message_encoding="json", schema_id=diag)
            writer.finish()

        with pytest.warns(UserWarning, match="skipped unsupported topics"):
            config = detect_mcap_config(path)
        assert {d.topic for d in config.data} == {"/imu"}

    def test_clean_file_does_not_warn(self, tmp_path, recwarn):
        path = tmp_path / "log.mcap"
        _write_mcap(path, [("sensors/msg/Imu", IMU_SCHEMA, "/imu")])

        detect_mcap_config(path)
        assert not [w for w in recwarn.list if issubclass(w.category, UserWarning)]

    def test_truncated_file_scans_linearly(self, tmp_path):
        path = tmp_path / "log.mcap"
        _write_mcap(path, [("sensors/msg/Imu", IMU_SCHEMA, "/imu")])
        # Cutting the trailing magic invalidates the footer, so detection
        # falls back to a linear scan of the intact data section.
        data = path.read_bytes()
        path.write_bytes(data[:-8])

        with pytest.warns(UserWarning, match="stopped reading"):
            config = detect_mcap_config(path)
        assert {d.topic for d in config.data} == {"/imu"}

    def test_rejects_non_mcap_file(self, tmp_path):
        path = tmp_path / "log.mcap"
        path.write_bytes(b"NOTMCAP!" + b"\x00" * 64)
        with pytest.raises(ValueError, match="not an MCAP file"):
            detect_mcap_config(path)

    def test_records_only_in_data_section_are_detected(self, tmp_path):
        # An unchunked file whose summary omits the schema/channel repeats is
        # spec-legal; the top-level pass must pick the records up.
        path = tmp_path / "log.mcap"
        with open(path, "wb") as f:
            writer = Writer(f, use_chunking=False, repeat_channels=False, repeat_schemas=False)
            writer.start()
            schema_id = writer.register_schema(
                name="sensors/msg/Imu", encoding="ros2msg", data=IMU_SCHEMA.encode()
            )
            writer.register_channel(topic="/imu", message_encoding="cdr", schema_id=schema_id)
            writer.finish()

        config = detect_mcap_config(path)
        assert {d.topic for d in config.data} == {"/imu"}

    def test_rejects_unsupported_chunk_compression(self, tmp_path):
        # The importer rejects unsupported compression regardless of
        # parse_error_policy, so detection must too rather than listing
        # channels for a file that can never import.
        path = tmp_path / "log.mcap"
        with open(path, "wb") as f:
            writer = Writer(f)  # defaults to zstd-compressed chunks
            writer.start()
            schema_id = writer.register_schema(
                name="sensors/msg/Imu", encoding="ros2msg", data=IMU_SCHEMA.encode()
            )
            channel_id = writer.register_channel(
                topic="/imu", message_encoding="cdr", schema_id=schema_id
            )
            writer.add_message(channel_id=channel_id, log_time=0, data=b"\x00", publish_time=0)
            writer.finish()
        # Rewrite the chunk's compression string to an unsupported one.
        path.write_bytes(path.read_bytes().replace(b"zstd", b"lzma"))

        with pytest.raises(ValueError, match="unsupported chunk compression"):
            detect_mcap_config(path)

    def test_duplicate_generated_names_raise(self, tmp_path):
        # Topic '/a' with variable array field 'b' expands to '/a.b.json',
        # colliding with topic '/a.b' scalar field 'json'. The importer
        # rejects the file the same way.
        path = tmp_path / "log.mcap"
        _write_mcap(
            path,
            [
                ("pkg/msg/A", "int32[] b\n", "/a"),
                ("pkg/msg/B", "int32 json\n", "/a.b"),
            ],
        )
        with pytest.raises(ValueError, match="conflicts with channel"):
            detect_mcap_config(path)
