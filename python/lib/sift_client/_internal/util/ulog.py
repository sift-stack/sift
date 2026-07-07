"""Detect channels in PX4 ULog (``.ulg``) files.

Detection parses the file with pyulog and enumerates the decoded topics,
multi-instance IDs, and logged-string channels.

Warning: pyulog silently drops what it cannot decode, so malformed records
and topics that never logged a decodable record are missing from the detected
channels. An import config with an empty ``data`` list still imports every
channel in the file.
"""

from __future__ import annotations

import contextlib
import sys
import warnings
from pathlib import Path

from pyulog import ULog

from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import UlogDataColumn, UlogImportConfig

# Map ULog C scalars to Sift channel types. Smaller ints widen to 32-bit, and
# char fields import as strings.
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

# Base channel for ULog logged strings; tagged logs use ``log_messages_<tag>``.
LOG_MESSAGES_CHANNEL = "log_messages"


def _is_padding(channel_key: str) -> bool:
    """Return whether a channel key contains a ULog padding field."""
    return any(seg.startswith("_padding") for seg in channel_key.split("."))


def expand_message_fields(message_formats: dict, message_name: str) -> list[tuple[str, str]]:
    """Flatten a message format into ``(field_key, c_type)`` leaf entries.

    Arrays expand to ``field[i]``, nested messages use dotted paths, and
    ``char[N]`` stays one string field.
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


def detect_ulog_fields(ulog: ULog) -> dict[str, str]:
    """Return importable channels as ``{channel_key: c_type}``.

    Decoded topics use ``<message>_<multi_id>.<field>``. The timestamp axis
    and padding fields are excluded; logged strings become ``log_messages``
    or ``log_messages_<tag>``.
    """
    fields: dict[str, str] = {}
    for dataset in ulog.data_list:
        # No top-level timestamp means no usable time axis.
        if not any(f[2] == "timestamp" for f in ulog.message_formats[dataset.name].fields):
            continue
        prefix = f"{dataset.name}_{dataset.multi_id}"
        for key, type_str in expand_message_fields(ulog.message_formats, dataset.name):
            # timestamp is the time axis; _padding fields are alignment bytes.
            if key == "timestamp" or _is_padding(key):
                continue
            fields[f"{prefix}.{key}"] = type_str
    if ulog.logged_messages:
        fields[LOG_MESSAGES_CHANNEL] = "char"
    for tag in sorted(ulog.logged_messages_tagged):
        fields[f"{LOG_MESSAGES_CHANNEL}_{tag}"] = "char"
    return fields


def _parse_ulog(path: Path) -> ULog:
    """Fully parse the file with pyulog, data records included."""
    # pyulog logs parse notes to stdout; keep the client's stdout clean.
    with contextlib.redirect_stdout(sys.stderr):
        try:
            return ULog(str(path))
        except Exception as e:
            raise ValueError(f"'{path.name}' could not be parsed as ULog: {e}") from e


def detect_ulog_config(file_path: str | Path, asset_name: str = "") -> UlogImportConfig:
    """Detect a ULog import config by enumerating the file's channels.

    Channels come from what pyulog decodes, so malformed records and topics
    without decodable records are missing from the result; a warning is
    emitted when pyulog reports corruption. On files with parse errors the
    result can also differ from the channels the import finds; if the import
    rejects the returned channel list, clear ``data`` to import all channels.

    Args:
        file_path: Path to the ``.ulg`` file.
        asset_name: The asset name to set on the config.

    Returns:
        A config whose ``data`` lists detected channels with default Sift names
        and data types. Remove entries to skip channels, or edit entries before
        importing. Leaving ``data`` empty imports all channels with the same
        defaults.
    """
    path = Path(file_path)
    ulog = _parse_ulog(path)
    if ulog.file_corruption:
        warnings.warn(
            f"'{path.name}' has records pyulog could not decode and dropped; "
            "the detected channels may be incomplete.",
            stacklevel=2,
        )
    detected = detect_ulog_fields(ulog)
    data = [
        UlogDataColumn(channel=key, name=key, data_type=ULOG_TO_SIFT_TYPE[c_type])
        for key, c_type in detected.items()
    ]
    return UlogImportConfig(asset_name=asset_name, data=data)
