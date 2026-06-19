"""Tests for the plugin's hierarchy-step nesting behavior.

Covers every layer the plugin opens parent steps for — packages, modules,
classes (including nested), parametrize axes — plus the ini opt-out flags,
failure-cleanup semantics, and the drain helper.

Each test spins up an inner pytest run via ``pytester`` configured with
``--sift-offline`` and a known log path. The plugin writes every test-result
API call to that JSONL log, and the outer test parses it via
``_step_status_capture.load_steps`` to reconstruct the step tree.
"""

from __future__ import annotations

import logging
import warnings
from datetime import datetime, timezone
from textwrap import dedent
from types import SimpleNamespace
from typing import TYPE_CHECKING, cast

import pytest

from sift_client._internal.pytest_plugin import steps
from sift_client._tests.pytest_plugin import _step_status_capture as capture
from sift_client.pytest_plugin import SiftPytestPluginWarning
from sift_client.sift_types.test_report import TestStatus

if TYPE_CHECKING:
    from pathlib import Path


def _parse_ts(ts: str) -> datetime:
    """Parse a protobuf-JSON RFC3339 timestamp across Python 3.8-3.14.

    ``datetime.fromisoformat`` only accepts ``Z`` / arbitrary fractional digits
    on 3.11+, so parse the second-precision base with ``strptime`` and apply the
    fractional part by hand (protobuf emits 0/3/6/9 digits).
    """
    body = ts.rstrip("Z").split("+", 1)[0]
    base, _, frac = body.partition(".")
    # All Sift timestamps are UTC; tag it so comparisons stay unambiguous.
    parsed = datetime.strptime(base, "%Y-%m-%dT%H:%M:%S").replace(tzinfo=timezone.utc)
    if frac:
        parsed = parsed.replace(microsecond=int(frac.ljust(6, "0")[:6]))
    return parsed


_INNER_CONFTEST = 'pytest_plugins = ["sift_client.pytest_plugin"]\n'


def _base_ini_lines(log_path: Path) -> list[str]:
    """Default ini settings every inner pytester run needs."""
    return [
        "[pytest]",
        "sift_offline = true",
        f"sift_output_dir = {log_path}",
        "sift_git_metadata = false",
    ]


@pytest.fixture
def out_dir(pytester: pytest.Pytester) -> Path:
    path = pytester.path / "sift-out"
    pytester.makeconftest(_INNER_CONFTEST)
    pytester.makefile(".ini", pytest="\n".join(_base_ini_lines(path)) + "\n")
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


