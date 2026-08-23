"""Detect channels in MCAP (``.mcap``) files.

Detection reads the file's schema and channel records without decoding
message payloads, then flattens each topic's ros2msg schema into importable
leaf fields, mirroring the server importer's rules.

See ``detect_mcap_config`` for caveats on unsupported topics and empty
``data`` behavior.
"""

from __future__ import annotations

import warnings
from collections import defaultdict
from pathlib import Path
from typing import NamedTuple

from mcap import records as mcap_records
from mcap.reader import make_reader
from mcap.stream_reader import StreamReader
from mcap_ros2 import _dynamic as ros2_dynamic

from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import McapDataColumn, McapImportConfig

MCAP_MAGIC = b"\x89MCAP0\r\n"

# The importer rejects chunk compressions outside this set.
SUPPORTED_COMPRESSIONS = frozenset(("", "zstd", "lz4"))

# ROS 2 scalar types to Sift channel types. Narrow integers widen to 32-bit
# like the other import types. byte and char both map to UINT_32: ROS 2
# defines them as unsigned 8-bit (octet and uint8).
ROS2_TO_SIFT_TYPE: dict[str, ChannelDataType] = {
    "bool": ChannelDataType.BOOL,
    "int8": ChannelDataType.INT_32,
    "int16": ChannelDataType.INT_32,
    "int32": ChannelDataType.INT_32,
    "int64": ChannelDataType.INT_64,
    "uint8": ChannelDataType.UINT_32,
    "uint16": ChannelDataType.UINT_32,
    "uint32": ChannelDataType.UINT_32,
    "uint64": ChannelDataType.UINT_64,
    "byte": ChannelDataType.UINT_32,
    "char": ChannelDataType.UINT_32,
    "float32": ChannelDataType.FLOAT,
    "float64": ChannelDataType.DOUBLE,
    "string": ChannelDataType.STRING,
}

# builtin_interfaces Time and Duration import as one INT_64 nanosecond channel.
TIME_MESSAGE_TYPES = frozenset(("builtin_interfaces/Time", "builtin_interfaces/Duration"))
ROS2_TIME_TYPE = "__time__"

# Suffix of the JSON expansion of a variable-cardinality field.
JSON_CHANNEL_SUFFIX = ".json"

# Guards malformed schemas with self-referential fixed nesting.
MAX_FIELD_DEPTH = 32


class UnsupportedTopicError(Exception):
    """The topic's schema cannot be decoded by the importer."""


class LeafField(NamedTuple):
    """A leaf field of a topic's message type. Scalar leaves carry one value
    per message; complex leaves are variable-cardinality and expand per the
    complex types import mode.
    """

    field_path: str
    kind: str  # "scalar" or "complex"
    # The ROS 2 base type for scalar leaves (ROS2_TIME_TYPE for
    # builtin_interfaces Time/Duration). None for complex leaves.
    ros_type: str | None

    def sift_type(self) -> ChannelDataType:
        if self.ros_type == ROS2_TIME_TYPE:
            return ChannelDataType.INT_64
        return ROS2_TO_SIFT_TYPE[self.ros_type]  # type: ignore[index]


class TopicInfo(NamedTuple):
    """A supported topic and its importable leaves."""

    topic: str
    leaves: list[LeafField]


def parse_schema_defs(schema: mcap_records.Schema):
    """Parse a ros2msg concatenated schema into (root msgdef, msgdefs by
    name), using the same vendored parser the server importer uses."""
    msgdefs: dict = {
        "builtin_interfaces/Time": ros2_dynamic.TimeDefinition,
        "builtin_interfaces/Duration": ros2_dynamic.TimeDefinition,
    }

    def handle(cur_schema_name: str, short_name: str, msgdef) -> None:
        msgdefs[cur_schema_name] = msgdef
        msgdefs[short_name] = msgdef

    try:
        text = schema.data.decode("utf-8")
        ros2_dynamic._for_each_msgdef(schema.name, text, handle)
    except Exception as e:
        raise UnsupportedTopicError(f"its schema failed to parse ({e})") from e

    root = msgdefs.get(schema.name) or msgdefs.get(
        "/".join((schema.name.split("/")[0], schema.name.split("/")[-1]))
    )
    if root is None:
        raise UnsupportedTopicError("its schema does not define the root message")
    return root, msgdefs


