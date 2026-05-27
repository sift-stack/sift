from __future__ import annotations

import getpass
import logging
import os
import socket
import subprocess
import tempfile
import traceback
import warnings
from collections import Counter
from contextlib import AbstractContextManager, contextmanager
from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING

import numpy as np

from sift_client.errors import SiftWarning
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
    out_of_bounds_mask,
    to_numpy_array,
)

if TYPE_CHECKING:
    import pandas as pd
    from numpy.typing import NDArray

    from sift_client.client import SiftClient
    from sift_client.sift_types.channel import Channel

logger = logging.getLogger(__name__)


def format_truncated_traceback(
    exc: type[BaseException] | None,
    exc_value: BaseException | None,
    tb: object | None,
) -> ErrorInfo:
    """Format an ErrorInfo from a traceback, keeping the first frame and the last 10."""
    stack = traceback.format_exception(exc, exc_value, tb)  # type: ignore[arg-type]
    stack = [stack[0], *stack[-10:]] if len(stack) > 10 else stack
    return ErrorInfo(error_code=1, error_message="".join(stack))


def format_assertion_message(
    exc: type[BaseException] | None,
    exc_value: BaseException | None,
) -> ErrorInfo:
    """Format an ErrorInfo from just the exception line(s), no traceback frames.

    For assertion failures the rewritten ``assert`` explanation lives on the
    exception itself, so stack frames add noise without information. Equivalent
    to pytest's ``excinfo.exconly()``.
    """
    lines = traceback.format_exception_only(exc, exc_value)  # type: ignore[arg-type]
    return ErrorInfo(error_code=1, error_message="".join(lines))


