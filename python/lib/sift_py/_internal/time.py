from datetime import datetime, timezone
from typing import Union, cast

import pandas as pd
from google.protobuf.timestamp_pb2 import Timestamp as TimestampPb


def to_timestamp_nanos(arg: Union[TimestampPb, pd.Timestamp, datetime, str, int]) -> pd.Timestamp:
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

        return cast(pd.Timestamp, ts + pd.Timedelta(nanos, unit="ns"))

    elif isinstance(arg, int):
        dt = datetime.fromtimestamp(arg, tz=timezone.utc)
        return cast(pd.Timestamp, pd.Timestamp(dt))

    else:
        return cast(pd.Timestamp, pd.Timestamp(arg))


def to_timestamp_pb(arg: Union[datetime, str, int]) -> TimestampPb:
    """
    Mainly used for testing at the moment. If using this for non-testing purposes
    should probably make this more robust and support nano-second precision.
    """

    ts = TimestampPb()

    if isinstance(arg, datetime):
        ts.FromDatetime(arg)
        return ts
    elif isinstance(arg, int):
        ts.FromDatetime(datetime.fromtimestamp(arg))
        return ts
    else:
        ts.FromDatetime(datetime.fromisoformat(arg))
        return ts
