"""Shared helpers for the pytest-plugin test suite.

The tests in this directory drive inner pytester sessions to exercise the
plugin's behavior in isolation. The fixtures below produce the boilerplate
conftests those inner sessions need:

- ``write_plugin_conftest``: minimal conftest that loads the plugin
- ``write_probe_conftest``: conftest that loads the plugin and runs a probe
  block inside ``pytest_configure``, useful for inspecting internal state
  without running tests against a real backend

The offline-log tests (``test_hierarchy.py``, ``test_pass_fail.py``) drive the
inner session in-process via ``pytester.runpytest_inprocess(...)``. This is
fast because the outer session already preloads the plugin (``pyproject.toml``
sets ``addopts = "... -p sift_client.pytest_plugin ..."``), so the numpy C
extensions the plugin pulls in are imported once for the whole outer process
and reused by every inner run — no per-test interpreter spawn, and no
``cannot load module more than once per process`` re-init guard to trip.

Tests that need true process isolation (fresh env vars, credential and
connection resolution, ini parsing) still use ``pytester.runpytest_subprocess(...)``
so the inner session starts from a clean interpreter.
"""

from __future__ import annotations

import textwrap
from typing import Callable

import pytest

_SIFT_ENV_VARS = ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI", "SIFT_DISABLED")


@pytest.fixture
def clear_sift_env(monkeypatch: pytest.MonkeyPatch) -> None:
    """Unset all ``SIFT_*`` environment variables for the duration of the test."""
    for name in _SIFT_ENV_VARS:
        monkeypatch.delenv(name, raising=False)


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
