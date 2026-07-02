"""Report construction, status resolution, and step creation.

Builds the session ``ReportContext`` from resolved settings (name/test_case
templates, log-file mode, credentials for disabled mode), resolves a function
step's status from pytest's per-phase reports, and finalizes after teardown.
``report_context_impl`` is a pure generator that yields the context; the
plugin's ``report_context`` fixture owns the module-level ``REPORT_CONTEXT``.
"""

from __future__ import annotations

import inspect
import logging
import os
import warnings
from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING, Any, Generator

import pytest

from sift_client import SiftClient, SiftConnectionConfig
from sift_client._internal.pytest_plugin.audit_log import log_event
from sift_client._internal.pytest_plugin.modes import is_offline
from sift_client._internal.pytest_plugin.options import (
    GIT_METADATA_OPTION,
    LOG_FILE_OPTION,
    METADATA_OPTION,
    PARAMETRIZE_NESTING_OPTION,
    PART_NUMBER_OPTION,
    REPORT_NAME_OPTION,
    SERIAL_NUMBER_OPTION,
    SYSTEM_OPERATOR_OPTION,
    TEST_CASE_OPTION,
    TEST_SYSTEM_NAME_OPTION,
)
from sift_client._internal.pytest_plugin.steps import (
    finalize_parents,
    parametrize_path_key,
    strip_param,
)
from sift_client.sift_types.test_report import ErrorInfo, TestStatus
from sift_client.util.test_results import ReportContext
from sift_client.util.test_results.context_manager import (
    _git_metadata,
    format_assertion_message,
    format_truncated_traceback,
)

if TYPE_CHECKING:
    from sift_client.util.test_results.context_manager import NewStep

logger = logging.getLogger(__name__)


def resolve_real_report_id(context: Any) -> str | None:
    """Resolve the real server-side report id for the online footer link.

    In synchronous online mode (``--no-sift-log-file``) the report is created
    directly against the API, so ``report.id_`` is already the real id. In the
    default incremental mode the report is created through the simulate path
    (a client-side UUID) and the background worker maps it to the real id on
    replay, recording it in the ``<log>.tracking`` sidecar's ``id_map``. By the
    time this footer runs the session-scoped report context has torn down and
    the worker has drained, so the sidecar is final.

    Returns ``None`` when the worker never mapped the report (e.g. it died before
    replaying the create), meaning no real report exists to link.
    """
    report = context.report
    if not report.id_:
        # No id was ever assigned (unset/empty); nothing to link.
        return None
    sim_id = str(report.id_)
    if not getattr(report, "is_simulated", False):
        return sim_id
    log_file = getattr(context, "log_file", None)
    if log_file is None:
        return None
    from sift_client._internal.low_level_wrappers._test_results_log import LogTracking

    return LogTracking.load(log_file).id_map.get(sim_id)


def resolve_report_link(context: Any, offline: bool) -> tuple[str | None, str | None]:
    """Resolve ``(report_id, report_url)`` for the terminal footer.

    Offline runs never upload, so the id is ``None``. Online, the id comes from
    ``resolve_real_report_id`` and the URL is built only when both the id and the
    client's ``app_url`` are set. Truthiness, not ``is not None``: a
    resolved-but-empty id (degenerate sidecar mapping, unset proto field) must
    fall through to the "not uploaded" path, not produce a ``/test-results/`` link.
    """
    report_id = None if offline else resolve_real_report_id(context)
    report_url = (
        f"{context.client.app_url}/test-results/{report_id}"
        if report_id and context.client.app_url
        else None
    )
    return report_id, report_url


def error_info_from_longrepr(longrepr: Any) -> ErrorInfo:
    """Fall back to the report's longrepr when no Python exception is available."""
    return ErrorInfo(error_code=1, error_message=str(longrepr) if longrepr is not None else "")


