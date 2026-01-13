from __future__ import annotations

import getpass
import os
import socket
import traceback
from contextlib import AbstractContextManager
from datetime import datetime, timezone
from typing import TYPE_CHECKING

import numpy as np
import pandas as pd

from sift_client.sift_types.test_report import (
    ErrorInfo,
    NumericBounds,
    TestMeasurementCreate,
    TestReport,
    TestReportCreate,
    TestStatus,
    TestStep,
    TestStepCreate,
    TestStepType,
)
from sift_client.util.test_results.bounds import (
    evaluate_measurement_bounds,
)

if TYPE_CHECKING:
    from numpy.typing import NDArray

    from sift_client.client import SiftClient


class ReportContext(AbstractContextManager):
    """Context manager for a new TestReport. See usage example in __init__.py."""

    report: TestReport
    step_is_open: bool
    step_stack: list[TestStep]
    step_number_at_depth: dict[int, int]
    open_step_results: dict[str, bool]
    any_failures: bool

    def __init__(
        self,
        client: SiftClient,
        name: str,
        test_system_name: str | None = None,
        system_operator: str | None = None,
        test_case: str | None = None,
    ):
        """Initialize a new report context.

        Args:
            client: The Sift client to use to create the report.
            name: The name of the report.
            test_system_name: The name of the test system. Will default to the hostname if not provided.
            system_operator: The operator of the test system. Will default to the current user if not provided.
            test_case: The name of the test case. Will default to the basename of the file containing the test if not provided.
        """
        self.step_is_open = False
        self.step_stack = []
        self.step_number_at_depth = {}
        self.open_step_results = {}
        self.any_failures = False

        # Create the report.
        test_case = test_case if test_case else os.path.basename(__file__)
        test_system_name = test_system_name if test_system_name else socket.gethostname()
        system_operator = system_operator if system_operator else getpass.getuser()
        create = TestReportCreate(
            name=name,
            test_system_name=test_system_name,
            test_case=test_case,
            start_time=datetime.now(timezone.utc),
            end_time=datetime.now(timezone.utc),
            status=TestStatus.IN_PROGRESS,
            system_operator=system_operator,
        )
        self.report = client.test_results.create(create)

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        update = {
            "end_time": datetime.now(timezone.utc),
        }
        if self.any_failures or exc_type:
            update["status"] = TestStatus.FAILED
        else:
            update["status"] = TestStatus.PASSED
        self.report.update(update)
        return True

    def new_step(
        self, name: str, description: str | None = None, assertion_as_fail_not_error: bool = True
    ) -> NewStep:
        """Alias to return a new step context manager from this report context. Use create_step for actually creating a TestStep in the current context."""
        return NewStep(
            self,
            name=name,
            description=description,
            assertion_as_fail_not_error=assertion_as_fail_not_error,
        )

    def get_next_step_path(self) -> str:
        """Get the next step path for the current depth."""
        top_step = self.step_stack[-1] if self.step_stack else None
        step_path = top_step.step_path if top_step else ""
        next_step_number = self.step_number_at_depth.get(len(self.step_stack), 0) + 1
        prefix = f"{step_path}." if step_path else ""
        return f"{prefix}{next_step_number}"

    def create_step(self, name: str, description: str | None = None) -> TestStep:
        """Create a new step in the report context.

        Args:
            name: The name of the step.
            description: The description of the step.

        Returns:
            The created step.
        """
        step_path = self.get_next_step_path()
        parent_step = self.step_stack[-1] if self.step_stack else None

        step = self.report.client.test_results.create_step(
            TestStepCreate(
                test_report_id=str(self.report.id_),
                name=name,
                step_type=TestStepType.ACTION,
                step_path=step_path,
                status=TestStatus.IN_PROGRESS,
                start_time=datetime.now(timezone.utc),
                end_time=datetime.now(timezone.utc),
                description=description,
                parent_step_id=parent_step.id_ if parent_step else None,
            )
        )

        # Update the step tracking structures.
        self.step_number_at_depth[len(self.step_stack)] = (
            self.step_number_at_depth.get(len(self.step_stack), 0) + 1
        )
        self.step_stack.append(step)
        self.open_step_results[step.step_path] = True

        return step

    def record_step_outcome(self, outcome: bool, step: TestStep):
        """Report a failure to the report context."""
        # Failures will be propogated when the step exits.
        if not outcome:
            self.open_step_results[step.step_path] = False
            self.any_failures = True

    def resolve_and_propagate_step_result(
        self,
        step: TestStep,
        error_info: ErrorInfo | None = None,
    ) -> bool:
        """Resolve the result of a step and propagate the result to the parent step if it failed."""
        result = self.open_step_results.get(step.step_path, True)
        if error_info:
            result = False
        if step.status != TestStatus.IN_PROGRESS:
            # The step was manually completed so use that result.
            # Skipped steps are considered passed.
            result = step.status in (TestStatus.PASSED, TestStatus.SKIPPED)

        # Update the parent step results if this step failed (true by default so no need to do anything if we didn't fail).
        if not result:
            self.any_failures = True
            self.open_step_results[step.step_path] = False
            path_parts = step.step_path.split(".")
            if len(path_parts) > 1:
                parent_step_path = ".".join(path_parts[:-1])
                self.open_step_results[parent_step_path] = False

        return result

    def exit_step(self, step: TestStep):
        """Exit a step and update the report context."""
        self.step_number_at_depth[len(self.step_stack)] = 0
        stack_top = self.step_stack.pop()
        self.open_step_results.pop(step.step_path)

        if stack_top.id_ != step.id_:
            raise ValueError(
                "The popped step was not the top of the stack. This should never happen."
            )