def _is_variable_array(ftype) -> bool:
    """Unbounded and bounded ([<=N]) arrays decode with a dynamic length."""
    return ftype.is_array and (ftype.array_size is None or ftype.is_upper_bound)


def _check_primitive_supported(type_name: str, label: str) -> None:
    if type_name == "wstring":
        raise UnsupportedTopicError(
            f"field '{label}' uses wstring, which the decoder does not implement"
        )
    if type_name not in ROS2_TO_SIFT_TYPE:
        raise UnsupportedTopicError(f"field '{label}' has unsupported type '{type_name}'")


def _resolve_or_raise(msgdefs: dict, ftype, label: str):
    nested = msgdefs.get(f"{ftype.pkg_name}/{ftype.type}")
    if nested is None:
        raise UnsupportedTopicError(
            f"field '{label}' has unknown type '{ftype.pkg_name}/{ftype.type}'"
        )
    return nested


def _check_ftype_decodable(
    msgdefs: dict, ftype, label: str, visited: frozenset = frozenset(), depth: int = 0
) -> None:
    """Raise UnsupportedTopicError if the field type cannot be decoded.

    Variable-cardinality fields do not expand into leaves, but the importer
    still decodes every element, so wstring or unknown types anywhere in the
    subtree make the whole topic unsupported.
    """
    if ftype.is_primitive_type():
        _check_primitive_supported(ftype.type, label)
        return
    if f"{ftype.pkg_name}/{ftype.type}" in TIME_MESSAGE_TYPES:
        return
    _check_subtree_decodable(msgdefs, _resolve_or_raise(msgdefs, ftype, label), visited, depth)


def _check_subtree_decodable(msgdefs: dict, msgdef, visited: frozenset, depth: int) -> None:
    if depth > MAX_FIELD_DEPTH:
        raise UnsupportedTopicError(f"its schema nests deeper than {MAX_FIELD_DEPTH} levels")
    name = f"{msgdef.base_type.pkg_name}/{msgdef.msg_name}"
    if name in visited:
        return
    visited = visited | {name}
    for field in msgdef.fields:
        _check_ftype_decodable(msgdefs, field.type, field.name, visited, depth + 1)


def expand_message_fields(root_msgdef, msgdefs: dict) -> list[LeafField]:
    """Flatten the root message into importable leaves.

    Scalar leaves append their dot-delimited field path. Fixed-size arrays
    expand with bracketed indexes. Variable-cardinality fields become one
    complex leaf. Named constants are not message fields and never appear.
    """
    leaves: list[LeafField] = []

    def walk(prefix: str, msgdef, depth: int) -> None:
        if depth > MAX_FIELD_DEPTH:
            raise UnsupportedTopicError(f"its schema nests deeper than {MAX_FIELD_DEPTH} levels")
        for field in msgdef.fields:
            ftype = field.type
            path = f"{prefix}{field.name}"
            if _is_variable_array(ftype):
                # The importer decodes every element even though the leaf is
                # imported whole, so verify the element type decodes.
                _check_ftype_decodable(msgdefs, ftype, path)
                leaves.append(LeafField(path, "complex", None))
                continue

            indexes = [f"[{i}]" for i in range(ftype.array_size)] if ftype.is_array else [""]
            if ftype.is_primitive_type():
                _check_primitive_supported(ftype.type, path)
                leaves.extend(
                    LeafField(f"{path}{index}", "scalar", ftype.type) for index in indexes
                )
            elif f"{ftype.pkg_name}/{ftype.type}" in TIME_MESSAGE_TYPES:
                leaves.extend(
                    LeafField(f"{path}{index}", "scalar", ROS2_TIME_TYPE) for index in indexes
                )
            else:
                nested = _resolve_or_raise(msgdefs, ftype, path)
                for index in indexes:
                    walk(f"{path}{index}.", nested, depth + 1)

    walk("", root_msgdef, 0)
    return leaves


