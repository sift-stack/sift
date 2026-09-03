from __future__ import annotations

from datetime import datetime, timezone
from typing import cast

import pandas as pd
from google.protobuf.timestamp_pb2 import Timestamp as TimestampPb


def to_timestamp_nanos(arg: TimestampPb | pd.Timestamp | datetime | str | int) -> pd.Timestamp:
    """
    Converts a variety of time-types to a pandas timestamp which supports nano-second precision.
    """

    if isinstance(arg, pd.Timestamp):
        return arg
    elif isinstance(arg, TimestampPb):
        seconds = arg.seconds
        nanos = arg.nanos

        dt = datetime.fromtimestamp(seconds, tz=timezone.utc)
        ts = pd.Timestamp(dt)

        return cast("pd.Timestamp", ts + pd.Timedelta(nanos, unit="ns"))

    elif isinstance(arg, int):
        dt = datetime.fromtimestamp(arg, tz=timezone.utc)
        return cast("pd.Timestamp", pd.Timestamp(dt))

    else:
        return cast("pd.Timestamp", pd.Timestamp(arg))


def to_timestamp_pb(arg: pd.Timestamp | datetime | str | int | float) -> TimestampPb:
    """
    Converts a variety of time-types to a protobuf timestamp.

    A ``pd.Timestamp`` keeps its nanoseconds. Every other input has
    microsecond resolution at best, since ``TimestampPb.FromDatetime``
    reads ``microsecond`` and nothing finer. A naive input is read as
    UTC on either path.
    """

    ts = TimestampPb()

    if isinstance(arg, pd.Timestamp) and arg.nanosecond:
        # A nonzero sub-microsecond remainder is exactly what
        # ``FromDatetime`` drops, and it implies nanosecond units, so
        # ``value`` can't overflow here. Coarser units reach dates
        # outside the ns range where it would.
        ts.FromNanoseconds(arg.value)
        return ts
    elif isinstance(arg, datetime):
        ts.FromDatetime(arg)
        return ts
    elif isinstance(arg, (int, float)):
        ts.FromDatetime(datetime.fromtimestamp(arg, tz=timezone.utc))
        return ts
    else:
        ts.FromDatetime(datetime.fromisoformat(arg))
        return ts
