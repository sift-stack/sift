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

By default, every test in the session produces a Sift report: one
`TestReport` per session, one step per test function (`step`), and one
parent step per Python package (directory with `__init__.py`), test file,
and test class
above it. Individual layers can be flattened via the `sift_package_step`,
`sift_module_step`, `sift_class_step`, and `sift_parametrize_nesting` ini
flags. The plugin also registers a default `sift_client` fixture that reads
`SIFT_API_KEY`, `SIFT_GRPC_URI`, and `SIFT_REST_URI` from the environment.
Override it by defining your own `sift_client` fixture in your conftest.

Note: FedRAMP users: results are buffered to a temp file and uploaded by a
subprocess at session end (no API calls during the run). Disable the buffer
entirely with `--no-sift-log-file` for inline uploads.

### Controlling which tests produce reports

The autouse fixtures fire for every test by default. To narrow that:

- Set `sift_autouse = false` in `pyproject.toml` to flip the
  project default off, then opt tests back in below.
- `@pytest.mark.sift_include` forces reporting on for a test, class, or
  module. `@pytest.mark.sift_exclude` forces it off. Closest marker wins.
  `sift_exclude` beats `sift_include` when both apply.
- `pytestmark` at the class or module level inherits to every test in scope.
- For a whole directory, apply the marker in bulk from that directory's
  `conftest.py`:

```python
# tests/integration/conftest.py
from pathlib import Path

import pytest

_HERE = Path(__file__).parent


def pytest_collection_modifyitems(config, items):
    for item in items:
        try:
            item.path.relative_to(_HERE)
        except ValueError:
            continue
        item.add_marker(pytest.mark.sift_include)
```

#### Configuration

CLI options registered by the plugin:

- `--sift-offline`: Run without contacting Sift. All create/update calls are
  written to the JSONL log file for later replay via `import-test-result-log`.
  No session-start ping is attempted.
- `--sift-disabled`: Skip Sift entirely. Nothing contacts the API and no
  log file is written. `step.measure(...)` still evaluates bounds and
  returns a real pass/fail boolean. Returned entities expose
  ``is_simulated == True``. Also honored via the `SIFT_DISABLED` env
  var. Supersedes every other flag.
- `--sift-output-dir`: Directory for this run's artifacts (JSONL log, audit
  trace). Each run gets its own random subfolder. Defaults to a temp directory.
- `--no-sift-log-file`: Disable the JSONL log (written by default). Incompatible
  with `--sift-offline`, which needs the log as its only sink.
- `--no-sift-audit-log`: Disable the DEBUG audit trace (written by default).
- `--no-sift-git-metadata`: Exclude git metadata (repo, branch,
  commit) from the test report. Included by default.

Each option has a matching ini key for per-project configuration under
``[tool.pytest.ini_options]`` in ``pyproject.toml`` (or ``[pytest]`` in
``pytest.ini``). CLI flags override ini values. The
``sift_autouse`` ini key (bool, default ``true``) sets the
project-wide default for the gate described above. The default
``sift_client`` fixture reads ``sift_grpc_uri`` and ``sift_rest_uri`` as
fallbacks when the corresponding env vars are unset (env vars win when
both are set). ``SIFT_API_KEY`` is env-only. Load it from a ``.env`` file
via the ``pytest-dotenv`` plugin or inject it via your CI secret manager.

```toml
[tool.pytest.ini_options]
sift_autouse = false
sift_offline = true
sift_git_metadata = false
sift_grpc_uri = "your-org.sift.example:443"
sift_rest_uri = "https://your-org.sift.example"
```

To disable the plugin for a single run:
`pytest -p no:sift_client.pytest_plugin`.
"""

from .context_manager import NewStep, ReportContext

__all__ = [
    "NewStep",
    "ReportContext",
]
