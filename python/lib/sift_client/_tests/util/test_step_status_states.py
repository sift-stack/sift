"""Characterization suite: maps each pytest exit path to the ``TestStatus``
the plugin currently records on the step.

Each scenario writes a tiny inner test file and runs it through pytester
with a fake ``sift_client`` injected via a generated conftest. The fake
records every step status write into ``_step_status_capture.CAPTURED_STEPS``
so this outer test can assert on what the plugin produced.

The expected statuses below reflect **current** behavior. Where current
behavior contradicts the audit target (setup-phase, teardown-phase, xfail),
the assertion is paired with an ``AUDIT:`` comment naming the target.
Updating these tests is the regression check once the fix lands.
"""

from __future__ import annotations

import textwrap

import pytest

from sift_client._tests.util import _step_status_capture as capture
from sift_client.sift_types.test_report import TestStatus

pytest_plugins = ["pytester"]


_INNER_CONFTEST_SRC = '''
"""Auto-generated conftest for the step-status characterization suite.

Installs the Sift pytest plugin and a fake ``sift_client`` that records
every step status write into the outer test's CAPTURED_STEPS dict.
"""

from __future__ import annotations

import uuid

import pytest

# Bring the Sift fixtures + the makereport hook into this inner session.
from sift_client.util.test_results import *  # noqa: F401,F403

from sift_client._tests.util._step_status_capture import CAPTURED_STEPS, CapturedStep
from sift_client.sift_types.test_report import (
    TestMeasurement,
    TestReport,
    TestStep,
)



class _FakeTestResults:
    def __init__(self, client):
        self._client = client

    def create(self, test_report, log_file=None):
        report = TestReport(
            id_=str(uuid.uuid4()),
            status=test_report.status,
            name=test_report.name,
            test_system_name=test_report.test_system_name,
            test_case=test_report.test_case,
            start_time=test_report.start_time,
            end_time=test_report.end_time,
            metadata=test_report.metadata or {},
            is_archived=False,
        )
        report._apply_client_to_instance(self._client)
        return report

    def update(self, test_report, update, log_file=None):
        return test_report

    def create_step(self, test_step, log_file=None):
        step_id = str(uuid.uuid4())
        CAPTURED_STEPS[step_id] = CapturedStep(
            step_id=step_id,
            name=test_step.name,
            step_path=test_step.step_path,
            parent_step_id=test_step.parent_step_id,
            statuses=[test_step.status],
        )
        step = TestStep(
            id_=step_id,
            test_report_id=test_step.test_report_id,
            parent_step_id=test_step.parent_step_id,
            name=test_step.name,
            description=test_step.description,
            step_type=test_step.step_type,
            step_path=test_step.step_path,
            status=test_step.status,
            start_time=test_step.start_time,
            end_time=test_step.end_time,
            error_info=test_step.error_info,
        )
        step._apply_client_to_instance(self._client)
        return step

    def update_step(self, test_step, update, log_file=None):
        new_status = (
            update.get("status") if isinstance(update, dict) else update.status
        )
        if test_step.id_ in CAPTURED_STEPS and new_status is not None:
            CAPTURED_STEPS[test_step.id_].statuses.append(new_status)
        merged_status = new_status if new_status is not None else test_step.status
        updated = TestStep(
            id_=test_step.id_,
            test_report_id=test_step.test_report_id,
            parent_step_id=test_step.parent_step_id,
            name=test_step.name,
            description=test_step.description,
            step_type=test_step.step_type,
            step_path=test_step.step_path,
            status=merged_status,
            start_time=test_step.start_time,
            end_time=test_step.end_time,
            error_info=test_step.error_info,
        )
        updated._apply_client_to_instance(self._client)
        return updated

    def create_measurement(self, test_measurement, update_step=False, log_file=None):
        measurement = TestMeasurement(
            id_=str(uuid.uuid4()),
            measurement_type=test_measurement.measurement_type,
            name=test_measurement.name,
            test_step_id=test_measurement.test_step_id,
            numeric_value=test_measurement.numeric_value,
            string_value=test_measurement.string_value,
            boolean_value=test_measurement.boolean_value,
            unit=test_measurement.unit,
            numeric_bounds=test_measurement.numeric_bounds,
            string_expected_value=test_measurement.string_expected_value,
            passed=test_measurement.passed,
            timestamp=test_measurement.timestamp,
        )
        measurement._apply_client_to_instance(self._client)
        return measurement


class _FakePing:
    def ping(self):
        return None


class _FakeSiftClient:
    def __init__(self):
        self.test_results = _FakeTestResults(self)
        self.ping = _FakePing()


@pytest.fixture(scope="session")
def sift_client():
    return _FakeSiftClient()
'''


