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
        "--sift-test-results-git-metadata",
        action="store_true",
        default=True,
        help="Include git metadata in the Sift test results.",
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
        bool(pytestconfig.getoption("--sift-test-results-git-metadata", default=False))
        if pytestconfig
        else False
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


@pytest.fixture(scope="session", autouse=True)
def report_context(
    sift_client: SiftClient, request: pytest.FixtureRequest, pytestconfig: pytest.Config
) -> Generator[ReportContext | None, None, None]:
    """Create a report context for the session.

    The log file destination is controlled by ``--sift-test-results-log-file``.
    Defaults to a temp file when not set.
    """
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
    report_context: ReportContext, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    """Create an outer step for the function."""
    yield from _step_impl(report_context, request)


@pytest.fixture(scope="module", autouse=True)
def module_substep(
    report_context: ReportContext, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    """Create a step per module."""
    yield from _step_impl(report_context, request)


@pytest.fixture(scope="session")
def client_has_connection(sift_client):
    """Check if the SiftClient has a connection to the Sift server.

    Can be used to skip tests that require a connection to the Sift server.
    """
    has_connection = False
    try:
        sift_client.ping.ping()
        has_connection = True
    except Exception:
        has_connection = False
    return has_connection


########################################################
# The following fixtures will conditionally create a report if the client has a connection to the Sift server.
# If you want to use these, you must also import or implement the client_has_connection fixture.
########################################################


@pytest.fixture(scope="session", autouse=True)
def report_context_check_connection(
    sift_client: SiftClient,
    client_has_connection: bool,
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
) -> Generator[ReportContext | None, None, None]:
    """Create a report context for the session. Doesn't run if the client has no connection to the Sift server."""
    if client_has_connection:
        yield from _report_context_impl(sift_client, request, pytestconfig=pytestconfig)
    else:
        yield None


@pytest.fixture(autouse=True)
def step_check_connection(
    report_context: ReportContext, client_has_connection: bool, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    """Create an outer step for the function. Doesn't run if the client has no connection to the Sift server."""
    if client_has_connection:
        yield from _step_impl(report_context, request)
    else:
        yield None


@pytest.fixture(scope="module", autouse=True)
def module_substep_check_connection(
    report_context: ReportContext, client_has_connection: bool, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    """Create a step per module. Doesn't run if the client has no connection to the Sift server."""
    if client_has_connection:
        yield from _step_impl(report_context, request)
    else:
        yield None
