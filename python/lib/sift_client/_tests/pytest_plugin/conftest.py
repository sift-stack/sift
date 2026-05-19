"""Shared helpers for the pytest-plugin test suite.

The tests in this directory drive inner pytester sessions to exercise the
plugin's behavior in isolation. The fixtures below produce the boilerplate
conftests those inner sessions need:

- ``write_plugin_conftest``: minimal conftest that loads the plugin
- ``write_probe_conftest``: conftest that loads the plugin and runs a probe
  block inside ``pytest_configure``, useful for inspecting internal state
  without running tests against a real backend

Every test in this suite invokes the inner session via
``pytester.runpytest_subprocess(...)`` rather than ``pytester.runpytest(...)``.
``runpytest`` runs the inner pytest in-process, which re-imports the Sift
plugin on each test; the plugin transitively imports numpy, whose C
extensions refuse to initialize twice in one process and raise
``cannot load module more than once per process``. Spawning a subprocess
gives each inner session a fresh interpreter and sidesteps that guard.
"""

from __future__ import annotations

import textwrap
from typing import Callable

import pytest


@pytest.fixture
def write_plugin_conftest(pytester: pytest.Pytester) -> Callable[[], None]:
    """Return a callable that writes a minimal conftest loading the plugin."""

    def _write() -> None:
        pytester.makeconftest('pytest_plugins = ["sift_client.pytest_plugin"]')

    return _write


@pytest.fixture
def write_probe_conftest(pytester: pytest.Pytester) -> Callable[[str], None]:
    """Return a callable that writes a conftest running ``probe_body`` in ``pytest_configure``.

    ``probe_body`` is python source that runs at config time with ``config``
    in scope; use ``print(...)`` calls and capture them with
    ``result.stdout.fnmatch_lines``.
    """

    def _write(probe_body: str) -> None:
        pytester.makeconftest(
            'pytest_plugins = ["sift_client.pytest_plugin"]\n\n'
            "def pytest_configure(config):\n" + textwrap.indent(textwrap.dedent(probe_body), "    ")
        )

    return _write
