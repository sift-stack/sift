from __future__ import annotations

from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING, Generator

import pytest

from sift_client.util.test_results import ReportContext

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.util.test_results.context_manager import NewStep


@pytest.fixture(scope="session", autouse=True)
def report_context(
    sift_client: SiftClient, client_has_connection: bool, request: pytest.FixtureRequest
) -> Generator[ReportContext | None, None, None]:
    """Create a report context for the session."""
    if client_has_connection:
        test_path = Path(request.config.invocation_params.args[0])
        base_name = test_path.name if test_path.exists() else " ".join(request.config.invocation_params.args)
        test_case = test_path if test_path.exists() else base_name
        with ReportContext(
            sift_client,
            name=f"{base_name} {datetime.now(timezone.utc).isoformat()}",
            test_case=str(test_case),
        ) as context:
            yield context
    else:
        yield None


@pytest.fixture(autouse=True)
def step(
    report_context: ReportContext, client_has_connection: bool, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    """Create an outer step for the function."""
    if client_has_connection:
        name = str(request.node.name)
        with report_context.new_step(name=name) as new_step:
            yield new_step
    else:
        yield None


@pytest.fixture(scope="module", autouse=True)
def module_substep(
    report_context: ReportContext, client_has_connection: bool, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    """Create a step per module."""
    if client_has_connection:
        name = str(request.node.name)
        with report_context.new_step(name=name) as new_step:
            yield new_step
    else:
        yield None
