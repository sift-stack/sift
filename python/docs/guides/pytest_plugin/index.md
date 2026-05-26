# Pytest Plugin

The Sift Python client ships a pytest plugin that turns a pytest run into a
`TestReport` in Sift. Each test function becomes a `TestStep`, measurements are presented
as rows under that step, and failures propagate up through nested substeps to
the report itself.

## Quick start

Install the client and pytest:

```bash
pip install sift-stack-py pytest python-dotenv
```

Set your connection details in a `.env` next to your tests:

```bash title=".env"
SIFT_API_KEY="..."
SIFT_GRPC_URI="..."
SIFT_REST_URI="..."
```

Find these on the Sift Manage page, where you can also generate an API key.

Register the plugin with a single `pytest_plugins` declaration in your top-level
`conftest.py`:

```python title="conftest.py"
from dotenv import load_dotenv

load_dotenv()

pytest_plugins = ["sift_client.pytest_plugin"]
```

Write a test. The `step` fixture is `autouse`, so any test becomes a step on the
report. Take it as an argument when you want to record a measurement:

```python title="test_battery.py"
def test_battery_voltage(step):
    step.measure(
        name="battery_voltage",
        value=4.97,
        bounds={"min": 4.8, "max": 5.2},
        unit="V",
    )
    step.fail_if_measurements_failed()
```

Run it:

```bash
pytest
```

A `TestReport` shows up in Sift once the session finishes.

!!! tip "Fail at the end, not per measurement"
    `step.measure(...)` returns a pass/fail boolean and never raises, so a
    failing measurement marks the step failed without aborting the test. Take
    every measurement first, then call `step.fail_if_measurements_failed()` once
    at the end, so every measurement still lands in the report even when one
    fails. It fails the test via `pytest.fail` (no assertion noise in
    `error_info`), and unlike asserting on an individual `step.measure(...)` call
    it does not short-circuit on the first failure and skip every measurement
    after it.

## Sensible defaults

With nothing but the `conftest.py` above, you get:

- **Full step tree.** Every Python package, test module, test class, and
  parametrize axis above a test becomes a parent step, so the report mirrors
  your test layout.
- **Online mode.** The plugin pings Sift at session start and streams
  create/update calls to your tenant during the run.
- **Git metadata.** Repo, branch, and commit are captured on the report
  automatically.

Everything is on by default and individually overridable. See
[Configuration & Defaults](configuration.md) for the full audit of every knob,
marker, flag, and fixture.

## Running modes

The plugin runs in one of three modes, picked at invocation.

| Mode | How to select | Contacts Sift | When to use                                                   |
|---|---|---|---------------------------------------------------------------|
| **Online** | default (no flag) | Yes, during the run | Default choice                                                |
| **Offline** | `--sift-offline` | No; records to a log file for later replay | Environments without Sift access.                             |
| **Disabled** | `--sift-disabled` | No | Local dev. Bounds still evaluate and return a real pass/fail. |

Online mode pings Sift once at session start and aborts if Sift is unreachable or the credentials are invalid, 
so a misconfigured job fails immediately instead of silently producing no report. 
During the run, every create and update is appended to a JSONL log file. 
A background worker uploads new entries to Sift incrementally. 
If the connection drops mid-test, the test keeps running and the log keeps writing locally. 
The remaining entries can be uploaded afterward by running import-test-result-log, which the plugin prints on exit.

See [Running Modes](running_modes.md) for the log-file and replay pipeline,
overriding the connection check, and replaying a saved log.

## Report structure

The report tree mirrors your test layout: packages, modules, classes, and
parametrize axes nest automatically, and you can open arbitrary substeps inside
a test. See [Report Structure](report_structure.md) for the layout-to-tree
mapping, measurement variants, and report metadata.

## Pass/fail outcomes

Every pytest outcome (pass, assertion failure, exception, skip, xfail, hard
exit) maps to a `TestStatus`, and failures roll up to the parent steps and the
report. See [Pass/Fail Behavior](pass_fail_behavior.md).

## Try the runnable demo

The [Pytest Plugin Quickstart](../../examples/pytest_plugin_quickstart.md) walks
through a self-contained demo project that exercises every layer of the step
tree, with instructions to run it with or without a Sift tenant.
