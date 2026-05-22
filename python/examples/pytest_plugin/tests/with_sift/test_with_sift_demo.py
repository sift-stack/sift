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
    step.measure(name="numeric_value", value=1.5, bounds={"min": 0.0, "max": 2.0}, channel_names=["channel_1", "channel_2"])


def test_substeps(step) -> None:
    """``step.substep(...)`` opens child steps inside one test; substeps nest arbitrarily.
    This can be useful for grouping related measurements or for creating a more natural report structure
    without the need to create a new test, class, etc.
    """
    with step.substep(name="phase_1") as s1:
        s1.measure(name="value", value=1.0, bounds={"min": 0.0, "max": 2.0})

    with step.substep(name="phase_2") as s2:
        with s2.substep(name="phase_2a") as s2a:
            s2a.measure(name="value", value=1.0, bounds={"min": 0.0, "max": 2.0})


@pytest.mark.sift_exclude
def test_excluded() -> None:
    """``sift_exclude`` runs the test in pytest but produces no Sift step."""
    assert True


class TestClassStep:
    """A test class becomes its own step in the report tree.

    This docstring becomes the description of the ``TestClassStep`` step.
    """

    @pytest.mark.parametrize("axis_b", ["b1", "b2"])
    @pytest.mark.parametrize("axis_a", ["a1", "a2"])
    def test_parametrize(self, step, axis_a: str, axis_b: str) -> None:
        """Stacked parametrize nests outer-to-inner in decorator-on-page order."""
        step.measure(name="value", value=1.0, bounds={"min": 0.0, "max": 2.0})

    class TestNested:
        """Nested classes produce nested class steps."""

        def test_report_outcome(self, step) -> None:
            """``step.report_outcome`` records a non-numeric pass/fail substep."""
            step.report_outcome(name="check", result=True, reason="value matched")
