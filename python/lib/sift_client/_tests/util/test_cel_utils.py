import re
from datetime import datetime, timedelta, timezone

from sift_client.util.cel_utils import (
    and_,
    contains,
    equals,
    equals_all,
    equals_any,
    equals_double,
    equals_null,
    greater_than,
    in_,
    less_than,
    match,
    not_,
    or_,
    parens,
)


class TestCelUtils:
    def test_in_empty_list(self):
        """Test in_ function with an empty list."""
        assert in_("field", []) == ""

    def test_in_single_value(self):
        """Test in_ function with a single value."""
        assert in_("field", ["value"]) == "field in ['value']"

    def test_in_multiple_values(self):
        """Test in_ function with multiple values."""
        assert in_("field", ["value1", "value2"]) == "field in ['value1','value2']"

    def test_parens(self):
        """Test parens function."""
        assert parens("expression") == "(expression)"

    def test_equals_none(self):
        """Test equals function with None value."""
        assert equals("field", None) == "field == null"

    def test_equals_string(self):
        """Test equals function with string value."""
        assert equals("field", "value") == "field == 'value'"

    def test_equals_number(self):
        """Test equals function with numeric value."""
        assert equals("field", 42) == "field == 42"
        assert equals("field", 3.14) == "field == 3.14"

    def test_equals_boolean(self):
        """Test equals function with boolean value."""
        assert equals("field", True) == "field == true"
        assert equals("field", False) == "field == false"

    def test_equals_all_empty(self):
        """Test equals_all function with empty dict."""
        assert equals_all({}) == ""

    def test_equals_all_single(self):
        """Test equals_all function with single key-value pair."""
        assert equals_all({"field": "value"}) == "field == 'value'"

    def test_equals_all_multiple(self):
        """Test equals_all function with multiple key-value pairs."""
        result = equals_all({"field1": "value1", "field2": 42})
        # Check both possible orderings
        assert (
            result == "field1 == 'value1' && field2 == 42"
            or result == "field2 == 42 && field1 == 'value1'"
        )

    def test_equals_any_empty(self):
        """Test equals_any function with empty dict."""
        assert equals_any({}) == ""

    def test_equals_any_single(self):
        """Test equals_any function with single key-value pair."""
        assert equals_any({"field": "value"}) == "field == 'value'"

    def test_equals_any_multiple(self):
        """Test equals_any function with multiple key-value pairs."""
        result = equals_any({"field1": "value1", "field2": 42})
        # Check both possible orderings
        assert (
            result == "field1 == 'value1' || field2 == 42"
            or result == "field2 == 42 || field1 == 'value1'"
        )

    def test_equals_double_none(self):
        """Test equals_double function with None value."""
        assert equals_double("field", None) == "field == null"

    def test_equals_double_number(self):
        """Test equals_double function with numeric value."""
        assert equals_double("field", 42) == "field == double(42)"
        assert equals_double("field", 3.14) == "field == double(3.14)"

    def test_equals_null(self):
        """Test equals_null function."""
        assert equals_null("field") == "field == null"

    def test_and_empty(self):
        """Test and_ function with no clauses."""
        assert and_() == ""

    def test_and_single(self):
        """Test and_ function with a single clause."""
        assert and_("clause") == "clause"

    def test_and_multiple(self):
        """Test and_ function with multiple clauses."""
        assert and_("clause1", "clause2", "clause3") == "clause1 && clause2 && clause3"

    def test_or_empty(self):
        """Test or_ function with no clauses."""
        assert or_() == ""

    def test_or_single(self):
        """Test or_ function with a single clause."""
        assert or_("clause") == "clause"

    def test_or_multiple(self):
        """Test or_ function with multiple clauses."""
        assert or_("clause1", "clause2", "clause3") == "clause1 || clause2 || clause3"

    def test_not(self):
        """Test not_ function."""
        assert not_("clause") == "!(clause)"

    def test_contains(self):
        """Test contains function."""
        assert contains("field", "substring") == "field.contains('substring')"

    def test_match_string(self):
        """Test match function with string pattern."""
        assert match("field", "pattern") == "field.matches('pattern')"

    def test_match_pattern(self):
        """Test match function with regex pattern."""
        pattern = re.compile(r"test\d+")
        assert match("field", pattern) == r"field.matches('test\\d+')"

    def test_match_with_backslashes(self):
        """Test match function with pattern containing backslashes."""
        assert match("field", r"test\\d+") == r"field.matches('test\\\\d+')"

    def test_greater_than_number(self):
        """Test greater_than function with numeric value."""
        assert greater_than("field", 42) == "field > 42"
        assert greater_than("field", 3.14) == "field > 3.14"

    def test_greater_than_datetime(self):
        """Test greater_than function with datetime value."""
        dt = datetime(2023, 1, 1, 12, 0, 0, tzinfo=timezone.utc)
        assert greater_than("field", dt) == f"field > timestamp('{dt.isoformat()}')"

    def test_less_than_number(self):
        """Test less_than function with numeric value."""
        assert less_than("field", 42) == "field < 42"
        assert less_than("field", 3.14) == "field < 3.14"

    def test_less_than_datetime(self):
        """Test less_than function with datetime value."""
        dt = datetime(2023, 1, 1, 12, 0, 0, tzinfo=timezone.utc)
        assert less_than("field", dt) == f"field < timestamp('{dt.isoformat()}')"

    def test_greater_than_duration(self):
        """Test greater_than function with timedelta value."""
        duration = timedelta(hours=2, minutes=30, seconds=15)
        expected_seconds = duration.total_seconds()
        assert greater_than("field", duration) == f"field > duration('{expected_seconds}s')"

    def test_less_than_duration(self):
        """Test less_than function with timedelta value."""
        duration = timedelta(hours=1, minutes=15, seconds=30)
        expected_seconds = duration.total_seconds()
        assert less_than("field", duration) == f"field < duration('{expected_seconds}s')"
