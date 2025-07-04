from datetime import datetime

from google.protobuf.timestamp_pb2 import Timestamp


def to_pb_timestamp(timestamp: datetime) -> Timestamp:
    timestamp_pb = Timestamp()
    timestamp_pb.FromDatetime(timestamp)
    return timestamp_pb
