# Report structure

The report tree mirrors your test layout. Every Python package, test module,
test class, and parametrize axis above a test becomes a parent step, and you can
open arbitrary substeps inside a test. This page covers the layout-to-tree
mapping, the measurement variants you record into it, and the metadata the
plugin captures for you.

## Recording measurements

With the conftest in place, the simplest test needs nothing extra. The `step`
fixture is `autouse=True` and pytest test failures and skips are mapped to step
statuses automatically.

```python title="test_basic.py"
def test_no_fixtures_still_creates_a_step():
    """Autouse `step` records this function as a step on the session report."""
    assert 1 + 1 == 2


def test_measure_a_single_value(step):
    """Take `step` explicitly when you want to record a measurement."""
    voltage = 4.97
    step.measure(
        name="battery_voltage",
        value=voltage,
        bounds={"min": 4.8, "max": 5.2},
        unit="V",
    )
    # An out-of-bounds measurement already marks the step FAILED. Call this at
    # the end to also fail pytest, without an assertion message in error_info.
    step.pytest_fail_if_step_failed()


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
    otherwise. A `False` result marks the enclosing step as failed but does not
    raise. Chain measurements freely and inspect the boolean if you need custom
    flow control. For how outcomes map to `TestStatus` and propagate upward, see
    [Pass/Fail Behavior](pass_fail_behavior.md).

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

Each step gets a hierarchical `step_path` (`1`, `1.1`, `1.1.2`, `2`, …) assigned
by `ReportContext`. Sibling substeps within the same parent auto-increment;
opening a new top-level step starts a new branch.

### Mirroring the test layout

The plugin opens a parent step for each Python package (`__init__.py`
directory), test file, and test class above every test, plus a parent step for
each `@pytest.mark.parametrize` axis. Every layer is on by default and
individually opt-out via ini flags (`sift_package_step`, `sift_module_step`,
`sift_class_step`, `sift_parametrize_nesting`). Class/module/package docstrings
become the matching step's description.

A parent step is created `IN_PROGRESS` and resolves to its final status as soon
as the last test in its subtree finishes, independent of test execution order,
so with incremental upload the report tree fills in progressively rather than
all at once at the end. Its time window spans from its first test starting to its
last test finishing.

### Linking a Run to the report

`report_context` is the session-scoped fixture; mutating it in one test affects
the whole report.

```python
def test_link_run_to_report(report_context, sift_client):
    run = sift_client.runs.create(...)  # however you create your run
    report_context.report.update({"run_id": run.id_})
```

The same `update({...})` pattern works for any field on `TestReportUpdate`,
including `serial_number`, `part_number`, `system_operator`, and `metadata`.

## How pytest layout maps to a Sift report

The plugin builds the report tree by hooking pytest's collection: every test
node it sees becomes a step. What you control is which constructs create nodes
and where you nest substeps inside them. Common layouts and the resulting report
trees:

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

### Modules nested under a package

Two test files under the same Python package (directory with `__init__.py`)
share that package step as their parent.

```python title="suites/__init__.py"
```

```python title="suites/test_battery.py"
def test_voltage(step): ...
def test_current(step): ...
```

```python title="suites/test_thermal.py"
def test_idle_temp(step): ...
def test_load_temp(step): ...
```

```text title="Sift report"
TestReport
└── suites
    ├── test_battery.py
    │   ├── test_voltage
    │   └── test_current
    └── test_thermal.py
        ├── test_idle_temp
        └── test_load_temp
```

!!! note "A package step appears for any directory with `__init__.py`"
    The package step comes from pytest collecting a directory with an
    `__init__.py` as a `pytest.Package`. A directory without one is a
    `pytest.Dir`, which the plugin skips. So a `tests/__init__.py` adds a
    `tests` step to every report, which is often not what you want.

    To drop a single unwanted package step, delete that directory's
    `__init__.py`. This is safe when you use `--import-mode=importlib` (which
    needs no `__init__.py`) and your tests have no package-relative imports
    (`from . import ...`). Under the default `prepend` import mode, removing
    `__init__.py` can cause import collisions when test files share a basename
    across directories, so keep it there.

    To drop package steps everywhere, set `sift_package_step = false`. No pytest
    setting (`testpaths`, `rootdir`, …) removes a package step on its own. The
    step exists if and only if the directory has an `__init__.py`.

### Test classes (and nested classes)

`class TestFoo:` and `class TestOuter: class TestInner:` produce class and
nested class steps automatically, with no manual fixture needed.

```python title="test_charging.py"
class TestCharging:
    """Charging subsystem."""

    def test_starts_at_zero(self, step): ...
    def test_reaches_full(self, step): ...
    def test_thermal_throttle(self, step): ...
```

```text title="Sift report"
TestReport
└── test_charging.py
    └── TestCharging
        ├── test_starts_at_zero
        ├── test_reaches_full
        └── test_thermal_throttle
