from datetime import datetime, timezone

import pytest

from sift_client.sift_types.test_report import (
    TestMeasurementCreate,
    TestMeasurementType,
    TestMeasurementUpdate,
    TestStatus,
    TestStepType,
)
from sift_client.util.test_results.bounds import (
    assign_value_to_measurement,
    evaluate_measurement_bounds,
)
from sift_client.util.test_results.context_manager import NewStep

pytestmark = pytest.mark.integration


class TestContextManager:
    def test_new_step(self, report_context):
        initial_end_time = report_context.report.end_time
        first_step_path = report_context.get_next_step_path()
        substep_path = f"{first_step_path}.1"
        nested_substep_path = f"{substep_path}.1"
        sibling_substep_path = f"{first_step_path}.2"
        sibling_nested_substep_path = f"{substep_path}.2"
        first_step_path_parts = first_step_path.split(".")
        prefix = ""
        if len(first_step_path_parts) > 1:
            prefix = f"{'.'.join(first_step_path_parts[:-1])}."
        second_step_path = f"{prefix}{int(first_step_path_parts[-1]) + 1}"
        test_step = None
        with NewStep(report_context, "Test Step", "Test Description") as new_step:
            test_step = new_step.current_step
            assert test_step.test_report_id == report_context.report.id_
            assert test_step.name == "Test Step"
            assert test_step.description == "Test Description"
            assert test_step.start_time
            assert test_step.end_time
            assert test_step.status == TestStatus.IN_PROGRESS
            assert test_step.step_path == first_step_path
            assert test_step.step_type == TestStepType.ACTION
            assert test_step.error_info == None

            with new_step.substep("Substep", "Substep Description") as substep:
                current_substep = substep.current_step
                assert current_substep.test_report_id == report_context.report.id_
                assert current_substep.name == "Substep"
                assert current_substep.description == "Substep Description"
                assert current_substep.step_path == substep_path
                assert current_substep.parent_step_id == test_step.id_
                assert current_substep.start_time
                assert current_substep.end_time
                assert current_substep.status == TestStatus.IN_PROGRESS
                assert current_substep.step_type == TestStepType.ACTION
                assert current_substep.error_info == None

                with substep.substep(
                    "nested substep", "Nested substep Description"
                ) as nested_substep:
                    current_nested_substep = nested_substep.current_step
                    assert current_nested_substep.test_report_id == report_context.report.id_
                    assert current_nested_substep.name == "nested substep"
                    assert current_nested_substep.description == "Nested substep Description"
                    assert current_nested_substep.step_path == nested_substep_path
                    assert current_nested_substep.parent_step_id == current_substep.id_
                    assert current_nested_substep.start_time
                    assert current_nested_substep.end_time
                    assert current_nested_substep.parent_step_id == current_substep.id_

                with substep.substep(
                    "sibling nested substep", "Sibling nested substep Description"
                ) as sibling_substep:
                    current_sibling_substep = sibling_substep.current_step
                    assert current_sibling_substep.test_report_id == report_context.report.id_
                    assert current_sibling_substep.name == "sibling nested substep"
                    assert (
                        current_sibling_substep.description == "Sibling nested substep Description"
                    )
                    assert current_sibling_substep.step_path == sibling_nested_substep_path
                    assert current_sibling_substep.parent_step_id == current_substep.id_

            with new_step.substep(
                "sibling substep", "Sibling substep Description"
            ) as sibling_substep:
                current_sibling_substep = sibling_substep.current_step
                assert current_sibling_substep.test_report_id == report_context.report.id_
                assert current_sibling_substep.name == "sibling substep"
                assert current_sibling_substep.description == "Sibling substep Description"
                assert current_sibling_substep.step_path == sibling_substep_path
                assert current_sibling_substep.parent_step_id == test_step.id_

        with report_context.new_step("Test Step 2", "Test Step 2 Description") as new_step_2:
            test_step_2 = new_step_2.current_step
            assert test_step_2.test_report_id == report_context.report.id_
            assert test_step_2.name == "Test Step 2"
            assert test_step_2.description == "Test Step 2 Description"
            assert test_step_2.step_path == second_step_path

        assert test_step.end_time > initial_end_time
        assert test_step_2.start_time > test_step.end_time
        assert test_step.status == TestStatus.PASSED

    def test_measurement_update(self, report_context):
        test_step = None
        with report_context.new_step("Test Measure", "Test Measure Description") as new_step:
            test_step = new_step.current_step
            new_step.measure(name="Test Measurement", value=10, bounds={"min": 0, "max": 10})
            new_step.measure(name="Test Measurement 2", value="string value", bounds="string value")
            new_step.measure(name="Test Measurement 3", value=True, bounds="true")

        assert len(test_step.measurements) == 3
        assert test_step.measurements[0].name == "Test Measurement"
        assert test_step.measurements[0].numeric_value == 10
        assert test_step.measurements[0].measurement_type == TestMeasurementType.DOUBLE
        assert test_step.measurements[1].name == "Test Measurement 2"
        assert test_step.measurements[1].string_value == "string value"
        assert test_step.measurements[1].measurement_type == TestMeasurementType.STRING
        assert test_step.measurements[2].name == "Test Measurement 3"
        assert test_step.measurements[2].boolean_value == True
        assert test_step.measurements[2].measurement_type == TestMeasurementType.BOOLEAN

    def test_error_info(self, report_context, step):
        test_step = None
        parent_step_path = step.current_step.step_path
        open_step_result = report_context.open_step_results.get(parent_step_path, False)
        any_failures = report_context.any_failures

        with report_context.new_step("Test Error", "Test Error Description") as new_step:
            test_step = new_step.current_step
            raise Exception("Test Error")
        assert test_step.error_info is not None
        assert test_step.error_info.error_code == 1
        assert test_step.error_info.error_message == "Test Error"
        assert test_step.status == TestStatus.ERROR
        # If the parent step is not marked as failed already, make sure it remains passed at this point.
        if not open_step_result:
            report_context.open_step_results[parent_step_path] = True
        if not any_failures:
            report_context.any_failures = False


