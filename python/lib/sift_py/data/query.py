from __future__ import annotations

from datetime import datetime
from typing import Any, Dict, List, Optional, Tuple, TypedDict, Union, cast

from typing_extensions import NotRequired, TypeAlias

from sift_py._internal.channel import channel_fqn
from sift_py.data.channel import ChannelTimeSeries
from sift_py.ingestion.channel import ChannelDataType


class DataQuery:
    DEFAULT_PAGE_SIZE = 100_000

    asset_name: str
    start_time: datetime
    end_time: datetime
    sample_ms: int
    page_size: int
    channels: List[Union[ChannelQuery, CalculatedChannelQuery]]

    def __init__(
        self,
        asset_name: str,
        start_time: datetime,
        end_time: datetime,
        sample_ms: int,
        channels: List[Union[ChannelQuery, CalculatedChannelQuery]],
        page_size: int = DEFAULT_PAGE_SIZE,
    ):
        self.asset_name = asset_name
        self.start_time = start_time
        self.end_time = end_time
        self.sample_ms = sample_ms
        self.channels = channels
        self.page_size = page_size


"""
Either the fully qualified channel name or a tuple of the fully qualified
channel name as well as the channel's type.
"""
ChannelLookupInfo: TypeAlias = Union[str, Tuple[str, ChannelDataType]]


class DataQueryResult:
    _result: Dict[str, List[ChannelTimeSeries]]

    def __init__(self, merged_channel_data: Dict[str, List[ChannelTimeSeries]]):
        self._result = merged_channel_data

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
                    {
                        "identifier": info,
                        "time_column": series.time_column,
                        "value_column": series.value_column,
                    }
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
                        {
                            "identifier": identifier,
                            "time_column": series.time_column,
                            "value_column": series.value_column,
                        }
                    )
                    continue

                for series in time_series:
                    if series.data_type == data_type:
                        result.append(
                            {
                                "identifier": identifier,
                                "time_column": series.time_column,
                                "value_column": series.value_column,
                            }
                        )
                        break

        return result


class DataQueryResultSet(TypedDict):
    identifier: str
    time_column: List[datetime]
    value_column: List[Any]


class ChannelQuery:
    channel_name: str
    component: Optional[str]
    run_name: Optional[str]

    def __init__(
        self,
        channel_name: str,
        component: Optional[str] = None,
        run_name: Optional[str] = None,
    ):
        self.channel_name = channel_name
        self.component = component
        self.run_name = run_name

    def fqn(self) -> str:
        return channel_fqn(self.channel_name, self.component)


class CalculatedChannelQuery:
    ChannelName: TypeAlias = str
    ChannelIdentifier = TypedDict(
        "ChannelIdentifier",
        {
            "channel_name": str,
            "component": NotRequired[str],
        },
    )

    channel_key: str
    run_name: str
    expression: str
    expression_channel_references: Dict[ChannelName, ChannelIdentifier]

    def __init__(
        self,
        channel_key: str,
        run_name: str,
        expression: str,
        expression_channel_references: Dict[ChannelName, ChannelIdentifier],
    ):
        self.channel_key = channel_key
        self.run_name = run_name
        self.expression = expression
        self.expression_channel_references = expression_channel_references