_RUN_ARGS = (
    "--sift-test-results-log-file=false",
    "--no-sift-test-results-git-metadata",
)


@pytest.fixture
def inner(pytester):
    """Reset the capture state and install the inner conftest. Returns ``pytester``."""
    capture.reset()
    pytester.makeconftest(_INNER_CONFTEST_SRC)
    return pytester


def _run(pytester, body: str) -> None:
    pytester.makepyfile(textwrap.dedent(body))
    pytester.runpytest_inprocess(*_RUN_ARGS)


# ---------------------------------------------------------------------------
# Call-phase exit paths
# ---------------------------------------------------------------------------


def test_pass_maps_to_passed(inner):
    _run(
        inner,
        """
        def test_x():
            assert True
        """,
    )
    assert capture.final_status("test_x") == TestStatus.PASSED


def test_assert_failure_maps_to_failed(inner):
    _run(
        inner,
        """
        def test_x():
            assert 1 == 2
        """,
    )
    assert capture.final_status("test_x") == TestStatus.FAILED


def test_generic_exception_maps_to_error(inner):
    _run(
        inner,
        """
        def test_x():
            raise ValueError("boom")
        """,
    )
    assert capture.final_status("test_x") == TestStatus.ERROR


def test_system_exit_maps_to_error(inner):
    _run(
        inner,
        """
        import sys
        def test_x():
            sys.exit(1)
        """,
    )
    # AUDIT: SystemExit is currently routed through the generic-exception
    # path because it isn't an AssertionError. The audit may want a
    # dedicated bucket (e.g., ABORTED) since the test didn't really "error"
    # so much as exit deliberately.
    assert capture.final_status("test_x") == TestStatus.ERROR


def test_pytest_fail_maps_to_error(inner):
    _run(
        inner,
        """
        import pytest
        def test_x():
            pytest.fail("intentional failure")
        """,
    )
    # AUDIT: target is FAILED. pytest.fail raises a Failed OutcomeException,
    # which the plugin treats as a generic exception (not AssertionError)
    # and routes to ERROR. Users expect pytest.fail and assert-fail to land
    # in the same bucket.
    assert capture.final_status("test_x") == TestStatus.ERROR


def test_keyboard_interrupt_aborts_session(inner):
    # KeyboardInterrupt propagates out of pytester's inprocess runner, so
    # we catch it here. Pytest aborts the session before the call phase's
    # makereport fires, so the plugin never sees the interrupt; the step
    # fixture's teardown runs with no rep_call.excinfo and resolves the
    # step to PASSED.
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
    # AUDIT: target is ABORTED (or similar). The step is recorded as PASSED
    # despite the test having been aborted, which is misleading.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.PASSED


# ---------------------------------------------------------------------------
# Skip paths
# ---------------------------------------------------------------------------


def test_pytest_skip_in_body_records_skipped_substep(inner):
    _run(
        inner,
        """
        import pytest
        def test_x():
            pytest.skip("not today")
        """,
    )
    # The plugin's makereport hook creates a separate SKIPPED step nested
    # under the autouse outer step. The outer step itself is driven by the
    # call-phase excinfo (a Skipped exception), which the plugin treats as
    # a generic exception -> ERROR.
    skipped_steps = [
        s
        for s in capture.steps_by_name("test_x")
        if s.statuses and s.statuses[-1] == TestStatus.SKIPPED
    ]
    assert skipped_steps, "expected at least one SKIPPED step for a skipped test"
    # AUDIT: the outer step records ERROR for an in-body pytest.skip().
    # Target: outer step should be SKIPPED; no nested duplicate.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.ERROR


def test_pytest_mark_skip_records_skipped(inner):
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


def test_skip_inside_fixture_setup(inner):
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
    # AUDIT: target is SKIPPED with phase=setup on the outer step. Today
    # the autouse outer step lands in PASSED (its own setup ran, no failure
    # was recorded), and a separate nested SKIPPED step is created by the
    # makereport hook from the setup-phase skip report.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.PASSED
    nested_skipped = [
        s
        for s in capture.steps_by_name("test_x")
        if s.parent_step_id is not None and s.statuses[-1] == TestStatus.SKIPPED
    ]
    assert nested_skipped, "fixture-skip currently produces a nested SKIPPED step"


# ---------------------------------------------------------------------------
# xfail / xpass
# ---------------------------------------------------------------------------


