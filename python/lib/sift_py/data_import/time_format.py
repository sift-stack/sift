from enum import Enum
from typing import Optional

from typing_extensions import Self


class TimeFormatType(Enum):
    ABSOLUTE_RFC3339 = "TIME_FORMAT_ABSOLUTE_RFC3339"
    ABSOLUTE_DATETIME = "TIME_FORMAT_ABSOLUTE_DATETIME"
    ABSOLUTE_UNIX_SECONDS = "TIME_FORMAT_ABSOLUTE_UNIX_SECONDS"
    ABSOLUTE_UNIX_MILLISECONDS = "TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS"
    ABSOLUTE_UNIX_MICROSECONDS = "TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS"
    ABSOLUTE_UNIX_NANOSECONDS = "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS"
    RELATIVE_NANOSECONDS = "TIME_FORMAT_RELATIVE_NANOSECONDS"
    RELATIVE_MICROSECONDS = "TIME_FORMAT_RELATIVE_MICROSECONDS"
    RELATIVE_MILLISECONDS = "TIME_FORMAT_RELATIVE_MILLISECONDS"
    RELATIVE_SECONDS = "TIME_FORMAT_RELATIVE_SECONDS"
    RELATIVE_MINUTES = "TIME_FORMAT_RELATIVE_MINUTES"
    RELATIVE_HOURS = "TIME_FORMAT_RELATIVE_HOURS"

    @classmethod
    def from_str(cls, val: str) -> Optional[Self]:
        try:
            return cls(val)
        except ValueError:
            return None

    def as_human_str(self) -> str:
        return self.value

    def is_relative(self) -> bool:
        return self in [
            self.RELATIVE_NANOSECONDS,
            self.RELATIVE_MICROSECONDS,
            self.RELATIVE_MILLISECONDS,
            self.RELATIVE_SECONDS,
            self.RELATIVE_MINUTES,
            self.RELATIVE_HOURS,
        ]
