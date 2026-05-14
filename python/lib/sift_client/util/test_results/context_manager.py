from __future__ import annotations

import getpass
import logging
import os
import socket
import subprocess
import tempfile
import traceback
from contextlib import AbstractContextManager, contextmanager
from datetime import datetime, timezone
from pathlib import Path
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
    from sift_client.sift_types.channel import Channel

logger = logging.getLogger(__name__)


def log_replay_instructions(log_file: str | Path | None) -> None:
    """Log instructions for manually replaying a test result log file.

    Used when an import/replay attempt fails so the user can retry against the same file.
    """
    if log_file is None:
        return
    logger.error(
        f"Error replaying log file: {log_file}.\n"
        f"  Can replay with `replay-test-result-log {log_file}`."
    )


@contextmanager
def _quiet_fork_stderr():
    """Redirect fd 2 to /dev/null across a ``fork()`` to discard gRPC's prefork notices.

    Redirecting fd 2 at the fd level (``os.dup2``) is what gRPC's handlers actually
    write to, so wrapping a fork-site in this context manager reliably swallows those
    notices without touching gRPC's global state. Scope the ``with`` block as tightly
    as possible since it affects every thread in the process while active.
    """
    saved_fd = os.dup(2)
    devnull_fd = os.open(os.devnull, os.O_WRONLY)
    try:
        os.dup2(devnull_fd, 2)
        os.close(devnull_fd)
        yield
    finally:
        os.dup2(saved_fd, 2)
        os.close(saved_fd)


def _git_metadata() -> dict[str, str] | None:
    """Return git branch and commit hash, or None if not in a git repo."""
    try:
        with _quiet_fork_stderr():
            branch = subprocess.check_output(
                ["git", "rev-parse", "--abbrev-ref", "HEAD"],
                stderr=subprocess.DEVNULL,
                text=True,
            ).strip()
            commit = subprocess.check_output(
                ["git", "describe", "--always", "--dirty", "--exclude", "*"],
                stderr=subprocess.DEVNULL,
                text=True,
            ).strip()
            repo = subprocess.check_output(
                ["git", "remote", "get-url", "origin"],
                stderr=subprocess.DEVNULL,
                text=True,
            ).strip()
        return {"git_repo": repo, "git_branch": branch, "git_commit": commit}
    except Exception:
        return None


