"""Tests for ULog channel detection."""

from __future__ import annotations

import struct

import pytest

from sift_client._internal.util.ulog import (
    DetectedUlogChannel,
    detect_ulog_config,
    detect_ulog_fields,
    expand_message_fields,
)
from sift_client.sift_types.channel import ChannelDataType


class _FakeFormat:
    """Stand-in for pyulog's MessageFormat: `fields` is a list of (type, array_size, name)."""

    def __init__(self, fields):
        self.fields = fields


class _FakeDataset:
    """Stand-in for pyulog's Data: a decoded topic instance."""

    def __init__(self, name, multi_id):
        self.name = name
        self.multi_id = multi_id


class _FakeUlog:
    """Stand-in for a parsed pyulog ULog: only the attributes detection reads."""

    def __init__(
        self,
        message_formats=None,
        data_list=None,
        logged_messages=None,
        logged_messages_tagged=None,
    ):
        self.message_formats = message_formats or {}
        self.data_list = data_list or []
        self.logged_messages = logged_messages or []
        self.logged_messages_tagged = logged_messages_tagged or {}


# Builders for real ULog bytes; the TestDetectUlogConfig cases parse these with pyulog.


def _header() -> bytes:
    # 7 magic bytes + 1 version byte + uint64 start timestamp.
    return b"\x55\x4c\x6f\x67\x01\x12\x35" + b"\x01" + struct.pack("<Q", 0)


def _message(type_char: str, payload: bytes) -> bytes:
    return struct.pack("<HB", len(payload), ord(type_char)) + payload


def _flag_bits() -> bytes:
    # 'B': compat[8], incompat[8], appended offsets[24]; must follow the header.
    return _message("B", b"\x00" * 40)


def _format(definition: str) -> bytes:
    return _message("F", definition.encode())


def _add_logged(multi_id: int, msg_id: int, name: str) -> bytes:
    return _message("A", bytes([multi_id]) + struct.pack("<H", msg_id) + name.encode())


def _data_record(msg_id: int, values: bytes) -> bytes:
    return _message("D", struct.pack("<H", msg_id) + values)


def _logged_string(text: str = "boot") -> bytes:
    # 'L': log_level[1], timestamp[8], message.
    return _message("L", b"\x06" + struct.pack("<Q", 0) + text.encode())


def _tagged_logged_string(tag: int, text: str = "tagged") -> bytes:
    # 'C': log_level[1], tag[2], timestamp[8], message.
    return _message("C", b"\x06" + struct.pack("<H", tag) + struct.pack("<Q", 0) + text.encode())


ACCEL_FORMAT = _format("sensor_accel:uint64_t timestamp;float x;float y;float z;")
ACCEL_RECORD = struct.pack("<Qfff", 100, 1.0, 2.0, 3.0)


def _accel_log() -> bytes:
    """A minimal well-formed log: one topic, one record."""
    return (
        _header()
        + _flag_bits()
        + ACCEL_FORMAT
        + _add_logged(0, 0, "sensor_accel")
        + _data_record(0, ACCEL_RECORD)
    )


class TestExpandMessageFields:
    def test_scalars(self):
        formats = {
            "sensor_accel": _FakeFormat(
                [("uint64_t", 0, "timestamp"), ("float", 0, "x"), ("float", 0, "y")]
            )
        }
        assert expand_message_fields(formats, "sensor_accel") == [
            ("timestamp", "uint64_t"),
            ("x", "float"),
            ("y", "float"),
        ]

    def test_array_expands_per_element(self):
        formats = {"gyro": _FakeFormat([("float", 3, "gyro_rad")])}
        assert expand_message_fields(formats, "gyro") == [
            ("gyro_rad[0]", "float"),
            ("gyro_rad[1]", "float"),
            ("gyro_rad[2]", "float"),
        ]

    def test_char_scalar_and_array_collapse(self):
        formats = {"m": _FakeFormat([("char", 0, "a"), ("char", 16, "b")])}
        assert expand_message_fields(formats, "m") == [("a", "char"), ("b", "char")]

    def test_nested_message_recurses(self):
        formats = {
            "outer": _FakeFormat([("inner", 0, "current")]),
            "inner": _FakeFormat([("double", 0, "lat"), ("double", 0, "lon")]),
        }
        assert expand_message_fields(formats, "outer") == [
            ("current.lat", "double"),
            ("current.lon", "double"),
        ]

    def test_nested_message_array(self):
        formats = {
            "report": _FakeFormat([("esc", 2, "esc")]),
            "esc": _FakeFormat([("int32_t", 0, "rpm")]),
        }
        assert expand_message_fields(formats, "report") == [
            ("esc[0].rpm", "int32_t"),
            ("esc[1].rpm", "int32_t"),
        ]


class TestDetectUlogFields:
    def test_skips_timestamp_and_padding(self):
        ulog = _FakeUlog(
            message_formats={
                "sensor_accel": _FakeFormat(
                    [
                        ("uint64_t", 0, "timestamp"),
                        ("float", 0, "x"),
                        ("uint8_t", 0, "_padding0"),
                    ]
                )
            },
            data_list=[_FakeDataset("sensor_accel", 0)],
        )
        assert detect_ulog_fields(ulog) == {
            "sensor_accel_0.x": DetectedUlogChannel("sensor_accel", 0, "x", "float")
        }

    def test_skips_message_without_timestamp(self):
        ulog = _FakeUlog(
            message_formats={"no_time": _FakeFormat([("float", 0, "x")])},
            data_list=[_FakeDataset("no_time", 0)],
        )
        assert detect_ulog_fields(ulog) == {}

    def test_log_message_channels_sorted_by_tag(self):
        ulog = _FakeUlog(logged_messages=["boot"], logged_messages_tagged={2: [], 9: [], 5: []})
        assert list(detect_ulog_fields(ulog).items()) == [
            ("log_messages", DetectedUlogChannel("log_messages", 0, "", "char")),
            ("log_messages_2", DetectedUlogChannel("log_messages_2", 0, "", "char")),
            ("log_messages_5", DetectedUlogChannel("log_messages_5", 0, "", "char")),
            ("log_messages_9", DetectedUlogChannel("log_messages_9", 0, "", "char")),
        ]


