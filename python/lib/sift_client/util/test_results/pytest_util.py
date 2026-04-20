from __future__ import annotations

from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING, Any, Generator

import pytest

from sift_client.sift_types.test_report import TestStatus
from sift_client.util.test_results import ReportContext

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.util.test_results.context_manager import NewStep

REPORT_CONTEXT: ReportContext | None = None


def pytest_addoption(parser: pytest.Parser) -> None:
    """Register Sift-specific command-line options."""
    parser.addoption(
        "--sift-test-results-log-file",
        default=None,
        help="Path to write the Sift test result log file. "
        "Use 'true' (default) to auto-create a temp file, "
        "False, 'false', or 'none' to disable logging, "
        "or a file path to write to a specific location.",
    )
    parser.addoption(
        "--no-sift-test-results-git-metadata",
        action="store_false",
        dest="sift_test_results_git_metadata",
        default=True,
        help="Exclude git metadata from the Sift test results. "
        "Git metadata (repo, branch, commit) is included by default.",
    )
    parser.addoption(
        "--sift-test-results-check-connection",
        action="store_true",
        default=False,
        help="Skip the sift test-result fixtures (report_context, step, module_substep) "
        "when the Sift client has no connection to the server. Requires a "
        "`client_has_connection` fixture to be available in the test session.",
    )


def _resolve_log_file(pytestconfig: pytest.Config | None) -> str | Path | bool | None:
    """Determine log_file value from --sift-test-results-log-file option."""
    raw = None
    if pytestconfig is not None:
        raw = pytestconfig.getoption("--sift-test-results-log-file", default=None)
    if raw is None:
        return True
    lower = str(raw).lower()
    if lower in ("true", "1"):
        return True
    if lower in ("false", "none"):
        return None
    return Path(raw)


@pytest.hookimpl(tryfirst=True, hookwrapper=True)
def pytest_runtest_makereport(item: pytest.Item, call: pytest.CallInfo[Any]):
    """You should import this hook to capture any AssertionErrors that occur during the test. If not included, any assert failures in a test will not automatically fail the step."""
    outcome = yield
    report = outcome.get_result()
    if report.outcome == "skipped":
        # Skipped steps won't invoke the method/fixtures at all, so we need to manually record a step.
        if REPORT_CONTEXT:
            with REPORT_CONTEXT.new_step(name=item.name) as new_step:
                new_step.current_step.update(
                    {"status": TestStatus.SKIPPED}, log_file=REPORT_CONTEXT.log_file
                )
    setattr(item, "rep_" + report.when, call)


def _report_context_impl(
    sift_client: SiftClient,
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config | None = None,
) -> Generator[ReportContext | None, None, None]:
    test_path = Path(request.config.invocation_params.args[0])
    base_name = (
        test_path.name
        if test_path.exists()
        else "pytest " + " ".join(request.config.invocation_params.args)
    )
    test_case = test_path if test_path.exists() else base_name
    log_file = _resolve_log_file(pytestconfig)
    include_git_metadata = (
        bool(pytestconfig.getoption("sift_test_results_git_metadata", default=True))
        if pytestconfig
        else True
    )
    with ReportContext(
        sift_client,
        name=f"{base_name} {datetime.now(timezone.utc).isoformat()}",
        test_case=str(test_case),
        log_file=log_file,
        include_git_metadata=include_git_metadata,
    ) as context:
        # Set a global so we can access this in pytest hooks.
        global REPORT_CONTEXT
        REPORT_CONTEXT = context
        yield context


def _check_connection_enabled(pytestconfig: pytest.Config | None) -> bool:
    """Return True when the caller opted into `--sift-test-results-check-connection`."""
    if pytestconfig is None:
        return False
    return bool(pytestconfig.getoption("sift_test_results_check_connection", default=False))


def _has_sift_connection(request: pytest.FixtureRequest) -> bool:
    """Resolve the `client_has_connection` fixture lazily; only called when the check is enabled."""
    return bool(request.getfixturevalue("client_has_connection"))


@pytest.fixture(scope="session", autouse=True)
def report_context(
    sift_client: SiftClient, request: pytest.FixtureRequest, pytestconfig: pytest.Config
) -> Generator[ReportContext | None, None, None]:
    """Create a report context for the session.

    The log file destination is controlled by ``--sift-test-results-log-file``.
    Defaults to a temp file when not set.

    When ``--sift-test-results-check-connection`` is passed, this fixture will no-op
    (yield None) if the Sift client has no connection to the server. That mode
    requires a ``client_has_connection`` fixture to be available in the session.
    """
    if _check_connection_enabled(pytestconfig) and not _has_sift_connection(request):
        yield None
        return
    yield from _report_context_impl(sift_client, request, pytestconfig=pytestconfig)


def _step_impl(
    report_context: ReportContext, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    name = str(request.node.name)
    existing_docstring = request.node.obj.__doc__ or None
    with report_context.new_step(
        name=name, description=existing_docstring, assertion_as_fail_not_error=False
    ) as new_step:
        yield new_step
        if hasattr(request.node, "rep_call") and request.node.rep_call.excinfo:
            new_step.update_step_from_result(
                request.node.rep_call.excinfo,
                request.node.rep_call.excinfo.value,
                request.node.rep_call.excinfo.tb,
            )


@pytest.fixture(autouse=True)
def step(
    report_context: ReportContext | None,
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
) -> Generator[NewStep | None, None, None]:
    """Create an outer step for the function.

    No-ops when ``--sift-test-results-check-connection`` is set and the client
    has no connection (or when the session-scoped ``report_context`` resolved to None).
    """
    if report_context is None or (
        _check_connection_enabled(pytestconfig) and not _has_sift_connection(request)
    ):
        yield None
        return
    yield from _step_impl(report_context, request)


@pytest.fixture(scope="module", autouse=True)
def module_substep(
    report_context: ReportContext | None,
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
) -> Generator[NewStep | None, None, None]:
    """Create a step per module.

    No-ops when ``--sift-test-results-check-connection`` is set and the client
    has no connection (or when the session-scoped ``report_context`` resolved to None).
    """
    if report_context is None or (
        _check_connection_enabled(pytestconfig) and not _has_sift_connection(request)
    ):
        yield None
        return
    yield from _step_impl(report_context, request)


@pytest.fixture(scope="session")
def client_has_connection(sift_client):
    """Check if the SiftClient has a connection to the Sift server.

    Can be used to skip tests that require a connection to the Sift server, and is
    consulted by the Sift fixtures when ``--sift-test-results-check-connection`` is set.
    """
    has_connection = False
    try:
        sift_client.ping.ping()
        has_connection = True
    except Exception:
        has_connection = False
    return has_connection
