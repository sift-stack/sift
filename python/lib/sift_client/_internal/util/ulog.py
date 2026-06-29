"""ULog (PX4 .ulg) channel detection.

ULog files are self-describing: the embedded schema names every channel and its
C type, so detection enumerates channels with no user input. This mirrors the
server-side importer so a detected config imports identically.

Detection never decodes data records. It walks the message stream reading only
the AddLogged ('A') subscriptions and logged-string ('L'/'C') markers, and
parses the Definitions section (via pyulog, header-only) for the message
formats. The channel naming, type mapping, and field flattening match the
server importer field for field.
"""

from __future__ import annotations

import contextlib
import struct
import sys
from pathlib import Path
from typing import TYPE_CHECKING

from pyulog import ULog

from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import UlogDataColumn, UlogImportConfig

if TYPE_CHECKING:
    from typing import BinaryIO

# ULog C scalar types to Sift channel types. Narrow integers widen to 32-bit
# and char/char[N] become STRING, matching the server importer and the other
# import formats.
ULOG_TO_SIFT_TYPE: dict[str, ChannelDataType] = {
    "int8_t": ChannelDataType.INT_32,
    "int16_t": ChannelDataType.INT_32,
    "int32_t": ChannelDataType.INT_32,
    "int64_t": ChannelDataType.INT_64,
    "uint8_t": ChannelDataType.UINT_32,
    "uint16_t": ChannelDataType.UINT_32,
    "uint32_t": ChannelDataType.UINT_32,
    "uint64_t": ChannelDataType.UINT_64,
    "float": ChannelDataType.FLOAT,
    "double": ChannelDataType.DOUBLE,
    "bool": ChannelDataType.BOOL,
    "char": ChannelDataType.STRING,
}

# Channel that collects printf-style logged strings ('L' and tagged 'C').
LOG_MESSAGES_CHANNEL = "log_messages"

ULOG_MAGIC = b"\x55\x4c\x6f\x67\x01\x12\x35"
ULOG_HEADER_SIZE = 16
# Full sync message: 3-byte header (size=8, type 'S') + the 8 sync magic bytes.
SYNC_MESSAGE = struct.pack("<HB", 8, ord("S")) + bytes(
    [0x2F, 0x73, 0x13, 0x20, 0x25, 0x0C, 0xBB, 0x12]
)
KNOWN_MESSAGE_TYPES = frozenset(b"BFIMPQARDLCSO")


def _is_padding(channel_key: str) -> bool:
    """True if any dotted segment of the key is a ULog padding field."""
    return any(seg.startswith("_padding") for seg in channel_key.split("."))


def _find_next_sync(f: BinaryIO, start: int) -> int:
    """Absolute offset of the next sync marker at or after ``start``, or -1."""
    chunk_size = 1 << 16
    overlap = len(SYNC_MESSAGE) - 1
    f.seek(start)
    chunk_start = start
    carry = b""
    while True:
        chunk = f.read(chunk_size)
        if not chunk:
            return -1
        buf = carry + chunk
        idx = buf.find(SYNC_MESSAGE)
        if idx != -1:
            return chunk_start - len(carry) + idx
        chunk_start += len(chunk)
        carry = buf[-overlap:]


class _ScanResult:
    """Structural findings the header parse does not surface: which
    ``(message_name, multi_id)`` pairs were logged and whether the file carries
    logged-string channels."""

    def __init__(self) -> None:
        self.subscriptions: list[tuple[str, int]] = []
        self.has_untagged_logs = False
        self.log_tags: set[int] = set()


def _scan_subscriptions(path: Path) -> _ScanResult:
    """Walk the message stream for subscriptions and logged-string markers.

    Reads only 'A', 'L', and 'C' payloads and skips 'D' data records, so the
    cost is one streaming pass with no data decode. Tolerates a truncated final
    record and reframes past garbage at the next sync marker, so a partially
    corrupt log still enumerates its channels.
    """
    result = _ScanResult()
    size = path.stat().st_size
    if size < ULOG_HEADER_SIZE:
        raise ValueError(f"'{path.name}' is not a ULog file (invalid size).")

    with open(path, "rb") as f:
        if f.read(7) != ULOG_MAGIC[:7]:
            raise ValueError(f"'{path.name}' is not a ULog file (bad magic bytes).")

        pos = ULOG_HEADER_SIZE
        f.seek(pos)
        while pos < size:
            if pos + 3 > size:
                break  # truncated message header
            msg_size, msg_type = struct.unpack("<HB", f.read(3))

            if msg_type not in KNOWN_MESSAGE_TYPES:
                # Lost framing: reframe at the next sync marker.
                next_sync = _find_next_sync(f, pos)
                if next_sync == -1:
                    break
                pos = next_sync
                f.seek(pos)
                continue

            if pos + 3 + msg_size > size:
                break  # truncated final record

            consumed = 0
            if msg_type == ord("A"):
                # 'A' (add subscription): multi_id[1], msg_id[2], message_name.
                if msg_size >= 4:
                    payload = f.read(msg_size)
                    consumed = msg_size
                    multi_id = payload[0]
                    name = payload[3:msg_size].decode("utf-8", errors="replace")
                    result.subscriptions.append((name, multi_id))
            elif msg_type == ord("L"):
                result.has_untagged_logs = True
            elif msg_type == ord("C"):
                # 'C' (tagged logged string): log_level[1], tag[2], ...
                if msg_size >= 3:
                    (tag,) = struct.unpack_from("<H", f.read(3), 1)
                    consumed = 3
                    result.log_tags.add(tag)

            if consumed < msg_size:
                f.seek(msg_size - consumed, 1)
            pos += 3 + msg_size

    return result


