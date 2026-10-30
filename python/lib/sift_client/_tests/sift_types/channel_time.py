"""Shared check that a time axis matches the index ``get_data`` builds."""

import pandas as pd
from google.protobuf.any_pb2 import Any
from google.protobuf.timestamp_pb2 import Timestamp
from sift.common.type.v1.channel_data_type_pb2 import CHANNEL_DATA_TYPE_DOUBLE
from sift.data.v2.data_pb2 import DoubleValue, DoubleValues, Metadata

from sift_client._internal.low_level_wrappers.data import DataLowLevelClient


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


def assert_axis_matches_channel(start, stop, *timestamps: Timestamp) -> None:
    """A ``date_range`` across these bounds uses the channel index scale."""
    samples = channel_index(*timestamps)
    axis = pd.date_range(start, stop, periods=2)
    assert str(samples.dtype).startswith("datetime64[ns")
    assert axis.dtype == samples.dtype
    assert [int(value) for value in axis.asi8] == [int(value) for value in samples.asi8]
