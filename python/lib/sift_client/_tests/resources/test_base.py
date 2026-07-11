"""Unit tests for the shared CEL filter builders on ResourceBase."""

from unittest.mock import MagicMock

from sift_client.resources._base import ResourceBase
from sift_client.util import cel_utils as cel


def _base() -> ResourceBase:
    return ResourceBase(MagicMock())


class TestBuildTimeCelFilters:
    def test_modified_by_filters_on_modified_by_value(self):
        # Regression: the modified_by branch used to emit the created_by value.
        parts = _base()._build_time_cel_filters(modified_by="u2")

        assert parts == [cel.equals("modified_by_user_id", "u2")]

    def test_created_by_filters_on_created_by_value(self):
        parts = _base()._build_time_cel_filters(created_by="u1")

        assert parts == [cel.equals("created_by_user_id", "u1")]


class TestBuildNameCelFilters:
    def test_defaults_to_name_field(self):
        parts = _base()._build_name_cel_filters(name="licenses")

        assert parts == [cel.equals("name", "licenses")]

    def test_field_overrides_the_cel_field(self):
        parts = _base()._build_name_cel_filters(name="licenses", field="display_name")

        assert parts == [cel.equals("display_name", "licenses")]
