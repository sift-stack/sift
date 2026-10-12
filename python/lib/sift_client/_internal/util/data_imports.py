"""Helpers for applying ``relative_start_time`` to detected import configs."""

from __future__ import annotations

import warnings
from typing import TYPE_CHECKING

from sift_client._internal.time import to_timestamp_pb
from sift_client._internal.util.util import caller_stacklevel
from sift_client.errors import SiftIgnoredInputWarning
from sift_client.sift_types.data_import import (
    Hdf5ImportConfig,
    McapImportConfig,
    TdmsImportConfig,
    TimeFormat,
    UlogImportConfig,
)

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.sift_types.data_import import ImportConfig


def apply_relative_start_time(
    config: ImportConfig, relative_start_time: datetime, time_format: TimeFormat | None
) -> None:
    """Set the relative start time on a detected config, dispatching by config type.

    ULog and MCAP always take it, regardless of ``time_format``: ULog as the
    log start time, MCAP as the start that non-Unix-epoch ``log_time`` values
    are measured from. Other formats take it only when ``time_format``, the
    config's current format, is relative; otherwise it is ignored with a
    warning.
    """
    if isinstance(config, (UlogImportConfig, McapImportConfig)):
        config.relative_start_time = relative_start_time
        return
    if time_format is None or not time_format.is_relative:
        warnings.warn(
            "'relative_start_time' is ignored because the time format "
            f"({time_format.name if time_format else 'unset'}) is not relative.",
            SiftIgnoredInputWarning,
            stacklevel=caller_stacklevel(),
        )
        return
    if isinstance(config, (TdmsImportConfig, Hdf5ImportConfig)):
        config.relative_start_time = relative_start_time
    else:
        config.time_column.relative_start_time = relative_start_time


def check_relative_start_time(config: ImportConfig, time_format: TimeFormat | None) -> None:
    """Raise if ``time_format``, the config's current format, is relative with no start time.

    Assigning a time format bypasses the config validators, so a
    ``time_format`` override is checked here before an auto-detected config
    is imported. Only formats whose configs require a start time are checked.
    """
    if isinstance(config, (TdmsImportConfig, UlogImportConfig, McapImportConfig)):
        return
    if isinstance(config, Hdf5ImportConfig):
        relative_start_time = config.relative_start_time
    else:
        relative_start_time = config.time_column.relative_start_time
    if time_format is not None and time_format.is_relative and relative_start_time is None:
        raise ValueError(
            f"'relative_start_time' is required when using a relative time format ({time_format.name})."
        )


_RELATIVE_FORMAT_VALUES = {f.value for f in TimeFormat if f.is_relative}


def set_detected_start_time(time_column, relative_start_time: datetime | None) -> None:
    """Set the start time on a detected relative time column proto.

    Without it, a relative format detected by the server fails config
    validation before ``relative_start_time`` can be applied.
    """
    if relative_start_time is not None and time_column.format in _RELATIVE_FORMAT_VALUES:
        time_column.relative_start_time.CopyFrom(to_timestamp_pb(relative_start_time))