class TestDetectUlogConfig:
    def test_detects_channel_per_field(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(_accel_log())

        config = detect_ulog_config(path, asset_name="drone")
        assert config.asset_name == "drone"
        channels = {
            (d.message_name, d.instance, d.field_name, d.name, d.data_type) for d in config.data
        }
        assert channels == {
            ("sensor_accel", 0, "x", "sensor_accel_0.x", ChannelDataType.FLOAT),
            ("sensor_accel", 0, "y", "sensor_accel_0.y", ChannelDataType.FLOAT),
            ("sensor_accel", 0, "z", "sensor_accel_0.z", ChannelDataType.FLOAT),
        }

    def test_multi_instance_topics_stay_separate(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(
            _header()
            + _flag_bits()
            + ACCEL_FORMAT
            + _add_logged(0, 0, "sensor_accel")
            + _add_logged(1, 1, "sensor_accel")
            + _data_record(0, ACCEL_RECORD)
            + _data_record(1, ACCEL_RECORD)
        )

        config = detect_ulog_config(path)
        assert all(d.message_name == "sensor_accel" for d in config.data)
        assert {(d.instance, d.field_name) for d in config.data} == {
            (0, "x"),
            (0, "y"),
            (0, "z"),
            (1, "x"),
            (1, "y"),
            (1, "z"),
        }

    def test_log_message_channels_select_by_name_with_empty_field(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(_header() + _flag_bits() + _logged_string() + _tagged_logged_string(5))

        config = detect_ulog_config(path)
        channels = {
            (d.message_name, d.instance, d.field_name, d.name, d.data_type) for d in config.data
        }
        assert channels == {
            ("log_messages", 0, "", "log_messages", ChannelDataType.STRING),
            ("log_messages_5", 0, "", "log_messages_5", ChannelDataType.STRING),
        }

    def test_maps_every_ulog_scalar_type(self, tmp_path):
        # Narrow ints widen to 32-bit, char[N] imports as one string.
        path = tmp_path / "log.ulg"
        fmt = _format(
            "types:uint64_t timestamp;int8_t i8;int16_t i16;int32_t i32;int64_t i64;"
            "uint8_t u8;uint16_t u16;uint32_t u32;uint64_t u64;"
            "float f;double d;bool b;char[4] s;"
        )
        record = struct.pack(
            "<QbhiqBHIQfd?4s", 100, -1, -2, -3, -4, 1, 2, 3, 4, 1.5, 2.5, True, b"abcd"
        )
        path.write_bytes(
            _header() + _flag_bits() + fmt + _add_logged(0, 0, "types") + _data_record(0, record)
        )

        config = detect_ulog_config(path)
        assert {(d.field_name, d.data_type) for d in config.data} == {
            ("i8", ChannelDataType.INT_32),
            ("i16", ChannelDataType.INT_32),
            ("i32", ChannelDataType.INT_32),
            ("i64", ChannelDataType.INT_64),
            ("u8", ChannelDataType.UINT_32),
            ("u16", ChannelDataType.UINT_32),
            ("u32", ChannelDataType.UINT_32),
            ("u64", ChannelDataType.UINT_64),
            ("f", ChannelDataType.FLOAT),
            ("d", ChannelDataType.DOUBLE),
            ("b", ChannelDataType.BOOL),
            ("s", ChannelDataType.STRING),
        }

    def test_clean_file_does_not_warn(self, tmp_path, recwarn):
        path = tmp_path / "log.ulg"
        path.write_bytes(_accel_log())

        detect_ulog_config(path)
        assert not [w for w in recwarn.list if issubclass(w.category, UserWarning)]

    def test_topic_without_records_is_dropped(self, tmp_path):
        # pyulog only lists topics with decoded records, so a subscription
        # that never logged data yields no channels.
        path = tmp_path / "log.ulg"
        path.write_bytes(
            _header() + _flag_bits() + ACCEL_FORMAT + _add_logged(0, 0, "sensor_accel")
        )

        config = detect_ulog_config(path)
        assert config.data == []

    def test_undersized_record_warns_and_is_dropped(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(
            _header()
            + _flag_bits()
            + ACCEL_FORMAT
            + _add_logged(0, 0, "sensor_accel")
            + _data_record(0, ACCEL_RECORD[:4])
        )

        with pytest.warns(UserWarning, match="could not decode"):
            config = detect_ulog_config(path)
        assert config.data == []

    def test_unknown_msg_id_warns_but_keeps_valid_channels(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(_accel_log() + _data_record(99, ACCEL_RECORD))

        with pytest.warns(UserWarning, match="could not decode"):
            config = detect_ulog_config(path)
        assert {d.channel for d in config.data} == {
            "sensor_accel_0.x",
            "sensor_accel_0.y",
            "sensor_accel_0.z",
        }

    def test_rejects_non_ulog_file(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(b"NOTULOG!" + b"\x00" * 8)
        with pytest.raises(ValueError, match="could not be parsed as ULog"):
            detect_ulog_config(path)
