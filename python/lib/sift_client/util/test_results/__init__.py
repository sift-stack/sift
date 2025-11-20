"""Test Results Utilities.

This module provides utilities for working with test results.

# Context Managers
- `ReportContext` - Context manager for a new TestReport.
- `NewStep` - Context manager to create a new step in a test report.

### Example

```python
client = SiftClient(api_key=api_key, grpc_url=grpc_url, rest_url=rest_url)
with ReportContext(client, name="Example Report") as rc:
    with rc.new_step(name="Setup") as step:
        controller_setup(step)
    with rc.new_step(name="Example Step", description=desc) as parent_step:
        cmd_interface.cmd("ec1", "rtv.cmd", 75.0)
        sleep(0.01)

        with parent_step.substep(name="Substep 1", description="Measure position") as substep:
            ec = "ec1"
            pos_channel = "rtv.pos"
            pos = tlm.read(ec, pos_channel)
            result = substep.measure(pos, name=f"{ec}.{pos_channel}", bounds=(min=74.9, max=75.1))
            return result # This is optional for other uses, but the step and its parents will be updated correctly i.e. failed if the measurement fails.
```

#### Manually Updating Underlyling Report
You can also manually update the underlying report or steps by accessing the context manager's attributes.
```python
with ReportContext(client, name="Example Report") as rc:
    with rc.new_step(name="Example Step") as step:
        if !conditions:
            step.update({"status": TestStatus.SKIPPED})
        else:
            step.measure(name="Example Measurement", value=test_value, bounds={"min": -1, "max": 10})
    rc.report.update({"run_id": run_id})
```

For a larger class or script, consider creating the context in a setup method and passing it to the test functions.
```python
def main(self):
    self.sift_client = SiftClient(api_key=api_key, grpc_url=grpc_url, rest_url=rest_url)
    with ReportContext(self.sift_client, name="Test Class", description="Test Class") as rc:
        setup(rc)
        test_one(rc)
        test_two(rc)
        teardown(rc)
    cleanup()
```

## Pytest Fixtures

The report context and steps can also be accessed in pytest by importing the `report_context` and `step` fixtures.

### How to use:
- These fixtures are set to autouse and will automatically create a report and steps for each test function.
  - If you want each module(file) to be marked as a step w/ each test as a substep, import the `module_substep` fixture as well.
- The `report_context` fixture requires a fixture `sift_client` returning an `SiftClient` instance to be passed in.

###### Example at top of your test file or in your conftest.py file:

```python
import pytest

@pytest.fixture(scope="session")
def sift_client() -> SiftClient:
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_API_KEY", "")

    client = SiftClient(api_key=api_key, grpc_url=grpc_url, rest_url=rest_url)

    return client

from sift_client.util.test_results import pytest_runtest_makereport, report_context, step, module_substep
```

###### Then in your test file:

```python
# Because step was already imported and set autouse=True, this test will automatically get a step created for it.
def test_no_includes():
    assert condition, "Example failure"

# Passing the fixtures to the test function allows you to take measurements or create substeps.
def test_example(report_context, step):
    # This will add a measurement to the current step for this function
    step.measure(name="Example Measurement", value=test_string_value, bounds="expected_string_value")

    with report_context.new_step(name="Example Step") as substep:
        example_measurement = tlm.read(channel_name)
        substep.measure(name="Substep Measurement", value=example_measurement, bounds=(min=74.9, max=75.1))
```
"""

from .context_manager import NewStep, ReportContext
from .pytest_util import (
    client_has_connection,
    module_substep,
    module_substep_check_connection,
    pytest_runtest_makereport,
    report_context,
    report_context_check_connection,
    step,
    step_check_connection,
)

__all__ = [
    "NewStep",
    "ReportContext",
    "client_has_connection",
    "module_substep",
    "module_substep_check_connection",
    "pytest_runtest_makereport",
    "report_context",
    "report_context_check_connection",
    "step",
    "step_check_connection",
]
