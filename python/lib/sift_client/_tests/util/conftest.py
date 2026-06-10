import os
import platform
from importlib.metadata import PackageNotFoundError, version
from pathlib import Path

import pytest

_HERE = Path(__file__).parent


def pytest_configure(config: pytest.Config) -> None:
    """Configure the pytest configuration to disable the Sift test results log file."""
    config.option.sift_log_file = False


@pytest.fixture(scope="session", autouse=True)
def stamp_run_metadata(request: pytest.FixtureRequest) -> None:
    """Stamp run-wide metadata on the report: where it ran plus SDK/Python versions.

    ``environment`` is ``ci`` when a CI provider sets ``CI=true``, else ``local``.
    ``sdk_version``/``python_version`` record what produced the report. Stamped
    once per session, spreading the existing metadata first since ``update``
    replaces the map wholesale. Skipped under ``--sift-disabled`` (unit runs) so
    no report is created just to carry it.
    """
    if request.config.option.sift_disabled:
        return
    try:
        sdk_version = version("sift_stack_py")
    except PackageNotFoundError:
        sdk_version = "unknown"
    report_context = request.getfixturevalue("report_context")
    report_context.report.update(
        {
            "metadata": {
                **report_context.report.metadata,
                "environment": "ci" if os.environ.get("CI") else "local",
                "sdk_version": sdk_version,
                "python_version": platform.python_version(),
            }
        }
    )


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
