from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client.sift_types.channel import ChannelDataType

if TYPE_CHECKING:
    from collections.abc import Iterable

_TIME_COLUMN_NAMES: frozenset[str] = frozenset({"ts", "timestamp", "time"})
_TIME_COLUMN_TYPES: frozenset[ChannelDataType] = frozenset(
    {ChannelDataType.INT_64, ChannelDataType.UINT_64}
)


def infer_time_column(
    columns: Iterable[tuple[str, ChannelDataType, str]],
) -> str | None:
    """Pick a likely time column when the server couldn't identify one.

    Returns the path of an INT64 or UINT64 column whose name
    (case-insensitive) matches one of ``ts``, ``timestamp``, or ``time``.
    Returns None otherwise.
    """
    data_columns = sorted(columns, key=lambda c: c[0].lower())
    for name, data_type, path in data_columns:
        if data_type in _TIME_COLUMN_TYPES and name.lower() in _TIME_COLUMN_NAMES:
            return path
    return None
