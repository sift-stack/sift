# Pytest Plugin

The Sift Python client ships a pytest plugin that turns a pytest run into a
`TestReport` in Sift. Each test function becomes a `TestStep`, measurements
land as rows under that step, and failures propagate up through nested
substeps to the report itself.

This page walks through wiring the plugin into a project, the fixtures and
hooks it provides, and the patterns you'll use day-to-day.

!!! info "Where the plugin lives"
    The plugin is part of `sift_client.util.test_results`. It is **not**
    registered as a `pytest11` entry point. Projects opt in with a
    `from sift_client.util.test_results import *` in their `conftest.py`.
    That import is what wires up the fixtures, the CLI options, and the
    `pytest_runtest_makereport` hook.

## Install

```bash
pip install sift-stack-py pytest python-dotenv
```

Set the connection details in a `.env` next to your tests:

```bash
SIFT_API_KEY="your-api-key"
SIFT_GRPC_URI="..."
SIFT_REST_URI="..."
```

The `SIFT_GRPC_URI` and `SIFT_REST_URI` are the gRPC and REST endpoints for your Sift organization. You can find these on the Sift Manage page as well as generate an API key.

## Wire the plugin into `conftest.py`

Two things are required: a session-scoped `sift_client` fixture (the plugin's
`report_context` fixture resolves it by name), and a star-import that registers
the plugin's fixtures into the conftest's namespace.

```python title="conftest.py"
import os

import pytest
from dotenv import load_dotenv

from sift_client import SiftClient, SiftConnectionConfig

# Star-import wires fixtures + hooks + CLI options into pytest collection.
from sift_client.util.test_results import *

load_dotenv()


@pytest.fixture(scope="session")
def sift_client() -> SiftClient:
    grpc_url = os.getenv("SIFT_GRPC_URI")
    rest_url = os.getenv("SIFT_REST_URI")
    api_key = os.getenv("SIFT_API_KEY")
    
    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=api_key,
            grpc_url=grpc_url,
            rest_url=rest_url,
        )
    )
```

That's the whole setup. Every test in the session will now create a step on a
single shared `TestReport`.

## Plugin provided fixtures

| Name | Kind | Scope | Purpose |
|---|---|---|---|
| `report_context` | fixture (autouse) | session | The `ReportContext` backing the run's `TestReport`. Use it to attach metadata or open ad-hoc steps. |
| `step` | fixture (autouse) | function | A `NewStep` created for the current test function. Exposes `measure*`, `substep`, `report_outcome`, and `current_step`. |
| `module_substep` | fixture (autouse) | module | One step per test file with each function nested as a substep. |
| `client_has_connection` | fixture | session | Calls `sift_client.ping.ping()`; consulted only when `--sift-test-results-check-connection` is set. |

### CLI options

| Flag | Default | Effect |
|---|---|---|
| `--sift-test-results-log-file=<path\|true\|false>` | temp file | Where the JSONL log of create/update calls goes. With a log file set, the plugin spawns an `import-test-result-log --incremental` worker that polls the file and replays entries against Sift while the run is in flight. Pass `false` to disable the file entirely; create/update calls then go straight to the API synchronously during tests. |
| `--no-sift-test-results-git-metadata` | git metadata on | Skip capturing git repo/branch/commit on the report's metadata. |
| `--sift-test-results-check-connection` | off | Make `report_context`, `step`, and `module_substep` no-op (yield `None`) when `client_has_connection` is `False`. Lets the same suite run locally without a Sift backend. |

These can be set permanently in `pytest.ini`:

```ini title="pytest.ini"
[pytest]
addopts = --sift-test-results-check-connection
```

!!! warning "FedRAMP / shared environments"
    Pass `--sift-test-results-log-file=false` to skip the temp file + worker
    pipeline. Create/update calls then run inline against the API instead of
    being deferred through a subprocess.

### Report metadata captured automatically

Every report the plugin creates includes:

