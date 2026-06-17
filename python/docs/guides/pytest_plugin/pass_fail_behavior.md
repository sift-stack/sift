# Pass/fail behavior

The pytest plugin maps every pytest outcome to a `TestStatus` on the
corresponding Sift step. Use this page to look up what a given test will
produce, and how that result rolls up to the parent steps and the report.

## `TestStatus` values

The statuses below come from `sift_client.sift_types.test_report.TestStatus`.

| Status        | Meaning                                                                                                                |
| ------------- |------------------------------------------------------------------------------------------------------------------------|
| `PASSED`      | The step completed and every check it owns succeeded.                                                                  |
| `FAILED`      | An assertion, a `pytest.fail(...)`, a failed `report_outcome`, or a failing measurement marked it.                     |
| `ERROR`       | An unexpected exception escaped the test body or a fixture (setup or teardown).                                        |
| `ABORTED`     | A hard exit (`SystemExit` or `KeyboardInterrupt`) cut the test off; resolved while pytest tears the session down.      |
| `SKIPPED`     | The test was skipped at collection time, at runtime, or from a fixture.                                                |
| `IN_PROGRESS` | A transient creation state. It survives into the report only if the process is killed so abruptly that teardown never runs. |

## Normal test outcomes

| Scenario                                  | Trigger                              | Outcome  |
| ----------------------------------------- | ------------------------------------ | -------- |
| Test passes                               | function body returns cleanly        | `PASSED` |
| Assertion failure                         | `assert 1 == 2`                      | `FAILED` |
| `pytest.fail("...")` from the body        | `pytest.fail("intentional failure")` | `FAILED` |
| Uncaught non-assertion exception          | `raise ValueError("boom")`           | `ERROR`  |

An assertion failure records the concise assertion message (the exception
line(s), no traceback frames) on `step.error_info.error_message` while still
mapping to `FAILED`. A non-assertion exception gets its formatted traceback
(the last 10 frames plus the first frame) recorded on
`step.error_info.error_message`.

## Hard exits

A hard exit resolves the cut-off step to `ABORTED`, recorded while pytest runs
fixture finalizers on the way out. What the containers (class, module, package)
and the report resolve to depends on why the run stopped:

- A **failure stop** rolls up `FAILED`. `pytest.exit()`, `sys.exit()` /
  `SystemExit`, an `assert`, or an exception means the test ended the run on a
  fault, so the exited step is `ABORTED` (or `FAILED`/`ERROR` for an
  assert/exception) while its containers and the report read `FAILED`.
- A **system stop** rolls up `ABORTED`. Ctrl-C / `KeyboardInterrupt`, or the
  `abort()` helper below, means the run was cut off rather than a test failing,
  so the exited step, its containers, and the report all read `ABORTED`.

`SystemExit` is read from the call-phase report; the session-stopping exits abort
before that report fires, so the step resolves during teardown instead. If the
process dies before finalizers run (`SIGKILL`, OOM, power loss) nothing more is
written and the step stays `IN_PROGRESS`, the only path that leaves a step
`IN_PROGRESS` in a finished report.

| Trigger                          | Exited step        | Containers + report |
| -------------------------------- | ------------------ | ------------------- |
| `assert` / exception             | `FAILED` / `ERROR` | `FAILED`            |
| `sys.exit()` / `SystemExit`      | `ABORTED`          | `FAILED`            |
| `pytest.exit("...")`             | `ABORTED`          | `FAILED`            |
| `abort("...")` (Sift)            | `ABORTED`          | `ABORTED`           |
| Ctrl-C / `KeyboardInterrupt`     | `ABORTED`          | `ABORTED`           |
| process killed (`SIGKILL`/OOM)   | `IN_PROGRESS`      | `IN_PROGRESS`       |

Within a stop, `ABORTED` is recorded on each step the exit unwinds through: the
open substeps and the test step. A substep that closed before the exit keeps its
own status.

```python title="test_abort.py"
import sys


def test_x(step):
    with step.substep(name="completed_sub"):
        pass  # closed PASSED before the abort
    with step.substep(name="outer_sub") as outer_sub:
        with outer_sub.substep(name="inner_sub"):
            sys.exit(1)
```

`completed_sub` stays `PASSED`; `inner_sub`, `outer_sub`, and the test step are
`ABORTED`. The enclosing module reads `FAILED`, since `sys.exit()` is a failure
stop.

### Stopping a run as aborted

`sift_client.pytest_plugin.abort(reason)` stops the session and records the
report and the open parent steps as `ABORTED` rather than `FAILED`. Use it for a
system-level stop where the run was cut off rather than a test failing, such as
the device under test losing power. A real Ctrl-C does the same automatically.

```python
from sift_client.pytest_plugin import abort


def test_flash(step):
    if not device_responding():
        abort("device under test is not responding")
    ...
```

For a stop that should read as a failure, use `pytest.exit()`; for a single
failing test, use `pytest.fail()`.

## Skips

