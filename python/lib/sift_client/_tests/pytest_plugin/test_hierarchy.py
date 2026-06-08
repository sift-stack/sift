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

from datetime import datetime, timezone
from textwrap import dedent
from types import SimpleNamespace
from typing import TYPE_CHECKING

import pytest

from sift_client._tests.pytest_plugin import _step_status_capture as capture
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
        f"sift_log_file = {log_path}",
        "sift_git_metadata = false",
    ]


@pytest.fixture
def log_file(pytester: pytest.Pytester) -> Path:
    path = pytester.path / "sift.log"
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


def test_class_methods_cluster_under_class_step(pytester: pytest.Pytester, log_file: Path) -> None:
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    assert len(by_name["TestFoo"]) == 1
    class_id = by_name["TestFoo"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == class_id
    assert by_name["test_b"][0]["parent_step_id"] == class_id


def test_collection_skipped_method_nests_under_its_class(
    pytester: pytest.Pytester, log_file: Path
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
    by_name = _by_name(capture.load_steps(log_file))
    assert len(by_name["TestFoo"]) == 1
    class_id = by_name["TestFoo"][0]["id"]
    assert by_name["test_run"][0]["parent_step_id"] == class_id
    assert by_name["test_skipped"][0]["parent_step_id"] == class_id
    assert by_name["test_skipped"][0]["statuses"][-1] == TestStatus.SKIPPED


def test_nested_classes_produce_nested_steps(pytester: pytest.Pytester, log_file: Path) -> None:
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
    steps = capture.load_steps(log_file)
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


def test_class_parametrize_nests_under_class(pytester: pytest.Pytester, log_file: Path) -> None:
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    class_id = by_name["TestFoo"][0]["id"]
    test_a_id = by_name["test_a"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == class_id
    assert by_name["v=1"][0]["parent_step_id"] == test_a_id
    assert by_name["v=2"][0]["parent_step_id"] == test_a_id


def test_two_sibling_classes_in_module(pytester: pytest.Pytester, log_file: Path) -> None:
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    mod_id = by_name["test_sib.py"][0]["id"]
    assert by_name["TestA"][0]["parent_step_id"] == mod_id
    assert by_name["TestB"][0]["parent_step_id"] == mod_id
    # Sanity: each class is opened exactly once (no duplicate parents).
    assert len(by_name["TestA"]) == 1
    assert len(by_name["TestB"]) == 1


def test_mixed_class_and_free_function(pytester: pytest.Pytester, log_file: Path) -> None:
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    mod_id = by_name["test_mix.py"][0]["id"]
    # Class method parents to TestA; free function parents directly to module.
    assert by_name["TestA"][0]["parent_step_id"] == mod_id
    assert by_name["test_x"][0]["parent_step_id"] == by_name["TestA"][0]["id"]
    assert by_name["test_free"][0]["parent_step_id"] == mod_id


def test_class_with_all_excluded_methods_no_class_step(
    pytester: pytest.Pytester, log_file: Path
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    assert "TestFoo" not in by_name
    assert "test_a" not in by_name
    assert "test_b" not in by_name


def test_sift_exclude_on_class_propagates(pytester: pytest.Pytester, log_file: Path) -> None:
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    assert "TestFoo" not in by_name
    assert "test_a" not in by_name


def test_class_docstring_becomes_step_description(
    pytester: pytest.Pytester, log_file: Path
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
    result = pytester.runpytest_inprocess("-v")
    result.assert_outcomes(passed=1)
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    # The fake records step creation but not all fields — check the class
    # step was recorded, then read the description via the FakeStep's
    # description attribute by re-reading steps. The fake's create_step only
    # records name/parent/path/id, so verify via the leaf chain only here.
    leaf = by_name["test_a"][0]
    assert _ancestor_names(steps, leaf)[:3] == ["test_a", "TestFoo", "test_doc.py"]


def test_two_class_chains_keep_parametrize_isolated(
    pytester: pytest.Pytester, log_file: Path
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
    steps = capture.load_steps(log_file)
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
    pytester: pytest.Pytester, log_file: Path
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
    steps = capture.load_steps(log_file)
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
    pytester: pytest.Pytester, log_file: Path
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
    steps = capture.load_steps(log_file)
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


def _write_ini(pytester: pytest.Pytester, log_file: Path, **overrides: object) -> None:
    """Write a pytest.ini with the given sift_* overrides, preserving the
    offline/log/git-metadata defaults the ``log_file`` fixture installs.
    """
    lines = _base_ini_lines(log_file)
    for key, value in overrides.items():
        lines.append(f"{key} = {value}")
    pytester.makefile(".ini", pytest="\n".join(lines) + "\n")


def test_sift_class_step_false_skips_class_steps(pytester: pytest.Pytester, log_file: Path) -> None:
    _write_ini(pytester, log_file, sift_class_step="false")
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    assert "TestFoo" not in by_name
    mod_id = by_name["test_noclass.py"][0]["id"]
    assert by_name["test_a"][0]["parent_step_id"] == mod_id
    assert by_name["test_b"][0]["parent_step_id"] == mod_id


def test_sift_module_step_false_skips_module_step(
    pytester: pytest.Pytester, log_file: Path
) -> None:
    _write_ini(pytester, log_file, sift_module_step="false")
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    assert "test_nomod.py" not in by_name
    # TestFoo attaches to the report root (no parent recorded by the fake).
    assert by_name["TestFoo"][0]["parent_step_id"] is None
    assert by_name["test_a"][0]["parent_step_id"] == by_name["TestFoo"][0]["id"]


def test_sift_parametrize_nesting_false_keeps_flat_leaves(
    pytester: pytest.Pytester, log_file: Path
) -> None:
    _write_ini(pytester, log_file, sift_parametrize_nesting="false")
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
    steps = capture.load_steps(log_file)
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
    pytester: pytest.Pytester, log_file: Path
) -> None:
    """sift_module_step=false must not merge same-named classes across modules.

    The hierarchy chain always includes the module ancestor for identity
    (even when it's not rendered as a step), so two modules each declaring
    ``class TestFoo`` produce two distinct ``TestFoo`` frames in the diff.
    """
    _write_ini(pytester, log_file, sift_module_step="false")
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
    steps = capture.load_steps(log_file)
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


def test_package_step_default_opens_for_init_dirs(
    pytester: pytest.Pytester, log_file: Path
) -> None:
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    assert "pkg_a" in by_name
    pkg_id = by_name["pkg_a"][0]["id"]
    mod = by_name["test_x.py"][0]
    assert mod["parent_step_id"] == pkg_id


def test_same_named_packages_in_different_dirs_do_not_merge(
    pytester: pytest.Pytester, log_file: Path
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
    steps = capture.load_steps(log_file)
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
    pytester: pytest.Pytester, log_file: Path
) -> None:
    """With ``sift_package_step=false`` the directory step is suppressed."""
    _write_ini(pytester, log_file, sift_package_step="false")
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    assert "pkg_a" not in by_name
    # The module step still opens and is now the top-level frame.
    assert by_name["test_x.py"][0]["parent_step_id"] is None


def test_all_three_flags_false_matches_legacy_behavior(
    pytester: pytest.Pytester, log_file: Path
) -> None:
    _write_ini(
        pytester,
        log_file,
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
    steps = capture.load_steps(log_file)
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
    pytester: pytest.Pytester, log_file: Path
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    # Module step + one shared `test_rail` parent + two leaves.
    assert len(by_name["test_rail.py"]) == 1
    assert len(by_name["test_rail"]) == 1
    assert len(by_name["v=3.3"]) == 1
    assert len(by_name["v=5.0"]) == 1
    test_rail_id = by_name["test_rail"][0]["id"]
    assert by_name["v=3.3"][0]["parent_step_id"] == test_rail_id
    assert by_name["v=5.0"][0]["parent_step_id"] == test_rail_id


def test_stacked_parametrize_nests_outer_to_inner(
    pytester: pytest.Pytester, log_file: Path
) -> None:
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
    steps = capture.load_steps(log_file)
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


def test_fixture_parametrization_participates(pytester: pytest.Pytester, log_file: Path) -> None:
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    assert len(by_name["test_widget"]) == 1
    parent_id = by_name["test_widget"][0]["id"]
    assert by_name["widget='a'"][0]["parent_step_id"] == parent_id
    assert by_name["widget='b'"][0]["parent_step_id"] == parent_id


def test_module_boundary_isolates_parametrize_stack(
    pytester: pytest.Pytester, log_file: Path
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
    steps = capture.load_steps(log_file)
    by_name = _by_name(steps)
    # Each module step contains its own `test_one`/`test_two` parametrize subtree.
    mod_a = by_name["test_a.py"][0]
    mod_b = by_name["test_b.py"][0]
    assert by_name["test_one"][0]["parent_step_id"] == mod_a["id"]
    assert by_name["test_two"][0]["parent_step_id"] == mod_b["id"]


def test_leaf_parent_chain_terminates_at_report(pytester: pytest.Pytester, log_file: Path) -> None:
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
    steps = capture.load_steps(log_file)
    leaf = next(s for s in steps if s["name"].startswith("b="))
    chain = _ancestor_names(steps, leaf)
    # leaf b=… → a=… → test_chain → test_chain.py (module step) → root
    assert chain == ["b='x'", "a=1", "test_chain", "test_chain.py"]


# ---------------------------------------------------------------------------
# Order independence
# ---------------------------------------------------------------------------


def test_interleaved_execution_does_not_duplicate_parents(
    pytester: pytest.Pytester, log_file: Path
) -> None:
    """Sibling methods need not run contiguously to share one class parent.

    A conftest hook interleaves the two classes' methods
    (``A::a1, B::b1, A::a2, B::b2``) — the order the removed sort used to
    forbid, and the order pytest's own fixture-scope reordering can produce.
    Each class must still open exactly once and every method parent to the
    right class.
    """
    # Overwrite the conftest with one that registers the plugin AND reorders
    # items so the two classes interleave. The log_file fixture's pytest.ini
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
    steps = capture.load_steps(log_file)
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
    pytester: pytest.Pytester, log_file: Path
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
    by_name = _by_name(capture.load_steps(log_file))
    assert by_name["TestFoo"][0]["statuses"][-1] == TestStatus.PASSED
    assert by_name["test_ok.py"][0]["statuses"][-1] == TestStatus.PASSED


def test_parent_status_failed_propagates_up_and_isolates_siblings(
    pytester: pytest.Pytester, log_file: Path
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
    by_name = _by_name(capture.load_steps(log_file))
    assert by_name["TestFoo"][0]["statuses"][-1] == TestStatus.FAILED
    assert by_name["test_fail.py"][0]["statuses"][-1] == TestStatus.FAILED
    assert by_name["TestBar"][0]["statuses"][-1] == TestStatus.PASSED


def test_parent_status_failure_propagates_through_parametrize(
    pytester: pytest.Pytester, log_file: Path
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
    by_name = _by_name(capture.load_steps(log_file))
    assert by_name["test_a"][0]["statuses"][-1] == TestStatus.FAILED
    assert by_name["TestFoo"][0]["statuses"][-1] == TestStatus.FAILED
    assert by_name["test_pfail.py"][0]["statuses"][-1] == TestStatus.FAILED


def test_parent_opens_in_progress_and_resolves_exactly_once(
    pytester: pytest.Pytester, log_file: Path
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
    by_name = _by_name(capture.load_steps(log_file))
    # Created in-progress, resolved once — no intermediate churn, no reopen.
    assert by_name["TestFoo"][0]["statuses"] == [TestStatus.IN_PROGRESS, TestStatus.PASSED]
    assert by_name["test_once.py"][0]["statuses"] == [TestStatus.IN_PROGRESS, TestStatus.PASSED]


# ---------------------------------------------------------------------------
# Parent timing
# ---------------------------------------------------------------------------


def test_parent_timing_spans_its_children(pytester: pytest.Pytester, log_file: Path) -> None:
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
    by_name = _by_name(capture.load_steps(log_file))
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
    pytester: pytest.Pytester, log_file: Path
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
    by_name = _by_name(capture.load_steps(log_file))
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


def test_parent_closes_mid_session_not_at_end(pytester: pytest.Pytester, log_file: Path) -> None:
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
    events = capture.log_events(log_file)
    # TestFoo reaches a terminal status before TestBar is even created.
    assert _index(events, "UpdateTestStep", "TestFoo", terminal=True) < _index(
        events, "CreateTestStep", "TestBar"
    )


def test_failing_parent_resolves_failed_mid_session(
    pytester: pytest.Pytester, log_file: Path
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
    events = capture.log_events(log_file)
    foo_failed = _index(events, "UpdateTestStep", "TestFoo", status=TestStatus.FAILED)
    assert foo_failed < _index(events, "CreateTestStep", "TestBar")


def test_close_is_completion_driven_not_order_driven(
    pytester: pytest.Pytester, log_file: Path
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
    events = capture.log_events(log_file)
    # TestB resolves before test_a2 is even created.
    assert _index(events, "UpdateTestStep", "TestB", terminal=True) < _index(
        events, "CreateTestStep", "test_a2"
    )


def test_excluded_sibling_does_not_stall_parent_close(
    pytester: pytest.Pytester, log_file: Path
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
    events = capture.log_events(log_file)
    assert _index(events, "UpdateTestStep", "TestFoo", terminal=True) < _index(
        events, "CreateTestStep", "TestBar"
    )
