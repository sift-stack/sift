# Pass/Fail Behavior

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
| `ABORTED`     | A hard exit (`SystemExit`, observed `KeyboardInterrupt`) interrupted the test.                                         |
| `SKIPPED`     | The test was skipped at collection time, at runtime, or from a fixture.                                                |
| `IN_PROGRESS` | Test in progress or the plugin never observed a final outcome (e.g. a session-aborting interrupt killed pytest first). |

## Normal test outcomes

| Scenario                                  | Trigger                              | Outcome  |
| ----------------------------------------- | ------------------------------------ | -------- |
| Test passes                               | function body returns cleanly        | `PASSED` |
| Assertion failure                         | `assert 1 == 2`                      | `FAILED` |
| `pytest.fail("...")` from the body        | `pytest.fail("intentional failure")` | `FAILED` |
| Uncaught non-assertion exception          | `raise ValueError("boom")`           | `ERROR`  |

A non-assertion exception gets its formatted traceback recorded on
`step.error_info.error_message`.

## Hard exits

Hard exits the plugin can observe map to `ABORTED`. If pytest tears the
session down before the plugin sees the exit, the step stays at
`IN_PROGRESS` instead of resolving.

| Scenario                                       | Trigger                   | Outcome                                                              |
| ---------------------------------------------- | ------------------------- | -------------------------------------------------------------------- |
| `SystemExit` from the test body                | `sys.exit(1)`             | `ABORTED`                                                            |
| `KeyboardInterrupt` the plugin observes        | `raise KeyboardInterrupt` | `ABORTED`                                                            |
| Session-aborting `KeyboardInterrupt`           | Ctrl-C terminates pytest  | `IN_PROGRESS` (session ends before the plugin's hooks fire)          |

### Abort propagation through nested substeps

Every step that was open when the abort fired records
`ABORTED`.

```python title="test_abort.py"
import sys


def test_x(step):
    with step.substep(name="completed_sub"):
        pass  # closes as PASSED before the abort
    with step.substep(name="outer_sub") as outer_sub:
        with outer_sub.substep(name="inner_sub"):
            sys.exit(1)  # ABORTED applied to inner_sub, outer_sub, and the test step
```

The Sift report shows `completed_sub` as `PASSED` and the three steps
still open at the abort (`inner_sub`, `outer_sub`, and the test step
itself) as `ABORTED`.

## Skips

| Scenario                              | Trigger                                       | Outcome   |
| ------------------------------------- | --------------------------------------------- | --------- |
| Collection-time skip                  | `@pytest.mark.skip(reason=...)`               | `SKIPPED` |
| Conditional collection-time skip      | `@pytest.mark.skipif(True, reason=...)`       | `SKIPPED` |
| Runtime skip from the test body       | `pytest.skip("...")`                          | `SKIPPED` |
| Skip raised inside a fixture          | `@pytest.fixture` calls `pytest.skip("...")`  | `SKIPPED` |

`SKIPPED` does not propagate as a failure. A skipped substep or test does
not block its parent from resolving to `PASSED`.

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

- A hard exit (`SystemExit` or an observed `KeyboardInterrupt`) in the
  step's own scope records `ABORTED`. `ABORTED` propagates through every
  step the abort passes through on its way up.
- A child that already recorded a non-`PASSED`/`SKIPPED` outcome marks
  the parent as `FAILED`. This holds whether or not an exception is still
  propagating through the parent's scope: only the originating substep
  records `ERROR`; ancestors inherit `FAILED`. The traceback stays on
  the originating step's `error_info`.
- A step records `ERROR` only when its own scope raised a non-Assertion
  exception AND no child has failed.

`SKIPPED` does not propagate. A status set explicitly via
`current_step.update` is kept.