def resolve_initial_status(new_step: NewStep, item: pytest.Item) -> None:
    """Resolve the function step's status from pytest's per-phase reports.

    Reads ``_sift_phase_setup`` / ``_sift_phase_call`` and the test's xfail marker,
    then mutates ``new_step.current_step`` in place and flips
    ``new_step._sift_managed_externally`` so ``NewStep.__exit__`` emits the
    resolved status without re-classifying.

    When the call phase reports ``passed`` and no override is needed (i.e. the
    test's own status or substep failures should drive the result), this leaves
    the step alone so the default ``__exit__`` resolution stays in charge.
    """
    current_step = new_step.current_step
    if current_step is None:
        # The step never opened (the autouse fixture short-circuited or was
        # disabled). Nothing to resolve.
        return
    setup_phase = getattr(item, "_sift_phase_setup", None)
    call_phase = getattr(item, "_sift_phase_call", None)
    xfail_marker = item.get_closest_marker("xfail")
    xfail_runs = xfail_marker.kwargs.get("run", True) if xfail_marker is not None else True

    status: TestStatus | None = None
    error_info: ErrorInfo | None = None

    if setup_phase is not None and setup_phase.report.outcome == "failed":
        status = TestStatus.ERROR
        excinfo = setup_phase.call.excinfo
        if excinfo is not None:
            error_info = format_truncated_traceback(excinfo.type, excinfo.value, excinfo.tb)
        else:
            error_info = error_info_from_longrepr(setup_phase.report.longrepr)
    elif setup_phase is not None and setup_phase.report.outcome == "skipped":
        status = TestStatus.SKIPPED
    elif call_phase is None:
        # Setup completed but the call-phase report never fired; the session was
        # aborted (e.g. by KeyboardInterrupt) before the plugin could observe the
        # outcome. Resolve to ABORTED rather than leaving it IN_PROGRESS, since the
        # test was cut off and a finalized report should not carry a step that
        # still reads as in-progress. No call ``excinfo`` exists here, so there is
        # no traceback to attach.
        status = TestStatus.ABORTED
    else:
        wasxfail = getattr(call_phase.report, "wasxfail", None)
        if wasxfail is not None:
            if call_phase.report.outcome == "failed":
                # Strict xpass: pytest synthesizes a failure when an xfail(strict=True)
                # test unexpectedly passes. The xfail mark no longer matches reality.
                status = TestStatus.FAILED
            elif call_phase.report.outcome == "skipped":
                if xfail_marker is not None and xfail_runs is False:
                    # xfail(run=False): the test body never executed.
                    status = TestStatus.SKIPPED
                else:
                    # xfail + expected failure: the test fulfilled its xfail expectation.
                    status = TestStatus.PASSED
            else:
                # Non-strict xpass: passes that weren't required to fail.
                status = TestStatus.PASSED
        elif call_phase.report.outcome == "passed":
            # Default __exit__ resolves PASSED/FAILED from open_step_results and any
            # status the test code may have set. Don't override it here.
            return
        elif call_phase.report.outcome == "skipped":
            status = TestStatus.SKIPPED
        elif call_phase.report.outcome == "failed":
            excinfo = call_phase.call.excinfo
            children_passed = new_step.report_context.open_step_results.get(
                current_step.step_path, True
            )
            if excinfo is None:
                status = TestStatus.FAILED
            elif isinstance(excinfo.value, AssertionError):
                status = TestStatus.FAILED
                error_info = format_assertion_message(excinfo.type, excinfo.value)
            elif isinstance(excinfo.value, pytest.fail.Exception):
                status = TestStatus.FAILED
            elif isinstance(excinfo.value, (KeyboardInterrupt, SystemExit)):
                # Hard exits the plugin can observe: pytest converted the
                # raise into a call-phase report. The session-aborting variant
                # (call_phase is None) lands in the branch above, also ABORTED.
                status = TestStatus.ABORTED
                error_info = format_truncated_traceback(excinfo.type, excinfo.value, excinfo.tb)
            elif xfail_marker is not None:
                # xfail(raises=X) with a non-matching exception: the contract failed.
                status = TestStatus.FAILED
                error_info = format_truncated_traceback(excinfo.type, excinfo.value, excinfo.tb)
            elif not children_passed:
                # A substep already recorded the error and carries the traceback;
                # the test step only inherits the child-failed signal.
                status = TestStatus.FAILED
            else:
                status = TestStatus.ERROR
                error_info = format_truncated_traceback(excinfo.type, excinfo.value, excinfo.tb)

    if status is None:
        return

    # BaseType is frozen; mutate via __dict__ the same way _apply_client_to_instance does.
    current_step.__dict__["status"] = status
    if error_info is not None:
        current_step.__dict__["error_info"] = error_info
    new_step._sift_managed_externally = True