class NewStep(AbstractContextManager):
    """Context manager to create a new step in a test report. See usage example in __init__.py."""

    report_context: ReportContext
    client: SiftClient
    assertion_as_fail_not_error: bool = True
    current_step: TestStep | None = None

    def __init__(
        self,
        report_context: ReportContext,
        name: str,
        description: str | None = None,
        assertion_as_fail_not_error: bool = True,
    ):
        """Initialize a new step context.

        Args:
            report_context: The report context to create the step in.
            name: The name of the step.
            description: The description of the step.
            assertion_as_fail_not_error: Mark steps with assertion errors as failed instead of error+traceback (some users want assertions to work as simple failures especially when using pytest).
        """
        self.report_context = report_context
        self.client = report_context.report.client
        self.current_step = self.report_context.create_step(name, description)
        self.assertion_as_fail_not_error = assertion_as_fail_not_error

    def __enter__(self):
        """Enter the context manager to create a new step.

        returns: The current step.
        """
        return self

    def update_step_from_result(
        self,
        exc: type[Exception] | None,
        exc_value: Exception | None,
        tb: traceback.TracebackException | None,
    ) -> bool:
        """Update the step based on its substeps and if there was an exception while executing the step.

        Args:
            exc: The class of Exception that was raised.
            exc_value: The exception value.
            tb: The traceback object.

        returns: The false if step failed or errored, true otherwise.
        """
        error_info = None
        assert self.current_step is not None
        if exc:
            if isinstance(exc_value, AssertionError) and not self.assertion_as_fail_not_error:
                # If we're not showing assertion errors (i.e. pytest), mark step as failed but don't set error info.
                self.report_context.record_step_outcome(False, self.current_step)
            else:
                stack = traceback.format_exception(exc, exc_value, tb)  # type: ignore
                stack = [stack[0], *stack[-10:]] if len(stack) > 10 else stack
                trace = "".join(stack)
                error_info = ErrorInfo(
                    error_code=1,
                    error_message=trace,
                )

        # Resolve the status of this step (i.e. fail if children failed) and propagate the result to the parent step.
        result = self.report_context.resolve_and_propagate_step_result(
            self.current_step, error_info
        )

        # Mark the step as completed
        status = self.current_step.status
        if status == TestStatus.IN_PROGRESS:
            # Update the status only if the step was in progress i.e. not updated elsewhere.
            status = TestStatus.PASSED if result else TestStatus.FAILED
        if error_info:
            status = TestStatus.ERROR
        self.current_step.update(
            {
                "status": status,
                "end_time": datetime.now(timezone.utc),
                "error_info": error_info,
            }
        )

        return result

    def __exit__(self, exc, exc_value, tb):
        result = self.update_step_from_result(exc, exc_value, tb)

        # Now that the step is updated. Let the report context handle removing it from the stack and updating the report context.
        self.report_context.exit_step(self.current_step)

        # Test only attribute (hence not public class variable)
        # This changes the result after the status and error info are set.
        if hasattr(self, "force_result"):
            result = self.force_result

        return result

    def measure(
        self,
        *,
        name: str,
        value: float | str | bool | int,
        bounds: dict[str, float] | NumericBounds | str | None = None,
        timestamp: datetime | None = None,
        unit: str | None = None,
    ) -> bool:
        """Measure a value and return the result.

        Args:
            name: The name of the measurement.
            value: The value of the measurement.
            bounds: [Optional] The bounds to compare the value to.
            timestamp: [Optional] The timestamp of the measurement. Defaults to the current time.
            unit: [Optional] The unit of the measurement.

        returns: The result of the measurement.
        """
        assert self.current_step is not None
        create = TestMeasurementCreate(
            test_step_id=str(self.current_step.id_),
            name=name,
            passed=True,
            timestamp=timestamp if timestamp else datetime.now(timezone.utc),
            unit=unit,
        )
        evaluate_measurement_bounds(create, value, bounds)
        measurement = self.client.test_results.create_measurement(create)
        self.report_context.record_step_outcome(measurement.passed, self.current_step)

        return measurement.passed

    def measure_avg(
        self,
        *,
        name: str,
        values: list[float | int] | NDArray[np.float64] | pd.Series,
        bounds: dict[str, float] | NumericBounds,
        timestamp: datetime | None = None,
        unit: str | None = None,
    ) -> bool:
        """Calculate the average of a list of values, measure the average against given bounds, and return the result.

        Args:
            name: The name of the measurement.
            values: The list of values to measure the average of.
            bounds: The bounds to compare the value to.
            timestamp: [Optional] The timestamp of the measurement. Defaults to the current time.
            unit: [Optional] The unit of the measurement.

        returns: The true if the average of the values is within the bounds, false otherwise.
        """
        timestamp = timestamp if timestamp else datetime.now(timezone.utc)
        np_array = None
        if isinstance(values, list):
            np_array = np.array(values)
        elif isinstance(values, np.ndarray):
            np_array = values
        elif isinstance(values, pd.Series):
            np_array = values.to_numpy()
        else:
            raise ValueError(f"Invalid value type: {type(values)}")
        avg = float(np.mean(np_array))
        result = self.measure(name=name, value=avg, bounds=bounds, timestamp=timestamp, unit=unit)
        assert self.current_step is not None
        self.report_context.record_step_outcome(result, self.current_step)

        return result

    def measure_all(
        self,
        *,
        name: str,
        values: list[float | int] | NDArray[np.float64] | pd.Series,
        bounds: dict[str, float] | NumericBounds,
        timestamp: datetime | None = None,
        unit: str | None = None,
    ) -> bool:
        """Ensure that all values in a list are within bounds and return the result. Records measurements for all values outside the bounds.

        Note: Measurements will only be recorded for values outside the bounds. To record measurements for all values, just call measure for each value.

        Args:
            name: The name of the measurement.
            values: The list of values to measure the average of.
            bounds: The bounds to compare the value to.
            timestamp: [Optional] The timestamp of the measurement. Defaults to the current time.
            unit: [Optional] The unit of the measurement.

        returns: The true if all values are within the bounds, false otherwise.
        """
        timestamp = timestamp if timestamp else datetime.now(timezone.utc)
        np_array = None
        if isinstance(values, list):
            np_array = np.array(values)
        elif isinstance(values, np.ndarray):
            np_array = values
        elif isinstance(values, pd.Series):
            np_array = values.to_numpy()
        else:
            raise ValueError(f"Invalid value type: {type(values)}")

        numeric_bounds = bounds
        if isinstance(numeric_bounds, dict):
            numeric_bounds = NumericBounds(min=bounds.get("min"), max=bounds.get("max"))  # type: ignore

        # Construct a mask of the values that are outside the bounds.
        mask = None
        if numeric_bounds.min is not None:
            mask = np_array < numeric_bounds.min
        if numeric_bounds.max is not None:
            val_above_max = np_array > numeric_bounds.max
            mask = mask | val_above_max if mask is not None else val_above_max
        if mask is None:
            raise ValueError("No bounds provided")

        rows_outside_bounds = np_array[mask]
        for row in rows_outside_bounds:
            self.measure(name=name, value=row, bounds=bounds, timestamp=timestamp, unit=unit)

        result = rows_outside_bounds.size == 0
        assert self.current_step is not None
        self.report_context.record_step_outcome(result, self.current_step)

        return result

    def report_outcome(self, name: str, result: bool, reason: str | None = None) -> bool:
        """Report an outcome from some action or measurement. Creates a substep that is pass/fail with the optional reason as the description.

        Args:
            name: The name of the substep.
            result: True if the action or measurement passed, False otherwise.
            reason: [Optional] The context to include in the description of the substep.

        returns: The given result so the function can be used in line.
        """
        with self.substep(name=name, description=reason) as substep:
            self.report_context.record_step_outcome(result, substep.current_step)
        return result

    def substep(self, name: str, description: str | None = None) -> NewStep:
        """Alias to return a new step context manager from the current step. The ReportContext will manage nesting of steps."""
        return self.report_context.new_step(
            name=name,
            description=description,
            assertion_as_fail_not_error=self.assertion_as_fail_not_error,
        )
