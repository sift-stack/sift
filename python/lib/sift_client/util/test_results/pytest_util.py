from __future__ import annotations

from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING, Any, Generator

import pytest

from sift_client.util.test_results import ReportContext

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.util.test_results.context_manager import NewStep


@pytest.hookimpl(tryfirst=True, hookwrapper=True)
def pytest_runtest_makereport(item: pytest.Item, call: pytest.CallInfo[Any]):
    """You should import this hook to capture any AssertionErrors that occur during the test. If not included, any assert failures in a test will not automatically fail the step."""
    outcome = yield
    rep = outcome.get_result()
    setattr(item, "rep_" + rep.when, call)


def _report_context_impl(
    sift_client: SiftClient, request: pytest.FixtureRequest
) -> Generator[ReportContext | None, None, None]:
    test_path = Path(request.config.invocation_params.args[0])
    base_name = (
        test_path.name
        if test_path.exists()
        else "pytest " + " ".join(request.config.invocation_params.args)
    )
    test_case = test_path if test_path.exists() else base_name
    with ReportContext(
        sift_client,
        name=f"{base_name} {datetime.now(timezone.utc).isoformat()}",
        test_case=str(test_case),
    ) as context:
        yield context


@pytest.fixture(scope="session", autouse=True)
def report_context(
    sift_client: SiftClient, request: pytest.FixtureRequest
) -> Generator[ReportContext | None, None, None]:
    """Create a report context for the session."""
    yield from _report_context_impl(sift_client, request)


def _step_impl(
    report_context: ReportContext, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    name = str(request.node.name)
    with report_context.new_step(name=name) as new_step:
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
    sift_client: SiftClient, client_has_connection: bool, request: pytest.FixtureRequest
) -> Generator[ReportContext | None, None, None]:
    """Create a report context for the session. Doesn't run if the client has no connection to the Sift server."""
    if client_has_connection:
        yield from _report_context_impl(sift_client, request)
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