- `name` and `test_case`: derived from the first positional argument to `pytest`. When it resolves to an existing path the plugin uses the basename for `name` and the full path string for `test_case`; otherwise both fall back to `pytest <args>`. `name` always has a UTC ISO timestamp appended. See examples below.
- `test_system_name`: `socket.gethostname()`.
- `system_operator`: `getpass.getuser()`.
- `start_time` / `end_time`: set on session enter/exit.
- `status`: starts at `IN_PROGRESS`, finalized to `PASSED` or `FAILED` on session exit (failure if any step failed or an exception escaped the session).
- `metadata.git_repo`, `metadata.git_branch`, `metadata.git_commit`: captured via `git remote get-url origin` / `git rev-parse --abbrev-ref HEAD` / `git describe --always --dirty --exclude '*'`. Suppressed by `--no-sift-test-results-git-metadata` or when not in a git repo.

Example invocations:

| Pytest invocation | Report `name` | Report `test_case` |
|---|---|---|
| `pytest tests/test_battery.py` | `test_battery.py 2026-05-04T12:00:00.123456+00:00` | `tests/test_battery.py` |
| `pytest tests/` | `tests 2026-05-04T12:00:00.123456+00:00` | `tests` |
| `pytest -k voltage` | `pytest -k voltage 2026-05-04T12:00:00.123456+00:00` | `pytest -k voltage` |

To override defaults (e.g. set a serial number, system operator, or extra
metadata), call `report_context.report.update({...})` from any test or
fixture. See [Linking a Run](#linking-a-run-to-the-report) for the same
pattern applied to `run_id`.

## Basic usage

With the conftest in place, the simplest test needs nothing extra. The `step`
fixture is `autouse=True` and pytest test failures and skips are mapped to
step statuses automatically.

```python title="test_basic.py"
def test_no_fixtures_still_creates_a_step():
    """Autouse `step` records this function as a step on the session report."""
    assert 1 + 1 == 2


def test_measure_a_single_value(step):
    """Take `step` explicitly when you want to record a measurement."""
    voltage = 4.97
    passed = step.measure(
        name="battery_voltage",
        value=voltage,
        bounds={"min": 4.8, "max": 5.2},
        unit="V",
    )
    assert passed, f"voltage {voltage}V out of bounds"


def test_measure_strings_and_booleans(step):
    """`bounds` accepts a string or `True`/`False` for non-numeric values."""
    step.measure(name="firmware_version", value="1.4.2", bounds="1.4.2")
    step.measure(name="self_test_passed", value=True, bounds=True)


def test_docstring_becomes_step_description(step):
    """This docstring is the step's description in Sift.

    The plugin pulls `request.node.obj.__doc__` when it creates the step.
    Helper functions called from within the test do not get this treatment;
    pass `description="..."` explicitly on `substep(...)` instead.
    """
    assert step.current_step.description is not None
```

!!! tip "Measurements never raise"
    `step.measure(...)` returns `True` if the value is in bounds and `False`
    otherwise. A `False` result marks the enclosing step as failed but does
    not raise. Chain measurements freely and inspect the boolean if you need
    custom flow control.

### Status semantics for failures

The plugin uses the step exit handler in `NewStep.__exit__` to translate test
outcomes into `TestStatus`:

| Outcome | Resulting `TestStatus` |
|---|---|
| In-bounds measurements only | `PASSED` |
| Failed measurement, failed `report_outcome`, failed substep, or `AssertionError` raised by the test | `FAILED` (no traceback is attached, since pytest already prints it in the runner output) |
| Non-`AssertionError` exception escapes the test (e.g. `ValueError`, `TimeoutError`) | `ERROR`, with the formatted traceback (last 10 frames plus the first frame) on `step.error_info.error_message` |
| Manual `step.current_step.update({"status": ...})` | Whatever you set; the step exit handler honors a manually-resolved status |

A failure or error at any depth propagates upward: the parent substep, the
function step, the module step (if `module_substep` is active), and the
session report all get marked failed.

See [Pass/Fail Behavior](pass_fail_behavior.md) for the full mapping from
test outcomes to step statuses, propagation rules, and how to manually
override a step's status.

## Nested steps

Use `step.substep(name=...)` to open a child step. Substeps nest arbitrarily
deep, and a failure at any depth propagates up to fail the parent and the
report.

```python title="test_nested_steps.py"
import time


def test_phased_check(step):
    """Phase a single test into setup/exercise/verify substeps."""
    with step.substep(name="setup", description="Power on and wait for boot") as setup:
        setup.measure(name="boot_time_s", value=2.1, bounds={"max": 5.0}, unit="s")

    with step.substep(name="exercise", description="Drive the test sequence"):
        time.sleep(0.01)

    with step.substep(name="verify", description="Read final state") as verify:
        verify.measure(name="final_state", value="IDLE", bounds="IDLE")


def test_deeply_nested(step):
    """A failure at the bottom fails everyone above it."""
    with step.substep(name="level_1") as l1:
        with l1.substep(name="level_2") as l2:
            with l2.substep(name="level_3") as l3:
                l3.measure(name="leaf_value", value=42, bounds={"min": 0, "max": 100})
```

Each step gets a hierarchical `step_path` (`1`, `1.1`, `1.1.2`, `2`, …)
assigned by `ReportContext`. Sibling substeps within the same parent
auto-increment; opening a new top-level step starts a new branch.

### One step per file

`module_substep` is autouse and module-scoped. When it's active (it's pulled
in by the star-import in `conftest.py`), each file becomes a parent step and
every function in it nests one level down. Its name is the test file's
basename and its description is the module's docstring (if any).

