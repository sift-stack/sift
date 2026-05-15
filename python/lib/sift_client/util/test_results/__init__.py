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

Note: FedRAMP users: `report_context` defaults to writing a temp log file and
deferring uploads through an `import-test-result-log` worker. In shared
environments, pass `--no-sift-log-file` to skip the file and run create/update
calls inline against the API.

#### Configuration

CLI options registered by the plugin:

- `--sift-offline`: Run without contacting Sift. All create/update calls are
  written to a JSONL log file for later replay. No session-start ping is made.
- `--sift-log-file=<path>`: Path to write the JSONL log file. Defaults to a
  temp file with its path logged at session start.
- `--no-sift-log-file`: Disable the JSONL log file (online mode only).
- `--sift-no-git-metadata`: Exclude git metadata from the report.

To disable the plugin for a single run: `pytest -p no:sift_client.pytest_plugin`.
To keep test code working with the plugin disabled, wire in
`sift_client.pytest_plugin_noop` instead — it ships matching fixture names
whose `measure*` calls evaluate bounds locally without contacting Sift.
"""

from .context_manager import NewStep, ReportContext

__all__ = [
    "NewStep",
    "ReportContext",
]
