# Pass/Fail Behavior

How pytest outcomes, measurement results, and manual status updates map to
step and report statuses, and how failures propagate up the tree.

See [Usage](usage.md) for the broader plugin reference.

## Step status from test outcome

When a test function returns (or raises), the plugin's step exit handler in
`NewStep.__exit__` resolves the step's `TestStatus`:

| Outcome inside the test | Resulting `TestStatus` | Notes |
|---|---|---|
| All measurements in-bounds, no raised exception | `PASSED` | |
| A `measure*` call returned `False`, a `report_outcome` returned `False`, a substep failed, the test raised `AssertionError`, or the test called `pytest.fail(...)` | `FAILED` | No traceback is attached — pytest already prints one in the runner output. |
| Test raised any other exception (e.g. `ValueError`, `TimeoutError`) | `ERROR` | The formatted traceback (last 10 frames plus the first frame) is attached to `step.error_info.error_message`. |
| Test was skipped via `@pytest.mark.skip` / `skipif` / `pytest.skip(...)` — whether at collection time, from the test body, or from a fixture | `SKIPPED` | |
| Code in the test called `step.current_step.update({"status": ...})` | Whatever you set | Manual status is honored over the inferred one. |

See [xfail / xpass](#xfail--xpass), [Setup and teardown phases](#setup-and-teardown-phases),
and [Hard exits](#hard-exits) below for cases the table above does not cover.

## Measurement results never raise

`step.measure(...)`, `step.measure_avg(...)`, `step.measure_all(...)`, and
`step.report_outcome(...)` all return a boolean and mark the enclosing step
`FAILED` on `False`. None of them raise. Chain them freely:

```python
def test_power_on(step):
    voltage_ok = step.measure(name="rail_v", value=4.97, bounds={"min": 4.8, "max": 5.2})
    config_ok = step.report_outcome(name="config_loaded", result=True)
    # The step is already FAILED if either was False. Assert here only if you
    # also want pytest to fail the test.
    assert voltage_ok and config_ok
```

This decouples "the step failed in Sift" from "the test failed in pytest."
Skip the `assert` and the test passes in pytest while the step is still
recorded as failed in Sift — useful for keeping a suite running through
known soft failures.

## Failure propagation

A failed step propagates upward through every parent that wraps it:

```text
substep (level_3)  ──FAILED──┐
substep (level_2)         ◄──┘──FAILED──┐
substep (level_1)                    ◄──┘──FAILED──┐
function step                                   ◄──┘──FAILED──┐
module step (if module_substep is active)                  ◄──┘──FAILED──┐
TestReport                                                            ◄──┘
```

When a leaf is marked `FAILED`, every parent up to the `TestReport` inherits
`FAILED` at session exit. `ERROR` propagates the same way; the report
finalizes to `FAILED` (not `ERROR`) if any step errored or any exception
escaped the session.

`SKIPPED` does **not** propagate as a failure. Marking a substep `SKIPPED`
leaves its parent's resolved status unchanged.

## xfail / xpass

The plugin maps pytest's `xfail` outcomes onto existing statuses:

| Scenario | `TestStatus` |
|---|---|
| `xfail` test fails as expected | `PASSED` |
| `xfail(strict=False)` test passes (xpass) | `PASSED` |
| `xfail(strict=True)` test passes (xpass) | `FAILED` |
| `xfail(raises=X)` raises a different exception | `FAILED` |
| `xfail(run=False)` (test is never executed) | `SKIPPED` |

The strict-xpass and `raises=` mismatch cases route to `FAILED` because both
signal that the `xfail` mark no longer matches reality: either the bug was
fixed (and the mark should be removed) or the failure mode shifted (and the
mark needs updating). Conflating them with `PASSED` would hide the signal.

## Setup and teardown phases

A pytest test runs in three phases: `setup` (fixtures initialize up to
`yield`), `call` (the test body), and `teardown` (fixtures run cleanup after
`yield`). The plugin records the failing phase's outcome on the outer step:

| Phase that failed | Step status |
|---|---|
| Setup — fixture raised before `yield`, or a required fixture was missing | `ERROR` |
| Call — test body raised or a measurement failed | `FAILED` or `ERROR` per the table at the top of this page |
| Teardown — fixture raised after `yield` | `FAILED` |

When the call phase and the teardown phase both fail, the call-phase outcome
determines the step's status and the teardown error is surfaced alongside it
rather than being silently discarded.

## Hard exits

| Trigger in the test body | `TestStatus` |
|---|---|
| `pytest.fail("...")` | `FAILED` |
| `pytest.skip("...")` | `SKIPPED` |
| `SystemExit` | `ERROR` |
| `KeyboardInterrupt` | `ERROR` when the plugin sees the interrupt; a session-aborting `Ctrl-C` may bypass the plugin and leave the step in `IN_PROGRESS`. |

A dedicated `ABORTED` status for `SystemExit` and `KeyboardInterrupt` is
planned; today both map to `ERROR`.

## Manually skipping a substep

Inside a test, mark a single substep skipped without aborting the test or
failing its parent:

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

The exit handler honors the manually-resolved `SKIPPED`. The enclosing
function step still resolves on its own measurements and remaining substeps.

## Report status

The session-scoped `TestReport`:

- Starts at `IN_PROGRESS` on session enter.
- Finalizes on session exit to:
    - `FAILED` if any step finalized as `FAILED` or `ERROR`, or any exception escaped the session.
    - `PASSED` otherwise.

A run with only `PASSED` and `SKIPPED` steps resolves to `PASSED`.
