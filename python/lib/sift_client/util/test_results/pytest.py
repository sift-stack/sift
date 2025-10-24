from __future__ import annotations

from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING

import pytest

from sift_client.util.test_results import ReportContext

if TYPE_CHECKING:
    from sift_client.client import SiftClient


@pytest.fixture(scope="session", autouse=True)
def report_context(sift_client: SiftClient, request: pytest.FixtureRequest) -> ReportContext:
    """Create a report context for the session."""
    test_path = Path(request.config.invocation_params.args[0])
    with ReportContext(
        sift_client,
        name=f"{test_path.name} {datetime.now(timezone.utc).isoformat()}",
        test_case=str(test_path),
    ) as context:
        yield context


@pytest.fixture(autouse=True)
def step(report_context: ReportContext, request: pytest.FixtureRequest):
    """Create an outer step for the function."""
    name = str(request.node.name)
    with report_context.new_step(name=name) as new_step:
        yield new_step


@pytest.fixture(scope="module", autouse=True)
def module_substep(report_context: ReportContext, request: pytest.FixtureRequest):
    """Create a step per module."""
    name = str(request.node.name)
    with report_context.new_step(name=name) as new_step:
        yield new_step
