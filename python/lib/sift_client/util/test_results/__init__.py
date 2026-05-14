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

## Pytest Plugin

The pytest plugin lives at `sift_client.pytest_plugin`. Opt in
from your `conftest.py`:

```python
# conftest.py
pytest_plugins = ["sift_client.pytest_plugin"]
```

The plugin ships an autouse session-scoped `report_context` fixture (one
`TestReport` per session), an autouse function-scoped `step` fixture, and an
optional `module_substep` fixture. It also registers a default `sift_client`
fixture that reads `SIFT_API_KEY`, `SIFT_GRPC_URI`, and `SIFT_REST_URI` from
the environment. Override it by defining your own `sift_client` fixture in
your conftest.

Note: FedRAMP users: `report_context` will log test results to a temp file to
avoid API calls during test execution. If this is a shared environment, you
can disable logging by passing `--sift-test-results-log-file=false`.

#### Configuration

CLI options registered by the plugin:

- `--sift-test-results-log-file`: Path to write the JSONL log file. `true`
  (default) auto-creates a temp file; `false`/`none` disables logging; a path
  writes to that location.
- `--no-sift-test-results-git-metadata`: Exclude git metadata (repo, branch,
  commit) from the test report. Included by default.
- `--sift-test-results-check-connection`: Make `report_context`, `step`, and
  `module_substep` no-op when the client has no connection. Requires a
  `client_has_connection` fixture (the plugin ships a default).

To disable the plugin for a single run:
`pytest -p no:sift_client.pytest_plugin`.
"""

from .context_manager import NewStep, ReportContext

__all__ = [
    "NewStep",
    "ReportContext",
]
