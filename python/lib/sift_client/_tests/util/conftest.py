"""Override report_context to disable log file simulation for integration tests in this directory so that we can exercise the context manager when no log file is provided."""

from __future__ import annotations

from typing import TYPE_CHECKING, Generator

import pytest

from sift_client.util.test_results.pytest_util import _report_context_impl, _step_impl

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.util.test_results.context_manager import NewStep, ReportContext


@pytest.fixture(scope="session", autouse=True)
def report_context(
    sift_client: SiftClient, client_has_connection: bool, request: pytest.FixtureRequest
) -> Generator[ReportContext | None, None, None]:
    if client_has_connection:
        yield from _report_context_impl(sift_client, request, log_file=None)
    else:
        yield None


@pytest.fixture(autouse=True)
def step(
    report_context: ReportContext, client_has_connection: bool, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    if client_has_connection:
        yield from _step_impl(report_context, request)
    else:
        yield None


@pytest.fixture(scope="module", autouse=True)
def module_substep(
    report_context: ReportContext, client_has_connection: bool, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    if client_has_connection:
        yield from _step_impl(report_context, request)
    else:
        yield None
