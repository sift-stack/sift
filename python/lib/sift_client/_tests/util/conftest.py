from __future__ import annotations

import os
import platform
from importlib.metadata import PackageNotFoundError, version
from pathlib import Path

import pytest

_HERE = Path(__file__).parent


def pytest_configure(config: pytest.Config) -> None:
    """Configure the pytest configuration to disable the Sift test results log file."""
    config.option.sift_log_file = False


@pytest.fixture(scope="session")
def sift_report_metadata() -> dict[str, str | float | bool]:
    """Stamp run-wide metadata on the report: where it ran plus SDK/Python versions.

    Overrides the plugin's default (empty) fixture, so these layer over the
    ``[tool.sift.pytest.report.metadata]`` TOML table. ``environment`` is ``ci``
    when a CI provider sets ``CI=true``, else ``local``; ``sdk_version`` and
    ``python_version`` record what produced the report. The plugin resolves this
    only while building the report, so a unit run that creates no report never
    calls it.
    """
    try:
        sdk_version = version("sift_stack_py")
    except PackageNotFoundError:
        sdk_version = "unknown"
    return {
        "environment": "ci" if os.environ.get("CI") else "local",
        "sdk_version": sdk_version,
        "python_version": platform.python_version(),
    }


def pytest_collection_modifyitems(config: pytest.Config, items: list[pytest.Item]) -> None:
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