def _read_schemas_and_channels(
    path: Path,
) -> tuple[dict[int, mcap_records.Schema], list[mcap_records.Channel], list[str]]:
    """Read schema and channel records without decoding message payloads.

    Mirrors the server importer's scan: a top-level pass validates chunk
    compression and collects the records of unchunked files, the summary
    section supplies the records of chunked files, and a file without a
    readable summary (e.g. truncated) is scanned through its decompressed
    chunks instead, keeping what parsed with a warning.
    """
    parse_warnings: list[str] = []
    schemas: dict[int, mcap_records.Schema] = {}
    channels: list[mcap_records.Channel] = []
    seen_channel_ids: set[int] = set()

    def add_channel(channel: mcap_records.Channel) -> None:
        if channel.id not in seen_channel_ids:
            seen_channel_ids.add(channel.id)
            channels.append(channel)

    def scan(stream: StreamReader) -> None:
        records = iter(stream.records)
        while True:
            try:
                record = next(records)
            except StopIteration:
                return
            except Exception as e:
                # Truncation errors often stringify empty, so always say something.
                detail = str(e) or type(e).__name__
                message = (
                    "stopped reading at an unparseable record; the detected "
                    f"channels may be incomplete: {detail}"
                )
                # Both passes hit the same broken spot.
                if message not in parse_warnings:
                    parse_warnings.append(message)
                return
            if isinstance(record, mcap_records.Chunk):
                # The importer rejects unsupported compression regardless of
                # parse_error_policy; the mcap reader would silently treat it
                # as uncompressed.
                if record.compression not in SUPPORTED_COMPRESSIONS:
                    raise ValueError(
                        f"unsupported chunk compression '{record.compression}'; "
                        "supported compressions are none, zstd, and lz4"
                    )
            elif isinstance(record, mcap_records.Schema):
                schemas[record.id] = record
            elif isinstance(record, mcap_records.Channel):
                add_channel(record)

    with open(path, "rb") as file:
        if file.read(len(MCAP_MAGIC)) != MCAP_MAGIC:
            raise ValueError(f"'{path.name}' is not an MCAP file (bad magic bytes)")
        file.seek(0)

        # Top-level pass: chunks stay unopened, so this validates compression
        # cheaply and picks up the records of unchunked files.
        scan(StreamReader(file, emit_chunks=True))

        # Chunked files carry their records inside chunks; the summary section
        # repeats them. Without a readable summary (e.g. a truncated file),
        # scan through the decompressed chunks instead.
        file.seek(0)
        try:
            summary = make_reader(file).get_summary()
        except Exception:
            summary = None
        if summary is not None:
            schemas.update(summary.schemas)
            for channel in sorted(summary.channels.values(), key=lambda c: c.id):
                add_channel(channel)
        else:
            file.seek(0)
            scan(StreamReader(file, emit_chunks=False))

    return schemas, channels, parse_warnings


def detect_mcap_topics(
    schemas: dict[int, mcap_records.Schema],
    channels: list[mcap_records.Channel],
    parse_warnings: list[str],
) -> list[TopicInfo]:
    """Derive the supported topics and their importable leaves.

    Same-topic channels merge only when their schemas and message encodings
    match. Distinct topics colliding case-insensitively keep the first.
    Unsupported topics are skipped with a warning; the import itself gates
    them on ``parse_error_policy``.
    """
    channels_by_topic: defaultdict[str, list[mcap_records.Channel]] = defaultdict(list)
    for channel in channels:
        channels_by_topic[channel.topic].append(channel)

    # Sift channel names compare case-insensitively, so distinct topics
    # colliding only by case conflict; the first wins.
    kept_by_lower: dict[str, str] = {}
    for topic in channels_by_topic:
        first = kept_by_lower.setdefault(topic.lower(), topic)
        if first != topic:
            parse_warnings.append(
                f"topic '{topic}' collides with topic '{first}' by case only; kept the first"
            )

    topics: list[TopicInfo] = []
    unsupported: dict[str, str] = {}
    for topic, topic_channels in channels_by_topic.items():
        if kept_by_lower[topic.lower()] != topic:
            continue
        # Same-topic channels merge only when they agree.
        encodings = {c.message_encoding for c in topic_channels}
        topic_schemas = [schemas.get(c.schema_id) for c in topic_channels]
        signatures = {None if s is None else (s.name, s.encoding, s.data) for s in topic_schemas}
        if len(encodings) > 1 or len(signatures) > 1:
            unsupported[topic] = (
                "it has multiple channels with mismatched schemas or message encodings"
            )
            continue
        channel = topic_channels[0]
        schema = topic_schemas[0]
        if schema is None:
            unsupported[topic] = "it has no schema"
            continue
        if channel.message_encoding != "cdr":
            unsupported[topic] = (
                f"its message encoding is '{channel.message_encoding}' (only cdr is supported)"
            )
            continue
        if schema.encoding != "ros2msg":
            unsupported[topic] = (
                f"its schema encoding is '{schema.encoding}' (only ros2msg is supported)"
            )
            continue
        try:
            root, msgdefs = parse_schema_defs(schema)
            leaves = expand_message_fields(root, msgdefs)
        except UnsupportedTopicError as e:
            unsupported[topic] = str(e)
            continue
        topics.append(TopicInfo(topic=topic, leaves=leaves))

    if unsupported:
        details = "; ".join(f"'{t}': {reason}" for t, reason in sorted(unsupported.items()))
        parse_warnings.append(f"skipped unsupported topics: {details}")
    return topics