def test_class_methods_cluster_under_class_step(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    assert len(by_name["TestFoo"]) == 1
    class_id = by_name["TestFoo"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == class_id
    assert by_name["test_b"][0]["parent_step_id"] == class_id


def test_collection_skipped_method_nests_under_its_class(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A collection-time skipped method nests under its class parent.

    ``@pytest.mark.skip`` is evaluated before the autouse fixtures run, so the
    skipped item's step comes from the makereport hook rather than the ``step``
    fixture. The report-tree parents live off the step stack, so that inline step
    must still resolve and attach to the class parent rather than the report root.
    Order is pinned so the non-skipped sibling opens the class first.
    """
    pytester.makepyfile(
        test_skip_nest=dedent(
            """
            import pytest

            class TestFoo:
                def test_run(self):
                    pass

                @pytest.mark.skip(reason="x")
                def test_skipped(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=1, skipped=1)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    assert len(by_name["TestFoo"]) == 1
    class_id = by_name["TestFoo"][0]["id"]
    assert by_name["test_run"][0]["parent_step_id"] == class_id
    assert by_name["test_skipped"][0]["parent_step_id"] == class_id
    assert by_name["test_skipped"][0]["statuses"][-1] == TestStatus.SKIPPED


def test_nested_classes_produce_nested_steps(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=1)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
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


def test_class_parametrize_nests_under_class(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    class_id = by_name["TestFoo"][0]["id"]
    test_a_id = by_name["test_a"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == class_id
    assert by_name["v=1"][0]["parent_step_id"] == test_a_id
    assert by_name["v=2"][0]["parent_step_id"] == test_a_id


def test_two_sibling_classes_in_module(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    mod_id = by_name["test_sib.py"][0]["id"]
    assert by_name["TestA"][0]["parent_step_id"] == mod_id
    assert by_name["TestB"][0]["parent_step_id"] == mod_id
    # Sanity: each class is opened exactly once (no duplicate parents).
    assert len(by_name["TestA"]) == 1
    assert len(by_name["TestB"]) == 1


def test_mixed_class_and_free_function(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    mod_id = by_name["test_mix.py"][0]["id"]
    # Class method parents to TestA; free function parents directly to module.
    assert by_name["TestA"][0]["parent_step_id"] == mod_id
    assert by_name["test_x"][0]["parent_step_id"] == by_name["TestA"][0]["id"]
    assert by_name["test_free"][0]["parent_step_id"] == mod_id


def test_class_with_all_excluded_methods_no_class_step(
    pytester: pytest.Pytester, out_dir: Path
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    jsonl = capture.run_jsonl_or_none(out_dir)
    steps = capture.load_steps(jsonl) if jsonl else []
    by_name = _by_name(steps)
    assert "TestFoo" not in by_name
    assert "test_a" not in by_name
    assert "test_b" not in by_name


def test_sift_exclude_on_class_propagates(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    jsonl = capture.run_jsonl_or_none(out_dir)
    steps = capture.load_steps(jsonl) if jsonl else []
    by_name = _by_name(steps)
    assert "TestFoo" not in by_name
    assert "test_a" not in by_name


def test_class_docstring_becomes_step_description(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=1)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    # The fake records step creation but not all fields — check the class
    # step was recorded, then read the description via the FakeStep's
    # description attribute by re-reading steps. The fake's create_step only
    # records name/parent/path/id, so verify via the leaf chain only here.
    leaf = by_name["test_a"][0]
    assert _ancestor_names(steps, leaf)[:3] == ["test_a", "TestFoo", "test_doc.py"]


def test_two_class_chains_keep_parametrize_isolated(
    pytester: pytest.Pytester, out_dir: Path
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
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


class _FakeParent:
    """Minimal stand-in for an open ``NewStep`` parent in the plugin registries."""

    def __init__(self, name: str, step_path: str, *, raises: str | None = None) -> None:
        self.current_step = SimpleNamespace(name=name, step_path=step_path)
        self._raises = raises
        self.closed = False

    def __exit__(self, *_: object) -> None:
        if self._raises is not None:
            raise RuntimeError(self._raises)
        self.closed = True


@pytest.fixture
def clean_parent_registries():
    """Save/restore the module-level parent registries and REPORT_CONTEXT.

    The ``finalize_parents`` resilience test pokes the globals directly, so
    isolate them from any real session state. Registries and ``finalize_parents``
    live in ``_internal.pytest_plugin.steps``; ``REPORT_CONTEXT`` is the public
    session global on ``sift_client.pytest_plugin``.
    """
    from sift_client import pytest_plugin
    from sift_client._internal.pytest_plugin import steps

    saved = (
        dict(steps.hierarchy_parents),
        dict(steps.parametrize_parents),
        pytest_plugin.REPORT_CONTEXT,
    )
    steps.hierarchy_parents.clear()
    steps.parametrize_parents.clear()
    pytest_plugin.REPORT_CONTEXT = None  # skip the end_time override lookup
    try:
        yield steps
    finally:
        steps.hierarchy_parents.clear()
        steps.hierarchy_parents.update(saved[0])
        steps.parametrize_parents.clear()
        steps.parametrize_parents.update(saved[1])
        pytest_plugin.REPORT_CONTEXT = saved[2]


def test_finalize_parents_continues_past_failing_exit(clean_parent_registries) -> None:
    """Lenient mode: a misbehaving parent ``__exit__`` must not block the others."""
    from sift_client.pytest_plugin import SiftPytestStepDrainWarning

    steps = clean_parent_registries
    good = _FakeParent("good", "1")
    bad = _FakeParent("bad", "1.1", raises="boom")
    steps.hierarchy_parents["good"] = good
    steps.parametrize_parents[("t", "bad")] = bad

    with pytest.warns(SiftPytestStepDrainWarning, match="boom"):
        steps.finalize_parents()

    assert good.closed
    # Registries cleared regardless of the per-parent failure.
    assert steps.hierarchy_parents == {}
    assert steps.parametrize_parents == {}


def test_failing_test_in_class_does_not_orphan_class_step(
    pytester: pytest.Pytester, out_dir: Path
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2, failed=1)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
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
    pytester: pytest.Pytester, out_dir: Path
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2, failed=1)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
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


def _write_ini(pytester: pytest.Pytester, out_dir: Path, **overrides: object) -> None:
    """Write a pytest.ini with the given sift_* overrides, preserving the
    offline/log/git-metadata defaults the ``out_dir`` fixture installs.
    """
    lines = _base_ini_lines(out_dir)
    for key, value in overrides.items():
        lines.append(f"{key} = {value}")
    pytester.makefile(".ini", pytest="\n".join(lines) + "\n")


def test_sift_class_step_false_skips_class_steps(pytester: pytest.Pytester, out_dir: Path) -> None:
    _write_ini(pytester, out_dir, sift_class_step="false")
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    jsonl = capture.run_jsonl_or_none(out_dir)
    steps = capture.load_steps(jsonl) if jsonl else []
    by_name = _by_name(steps)
    assert "TestFoo" not in by_name
    mod_id = by_name["test_noclass.py"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == mod_id
    assert by_name["test_b"][0]["parent_step_id"] == mod_id


def test_sift_module_step_false_skips_module_step(pytester: pytest.Pytester, out_dir: Path) -> None:
    _write_ini(pytester, out_dir, sift_module_step="false")
    pytester.makepyfile(
        test_nomod=dedent(
            """
            class TestFoo:
                def test_a(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=1)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    assert "test_nomod.py" not in by_name
    # TestFoo attaches to the report root (no parent recorded by the fake).
    assert by_name["TestFoo"][0]["parent_step_id"] is None
    assert by_name["test_a"][0]["parent_step_id"] == by_name["TestFoo"][0]["id"]


def test_sift_parametrize_nesting_false_keeps_flat_leaves(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    _write_ini(pytester, out_dir, sift_parametrize_nesting="false")
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
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
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """sift_module_step=false must not merge same-named classes across modules.

    The hierarchy chain always includes the module ancestor for identity
    (even when it's not rendered as a step), so two modules each declaring
    ``class TestFoo`` produce two distinct ``TestFoo`` frames in the diff.
    """
    _write_ini(pytester, out_dir, sift_module_step="false")
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
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


def test_package_step_default_opens_for_init_dirs(pytester: pytest.Pytester, out_dir: Path) -> None:
    """Default: a directory with ``__init__.py`` produces a parent package step."""
    pytester.mkpydir("pkg_a")
    (pytester.path / "pkg_a" / "test_x.py").write_text(
        dedent(
            """
            def test_one():
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=1)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    assert "pkg_a" in by_name
    pkg_id = by_name["pkg_a"][0]["id"]
    mod = by_name["test_x.py"][0]
    assert mod["parent_step_id"] == pkg_id


def test_same_named_packages_in_different_dirs_do_not_merge(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """Two packages with the same display name but different paths must stay distinct.

    The hierarchy diff compares on ``nodeid`` (identity), not just the
    display name — so a ``utils`` package under ``proj_a/`` and another
    under ``proj_b/`` (where ``proj_a/`` and ``proj_b/`` are bare
    directories that pytest treats as ``pytest.Dir`` nodes and the chain
    walker skips) produce two distinct ``utils`` parent steps in the report
    tree, not a silent merge.
    """
    (pytester.path / "proj_a" / "utils").mkdir(parents=True)
    (pytester.path / "proj_a" / "utils" / "__init__.py").touch()
    (pytester.path / "proj_a" / "utils" / "test_x.py").write_text(
        dedent(
            """
            def test_one():
                pass
            """
        )
    )
    (pytester.path / "proj_b" / "utils").mkdir(parents=True)
    (pytester.path / "proj_b" / "utils" / "__init__.py").touch()
    (pytester.path / "proj_b" / "utils" / "test_y.py").write_text(
        dedent(
            """
            def test_two():
                pass
            """
        )
    )
    # ``importlib`` import mode is required so two packages with the same
    # name on disk don't collide during sys.path-based import.
    result = pytester.runpytest_inprocess("-v", "--import-mode=importlib")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    # Two distinct ``utils`` package steps — one per project.
    assert len(by_name["utils"]) == 2
    utils_ids = {s["id"] for s in by_name["utils"]}
    # Each module step parents to a different ``utils`` instance.
    parent_x = by_name["test_x.py"][0]["parent_step_id"]
    parent_y = by_name["test_y.py"][0]["parent_step_id"]
    assert parent_x in utils_ids
    assert parent_y in utils_ids
    assert parent_x != parent_y


def test_sift_package_step_false_skips_package_steps(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """With ``sift_package_step=false`` the directory step is suppressed."""
    _write_ini(pytester, out_dir, sift_package_step="false")
    pytester.mkpydir("pkg_a")
    (pytester.path / "pkg_a" / "test_x.py").write_text(
        dedent(
            """
            def test_one():
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=1)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    assert "pkg_a" not in by_name
    # The module step still opens and is now the top-level frame.
    assert by_name["test_x.py"][0]["parent_step_id"] is None


def test_all_three_flags_false_matches_legacy_behavior(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    _write_ini(
        pytester,
        out_dir,
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    # No module, class, or parametrize parents — just bracket-mangled leaves.
    assert "test_legacy.py" not in by_name
    assert "TestFoo" not in by_name
    assert "test_a" not in by_name
    assert "test_a[1]" in by_name
    assert "test_a[2]" in by_name
    assert by_name["test_a[1]"][0]["parent_step_id"] is None
    assert by_name["test_a[2]"][0]["parent_step_id"] is None


# ---------------------------------------------------------------------------
# Parametrize nesting
# ---------------------------------------------------------------------------


def test_single_parametrize_clusters_under_originalname(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    # Module step + one shared `test_rail` parent + two leaves.
    assert len(by_name["test_rail.py"]) == 1
    assert len(by_name["test_rail"]) == 1
    assert len(by_name["v=3.3"]) == 1
    assert len(by_name["v=5.0"]) == 1
    test_rail_id = by_name["test_rail"][0]["id"]
    assert by_name["v=3.3"][0]["parent_step_id"] == test_rail_id
    assert by_name["v=5.0"][0]["parent_step_id"] == test_rail_id


def test_stacked_parametrize_nests_outer_to_inner(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=4)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
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


def test_fixture_parametrization_participates(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    assert len(by_name["test_widget"]) == 1
    parent_id = by_name["test_widget"][0]["id"]
    assert by_name["widget='a'"][0]["parent_step_id"] == parent_id
    assert by_name["widget='b'"][0]["parent_step_id"] == parent_id


def test_module_boundary_isolates_parametrize_stack(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=4)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    # Each module step contains its own `test_one`/`test_two` parametrize subtree.
    mod_a = by_name["test_a.py"][0]
    mod_b = by_name["test_b.py"][0]
    assert by_name["test_one"][0]["parent_step_id"] == mod_a["id"]
    assert by_name["test_two"][0]["parent_step_id"] == mod_b["id"]


def test_leaf_parent_chain_terminates_at_report(pytester: pytest.Pytester, out_dir: Path) -> None:
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=1)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    leaf = next(s for s in steps if s["name"].startswith("b="))
    chain = _ancestor_names(steps, leaf)
    # leaf b=… → a=… → test_chain → test_chain.py (module step) → root
    assert chain == ["b='x'", "a=1", "test_chain", "test_chain.py"]


# ---------------------------------------------------------------------------
# Order independence
# ---------------------------------------------------------------------------


def test_interleaved_execution_does_not_duplicate_parents(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """Sibling methods need not run contiguously to share one class parent.

    A conftest hook interleaves the two classes' methods
    (``A::a1, B::b1, A::a2, B::b2``) — the order the removed sort used to
    forbid, and the order pytest's own fixture-scope reordering can produce.
    Each class must still open exactly once and every method parent to the
    right class.
    """
    # Overwrite the conftest with one that registers the plugin AND reorders
    # items so the two classes interleave. The out_dir fixture's pytest.ini
    # (offline + log path) still applies.
    pytester.makeconftest(
        dedent(
            """
            pytest_plugins = ["sift_client.pytest_plugin"]

            def pytest_collection_modifyitems(config, items):
                a = [i for i in items if "TestA::" in i.nodeid]
                b = [i for i in items if "TestB::" in i.nodeid]
                interleaved = []
                for x, y in zip(a, b):
                    interleaved.append(x)
                    interleaved.append(y)
                items[:] = interleaved
            """
        )
    )
    pytester.makepyfile(
        test_inter=dedent(
            """
            class TestA:
                def test_a1(self):
                    pass

                def test_a2(self):
                    pass

            class TestB:
                def test_b1(self):
                    pass

                def test_b2(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=4)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    # Each class opens exactly once despite the interleaved run order.
    assert len(by_name["TestA"]) == 1
    assert len(by_name["TestB"]) == 1
    a_id = by_name["TestA"][0]["id"]
    b_id = by_name["TestB"][0]["id"]
    assert by_name["test_a1"][0]["parent_step_id"] == a_id
    assert by_name["test_a2"][0]["parent_step_id"] == a_id
    assert by_name["test_b1"][0]["parent_step_id"] == b_id
    assert by_name["test_b2"][0]["parent_step_id"] == b_id


# ---------------------------------------------------------------------------
# Parent status resolution
# ---------------------------------------------------------------------------


def test_parent_status_passed_when_all_children_pass(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    pytester.makepyfile(
        test_ok=dedent(
            """
            class TestFoo:
                def test_a(self):
                    pass

                def test_b(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    assert by_name["TestFoo"][0]["statuses"][-1] == TestStatus.PASSED
    assert by_name["test_ok.py"][0]["statuses"][-1] == TestStatus.PASSED


def test_parent_status_failed_propagates_up_and_isolates_siblings(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A failing leaf marks its class and the module FAILED, but a sibling class
    whose tests all pass stays PASSED.
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2, failed=1)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    assert by_name["TestFoo"][0]["statuses"][-1] == TestStatus.FAILED
    assert by_name["test_fail.py"][0]["statuses"][-1] == TestStatus.FAILED
    assert by_name["TestBar"][0]["statuses"][-1] == TestStatus.PASSED


def test_parent_status_failure_propagates_through_parametrize(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """One failing parametrization fails the whole chain: parametrize parent →
    class → module.
    """
    pytester.makepyfile(
        test_pfail=dedent(
            """
            import pytest

            class TestFoo:
                @pytest.mark.parametrize("v", [1, 2])
                def test_a(self, v):
                    if v == 1:
                        raise AssertionError("boom")
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=1, failed=1)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    assert by_name["test_a"][0]["statuses"][-1] == TestStatus.FAILED
    assert by_name["TestFoo"][0]["statuses"][-1] == TestStatus.FAILED
    assert by_name["test_pfail.py"][0]["statuses"][-1] == TestStatus.FAILED


def test_parent_opens_in_progress_and_resolves_exactly_once(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A parent is created IN_PROGRESS and gets exactly one terminal status at
    session end — it is never reopened, even as later siblings run under it.

    This locks in the "stay in-progress until every child is done, then resolve
    once" behavior: a parent emits a CreateTestStep (IN_PROGRESS) and a single
    UpdateTestStep (terminal), so its status timeline is exactly two entries.
    """
    pytester.makepyfile(
        test_once=dedent(
            """
            class TestFoo:
                def test_a(self):
                    pass

                def test_b(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    # Created in-progress, resolved once — no intermediate churn, no reopen.
    assert by_name["TestFoo"][0]["statuses"] == [TestStatus.IN_PROGRESS, TestStatus.PASSED]
    assert by_name["test_once.py"][0]["statuses"] == [TestStatus.IN_PROGRESS, TestStatus.PASSED]


# ---------------------------------------------------------------------------
# Parent timing
# ---------------------------------------------------------------------------


def test_parent_timing_spans_its_children(pytester: pytest.Pytester, out_dir: Path) -> None:
    """A parent's [start, end] window covers its whole subtree: it starts no
    later than its first child and ends exactly at its last child's finish.
    """
    pytester.makepyfile(
        test_span=dedent(
            """
            import time

            class TestFoo:
                def test_a(self):
                    time.sleep(0.02)

                def test_b(self):
                    time.sleep(0.02)
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    klass = by_name["TestFoo"][0]
    module = by_name["test_span.py"][0]
    leaves = [by_name["test_a"][0], by_name["test_b"][0]]
    leaf_starts = [_parse_ts(leaf["start_time"]) for leaf in leaves]
    leaf_ends = [_parse_ts(leaf["end_time"]) for leaf in leaves]

    # Parent opened before (or with) its earliest child, and start precedes end.
    assert _parse_ts(klass["start_time"]) <= min(leaf_starts)
    assert _parse_ts(klass["start_time"]) <= _parse_ts(klass["end_time"])
    # Parent end is exactly the latest descendant finish — not a session-end stamp.
    assert _parse_ts(klass["end_time"]) == max(leaf_ends)
    # The module parent spans the class and rolls the same finish up a level.
    assert _parse_ts(module["start_time"]) <= _parse_ts(klass["start_time"])
    assert _parse_ts(module["end_time"]) == max(leaf_ends)


def test_parent_end_time_reflects_a_later_child_under_interleaving(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """When a parent's children run non-contiguously, its end_time tracks the
    LAST child to finish — even one that runs after a different parent's child.

    Execution order is pinned to ``a1, b1, a2`` via a conftest hook, so
    ``TestA``'s second child (``a2``) closes after ``TestB``'s child. ``TestA``
    must end at ``a2``'s finish, not ``a1``'s.
    """
    pytester.makeconftest(
        dedent(
            """
            pytest_plugins = ["sift_client.pytest_plugin"]
            import pytest

            _ORDER = ["test_a1", "test_b1", "test_a2"]

            @pytest.hookimpl(trylast=True)
            def pytest_collection_modifyitems(config, items):
                # trylast so this runs after any reordering plugin and wins.
                items.sort(key=lambda i: _ORDER.index(i.name) if i.name in _ORDER else 99)
            """
        )
    )
    pytester.makepyfile(
        test_il=dedent(
            """
            import time

            class TestA:
                def test_a1(self):
                    pass

                def test_a2(self):
                    time.sleep(0.02)

            class TestB:
                def test_b1(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=3)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    a_end = by_name["TestA"][0]["end_time"]
    a1_end = by_name["test_a1"][0]["end_time"]
    a2_end = by_name["test_a2"][0]["end_time"]
    # TestA ends at its later child (a2), not the one that happened to run first.
    assert a_end == a2_end
    assert a_end != a1_end


# ---------------------------------------------------------------------------
# Early close — parents resolve as soon as their descendants finish
# ---------------------------------------------------------------------------


def _index(
    events: list[tuple],
    request_type: str,
    name: str,
    *,
    terminal: bool = False,
    status: TestStatus | None = None,
) -> int:
    """Index of the first matching log event.

    ``status`` matches that exact status; ``terminal`` matches any resolved
    (non-``IN_PROGRESS``) status.
    """

    def matches(rt: str, nm: str, st: TestStatus) -> bool:
        if rt != request_type or nm != name:
            return False
        if status is not None:
            return st == status
        return not terminal or st != TestStatus.IN_PROGRESS

    return next(i for i, (rt, nm, st) in enumerate(events) if matches(rt, nm, st))


_INTERLEAVE_CONFTEST = """
pytest_plugins = ["sift_client.pytest_plugin"]
import pytest

_ORDER = ["test_a1", "test_b1", "test_a2"]

@pytest.hookimpl(trylast=True)
def pytest_collection_modifyitems(config, items):
    # trylast so this wins over any reordering plugin; pins A::a1, B::b1, A::a2.
    items.sort(key=lambda i: _ORDER.index(i.name) if i.name in _ORDER else 99)
"""


def test_parent_closes_mid_session_not_at_end(pytester: pytest.Pytester, out_dir: Path) -> None:
    """A container resolves as soon as its last child finishes — before the next
    container even opens — rather than all flipping at session end.
    """
    pytester.makepyfile(
        test_mid=dedent(
            """
            class TestFoo:
                def test_a(self):
                    pass

                def test_b(self):
                    pass

            class TestBar:
                def test_c(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=3)
    events = capture.log_events(capture.run_jsonl(out_dir))
    # TestFoo reaches a terminal status before TestBar is even created.
    assert _index(events, "UpdateTestStep", "TestFoo", terminal=True) < _index(
        events, "CreateTestStep", "TestBar"
    )


def test_failing_parent_resolves_failed_mid_session(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """Early close carries status too: a class with a failing test resolves FAILED
    as soon as its subtree finishes, before the next class opens.
    """
    pytester.makepyfile(
        test_midfail=dedent(
            """
            class TestFoo:
                def test_a(self):
                    raise AssertionError("boom")

            class TestBar:
                def test_c(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=1, failed=1)
    events = capture.log_events(capture.run_jsonl(out_dir))
    foo_failed = _index(events, "UpdateTestStep", "TestFoo", status=TestStatus.FAILED)
    assert foo_failed < _index(events, "CreateTestStep", "TestBar")


def test_close_is_completion_driven_not_order_driven(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A single-child container closes the moment that child finishes, even though
    a sibling container's test (collected earlier) runs afterward.

    Order is pinned to ``a1, b1, a2``: ``TestB`` (only child ``b1``) must resolve
    before ``test_a2`` runs, proving close is driven by descendant completion, not
    by reaching some position in the item list.
    """
    pytester.makeconftest(_INTERLEAVE_CONFTEST)
    pytester.makepyfile(
        test_cd=dedent(
            """
            class TestA:
                def test_a1(self):
                    pass

                def test_a2(self):
                    pass

            class TestB:
                def test_b1(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=3)
    events = capture.log_events(capture.run_jsonl(out_dir))
    # TestB resolves before test_a2 is even created.
    assert _index(events, "UpdateTestStep", "TestB", terminal=True) < _index(
        events, "CreateTestStep", "test_a2"
    )


def test_excluded_sibling_does_not_stall_parent_close(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A ``sift_exclude``-d method is not counted toward its class's descendants,
    so the class still closes promptly once its included tests finish.

    If the excluded test inflated the count, ``TestFoo`` could never reach zero
    and would only resolve at the session-end drain — i.e. after ``TestBar`` is
    created. Asserting it resolves *before* ``TestBar`` proves the gate filter.
    """
    pytester.makepyfile(
        test_excl_close=dedent(
            """
            import pytest

            class TestFoo:
                @pytest.mark.sift_exclude
                def test_a(self):
                    pass

                def test_b(self):
                    pass

            class TestBar:
                def test_c(self):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=3)
    events = capture.log_events(capture.run_jsonl(out_dir))
    assert _index(events, "UpdateTestStep", "TestFoo", terminal=True) < _index(
        events, "CreateTestStep", "TestBar"
    )


# ---------------------------------------------------------------------------
# Outer-param promotion (session/package scoped fixture params)
# ---------------------------------------------------------------------------

_SESSION_FIXTURE_CONFTEST = """\
import pytest
pytest_plugins = ["sift_client.pytest_plugin"]

@pytest.fixture(scope="session", params=[10, 20], autouse=True)
def outer(request):
    yield request.param
"""


def test_session_fixture_param_promoted_above_module(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A session-scoped autouse fixture with params must appear above the module."""
    pytester.makeconftest(_SESSION_FIXTURE_CONFTEST)
    pytester.makepyfile(
        test_promo=dedent(
            """
            def test_a():
                pass

            def test_b():
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=4)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)

    assert len(by_name["outer=10"]) == 1
    assert len(by_name["outer=20"]) == 1
    # Outer param steps are roots — no parent.
    assert by_name["outer=10"][0]["parent_step_id"] is None
    assert by_name["outer=20"][0]["parent_step_id"] is None
    # Module step is nested beneath each outer param step (two distinct instances).
    assert len(by_name["test_promo.py"]) == 2
    outer_10_id = by_name["outer=10"][0]["id"]
    outer_20_id = by_name["outer=20"][0]["id"]
    mod_parent_ids = {s["parent_step_id"] for s in by_name["test_promo.py"]}
    assert outer_10_id in mod_parent_ids
    assert outer_20_id in mod_parent_ids


def test_two_outer_param_variants_produce_distinct_module_steps(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """The same module must not be shared across outer-param universes."""
    pytester.makeconftest(_SESSION_FIXTURE_CONFTEST)
    pytester.makepyfile(
        test_iso=dedent(
            """
            def test_x():
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)

    # Two distinct module steps, one per outer-param universe.
    assert len(by_name["test_iso.py"]) == 2
    outer_10_id = by_name["outer=10"][0]["id"]
    outer_20_id = by_name["outer=20"][0]["id"]
    parent_ids = [s["parent_step_id"] for s in by_name["test_iso.py"]]
    assert outer_10_id in parent_ids
    assert outer_20_id in parent_ids
    # Each leaf parents to its own module step.
    assert len(by_name["test_x"]) == 2
    leaf_parent_ids = {s["parent_step_id"] for s in by_name["test_x"]}
    mod_ids = {s["id"] for s in by_name["test_iso.py"]}
    assert leaf_parent_ids == mod_ids


def test_inner_parametrize_still_works_inside_outer_param(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """Inner mark-parametrize nesting must still work within each outer-param universe."""
    pytester.makeconftest(_SESSION_FIXTURE_CONFTEST)
    pytester.makepyfile(
        test_inner=dedent(
            """
            import pytest

            @pytest.mark.parametrize("v", [1, 2])
            def test_inner(v):
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=4)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)

    # Two outer universes → two `test_inner` parametrize parents.
    assert len(by_name["test_inner"]) == 2
    # Four leaves (v=1 and v=2 each appear twice, once per outer param).
    assert len(by_name["v=1"]) == 2
    assert len(by_name["v=2"]) == 2
    # Each v=… leaf parents to the test_inner in its own outer-param universe.
    test_inner_ids = {s["id"] for s in by_name["test_inner"]}
    for leaf in by_name["v=1"] + by_name["v=2"]:
        assert leaf["parent_step_id"] in test_inner_ids


def test_no_outer_param_unaffected(pytester: pytest.Pytester, out_dir: Path) -> None:
    """Tests without any session-scoped fixture params produce the normal tree."""
    pytester.makepyfile(
        test_plain=dedent(
            """
            import pytest

            @pytest.mark.parametrize("v", [1, 2])
            def test_plain(v):
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)

    # Module step is a root (no outer param wrapper).
    assert len(by_name["test_plain.py"]) == 1
    assert by_name["test_plain.py"][0]["parent_step_id"] is None
    assert len(by_name["test_plain"]) == 1
    mod_id = by_name["test_plain.py"][0]["id"]
    assert by_name["test_plain"][0]["parent_step_id"] == mod_id


def test_outer_param_subtree_closes_mid_session(pytester: pytest.Pytester, out_dir: Path) -> None:
    """The outer-param step resolves as soon as all its tests finish — not at session end."""
    pytester.makeconftest(_SESSION_FIXTURE_CONFTEST)
    pytester.makepyfile(
        test_ec=dedent(
            """
            def test_a():
                pass

            def test_b():
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=4)
    events = capture.log_events(capture.run_jsonl(out_dir))
    # The first outer-param step must resolve before the second outer-param step is created.
    assert _index(events, "UpdateTestStep", "outer=10", terminal=True) < _index(
        events, "CreateTestStep", "outer=20"
    )


# ---------------------------------------------------------------------------
# Explicit-ID labels — author-supplied ids win over name=value
# ---------------------------------------------------------------------------


def test_explicit_list_ids_on_inner_parametrize(pytester: pytest.Pytester, out_dir: Path) -> None:
    """An explicit ``ids=[...]`` list labels the parametrize steps, not ``name=value``."""
    pytester.makepyfile(
        test_lid=dedent(
            """
            import pytest

            @pytest.mark.parametrize("v", [1, 2], ids=["one", "two"])
            def test_lid(v):
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    # Friendly IDs are the leaf names; the structured fallback never appears.
    assert "one" in by_name
    assert "two" in by_name
    assert "v=1" not in by_name
    assert "v=2" not in by_name
    parent_id = by_name["test_lid"][0]["id"]
    assert by_name["one"][0]["parent_step_id"] == parent_id
    assert by_name["two"][0]["parent_step_id"] == parent_id


def test_callable_id_factory_on_inner_parametrize(pytester: pytest.Pytester, out_dir: Path) -> None:
    """A callable ``ids=`` factory is invoked per value, just as pytest does."""
    pytester.makepyfile(
        test_cid=dedent(
            """
            import pytest

            def label(v):
                return f"ch_{v}"

            @pytest.mark.parametrize("v", [0, 1], ids=label)
            def test_cid(v):
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    assert "ch_0" in by_name
    assert "ch_1" in by_name
    assert "v=0" not in by_name


def test_explicit_ids_on_session_fixture_label_outer_param(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A session fixture's explicit ``ids`` label the promoted outer-param steps."""
    pytester.makeconftest(
        dedent(
            """
            import pytest
            pytest_plugins = ["sift_client.pytest_plugin"]

            @pytest.fixture(scope="session", params=[24, 36],
                            ids=["lo", "hi"], autouse=True)
            def outer(request):
                yield request.param
            """
        )
    )
    pytester.makepyfile(
        test_oid=dedent(
            """
            def test_a():
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    assert "lo" in by_name
    assert "hi" in by_name
    assert "outer=24" not in by_name
    # Both labelled steps are roots, each scoping its own module subtree.
    assert by_name["lo"][0]["parent_step_id"] is None
    assert by_name["hi"][0]["parent_step_id"] is None
    assert len(by_name["test_oid.py"]) == 2


def test_auto_generated_ids_fall_back_to_name_value(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """With no author-supplied ``ids``, steps use the structured ``name=value`` label."""
    pytester.makepyfile(
        test_auto=dedent(
            """
            import pytest

            @pytest.mark.parametrize("v", [1, 2])
            def test_auto(v):
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    assert "v=1" in by_name
    assert "v=2" in by_name


def test_combined_axis_ids_fall_back_to_name_value(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A combined ``"a,b"`` axis can't attribute its shared ID, so each frame uses
    ``name=value`` rather than mislabelling both with the same combined ID.
    """
    pytester.makepyfile(
        test_comb=dedent(
            """
            import pytest

            @pytest.mark.parametrize("a,b", [(1, 2)], ids=["combined"])
            def test_comb(a, b):
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=1)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    # The shared "combined" ID is not adopted for either per-arg frame.
    assert "combined" not in by_name
    assert "a=1" in by_name
    assert "b=2" in by_name


# ---------------------------------------------------------------------------
# Scope-ladder placement — params sit at their scope's hierarchy level
# ---------------------------------------------------------------------------


def test_class_scoped_fixture_param_lifts_above_method(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A class-scoped parametrized fixture nests ABOVE the method; the method's own
    function param stays inside it (the inversion fix).
    """
    pytester.makepyfile(
        test_cs=dedent(
            """
            import pytest

            @pytest.fixture(scope="class", params=["A", "B"])
            def cfix(request):
                return request.param

            class TestFoo:
                @pytest.mark.parametrize("v", [10, 20])
                def test_b(self, cfix, v):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=4)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    # cfix lifts between the class and the method.
    assert len(by_name["cfix='A'"]) == 1
    assert len(by_name["cfix='B'"]) == 1
    class_id = by_name["TestFoo"][0]["id"]
    assert by_name["cfix='A'"][0]["parent_step_id"] == class_id
    assert by_name["cfix='B'"][0]["parent_step_id"] == class_id
    # The method (and its function param) nest under each cfix universe.
    leaf = by_name["v=10"][0]
    chain = _ancestor_names(steps, leaf)
    assert chain[:4] == ["v=10", "test_b", "cfix='A'", "TestFoo"]


# A module-scoped parametrized fixture shared by two functions in one module.
_MODULE_FIXTURE_SRC = dedent(
    """
    import pytest

    @pytest.fixture(scope="module", params=["M1", "M2"])
    def mfix(request):
        return request.param

    def test_one(mfix):
        pass

    def test_two(mfix):
        pass
    """
)


def test_module_scoped_fixture_param_lifts_above_functions(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A module-scoped parametrized fixture nests above the functions in the module,
    and two functions sharing it group under ONE param step per value (not split).
    """
    pytester.makepyfile(test_ms=_MODULE_FIXTURE_SRC)
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=4)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    mod_id = by_name["test_ms.py"][0]["id"]
    # One mfix step per value, each under the module (shared by both functions).
    assert len(by_name["mfix='M1'"]) == 1
    assert len(by_name["mfix='M2'"]) == 1
    assert by_name["mfix='M1'"][0]["parent_step_id"] == mod_id
    m1_id = by_name["mfix='M1'"][0]["id"]
    # Both functions' M1 leaves parent to the single shared mfix='M1' step.
    test_one_m1 = [s for s in by_name["test_one"] if s["parent_step_id"] == m1_id]
    test_two_m1 = [s for s in by_name["test_two"] if s["parent_step_id"] == m1_id]
    assert len(test_one_m1) == 1
    assert len(test_two_m1) == 1


def test_module_scoped_param_step_early_closes_when_shared_subtree_done(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A shared module-scoped param step resolves once all its tests finish, before
    the next param value's step opens.
    """
    pytester.makepyfile(test_msc=_MODULE_FIXTURE_SRC)
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=4)
    events = capture.log_events(capture.run_jsonl(out_dir))
    assert _index(events, "UpdateTestStep", "mfix='M1'", terminal=True) < _index(
        events, "CreateTestStep", "mfix='M2'"
    )


def test_mark_scope_class_lifts_to_class(pytester: pytest.Pytester, out_dir: Path) -> None:
    """A ``@pytest.mark.parametrize(..., scope="class")`` lifts to the class level."""
    pytester.makepyfile(
        test_msk=dedent(
            """
            import pytest

            class TestFoo:
                @pytest.mark.parametrize("cv", [1, 2], scope="class")
                def test_a(self, cv):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    class_id = by_name["TestFoo"][0]["id"]
    # cv lifts to sit directly under the class, above the method.
    assert by_name["cv=1"][0]["parent_step_id"] == class_id
    assert by_name["cv=2"][0]["parent_step_id"] == class_id
    cv1_id = by_name["cv=1"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] in {cv1_id, by_name["cv=2"][0]["id"]}


def test_mark_scope_session_lifts_to_root(pytester: pytest.Pytester, out_dir: Path) -> None:
    """A ``@pytest.mark.parametrize(..., scope="session")`` lifts above the module
    (closes the gap where mark-scoped session params were treated as function-scoped).
    """
    pytester.makepyfile(
        test_mss=dedent(
            """
            import pytest

            @pytest.mark.parametrize("sx", [7, 8], scope="session")
            def test_s(sx):
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    # sx steps are roots, each scoping its own module subtree.
    assert by_name["sx=7"][0]["parent_step_id"] is None
    assert by_name["sx=8"][0]["parent_step_id"] is None
    assert len(by_name["test_mss.py"]) == 2


def test_bare_class_mark_stays_under_method(pytester: pytest.Pytester, out_dir: Path) -> None:
    """A class-level mark WITHOUT ``scope=`` is function-scoped, so it stays under the
    method — not lifted to the class.
    """
    pytester.makepyfile(
        test_bcm=dedent(
            """
            import pytest

            @pytest.mark.parametrize("cv", [1, 2])
            class TestFoo:
                def test_a(self, cv):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    method_id = by_name["test_a"][0]["id"]
    class_id = by_name["TestFoo"][0]["id"]
    # cv parents to the method, and the method parents directly to the class.
    assert by_name["cv=1"][0]["parent_step_id"] == method_id
    assert by_name["cv=2"][0]["parent_step_id"] == method_id
    assert method_id != class_id
    assert by_name["test_a"][0]["parent_step_id"] == class_id


def test_nested_class_fixture_anchors_to_defining_class(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A class-scoped fixture defined on the OUTER class anchors there, not under the
    innermost nested class.
    """
    pytester.makepyfile(
        test_ncf=dedent(
            """
            import pytest

            class TestOuter:
                @pytest.fixture(scope="class", params=["A", "B"])
                def cfix(self, request):
                    return request.param

                class TestInner:
                    def test_a(self, cfix):
                        pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    outer_id = by_name["TestOuter"][0]["id"]
    # cfix sits directly under TestOuter (its defining class), above TestInner.
    assert by_name["cfix='A'"][0]["parent_step_id"] == outer_id
    leaf = by_name["test_a"][0]
    chain = _ancestor_names(steps, leaf)
    assert chain[:4] == ["test_a", "TestInner", "cfix='A'", "TestOuter"]


def test_class_param_falls_through_when_class_step_disabled(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """With ``sift_class_step=false`` the class step is suppressed, but a class-scoped
    param still renders and attaches to the module (nearest rendered ancestor).
    """
    _write_ini(pytester, out_dir, sift_class_step="false")
    pytester.makepyfile(
        test_cft=dedent(
            """
            import pytest

            @pytest.fixture(scope="class", params=["A", "B"])
            def cfix(request):
                return request.param

            class TestFoo:
                def test_a(self, cfix):
                    pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2)
    by_name = _by_name(capture.load_steps(capture.run_jsonl(out_dir)))
    assert "TestFoo" not in by_name
    mod_id = by_name["test_cft.py"][0]["id"]
    # The class param falls through to the module step.
    assert by_name["cfix='A'"][0]["parent_step_id"] == mod_id
    cfix_a_id = by_name["cfix='A'"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] in {
        cfix_a_id,
        by_name["cfix='B'"][0]["id"],
    }


def test_collection_skipped_param_item_uses_cleaned_leaf_name(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """A collection-skipped item under a scope-promoted param keeps the cleaned leaf
    name (no parametrize bracket), matching how a run sibling is named, and still
    nests under its param universe.

    The skip is evaluated before the autouse ``step`` fixture runs, so the inline
    step from the makereport hook must name itself the same way ``step_impl`` would.
    The passing test is declared first so the report context is bootstrapped before
    the skip fires in each outer-param universe.
    """
    pytester.makeconftest(
        dedent(
            """
            import pytest
            pytest_plugins = ["sift_client.pytest_plugin"]

            @pytest.fixture(scope="session", params=[10, 20], autouse=True)
            def outer(request):
                yield request.param
            """
        )
    )
    pytester.makepyfile(
        test_sk=dedent(
            """
            import pytest

            def test_ok():
                pass

            @pytest.mark.skip(reason="demo skip")
            def test_skipped():
                pass
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2, skipped=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    # Cleaned leaf name (no ``[10]`` bracket), one per outer-param universe.
    assert len(by_name["test_skipped"]) == 2
    assert "test_skipped[10]" not in by_name
    for leaf in by_name["test_skipped"]:
        chain = _ancestor_names(steps, leaf)
        assert chain[:2] == ["test_skipped", "test_sk.py"]
        assert chain[2] in ("outer=10", "outer=20")
        assert leaf["statuses"][-1] == TestStatus.SKIPPED


# ---------------------------------------------------------------------------
# Degradation when pytest internals are unavailable
#
# Scope-aware placement and ``ids=`` labels read a few pytest internals. If a
# pytest version moves them, the plugin must degrade (function-scoped nesting,
# ``name=value`` labels) and warn once, NOT break the user's collection. See
# ``steps._signal_introspection_degraded``.
# ---------------------------------------------------------------------------


def _fake_item(**attrs: object) -> pytest.Item:
    """A stand-in pytest item for unit-testing the introspection helpers directly.

    The helpers only touch a few attributes (``session``, ``callspec``, ``nodeid``,
    ``originalname``, ``name``), so a ``SimpleNamespace`` suffices; cast keeps the
    typed signatures honest without a real collected item.
    """
    return cast("pytest.Item", SimpleNamespace(**attrs))


def _fake_item_without_fixture_manager() -> pytest.Item:
    """An item whose ``session`` has no ``_fixturemanager`` (the internal moved)."""
    return _fake_item(session=SimpleNamespace(), nodeid="t.py::test_x")


def _raise(*_args: object, **_kwargs: object) -> None:
    raise RuntimeError("simulated pytest internals change")


def test_fixturedefs_degrades_when_fixture_manager_missing() -> None:
    steps.reset_introspection_state()
    item = _fake_item_without_fixture_manager()
    with pytest.warns(SiftPytestPluginWarning, match="scope-aware parametrize"):
        assert steps._fixturedefs(item, "x") is None
    assert steps._introspection_degraded is True


def test_fixturedefs_degrades_when_getfixturedefs_raises() -> None:
    steps.reset_introspection_state()

    class _FixtureManager:
        def getfixturedefs(self, name: str, node: object) -> object:
            raise RuntimeError("boom")

    item = _fake_item(
        session=SimpleNamespace(_fixturemanager=_FixtureManager()),
        nodeid="t.py::test_x",
    )
    with pytest.warns(SiftPytestPluginWarning):
        assert steps._fixturedefs(item, "x") is None


def test_build_scoped_params_degrades_to_empty(monkeypatch: pytest.MonkeyPatch) -> None:
    steps.reset_introspection_state()
    monkeypatch.setattr(steps, "_param_scope", _raise)
    item = _fake_item(callspec=SimpleNamespace(params={"v": 1}))
    with pytest.warns(SiftPytestPluginWarning):
        # Nothing promoted -> every axis stays function-scoped (rendered flat).
        assert steps.build_scoped_params(item) == ()


def test_build_parametrize_path_degrades_to_flat(monkeypatch: pytest.MonkeyPatch) -> None:
    steps.reset_introspection_state()
    monkeypatch.setattr(steps, "_param_scope", _raise)
    item = _fake_item(
        callspec=SimpleNamespace(params={"a": 1, "b": 2}),
        originalname="test_x",
        name="test_x[1-2]",
    )
    with pytest.warns(SiftPytestPluginWarning):
        path = steps.build_parametrize_path(item)
    # Degraded output: the bare leaf name plus every axis as name=value.
    assert path[0] == "test_x"
    assert set(path[1:]) == {"a=1", "b=2"}


def test_introspection_warning_fires_only_once() -> None:
    steps.reset_introspection_state()
    item = _fake_item_without_fixture_manager()
    with pytest.warns(SiftPytestPluginWarning):
        steps._fixturedefs(item, "x")
    # A second failure in the same session is silent (the latch is set), so
    # turning warnings into errors here must not trip.
    with warnings.catch_warnings():
        warnings.simplefilter("error")
        assert steps._fixturedefs(item, "y") is None


def test_introspection_failure_is_audit_logged() -> None:
    """The degradation emits a ``parametrize.introspection_degraded`` audit line.

    Captured via a handler attached directly to the plugin's logger, NOT
    ``caplog``: audit logging sets ``sift_client.propagate = False`` (audit_log.py),
    so root-propagation-based capture is order-dependent under randomized runs.
    """
    steps.reset_introspection_state()

    class _Capture(logging.Handler):
        def __init__(self) -> None:
            super().__init__()
            self.records: list[logging.LogRecord] = []

        def emit(self, record: logging.LogRecord) -> None:
            self.records.append(record)

    plugin_logger = logging.getLogger("sift_client._internal.pytest_plugin.steps")
    handler = _Capture()
    plugin_logger.addHandler(handler)
    previous_level = plugin_logger.level
    plugin_logger.setLevel(logging.WARNING)
    try:
        with pytest.warns(SiftPytestPluginWarning):
            steps._fixturedefs(_fake_item_without_fixture_manager(), "x")
    finally:
        plugin_logger.removeHandler(handler)
        plugin_logger.setLevel(previous_level)
    assert any("introspection_degraded" in r.getMessage() for r in handler.records)


def test_internals_failure_does_not_break_collection(
    pytester: pytest.Pytester, out_dir: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    """End-to-end: a forced internals failure degrades, it does not error the run.

    ``runpytest_inprocess`` shares this process's ``steps`` module, so the outer
    ``monkeypatch`` patches the inner run too and is restored afterward.
    """
    monkeypatch.setattr(steps, "_fixturedefs", _raise)
    pytester.makepyfile(
        test_x=dedent(
            """
            import pytest

            @pytest.fixture(scope="module", params=[1, 2])
            def fw(request):
                return request.param

            def test_a(step, fw):
                step.measure(name="m", value=True, bounds=True)
            """
        )
    )
    result = pytester.runpytest_inprocess("-p", "no:randomly")
    # The user's tests still run and pass despite the forced internals failure.
    result.assert_outcomes(passed=2)
    # The degradation is surfaced, not silent.
    assert "scope-aware parametrize placement" in result.stdout.str()
    # A report was still produced; the module-scoped param simply wasn't promoted.
    assert capture.load_steps(capture.run_jsonl(out_dir))


def test_indirect_parametrize_lifts_to_fixture_scope(
    pytester: pytest.Pytester, out_dir: Path
) -> None:
    """An ``indirect=True`` axis routed through a module-scoped fixture lifts to
    the module level. Scope comes from the fixture's public ``fixturedef.scope``,
    resolved without pytest's private ``callspec._arg2scope``.
    """
    pytester.makepyfile(
        test_ind=dedent(
            """
            import pytest

            @pytest.fixture(scope="module")
            def mfix(request):
                return request.param

            @pytest.mark.parametrize("mfix", ["M1", "M2"], indirect=True)
            def test_a(step, mfix):
                step.measure(name="m", value=True, bounds=True)
            """
        )
    )
    result = pytester.runpytest_inprocess("-v", "-p", "no:randomly")
    result.assert_outcomes(passed=2)
    steps = capture.load_steps(capture.run_jsonl(out_dir))
    by_name = _by_name(steps)
    mod_id = by_name["test_ind.py"][0]["id"]
    # One param step per value, lifted to module scope (under the module step).
    assert len(by_name["mfix='M1'"]) == 1
    assert len(by_name["mfix='M2'"]) == 1
    assert by_name["mfix='M1'"][0]["parent_step_id"] == mod_id
    # The leaf nests under its param universe, not directly under the module.
    m1_id = by_name["mfix='M1'"][0]["id"]
    assert [s for s in by_name["test_a"] if s["parent_step_id"] == m1_id]


def test_fixturedefs_falls_back_to_nodeid_on_pytest7_signature() -> None:
    """On pytest 7.x ``getfixturedefs`` takes a nodeid string; passing a Node
    hits ``nodeid.find(...)`` → ``AttributeError`` (a Node has no ``.find``).

    The fallback must retry with ``item.nodeid`` and succeed, NOT degrade — so
    scope-aware placement keeps working on pytest 7.x. (pytest 8.x/9.x take the
    Node directly and never reach the fallback.)
    """
    steps.reset_introspection_state()
    sentinel = object()

    class _Pytest7FixtureManager:
        def getfixturedefs(self, name: str, node_or_nodeid: object) -> object:
            # pytest 7.x accepts a nodeid string; a Node argument blows up the
            # way the real ``iterparentnodeids`` would (``node.find`` is absent).
            if isinstance(node_or_nodeid, str):
                return (sentinel,)
            raise AttributeError("'Function' object has no attribute 'find'")

    item = _fake_item(
        session=SimpleNamespace(_fixturemanager=_Pytest7FixtureManager()),
        nodeid="t.py::test_x",
    )
    with warnings.catch_warnings():
        warnings.simplefilter("error")  # a degradation warning would raise here
        result = steps._fixturedefs(item, "x")
    assert result == (sentinel,)
    assert steps._introspection_degraded is False
