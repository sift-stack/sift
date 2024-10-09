from enum import Enum


class TimeFormatType(Enum):
    ABSOLUTE_RFC339 = "TIME_FORMAT_ABSOLUTE_RFC3339"
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
    def from_str(cls, val: str):
        if val == cls.ABSOLUTE_RFC339.value:
            return cls.ABSOLUTE_RFC339
        elif val == cls.ABSOLUTE_DATETIME.value:
            return cls.ABSOLUTE_DATETIME
        elif val == cls.ABSOLUTE_UNIX_SECONDS.value:
            return cls.ABSOLUTE_UNIX_SECONDS
        elif val == cls.ABSOLUTE_UNIX_MILLISECONDS.value:
            return cls.ABSOLUTE_UNIX_MILLISECONDS
        elif val == cls.ABSOLUTE_UNIX_MICROSECONDS.value:
            return cls.ABSOLUTE_UNIX_MICROSECONDS
        elif val == cls.ABSOLUTE_UNIX_NANOSECONDS.value:
            return cls.ABSOLUTE_UNIX_NANOSECONDS
        elif val == cls.RELATIVE_NANOSECONDS.value:
            return cls.RELATIVE_NANOSECONDS
        elif val == cls.RELATIVE_MICROSECONDS.value:
            return cls.RELATIVE_MICROSECONDS
        elif val == cls.RELATIVE_MILLISECONDS.value:
            return cls.RELATIVE_MILLISECONDS
        elif val == cls.RELATIVE_SECONDS.value:
            return cls.RELATIVE_SECONDS
        elif val == cls.RELATIVE_MINUTES.value:
            return cls.RELATIVE_MINUTES
        elif val == cls.RELATIVE_HOURS.value:
            return cls.RELATIVE_HOURS

        return None

    def is_relative(self):
        return self in [
            self.RELATIVE_NANOSECONDS,
            self.RELATIVE_MICROSECONDS,
            self.RELATIVE_MILLISECONDS,
            self.RELATIVE_SECONDS,
            self.RELATIVE_MINUTES,
            self.RELATIVE_HOURS,
        ]