def describe_step_failure(new_step: NewStep) -> str:
    """A short, human reason a step failed, for the audit log. Empty when none.

    Prefers a named out-of-bounds measurement (the common test-failure cause),
    then falls back to the first line of the step's ``error_info``.
    """
    failed = getattr(new_step, "_failed_measurements", [])
    if failed:
        extra = f" (and {len(failed) - 1} more)" if len(failed) > 1 else ""
        return f"{failed[0]}{extra}"
    step = new_step.current_step
    if step is not None and step.error_info is not None and step.error_info.error_message:
        return step.error_info.error_message.strip().splitlines()[0]
    return ""


def skip_or_xfail_reason(phase: Any) -> str:
    """The skip/xfail reason text from a pytest phase, or empty when none.

    xfail reasons ride on ``report.wasxfail``; plain skips store
    ``(path, lineno, "Skipped: <reason>")`` in ``report.longrepr``.
    """
    if phase is None:
        return ""
    report = phase.report
    wasxfail = getattr(report, "wasxfail", None)
    if wasxfail:
        return f"xfail: {wasxfail}"
    longrepr = getattr(report, "longrepr", None)
    if isinstance(longrepr, tuple) and len(longrepr) == 3:
        return str(longrepr[2])
    return ""


def finalize_after_teardown(item: pytest.Item, teardown_report: pytest.TestReport) -> None:
    """Upgrade a closed step to FAILED when the teardown phase failed.

    The autouse step fixture has already exited by the time the teardown
    makereport hook fires, so call ``step.update`` again to override the status
    server-side and propagate the failure to the still-open parent step.
    """
    step: NewStep | None = getattr(item, "_sift_step", None)
    if step is None:
        return
    current_step = step.current_step
    if current_step is None:
        return
    if teardown_report.outcome == "failed" and current_step.status == TestStatus.PASSED:
        log_event(
            logger,
            logging.WARNING,
            "status",
            path=item.nodeid,
            pytest="teardown_failed",
            sift="PASSED->FAILED",
        )
        current_step.update({"status": TestStatus.FAILED})
        step.report_context.mark_step_failed_after_close(current_step)


def _relativize(path: Path, rootpath: Path) -> str:
    """Path relative to rootdir, or the basename when it sits outside the tree."""
    try:
        rel = str(path.relative_to(rootpath))
    except ValueError:
        return path.name
    return "" if rel == "." else rel


def derive_target(request: pytest.FixtureRequest, args: tuple[str, ...]) -> str:
    """Describe what was run, from the collected items rather than the command line.

    Collection is the ground truth of selection, independent of flag order,
    ``-k`` / ``-m`` filters, or which path form was typed. Every value is
    anchored to the rootdir (project) name so the shape is uniform; granularity
    narrows with the selection:

    * a single test -> ``project/tests/test_motor.py::test_spin`` (param stripped)
    * a single file -> ``project/tests/test_motor.py``
    * many files    -> their common directory, ``project/tests/motor``
    * whole tree / nothing collected / paths outside rootdir -> ``project``

    The report is session-level and individual tests are its steps, so the
    file/directory grain is the natural unit of "what ran" for the report
    itself. The verbatim invocation stays available via ``{command}`` and the
    ``pytest_command`` metadata key.
    """
    rootpath = request.config.rootpath
    root = rootpath.name

    def _anchor(rel: str) -> str:
        return f"{root}/{rel}" if rel else root

    items = list(getattr(request.session, "items", ()) or ())
    if not items:
        return root
    if len(items) == 1:
        return _anchor(strip_param(items[0].nodeid))
    paths = {p for p in (getattr(i, "path", None) for i in items) if p is not None}
    if not paths:
        return root
    if len(paths) == 1:
        return _anchor(_relativize(next(iter(paths)), rootpath))
    try:
        common = Path(os.path.commonpath([str(p) for p in paths]))
    except ValueError:
        # e.g. paths on different drives (Windows); fall back to the project.
        return root
    return _anchor(_relativize(common, rootpath))