### Linking a Run to the report

`report_context` is the session-scoped fixture; mutating it in one test
affects the whole report.

```python
def test_link_run_to_report(report_context, sift_client):
    run = sift_client.runs.create(...)  # however you create your run
    report_context.report.update({"run_id": run.id_})
```

The same `update({...})` pattern works for any field on `TestReportUpdate`,
including `serial_number`, `part_number`, `system_operator`, and `metadata`.

## How pytest layout maps to a Sift report

The plugin builds the report tree by hooking pytest's collection: every test
node it sees becomes a step. What you control is which constructs create
nodes and where you nest substeps inside them. Common layouts and the
resulting report trees:

### Flat module of test functions

The default. Each function is one step directly under the report.

```python title="test_battery.py"
def test_voltage(step): ...
def test_current(step): ...
def test_temperature(step): ...
```

```text title="Sift report"
TestReport
├── test_voltage
├── test_current
└── test_temperature
```

### One step per file with `module_substep`

`module_substep` is autouse and module-scoped. Every file becomes a parent
step and every function in it nests one level down.

```python title="test_battery.py"
def test_voltage(step): ...
def test_current(step): ...
```

```python title="test_thermal.py"
def test_idle_temp(step): ...
def test_load_temp(step): ...
```

```text title="Sift report"
TestReport
├── test_battery.py
│   ├── test_voltage
│   └── test_current
└── test_thermal.py
    ├── test_idle_temp
    └── test_load_temp
```

### Test classes

Pytest classes (`class TestFoo: ...`) do not create a parent step on their
own. The plugin keys off the test node's `name`, which is just the method
name. To group a class's methods under a class-level step, add a class-scoped
fixture that opens a step with `report_context.new_step(...)`:

```python title="test_charging.py"
import pytest


class TestCharging:
    @pytest.fixture(scope="class", autouse=True)
    def class_step(self, report_context):
        with report_context.new_step(
            name="TestCharging",
            description="Charging subsystem",
        ) as parent:
            yield parent

    def test_starts_at_zero(self, step): ...
    def test_reaches_full(self, step): ...
    def test_thermal_throttle(self, step): ...
```

```text title="Sift report"
TestReport
└── TestCharging
    ├── test_starts_at_zero
    ├── test_reaches_full
    └── test_thermal_throttle
```

