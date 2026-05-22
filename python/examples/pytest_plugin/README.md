# Pytest plugin demo

A self-contained pytest project that exercises every feature of
`sift_client.pytest_plugin`: package / module / class / parametrize step
nesting, nested classes, manual substeps, `step.measure(...)` against
numeric / string / bool bounds, gate markers, and the ini opt-outs.

```
examples/pytest_plugin/
├── conftest.py                            # registers the plugin
├── pytest.ini                             # available ini knobs (all commented at defaults)
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
`sift_parametrize_nesting` to `false` in `pytest.ini` to disable this behavior.

## Run it

**Against a real Sift org**:

```bash
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

With the plugin's defaults (everything in `pytest.ini` left commented), running
this demo produces a tree like:

```
TestReport
├── pytest_only                         ← package step
│   └── test_pytest_only_demo.py        ← module step
│       ├── test_passes
│       ├── test_uses_a_pytest_fixture
│       ├── test_parametrize_without_step
│       │   ├── value='v1'
│       │   └── value='v2'
│       └── TestPytestClass
│           └── test_method
└── with_sift                           ← package step
    └── test_with_sift_demo.py          ← module step
        ├── test_measurements
        ├── test_substeps
        │   ├── phase_1
        │   └── phase_2
        │       └── phase_2a
        │   (test_excluded: @sift_exclude, runs in pytest, NOT in tree)
        └── TestClassStep
            ├── test_parametrize
            │   ├── axis_a='a1'
            │   │   ├── axis_b='b1'
            │   │   └── axis_b='b2'
            │   └── axis_a='a2'
            │       ├── axis_b='b1'
            │       └── axis_b='b2'
            └── TestNested
                └── test_report_outcome
                    └── check
```

Toggle any of the `sift_*_step` / `sift_parametrize_nesting` flags in
`pytest.ini` to `false` to collapse a layer.

## What each file demonstrates

| File | Feature |
|---|---|
| `conftest.py` | Plugin registration via `pytest_plugins`; optional `load_dotenv()` |
| `pytest.ini` | The four nesting flags + git metadata flag at their defaults |
| `tests/pytest_only/test_pytest_only_demo.py` | Plain pytest tests with no Sift APIs. The plugin captures pass/fail automatically; covers functions, fixtures, parametrize, and classes |
| `tests/with_sift/test_with_sift_demo.py` | `step.measure` with numeric/string/bool bounds, nested `step.substep`, `@pytest.mark.sift_exclude`, class step + class docstring → description, nested classes, stacked `@pytest.mark.parametrize`, `step.report_outcome` |
| `tests/{pytest_only,with_sift}/__init__.py` | Each Python package (directory with `__init__.py`) becomes a parent step in the report tree |
