from pathlib import Path

import pytest

_HERE = Path(__file__).parent


def pytest_configure(config: pytest.Config) -> None:
    """Configure the pytest configuration to disable the Sift test results log file."""
    config.option.sift_log_file = False


def pytest_collection_modifyitems(config: pytest.Config, items: "list[pytest.Item]") -> None:
    """Bulk-apply ``@pytest.mark.sift_include`` to integration tests under util/.

    The project-wide default in ``pyproject.toml`` is ``sift_autouse
    = false`` so unit tests pay nothing for the globally-loaded Sift plugin.
    Integration tests in this subtree still need the autouse fixtures, so this
    hook flips the gate back on for any test already marked
    ``@pytest.mark.integration``. Unit tests in the same directory (e.g.
    ``test_cel_utils.py``) are left alone.

    ``pytest_collection_modifyitems`` receives all items in the session (pytest
    does not auto-scope it to the conftest's directory), so we filter by path
    explicitly. ``Path.relative_to`` is the 3.8-compatible form of the path
    containment check (``Path.is_relative_to`` arrived in 3.9).
    """
    for item in items:
        try:
            item.path.relative_to(_HERE)
        except ValueError:
            continue
        if item.get_closest_marker("integration") is None:
            continue
        item.add_marker(pytest.mark.sift_include)