def build_template_fields(
    target: str,
    command: str,
    args: tuple[str, ...],
    request: pytest.FixtureRequest,
) -> dict[str, Any]:
    """Build the placeholder mapping shared by the name and test_case templates."""
    items = getattr(request.session, "items", ()) or ()
    git = _git_metadata() or {}
    return {
        "target": target,
        "command": command,
        "args": " ".join(args),
        "rootdir": request.config.rootpath.name,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "count": len(items),
        "git_repo": git.get("git_repo", ""),
        "git_branch": git.get("git_branch", ""),
        "git_commit": git.get("git_commit", ""),
    }


def format_template(
    template: str,
    fields: dict[str, Any],
    *,
    fallback: str,
    option_label: str,
) -> str:
    """Format ``template`` with ``fields``; on bad input, warn and return ``fallback``.

    A bad template should never block test results from being recorded, so the
    rendering errors collapse to a warning + fallback rather than aborting the
    session.
    """
    from sift_client.pytest_plugin import SiftPytestPluginWarning

    try:
        return template.format(**fields)
    except (KeyError, IndexError, ValueError) as exc:
        log_event(
            logger,
            logging.WARNING,
            "template.invalid",
            option=option_label,
            template=template,
            error=repr(exc),
            fallback=fallback,
        )
        warnings.warn(
            f"Invalid {option_label} template {template!r} ({exc}); using fallback.",
            SiftPytestPluginWarning,
            stacklevel=2,
        )
        return fallback


def log_file_enabled(pytestconfig: pytest.Config | None) -> bool:
    """Whether the JSONL log of create/update calls is written.

    On by default; ``--no-sift-log-file`` disables it. Offline mode routes every
    create/update call through the log as its only sink, so disabling the log
    while offline is a usage error.
    """
    enabled = bool(LOG_FILE_OPTION.resolve(pytestconfig))
    if not enabled and is_offline(pytestconfig):
        raise pytest.UsageError(
            "--no-sift-log-file is incompatible with --sift-offline; offline mode "
            "requires the JSONL log as its only sink. Drop one of the two flags."
        )
    return enabled


