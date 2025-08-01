from datetime import datetime

from google.protobuf.timestamp_pb2 import Timestamp
from sift_stream_bindings import TimeValuePy


def to_pb_timestamp(timestamp: datetime) -> Timestamp:
    timestamp_pb = Timestamp()
    timestamp_pb.FromDatetime(timestamp)
    return timestamp_pb


def to_rust_py_timestamp(time: datetime) -> TimeValuePy:
    ts = time.timestamp()
    secs = int(ts)
    nsecs = int((ts - secs) * 1_000_000_000)
    return TimeValuePy.from_timestamp(secs, nsecs)
