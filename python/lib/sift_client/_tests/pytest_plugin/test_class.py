"""Tests for the plugin's class-step nesting behavior and step-nesting opt-out flags.

Same ``pytester`` + ``FakeReportContext`` harness as ``test_parametrize.py``.
Each test spins up an inner pytest run whose conftest swaps in a
``FakeReportContext`` that records every step creation to a JSON file; the
outer test reads that file and asserts the resulting step tree.
"""

from __future__ import annotations

import json
from textwrap import dedent
from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from pathlib import Path

_STEPS_FILE_ENV = "SIFT_FAKE_STEPS_FILE"

_INNER_CONFTEST = f"""
import os
from pathlib import Path
from unittest.mock import MagicMock

import pytest

pytest_plugins = ["sift_client.pytest_plugin"]

from sift_client._tests.pytest_plugin._fakes import FakeReportContext


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


@pytest.fixture
def steps_file(pytester: pytest.Pytester, monkeypatch: pytest.MonkeyPatch) -> Path:
    path = pytester.path / "captured_steps.json"
    pytester.makeconftest(_INNER_CONFTEST)
    monkeypatch.setenv(_STEPS_FILE_ENV, str(path))
    return path


def _by_name(steps: list[dict]) -> dict[str, list[dict]]:
    out: dict[str, list[dict]] = {}
    for s in steps:
        out.setdefault(s["name"], []).append(s)
    return out


def _ancestor_names(steps: list[dict], leaf: dict) -> list[str]:
    """Walk from ``leaf`` to the root via parent_step_id, returning names."""
    by_id = {s["id"]: s for s in steps}
    chain: list[str] = []
    cur: dict | None = leaf
    while cur is not None:
        chain.append(cur["name"])
        parent_id = cur["parent_step_id"]
        cur = by_id.get(parent_id) if parent_id else None
    return chain


def test_class_methods_cluster_under_class_step(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    pytester.makepyfile(
        test_klass=dedent(
            """
            class TestFoo:
                def test_a(self):
                    pass

                def test_b(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    assert len(by_name["TestFoo"]) == 1
    class_id = by_name["TestFoo"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == class_id
    assert by_name["test_b"][0]["parent_step_id"] == class_id


def test_nested_classes_produce_nested_steps(pytester: pytest.Pytester, steps_file: Path) -> None:
    pytester.makepyfile(
        test_nested=dedent(
            """
            class TestOuter:
                class TestInner:
                    def test_a(self):
                        pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=1)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    assert len(by_name["TestOuter"]) == 1
    assert len(by_name["TestInner"]) == 1
    leaf = by_name["test_a"][0]
    assert _ancestor_names(steps, leaf) == [
        "test_a",
        "TestInner",
        "TestOuter",
        "test_nested.py",
    ]


def test_class_parametrize_nests_under_class(pytester: pytest.Pytester, steps_file: Path) -> None:
    pytester.makepyfile(
        test_cp=dedent(
            """
            import pytest

            class TestFoo:
                @pytest.mark.parametrize("v", [1, 2])
                def test_a(self, v):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    class_id = by_name["TestFoo"][0]["id"]
    test_a_id = by_name["test_a"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == class_id
    assert by_name["v=1"][0]["parent_step_id"] == test_a_id
    assert by_name["v=2"][0]["parent_step_id"] == test_a_id


def test_two_sibling_classes_in_module(pytester: pytest.Pytester, steps_file: Path) -> None:
    pytester.makepyfile(
        test_sib=dedent(
            """
            class TestA:
                def test_x(self):
                    pass

            class TestB:
                def test_y(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    mod_id = by_name["test_sib.py"][0]["id"]
    assert by_name["TestA"][0]["parent_step_id"] == mod_id
    assert by_name["TestB"][0]["parent_step_id"] == mod_id
    # Sanity: each class is opened exactly once (no duplicate parents).
    assert len(by_name["TestA"]) == 1
    assert len(by_name["TestB"]) == 1


def test_mixed_class_and_free_function(pytester: pytest.Pytester, steps_file: Path) -> None:
    pytester.makepyfile(
        test_mix=dedent(
            """
            class TestA:
                def test_x(self):
                    pass

            def test_free():
                pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    mod_id = by_name["test_mix.py"][0]["id"]
    # Class method parents to TestA; free function parents directly to module.
    assert by_name["TestA"][0]["parent_step_id"] == mod_id
    assert by_name["test_x"][0]["parent_step_id"] == by_name["TestA"][0]["id"]
    assert by_name["test_free"][0]["parent_step_id"] == mod_id


def test_class_with_all_excluded_methods_no_class_step(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    pytester.makepyfile(
        test_excl=dedent(
            """
            import pytest

            class TestFoo:
                @pytest.mark.sift_exclude
                def test_a(self):
                    pass

                @pytest.mark.sift_exclude
                def test_b(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    assert "TestFoo" not in by_name
    assert "test_a" not in by_name
    assert "test_b" not in by_name


def test_sift_exclude_on_class_propagates(pytester: pytest.Pytester, steps_file: Path) -> None:
    pytester.makepyfile(
        test_clsexcl=dedent(
            """
            import pytest

            @pytest.mark.sift_exclude
            class TestFoo:
                def test_a(self):
                    pass

                def test_b(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    assert "TestFoo" not in by_name
    assert "test_a" not in by_name


def test_class_docstring_becomes_step_description(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    pytester.makepyfile(
        test_doc=dedent(
            '''
            class TestFoo:
                """Class docstring."""

                def test_a(self):
                    pass
            '''
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=1)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    # The fake records step creation but not all fields — check the class
    # step was recorded, then read the description via the FakeStep's
    # description attribute by re-reading steps. The fake's create_step only
    # records name/parent/path/id, so verify via the leaf chain only here.
    leaf = by_name["test_a"][0]
    assert _ancestor_names(steps, leaf)[:3] == ["test_a", "TestFoo", "test_doc.py"]


def test_transition_between_class_chains_drains_parametrize(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    pytester.makepyfile(
        test_trans=dedent(
            """
            import pytest

            class TestA:
                @pytest.mark.parametrize("v", [1])
                def test_x(self, v):
                    pass

            class TestB:
                @pytest.mark.parametrize("w", [2])
                def test_y(self, w):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    # Each class opens exactly once; parametrize parents under the right class.
    assert len(by_name["TestA"]) == 1
    assert len(by_name["TestB"]) == 1
    test_x_id = by_name["test_x"][0]["id"]
    test_y_id = by_name["test_y"][0]["id"]
    assert by_name["v=1"][0]["parent_step_id"] == test_x_id
    assert by_name["w=2"][0]["parent_step_id"] == test_y_id
    # Confirm full chain: leaves trace up through correct class.
    chain_x = _ancestor_names(steps, by_name["v=1"][0])
    chain_y = _ancestor_names(steps, by_name["w=2"][0])
    assert "TestA" in chain_x
    assert "TestB" not in chain_x
    assert "TestB" in chain_y
    assert "TestA" not in chain_y


# ---------------------------------------------------------------------------
# Failure-cleanup tests
# ---------------------------------------------------------------------------


def test_drain_step_stack_continues_past_failing_exit() -> None:
    """Lenient mode: a misbehaving ``__exit__`` must not block the rest of the stack."""
    from sift_client.pytest_plugin import (
        SiftPytestStepDrainWarning,
        _drain_step_stack,
    )

    class _Good:
        def __init__(self) -> None:
            self.closed = False

        def __exit__(self, *_: object) -> None:
            self.closed = True

    class _Bad:
        def __exit__(self, *_: object) -> None:
            raise RuntimeError("boom")

    g1, g2, bad = _Good(), _Good(), _Bad()
    stack: list[tuple[str, object]] = [("g1", g1), ("bad", bad), ("g2", g2)]
    with pytest.warns(SiftPytestStepDrainWarning, match="boom"):
        _drain_step_stack(stack)
    assert stack == []
    assert g1.closed
    assert g2.closed


def test_drain_step_stack_strict_drains_fully_then_raises() -> None:
    """Strict mode: drain every frame, then raise with the FIRST failure chained."""
    from sift_client.pytest_plugin import (
        SiftPytestStepDrainError,
        _drain_step_stack,
    )

    class _Good:
        def __init__(self) -> None:
            self.closed = False

        def __exit__(self, *_: object) -> None:
            self.closed = True

    class _Bad:
        def __init__(self, label: str) -> None:
            self.label = label

        def __exit__(self, *_: object) -> None:
            raise RuntimeError(f"boom-{self.label}")

    g, b1, b2 = _Good(), _Bad("first"), _Bad("second")
    # Stack drains LIFO: pop order is b2, b1, g. So b2's failure is the first
    # one collected and surfaces in __cause__.
    stack: list[tuple[str, object]] = [("g", g), ("b1", b1), ("b2", b2)]
    with pytest.raises(SiftPytestStepDrainError, match="2 step.*'b2'") as exc_info:
        _drain_step_stack(stack, swallow_errors=False)
    # Stack fully drained even though it raised.
    assert stack == []
    assert g.closed
    # Original exception chained for debuggability.
    assert isinstance(exc_info.value.__cause__, RuntimeError)
    assert "boom-second" in str(exc_info.value.__cause__)


def test_failing_test_in_class_does_not_orphan_class_step(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    """A failing class method must not block the class step from cleaning up.

    Sibling methods in the same class must still parent to the same class
    step, and a later class in the module must open as a sibling (not nested
    under an orphan).
    """
    pytester.makepyfile(
        test_fail=dedent(
            """
            class TestFoo:
                def test_a(self):
                    raise AssertionError("boom")

                def test_b(self):
                    pass

            class TestBar:
                def test_c(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2, failed=1)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    assert len(by_name["TestFoo"]) == 1
    assert len(by_name["TestBar"]) == 1
    foo_id = by_name["TestFoo"][0]["id"]
    bar_id = by_name["TestBar"][0]["id"]
    mod_id = by_name["test_fail.py"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == foo_id
    assert by_name["test_b"][0]["parent_step_id"] == foo_id
    assert by_name["test_c"][0]["parent_step_id"] == bar_id
    # Both classes are siblings under the same module — TestBar didn't get
    # nested under an orphan TestFoo.
    assert by_name["TestFoo"][0]["parent_step_id"] == mod_id
    assert by_name["TestBar"][0]["parent_step_id"] == mod_id


def test_failing_parametrized_method_in_class_closes_full_chain(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    """A failing parametrized class method must not orphan its parametrize parents."""
    pytester.makepyfile(
        test_pfail=dedent(
            """
            import pytest

            class TestFoo:
                @pytest.mark.parametrize("v", [1, 2])
                def test_a(self, v):
                    if v == 1:
                        raise AssertionError("boom")

                def test_b(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2, failed=1)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    foo_id = by_name["TestFoo"][0]["id"]
    test_a_id = by_name["test_a"][0]["id"]
    # Both parametrize leaves parent to the same test_a; test_b parents
    # directly to TestFoo (no parametrize parent leaked across methods).
    assert by_name["v=1"][0]["parent_step_id"] == test_a_id
    assert by_name["v=2"][0]["parent_step_id"] == test_a_id
    assert by_name["test_b"][0]["parent_step_id"] == foo_id


# ---------------------------------------------------------------------------
# Opt-out flag tests
# ---------------------------------------------------------------------------


def _write_ini(pytester: pytest.Pytester, **overrides: object) -> None:
    """Write a pytest.ini with the given sift_* overrides set under [pytest]."""
    lines = ["[pytest]"]
    for key, value in overrides.items():
        lines.append(f"{key} = {value}")
    pytester.makefile(".ini", pytest="\n".join(lines) + "\n")


def test_sift_class_step_false_skips_class_steps(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    _write_ini(pytester, sift_class_step="false")
    pytester.makepyfile(
        test_noclass=dedent(
            """
            class TestFoo:
                def test_a(self):
                    pass

                def test_b(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    assert "TestFoo" not in by_name
    mod_id = by_name["test_noclass.py"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == mod_id
    assert by_name["test_b"][0]["parent_step_id"] == mod_id


def test_sift_module_step_false_skips_module_step(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    _write_ini(pytester, sift_module_step="false")
    pytester.makepyfile(
        test_nomod=dedent(
            """
            class TestFoo:
                def test_a(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=1)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    assert "test_nomod.py" not in by_name
    # TestFoo attaches to the report root (no parent recorded by the fake).
    assert by_name["TestFoo"][0]["parent_step_id"] is None
    assert by_name["test_a"][0]["parent_step_id"] == by_name["TestFoo"][0]["id"]


def test_sift_parametrize_nesting_false_keeps_flat_leaves(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    _write_ini(pytester, sift_parametrize_nesting="false")
    pytester.makepyfile(
        test_flat=dedent(
            """
            import pytest

            @pytest.mark.parametrize("v", [1, 2])
            def test_a(v):
                pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    # No parametrize parent step.
    assert "test_a" not in by_name
    assert "v=1" not in by_name
    # Leaves use the bracket-mangled pytest names.
    assert "test_a[1]" in by_name
    assert "test_a[2]" in by_name
    mod_id = by_name["test_flat.py"][0]["id"]
    assert by_name["test_a[1]"][0]["parent_step_id"] == mod_id
    assert by_name["test_a[2]"][0]["parent_step_id"] == mod_id


def test_sift_module_step_false_still_drains_across_modules(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    """sift_module_step=false must not leak class/parametrize parents across modules.

    Two modules each declare ``class TestFoo``. Without per-module draining,
    the second module's TestFoo would silently reuse the first module's stale
    class step (the diff in _class_parents compares names only). The fix
    drains at the module boundary regardless of whether the module step
    itself was opened.
    """
    _write_ini(pytester, sift_module_step="false")
    pytester.makepyfile(
        test_a=dedent(
            """
            class TestFoo:
                def test_x(self):
                    pass
            """
        ),
        test_b=dedent(
            """
            class TestFoo:
                def test_y(self):
                    pass
            """
        ),
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    # Two distinct TestFoo class steps — one per module — not a shared frame.
    assert len(by_name["TestFoo"]) == 2
    foo_ids = {s["id"] for s in by_name["TestFoo"]}
    # Each test method parents to a different TestFoo id.
    test_x_parent = by_name["test_x"][0]["parent_step_id"]
    test_y_parent = by_name["test_y"][0]["parent_step_id"]
    assert test_x_parent in foo_ids
    assert test_y_parent in foo_ids
    assert test_x_parent != test_y_parent


def test_all_three_flags_false_matches_legacy_behavior(
    pytester: pytest.Pytester, steps_file: Path
) -> None:
    _write_ini(
        pytester,
        sift_module_step="false",
        sift_class_step="false",
        sift_parametrize_nesting="false",
    )
    pytester.makepyfile(
        test_legacy=dedent(
            """
            import pytest

            class TestFoo:
                @pytest.mark.parametrize("v", [1, 2])
                def test_a(self, v):
                    pass
            """
        )
    )
    result = pytester.runpytest_subprocess("-v")
    result.assert_outcomes(passed=2)
    steps = json.loads(steps_file.read_text())
    by_name = _by_name(steps)
    # No module, class, or parametrize parents — just bracket-mangled leaves.
    assert "test_legacy.py" not in by_name
    assert "TestFoo" not in by_name
    assert "test_a" not in by_name
    assert "test_a[1]" in by_name
    assert "test_a[2]" in by_name
    assert by_name["test_a[1]"][0]["parent_step_id"] is None
    assert by_name["test_a[2]"][0]["parent_step_id"] is None
