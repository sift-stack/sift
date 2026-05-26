"""End-to-end demo of the test-results features: measurements, substeps,
exclusion, classes, nested classes, and stacked parametrize."""

import pytest


def test_measurements(step) -> None:
    """Measurements are the first-class method for recording numeric, string, or bool bounds criteria and their outcomes. These show up in report steps.
    ``step.measure`` accepts numeric (min/max), string, or bool bounds.
    Names should be chosen that provide sufficient context, but general enough that similar/identical measurements
    across steps or reports can be compared.
    """
    step.measure(name="numeric_value", value=1.5, bounds={"min": 0.0, "max": 2.0})
    step.measure(name="string_label", value="ok", bounds="ok")
    step.measure(name="bool_flag", value=True, bounds=True)

    # Descriptions and metadata can also be provided to measurements.
    step.measure(
        name="numeric_value_2",
        value=1.5,
        bounds={"min": 0.0, "max": 2.0},
        description="Numeric that represents X, Y, Z",
        metadata={"subsystem": "A"},
    )

    # If you plan to link the pytest report to a Sift Run, you can also assign related channels for easy plotting in the app
    step.measure(
        name="numeric_value",
        value=1.5,
        bounds={"min": 0.0, "max": 2.0},
        channel_names=["channel_1", "channel_2"],
    )


def test_substeps(step) -> None:
    """``step.substep(...)`` opens child steps inside one test; substeps nest arbitrarily.
    This can be useful for grouping related measurements or for creating a more natural report structure
    without the need to create a new test, class, etc.

    Metadata can be attached at the step level by passing ``metadata=...`` to
    ``substep``; the same keyword is accepted by ``report_context.new_step``
    and propagates to the resulting ``TestStep``.
    """
    with step.substep(name="phase_1", metadata={"phase_index": 1}) as s1:
        s1.measure(name="value", value=1.0, bounds={"min": 0.0, "max": 2.0})

    with step.substep(name="phase_2", metadata={"phase_index": 2}) as s2:
        with s2.substep(name="phase_2a") as s2a:
            s2a.measure(name="value", value=1.0, bounds={"min": 0.0, "max": 2.0})


def test_measure_series(step) -> None:
    """``measure_avg`` and ``measure_all`` are the series variants of ``measure``.

    Both accept a list, numpy array, or pandas series of numeric values.
    ``measure_avg`` records one row holding the mean of the series and
    bounds-checks it. ``measure_all`` evaluates every value individually and
    records one row per out-of-bounds element (in-bounds values are NOT
    recorded, keeping the report compact).
    """
    voltages = [4.95, 5.02, 5.01, 4.98, 5.00]
    step.measure_avg(
        name="voltage_mean",
        values=voltages,
        bounds={"min": 4.9, "max": 5.1},
        unit="V",
    )
    # All values are in-bounds here, so measure_all records nothing extra;
    # change one to e.g. 6.0 to see an out-of-bounds row appear.
    step.measure_all(
        name="voltage_samples",
        values=voltages,
        bounds={"min": 4.9, "max": 5.1},
        unit="V",
    )


def test_failed_measurement_marks_sift_step_failed(step) -> None:
    """An out-of-bounds measurement marks the Sift step as ``FAILED``
    without raising. The pytest test still passes (no assertion, no
    exception); the Sift report records bounds compliance while pytest
    records control flow.

    Use this pattern when measurements are diagnostic data you want to
    collect alongside the test result, even when some readings fall outside
    spec. See ``test_assert_passed_at_end`` below for the recommended way
    to also fail pytest when any measurement is out of bounds.
    """
    step.measure(
        name="voltage",
        value=99.0,  # outside the bounds below; marks the step FAILED in Sift
        bounds={"min": 0.0, "max": 10.0},
        unit="V",
    )


def test_fail_if_measurements_failed_at_end(step) -> None:
    """Recommended pattern: take every measurement first, then call
    ``step.fail_if_measurements_failed()`` once at the end.

    Asserting on individual ``step.measure(...)`` calls raises
    ``AssertionError`` on the first failure, so any measurements after the
    failing one never run and never land in the Sift report. The end-of-test
    call is strictly better for diagnostic completeness: every measurement is
    recorded, including the failures, and the aggregate result is then folded
    into the pytest outcome. It fails via ``pytest.fail`` rather than an
    assertion, so the failed step carries no assertion noise in ``error_info``.

    The ``b`` measurement below is deliberately out of bounds. ``c`` still
    runs and is recorded; only the final call fails the test.
    """
    step.measure(name="a", value=1.0, bounds={"min": 0.0, "max": 2.0})
    step.measure(name="b", value=99.0, bounds={"min": 0.0, "max": 2.0})  # out of bounds
    step.measure(name="c", value=1.5, bounds={"min": 0.0, "max": 2.0})  # still recorded
    step.fail_if_measurements_failed()


def test_report_level_metadata(step, report_context) -> None:
    """Attach metadata to the run-wide ``TestReport`` via ``report_context.report.update(...)``.

    The same ``update({...})`` pattern works for any field on
    ``TestReportUpdate`` (``run_id``, ``serial_number``, ``part_number``,
    ``system_operator``, ``metadata``, ...). Useful for linking a session
    to a Sift Run or tagging the report with build / operator info.
    """
    report_context.report.update(
        {
            "metadata": {
                "build_id": "v1.2.3",
                "operator": "ci",
            }
        }
    )
    step.measure(name="value", value=1.0, bounds={"min": 0.0, "max": 2.0})


@pytest.mark.sift_exclude
def test_excluded() -> None:
    """``sift_exclude`` runs the test in pytest but produces no Sift step."""
    assert True


class TestClassStep:
    """A test class becomes its own step in the report tree.

    This docstring becomes the description of the ``TestClassStep`` step.
    """

    @pytest.mark.parametrize("axis_a", ["a1", "a2"])
    @pytest.mark.parametrize("axis_b", ["b1", "b2"])
    def test_parametrize(self, step, axis_a: str, axis_b: str) -> None:
        """Stacked parametrize nests outer-to-inner in decorator-on-page order."""
        step.measure(name="value", value=1.0, bounds={"min": 0.0, "max": 2.0})

    class TestNested:
        """Nested classes produce nested class steps."""

        def test_report_outcome(self, step) -> None:
            """``step.report_outcome`` records a non-numeric pass/fail substep."""
            step.report_outcome(name="check", result=True, reason="value matched")
