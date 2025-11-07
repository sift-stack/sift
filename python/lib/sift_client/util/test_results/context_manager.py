from __future__ import annotations

import getpass
import os
import socket
import traceback
from contextlib import AbstractContextManager
from datetime import datetime, timezone
from typing import TYPE_CHECKING

from sift_client.sift_types.test_report import (
    ErrorInfo,
    NumericBounds,
    TestMeasurement,
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

    def new_step(self, name: str, description: str | None = None) -> NewStep:
        """Alias to return a new step context manager from this report context. Use create_step for actually creating a TestStep in the current context."""
        return NewStep(self, name=name, description=description)

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

    def report_measurement(self, measurement: TestMeasurement, step: TestStep):
        """Report a failure to the report context."""
        # Failures will be propogated when the step exits.
        if not measurement.passed:
            self.open_step_results[step.step_path] = False
            self.any_failures = True

    def resolve_and_propagate_step_result(
        self,
        step: TestStep,
        parent_step: TestStep | None = None,
        error_info: ErrorInfo | None = None,
    ) -> bool:
        """Resolve the result of a step and propagate the result to the parent step if it failed."""
        result = self.open_step_results.get(step.step_path, True)
        if error_info:
            result = False
        if step.status != TestStatus.IN_PROGRESS:
            # The step was manually completed so use that result.
            result = step.status == TestStatus.PASSED

        # Update the parent step results if this step failed (true by default so no need to do anything if we didn't fail).
        if not result:
            self.any_failures = True
            if parent_step:
                self.open_step_results[parent_step.step_path] = False

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
    current_step: TestStep | None = None
    parent_step: TestStep | None = None

    def __init__(
        self,
        report_context: ReportContext,
        name: str,
        description: str | None = None,
    ):
        """Initialize a new step context.

        Args:
            report_context: The report context to create the step in.
            name: The name of the step.
            description: The description of the step.
        """
        self.report_context = report_context
        self.client = report_context.report.client
        self.current_step = self.report_context.create_step(name, description)

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
    ):
        """Update the step based on its substeps and if there was an exception while executing the step.

        Args:
            exc: The class of Exception that was raised.
            exc_value: The exception value.
            tb: The traceback object.
        """
        error_info = None
        if exc:
            stack = traceback.format_exception(exc, exc_value, tb)  # type: ignore
            stack = [stack[0], *stack[-10:]] if len(stack) > 10 else stack
            trace = "".join(stack)
            error_info = ErrorInfo(
                error_code=1,
                error_message=trace,
            )
        assert self.current_step is not None

        # Resolve the status of this step (i.e. fail if children failed) and propagate the result to the parent step.
        result = self.report_context.resolve_and_propagate_step_result(
            self.current_step, self.parent_step, error_info
        )

        # Mark the step as completed
        status = self.current_step.status
        if status == TestStatus.IN_PROGRESS:
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

    def __exit__(self, exc, exc_value, tb):
        self.update_step_from_result(exc, exc_value, tb)

        # Now that the step is updated. Let the report context handle removing it from the stack and updating the report context.
        self.report_context.exit_step(self.current_step)

        return True

    def measure(
        self,
        *,
        name: str,
        value: float | str | bool,
        bounds: dict[str, float] | NumericBounds | str | None = None,
    ) -> bool:
        """Measure a value and return the result.

        returns: The measurement object.
        """
        assert self.current_step is not None
        create = TestMeasurementCreate(
            test_step_id=str(self.current_step.id_),
            name=name,
            passed=True,
            timestamp=datetime.now(timezone.utc),
        )
        evaluate_measurement_bounds(create, value, bounds)
        measurement = self.client.test_results.create_measurement(create)
        self.report_context.report_measurement(measurement, self.current_step)

        return measurement.passed

    def substep(self, name: str, description: str | None = None) -> NewStep:
        """Alias to return a new step context manager from the current step. The ReportContext will manage nesting of steps."""
        return self.report_context.new_step(name=name, description=description)
