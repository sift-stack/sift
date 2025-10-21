from __future__ import annotations

from contextlib import AbstractContextManager
import os
from datetime import datetime, timezone
from typing import TYPE_CHECKING, ClassVar

from sift_client.sift_types.test_report import (
    NumericBounds,
    TestMeasurementCreate,
    TestReport,
    TestReportCreate,
    TestStatus,
    TestStep,
    TestStepCreate,
    TestStepType,
)

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class ReportContext:
    """Context for a new TestReport. Mostly serves as a store to communicate between step context managers since they can be nested or siblings."""

    report: TestReport
    step_is_open: bool = False
    step_stack: ClassVar[list[(int, TestStep)]] = []
    open_step_results: ClassVar[dict[str, bool]] = {}
    any_failures: bool = False

    def __init__(self, report: TestReport):
        """Initialize a new report context.

        Args:
            report: The report to create the context for.
        """
        self.report = report

    @classmethod
    def create(
        cls, client: SiftClient, name: str, test_system_name: str, test_case: str | None = None
    ) -> ReportContext:
        """Create a new report context."""
        test_case = test_case if test_case else os.path.basename(__file__)

        create = TestReportCreate(
            name=name,
            test_system_name=test_system_name,
            test_case=test_case,
            start_time=datetime.now(timezone.utc),
            end_time=datetime.now(timezone.utc),
            status=TestStatus.IN_PROGRESS,
        )
        report = client.test_results.create(create)
        return cls(report)

    def new_step(self, name: str, description: str | None = None) -> NewStep:
        """Create a new step in the report context."""
        return NewStep(self, name=name, description=description)


class NewStep(AbstractContextManager):
    """Context manager to create a new step in a test report."""

    report_context: ReportContext
    client: SiftClient
    current_step: TestStep | None = None
    name: str | None = None
    step_path: str | None = None
    description: str | None = None
    parent_step: TestStep | None = None

    def __init__(
        self,
        report_context: ReportContext,
        name: str | None = None,
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
        self.name = name
        self.description = description
        self._update_step_stack()


    def __enter__(self):
        """Enter the context manager to create a new step.

        returns: The current step.
        """
        self.current_step = self.client.test_results.create_step(
            TestStepCreate(
                test_report_id=self.report_context.report.id_,
                name=self.name,
                step_type=TestStepType.ACTION,
                step_path=self.step_path,
                status=TestStatus.IN_PROGRESS,
                start_time=datetime.now(timezone.utc),
                end_time=datetime.now(timezone.utc),
                description=self.description,
                parent_step_id=self.parent_step.id_ if self.parent_step else None,
            )
        )
        self.report_context.step_stack.append((0, self.current_step))
        # Create an entry in the open step results for this step that can be modified by substeps/measurements.
        self.report_context.open_step_results[self.step_path] = True
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        result = self._resolve_and_propagate_result()

        # Mark the step as completed
        self.current_step.update(
            {
                "status": TestStatus.PASSED if result else TestStatus.FAILED,
                "end_time": datetime.now(timezone.utc),
            }
        )
        # Update the last step to the parent.
        _, stack_top = self.report_context.step_stack.pop()
        if stack_top.id_ != self.current_step.id_:
            raise ValueError(
                "The current step is not the top of the stack. This should never happen."
            )

    def _update_step_stack(self):
        """Update the step stack with the new step number if there is an existing stack."""
        step_number, parent_step = (
            self.report_context.step_stack[-1] if self.report_context.step_stack else (0, None)
        )
        step_number += 1
        prefix = f"{parent_step.step_path}." if parent_step else ""
        if parent_step:
            # Increment the step number in the stack.
            _, parent_step = self.report_context.step_stack.pop()
            self.report_context.step_stack.append((step_number, parent_step))
        self.step_path = f"{prefix}{step_number}"

    def _resolve_and_propagate_result(self) -> bool:
        """Get the result of the step from the report context and update the report context for the parent step if this step failed."""
        result = self.report_context.open_step_results.get(self.current_step.step_path, True)
        if self.current_step.status != TestStatus.IN_PROGRESS:
            # The step was not manually completed so use that.
            result = self.current_step.status == TestStatus.PASSED

        # Update the parent step results if this step failed (true by default so no need to do anything if we didn't fail).
        if self.parent_step and not result:
            parent_result = self.report_context.open_step_results.get(
                self.parent_step.step_path, True
            )
            self.report_context.open_step_results[self.parent_step.step_path] = (
                parent_result and result
            )
            self.report_context.any_failures = True

        # TODO: Cleanup the open step results for this step?

        return result

    def measure(
        self, *, name: str, value: float | str | bool, bounds: NumericBounds | str | None = None
    ) -> bool:
        """Measure a value and return the result.

        returns: The measurement object.
        """

        create = TestMeasurementCreate(
            test_step_id=self.current_step.id_,
            name=name,
            passed=True,
            timestamp=datetime.now(timezone.utc),
        )

        if bounds is not None:
            if isinstance(bounds, str):
                if not isinstance(value, str):
                    raise ValueError("Value must be a string if bounds provided is a string")
                create.string_expected_value = bounds
                create.string_value = value
                create.passed = value == bounds
            elif isinstance(bounds, NumericBounds):
                if not (isinstance(value, float) or isinstance(value, int)):
                    raise ValueError(
                        "Value must be a float or int if bounds provided are numeric bounds"
                    )
                create.numeric_bounds = bounds
                create.numeric_value = float(value)
                if create.numeric_value.min is not None:
                    create.passed = (
                        create.passed and create.numeric_value.min <= create.numeric_value
                    )
                if create.numeric_value.max is not None:
                    create.passed = (
                        create.passed and create.numeric_value.max >= create.numeric_value
                    )

        if not create.passed:
            # Propogate failures to the report context so the step will be marked correctly when it exists context.
            self.report_context.open_step_results[self.current_step.step_path] = False

        measurement = self.client.test_results.create_measurement(create)
        return measurement.passed