class TestBounds:
    def test_assign_value_to_measurement(self):
        measurement = TestMeasurementUpdate(
            measurement_type=TestMeasurementType.DOUBLE,
        )
        assign_value_to_measurement(measurement, 10)
        assert measurement.numeric_value == 10
        assert measurement.measurement_type == TestMeasurementType.DOUBLE
        measurement = TestMeasurementCreate(
            test_step_id="test_step_id",
            name="Test Measurement",
            passed=True,
            timestamp=datetime.now(timezone.utc),
        )
        assign_value_to_measurement(measurement, "string value")
        assert measurement.string_value == "string value"
        assert measurement.measurement_type == TestMeasurementType.STRING
        measurement = TestMeasurementUpdate()
        assign_value_to_measurement(measurement, True)
        assert measurement.boolean_value == True

    def test_evaluate_measurement_bounds(self):
        measurement = TestMeasurementUpdate(
            measurement_type=TestMeasurementType.DOUBLE,
        )
        result = evaluate_measurement_bounds(measurement, 10, {"min": 0, "max": 10})
        assert result == True
        assert measurement.passed == True

    def test_evaluate_measurement_bounds_numeric_within_range(self):
        """Test numeric value within min and max bounds."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 5.0, {"min": 0.0, "max": 10.0})
        assert result == True
        assert measurement.passed == True
        assert measurement.numeric_value == 5.0
        assert measurement.measurement_type == TestMeasurementType.DOUBLE

    def test_evaluate_measurement_bounds_numeric_at_min(self):
        """Test numeric value exactly at minimum bound."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 0.0, {"min": 0.0, "max": 10.0})
        assert result == True
        assert measurement.passed == True

    def test_evaluate_measurement_bounds_numeric_at_max(self):
        """Test numeric value exactly at maximum bound."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 10.0, {"min": 0.0, "max": 10.0})
        assert result == True
        assert measurement.passed == True

    def test_evaluate_measurement_bounds_numeric_below_min(self):
        """Test numeric value below minimum bound."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, -1.0, {"min": 0.0, "max": 10.0})
        assert result == False
        assert measurement.passed == False

    def test_evaluate_measurement_bounds_numeric_above_max(self):
        """Test numeric value above maximum bound."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 11.0, {"min": 0.0, "max": 10.0})
        assert result == False
        assert measurement.passed == False

    def test_evaluate_measurement_bounds_numeric_min_only(self):
        """Test numeric value with only minimum bound."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 5.0, {"min": 0.0})
        assert result == True
        assert measurement.passed == True

        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, -1.0, {"min": 0.0})
        assert result == False
        assert measurement.passed == False

    def test_evaluate_measurement_bounds_numeric_max_only(self):
        """Test numeric value with only maximum bound."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 5.0, {"max": 10.0})
        assert result == True
        assert measurement.passed == True

        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 11.0, {"max": 10.0})
        assert result == False
        assert measurement.passed == False

    def test_evaluate_measurement_bounds_with_numeric_bounds_object(self):
        """Test using NumericBounds object instead of dict."""
        from sift_client.sift_types.test_report import NumericBounds

        measurement = TestMeasurementUpdate()
        bounds = NumericBounds(min=0.0, max=10.0)
        result = evaluate_measurement_bounds(measurement, 5.0, bounds)
        assert result == True
        assert measurement.passed == True
        assert measurement.numeric_bounds.min == 0.0
        assert measurement.numeric_bounds.max == 10.0

    def test_evaluate_measurement_bounds_string_matching(self):
        """Test string value matching expected string."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, "expected", "expected")
        assert result == True
        assert measurement.passed == True
        assert measurement.string_value == "expected"
        assert measurement.string_expected_value == "expected"
        assert measurement.measurement_type == TestMeasurementType.STRING

    def test_evaluate_measurement_bounds_string_not_matching(self):
        """Test string value not matching expected string."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, "actual", "expected")
        assert result == False
        assert measurement.passed == False
        assert measurement.string_value == "actual"
        assert measurement.string_expected_value == "expected"

    def test_evaluate_measurement_bounds_boolean_matching(self):
        """Test boolean value matching expected string."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, True, "true")
        assert result == True
        assert measurement.passed == True
        assert measurement.boolean_value == True
        assert measurement.string_expected_value == "true"
        assert measurement.measurement_type == TestMeasurementType.BOOLEAN

    def test_evaluate_measurement_bounds_boolean_not_matching(self):
        """Test boolean value not matching expected string."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, False, "true")
        assert result == False
        assert measurement.passed == False
        assert measurement.boolean_value == False
        assert measurement.string_expected_value == "true"

    def test_evaluate_measurement_bounds_boolean_case_insensitive(self):
        """Test boolean comparison is case insensitive."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, True, "TRUE")
        assert result == True
        assert measurement.passed == True

        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, True, "True")
        assert result == True
        assert measurement.passed == True

    def test_evaluate_measurement_bounds_no_bounds(self):
        """Test measurement without bounds. Expected behavior is that the measurement's passed value is unchanged."""
        measurement = TestMeasurementUpdate(passed=False)
        result = evaluate_measurement_bounds(measurement, 5.0, None)
        assert result == False  #
        assert measurement.passed == False
        assert measurement.numeric_value == 5.0

        measurement = TestMeasurementUpdate(passed=True)
        result = evaluate_measurement_bounds(measurement, "string value", None)
        assert result == True
        assert measurement.passed == True
        assert measurement.string_value == "string value"

    def test_evaluate_measurement_bounds_integer_value(self):
        """Test that integer values are converted to float."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 5, {"min": 0, "max": 10})
        assert result == True
        assert measurement.passed == True
        assert measurement.numeric_value == 5.0
        assert measurement.measurement_type == TestMeasurementType.DOUBLE

    def test_evaluate_measurement_bounds_with_test_measurement_create(self):
        """Test evaluation with TestMeasurementCreate."""
        measurement = TestMeasurementCreate(
            test_step_id="test_step_id",
            name="Test Measurement",
            passed=True,
            timestamp=datetime.now(timezone.utc),
        )
        result = evaluate_measurement_bounds(measurement, 5.0, {"min": 0.0, "max": 10.0})
        assert result == True
        assert measurement.passed == True
        assert measurement.numeric_value == 5.0

    def test_evaluate_measurement_bounds_negative_range(self):
        """Test numeric bounds with negative values."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, -5.0, {"min": -10.0, "max": -1.0})
        assert result == True
        assert measurement.passed == True

        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 0.0, {"min": -10.0, "max": -1.0})
        assert result == False
        assert measurement.passed == False

    def test_evaluate_measurement_bounds_large_values(self):
        """Test numeric bounds with large values."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 1e6, {"min": 0.0, "max": 1e9})
        assert result == True
        assert measurement.passed == True

    def test_evaluate_measurement_bounds_small_precision(self):
        """Test numeric bounds with small decimal precision."""
        measurement = TestMeasurementUpdate()
        result = evaluate_measurement_bounds(measurement, 0.00001, {"min": 0.0, "max": 0.0001})
        assert result == True
        assert measurement.passed == True