def expand_message_fields(message_formats: dict, message_name: str) -> list[tuple[str, str]]:
    """Flatten one message format into ``(field_key, c_type)`` leaf entries.

    Scalars yield one entry; ``type[N]`` arrays yield one per element with the
    index in brackets (``gyro_rad[0]``); ``char``/``char[N]`` collapse to a
    single ``char`` entry; nested message types recurse to dotted leaf paths.
    """
    flattened: list[tuple[str, str]] = []

    def walk(prefix: str, type_name: str) -> None:
        for field_type, array_size, field_name in message_formats[type_name].fields:
            if field_type == "char":
                # char and char[N] both collapse to a single STRING channel.
                flattened.append((f"{prefix}{field_name}", "char"))
            elif field_type in ULOG_TO_SIFT_TYPE:
                if array_size > 0:
                    flattened.extend(
                        (f"{prefix}{field_name}[{i}]", field_type) for i in range(array_size)
                    )
                else:
                    flattened.append((f"{prefix}{field_name}", field_type))
            else:  # nested message type
                if array_size > 0:
                    for i in range(array_size):
                        walk(f"{prefix}{field_name}[{i}].", field_type)
                else:
                    walk(f"{prefix}{field_name}.", field_type)

    walk("", message_name)
    return flattened


def detect_ulog_fields(message_formats: dict, scan: _ScanResult) -> dict[str, str]:
    """Enumerate importable channels as ``{channel_key: c_type}``.

    Each logged ``(message_name, multi_id)`` contributes its non-timestamp,
    non-padding fields under the ``<message>_<multi_id>.<field>`` prefix.
    Logged-string channels are appended as ``log_messages`` and
    ``log_messages_<tag>``.
    """
    fields: dict[str, str] = {}
    for message_name, multi_id in scan.subscriptions:
        # Truncated headers can leave a subscription without a format.
        if message_name not in message_formats:
            continue
        # No top-level timestamp means no usable time axis.
        if not any(f[2] == "timestamp" for f in message_formats[message_name].fields):
            continue
        prefix = f"{message_name}_{multi_id}"
        for key, type_str in expand_message_fields(message_formats, message_name):
            # timestamp is the time axis; _padding fields are alignment bytes.
            if key == "timestamp" or _is_padding(key):
                continue
            fields[f"{prefix}.{key}"] = type_str
    if scan.has_untagged_logs:
        fields[LOG_MESSAGES_CHANNEL] = "char"
    for tag in sorted(scan.log_tags):
        fields[f"{LOG_MESSAGES_CHANNEL}_{tag}"] = "char"
    return fields


def _parse_header(path: Path) -> ULog:
    """Parse only the ULog Definitions section (no data records)."""
    # pyulog logs format notes to stdout; keep the client's stdout clean.
    with contextlib.redirect_stdout(sys.stderr):
        return ULog(str(path), parse_header_only=True)


def detect_ulog_config(file_path: str | Path, asset_name: str = "") -> UlogImportConfig:
    """Detect a ULog import config by enumerating the file's channels.

    Args:
        file_path: Path to the ``.ulg`` file.
        asset_name: The asset name to set on the config.

    Returns:
        A UlogImportConfig whose ``data`` lists every detected channel with its
        default name (the channel key) and mapped data type. Drop, rename, or
        retype entries before importing; an empty ``data`` list imports all
        channels with these same defaults server-side.
    """
    path = Path(file_path)
    scan = _scan_subscriptions(path)
    header = _parse_header(path)
    detected = detect_ulog_fields(header.message_formats, scan)
    data = [
        UlogDataColumn(channel=key, name=key, data_type=ULOG_TO_SIFT_TYPE[c_type])
        for key, c_type in detected.items()
    ]
    return UlogImportConfig(asset_name=asset_name, data=data)
