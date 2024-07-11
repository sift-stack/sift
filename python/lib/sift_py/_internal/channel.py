from datetime import datetime, timezone
from typing import Optional

from google.protobuf.timestamp_pb2 import Timestamp


def channel_fqn(name: str, component: Optional[str]) -> str:
    return name if component is None or len(component) == 0 else f"{component}.{name}"


def to_datetime(ts: Timestamp, tz: timezone = timezone.utc) -> datetime:
    return ts.ToDatetime(tz)
