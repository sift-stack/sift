"""Plain pytest tests are automatically captured by the plugin as steps.

No imports from ``sift_client`` or fixture usage required. Each test
becomes a step in the report tree: passing tests resolve to ``PASSED``,
failing tests to ``FAILED``. This allows integrating existing tests
with Sift Test Results without modification.
"""

import pytest


def test_passes():
    """Functions become steps in the report tree. The function docstring is used as the step description."""
    assert 1 + 1 == 2


@pytest.mark.parametrize("value", ["v1", "v2"])
def test_parametrize_without_step(value):
    """Parametrized tests are nested under a common step with sub steps for each permutation."""
    assert value.startswith("v")


class TestPytestClass:
    """Test classes are turned into parent steps for their methods. Class docstrings are used as step the description."""

    def test_method(self):
        assert True


def test_uses_a_pytest_fixture(tmp_path):
    """Normal pytest fixtures keep working the plugin doesn't intercept them."""
    (tmp_path / "marker").write_text("ok")
    assert (tmp_path / "marker").read_text() == "ok"