!!! note "Combining with `module_substep`"
    `module_substep` and a class-scoped step both open at module/class scope,
    so they each grab the next sibling slot under the report and the inner
    one nests under the outer. If you want both layers (file → class →
    method), make the class step itself open via the active outer step
    rather than the report root.

### Parametrized tests

Each parametrize case is a distinct pytest node, so each gets its own step.
The step name includes the parameter id pytest generates.

```python
@pytest.mark.parametrize("voltage", [3.3, 5.0, 12.0])
def test_rail(step, voltage):
    step.measure(name="rail_v", value=voltage, bounds={"min": 0.0})
```

```text title="Sift report"
TestReport
├── test_rail[3.3]
├── test_rail[5.0]
└── test_rail[12.0]
```

### Helper functions

Helpers called from a test do not auto-create a step. The plugin only sees
pytest-collected nodes. To represent helper work in the report, open a
substep at the call site and pass it into the helper:

```python
def measure_rail(step, name, value, bounds):
    return step.measure(name=name, value=value, bounds=bounds, unit="V")


def test_power_rails(step):
    with step.substep(name="3.3V rail") as rail_3v3:
        measure_rail(rail_3v3, "rail_v", 3.31, {"min": 3.2, "max": 3.4})

    with step.substep(name="5V rail") as rail_5v:
        measure_rail(rail_5v, "rail_v", 5.02, {"min": 4.9, "max": 5.1})
```

```text title="Sift report"
TestReport
└── test_power_rails
    ├── 3.3V rail
    │   └── rail_v        (measurement)
    └── 5V rail
        └── rail_v        (measurement)
```

!!! tip "Docstring-as-description is top-level only"
    The plugin reads the test function's docstring and uses it as the step
    description. Docstrings on helper functions are not picked up. Pass
    `description="..."` explicitly on `substep(...)` if you want one.

### Fixtures that contribute steps

A fixture can open its own substep around setup/teardown by using `step` (for
function-scope) or `report_context.new_step(...)` (for any scope). The substep
ends when the fixture's `yield` returns, which makes the report tree mirror
the lifecycle.

```python
@pytest.fixture
def warmed_up_dut(step):
    with step.substep(name="warmup", description="Bring DUT to operating temp"):
        # ... do warmup work ...
        yield "dut-handle"


def test_steady_state(step, warmed_up_dut):
    step.measure(name="temp_c", value=37.2, bounds={"min": 35.0, "max": 40.0})
```

```text title="Sift report"
TestReport
└── test_steady_state
    ├── warmup        (from fixture)
    └── temp_c        (measurement)
```

## Measurement variants

`step.measure(...)` records exactly one measurement. For datasets coming off a
sensor or calculated channel, use one of the bulk variants.

### `measure_avg`: one row, the mean

`measure_avg` accepts a Python list, a NumPy array, or a pandas `Series`,
takes the mean, and evaluates it against bounds.

```python
import numpy as np
import pandas as pd


def test_avg_with_list(step):
    samples = [4.97, 5.01, 5.03, 4.99, 5.02]
    step.measure_avg(
        name="bus_voltage_avg",
        values=samples,
        bounds={"min": 4.9, "max": 5.1},
        unit="V",
    )


def test_avg_with_numpy(step):
    samples = np.linspace(99.5, 100.5, num=50)
    step.measure_avg(
        name="cpu_temp_avg",
        values=samples,
        bounds={"min": 95.0, "max": 105.0},
        unit="C",
    )


def test_avg_with_pandas(step):
    series = pd.Series([0.998, 1.001, 0.999, 1.002, 1.000])
    step.measure_avg(
        name="reference_clock_ratio",
        values=series,
        bounds={"min": 0.99, "max": 1.01},
    )
```

### `measure_all`: only out-of-bounds rows

Records measurements only for samples that fail bounds, so an all-pass
dataset of N samples doesn't add N rows to the report. Returns `True` when
every sample is in bounds.

