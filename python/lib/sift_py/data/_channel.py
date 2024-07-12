from typing import Any, List

import pandas as pd

from sift_py.ingestion.channel import ChannelDataType


class ChannelTimeSeries:
    data_type: ChannelDataType
    time_column: List[pd.Timestamp]
    value_column: List[Any]

    def __init__(
        self,
        data_type: ChannelDataType,
        time_column: List[pd.Timestamp],
        value_column: List[Any],
    ):
        if len(time_column) != len(value_column):
            raise Exception("Both arguments, `time_column` and `value_column` must equal lengths.")

        self.data_type = data_type
        self.time_column = time_column
        self.value_column = value_column

    def sort_time_series(self):
        points = [(t, v) for t, v in zip(self.time_column, self.value_column)]
        points.sort(key=lambda x: x[0])

        time_column = []
        value_column = []

        for ts, val in points:
            time_column.append(ts)
            value_column.append(val)

        self.time_column = time_column
        self.value_column = value_column