| Scenario                              | Trigger                                       | Outcome   |
| ------------------------------------- | --------------------------------------------- | --------- |
| Collection-time skip                  | `@pytest.mark.skip(reason=...)`               | `SKIPPED` |
| Conditional collection-time skip      | `@pytest.mark.skipif(True, reason=...)`       | `SKIPPED` |
| Runtime skip from the test body       | `pytest.skip("...")`                          | `SKIPPED` |
| Skip raised inside a fixture          | `@pytest.fixture` calls `pytest.skip("...")`  | `SKIPPED` |

`SKIPPED` does not propagate as a failure. A skipped substep or test does
not block its parent from resolving to `PASSED`.

Inside a test function, you can mark just one substep as skipped without
aborting the whole test:

```python
from sift_client.sift_types.test_report import TestStatus


def test_runtime_skip(step):
    with step.substep(name="optional_calibration") as cal:
        if not precondition_met():
            cal.current_step.update({"status": TestStatus.SKIPPED})
```

The step's exit handler honors a manually-resolved status, so you do not need
any further bookkeeping.

## Expected failures (xfail / xpass)

xfail marks declare that a test is expected to fail. The plugin follows
the same semantics pytest does.

| Scenario                                  | Trigger                                                    | Outcome                                                       |
| ----------------------------------------- | ---------------------------------------------------------- | ------------------------------------------------------------- |
| xfail-marked test that fails              | `@pytest.mark.xfail` + `assert 1 == 2`                     | `PASSED` (the test fulfilled the xfail expectation)           |
| Strict xfail that unexpectedly passes     | `@pytest.mark.xfail(strict=True)` + `assert True`          | `FAILED` (the mark no longer matches reality)                 |
| Non-strict xfail that unexpectedly passes | `@pytest.mark.xfail()` + `assert True`                     | `PASSED` (`strict=False` does not insist on the failure)      |
| `xfail(raises=...)` with wrong exception  | `@pytest.mark.xfail(raises=ValueError)` + `raise KeyError` | `FAILED` (the `raises=` mismatch is a real test failure)      |
| `xfail(run=False)`                        | `@pytest.mark.xfail(run=False)`                            | `SKIPPED` (the body never ran)                                |

## Influencing outcomes from test code

A test can also set the step's outcome directly via the helpers below.
Substeps your test opens follow the same propagation rules as the ones
the plugin opens for you.

### Manual status override

`step.current_step.update({...})` sets the status directly. The step's
exit handler does not overwrite it.

```python
from sift_client.sift_types.test_report import TestStatus


def test_manual(step):
    step.current_step.update({"status": TestStatus.FAILED})
```

### `report_outcome` for externally computed checks

`report_outcome(name, result, reason)` records a named check whose
pass/fail was computed elsewhere (a subprocess, a remote system, your own
comparison logic). A failing outcome marks the step `FAILED`.

```python
def test_external_check(step):
    result, reason = run_external_validator()
    step.report_outcome("ext-validator", result, reason)
```

### Measurements with bounds

`step.measure(name=, value=, bounds=)` records a measurement and resolves
the step to `FAILED` if the value is out of bounds. The call returns the
pass/fail boolean and does not raise, so multiple measurements can run
without short-circuiting.

```python
def test_battery(step):
    step.measure(name="voltage", value=12.1, bounds={"min": 11.5, "max": 13.0}, unit="V")
    step.measure(name="current", value=0.42, bounds={"max": 1.0}, unit="A")
```

### Substep failures

A failed substep propagates failure to its parent step. A manually-set
`SKIPPED` on a substep does not.

```python
def test_with_substep(step):
    with step.substep(name="check") as inner:
        inner.measure(name="value", value=99.0, bounds={"min": 0.0, "max": 5.0})
    # The outer step resolves to FAILED because the substep failed.
```

## Propagation rules

Every non-`PASSED`/`SKIPPED` step marks its parent as failed. What the
parent records depends on whether its own scope had an abort and whether
a child already failed:

- A hard exit (`SystemExit` or an observed `KeyboardInterrupt`) records
  `ABORTED` on the step in whose scope it fired, and `ABORTED` propagates
  through every step the exception unwinds through on its way up: the
  open substeps and the test step. Container parents (class, module,
  package) are closed out-of-band rather than by the unwinding exception,
  so they are not on that path. On a failure stop (`pytest.exit()`,
  `sys.exit`) they inherit `FAILED` like any other non-pass child; on a
  system stop (Ctrl-C / `KeyboardInterrupt`, or `abort()`) the run is flagged
  aborted and they resolve `ABORTED` instead. See [Hard exits](#hard-exits).
- A child that recorded a non-`PASSED`/`SKIPPED` outcome marks the parent
  as `FAILED`. This holds whether or not an exception is still propagating
  through the parent's scope: only the originating step records `ERROR` (or
  `ABORTED`); ancestors that inherit the result take `FAILED`. The
  traceback stays on the originating step's `error_info`.
- A step records `ERROR` only when its own scope raised a non-Assertion
  exception AND no child has failed.

`SKIPPED` does not propagate. A status set explicitly via
`current_step.update` is kept.