```python
def test_only_outliers_recorded(step):
    samples = [10.1, 10.2, 10.3, 99.9, 10.0, 10.1]  # 99.9 is the outlier
    all_in_bounds = step.measure_all(
        name="pressure_psi",
        values=samples,
        bounds={"min": 9.0, "max": 11.0},
        unit="psi",
    )
    # Returns False because 99.9 is out of bounds. The step is already
    # marked failed; raise here only if you also want pytest to fail.
    assert all_in_bounds
```

!!! note "`measure_all` requires at least one bound"
    Passing `bounds={}` raises `ValueError("No bounds provided")`. At
    least one of `min` or `max` must be set.

### `report_outcome`: externally computed pass/fail

When the decision is computed elsewhere, drop it onto the report as a
named substep with an optional reason. Returns the result you passed in,
so you can use it inline.

```python
def test_external_checks(step):
    step.report_outcome(
        name="config_loaded",
        result=True,
        reason="loaded /etc/dut/config.yaml",
    )

    # Failures show up as a failed substep without raising.
    rare_warning_seen = False
    step.report_outcome(
        name="no_rare_warning",
        result=not rare_warning_seen,
        reason="grep'd dmesg for the known-flaky warning",
    )
```

### Bounds reference

| Pass to `bounds=` | Value type | Effect |
|---|---|---|
| `{"min": x, "max": y}` (either key optional) | `int` / `float` | Numeric window. One-sided is fine. |
| `NumericBounds(min=x, max=y)` | `int` / `float` | Same as the dict form, explicit. |
| `"expected-string"` | `str` (or `bool`) | Exact equality. For `bool` values, compares lowercased string (`"true"`/`"false"`). |
| `True` or `False` | `bool` (or `str`) | Exact equality. For `str` values, compares lowercased strings. |
| `None` | any | Records the value but does not evaluate it; measurement is recorded as `passed=True`. |

The `unit` argument is a free-form string label (e.g. `"V"`, `"C"`, `"psi"`).

## Skip handling

- `@pytest.mark.skip` and `@pytest.mark.skipif`: the plugin's
  `pytest_runtest_makereport` hook sees the skipped outcome and creates a
  step with `TestStatus.SKIPPED`.
- Inside a test function, you can mark just one substep as skipped without
  aborting the whole test:

  ```python
  from sift_client.sift_types.test_report import TestStatus


  def test_runtime_skip(step):
      with step.substep(name="optional_calibration") as cal:
          if not precondition_met():
              cal.current_step.update(
                  {"status": TestStatus.SKIPPED},
                  log_file=step.report_context.log_file,
              )
  ```

  A manually-resolved status is honored by the step's exit handler. No
  further bookkeeping required. `SKIPPED` does not propagate as a failure.

## Running the suite

```bash
# Full run against your Sift tenant
pytest

# Pin the log file so you can replay it later if the import worker dies
pytest --sift-test-results-log-file=./sift-results.jsonl
```

