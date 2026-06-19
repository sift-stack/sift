"""Run-mode detection and the per-test Sift gate.

Resolves the active mode (disabled > offline > online) from the ``DISABLED_OPTION`` /
``OFFLINE_OPTION`` options, and decides whether the Sift autouse fixtures activate for
a given node via the ``sift_include`` / ``sift_exclude`` markers.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.pytest_plugin.options import (
    AUTOUSE_OPTION,
    DISABLED_OPTION,
    OFFLINE_OPTION,
)

if TYPE_CHECKING:
    import pytest


def is_offline(pytestconfig: pytest.Config | None) -> bool:
    return bool(OFFLINE_OPTION.resolve(pytestconfig))


def is_disabled(pytestconfig: pytest.Config | None) -> bool:
    return bool(DISABLED_OPTION.resolve(pytestconfig))


def sdk_version() -> str:
    """Return the installed ``sift_stack_py`` version, or ``"unknown"``."""
    from importlib.metadata import PackageNotFoundError, version

    try:
        return version("sift_stack_py")
    except PackageNotFoundError:
        return "unknown"


def mode_label(config: pytest.Config) -> str:
    """Resolve the active mode for the terminal header: disabled > offline > online."""
    if is_disabled(config):
        return "disabled"
    if is_offline(config):
        return "offline"
    return "online"


def sift_enabled_for(node: pytest.Item | pytest.Collector, default: bool) -> bool:
    """Resolve the Sift gate for a node: sift_exclude > sift_include > default.

    `get_closest_marker` walks the node hierarchy upward, so markers applied
    at any level (function, class, module, package, session) are honored.
    """
    if node.get_closest_marker("sift_exclude"):
        return False
    if node.get_closest_marker("sift_include"):
        return True
    return default


def gate_enabled(node: pytest.Item | pytest.Collector, config: pytest.Config) -> bool:
    """Whether the Sift autouse fixtures should activate for ``node``.

    Combines the ``sift_autouse`` ini default with the per-test marker gate, so
    the ``step`` and parent-step fixtures share one entry point.
    """
    return sift_enabled_for(node, bool(AUTOUSE_OPTION.resolve(config)))