```

The class's docstring becomes the step description.

### Parametrized tests

Parametrized tests cluster under a parent step named after the test function,
with one inner parent per parametrize axis (outer-to-inner in decorator-on-page
order). Stacked parametrize produces nested step levels.

```python
@pytest.mark.parametrize("voltage", [3.3, 5.0, 12.0])
def test_rail(step, voltage):
    step.measure(name="rail_v", value=voltage, bounds={"min": 0.0})
```

```text title="Sift report"
TestReport
└── test_module.py
    └── test_rail
        ├── voltage=3.3
        ├── voltage=5.0
        └── voltage=12.0
```

Stacked parametrize:

```python
@pytest.mark.parametrize("voltage", ["high", "low"])
@pytest.mark.parametrize("component", ["motor", "valve"])
def test_iso(step, voltage, component): ...
```

```text title="Sift report"
TestReport
└── test_module.py
    └── test_iso
        ├── voltage='high'
        │   ├── component='motor'
        │   └── component='valve'
        └── voltage='low'
            ├── component='motor'
            └── component='valve'
```

Set `sift_parametrize_nesting = false` in `pytest.ini` to fall back to flat leaf
names (`test_rail[3.3]`).

#### Human-readable labels

Each axis defaults to a `name=value` label. Supply
`ids=` to name it yourself: a list, or a callable factory pytest calls with
each value. This works on `@pytest.mark.parametrize` and on parametrized
fixtures alike:

```python
@pytest.mark.parametrize("voltage", [3.3, 5.0], ids=["nominal", "boosted"])
def test_rail(step, voltage): ...
```

```text title="Sift report"
TestReport
└── test_module.py
    └── test_rail
        ├── nominal
        └── boosted
```

#### Scope-based placement

The examples above use function-scoped parametrize,
which nests under the test. A parametrized *fixture* is placed at its own scope
instead: a class-scoped fixture param wraps the class's methods, a module-scoped
one wraps the module's tests, and a session-scoped one sits at the report root.
A `@pytest.mark.parametrize(..., scope="module")` follows the scope it names.
This keeps the tree matching how pytest re-runs work: broader scope
nests outside narrower.

### Helper functions

Helpers called from a test do not auto-create a step. The plugin only sees
pytest-collected nodes. To represent helper work in the report, open a substep
at the call site and pass it into the helper:

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
ends when the fixture's `yield` returns, which makes the report tree mirror the
lifecycle.

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

`measure_avg` accepts a Python list, a NumPy array, or a pandas `Series`, takes
the mean, and evaluates it against bounds.

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

Records measurements only for samples that fail bounds, so an all-pass dataset
of N samples doesn't add N rows to the report. Returns `True` when every sample
is in bounds.

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
    # marked failed; call this only if you also want pytest to fail.
    step.pytest_fail_if_step_failed()
```

!!! note "`measure_all` requires at least one bound"
    Passing `bounds={}` raises `ValueError("No bounds provided")`. At least one
    of `min` or `max` must be set.

### `report_outcome`: externally computed pass/fail

When the decision is computed elsewhere, drop it onto the report as a named
substep with an optional reason. Returns the result you passed in, so you can
use it inline.

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

## Report metadata captured automatically

Every report the plugin creates includes:

- `name` and `test_case`: derived from the first positional argument to `pytest`. When it resolves to an existing path the plugin uses the basename for `name` and the full path string for `test_case`; otherwise both fall back to `pytest <args>`. `name` always has a UTC ISO timestamp appended. See examples below.
- `test_system_name`: `socket.gethostname()`.
- `system_operator`: `getpass.getuser()`.
- `start_time` / `end_time`: set on session enter/exit.
- `status`: starts at `IN_PROGRESS`, finalized to `PASSED` or `FAILED` on session exit (failure if any step failed or an exception escaped the session).
- `metadata.git_repo`, `metadata.git_branch`, `metadata.git_commit`: captured via `git remote get-url origin` / `git rev-parse --abbrev-ref HEAD` / `git describe --always --dirty --exclude '*'`. Suppressed by `--no-sift-git-metadata` or when not in a git repo.

Example invocations:

| Pytest invocation | Report `name` | Report `test_case` |
|---|---|---|
| `pytest tests/test_battery.py` | `test_battery.py 2026-05-04T12:00:00.123456+00:00` | `tests/test_battery.py` |
| `pytest tests/` | `tests 2026-05-04T12:00:00.123456+00:00` | `tests` |
| `pytest -k voltage` | `pytest -k voltage 2026-05-04T12:00:00.123456+00:00` | `pytest -k voltage` |

To override defaults (e.g. set a serial number, system operator, or extra
metadata), call `report_context.report.update({...})` from any test or fixture.
See [Linking a Run](#linking-a-run-to-the-report) for the same pattern applied
to `run_id`.