See [Running offline](#running-offline) for the same suite running with or
without a reachable Sift server.

## Running offline

The plugin supports two offline workflows, depending on whether you want a
Sift report at all when the test environment can't reach Sift. The first
turns the plugin into a no-op when the server is unreachable. The second
keeps the plugin running normally and writes every create/update to a local
JSONL file that you upload from a connected machine afterward.

| Pattern | Flag | Runtime behavior | Follow-up |
|---|---|---|---|
| Skip when offline | `--sift-test-results-check-connection` | Fixtures yield `None`, no log file, no report. Pytest still reports pass/fail. | None. |
| Capture locally, upload later | `--sift-test-results-log-file=<path>` | Plugin writes every create/update to the JSONL file. | `import-test-result-log <path>` from a connected machine. |

Pattern 1 suits laptop dev and CI without Sift secrets. Pattern 2 suits
field tests, vehicles on remote sites, and air-gapped labs.

### Pattern 1: skip when offline

`--sift-test-results-check-connection` makes the plugin ping Sift once at
session start through the `client_has_connection` fixture (which by default
calls `sift_client.ping.ping()`). On a failed ping, `report_context`,
`step`, and `module_substep` yield `None` for the rest of the session.
Pytest still runs the tests and still reports pass/fail.

```bash
pytest --sift-test-results-check-connection
```

```ini title="pytest.ini"
[pytest]
addopts = --sift-test-results-check-connection
```

#### Handling `None` in tests

Calls on `step` raise `AttributeError` when it's `None`, so tests that take
`step` as a parameter need a guard. The cleanest fix is to shadow the
plugin's `step` fixture in your conftest and turn the `None` case into an
automatic skip.

```python title="conftest.py"
import pytest

from sift_client.util.test_results import *


@pytest.fixture(autouse=True)
def step(step):
    if step is None:
        pytest.skip("Sift unavailable")
    yield step
```

The `step` parameter on the override resolves to the plugin's fixture, not
to the override itself. `autouse=True` is required so the skip applies to
tests that don't request `step` directly. The same shadowing trick works
for `module_substep` and `report_context`.

For one-off tests that don't share a conftest, an inline guard works just
as well:

```python
def test_battery_voltage(step):
    if step is None:
        pytest.skip("Sift unavailable")
    step.measure(name="battery_voltage", value=4.97, bounds={"min": 4.8, "max": 5.2})
```

If you'd rather have tests pass through silently than skip them, wrap the
calls in a helper that no-ops on `None`:

```python
def safe_measure(step, **kwargs):
    if step is None:
        return True
    return step.measure(**kwargs)
```

#### Overriding the connection check

The default `client_has_connection` fixture calls `sift_client.ping.ping()`.
Override it in your conftest if pinging is the wrong signal for your
environment, for example a token cache that's only warm when authenticated:

```python title="conftest.py"
from pathlib import Path

import pytest


@pytest.fixture(scope="session")
def client_has_connection(sift_client) -> bool:
    return Path("~/.sift-token-cache").expanduser().is_file()
```

The plugin only consults this fixture when `--sift-test-results-check-connection`
is set, so an unused override has no effect on a normal run.

### Pattern 2: capture locally, upload later

This pattern keeps the plugin running normally even when Sift is
unreachable. The plugin writes to the log file, the worker dies on connect,
and the file is left on disk for you to upload later. Pin the log file path
so you can find it afterward, and don't pass
`--sift-test-results-check-connection`, which would suppress the logging
this pattern relies on.

```bash
pytest --sift-test-results-log-file=./run.jsonl
```

What happens during the run:

- Every report, step, and measurement create/update is written to
  `run.jsonl`. The plugin doesn't contact the Sift API for any of these
  calls; they return simulated responses keyed by UUIDs that the replay
  later maps to real IDs.
- The `import-test-result-log --incremental` worker subprocess starts and
  exits early when it can't reach Sift. The session does not fail when the
  worker exits before the run ends.
- Tests run against a real `step` fixture, so `step.measure(...)`,
  substeps, parametrize, fixtures, and `module_substep` behave exactly as
  they do online. No conftest changes are needed.

Once you have connectivity, replay the file:

```bash
import-test-result-log ./run.jsonl
```

The replay creates the report, steps, and measurements against Sift in one
batch. See [Replaying a saved log file](#replaying-a-saved-log-file) for
details on cleanup and the incremental flag.

!!! warning "Pin the log path for Pattern 2"
    Without `--sift-test-results-log-file=<path>`, the plugin writes to a
    `tempfile.NamedTemporaryFile` and only surfaces the path via a
    `logger.info` line. Always pin a known path when you intend to replay
    the file later.

## Replaying a saved log file

When the worker doesn't finish cleanly the plugin will print a hint mentioning
`import-test-result-log`. To import:

```bash
import-test-result-log <path-to-log.jsonl>
```

That replays the saved JSONL log as a single batch (no `--incremental`) and
deletes the file when it lives under the system temp dir.