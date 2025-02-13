"""
Module containing utilities to construct a data query which is ultimately
passed to `sift_py.data.service.DataService.execute` to download telemetry.

This module also contains types that represent the result of a data query
which can be easily converted into a `pandas` data frame or series.
"""

from __future__ import annotations

from datetime import datetime
from typing import Any, Dict, List, Optional, Tuple, TypedDict, Union, cast

import pandas as pd
from google.protobuf.timestamp_pb2 import Timestamp as TimestampPb
from typing_extensions import NotRequired, TypeAlias

from sift_py._internal.channel import channel_fqn
from sift_py._internal.time import to_timestamp_nanos
from sift_py.data._channel import ChannelTimeSeries
from sift_py.error import _component_deprecation_warning
from sift_py.ingestion.channel import ChannelDataType


class DataQuery:
    """
    A query that is meant to be passed to `sift_py.data.service.DataService.execute` to
    retrieve telemetry.

    - `asset_name`: The name of the asset to query telemetry for.
    - `start_time`: The start time of the time range of the data to request.
    - `end_time`: The end time of the time range of the data to request.
    - `sample_ms`:
        The sampling rate to use when retrieving data. The lower the sampling rate, the
        greater the data-fidelity. A sampling rate of `0` retrieves full-fidelity data.
    - `channels`:
        List of either `ChannelQuery` or `CalculatedChannelQuery`, but not both. Represents the
        channels to retrieve data from.
    """

    DEFAULT_PAGE_SIZE = 100_000

    asset_name: str
    start_time: pd.Timestamp
    end_time: pd.Timestamp
    sample_ms: int
    page_size: int
    channels: List[Union[ChannelQuery, CalculatedChannelQuery]]

    def __init__(
        self,
        asset_name: str,
        start_time: Union[pd.Timestamp, TimestampPb, datetime, str, int],
        end_time: Union[pd.Timestamp, TimestampPb, datetime, str, int],
        channels: List[Union[ChannelQuery, CalculatedChannelQuery]],
        sample_ms: int = 0,
        # Currently not in use outside of testing purposes.
        _: int = DEFAULT_PAGE_SIZE,
    ):
        self.start_time = to_timestamp_nanos(start_time)
        self.end_time = to_timestamp_nanos(end_time)
        self.asset_name = asset_name
        self.sample_ms = sample_ms
        self.channels = channels
        self.page_size = self.__class__.DEFAULT_PAGE_SIZE


"""
Either the fully qualified channel name or a tuple of the fully qualified
channel name as well as the channel's type.
"""
ChannelLookupInfo: TypeAlias = Union[str, Tuple[str, ChannelDataType]]


class DataQueryResult:
    """
    The result of a data query which can contain multiple channels.
    """

    _result: Dict[str, List[ChannelTimeSeries]]

    def __init__(self, merged_channel_data: Dict[str, List[ChannelTimeSeries]]):
        self._result = merged_channel_data

    def channel(self, lookup: ChannelLookupInfo) -> Optional[DataQueryResultSet]:
        """
        Like `channels` but returns a single `DataQueryResultSet`.
        """

        result = self.channels(lookup)

        if len(result) > 0:
            return result[0]

        return None

    def channels(self, *lookup: ChannelLookupInfo) -> List[DataQueryResultSet]:
        """
        Returns a `sift_py.data.channel.ChannelTimeSeries` given the `lookup` argument.
        If a `lookup` is a fully qualified name (FQN) `str` and there are multiple channels
        with the same FQN, this will raise a `ValueError`. In these situations, `lookup` must
        be a tuple where the first item is the channel FQN and the second the
        `sift_py.ingestion.channel.ChannelDataType`.

        If `lookup` is a tuple, then the channel data-type will be appended to the key referencing
        the `sift_py.data.channel.ChannelTimeSeries`.
        """

        result: List[DataQueryResultSet] = []

        for info in lookup:
            if isinstance(info, str):
                time_series = self._result.get(info)

                if not time_series:
                    continue
                if len(time_series) > 1:
                    raise ValueError(
                        f"Ambiguous lookup provided: '{info}' is associated with {len(time_series)} channels."
                    )

                series = time_series[0]
                result.append(
                    DataQueryResultSet(
                        identifier=info,
                        timestamps=series.time_column,
                        values=series.value_column,
                    )
                )
            else:
                fqn, data_type = cast(Tuple[str, ChannelDataType], info)
                identifier = f"{fqn}.{data_type.as_human_str()}"

                time_series = self._result.get(fqn)

                if not time_series:
                    continue
                if len(time_series) == 1:
                    series = time_series[0]
                    result.append(
                        DataQueryResultSet(
                            identifier=identifier,
                            timestamps=series.time_column,
                            values=series.value_column,
                        )
                    )
                    continue

                for series in time_series:
                    if series.data_type == data_type:
                        result.append(
                            DataQueryResultSet(
                                identifier=identifier,
                                timestamps=series.time_column,
                                values=series.value_column,
                            )
                        )
                        break

        return result

    def all_channels(self) -> List[DataQueryResultSet]:
        """
        Returns all channel data.
        """

        result = []

        for fqn, time_series in self._result.items():
            if len(time_series) > 1:
                for series in time_series:
                    human_data_type = series.data_type.as_human_str()
                    fqn_extended = f"{fqn}.{human_data_type}"

                    result.append(
                        DataQueryResultSet(
                            identifier=fqn_extended,
                            timestamps=series.time_column,
                            values=series.value_column,
                        )
                    )
                continue

            for series in time_series:
                result.append(
                    DataQueryResultSet(
                        identifier=fqn,
                        timestamps=series.time_column,
                        values=series.value_column,
                    )
                )

        return result


