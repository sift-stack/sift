from __future__ import annotations

import getpass
import logging
import os
import socket
import subprocess
import traceback
import warnings
from collections import Counter
from contextlib import AbstractContextManager, contextmanager
from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING

import numpy as np

from sift_client._internal.pytest_plugin.audit_log import _make_session_dir, log_event
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

# Sentinel for ``create_step``/``new_step``'s ``parent`` argument. Distinguishes
# "parent omitted -> use the top of the step stack" (the default, linear
# behavior) from an explicit ``parent=None`` (create at the report root). The
# pytest plugin passes an explicit parent to build its report tree out of
# execution order; everyday ``new_step``/``substep`` callers omit it.
_USE_STACK_TOP = object()


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
        f"Re-run with `import-test-result-log --incremental {log_file}` to complete the upload.",
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


def _is_session_exit(exc_value: BaseException | None) -> bool:
    """True for pytest's session-stopping ``Exit`` (``pytest.exit`` / sift ``abort``).

    Matched by type name and module so this framework-agnostic module needs no
    pytest import. ``SystemExit`` and ``KeyboardInterrupt`` are handled by their
    own ``isinstance`` checks at the call site.
    """
    cls = type(exc_value)
    return cls.__name__ == "Exit" and cls.__module__ == "_pytest.outcomes"