class ReportContext(AbstractContextManager):
    """Context manager for a new TestReport. See usage example in __init__.py."""

    report: TestReport
    client: SiftClient
    log_file: Path | None
    step_is_open: bool
    step_stack: list[TestStep]
    step_number_at_depth: dict[int, int]
    open_step_results: dict[str, bool]
    any_failures: bool
    _import_proc: subprocess.Popen | None = None

    def __init__(
        self,
        client: SiftClient,
        name: str,
        test_system_name: str | None = None,
        system_operator: str | None = None,
        test_case: str | None = None,
        log_file: str | Path | bool | None = None,
        include_git_metadata: bool = False,
    ):
        """Initialize a new report context.

        Args:
            client: The Sift client to use to create the report.
            name: The name of the report.
            test_system_name: The name of the test system. Will default to the hostname if not provided.
            system_operator: The operator of the test system. Will default to the current user if not provided.
            test_case: The name of the test case. Will default to the basename of the file containing the test if not provided.
            log_file: If True, create a temp log file. If a path, use that path.
                All create/update operations will be logged to this file.
            include_git_metadata: If True, include git metadata in the report.
        """
        self.client = client
        self.step_is_open = False
        self.step_stack = []
        self.step_number_at_depth = {}
        self.open_step_results = {}
        self.any_failures = False

        if log_file is True:
            tmp = tempfile.NamedTemporaryFile(suffix=".jsonl", delete=False)
            self.log_file = Path(tmp.name)
            logger.info(f"Created temporary log file: {self.log_file}")
        elif log_file:
            self.log_file = Path(log_file)
        else:
            self.log_file = None

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
            metadata=_git_metadata() if include_git_metadata else None,  # type: ignore
        )
        self.report = client.test_results.create(create, log_file=self.log_file)

    def _open_import_proc(self):
        """Open a subprocess to import the log file."""
        with _quiet_fork_stderr():
            self._import_proc = subprocess.Popen(
                [
                    "import-test-result-log",
                    "--incremental",
                    str(self.log_file),
                    "--grpc-url",
                    self.client.grpc_client._config.uri,
                    "--rest-url",
                    self.client.rest_client._config.base_url,
                    "--api-key",
                    self.client.grpc_client._config.api_key,
                ],
                stdin=subprocess.PIPE,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )

    def __enter__(self):
        if self.log_file:
            self._open_import_proc()
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

        if self._import_proc is not None:
            try:
                self._import_proc.communicate(timeout=1)
            except subprocess.TimeoutExpired:
                logger.error("Import process did not exit in 10s, killing it")
                self._import_proc.kill()
                self._import_proc.wait()
                log_replay_instructions(self.log_file)
                raise

        return True

    def new_step(
        self,
        name: str,
        description: str | None = None,
        assertion_as_fail_not_error: bool = True,
        metadata: dict[str, str | float | bool] | None = None,
    ) -> NewStep:
        """Alias to return a new step context manager from this report context. Use create_step for actually creating a TestStep in the current context."""
        return NewStep(
            self,
            name=name,
            description=description,
            assertion_as_fail_not_error=assertion_as_fail_not_error,
            metadata=metadata,
        )

    def get_next_step_path(self) -> str:
        """Get the next step path for the current depth."""
        top_step = self.step_stack[-1] if self.step_stack else None
        step_path = top_step.step_path if top_step else ""
        next_step_number = self.step_number_at_depth.get(len(self.step_stack), 0) + 1
        prefix = f"{step_path}." if step_path else ""
        return f"{prefix}{next_step_number}"

    def create_step(
        self,
        name: str,
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
    ) -> TestStep:
        """Create a new step in the report context.

        Args:
            name: The name of the step.
            description: The description of the step.
            metadata: [Optional] Structured key/value metadata to attach to the step. For
                metadata shared across every step in a report, prefer the `metadata` attribute
                of the enclosing `TestReport`.

        Returns:
            The created step.
        """
        step_path = self.get_next_step_path()
        parent_step = self.step_stack[-1] if self.step_stack else None

        step = self.client.test_results.create_step(
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
                metadata=metadata,
            ),
            log_file=self.log_file,
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
        metadata: dict[str, str | float | bool] | None = None,
    ):
        """Initialize a new step context.

        Args:
            report_context: The report context to create the step in.
            name: The name of the step.
            description: The description of the step.
            assertion_as_fail_not_error: Mark steps with assertion errors as failed instead of error+traceback (some users want assertions to work as simple failures especially when using pytest).
            metadata: [Optional] Structured key/value metadata to attach to the step.
        """
        self.report_context = report_context
        self.client = report_context.client
        self.current_step = self.report_context.create_step(name, description, metadata=metadata)
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
            },
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
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
        channel_names: list[str] | list[Channel] | None = None,
    ) -> bool:
        """Measure a value and return the result.

        Args:
            name: The name of the measurement.
            value: The value of the measurement.
            bounds: [Optional] The bounds to compare the value to.
            timestamp: [Optional] The timestamp of the measurement. Defaults to the current time.
            unit: [Optional] The unit of the measurement.
            description: [Optional] Notes about the measurement. Server caps at 2000 characters;
                longer strings are truncated with a warning.
            metadata: [Optional] Structured key/value metadata to attach to the measurement.
                For metadata shared across measurements, prefer the `metadata` attribute of the
                enclosing `TestStep` or `TestReport`.
            channel_names: [Optional] Sift channel names or `Channel` instances this measurement
                is associated with. Enables cross-plotting in Explore using the report's
                associated Run.

        returns: The result of the measurement.
        """
        assert self.current_step is not None
        create = TestMeasurementCreate(
            test_step_id=str(self.current_step.id_),
            name=name,
            passed=True,
            timestamp=timestamp if timestamp else datetime.now(timezone.utc),
            unit=unit,
            description=description,
            metadata=metadata,
            channel_names=channel_names,
        )
        evaluate_measurement_bounds(create, value, bounds)
        measurement = self.client.test_results.create_measurement(
            create, log_file=self.report_context.log_file
        )
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
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
        channel_names: list[str] | list[Channel] | None = None,
    ) -> bool:
        """Calculate the average of a list of values, measure the average against given bounds, and return the result.

        Args:
            name: The name of the measurement.
            values: The list of values to measure the average of.
            bounds: The bounds to compare the value to.
            timestamp: [Optional] The timestamp of the measurement. Defaults to the current time.
            unit: [Optional] The unit of the measurement.
            description: [Optional] Notes about the measurement. Server caps at 2000 characters;
                longer strings are truncated with a warning.
            metadata: [Optional] Structured key/value metadata to attach to the measurement.
            channel_names: [Optional] Sift channel names or `Channel` instances this measurement
                is associated with.

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
        result = self.measure(
            name=name,
            value=avg,
            bounds=bounds,
            timestamp=timestamp,
            unit=unit,
            description=description,
            metadata=metadata,
            channel_names=channel_names,
        )
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
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
        channel_names: list[str] | list[Channel] | None = None,
    ) -> bool:
        """Ensure that all values in a list are within bounds and return the result. Records measurements for all values outside the bounds.

        Note: Measurements will only be recorded for values outside the bounds. To record measurements for all values, just call measure for each value.

        Args:
            name: The name of the measurement.
            values: The list of values to measure the average of.
            bounds: The bounds to compare the value to.
            timestamp: [Optional] The timestamp of the measurement. Defaults to the current time.
            unit: [Optional] The unit of the measurement.
            description: [Optional] Notes attached to each out-of-bounds measurement. Server caps
                at 2000 characters; longer strings are truncated with a warning.
            metadata: [Optional] Structured key/value metadata for each out-of-bounds measurement.
            channel_names: [Optional] Sift channel names or `Channel` instances to associate with
                each out-of-bounds measurement.

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
            self.measure(
                name=name,
                value=row,
                bounds=bounds,
                timestamp=timestamp,
                unit=unit,
                description=description,
                metadata=metadata,
                channel_names=channel_names,
            )

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

    def substep(
        self,
        name: str,
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
    ) -> NewStep:
        """Alias to return a new step context manager from the current step. The ReportContext will manage nesting of steps."""
        return self.report_context.new_step(
            name=name,
            description=description,
            assertion_as_fail_not_error=self.assertion_as_fail_not_error,
            metadata=metadata,
        )