def detect_mcap_fields(topics: list[TopicInfo]) -> list[McapDataColumn]:
    """Return importable channels as ``McapDataColumn``s with default names
    and data types.

    Scalar leaves become one channel named ``<topic>.<field_path>``. Complex
    leaves expand like the default complex types import mode (``BOTH``): Arrow
    IPC bytes under the base name and a JSON string under ``<base>.json``.
    """
    channels: list[McapDataColumn] = []
    # Sift channel names are unique per asset and compare case-insensitively.
    taken_names: dict[str, str] = {}
    for topic in topics:
        for leaf in topic.leaves:
            base_name = f"{topic.topic}.{leaf.field_path}"
            if leaf.kind == "scalar":
                expansions = [(base_name, leaf.sift_type())]
            else:
                expansions = [
                    (base_name, ChannelDataType.BYTES),
                    (base_name + JSON_CHANNEL_SUFFIX, ChannelDataType.STRING),
                ]
            for name, data_type in expansions:
                existing = taken_names.get(name.lower())
                if existing is not None:
                    raise ValueError(
                        f"the generated channel name '{name}' conflicts with channel '{existing}'"
                    )
                taken_names[name.lower()] = name
                channels.append(
                    McapDataColumn(
                        topic=topic.topic,
                        field_path=leaf.field_path,
                        name=name,
                        data_type=data_type,
                    )
                )
    return channels


def detect_mcap_config(file_path: str | Path, asset_name: str = "") -> McapImportConfig:
    """Detect an MCAP import config by enumerating the file's channels.

    Channels come from the file's schema and channel records; message payloads
    are not read, so a topic is listed even when it logged no messages. Topics
    the importer does not support (non-cdr message encodings, non-ros2msg
    schemas, undecodable schemas) are skipped with a warning; importing such a
    file fails under the default parse error policy, so set
    ``McapParseErrorPolicy.IGNORE_ERROR`` to import the rest.

    Args:
        file_path: Path to the ``.mcap`` file.
        asset_name: The asset name to set on the config.

    Returns:
        A config whose ``data`` lists detected channels with default Sift names
        and data types. Remove entries to skip channels, or edit entries before
        importing. Leaving ``data`` empty imports all channels with the same
        defaults.

    Raises:
        ValueError: If the file is not MCAP, uses an unsupported chunk
            compression, or two detected channels generate the same Sift
            channel name. The importer rejects all three regardless of
            ``parse_error_policy``.
    """
    path = Path(file_path)
    schemas, channels, parse_warnings = _read_schemas_and_channels(path)
    topics = detect_mcap_topics(schemas, channels, parse_warnings)
    data = detect_mcap_fields(topics)
    for message in parse_warnings:
        warnings.warn(f"'{path.name}': {message}", stacklevel=2)
    return McapImportConfig(asset_name=asset_name, data=data)