class ReportContext(AbstractContextManager):
    """Context manager for a new TestReport. See usage example in __init__.py."""

    report: TestReport
    client: SiftClient
    log_file: Path | None
    step_is_open: bool
    step_stack: list[TestStep]
    # Per-parent child counter keyed by the parent's ``step_path`` (``""`` is the
    # root bucket). Drives parent-relative path numbering so two parents at the
    # same depth never collide and a step's path is stable regardless of the
    # order siblings are created in.
    child_counts: dict[str, int]
    open_step_results: dict[str, bool]
    # Latest child ``end_time`` seen for each parent, keyed by the parent's
    # ``step_path``. A parent that stays open across the whole run (e.g. a
    # hierarchy/parametrize parent the pytest plugin holds in its registry) is
    # closed with this time, so its duration spans first-child-start to
    # last-descendant-finish rather than wall-clock at session end.
    parent_end_times: dict[str, datetime]
    any_failures: bool
    # Set when the run was stopped as a system-level abort (Ctrl-C, or the
    # pytest plugin's ``abort()`` helper) rather than a failure. Parents and the
    # report that close out-of-band during the unwind then resolve to ABORTED
    # instead of FAILED. A plain ``pytest.exit()`` leaves this False, so it stays
    # in the FAILED bucket.
    session_aborted: bool
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
    # When set, the path of the DEBUG audit log. The replay worker is spawned
    # with ``--audit-log <sibling>`` so its activity is traced too.
    audit_log: Path | None = None
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
        serial_number: str | None = None,
        part_number: str | None = None,
        log_file: str | Path | bool | None = None,
        include_git_metadata: bool = False,
        replay_log_file: bool = True,
        metadata: dict[str, str | float | bool] | None = None,
        audit_log: str | Path | None = None,
    ):
        """Initialize a new report context.

        Args:
            client: The Sift client to use to create the report.
            name: The name of the report.
            test_system_name: The name of the test system. Will default to the hostname if not provided.
            system_operator: The operator of the test system. Will default to the current user if not provided.
            test_case: The name of the test case. Will default to the basename of the file containing the test if not provided.
            serial_number: Optional serial_number stored on the report. Unset when None.
            part_number: Optional part_number stored on the report. Unset when None.
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
            audit_log: When set, the path of a DEBUG audit log. The replay worker
                is spawned with ``--audit-log <sibling>`` so its activity is
                traced to ``<path>.replay.log``.
        """
        self.client = client
        self.replay_log_file = replay_log_file
        self.audit_log = Path(audit_log) if audit_log is not None else None
        self.step_is_open = False
        self.step_stack = []
        self.child_counts = {}
        self.open_step_results = {}
        self.parent_end_times = {}
        self.any_failures = False
        self.session_aborted = False
        self.created_steps = []
        self.created_measurements = []
        self.replay_incomplete = False

        if log_file is True:
            session_dir = _make_session_dir()
            self.log_file = session_dir / f"{session_dir.name}.jsonl"
            log_event(logger, logging.INFO, "log_file.create", path=self.log_file, source="temp")
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
            serial_number=serial_number,
            part_number=part_number,
            metadata=combined_metadata or None,  # type: ignore
        )
        self.report = client.test_results.create(create, log_file=self.log_file)

    def _build_replay_command(self) -> list[str]:
        """Build the argv for the import-test-result-log replay subprocess.

        Factored out for testability — tests substitute commands that exit
        with controlled returncodes / stderr to exercise the ``__exit__``
        branches without depending on the real replay binary.
        """
        cmd = [
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
        if self.audit_log is not None:
            from sift_client._internal.pytest_plugin.audit_log import replay_audit_path

            cmd += ["--audit-log", str(replay_audit_path(self.audit_log))]
        return cmd

    def _open_import_proc(self):
        """Open a subprocess to import the log file.

        ``stderr`` is captured so a worker crash mid-session can surface its
        error at session end via ``__exit__`` rather than failing silently.
        """
        # Redact the API key before logging — never log the raw argv.
        api_key = self.client.grpc_client._config.api_key
        safe = ["***" if a == api_key else a for a in self._build_replay_command()]
        log_event(logger, logging.INFO, "replay.start", log=self.log_file)
        log_event(logger, logging.DEBUG, "replay.cmd", argv=" ".join(safe))
        try:
            with _quiet_fork_stderr():
                self._import_proc = subprocess.Popen(
                    self._build_replay_command(),
                    stdin=subprocess.PIPE,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.PIPE,
                )
        except OSError as exc:
            # e.g. the ``import-test-result-log`` entry point isn't on PATH.
            # Surface it; the JSONL log is still on disk for a manual replay.
            log_event(
                logger, logging.WARNING, "replay.spawn_failed", error=repr(exc), log=self.log_file
            )
            raise

    def __enter__(self):
        if self.log_file and self.replay_log_file:
            self._open_import_proc()
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        update = {
            "end_time": datetime.now(timezone.utc),
        }
        if self.session_aborted:
            update["status"] = TestStatus.ABORTED
        elif self.any_failures or exc_type:
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
            #      into inline mode (`--no-sift-log-file`) instead.
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
                log_event(
                    logger,
                    logging.WARNING,
                    "replay.timeout",
                    secs=self._import_proc_timeout,
                    log=self.log_file,
                )
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
                log_event(
                    logger,
                    logging.WARNING,
                    "replay.error",
                    code=self._import_proc.returncode,
                    log=self.log_file,
                    stderr=stderr_text or "",
                )
                warnings.warn(
                    f"Sift import worker exited with code "
                    f"{self._import_proc.returncode}. stderr: {stderr_text or '<empty>'}",
                    SiftWarning,
                    stacklevel=2,
                )
                log_replay_instructions(self.log_file)
            else:
                log_event(logger, logging.INFO, "replay.done")

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
        *,
        parent: TestStep | None | object = _USE_STACK_TOP,
        push: bool = True,
        origin: str = "step",
        source_path: str | None = None,
    ) -> NewStep:
        """Alias to return a new step context manager from this report context. Use create_step for actually creating a TestStep in the current context.

        ``parent`` and ``push`` default to the linear, stack-based behavior used
        by everyday callers. The pytest plugin passes an explicit ``parent`` with
        ``push=False`` to open report-tree parents that persist outside the stack;
        see :meth:`create_step`.

        ``origin`` (e.g. ``hierarchy``/``parametrize``/``test``/``substep``) and
        ``source_path`` (the pytest nodeid the step was created for) are
        audit-log labels only; they do not affect step creation.
        """
        return NewStep(
            self,
            name=name,
            description=description,
            assertion_as_fail_not_error=assertion_as_fail_not_error,
            metadata=metadata,
            parent=parent,
            push=push,
            origin=origin,
            source_path=source_path,
        )

    def _resolve_parent(self, parent: TestStep | None | object) -> TestStep | None:
        """Resolve a ``parent`` argument to a concrete parent step (or None for root)."""
        if parent is _USE_STACK_TOP:
            return self.step_stack[-1] if self.step_stack else None
        return parent  # type: ignore[return-value]

    def get_next_step_path(self, parent: TestStep | None | object = _USE_STACK_TOP) -> str:
        """Preview the path the next step under ``parent`` would get (no side effects).

        Parent-relative: a child's path is ``<parent path>.<nth child>``, or
        ``<n>`` at the root. Defaults to the top of the step stack so existing
        callers see the same value the next stacked ``create_step`` will assign.
        """
        parent_step = self._resolve_parent(parent)
        parent_path = parent_step.step_path if parent_step else ""
        next_number = self.child_counts.get(parent_path, 0) + 1
        return f"{parent_path}.{next_number}" if parent_path else str(next_number)

    def create_step(
        self,
        name: str,
        description: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
        *,
        parent: TestStep | None | object = _USE_STACK_TOP,
        push: bool = True,
    ) -> TestStep:
        """Create a new step in the report context.

        Args:
            name: The name of the step.
            description: The description of the step.
            metadata: [Optional] Structured key/value metadata to attach to the step. For
                metadata shared across every step in a report, prefer the `metadata` attribute
                of the enclosing `TestReport`.
            parent: The parent step to nest under. ``_USE_STACK_TOP`` (the
                default) parents to the current top of the step stack — the
                linear behavior. An explicit ``TestStep`` parents under that step
                regardless of stack state; explicit ``None`` creates a root step.
            push: Whether to push the new step onto the step stack. True (the
                default) for leaf/in-test steps so their substeps nest under
                them. The pytest plugin passes False for hierarchy/parametrize
                parents, which live in its own registry and would otherwise
                trap unrelated steps beneath them.

        Returns:
            The created step.
        """
        parent_step = self._resolve_parent(parent)
        parent_path = parent_step.step_path if parent_step else ""
        next_number = self.child_counts.get(parent_path, 0) + 1
        step_path = f"{parent_path}.{next_number}" if parent_path else str(next_number)

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
        self.child_counts[parent_path] = next_number
        if push:
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
            # Diagnostic: a failure recorded on this step's OWN scope (a failing
            # report_outcome or an out-of-bounds measure), as distinct from one
            # inherited from a child (which shows up as a step.propagate line).
            # Lets a reader tell why a later step.resolve marks the step failed.
            log_event(
                logger,
                logging.DEBUG,
                "step.outcome",
                step_path=step.step_path,
                result="fail",
            )

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
            parent_path = ".".join(path_parts[:-1])
            self.open_step_results[parent_path] = False
            # Diagnostic: a teardown-phase failure fires after the step's own
            # __exit__ has resolved, so it never runs through
            # propagate_step_result. Log the parent roll-up here so it is traced
            # like every other failure that reaches a parent.
            log_event(
                logger,
                logging.DEBUG,
                "step.propagate",
                step_path=step.step_path,
                status=step.status.name,
                signal="teardown_fail",
                parent=parent_path,
            )

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
                parent_path = ".".join(path_parts[:-1])
                self.open_step_results[parent_path] = False
                # Diagnostic: record the child→parent roll-up so the parent's
                # eventual ``cause=child_failed`` can be traced back to the step
                # that triggered it. The parent inherits a not-pass signal only;
                # ABORTED stays on the step in whose scope the exit fired (and the
                # substeps the exception unwinds through), so an aborted child
                # still rolls up to its container parent as FAILED. ``signal``
                # records the child's own terminal kind for the trace.
                log_event(
                    logger,
                    logging.DEBUG,
                    "step.propagate",
                    step_path=step.step_path,
                    status=status.name,
                    signal="abort" if status == TestStatus.ABORTED else "fail",
                    parent=parent_path,
                )
        return succeeded

    def note_close(self, step: TestStep) -> None:
        """Record a just-closed step's ``end_time`` against its parent.

        Lets a long-lived parent (one closed later, out of band) adopt the finish
        time of its latest child instead of wall-clock at its own close. Keyed by
        the parent's ``step_path`` (the child path minus its last segment).
        """
        end_time = step.end_time
        if end_time is None:
            return
        path_parts = step.step_path.split(".")
        if len(path_parts) <= 1:
            return
        parent_path = ".".join(path_parts[:-1])
        previous = self.parent_end_times.get(parent_path)
        if previous is None or end_time > previous:
            self.parent_end_times[parent_path] = end_time

    def exit_step(self, step: TestStep):
        """Exit a step and update the report context.

        Stacked steps (leaves and their in-test substeps) close in strict LIFO
        order, so a step that isn't the current top of the stack is a real
        invariant break. Steps created with an explicit parent and ``push=False``
        (the pytest plugin's hierarchy/parametrize parents) never sit on the
        stack and may close in any order — clearing ``open_step_results`` is all
        that's needed; their result was already propagated to their own parent.
        """
        self.open_step_results.pop(step.step_path, None)
        if self.step_stack and self.step_stack[-1].id_ == step.id_:
            self.step_stack.pop()
            return
        if any(s.id_ == step.id_ for s in self.step_stack):
            raise ValueError(
                "exit_step called out of LIFO order for a stacked step. This should never happen."
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
    # Set by the pytest plugin when finalizing a long-lived parent so ``__exit__``
    # stamps its last-descendant finish time instead of wall-clock at close.
    _sift_end_time_override: datetime | None = None

    def __init__(
        self,
        report_context: ReportContext,
        name: str,
        description: str | None = None,
        assertion_as_fail_not_error: bool = True,
        metadata: dict[str, str | float | bool] | None = None,
        *,
        parent: TestStep | None | object = _USE_STACK_TOP,
        push: bool = True,
        origin: str = "step",
        source_path: str | None = None,
    ):
        """Initialize a new step context.

        Args:
            report_context: The report context to create the step in.
            name: The name of the step.
            description: The description of the step.
            assertion_as_fail_not_error: Mark steps with assertion errors as failed instead of error+traceback (some users want assertions to work as simple failures especially when using pytest).
            metadata: [Optional] Structured key/value metadata to attach to the step.
            parent: Parent step to nest under; see :meth:`ReportContext.create_step`.
            push: Whether the step joins the step stack; see :meth:`ReportContext.create_step`.
            origin: Audit-log label for where the step came from (hierarchy,
                parametrize, test, substep); does not affect behavior.
            source_path: Audit-log label: the pytest nodeid this step was created
                for; does not affect behavior.
        """
        self.report_context = report_context
        self.client = report_context.client
        self.current_step = self.report_context.create_step(
            name, description, metadata=metadata, parent=parent, push=push
        )
        self.assertion_as_fail_not_error = assertion_as_fail_not_error
        self._origin = origin
        self._source_path = source_path
        # Per-step measurement-failure count for ``measurements_passed``.
        # Tracks only direct ``measure*`` calls on this NewStep instance;
        # substep / ``report_outcome`` failures are intentionally not folded
        # in here. ``pytest_fail_if_step_failed`` covers the broader case.
        self._failed_measurement_count = 0
        # Out-of-bounds measurements recorded on this step, retained so
        # ``pytest_fail_if_step_failed`` can name them in the failure message.
        self._failed_measurements: list[TestMeasurement] = []
        parent_path = self.current_step.step_path.rpartition(".")[0] or "-"
        log_event(
            logger,
            logging.DEBUG,
            "step.open",
            name=self.current_step.name,
            path=source_path or "-",
            step_path=self.current_step.step_path,
            origin=origin,
            parent=parent_path,
        )

    def __enter__(self):
        """Enter the context manager to create a new step.

        returns: The current step.
        """
        return self

    @property
    def measurements_passed(self) -> bool:
        """True if every measurement recorded directly on this step has passed.

        Counts only ``step.measure``, ``step.measure_avg``, and
        ``step.measure_all`` calls on this ``NewStep`` instance; substep and
        ``report_outcome`` failures are not folded in. For the end-of-test
        failure that mirrors the report, use ``pytest_fail_if_step_failed()``,
        which also covers failed substeps.
        """
        return self._failed_measurement_count == 0

    def pytest_fail_if_step_failed(self, message: str = "step failed") -> None:
        """Fail the running pytest test if this step or any descendant failed.

        Covers every signal that resolves the step to FAILED in the report:
        out-of-bounds measurements recorded directly on the step, failed
        substeps, and ``report_outcome`` failures. Call it once at the end of a
        test so the pytest verdict matches the report instead of passing green
        while the report shows a failure.

        It fails via ``pytest.fail(pytrace=False)`` so the step resolves to
        FAILED without an assertion traceback in ``error_info``. No-op when the
        step and all of its descendants passed. Call after the work is done so
        every measurement and substep is recorded before the failure fires.

        The failure message names each out-of-bounds measurement and each
        failed substep. ``message`` is used as the header line.
        """
        step = self.current_step
        # ``open_step_results[step_path]`` is the same signal ``__exit__`` reads
        # to resolve status: it is flipped False by a direct measurement failure
        # (record_step_outcome) and by any failed child as it propagates upward
        # (propagate_step_result). Default True covers a step that never opened.
        if step is None or self.report_context.open_step_results.get(step.step_path, True):
            return
        import pytest

        prefix = f"{step.step_path}."
        failed_substeps = [
            s
            for s in self.report_context.created_steps
            if s.step_path.startswith(prefix)
            and s.status not in (TestStatus.PASSED, TestStatus.SKIPPED, TestStatus.IN_PROGRESS)
        ]
        details = [f"  - measurement {m}" for m in self._failed_measurements]
        details += [f"  - substep {s.step_path!r}: {s.status.name}" for s in failed_substeps]
        header = f"{message} ({len(details)}):" if details else message
        pytest.fail("\n".join([header, *details]), pytrace=False)

    def _log_resolve(self, step: TestStep, status: TestStatus, cause: str) -> None:
        """Emit the ``step.resolve`` audit line: how this step's status was derived.

        For ``cause=child_failed`` the failure bucket is split at log time into
        ``measurement_failed`` (a failing measure on this step), ``own_failure``
        (a failing report_outcome / manual record_step_outcome, which are not
        distinguishable), or ``child_failed`` (a descendant failed, with ``from=``
        naming the direct children and their statuses). Other causes log as-is.
        The whole thing is guarded so the child scan only runs when audit is on.
        """
        if not logger.isEnabledFor(logging.DEBUG):
            return
        fields: dict[str, object] = {
            "name": step.name,
            "step_path": step.step_path,
            "status": status.name,
        }
        if cause == "child_failed":
            prefix = step.step_path + "."
            child_depth = step.step_path.count(".") + 1
            contributors = [
                f"{s.step_path}:{s.status.name}"
                for s in self.report_context.created_steps
                if s.step_path.startswith(prefix)
                and s.step_path.count(".") == child_depth
                and s.status not in (TestStatus.PASSED, TestStatus.SKIPPED, TestStatus.IN_PROGRESS)
            ]
            if contributors:
                fields["from"] = ",".join(contributors)
            elif self._failed_measurement_count > 0:
                cause = "measurement_failed"
            else:
                cause = "own_failure"
        fields["cause"] = cause
        log_event(logger, logging.DEBUG, "step.resolve", **fields)

    def _update_step_from_result(
        self,
        exc: type[Exception] | None,
        exc_value: Exception | None,
        tb: traceback.TracebackException | None,
        end_time: datetime | None = None,
    ) -> bool:
        """Update the step based on its substeps and if there was an exception while executing the step.

        Args:
            exc: The class of Exception that was raised.
            exc_value: The exception value.
            tb: The traceback object.
            end_time: Explicit end_time to stamp. Defaults to now(); the pytest
                plugin passes the last-child finish time when closing a long-lived
                parent so its duration reflects its subtree rather than its own
                late close.

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
            elif isinstance(exc_value, (KeyboardInterrupt, SystemExit)) or _is_session_exit(
                exc_value
            ):
                # Hard exit propagating through the step stack: a SystemExit /
                # KeyboardInterrupt, or pytest's session-stopping Exit
                # (pytest.exit / sift abort). Record ABORTED so every in-progress
                # step the exit unwinds through reflects the cut-off rather than
                # coercing to ERROR.
                aborted = True
                error_info = format_truncated_traceback(exc, exc_value, tb)
            else:
                errored = True
                error_info = format_truncated_traceback(exc, exc_value, tb)

        # Status is the governor: anything other than IN_PROGRESS was set
        # deliberately (manual override, plugin pre-resolution, etc.) and must
        # not be silently overwritten by side-channel signals. When the step is
        # still IN_PROGRESS, resolve from independent state: the step's own abort
        # first, then a child-failed signal (parents inherit FAILED, not the
        # originating ERROR or ABORTED), then the step's own captured exception,
        # then the children-pass default. error_info is diagnostic and never
        # drives status.
        status = current_step.status
        if status == TestStatus.IN_PROGRESS:
            children_passed = self.report_context.open_step_results.get(
                current_step.step_path, True
            )
            if aborted:
                status = TestStatus.ABORTED
                cause = "own_abort"
            elif self.report_context.session_aborted:
                # The run was stopped as a system abort (Ctrl-C / sift abort).
                # A parent closing out-of-band during the unwind inherits
                # ABORTED rather than FAILED, so the abort reaches the
                # containers and report. A plain pytest.exit() leaves the flag
                # unset and falls through to FAILED below.
                status = TestStatus.ABORTED
                cause = "session_aborted"
            elif not children_passed:
                status = TestStatus.FAILED
                cause = "child_failed"
            elif errored:
                status = TestStatus.ERROR
                cause = "own_error"
            else:
                status = TestStatus.PASSED
                cause = "clean"
        else:
            # Status was set deliberately upstream (manual override or plugin
            # pre-resolution) before this resolver ran.
            cause = "preset"

        self._log_resolve(current_step, status, cause)

        # Propagate based on the resolved status; error_info rides along as
        # pure diagnostics and does not affect propagation.
        result = self.report_context.propagate_step_result(current_step, status)
        current_step.update(
            {
                "status": status,
                "end_time": end_time if end_time is not None else datetime.now(timezone.utc),
                "error_info": error_info,
            },
        )
        self.report_context.note_close(current_step)

        return result

    def _log_close(self, step: TestStep) -> None:
        """Audit line for closing a step, with a per-step measurement summary.

        Summarizes (not enumerates) the measurements recorded on this step:
        passed/total, plus the out-of-bounds ones named so a reader sees what
        failed. Omits the measurement field for steps with none.
        """
        measurements = [
            m for m in self.report_context.created_measurements if m.test_step_id == str(step.id_)
        ]
        fields: dict[str, object] = {
            "name": step.name,
            "path": self._source_path or "-",
            "step_path": step.step_path,
            "origin": self._origin,
        }
        if measurements:
            passed = sum(1 for m in measurements if m.passed)
            fields["measurements"] = f"{passed}/{len(measurements)}"
            failed = [m for m in measurements if not m.passed]
            if failed:
                fields["failed"] = "[" + ", ".join(str(m) for m in failed) + "]"
        log_event(logger, logging.DEBUG, "step.close", **fields)

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
            override = getattr(self, "_sift_end_time_override", None)
            # Status was resolved upstream (pytest phase reports / manual
            # override); log it like the in-resolver path so every step has a
            # traceable ``step.resolve`` line.
            self._log_resolve(current_step, current_step.status, "external")
            result = self.report_context.propagate_step_result(current_step, current_step.status)
            current_step.update(
                {
                    "status": current_step.status,
                    "end_time": override if override is not None else datetime.now(timezone.utc),
                    "error_info": current_step.error_info,
                },
            )
            self.report_context.note_close(current_step)
            self._log_close(current_step)
            self.report_context.exit_step(current_step)
            if hasattr(self, "force_result"):
                result = self.force_result
            return result

        result = self._update_step_from_result(
            exc, exc_value, tb, end_time=getattr(self, "_sift_end_time_override", None)
        )

        # Now that the step is updated. Let the report context handle removing it from the stack and updating the report context.
        if self.current_step is not None:
            self._log_close(self.current_step)
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
            origin="substep",
            source_path=self._source_path,
        )
