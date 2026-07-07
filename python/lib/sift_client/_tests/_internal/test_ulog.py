"""Tests for ULog channel detection."""

from __future__ import annotations

import struct

import pytest

from sift_client._internal.util.ulog import (
    ULOG_TO_SIFT_TYPE,
    detect_ulog_config,
    detect_ulog_fields,
    expand_message_fields,
)
from sift_client.sift_types.channel import ChannelDataType


class _FakeFormat:
    """Stand-in for pyulog's MessageFormat: a list of (type, array_size, name)."""

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


# Builders for real ULog bytes, parsed by pyulog in the end-to-end tests.


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
        assert detect_ulog_fields(ulog) == {"sensor_accel_0.x": "float"}

    def test_multi_id_in_prefix(self):
        ulog = _FakeUlog(
            message_formats={
                "sensor_accel": _FakeFormat([("uint64_t", 0, "timestamp"), ("float", 0, "x")])
            },
            data_list=[_FakeDataset("sensor_accel", 0), _FakeDataset("sensor_accel", 1)],
        )
        assert set(detect_ulog_fields(ulog)) == {"sensor_accel_0.x", "sensor_accel_1.x"}

    def test_skips_message_without_timestamp(self):
        ulog = _FakeUlog(
            message_formats={"no_time": _FakeFormat([("float", 0, "x")])},
            data_list=[_FakeDataset("no_time", 0)],
        )
        assert detect_ulog_fields(ulog) == {}

    def test_log_message_channels_sorted_by_tag(self):
        ulog = _FakeUlog(logged_messages=["boot"], logged_messages_tagged={2: [], 9: [], 5: []})
        assert list(detect_ulog_fields(ulog).items()) == [
            ("log_messages", "char"),
            ("log_messages_2", "char"),
            ("log_messages_5", "char"),
            ("log_messages_9", "char"),
        ]


class TestTypeMapping:
    def test_narrow_ints_widen(self):
        assert ULOG_TO_SIFT_TYPE["int8_t"] == ChannelDataType.INT_32
        assert ULOG_TO_SIFT_TYPE["uint16_t"] == ChannelDataType.UINT_32
        assert ULOG_TO_SIFT_TYPE["int64_t"] == ChannelDataType.INT_64
        assert ULOG_TO_SIFT_TYPE["uint64_t"] == ChannelDataType.UINT_64

    def test_char_is_string(self):
        assert ULOG_TO_SIFT_TYPE["char"] == ChannelDataType.STRING


class TestDetectUlogConfig:
    def test_detects_channel_per_field(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(
            _header()
            + _flag_bits()
            + ACCEL_FORMAT
            + _add_logged(0, 0, "sensor_accel")
            + _data_record(0, ACCEL_RECORD)
        )

        config = detect_ulog_config(path, asset_name="drone")
        assert config.asset_name == "drone"
        channels = {(d.channel, d.name, d.data_type) for d in config.data}
        assert channels == {
            ("sensor_accel_0.x", "sensor_accel_0.x", ChannelDataType.FLOAT),
            ("sensor_accel_0.y", "sensor_accel_0.y", ChannelDataType.FLOAT),
            ("sensor_accel_0.z", "sensor_accel_0.z", ChannelDataType.FLOAT),
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
        assert {d.channel for d in config.data} == {
            "sensor_accel_0.x",
            "sensor_accel_0.y",
            "sensor_accel_0.z",
            "sensor_accel_1.x",
            "sensor_accel_1.y",
            "sensor_accel_1.z",
        }

    def test_log_message_channels(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(_header() + _flag_bits() + _logged_string() + _tagged_logged_string(5))

        config = detect_ulog_config(path)
        channels = {(d.channel, d.data_type) for d in config.data}
        assert channels == {
            ("log_messages", ChannelDataType.STRING),
            ("log_messages_5", ChannelDataType.STRING),
        }

    def test_bool_and_char_fields(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(
            _header()
            + _flag_bits()
            + _format("status:uint64_t timestamp;bool armed;char[4] name;")
            + _add_logged(0, 0, "status")
            + _data_record(0, struct.pack("<Q?", 100, True) + b"abcd")
        )

        config = detect_ulog_config(path)
        assert {(d.channel, d.data_type) for d in config.data} == {
            ("status_0.armed", ChannelDataType.BOOL),
            ("status_0.name", ChannelDataType.STRING),
        }

    def test_clean_file_does_not_warn(self, tmp_path, recwarn):
        path = tmp_path / "log.ulg"
        path.write_bytes(
            _header()
            + _flag_bits()
            + ACCEL_FORMAT
            + _add_logged(0, 0, "sensor_accel")
            + _data_record(0, ACCEL_RECORD)
        )

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
        path.write_bytes(
            _header()
            + _flag_bits()
            + ACCEL_FORMAT
            + _add_logged(0, 0, "sensor_accel")
            + _data_record(0, ACCEL_RECORD)
            + _data_record(99, ACCEL_RECORD)
        )

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

    def test_rejects_too_small_file(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(b"\x55\x4c")
        with pytest.raises(ValueError, match="could not be parsed as ULog"):
            detect_ulog_config(path)
