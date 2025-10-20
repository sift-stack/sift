from datetime import datetime

from google.protobuf.timestamp_pb2 import Timestamp


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
