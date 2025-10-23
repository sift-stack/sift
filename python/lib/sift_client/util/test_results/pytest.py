from __future__ import annotations

from datetime import datetime, timezone
from typing import TYPE_CHECKING

import pytest

from sift_client.util.test_results import ReportContext

if TYPE_CHECKING:
    from sift_client.client import SiftClient


# TODO FIGURE OUT HOW TO EXPORT THIS BETTER
@pytest.fixture(scope="module", autouse=True)
def report_context(sift_client: SiftClient, request: pytest.FixtureRequest) -> ReportContext:
    """Create a report context for the session."""
    test_case = str(request.node.name)
    print(f"report_context fixture: {test_case} {request.node.name}")
    context = ReportContext.create(
        sift_client,
        name=f"{test_case} {datetime.now(timezone.utc).isoformat()}",
        test_case=request.node.name,
    )
    yield context
    print("test_case in fixture after yield", request.node.name)
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
