"""Contract suite: maps each pytest exit path to the ``TestStatus`` the
Sift pytest plugin is required to record on the outer step.

Each scenario writes a tiny inner test file and runs it through pytester
with a fake ``sift_client`` injected via a generated conftest. The fake
records every step status write into ``_step_status_capture.CAPTURED_STEPS``
so this outer test can assert on what the plugin produced.

Assertions encode the contract from
``docs/guides/pytest_plugin/pass_fail_behavior.md``. Tests for scenarios the
plugin does not yet handle correctly are expected to **fail today** — they
are the punch list. ``lib/sift_client/_tests/pytest_plugin/step_status_states.md``
tracks each scenario's observed-today behavior next to the target so the
remaining gaps are visible without running the suite.
"""

from __future__ import annotations

import textwrap

import pytest

from sift_client._tests.pytest_plugin import _step_status_capture as capture
from sift_client.sift_types.test_report import TestStatus

pytest_plugins = ["pytester"]


_INNER_CONFTEST_SRC = '''
"""Auto-generated conftest. Loading the Sift plugin is the only thing the
inner session needs. ``--sift-offline`` on the CLI causes the plugin's
default ``sift_client`` fixture to construct a placeholder client and the
real ``ReportContext`` writes every API call to the JSONL log without
contacting Sift.
"""

pytest_plugins = ["sift_client.pytest_plugin"]
'''


@pytest.fixture
def inner(pytester):
    """Install the inner conftest. Returns ``pytester``."""
    pytester.makeconftest(_INNER_CONFTEST_SRC)
    return pytester


# Prepended to every inner test file. Pytest skips marker-based ``skip`` items
# before any autouse fixture runs, which would leave ``REPORT_CONTEXT`` unset
# and the plugin's inline-skip recording inert. A single passing item up-front
# forces ``report_context`` to initialize so the makereport hook can record
# the skip into the same session's JSONL.
_WARMUP = "def test_sift_warmup(): pass\n\n"


def _run(pytester, body: str) -> None:
    pytester.makepyfile(_WARMUP + textwrap.dedent(body))
    log_path = pytester.path / "sift.log"
    capture.set_log(log_path)
    pytester.runpytest_inprocess(
        "--sift-offline",
        f"--sift-log-file={log_path}",
        "--no-sift-git-metadata",
    )


# ---------------------------------------------------------------------------
# Call-phase exit paths
# ---------------------------------------------------------------------------


def test_pass_maps_to_passed(inner):
    # Case: CALL-01
    _run(
        inner,
        """
        def test_x():
            assert True
        """,
    )
    assert capture.final_status("test_x") == TestStatus.PASSED


def test_assert_failure_maps_to_failed(inner):
    # Case: CALL-02
    _run(
        inner,
        """
        def test_x():
            assert 1 == 2
        """,
    )
    assert capture.final_status("test_x") == TestStatus.FAILED


def test_generic_exception_maps_to_error(inner):
    # Case: CALL-03
    _run(
        inner,
        """
        def test_x():
            raise ValueError("boom")
        """,
    )
    assert capture.final_status("test_x") == TestStatus.ERROR


def test_system_exit_maps_to_aborted(inner):
    # Case: CALL-05
    _run(
        inner,
        """
        import sys
        def test_x():
            sys.exit(1)
        """,
    )
    assert capture.final_status("test_x") == TestStatus.ABORTED


def test_pytest_fail_maps_to_failed(inner):
    # Case: CALL-04
    _run(
        inner,
        """
        import pytest
        def test_x():
            pytest.fail("intentional failure")
        """,
    )
    assert capture.final_status("test_x") == TestStatus.FAILED


def test_keyboard_interrupt_leaves_step_in_progress(inner):
    # Case: CALL-06
    # KeyboardInterrupt aborts the session before the call-phase makereport
    # fires; the plugin can't observe the interrupt. The contract is that
    # the step is left in IN_PROGRESS rather than being silently resolved
    # to PASSED — a session-aborting interrupt should not look like a clean
    # pass in the report.
    try:
        _run(
            inner,
            """
            def test_x():
                raise KeyboardInterrupt
            """,
        )
    except KeyboardInterrupt:
        pass
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.IN_PROGRESS


# ---------------------------------------------------------------------------
# Skip paths
# ---------------------------------------------------------------------------


