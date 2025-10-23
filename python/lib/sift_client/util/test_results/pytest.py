from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING
import sys
import pytest
from pathlib import Path
from sift_client.util.test_results import ReportContext

if TYPE_CHECKING:
    from sift_client.client import SiftClient


# TODO FIGURE OUT HOW TO EXPORT THIS BETTER
@pytest.fixture(scope="session", autouse=True)
def report_context(sift_client: SiftClient, request: pytest.FixtureRequest) -> ReportContext:
    """Create a report context for the session."""
    test_path = Path(request.config.invocation_params.args[0])
    context = ReportContext.create(
        sift_client,
        name=f"{test_path.name} {datetime.now(timezone.utc).isoformat()}",
        test_case=str(test_path),
    )
    yield context
    update = {
        "end_time": datetime.now(timezone.utc),
    }
    if context.any_failures:
        update["status"] = 3  # TestStatus.FAILED
    context.report.update(update)


@pytest.fixture(autouse=True)
def step(report_context: ReportContext, request: pytest.FixtureRequest):
    """Create an outer step for the function."""
    name = str(request.node.name)
    print("Step fixture: function name in step", name)
    with report_context.new_step(name=name) as new_step:
        print("Step fixture: step", new_step)
        yield new_step


@pytest.fixture(scope="module", autouse=True)
def module_substep(report_context: ReportContext, request: pytest.FixtureRequest):
    """Create a step per module."""
    name = str(request.node.name)
    with report_context.new_step(name=name) as new_step:
        yield new_step
