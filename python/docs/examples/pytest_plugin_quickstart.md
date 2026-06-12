# Pytest Plugin Quickstart

A walkthrough of the runnable demo at
[`python/examples/pytest_plugin/`](https://github.com/sift-stack/sift/tree/main/python/examples/pytest_plugin).
The demo is a self-contained pytest project that exercises every layer of the
plugin's step tree: packages, modules, classes (including nested), parametrize
axes, manual substeps, and gate markers. It also includes a tests directory
that uses no Sift APIs at all, to show how the autouse fixtures capture plain
pytest tests for free.

For a conceptual reference (fixtures, ini flags, status semantics), see the
[Pytest Plugin guide](../guides/pytest_plugin/index.md).

## Project layout

```
examples/pytest_plugin/
├── conftest.py                            # registers the plugin
├── pyproject.toml                         # pytest knobs + report name/test_case/metadata
├── .env.example                           # credential template
└── tests/
    ├── pytest_only/                       # subpackage step
    │   ├── __init__.py
    │   └── test_pytest_only_demo.py       # plain pytest, no Sift APIs
    └── with_sift/                         # subpackage step
        ├── __init__.py
        └── test_with_sift_demo.py         # measurements, substeps, classes, parametrize, gates
```

Every Python package (directory with `__init__.py`), test file, and test class
above each test becomes its own parent step in the report tree.

## `conftest.py`

A single `pytest_plugins` declaration loads the plugin. The default
`sift_client` fixture reads `SIFT_API_KEY` / `SIFT_GRPC_URI` / `SIFT_REST_URI`
from the environment — set them in your shell, your CI secret store, or a
local `.env` (`pip install pytest-dotenv` auto-loads it).

```python title="conftest.py"
--8<-- "examples/pytest_plugin/conftest.py"
```

## `pyproject.toml`

Pytest behavior knobs sit under `[tool.pytest.ini_options]`, each commented at
its default — uncomment any line to opt out of a layer of the step tree. The
report's display `name`, `test_case`, and free-form `metadata` are set under
`[tool.sift.pytest.report]`; `name` and `test_case` accept template
placeholders.

```toml title="pyproject.toml"
--8<-- "examples/pytest_plugin/pyproject.toml"
```

## `.env.example`

```bash title=".env.example"
--8<-- "examples/pytest_plugin/.env.example"
```

## The pytest_only module

Plain pytest tests with no `sift_client` imports, no `step` fixture, no
markers. Each one still becomes a leaf step in the report tree. The plugin's
autouse fixtures capture pass/fail automatically.

```python title="tests/pytest_only/test_pytest_only_demo.py"
--8<-- "examples/pytest_plugin/tests/pytest_only/test_pytest_only_demo.py"
```

## The with_sift module

Exercises the plugin's full surface: numeric / string / bool bounds, nested
`step.substep`, `@pytest.mark.sift_exclude`, class steps with docstring
descriptions, nested classes, stacked `@pytest.mark.parametrize`, and
`step.report_outcome`.

```python title="tests/with_sift/test_with_sift_demo.py"
--8<-- "examples/pytest_plugin/tests/with_sift/test_with_sift_demo.py"
```

## Run it

### Without Sift credentials

```bash
cd python/examples/pytest_plugin
pytest --sift-disabled -v
```

`--sift-disabled` makes the plugin a no-op transport: `step.measure(...)`
still evaluates bounds and returns a real pass/fail boolean, but nothing
contacts Sift and no log file is written. Useful for previewing the report
tree or unit-testing measurement logic.

### Against a real Sift org

```bash
cp .env.example .env
# Fill in SIFT_API_KEY / SIFT_GRPC_URI / SIFT_REST_URI
pytest -v
```

A `TestReport` shows up in Sift once the session finishes.

### Offline (record now, replay later)

```bash
pytest --sift-offline --sift-log-file=/tmp/sift-demo.jsonl -v
# Later, from anywhere with credentials:
import-test-result-log /tmp/sift-demo.jsonl
```

## Expected report tree

With the plugin's defaults (every layer enabled), the demo produces:

```
TestReport (FAILED, since failures propagate up from leaves)
├── pytest_only                         ← package step (FAILED)
│   └── test_pytest_only_demo.py        ← module step (FAILED)
│       ├── test_passes                                              PASSED
│       ├── test_uses_a_pytest_fixture                               PASSED
│       ├── test_assertion_failure_marks_step_failed                 FAILED
│       ├── test_skipped                                             SKIPPED
│       ├── test_unexpected_exception_marks_step_errored             ERROR
│       ├── test_parametrize_without_step
│       │   ├── value='v1'                                           PASSED
│       │   └── value='v2'                                           PASSED
│       └── TestPytestClass
│           └── test_method                                          PASSED
└── with_sift                           ← package step (FAILED)
    └── test_with_sift_demo.py          ← module step (FAILED)
        ├── test_measurements                                        PASSED
        ├── test_substeps                                            PASSED
        │   ├── phase_1
        │   └── phase_2
        │       └── phase_2a
        │   (test_excluded: @sift_exclude, runs in pytest, NOT in tree)
        ├── test_measure_series                                      PASSED
        ├── test_failed_measurement_marks_sift_step_failed           FAILED  (pytest PASSED)
        ├── test_pytest_fail_if_step_failed_at_end                                FAILED  (pytest FAILED)
        ├── test_report_level_metadata                               PASSED
        ├── TestClassStep
        │   ├── test_parametrize
        │   │   ├── axis_a='a1'
        │   │   │   ├── axis_b='b1'                                  PASSED
        │   │   │   └── axis_b='b2'                                  PASSED
        │   │   └── axis_a='a2'
        │   │       ├── axis_b='b1'                                  PASSED
        │   │       └── axis_b='b2'                                  PASSED
        │   └── TestNested
        │       └── test_report_outcome
        │           └── check                                        PASSED
        └── TestScopedFixtureParam              ← class-scoped fixture param
            ├── stable                          ← ids= label (else firmware='1.4.2')
            │   ├── test_boots                                       PASSED
            │   └── test_reports_version                             PASSED
            └── beta
                ├── test_boots                                       PASSED
                └── test_reports_version                             PASSED
```

`TestScopedFixtureParam` shows two things. Scope-based placement: the
class-scoped `firmware` fixture's parameter lifts to wrap the class methods
(each runs once per value), unlike the function-level `@pytest.mark.parametrize`
in `TestClassStep`, whose axes nest under the test. Module- and session-scoped
fixture params lift higher still (above the module, and to the report root). And
human-readable labels: `firmware` declares `ids=["stable", "beta"]`, so the
steps use those names instead of the default `firmware='1.4.2'` form (a list or
a callable `ids=` factory both work, on parametrize axes as well as fixtures).

The `pytest_only` module deliberately includes one failing, one skipped, and
one erroring test so the demo shows every `TestStatus` mapping (`FAILED` for
assertions, `SKIPPED` for `pytest.skip`, `ERROR` for any other exception).
The `with_sift` module shows two patterns for handling measurement results:
`test_failed_measurement_marks_sift_step_failed` lets the test keep passing
in pytest while the Sift step is `FAILED` (useful when measurements are
diagnostic data you want to collect regardless of outcome); and
`test_pytest_fail_if_step_failed_at_end` takes every measurement first and
then calls `step.pytest_fail_if_step_failed()` once at the end, so every
measurement still lands in the report even when one fails. The end-of-test
call is the recommended pattern: it fails via `pytest.fail` (no assertion
noise in `error_info`), and unlike asserting on an individual
`step.measure(...)` call it does not short-circuit on the first failure and
skip every measurement that follows. Expected
pytest output is `20 passed, 3 failed, 1 skipped`.

Flip any of the `sift_*_step` / `sift_parametrize_nesting` flags in
`pyproject.toml` to `false` to collapse a layer.

## Next steps

- [Pytest Plugin guide](../guides/pytest_plugin/index.md): conceptual reference
  covering fixtures, configuration, report structure, and pass/fail behavior.
- The demo's [README](https://github.com/sift-stack/sift/blob/main/python/examples/pytest_plugin/README.md)
  on GitHub mirrors this page and is the canonical source.