def test_pytest_skip_in_body_maps_to_skipped(inner):
    # Case: SKIP-03
    _run(
        inner,
        """
        import pytest
        def test_x():
            pytest.skip("not today")
        """,
    )
    # Runtime skip in the body resolves the outer step to SKIPPED. The
    # makereport hook must not create a duplicate nested step.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.SKIPPED
    duplicates = [s for s in capture.steps_by_name("test_x") if s is not outer]
    assert not duplicates, f"expected no duplicate nested step; got {len(duplicates)}"


def test_pytest_mark_skip_records_skipped(inner):
    # Case: SKIP-01
    _run(
        inner,
        """
        import pytest
        @pytest.mark.skip(reason="collection-time skip")
        def test_x():
            assert False
        """,
    )
    # Collection-time skip: the autouse step fixture never runs. Only the
    # makereport hook creates a step, with status SKIPPED.
    assert capture.final_status("test_x") == TestStatus.SKIPPED


def test_pytest_mark_skipif_records_skipped(inner):
    # Case: SKIP-02
    _run(
        inner,
        """
        import pytest
        @pytest.mark.skipif(True, reason="conditional skip")
        def test_x():
            assert False
        """,
    )
    # `skipif` with a truthy condition follows the same path as
    # `@pytest.mark.skip`; only the makereport hook records a step.
    assert capture.final_status("test_x") == TestStatus.SKIPPED


def test_skip_inside_fixture_setup(inner):
    # Case: SKIP-04
    _run(
        inner,
        """
        import pytest

        @pytest.fixture
        def skipping_fixture():
            pytest.skip("environment not ready")

        def test_x(skipping_fixture):
            assert True
        """,
    )
    # A setup-phase skip resolves the outer step to SKIPPED. The makereport
    # hook must not create a duplicate nested step.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.SKIPPED
    duplicates = [s for s in capture.steps_by_name("test_x") if s is not outer]
    assert not duplicates, f"expected no duplicate nested step; got {len(duplicates)}"


# ---------------------------------------------------------------------------
# xfail / xpass
# ---------------------------------------------------------------------------


def test_xfail_marked_test_that_fails(inner):
    # Case: XFAIL-01
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(reason="known issue")
        def test_x():
            assert 1 == 2
        """,
    )
    # xfail + expected failure fulfills the contract; outer step resolves to
    # PASSED. No duplicate nested step from the makereport hook.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.PASSED
    duplicates = [s for s in capture.steps_by_name("test_x") if s is not outer]
    assert not duplicates, f"expected no duplicate nested step; got {len(duplicates)}"


def test_xfail_strict_unexpected_pass(inner):
    # Case: XFAIL-02
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(strict=True, reason="should fail")
        def test_x():
            assert True
        """,
    )
    # strict xfail that passes must surface as FAILED: either the bug was
    # fixed (remove the mark) or the test stopped exercising what it claimed.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.FAILED


def test_xfail_non_strict_unexpected_pass(inner):
    # Case: XFAIL-03
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(reason="might pass sometimes")
        def test_x():
            assert True
        """,
    )
    # Non-strict xfail does not insist on the failure, so a passing run is
    # PASSED.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.PASSED


def test_xfail_raises_mismatch(inner):
    # Case: XFAIL-04
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(raises=ValueError, reason="expected ValueError")
        def test_x():
            raise KeyError("wrong exception")
        """,
    )
    # `raises=` mismatch is a real test failure — the contract required a
    # specific exception type and a different one was thrown.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.FAILED


