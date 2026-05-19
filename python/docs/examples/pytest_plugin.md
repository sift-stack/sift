# Pytest Plugin

The Sift Python client ships a pytest plugin that turns a pytest run into a
`TestReport` in Sift. Each test function becomes a `TestStep`, measurements
land as rows under that step, and failures propagate up through nested
substeps to the report itself.

This page walks through wiring the plugin into a project, the fixtures and
hooks it provides, and the patterns you'll use day-to-day.

!!! info "Where the plugin lives"
    The plugin lives at `sift_client.pytest_plugin`. It is
    **not** registered as a `pytest11` entry point. Projects opt in with a
    `pytest_plugins` declaration in their top-level `conftest.py`. Pytest
    then loads the module as a real plugin: the fixtures, CLI options, and
    `pytest_runtest_makereport` hook all register through standard pytest
    machinery, so `pytest --trace-config` lists it and
    `pytest -p no:sift_client.pytest_plugin` disables it.

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

A single `pytest_plugins` declaration in your top-level `conftest.py` is all
that's required. The plugin ships a default `sift_client` fixture that reads
`SIFT_API_KEY`, `SIFT_GRPC_URI`, and `SIFT_REST_URI` from the environment.

```python title="conftest.py"
from dotenv import load_dotenv

load_dotenv()

pytest_plugins = ["sift_client.pytest_plugin"]
```

That's the whole setup. Every test in the session will now create a step on a
single shared `TestReport`.

### Customizing the `SiftClient`

To construct the client differently (custom TLS, timeouts, alternate
credentials, etc.), override the `sift_client` fixture in your conftest. The
plugin's default falls away in favor of your definition.

```python title="conftest.py"
import os

import pytest
from dotenv import load_dotenv

from sift_client import SiftClient, SiftConnectionConfig

load_dotenv()

pytest_plugins = ["sift_client.pytest_plugin"]


@pytest.fixture(scope="session")
def sift_client() -> SiftClient:
    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=os.getenv("SIFT_API_KEY"),
            grpc_url=os.getenv("SIFT_GRPC_URI"),
            rest_url=os.getenv("SIFT_REST_URI"),
            use_ssl=False,
        )
    )
```

## Plugin provided fixtures

| Name | Kind | Scope | Purpose |
|---|---|---|---|
| `report_context` | fixture (autouse) | session | The `ReportContext` backing the run's `TestReport`. Use it to attach metadata or open ad-hoc steps. |
| `step` | fixture (autouse) | function | A `NewStep` created for the current test function. Exposes `measure*`, `substep`, `report_outcome`, and `current_step`. |
| `module_substep` | fixture (autouse) | module | One step per test file with each function nested as a substep. |
| `client_has_connection` | fixture | session | Calls `sift_client.ping.ping()`; consulted by `report_context` at session start in online mode (the default). Override to skip the ping or use a different reachability signal. |

### CLI options

| Flag | Default | Effect |
|---|---|---|
| `--sift-offline` | off (online) | Skip the session-start ping and don't contact Sift. All create/update calls go to the JSONL log file for later replay via `import-test-result-log`. Missing `SIFT_*` env vars are tolerated; placeholders are filled. |
| `--sift-disabled` | off | Skip Sift entirely. Autouse fixtures yield stub objects; `step.measure(...)` still returns real pass/fail booleans by evaluating bounds locally. Nothing is sent to Sift and no log file is written. Also honored via `SIFT_DISABLED=1`. Incompatible with `--sift-offline`. |
| `--sift-test-results-log-file=<path\|true\|false>` | temp file | Where the JSONL log of create/update calls goes. With a log file set, the plugin spawns an `import-test-result-log --incremental` worker that polls the file and replays entries against Sift while the run is in flight. Pass `false` to disable the file entirely; create/update calls then go straight to the API synchronously during tests. Incompatible with `--sift-offline` since offline mode needs the log file as its sole sink. |
| `--no-sift-test-results-git-metadata` | git metadata on | Skip capturing git repo/branch/commit on the report's metadata. |

These can be passed permanently via `addopts`:

```ini title="pytest.ini"
[pytest]
addopts = --sift-offline
```

Or set the matching ini key directly (recommended for stable per-project
configuration). Each CLI flag has a corresponding key under
`[tool.pytest.ini_options]` in `pyproject.toml` or `[pytest]` in `pytest.ini`.
CLI flags, when passed, override the ini values.

