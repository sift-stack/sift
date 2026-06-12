# Pytest plugin demo

A self-contained pytest project that exercises every feature of
`sift_client.pytest_plugin`: package / module / class / parametrize step
nesting, nested classes, scope-based placement of parametrized-fixture params,
manual substeps, `step.measure(...)` against numeric / string / bool bounds,
gate markers, and the ini opt-outs.

```
examples/pytest_plugin/
├── conftest.py                            # registers the plugin
├── pyproject.toml                         # pytest knobs + report name/test_case/metadata
├── .env.example                           # credential template (copy to .env for local runs)
└── tests/
    ├── pytest_only/                       # subpackage step: `pytest_only` opens a parent step
    │   ├── __init__.py
    │   └── test_pytest_only_demo.py       # plain pytest tests with no Sift APIs
    └── with_sift/                         # subpackage step: `with_sift` opens a parent step
        ├── __init__.py
        └── test_with_sift_demo.py         # measurements, substeps, classes, nested classes,
                                            # stacked parametrize, sift_exclude marker
```

Every layer of organization shows up in the report tree: Python packages
(directories with `__init__.py`), modules (test files), classes (including
nested classes), and parametrize axes each open a parent step. Flip
`sift_package_step`, `sift_module_step`, `sift_class_step`, or
`sift_parametrize_nesting` to `false` in `pyproject.toml` to disable this behavior.

## Run it

**Against a real Sift org**:

```bash
pip install pytest-dotenv        # auto-loads .env; or export the vars yourself
cp .env.example .env
# Fill in SIFT_API_KEY / SIFT_GRPC_URI / SIFT_REST_URI
pytest -v
```

A `TestReport` shows up in Sift once the session finishes.

**Offline (record now, replay later - intended for offline environments)**:

```bash
pytest --sift-offline --sift-log-file=/tmp/sift-demo.jsonl -v
# Later, from anywhere with credentials:
import-test-result-log /tmp/sift-demo.jsonl
```

## What the report tree looks like

With the plugin's defaults (the `[tool.pytest.ini_options]` knobs left
commented), running this demo produces a tree like:

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

`TestScopedFixtureParam` shows two things. First, **scope-based placement**:
`firmware` is class-scoped, so its parameter lifts to wrap the class methods
(each runs once per value) instead of nesting under an individual test the way
the function-level `@pytest.mark.parametrize` in `TestClassStep` does. Module-
and session-scoped fixture params lift higher still (above the module, and to
the report root, respectively). Second, **human-readable labels**: the fixture
declares `ids=["stable", "beta"]`, so the steps use those names instead of the
default `firmware='1.4.2'` form. A list or a callable `ids=` factory both work,
on `@pytest.mark.parametrize` axes as well as fixtures.

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

Toggle any of the `sift_*_step` / `sift_parametrize_nesting` flags in
`pyproject.toml` to `false` to collapse a layer.

## What each file demonstrates

| File | Feature |
|---|---|
| `conftest.py` | Plugin registration via `pytest_plugins` (a single line) |
| `pyproject.toml` | Pytest nesting/git-metadata knobs at their defaults; report `name`, `test_case`, and `metadata` under `[tool.sift.pytest.report]` |
| `tests/pytest_only/test_pytest_only_demo.py` | Plain pytest tests with no Sift APIs. The plugin captures pass/fail automatically; covers functions, fixtures, parametrize, classes, plus one each of `AssertionError` (FAILED), `pytest.skip` (SKIPPED), and a raised `ValueError` (ERROR) |
| `tests/with_sift/test_with_sift_demo.py` | `step.measure` (numeric/string/bool bounds, units, description, metadata, `channel_names`), `step.measure_avg` and `step.measure_all` for series, an out-of-bounds measurement (pytest PASSED, Sift step FAILED), the recommended `step.pytest_fail_if_step_failed()` end-of-test call that fails pytest while still recording every measurement, nested `step.substep` (with step-level `metadata=...`), `@pytest.mark.sift_exclude`, class step + class docstring → description, nested classes, stacked `@pytest.mark.parametrize`, a class-scoped parametrized fixture lifted above its methods by scope and given human-readable step labels via `ids=`, `step.report_outcome`, and session-level metadata via `report_context.report.update({...})` |
| `tests/{pytest_only,with_sift}/__init__.py` | Each Python package (directory with `__init__.py`) becomes a parent step in the report tree |