def test_xfail_run_false(inner):
    # Case: XFAIL-05
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(run=False, reason="never run")
        def test_x():
            assert False
        """,
    )
    # The test never ran; outer step is SKIPPED.
    assert capture.final_status("test_x") == TestStatus.SKIPPED


# ---------------------------------------------------------------------------
# Setup-phase / teardown-phase fixture failures
# ---------------------------------------------------------------------------


def test_setup_phase_fixture_failure(inner):
    # Case: PHASE-01
    _run(
        inner,
        """
        import pytest

        @pytest.fixture
        def bad_setup():
            raise RuntimeError("setup boom")

        def test_x(bad_setup):
            assert True
        """,
    )
    # A fixture that raises before `yield` fails the setup phase. The outer
    # step must surface this as ERROR; the test body never executed and a
    # silently green step would hide the failure.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.ERROR


def test_teardown_phase_fixture_failure(inner):
    # Case: PHASE-02
    _run(
        inner,
        """
        import pytest

        @pytest.fixture
        def bad_teardown():
            yield
            raise RuntimeError("teardown boom")

        def test_x(bad_teardown):
            assert True
        """,
    )
    # A fixture that raises after `yield` fails the teardown phase. The
    # outer step's status reflects the teardown failure as FAILED rather
    # than the call-phase pass.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.FAILED


def test_call_fail_plus_teardown_fail(inner):
    # Case: PHASE-03
    _run(
        inner,
        """
        import pytest

        @pytest.fixture
        def bad_teardown():
            yield
            raise RuntimeError("teardown boom")

        def test_x(bad_teardown):
            assert 1 == 2
        """,
    )
    # Call-phase failure dominates the outer step status; the contract also
    # requires the teardown error to be surfaced somewhere on the step
    # (mechanism TBD — see pass_fail_behavior.md). This test asserts the
    # status today; tighten once a surfacing mechanism is chosen.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.FAILED


# ---------------------------------------------------------------------------
# Collection-phase failures
# ---------------------------------------------------------------------------


def test_missing_fixture_maps_to_error(inner):
    # Case: COLL-01
    _run(
        inner,
        """
        def test_x(nonexistent_fixture):
            assert True
        """,
    )
    # An unresolved fixture is a setup-phase failure. The outer step
    # surfaces as ERROR rather than a misleading green pass for a test
    # that never executed.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.ERROR


# ---------------------------------------------------------------------------
# Plugin-API exit paths (in-test mutations)
# ---------------------------------------------------------------------------


def test_manual_status_update_to_failed(inner):
    # Case: API-01
    _run(
        inner,
        """
        from sift_client.sift_types.test_report import TestStatus
        def test_x(step):
            step.current_step.update({"status": TestStatus.FAILED})
        """,
    )
    assert capture.final_status("test_x") == TestStatus.FAILED


def test_report_outcome_false_maps_to_failed(inner):
    # Case: API-02
    _run(
        inner,
        """
        def test_x(step):
            step.report_outcome("the_check", False, "did not match")
        """,
    )
    # Outer step sees a failed substep and rolls up to FAILED.
    assert capture.final_status("test_x") == TestStatus.FAILED


def test_measure_out_of_bounds_maps_to_failed(inner):
    # Case: API-03
    _run(
        inner,
        """
        def test_x(step):
            step.measure(name="m", value=10.0, bounds={"min": 0.0, "max": 5.0})
        """,
    )
    assert capture.final_status("test_x") == TestStatus.FAILED


def test_substep_failure_propagates_to_parent(inner):
    # Case: API-04
    _run(
        inner,
        """
        def test_x(step):
            with step.substep(name="inner") as inner_step:
                inner_step.measure(name="m", value=10.0, bounds={"min": 0.0, "max": 5.0})
        """,
    )
    # `test_measure_out_of_bounds_maps_to_failed` exercises a failed
    # measurement on the function step itself; this one verifies the same
    # failure on a nested substep propagates up to the parent.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.FAILED


def test_skipped_substep_does_not_fail_parent(inner):
    # Case: API-05
    _run(
        inner,
        """
        from sift_client.sift_types.test_report import TestStatus
        def test_x(step):
            with step.substep(name="optional_check") as cal:
                cal.current_step.update(
                    {"status": TestStatus.SKIPPED},
                    log_file=step.report_context.log_file,
                )
        """,
    )
    # A manually-resolved SKIPPED on a substep must not propagate as a failure
    # to the parent. The outer step has no measurements of its own and resolves
    # to PASSED.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.PASSED


def test_abort_inside_substep_marks_every_open_step_aborted(inner):
    # Case: API-06
    _run(
        inner,
        """
        import sys
        def test_x(step):
            with step.substep(name="completed_sub"):
                pass
            with step.substep(name="outer_sub") as outer_sub:
                with outer_sub.substep(name="inner_sub"):
                    sys.exit(1)
        """,
    )
    # SystemExit unwinds the substep stack on the way out. Every step that was
    # open when the abort fired (inner substep, outer substep, test step)
    # must record ABORTED. The sibling substep that closed cleanly before the
    # abort must retain its PASSED status.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.ABORTED
    outer_sub = next(iter(capture.steps_by_name("outer_sub")), None)
    inner_sub = next(iter(capture.steps_by_name("inner_sub")), None)
    completed_sub = next(iter(capture.steps_by_name("completed_sub")), None)
    assert outer_sub is not None
    assert inner_sub is not None
    assert completed_sub is not None
    assert outer_sub.statuses[-1] == TestStatus.ABORTED
    assert inner_sub.statuses[-1] == TestStatus.ABORTED
    assert completed_sub.statuses[-1] == TestStatus.PASSED
