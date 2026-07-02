"""Tests for ULog channel detection."""

from __future__ import annotations

import struct

import pytest

from sift_client._internal.util.ulog import (
    ULOG_HEADER_SIZE,
    ULOG_MAGIC,
    ULOG_TO_SIFT_TYPE,
    _scan_subscriptions,
    _ScanResult,
    detect_ulog_config,
    detect_ulog_fields,
    expand_message_fields,
)
from sift_client.sift_types.channel import ChannelDataType


class _FakeFormat:
    """Stand-in for pyulog's MessageFormat: a list of (type, array_size, name)."""

    def __init__(self, fields):
        self.fields = fields


def _header() -> bytes:
    # 7 magic bytes + 1 version byte + uint64 start timestamp.
    return ULOG_MAGIC[:7] + b"\x00" + struct.pack("<Q", 0)


def _message(type_char: str, payload: bytes) -> bytes:
    return struct.pack("<HB", len(payload), ord(type_char)) + payload


def _add_logged(multi_id: int, msg_id: int, name: str) -> bytes:
    payload = bytes([multi_id]) + struct.pack("<H", msg_id) + name.encode()
    return _message("A", payload)


def _data_record(msg_id: int, body: bytes = b"\x00\x00\x00\x00") -> bytes:
    return _message("D", struct.pack("<H", msg_id) + body)


def _logged_string(text: str = "boot") -> bytes:
    # 'L': log_level[1], timestamp[8], message.
    return _message("L", b"\x06" + struct.pack("<Q", 0) + text.encode())


def _tagged_logged_string(tag: int, text: str = "tagged") -> bytes:
    # 'C': log_level[1], tag[2], timestamp[8], message.
    return _message("C", b"\x06" + struct.pack("<H", tag) + struct.pack("<Q", 0) + text.encode())


class TestScanSubscriptions:
    def test_collects_subscriptions_in_order(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(
            _header()
            + _add_logged(0, 10, "sensor_accel")
            + _add_logged(1, 11, "sensor_accel")
            + _data_record(10)
            + _add_logged(0, 12, "vehicle_status")
        )
        scan = _scan_subscriptions(path)
        assert scan.subscriptions == [
            ("sensor_accel", 0),
            ("sensor_accel", 1),
            ("vehicle_status", 0),
        ]

    def test_collects_log_markers(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(
            _header()
            + _add_logged(0, 10, "sensor_accel")
            + _logged_string()
            + _tagged_logged_string(2)
            + _tagged_logged_string(5)
        )
        scan = _scan_subscriptions(path)
        assert scan.has_untagged_logs is True
        assert scan.log_tags == {2, 5}

    def test_tolerates_truncated_final_record(self, tmp_path):
        path = tmp_path / "log.ulg"
        # A valid subscription, then a header that claims more bytes than remain.
        truncated = struct.pack("<HB", 999, ord("D")) + b"\x0a\x00"
        path.write_bytes(_header() + _add_logged(0, 10, "sensor_accel") + truncated)
        scan = _scan_subscriptions(path)
        assert scan.subscriptions == [("sensor_accel", 0)]

    def test_rejects_bad_magic(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(b"NOTULOG!" + b"\x00" * ULOG_HEADER_SIZE)
        with pytest.raises(ValueError, match="bad magic bytes"):
            _scan_subscriptions(path)

    def test_rejects_too_small(self, tmp_path):
        path = tmp_path / "log.ulg"
        path.write_bytes(b"\x55\x4c")
        with pytest.raises(ValueError, match="invalid size"):
            _scan_subscriptions(path)


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
        formats = {
            "sensor_accel": _FakeFormat(
                [
                    ("uint64_t", 0, "timestamp"),
                    ("float", 0, "x"),
                    ("uint8_t", 0, "_padding0"),
                ]
            )
        }
        scan = _ScanResult()
        scan.subscriptions = [("sensor_accel", 0)]
        assert detect_ulog_fields(formats, scan) == {"sensor_accel_0.x": "float"}

    def test_multi_id_in_prefix(self):
        formats = {"sensor_accel": _FakeFormat([("uint64_t", 0, "timestamp"), ("float", 0, "x")])}
        scan = _ScanResult()
        scan.subscriptions = [("sensor_accel", 0), ("sensor_accel", 1)]
        detected = detect_ulog_fields(formats, scan)
        assert set(detected) == {"sensor_accel_0.x", "sensor_accel_1.x"}

    def test_skips_message_without_timestamp(self):
        formats = {"no_time": _FakeFormat([("float", 0, "x")])}
        scan = _ScanResult()
        scan.subscriptions = [("no_time", 0)]
        assert detect_ulog_fields(formats, scan) == {}

    def test_skips_subscription_without_format(self):
        scan = _ScanResult()
        scan.subscriptions = [("missing", 0)]
        assert detect_ulog_fields({}, scan) == {}

    def test_log_message_channels(self):
        scan = _ScanResult()
        scan.has_untagged_logs = True
        scan.log_tags = {5, 2}
        detected = detect_ulog_fields({}, scan)
        assert detected == {
            "log_messages": "char",
            "log_messages_2": "char",
            "log_messages_5": "char",
        }


class TestTypeMapping:
    def test_narrow_ints_widen(self):
        assert ULOG_TO_SIFT_TYPE["int8_t"] == ChannelDataType.INT_32
        assert ULOG_TO_SIFT_TYPE["uint16_t"] == ChannelDataType.UINT_32
        assert ULOG_TO_SIFT_TYPE["int64_t"] == ChannelDataType.INT_64
        assert ULOG_TO_SIFT_TYPE["uint64_t"] == ChannelDataType.UINT_64

    def test_char_is_string(self):
        assert ULOG_TO_SIFT_TYPE["char"] == ChannelDataType.STRING


class TestDetectUlogConfig:
    def test_end_to_end_minimal_log(self, tmp_path):
        """A minimal .ulg with one format and one subscription yields one
        channel per non-timestamp field, mapped to the right type.
        """
        flag_bits = _message("B", b"\x00" * 40)
        fmt = _message("F", b"sensor_accel:uint64_t timestamp;float x;float y;float z;")
        path = tmp_path / "log.ulg"
        path.write_bytes(
            _header() + flag_bits + fmt + _add_logged(0, 0, "sensor_accel") + _data_record(0)
        )

        config = detect_ulog_config(path, asset_name="drone")
        assert config.asset_name == "drone"
        channels = {(d.channel, d.name, d.data_type) for d in config.data}
        assert channels == {
            ("sensor_accel_0.x", "sensor_accel_0.x", ChannelDataType.FLOAT),
            ("sensor_accel_0.y", "sensor_accel_0.y", ChannelDataType.FLOAT),
            ("sensor_accel_0.z", "sensor_accel_0.z", ChannelDataType.FLOAT),
        }
