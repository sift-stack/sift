"""Shared check that a time axis matches the index ``get_data`` builds."""

from __future__ import annotations

from typing import TYPE_CHECKING

import numpy as np
import pandas as pd
from google.protobuf.any_pb2 import Any
from sift.common.type.v1.channel_data_type_pb2 import CHANNEL_DATA_TYPE_DOUBLE
from sift.data.v2.data_pb2 import DoubleValue, DoubleValues, Metadata

from sift_client._internal.low_level_wrappers.data import DataLowLevelClient

if TYPE_CHECKING:
    from google.protobuf.timestamp_pb2 import Timestamp


def channel_index(*timestamps: Timestamp) -> pd.DatetimeIndex:
    """Index ``get_data`` builds for double-channel samples at these times."""
    values = DoubleValues(
        metadata=Metadata(
            data_type=CHANNEL_DATA_TYPE_DOUBLE,
            sampled_ms=0,
            channel=Metadata.Channel(name="voltage"),
        ),
        values=[DoubleValue(timestamp=ts, value=1.0) for ts in timestamps],
    )
    packed = Any()
    packed.Pack(values)
    # ``Pack`` writes ``type.googleapis.com/...``. The data parser reads the
    # ``sift.data`` prefix the API sends.
    packed.type_url = "sift.data.v2.DoubleValues"
    frame = DataLowLevelClient.try_deserialize_channel_data(packed)["voltage"]
    index = frame.index
    assert isinstance(index, pd.DatetimeIndex)
    return index


def _integer_view(index: pd.DatetimeIndex) -> list[int]:
    """Integers in the index's own unit, the scale interpolation uses."""
    return [int(value) for value in np.asarray(index, dtype=np.int64)]


def assert_axis_matches_channel(start, stop, *timestamps: Timestamp) -> None:
    """A ``date_range`` across these bounds uses the channel index scale."""
    samples = channel_index(*timestamps)
    axis = pd.date_range(start, stop, periods=2)
    assert str(samples.dtype).startswith("datetime64[ns")
    assert axis.dtype == samples.dtype
    assert _integer_view(axis) == _integer_view(samples)
