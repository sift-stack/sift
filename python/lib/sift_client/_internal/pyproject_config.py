"""Loader for the ``[tool.sift]`` table in a project's ``pyproject.toml``.

The pytest plugin consumes this loader to resolve report-content config (under
``[tool.sift.pytest.report]``) and SDK-level fallbacks (URIs under
``[tool.sift]``). A malformed or missing ``pyproject.toml`` returns ``{}`` so a
bad config file never aborts the session — the plugin falls back to its
built-in defaults and surfaces a single warning.
"""

from __future__ import annotations

import warnings
from pathlib import Path
from typing import TYPE_CHECKING, Any

# ``tomllib`` landed in 3.11; ``tomli`` is the same parser packaged for older
# interpreters and is declared as a conditional install dep on 3.8-3.10.
try:
    import tomllib  # type: ignore[import-not-found,unused-ignore]
except ImportError:  # pragma: no cover - exercised on 3.8-3.10 only
    import tomli as tomllib  # type: ignore[no-redef]

if TYPE_CHECKING:
    import pytest


# Bound the upward walk so a misconfigured environment can't trigger an
# unbounded filesystem traversal looking for a project root that isn't there.
_MAX_PARENT_WALK = 3


def _find_pyproject(config: pytest.Config) -> Path | None:
    """Locate the active project's ``pyproject.toml``.

    Order:
    1. ``config.inipath`` when it is itself a ``pyproject.toml`` (the common
       case: project uses ``[tool.pytest.ini_options]`` so pytest loaded the
       ini settings directly from pyproject).
    2. ``<config.rootpath>/pyproject.toml``.
    3. A bounded walk upward from ``rootpath`` for monorepo layouts where
       pytest's rootdir is a subdirectory and the project pyproject lives
       higher up.
    """
    inipath = config.inipath
    if inipath is not None and inipath.name == "pyproject.toml" and inipath.is_file():
        return inipath
    cur = Path(config.rootpath).resolve()
    candidate = cur / "pyproject.toml"
    if candidate.is_file():
        return candidate
    for _ in range(_MAX_PARENT_WALK):
        cur = cur.parent
        candidate = cur / "pyproject.toml"
        if candidate.is_file():
            return candidate
    return None


def load_tool_sift(config: pytest.Config) -> dict[str, Any]:
    """Return the parsed ``[tool.sift]`` table from the project's pyproject.toml.

    Returns ``{}`` when no pyproject is discoverable, when the file omits the
    ``[tool.sift]`` table, or when parsing fails. A parse / IO failure emits a
    single :class:`SiftPytestPluginWarning` so the session continues with
    defaults rather than aborting on a malformed file.
    """
    pyproject = _find_pyproject(config)
    if pyproject is None:
        return {}
    try:
        with pyproject.open("rb") as fh:
            data = tomllib.load(fh)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        # Deferred import: ``pytest_plugin`` imports this loader, so a
        # top-level import here would close the cycle at module load time.
        from sift_client.pytest_plugin import SiftPytestPluginWarning

        warnings.warn(
            f"Failed to read {pyproject} for [tool.sift]: {type(exc).__name__}: {exc}",
            SiftPytestPluginWarning,
            stacklevel=2,
        )
        return {}
    return (data.get("tool") or {}).get("sift") or {}
