from datetime import datetime

from google.protobuf.timestamp_pb2 import Timestamp
from sift_stream_bindings import TimeValuePy


def to_pb_timestamp(timestamp: datetime) -> Timestamp:
    """Convert a Python datetime to a Protocol Buffer Timestamp.

    Args:
        timestamp: The datetime to convert

    Returns:
        A Protocol Buffer Timestamp representation
    """
    timestamp_pb = Timestamp()
    timestamp_pb.FromDatetime(timestamp)
    return timestamp_pb


def to_rust_py_timestamp(time: datetime) -> TimeValuePy:
    """Convert a Python datetime to a Rust TimeValuePy.

    Args:
        time: The datetime to convert

    Returns:
        A TimeValuePy representation
    """
    ts = time.timestamp()
    secs = int(ts)
    nsecs = int((ts - secs) * 1_000_000_000)
    return TimeValuePy.from_timestamp(secs, nsecs)