def report_context_impl(
    sift_client: SiftClient,
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config | None = None,
) -> Generator[ReportContext, None, None]:
    args = request.config.invocation_params.args
    # ``target`` is "what ran", derived from the collected items (see
    # derive_target), invocation-independent, unlike parsing the command
    # line. Both the display name and test_case default to it; the verbatim
    # command stays available via {command} and the pytest_command metadata.
    target = derive_target(request, args)
    command = "pytest " + " ".join(args) if args else "pytest"
    fields = build_template_fields(target, command, args, request)
    # What each report_name / test_case ``{placeholder}`` resolved to; the
    # resolved name/test_case themselves show on the ``report`` line.
    log_event(logger, logging.DEBUG, "template", **fields)
    name_template = REPORT_NAME_OPTION.resolve(pytestconfig) or "{target} {timestamp}"
    name = format_template(
        name_template,
        fields,
        fallback=f"{target} {fields['timestamp']}",
        option_label="sift_report_name",
    )
    test_case_template = TEST_CASE_OPTION.resolve(pytestconfig)
    test_case = (
        format_template(
            test_case_template,
            fields,
            fallback=target,
            option_label="sift_test_case",
        )
        if test_case_template
        else target
    )
    # Metadata starts from the [tool.sift.pytest.report.metadata] TOML table, then
    # the sift_report_metadata fixture layers over it (runtime values the static
    # table can't express), and the auto-recorded pytest_command layers in last so
    # neither can overwrite it.
    report_metadata: dict[str, str | float | bool] = {
        **METADATA_OPTION.resolve_merged(pytestconfig),
        **request.getfixturevalue("sift_report_metadata"),
        "pytest_command": command,
    }
    # Mode → ReportContext flags:
    #   online (default): log_file=<path in session dir>, replay_log_file=True
    #   --sift-offline:   log_file=<path in session dir>, replay_log_file=False
    #   --sift-disabled / --no-sift-log-file: log_file=False, replay_log_file=False
    disabled = sift_client._simulate
    offline = False if disabled else is_offline(pytestconfig)
    log_file: str | Path | bool = False if disabled else log_file_enabled(pytestconfig)
    # Place the JSONL inside the run's session dir so it lands alongside the
    # audit log. pytest_configure created the dir whenever the log is enabled; if
    # one isn't present (e.g. ReportContext used outside pytest), log_file stays
    # True and ReportContext mints its own dir.
    if log_file is True and pytestconfig is not None:
        from sift_client.pytest_plugin import SIFT_SESSION_DIR_STASH_KEY

        plugin_session_dir = pytestconfig.stash.get(SIFT_SESSION_DIR_STASH_KEY, None)
        if plugin_session_dir is not None:
            log_file = plugin_session_dir / f"{plugin_session_dir.name}.jsonl"
    include_git_metadata = bool(GIT_METADATA_OPTION.resolve(pytestconfig))
    # Local import avoids a circular import (pytest_plugin imports this module).
    audit_log = None
    if pytestconfig is not None:
        from sift_client.pytest_plugin import SIFT_AUDIT_LOG_STASH_KEY

        audit_log = pytestconfig.stash.get(SIFT_AUDIT_LOG_STASH_KEY, None)
    with ReportContext(
        sift_client,
        name=name,
        test_case=test_case,
        test_system_name=TEST_SYSTEM_NAME_OPTION.resolve(pytestconfig) or None,
        system_operator=SYSTEM_OPERATOR_OPTION.resolve(pytestconfig) or None,
        serial_number=SERIAL_NUMBER_OPTION.resolve(pytestconfig) or None,
        part_number=PART_NUMBER_OPTION.resolve(pytestconfig) or None,
        log_file=log_file,
        include_git_metadata=include_git_metadata,
        replay_log_file=not (disabled or offline),
        metadata=report_metadata,
        audit_log=audit_log,
    ) as context:
        report = context.report
        meta_kv = ",".join(f"{k}={v}" for k, v in (report.metadata or {}).items()) or "-"
        log_event(
            logger,
            logging.INFO,
            "report",
            name=report.name,
            test_case=report.test_case,
            id=report.id_ or "-",
            system=report.test_system_name or "-",
            operator=report.system_operator or "-",
            serial=report.serial_number or "-",
            part=report.part_number or "-",
            metadata=meta_kv,
        )
        # What actually happens with the JSONL log, not the raw setting: the
        # effective path (temp or pinned), or "disabled", plus whether the
        # background replay worker runs.
        log_event(
            logger,
            logging.INFO,
            "log_file",
            path=context.log_file or "disabled",
            replay=not (disabled or offline),
        )
        try:
            yield context
        finally:
            # Close any report-tree parents still open INSIDE the ReportContext's
            # ``with`` block, so their final ``__exit__`` update calls are written
            # to the log file BEFORE the import worker drains. Without this, the
            # worker exits with a partial backlog and the parent steps are stuck
            # IN_PROGRESS in the Sift report. Most parents already closed early as
            # their subtrees finished; this is the backstop for the rest.
            finalize_parents()


# Placeholder credentials used in --sift-offline mode when env/ini values
# are missing. Offline mode never makes network calls, so the values are
# only syntactically required by SiftConnectionConfig.
OFFLINE_DEFAULTS = {
    "SIFT_API_KEY": "offline",
    "SIFT_GRPC_URI": "offline.invalid:0",
    "SIFT_REST_URI": "http://offline.invalid",
}