def log_replay_instructions(log_file: str | Path | None) -> None:
    """Surface replay instructions when an import/replay attempt fails.

    Emitted as a ``SiftWarning`` (not a logger.error) so pytest and other
    runners surface it in their warning summary; logger.error is suppressed
    by default in most CLI tools.
    """
    if log_file is None:
        return
    warnings.warn(
        f"Sift log file was not fully replayed: {log_file}. "
        f"Re-run with `import-test-result-log {log_file}` to complete the upload.",
        SiftWarning,
        stacklevel=2,
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
    # Every step created in this report (including hierarchy/parametrize
    # parents), retained after close so end-of-run summaries can tally final
    # statuses. ``update`` mutates step instances in place, so these references
    # reflect late status changes (e.g. a teardown-phase failure).
    created_steps: list[TestStep]
    # Every measurement recorded in this report, retained for end-of-run
    # summaries. Appended in ``NewStep.measure``. A measurement's ``passed`` is
    # fixed at creation, so the retained references stay accurate.
    created_measurements: list[TestMeasurement]
    # Set True in ``__exit__`` when the background replay worker timed out or
    # exited non-zero, so callers (e.g. the pytest plugin footer) can flag that
    # the uploaded report may be missing entries.
    replay_incomplete: bool = False
    _import_proc: subprocess.Popen | None = None
    # Seconds to wait for the import worker subprocess to finish uploading
    # the JSONL backlog at session end before killing it. Tests substitute
    # a smaller value (via ``_make_context`` patching) so they don't wait
    # the full window for the timeout branch to trigger.
    _import_proc_timeout: float = 30.0

    def __init__(
        self,
        client: SiftClient,
        name: str,
        test_system_name: str | None = None,
        system_operator: str | None = None,
        test_case: str | None = None,
        log_file: str | Path | bool | None = None,
        include_git_metadata: bool = False,
        replay_log_file: bool = True,
        metadata: dict[str, str | float | bool] | None = None,
    ):
        """Initialize a new report context.

        Args:
            client: The Sift client to use to create the report.
            name: The name of the report.
            test_system_name: The name of the test system. Will default to the hostname if not provided.
            system_operator: The operator of the test system. Will default to the current user if not provided.
            test_case: The name of the test case. Will default to the basename of the file containing the test if not provided.
            log_file: If True, create a temp log file. If a path, use that path.
                If False/None, no log file is written and create/update calls
                the API.
            include_git_metadata: If True, include git metadata in the report.
            metadata: Structured key/value metadata to attach to the report. Merged
                on top of git metadata when ``include_git_metadata`` is True, so
                explicit keys win on collision.
            replay_log_file: When True (the default) and ``log_file`` is set,
                spawn ``import-test-result-log --incremental`` to push log
                entries to Sift in the background during the session. When
                False, the log file is just a record and no worker is spawned.
                Replay happens later via ``replay-test-result-log <path>``.
                Has no effect when ``log_file`` is None.
        """
        self.client = client
        self.replay_log_file = replay_log_file
        self.step_is_open = False
        self.step_stack = []
        self.step_number_at_depth = {}
        self.open_step_results = {}
        self.any_failures = False
        self.created_steps = []
        self.created_measurements = []
        self.replay_incomplete = False

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
        combined_metadata = {
            **(_git_metadata() or {} if include_git_metadata else {}),
            **(metadata or {}),
        }
        create = TestReportCreate(
            name=name,
            test_system_name=test_system_name,
            test_case=test_case,
            start_time=datetime.now(timezone.utc),
            end_time=datetime.now(timezone.utc),
            status=TestStatus.IN_PROGRESS,
            system_operator=system_operator,
            metadata=combined_metadata or None,  # type: ignore
        )
        self.report = client.test_results.create(create, log_file=self.log_file)

    def _build_replay_command(self) -> list[str]:
        """Build the argv for the import-test-result-log replay subprocess.

        Factored out for testability — tests substitute commands that exit
        with controlled returncodes / stderr to exercise the ``__exit__``
        branches without depending on the real replay binary.
        """
        return [
            "import-test-result-log",
            "--incremental",
            str(self.log_file),
            "--grpc-url",
            self.client.grpc_client._config.uri,
            "--rest-url",
            self.client.rest_client._config.base_url,
            "--api-key",
            self.client.grpc_client._config.api_key,
        ]

    def _open_import_proc(self):
        """Open a subprocess to import the log file.

        ``stderr`` is captured so a worker crash mid-session can surface its
        error at session end via ``__exit__`` rather than failing silently.
        """
        with _quiet_fork_stderr():
            self._import_proc = subprocess.Popen(
                self._build_replay_command(),
                stdin=subprocess.PIPE,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.PIPE,
            )

    def __enter__(self):
        if self.log_file and self.replay_log_file:
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
            # Three outcomes for the replay worker at session end. None of
            # them fail the session — tests already ran and their outcome
            # is independent of delivery. The local log file is the source
            # of recovery for both failure modes via
            # `import-test-result-log <path>`:
            #   1. Exits cleanly (returncode 0). Silent.
            #   2. Still running after the grace window (TimeoutExpired).
            #      Healthy worker with a large backlog; kill and surface
            #      replay instructions. 30 seconds is enough for a normal
            #      test suite to drain; pathological backlogs should opt
            #      into inline mode (`--sift-log-file=false`) instead.
            #   3. Exited with non-zero. Connection failures and API call
            #      errors land here — the worker's replay loop has no retry,
            #      so the first failed RPC crashes the subprocess. Surface
            #      the captured stderr with replay instructions.
            try:
                _, stderr_bytes = self._import_proc.communicate(timeout=self._import_proc_timeout)
            except subprocess.TimeoutExpired:
                self._import_proc.kill()
                self._import_proc.wait()
                self.replay_incomplete = True
                warnings.warn(
                    f"Sift import worker did not exit in "
                    f"{self._import_proc_timeout}s; killing it. "
                    "Local log file is preserved for manual replay.",
                    SiftWarning,
                    stacklevel=2,
                )
                log_replay_instructions(self.log_file)
                return True  # Ensures the session is marked as passed in pytest
            if self._import_proc.returncode != 0:
                self.replay_incomplete = True
                stderr_text = (
                    stderr_bytes.decode("utf-8", errors="replace").strip() if stderr_bytes else ""
                )
                warnings.warn(
                    f"Sift import worker exited with code "
                    f"{self._import_proc.returncode}. stderr: {stderr_text or '<empty>'}",
                    SiftWarning,
                    stacklevel=2,
                )
                log_replay_instructions(self.log_file)

        return True

    @property
    def is_simulated(self) -> bool:
        """True when this context's report came from the simulate path.

        Delegates to ``self.report.is_simulated``; see ``TestReport.is_simulated``
        for the full semantics.
        """
        return self.report.is_simulated

    @property
    def step_status_counts(self) -> Counter[TestStatus]:
        """Tally of every created step by its current status.

        Includes hierarchy/parametrize parent steps. Read at the end of a run for
        summaries; reflects late status changes since steps are mutated in place.
        """
        return Counter(step.status for step in self.created_steps)

    @property
    def measurement_counts(self) -> Counter[bool]:
        """Tally of recorded measurements keyed by ``passed`` (True/False).

        Read at the end of a run for summaries.
        """
        return Counter(m.passed for m in self.created_measurements)

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
        # Retained for end-of-run tallies; never popped (unlike step_stack).
        self.created_steps.append(step)

        return step

    def record_step_outcome(self, outcome: bool, step: TestStep):
        """Report a failure to the report context."""
        # Failures will be propogated when the step exits.
        if not outcome:
            self.open_step_results[step.step_path] = False
            self.any_failures = True

    def record_measurement(self, measurement: TestMeasurement) -> None:
        """Retain a recorded measurement for end-of-run summaries."""
        self.created_measurements.append(measurement)

    def mark_step_failed_after_close(self, step: TestStep):
        """Mark a step's parent as failed after the step has already been popped from the stack.

        Used by the pytest plugin when a teardown-phase report fires after the
        fixture's ``__exit__`` has already resolved and exited the step.
        """
        self.any_failures = True
        path_parts = step.step_path.split(".")
        if len(path_parts) > 1:
            self.open_step_results[".".join(path_parts[:-1])] = False

    def propagate_step_result(self, step: TestStep, status: TestStatus) -> bool:
        """Propagate this step's final status to the parent step.

        Status is the governor: anything outside ``{PASSED, SKIPPED}`` counts
        as a failure for the parent. ``error_info`` is intentionally not
        consulted here; it is free-form diagnostic data that may sit on a
        step regardless of status.
        """
        succeeded = status in (TestStatus.PASSED, TestStatus.SKIPPED)
        if not succeeded:
            self.any_failures = True
            self.open_step_results[step.step_path] = False
            path_parts = step.step_path.split(".")
            if len(path_parts) > 1:
                self.open_step_results[".".join(path_parts[:-1])] = False
        return succeeded

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
    # Set by the pytest plugin's ``_resolve_initial_status`` to signal that
    # status was already resolved upstream and ``__exit__`` should skip
    # re-classifying. Read via ``getattr`` so unset is treated as False.
    _sift_managed_externally: bool = False

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
        # Per-step measurement-failure count for ``measurements_passed``.
        # Tracks only direct ``measure*`` calls on this NewStep instance;
        # substep / ``report_outcome`` failures are intentionally not folded
        # in here (see ``measurements_passed`` vs ``passed``).
        self._failed_measurement_count = 0
        # Out-of-bounds measurements recorded on this step, retained so
        # ``fail_if_measurements_failed`` can name them in the failure message.
        self._failed_measurements: list[TestMeasurement] = []

    def __enter__(self):
        """Enter the context manager to create a new step.

        returns: The current step.
        """
        return self

    @property
    def measurements_passed(self) -> bool:
        """True if every measurement recorded directly on this step has passed.

        Counts only ``step.measure``, ``step.measure_avg``, and
        ``step.measure_all`` calls on this ``NewStep`` instance. Pair it with
        ``fail_if_measurements_failed()`` at the end of a test to fail pytest on
        any out-of-bounds measurement without short-circuiting on the first
        failure (asserting on individual ``measure(...)`` return values skips
        every measurement after the failing one).
        """
        return self._failed_measurement_count == 0

    def fail_if_measurements_failed(self, message: str = "measurements out of bounds") -> None:
        """Fail the pytest test if any measurement on this step was out of bounds.

        Use instead of ``assert step.measurements_passed``: it fails via
        ``pytest.fail`` so the step resolves to FAILED without attaching an
        assertion message to ``error_info``. No-op when every measurement
        passed. Call once at the end of the test so every measurement is still
        recorded before the failure fires.

        The failure message names each out-of-bounds measurement with its
        recorded value and bounds. ``message`` is used as the header line.
        """
        if self.measurements_passed:
            return
        import pytest

        failed = self._failed_measurements
        header = f"{message} ({len(failed)}):" if failed else message
        body = [f"  - {m}" for m in failed]
        pytest.fail("\n".join([header, *body]), pytrace=False)

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
        current_step = self.current_step
        if current_step is None:
            # The step was never opened; nothing to resolve. Treat as a pass
            # so callers that branch on the return value don't see a spurious
            # failure.
            return True

        error_info = None
        aborted = False
        errored = False
        if exc:
            if isinstance(exc_value, AssertionError) and not self.assertion_as_fail_not_error:
                # pytest-style: an assertion is a plain failure, not an error. Record the
                # failure and attach the concise assertion message (no traceback) so the
                # UI can show what was asserted.
                self.report_context.record_step_outcome(False, current_step)
                error_info = format_assertion_message(exc, exc_value)
            elif isinstance(exc_value, (KeyboardInterrupt, SystemExit)):
                # Hard exit propagating through the substep stack: record as
                # ABORTED so every in-progress step on the way out reflects
                # the abort rather than coercing to ERROR.
                aborted = True
                error_info = format_truncated_traceback(exc, exc_value, tb)
            else:
                errored = True
                error_info = format_truncated_traceback(exc, exc_value, tb)

        # Status is the governor: anything other than IN_PROGRESS was set
        # deliberately (manual override, plugin pre-resolution, etc.) and must
        # not be silently overwritten by side-channel signals. When the step is
        # still IN_PROGRESS, resolve from independent state: aborts first, then
        # a child-failed signal (parents inherit FAILED, not the originating
        # ERROR), then the step's own captured exception, then the children-pass
        # default. error_info is diagnostic and never drives status.
        status = current_step.status
        if status == TestStatus.IN_PROGRESS:
            children_passed = self.report_context.open_step_results.get(
                current_step.step_path, True
            )
            if aborted:
                status = TestStatus.ABORTED
            elif not children_passed:
                status = TestStatus.FAILED
            elif errored:
                status = TestStatus.ERROR
            else:
                status = TestStatus.PASSED

        # Propagate based on the resolved status; error_info rides along as
        # pure diagnostics and does not affect propagation.
        result = self.report_context.propagate_step_result(current_step, status)
        current_step.update(
            {
                "status": status,
                "end_time": datetime.now(timezone.utc),
                "error_info": error_info,
            },
        )

        return result

    def __exit__(self, exc, exc_value, tb):
        if getattr(self, "_sift_managed_externally", False):
            # The pytest fixture already resolved status from phase reports.
            # Propagate based on that resolved status, emit one update_step
            # with the resolved values, and pop from the stack without
            # re-classifying.
            current_step = self.current_step
            if current_step is None:
                # The step was never opened; nothing to propagate.
                return True
            result = self.report_context.propagate_step_result(current_step, current_step.status)
            current_step.update(
                {
                    "status": current_step.status,
                    "end_time": datetime.now(timezone.utc),
                    "error_info": current_step.error_info,
                },
            )
            self.report_context.exit_step(current_step)
            if hasattr(self, "force_result"):
                result = self.force_result
            return result

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
        self.report_context.record_measurement(measurement)
        if not measurement.passed:
            self._failed_measurement_count += 1
            self._failed_measurements.append(measurement)

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
        np_array = to_numpy_array(values)
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
        np_array = to_numpy_array(values)
        rows_outside_bounds = np_array[out_of_bounds_mask(np_array, bounds)]
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
