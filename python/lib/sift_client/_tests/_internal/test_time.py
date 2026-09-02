"""Tests for :mod:`sift_client._internal.time`.

The conversions here sit between the wire (nanosecond protobuf
timestamps) and pandas (nanosecond ``Timestamp``), with ``datetime`` —
microseconds at best — as the lossy type in the middle. These tests pin
where precision is kept and where it legitimately isn't.

The service filters query bounds on a millisecond plus sub-millisecond
pair, so a nanosecond bound is meaningful all the way down. Truncating
one on the way out is a client-side loss, not a match to the server's
resolution.
"""

from __future__ import annotations

from datetime import datetime, timezone

import pandas as pd
import pytest
from google.protobuf.timestamp_pb2 import Timestamp as TimestampPb

from sift_client._internal.time import to_timestamp_nanos, to_timestamp_pb

_NANOS = 123456789
_SECONDS = 1735689600  # 2025-01-01T00:00:00Z


class TestToTimestampPb:
    def test_pandas_timestamp_keeps_nanoseconds(self) -> None:
        """A ``pd.Timestamp`` survives the hop to protobuf intact.

        ``TimestampPb.FromDatetime`` reads ``microsecond`` and nothing
        finer, so a nanosecond-precise bound would otherwise reach the
        wire rounded down.
        """
        ts = pd.Timestamp(_SECONDS, unit="s", tz=timezone.utc) + pd.Timedelta(_NANOS, unit="ns")
        pb = to_timestamp_pb(ts)
        assert (pb.seconds, pb.nanos) == (_SECONDS, _NANOS)

    def test_coarser_units_take_the_datetime_path(self) -> None:
        """A non-nanosecond ``Timestamp`` converts without losing anything.

        Pandas 2.0 timestamps carry a storage unit. At ``us`` or
        coarser there is no sub-microsecond remainder, so
        ``FromDatetime`` is lossless.
        """
        ts = pd.Timestamp(_SECONDS, unit="s", tz=timezone.utc) + pd.Timedelta(_NANOS, unit="ns")
        assert to_timestamp_pb(ts.as_unit("us")).nanos == 123456000
        assert to_timestamp_pb(ts.as_unit("ms")).nanos == 123000000

    @pytest.mark.parametrize("stamp", ["2300-01-01", "1500-06-01"], ids=["future", "past"])
    def test_dates_outside_the_nanosecond_range(self, stamp: str) -> None:
        """Dates a ``Timestamp`` can hold but nanoseconds can't still convert.

        Only ``ns``-unit timestamps are bounded to roughly 1677-2262. A
        coarser unit reaches further, and reading ``value`` on one of
        those raises ``OverflowError``, so those take the
        ``FromDatetime`` path instead.
        """
        ts = pd.Timestamp(stamp, tz=timezone.utc).as_unit("s")
        assert to_timestamp_pb(ts).ToDatetime(tzinfo=timezone.utc) == ts.to_pydatetime()

    def test_datetime_carries_its_microseconds(self) -> None:
        """A plain ``datetime`` converts at the resolution it has."""
        dt = datetime(2025, 1, 1, microsecond=123456, tzinfo=timezone.utc)
        pb = to_timestamp_pb(dt)
        assert (pb.seconds, pb.nanos) == (_SECONDS, 123456000)

    def test_epoch_seconds_and_iso_string(self) -> None:
        """The numeric and string forms stay on the ``datetime`` path."""
        assert to_timestamp_pb(_SECONDS).seconds == _SECONDS
        assert to_timestamp_pb("2025-01-01T00:00:00+00:00").seconds == _SECONDS

    def test_round_trips_through_to_timestamp_nanos(self) -> None:
        """Proto to pandas and back is lossless at nanosecond resolution."""
        pb = TimestampPb(seconds=_SECONDS, nanos=_NANOS)
        again = to_timestamp_pb(to_timestamp_nanos(pb))
        assert (again.seconds, again.nanos) == (_SECONDS, _NANOS)