def test_xfail_marked_test_that_fails(inner):
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(reason="known issue")
        def test_x():
            assert 1 == 2
        """,
    )
    # AUDIT: target is XFAILED (distinct from SKIPPED). Today, pytest reports
    # outcome="skipped" for an xfailed test, so the makereport hook records
    # a SKIPPED nested step. The outer autouse step records FAILED from the
    # call-phase AssertionError.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.FAILED
    skipped_substeps = [
        s
        for s in capture.steps_by_name("test_x")
        if s.parent_step_id is not None and s.statuses[-1] == TestStatus.SKIPPED
    ]
    assert skipped_substeps, "xfailed test currently produces a nested SKIPPED step"


def test_xfail_strict_unexpected_pass(inner):
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(strict=True, reason="should fail")
        def test_x():
            assert True
        """,
    )
    # AUDIT: target is XPASSED. The test body raises no exception, so the
    # plugin records PASSED and never sees pytest's later "strict xfail
    # passed" failure attached to the report.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.PASSED


def test_xfail_non_strict_unexpected_pass(inner):
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(reason="might pass sometimes")
        def test_x():
            assert True
        """,
    )
    # AUDIT: target is XPASSED. Non-strict xfail that passes is reported by
    # pytest as outcome="passed" with wasxfail set; the plugin ignores
    # wasxfail and records PASSED.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.PASSED


def test_xfail_raises_mismatch(inner):
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(raises=ValueError, reason="expected ValueError")
        def test_x():
            raise KeyError("wrong exception")
        """,
    )
    # AUDIT: target is FAILED. Pytest treats a `raises=` mismatch as a real
    # call-phase failure; the plugin sees a non-assertion exception in
    # excinfo and routes it to ERROR.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.ERROR


def test_xfail_run_false(inner):
    _run(
        inner,
        """
        import pytest
        @pytest.mark.xfail(run=False, reason="never run")
        def test_x():
            assert False
        """,
    )
    # AUDIT: target is XFAILED. With run=False pytest reports the test as
    # skipped/xfailed without executing it, so today only the makereport
    # hook records a step, with status SKIPPED.
    assert capture.final_status("test_x") == TestStatus.SKIPPED


# ---------------------------------------------------------------------------
# Setup-phase / teardown-phase fixture failures
# ---------------------------------------------------------------------------


def test_setup_phase_fixture_failure(inner):
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
    # AUDIT: target is ERROR with phase=setup. Today the plugin doesn't
    # consult report.when, so the outer step (if it exists) lands in PASSED
    # because the call phase never ran and no failure was recorded.
    outer = capture.test_step("test_x")
    if outer is not None:
        assert outer.statuses[-1] == TestStatus.PASSED, (
            f"setup-fail outer step status was {outer.statuses[-1]}; "
            "audit target is ERROR with phase=setup"
        )


def test_teardown_phase_fixture_failure(inner):
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
    # AUDIT: target is FAILED with phase=teardown. Today the outer autouse
    # step closes BEFORE the failing fixture's teardown runs, so the test
    # body's success is recorded as PASSED and the teardown error is
    # invisible to the step.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.PASSED, (
        f"teardown-fail outer step status was {outer.statuses[-1]}; "
        "audit target is FAILED with phase=teardown"
    )


def test_call_fail_plus_teardown_fail(inner):
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
    # AUDIT: the call-phase failure dominates (status FAILED), and the
    # teardown error is silently lost. Target: status should reflect both
    # signals, e.g. FAILED with a teardown-phase annotation so the
    # teardown error is not invisible.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.FAILED


# ---------------------------------------------------------------------------
# Collection-phase failures
# ---------------------------------------------------------------------------


def test_missing_fixture_records_passed_step(inner):
    _run(
        inner,
        """
        def test_x(nonexistent_fixture):
            assert True
        """,
    )
    # AUDIT: target is ERROR with phase=setup. Today the autouse `step`
    # fixture's setup still runs (because it has no dependency on the
    # missing fixture), creates an outer step, then the missing-fixture
    # error aborts setup. The autouse step's teardown runs with no
    # rep_call.excinfo and resolves to PASSED -- so the user sees a
    # green step in Sift for a test that never executed.
    outer = capture.test_step("test_x")
    assert outer is not None
    assert outer.statuses[-1] == TestStatus.PASSED


# ---------------------------------------------------------------------------
# Plugin-API exit paths (in-test mutations)
# ---------------------------------------------------------------------------


def test_manual_status_update_to_failed(inner):
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
    _run(
        inner,
        """
        def test_x(step):
            step.measure(name="m", value=10.0, bounds={"min": 0.0, "max": 5.0})
        """,
    )
    assert capture.final_status("test_x") == TestStatus.FAILED