class DataQueryResultSet:
    """
    Represents time series data for a single channel. Can easily be converted into a `pandas` data frame like so:

    ```python
    pd.DataFrame(data_query_result_set.all_columns())
    ```

    """

    identifier: str
    timestamps: List[pd.Timestamp]
    values: List[Any]

    def __init__(self, identifier: str, timestamps: List[pd.Timestamp], values: List[Any]):
        self.identifier = identifier
        self.timestamps = timestamps
        self.values = values

    def value_column(self, column_name: Optional[str] = None) -> Dict[str, List[Any]]:
        """
        Returns a single key-value pair dictionary meant to represent the value column of the data-set.
        `column_name` can be used to override the name of the column.
        """

        if column_name is None:
            return {self.identifier: self.values}
        else:
            return {column_name: self.values}

    def time_column(self, column_name: Optional[str] = None) -> Dict[str, List[Any]]:
        """
        Returns a single key-value pair dictionary meant to represent the time column of the data-set.
        `column_name` can be used to override the name of the column.
        """
        if column_name is None:
            return {"time": self.timestamps}
        else:
            return {column_name: self.timestamps}

    def columns(
        self,
        time_column_name: Optional[str] = None,
        value_column_name: Optional[str] = None,
    ) -> Dict[str, List[Any]]:
        """
        Returns both the time and value columns with options to override the column names.
        """

        cols = self.time_column(time_column_name)
        cols.update(self.value_column(value_column_name))
        return cols


class ChannelQuery:
    """
    Represents a single channel to include in the `sift_py.data.query.DataQuery`.
    """

    channel_name: str
    component: Optional[str]  # Deprecated
    run_name: Optional[str]

    def __init__(
        self,
        channel_name: str,
        component: Optional[str] = None,  # Deprecated
        run_name: Optional[str] = None,
    ):
        self.channel_name = channel_name
        if component is not None:
            _component_deprecation_warning()
            self.channel_name = channel_fqn(name=self.channel_name, component=component)
        self.run_name = run_name

    def fqn(self) -> str:
        return channel_fqn(self.channel_name)


class ExpressionChannelReference(TypedDict):
    reference: str
    channel_name: str
    component: NotRequired[str]  # Deprecated
    data_type: NotRequired[ChannelDataType]


class CalculatedChannelQuery:
    """
    Represents a single calculated channel to include in the `sift_py.data.query.DataQuery`.
    """

    channel_key: str
    expression: str
    expression_channel_references: List[ExpressionChannelReference]
    run_name: Optional[str]

    def __init__(
        self,
        channel_key: str,
        expression: str,
        expression_channel_references: List[ExpressionChannelReference],
        run_name: Optional[str] = None,
    ):
        self.channel_key = channel_key
        self.run_name = run_name
        self.expression = expression
        self.expression_channel_references = expression_channel_references
