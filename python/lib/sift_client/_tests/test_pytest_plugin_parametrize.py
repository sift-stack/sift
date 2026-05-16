"""Integration tests for the pytest plugin's nested-parametrize behavior.

Each test spins up an inner pytest run via ``pytester`` whose conftest wires
in a ``FakeReportContext`` (defined in ``_pytester_fakes.py``) that records
every step creation to a JSON file. The outer test reads that file and
asserts the resulting step tree.

The inner conftest is intentionally a trivial wrapper around the real fake
so the test logic lives in plain Python that lints and types normally.
"""

from __future__ import annotations

import json
import os
from textwrap import dedent
from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from pathlib import Path

pytest_plugins = ["pytester"]

_STEPS_FILE_ENV = "SIFT_FAKE_STEPS_FILE"

_INNER_CONFTEST = f"""
import os
from pathlib import Path
from unittest.mock import MagicMock

import pytest

pytest_plugins = ["sift_client.pytest_plugin"]

from sift_client._tests._pytester_fakes import FakeReportContext


@pytest.fixture(scope="session")
def sift_client():
    return MagicMock()


@pytest.fixture(scope="session", autouse=True)
def report_context(sift_client):
    import sift_client.pytest_plugin as plugin_module
    steps_file = Path(os.environ[{_STEPS_FILE_ENV!r}])
    with FakeReportContext(steps_file) as ctx:
        plugin_module.REPORT_CONTEXT = ctx
        yield ctx
"""


@pytest.fixture(scope="session")
def client_has_connection() -> bool:
    """Force the outer session's plugin to no-op so it doesn't reach for a real Sift server."""
    return False


def _setup(pytester: pytest.Pytester) -> Path:
    steps_file = pytester.path / "captured_steps.json"
    pytester.makeconftest(_INNER_CONFTEST)
    os.environ[_STEPS_FILE_ENV] = str(steps_file)
    return steps_file


def _by_name(steps: list[dict]) -> dict[str, list[dict]]:
    out: dict[str, list[dict]] = {}
    for s in steps:
        out.setdefault(s["name"], []).append(s)
    return out


def test_single_parametrize_clusters_under_originalname(pytester: pytest.Pytester) -> None:
    steps_file = _setup(pytester)
    pytester.makepyfile(
        test_rail=dedent(
            """
            import pytest

            @pytest.mark.parametrize("v", [3.3, 5.0])
            def test_rail(v):
                pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    # Module step + one shared `test_rail` parent + two leaves.
    assert len(by_name["test_rail.py"]) == 1
    assert len(by_name["test_rail"]) == 1
    assert len(by_name["v=3.3"]) == 1
    assert len(by_name["v=5.0"]) == 1
    test_rail_id = by_name["test_rail"][0]["id"]
    assert by_name["v=3.3"][0]["parent_step_id"] == test_rail_id
    assert by_name["v=5.0"][0]["parent_step_id"] == test_rail_id


def test_stacked_parametrize_nests_outer_to_inner(pytester: pytest.Pytester) -> None:
    steps_file = _setup(pytester)
    pytester.makepyfile(
        test_iso=dedent(
            """
            import pytest

            @pytest.mark.parametrize("voltage", ["high", "low"])
            @pytest.mark.parametrize("component", ["motor", "ducer"])
            def test_iso(voltage, component):
                pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=4)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    # One `test_iso` parent, two `voltage='…'` parents, four `component='…'` leaves.
    assert len(by_name["test_iso"]) == 1
    assert len(by_name["voltage='high'"]) == 1
    assert len(by_name["voltage='low'"]) == 1
    assert len(by_name["component='motor'"]) == 2  # one per voltage
    assert len(by_name["component='ducer'"]) == 2
    test_iso_id = by_name["test_iso"][0]["id"]
    vh_id = by_name["voltage='high'"][0]["parent_step_id"]
    vl_id = by_name["voltage='low'"][0]["parent_step_id"]
    assert vh_id == test_iso_id
    assert vl_id == test_iso_id
    # Each component leaf parents to one of the voltage parents.
    voltage_ids = {
        by_name["voltage='high'"][0]["id"],
        by_name["voltage='low'"][0]["id"],
    }
    for leaf in by_name["component='motor'"] + by_name["component='ducer'"]:
        assert leaf["parent_step_id"] in voltage_ids


def test_fixture_parametrization_participates(pytester: pytest.Pytester) -> None:
    steps_file = _setup(pytester)
    pytester.makepyfile(
        test_widget=dedent(
            """
            import pytest

            @pytest.fixture(params=["a", "b"])
            def widget(request):
                return request.param

            def test_widget(widget):
                pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    assert len(by_name["test_widget"]) == 1
    parent_id = by_name["test_widget"][0]["id"]
    assert by_name["widget='a'"][0]["parent_step_id"] == parent_id
    assert by_name["widget='b'"][0]["parent_step_id"] == parent_id


def test_module_boundary_isolates_parametrize_stack(pytester: pytest.Pytester) -> None:
    steps_file = _setup(pytester)
    pytester.makepyfile(
        test_a=dedent(
            """
            import pytest

            @pytest.mark.parametrize("v", [1, 2])
            def test_one(v):
                pass
            """
        ),
        test_b=dedent(
            """
            import pytest

            @pytest.mark.parametrize("w", ["x", "y"])
            def test_two(w):
                pass
            """
        ),
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=4)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    # Each module step contains its own `test_one`/`test_two` parametrize subtree.
    mod_a = by_name["test_a.py"][0]
    mod_b = by_name["test_b.py"][0]
    assert by_name["test_one"][0]["parent_step_id"] == mod_a["id"]
    assert by_name["test_two"][0]["parent_step_id"] == mod_b["id"]


def test_leaf_parent_chain_terminates_at_report(pytester: pytest.Pytester) -> None:
    steps_file = _setup(pytester)
    pytester.makepyfile(
        test_chain=dedent(
            """
            import pytest

            @pytest.mark.parametrize("a", [1])
            @pytest.mark.parametrize("b", ["x"])
            def test_chain(a, b):
                pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=1)
    steps = json.loads(steps_file.read_text())
    by_id = {s["id"]: s for s in steps}
    leaf = next(s for s in steps if s["name"].startswith("b="))
    chain = []
    cur = leaf
    while cur is not None:
        chain.append(cur["name"])
        parent_id = cur["parent_step_id"]
        cur = by_id.get(parent_id) if parent_id else None
    # leaf b=… → a=… → test_chain → test_chain.py (module step) → root
    assert chain == ["b='x'", "a=1", "test_chain", "test_chain.py"]