def build_disabled_client() -> SiftClient:
    """Construct a SiftClient for ``--sift-disabled`` mode.

    Tagged with ``_simulate=True`` so test-results writes short-circuit through
    the existing low-level simulate path without contacting Sift. The URLs are
    syntactically valid but unreachable; nothing dials them.
    """
    client = SiftClient(
        connection_config=SiftConnectionConfig(
            api_key="disabled",
            grpc_url="disabled.invalid:0",
            rest_url="http://disabled.invalid",
        )
    )
    client._simulate = True
    return client


def leaf_step_name(node: pytest.Item, config: pytest.Config) -> str:
    """The display name for a test's leaf step.

    Items get a parametrize path stashed in ``pytest_itemcollected``; the leaf
    frame (``path[-1]``) is the test-specific display name with higher-scoped
    params promoted out (e.g. ``v=1``, or the bare function name). When
    parametrize-nesting is disabled, fall back to the bracket-mangled pytest
    name (e.g. ``test_a[1]``) so the leaf stays uniquely identifiable. Shared by
    the autouse ``step`` fixture and the collection-skip path so a skipped
    parametrized item is named the same way a run one would be.
    """
    if PARAMETRIZE_NESTING_OPTION.resolve(config):
        path = node.stash.get(parametrize_path_key, ())
        return path[-1] if path else str(node.name)
    return str(node.name)


def step_impl(
    report_context: ReportContext, request: pytest.FixtureRequest
) -> Generator[NewStep, None, None]:
    node = request.node
    # The leaf frame is the test-specific display name; parents are opened by
    # ``_sift_parents``.
    name = leaf_step_name(node, request.config)
    # ``node.obj`` may not exist (e.g., ``pytest.DoctestItem``) or may raise
    # when accessed; fall back to no description in those cases rather than
    # erroring out a perfectly valid test. ``getattr``'s default only
    # suppresses ``AttributeError``; the try/except catches everything else
    # (RuntimeError from a misbehaving ``__doc__`` descriptor, etc.).
    # ``inspect.getdoc`` cleans the docstring (dedents interior lines, trims
    # surrounding blank lines) and walks inheritance.
    try:
        obj = getattr(node, "obj", None)
        existing_docstring = (inspect.getdoc(obj) or None) if obj is not None else None
    except Exception:
        existing_docstring = None
    # Attach the leaf under the parent ``_sift_parents`` resolved for this item
    # (None -> a report-root step). ``push=True`` keeps the leaf on the step stack
    # so any in-test ``substep`` nests under it.
    parent_ns: NewStep | None = getattr(node, "_sift_parent", None)
    parent_step = parent_ns.current_step if parent_ns is not None else None
    with report_context.new_step(
        name=name,
        description=existing_docstring,
        assertion_as_fail_not_error=False,
        parent=parent_step,
        push=True,
        origin="step_impl",
        source_path=node.nodeid,
    ) as new_step:
        node._sift_step = new_step
        yield new_step
        resolve_initial_status(new_step, node)
    # One readable line per test, logged after __exit__ resolves the passing
    # path (incl. pytest-passed-but-step-failed via a bad measurement), pairing
    # the pytest outcome with the final Sift status and naming the cause on
    # failure so a reader sees what went wrong.
    call_phase = getattr(node, "_sift_phase_call", None)
    setup_phase = getattr(node, "_sift_phase_setup", None)
    phase = call_phase or setup_phase
    final_step = new_step.current_step
    status_name = final_step.status.name if final_step is not None else "IN_PROGRESS"
    if call_phase is not None:
        pytest_outcome = call_phase.report.outcome
    elif setup_phase is not None:
        pytest_outcome = setup_phase.report.outcome
    else:
        pytest_outcome = "not run"
    duration = getattr(phase.report, "duration", None) if phase is not None else None
    dur_ms = int(duration * 1000) if duration is not None else 0
    failed = status_name in ("FAILED", "ERROR")
    reason = describe_step_failure(new_step) if failed else skip_or_xfail_reason(phase)
    fields: dict[str, object] = {
        "path": node.nodeid,
        "pytest": pytest_outcome,
        "sift": status_name,
        "dur": f"{dur_ms}ms",
    }
    if reason:
        fields["reason"] = reason
    log_event(logger, logging.DEBUG, "status", **fields)
