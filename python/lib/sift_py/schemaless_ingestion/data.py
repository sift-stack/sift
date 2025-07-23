from datetime import datetime
from typing import List, TypedDict, Union


class SchemalessChannel(TypedDict):
    channel: str
    value: Union[float, str, bool]


class SchemalessData(TypedDict):
    timestamp: datetime
    values: List[SchemalessChannel]