| Ini key | Type | Equivalent CLI flag |
|---|---|---|
| `sift_test_results_log_file` | string (`true` / `false` / `none` / path) | `--sift-test-results-log-file=<value>` |
| `sift_test_results_git_metadata` | bool (default `true`) | `--no-sift-test-results-git-metadata` (sets to `false`) |
| `sift_offline` | bool (default `false`) | `--sift-offline` |
| `sift_disabled` | bool (default `false`) | `--sift-disabled` (also honors `SIFT_DISABLED` env var) |
| `sift_test_results_autouse` | bool (default `true`) | _(no CLI flag; controls the marker gate below)_ |

The default `sift_client` fixture reads its two URIs from environment first
and falls back to ini keys when the env vars are unset. `SIFT_API_KEY` is
intentionally env-only — keep it out of source control and supply it through
`pytest-dotenv` (see [API key handling](#api-key-handling) below). The env
var wins when both are set, so secrets injected into a CI environment
continue to override values committed to `pyproject.toml`. There are no CLI
flags for credentials.

| Ini key | Environment variable | Notes |
|---|---|---|
| _(none)_ | `SIFT_API_KEY` | Env-only. Use `.env` + `pytest-dotenv` locally; inject from your secret store in CI. |
| `sift_grpc_uri` | `SIFT_GRPC_URI` | Stable per-org gRPC endpoint; safe to commit. |
| `sift_rest_uri` | `SIFT_REST_URI` | Stable per-org REST endpoint; safe to commit. |

```toml title="pyproject.toml"
[tool.pytest.ini_options]
sift_offline = true
sift_test_results_git_metadata = false
sift_grpc_uri = "your-org.sift.example:443"
sift_rest_uri = "https://your-org.sift.example"
```

```ini title="pytest.ini"
[pytest]
sift_offline = true
sift_test_results_git_metadata = false
sift_grpc_uri = your-org.sift.example:443
sift_rest_uri = https://your-org.sift.example
```

#### API key handling

`SIFT_API_KEY` is deliberately read from the process environment only. The
recommended workflow uses the
[`pytest-dotenv`](https://pypi.org/project/pytest-dotenv/) plugin (already a
dependency of `sift-stack-py`), which loads variables from a `.env` file
into `os.environ` before tests run.

1. Add `.env` to `.gitignore`.
2. Drop your key into `.env` at the project root:

    ```bash title=".env"
    SIFT_API_KEY=sk-...your-key...
    ```

3. In CI, set `SIFT_API_KEY` directly via your provider's secret manager
   instead of committing a `.env` file.

`pytest-dotenv` picks the file up automatically; no `pytest_configure`
glue is needed.

!!! warning "FedRAMP / shared environments"
    Pass `--sift-test-results-log-file=false` (or set the ini key to `"false"`)
    to skip the temp file + worker pipeline. Create/update calls then run
    inline against the API instead of being deferred through a subprocess.

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

## Controlling which tests produce reports

By default every test in the session produces a Sift step. Two markers
and one ini key let you narrow that to a specific set of tests, which is
useful when a repo holds tests that you don't want included in the Sift test report.

| Setting                                                 | Effect                                                                                       |
|---------------------------------------------------------|----------------------------------------------------------------------------------------------|
| `sift_test_results_autouse = false` in `pyproject.toml` | Flip the project-wide default off. Tests no longer produce steps unless explicitly opted in. |
| `@pytest.mark.sift_include` on a test, class, or module | Force reporting on for that scope, regardless of the project default.                        |
| `@pytest.mark.sift_exclude` on a test, class, or module | Force reporting off for that scope, regardless of the project default.                       |

Closest marker determines setting. `sift_exclude` beats `sift_include` when both apply.
`pytestmark` at the class or module level inherits to every test in scope.

### Bulk-applying a marker to a directory

To opt an entire directory in (or out) without editing each file, hook
`pytest_collection_modifyitems` in the directory's `conftest.py`:

```python title="tests/example/conftest.py"
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

This applies `sift_include` to every test collected under `tests/example/`.
Combine with `sift_test_results_autouse = false` in `pyproject.toml` for
opting in to specific directories. 

`pytest_collection_modifyitems` receives every item in the session, not just
this directory's, so the `relative_to` filter is what scopes the marker.

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
              cal.current_step.update({"status": TestStatus.SKIPPED})
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

See [Running modes](#running-modes) for the offline and disabled flags
that let the same suite run without (or without contacting) Sift.

## Running modes

The plugin runs in one of three modes, selected at invocation:

| Mode | Flag | Network | Log file | `step.measure(...)` | Use case |
|---|---|---|---|---|---|
| Online (default) | _(none)_ | yes (pings at session start, fail-fast) | optional write-through backup | real measurement against Sift | CI with Sift credentials, local dev hitting your tenant |
| Offline | `--sift-offline` | none | required (the sole sink) | real measurement queued to log | field tests, vehicles, air-gapped labs, CI without network |
| Disabled | `--sift-disabled` | none | none | local bounds eval returning a real bool | local dev / CI without Sift credentials or where reports aren't wanted |

`--sift-offline` and `--sift-disabled` are mutually exclusive — disabled
writes nothing, offline writes everything to a log file.

### Online mode (default)

`report_context` resolves the `client_has_connection` fixture at session
start. The default implementation calls `sift_client.ping.ping()`. A failed
ping aborts the whole session with `pytest.UsageError`, naming both
`--sift-offline` and `--sift-disabled` as escape hatches. This is fail-fast
on purpose: a CI run that silently no-ops because the network was flaky
won't be noticed until someone goes looking for the report.

#### Overriding the connection check

Override `client_has_connection` in your conftest when pinging is the wrong
signal for your environment, for example a token cache that's only warm
when authenticated:

```python title="conftest.py"
from pathlib import Path

import pytest


@pytest.fixture(scope="session")
def client_has_connection(sift_client) -> bool:
    return Path("~/.sift-token-cache").expanduser().is_file()
```

The override has no effect under `--sift-offline` or `--sift-disabled`,
both of which skip the check entirely.

### Offline mode (`--sift-offline`)

Offline mode keeps the plugin running normally — same fixtures, same
`step.measure(...)` semantics — but routes every create/update through the
JSONL log file instead of the Sift API. The session-start ping is skipped,
missing `SIFT_*` env vars are tolerated (placeholders are filled), and the
`import-test-result-log --incremental` worker subprocess is not spawned at
session end. The log file is the sole sink.

```bash
pytest --sift-offline --sift-test-results-log-file=./run.jsonl
```

Once you have connectivity, replay the file:

```bash
import-test-result-log ./run.jsonl
```

The replay creates the report, steps, and measurements against Sift in one
batch. See [Replaying a saved log file](#replaying-a-saved-log-file) for
details on cleanup and the incremental flag.

`--sift-test-results-log-file=none` is rejected as a usage error under
`--sift-offline` since the log file is the sole sink — without it, results
are lost.

!!! warning "Pin the log path"
    Without `--sift-test-results-log-file=<path>`, offline mode writes to a
    `tempfile.NamedTemporaryFile` and only surfaces the path via a
    `logger.info` line. Always pin a known path when you intend to replay
    the file later.

### Disabled mode (`--sift-disabled`)

Disabled mode skips Sift entirely. The plugin stays loaded so the autouse
fixtures and markers still exist, but `report_context`, `step`, and
`module_substep` yield stub objects. `step.measure(...)`, `step.measure_avg(...)`,
`step.measure_all(...)`, and `step.substep(...)` all keep working — bounds
are evaluated locally and return the real pass/fail boolean — but nothing
is sent to Sift and no log file is written. No `SIFT_*` env vars or ini
keys are read.

Three ways to opt in, in order of how a project typically reaches for them:

```bash
# Per-environment: set once in .envrc, devcontainer, or CI job config
export SIFT_DISABLED=1

# Per-invocation: explicit kill-switch on the command line
pytest --sift-disabled

# Per-project as the default mode (rare; usually online is the default)
# pyproject.toml:
#   [tool.pytest.ini_options]
#   sift_disabled = true
```

Use this for local dev without Sift credentials, for open-source consumers
of a library that uses Sift internally, and for CI jobs (e.g. PR runs)
that shouldn't pollute the report stream with intermediate results.

## Replaying a saved log file

When the worker doesn't finish cleanly the plugin will print a hint mentioning
`import-test-result-log`. To import:

```bash
import-test-result-log <path-to-log.jsonl>
```

That replays the saved JSONL log as a single batch (no `--incremental`) and
deletes the file when it lives under the system temp dir.