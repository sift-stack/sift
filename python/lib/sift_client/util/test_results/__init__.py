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

Note: FedRAMP users: report_context will log test results to a temp file to avoid API calls during test execution. If this is a shared environment, you can disable logging by passing ``--sift-test-results-log-file=false``.

#### Configuration

Import the `pytest_addoption` function to add configuration options for Test Results to the commandline or add the options to your pyproject.toml file (https://docs.pytest.org/en/stable/reference/customize.html#configuration). If ommitted, will use the default values described below.

- Git metadata: Include git metadata (repo, branch, commit) in the test results. Default is True. You can disable it by passing `--no-sift-test-results-git-metadata`.
- Log file: Write test results to a file. This happens automatically but you can configure specify a specific log file by passing `--sift-test-results-log-file=<path>` or disable logging by passing `--sift-test-results-log-file=false`.
- Check connection: Pass `--sift-test-results-check-connection` (off by default) to make the `report_context`, `step`, and `module_substep` fixtures no-op when the Sift client has no connection to the server. Requires a `client_has_connection` fixture to be available.

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

from sift_client.util.test_results import *
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
    pytest_addoption,
    pytest_runtest_makereport,
    report_context,
    step,
)

__all__ = [
    "NewStep",
    "ReportContext",
    "client_has_connection",
    "module_substep",
    "pytest_addoption",
    "pytest_runtest_makereport",
    "report_context",
    "step",
]
