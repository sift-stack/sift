import time

from sift_stream_bindings import (
    TimeValuePy,
)


class TestTimeValue:
    """Test TimeValuePy functionality."""

    def test_create_time_value_from_timestamp(self):
        """Test creating TimeValuePy from timestamp."""
        secs = int(time.time())
        nsecs = 123456789
        time_value = TimeValuePy.from_timestamp(secs, nsecs)
        assert time_value is not None

    def test_create_time_value_from_timestamp_millis(self):
        """Test creating TimeValuePy from timestamp milliseconds."""
        millis = int(time.time() * 1000)
        time_value = TimeValuePy.from_timestamp_millis(millis)
        assert time_value is not None

    def test_create_time_value_from_timestamp_micros(self):
        """Test creating TimeValuePy from timestamp microseconds."""
        micros = int(time.time() * 1_000_000)
        time_value = TimeValuePy.from_timestamp_micros(micros)
        assert time_value is not None

    def test_create_time_value_from_timestamp_nanos(self):
        """Test creating TimeValuePy from timestamp nanoseconds."""
        nanos = int(time.time() * 1_000_000_000)
        time_value = TimeValuePy.from_timestamp_nanos(nanos)
        assert time_value is not None

    def test_create_time_value_from_rfc3339(self):
        """Test creating TimeValuePy from RFC3339 string."""
        rfc3339 = "2025-06-25T12:00:00Z"
        time_value = TimeValuePy.from_rfc3339(rfc3339)
        assert time_value is not None
