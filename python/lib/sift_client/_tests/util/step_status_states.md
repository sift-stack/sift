# Pytest-plugin step-status: observed vs. target

Companion document to `test_step_status_states.py`. Each row corresponds to
one scenario in that suite. The **observed** column is the status the Sift
pytest plugin records for the test's outer step today; the **target** column
is what the audit recommends. Rows where the two differ are the work items
for the fix.

`TestStatus` values referenced below come from
`sift_client.sift_types.test_report.TestStatus`: `PASSED`, `FAILED`, `ERROR`,
`SKIPPED`, plus the proposed `XFAILED` / `XPASSED` / `ABORTED` additions
called out in the audit.

## Call-phase exit paths

| Scenario                                | Trigger                                       | Observed today              | Target                                     | Status |
| --------------------------------------- | --------------------------------------------- | --------------------------- | ------------------------------------------ | ------ |
| Test passes                             | function body returns cleanly                 | `PASSED`                    | `PASSED`                                   | OK     |
| Assert failure in call phase            | `assert 1 == 2`                               | `FAILED`                    | `FAILED`                                   | OK     |
| Generic exception in call phase         | `raise ValueError("boom")`                    | `ERROR`                     | `ERROR`                                    | OK     |
| `pytest.fail("...")` from body          | `pytest.fail("intentional failure")`          | `ERROR`                     | `FAILED`                                   | Gap    |
| `SystemExit` from the test body         | `sys.exit(1)`                                 | `ERROR`                     | `ABORTED` (proposed) or documented `ERROR` | Gap    |
| `KeyboardInterrupt` in body             | `raise KeyboardInterrupt`                     | `PASSED` (session aborts before the plugin sees the interrupt) | `ABORTED` (proposed) | Gap |

## Skip paths

| Scenario                                | Trigger                                       | Observed today                                                              | Target                                                          | Status |
| --------------------------------------- | --------------------------------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------- | ------ |
| Collection-time skip                    | `@pytest.mark.skip(reason=...)`               | `SKIPPED` (only the makereport hook records a step; no autouse step ran)    | `SKIPPED`                                                       | OK     |
| Runtime skip in body                    | `pytest.skip("...")`                          | Outer step `ERROR`; a nested step with the same name records `SKIPPED`      | Outer step `SKIPPED`; no duplicate nested step                  | Gap    |
| Skip raised inside a fixture            | `@pytest.fixture` calls `pytest.skip("...")`  | Outer step `PASSED`; a nested `SKIPPED` step is created by the makereport hook | Outer step `SKIPPED` with `phase=setup`; no duplicate nested step | Gap |

## xfail / xpass

| Scenario                                | Trigger                                                | Observed today                                                                                  | Target                                                | Status |
| --------------------------------------- | ------------------------------------------------------ | ----------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ------ |
| xfail-marked test that fails            | `@pytest.mark.xfail` + `assert 1 == 2`                 | Outer step `FAILED`; nested `SKIPPED` substep from the makereport hook                          | Outer step `XFAILED`; no duplicate nested step        | Gap    |
| Strict xfail that unexpectedly passes   | `@pytest.mark.xfail(strict=True)` + `assert True`      | Outer step `PASSED` (plugin never sees pytest's "strict xpass" failure attached to the report)  | Outer step `XPASSED`                                  | Gap    |
| Non-strict xfail that unexpectedly passes | `@pytest.mark.xfail()` + `assert True`               | Outer step `PASSED` (pytest reports outcome="passed" with `wasxfail` set; plugin ignores it)    | Outer step `XPASSED`                                  | Gap    |
| `xfail(raises=...)` with wrong exception | `@pytest.mark.xfail(raises=ValueError)` + `raise KeyError` | Outer step `ERROR` (treated as a generic non-assertion exception)                           | `FAILED` (the `raises=` mismatch is a real test failure) | Gap |
| `xfail(run=False)`                      | `@pytest.mark.xfail(run=False)` (body never executed)  | `SKIPPED` (only the makereport hook records a step)                                              | `XFAILED`                                            | Gap    |

## Setup / teardown phases

| Scenario                                | Trigger                                                              | Observed today                                                                                                                            | Target                                                  | Status |
| --------------------------------------- | -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------ |
| Setup-phase fixture failure (RuntimeError) | `@pytest.fixture` raises before `yield`; test body never runs    | Outer step does not exist or lands `PASSED`; the plugin does not consult `report.when`                                                    | `ERROR` with `phase=setup` annotation                   | Gap    |
| Teardown-phase fixture failure          | `@pytest.fixture` raises after `yield`; test body passed             | Outer step `PASSED` — it closes before the failing teardown runs, so the error is invisible                                              | `FAILED` with `phase=teardown` annotation               | Gap    |
| Call-phase fail **plus** teardown-phase fail | `assert 1 == 2` in body AND `@pytest.fixture` raises after `yield` | Outer step `FAILED` (the call-phase failure dominates); the teardown error is silently lost                                              | `FAILED` with a `phase=teardown` annotation so the teardown error is also visible | Gap |

## Collection / fixture-resolution failures

| Scenario                                | Trigger                                       | Observed today                                                                                                                                  | Target                                                  | Status |
| --------------------------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------ |
| Missing fixture                         | `def test_x(nonexistent_fixture):`            | Outer step `PASSED` — the autouse `step` fixture's setup still runs before pytest detects the missing fixture; the user sees a green step for a test that never executed | `ERROR` with `phase=setup`                              | Gap    |

## Plugin-API exit paths (in-test mutations)

| Scenario                                | Trigger                                                                | Observed today | Target   | Status |
| --------------------------------------- | ---------------------------------------------------------------------- | -------------- | -------- | ------ |
| Manual status override                  | `step.current_step.update({"status": TestStatus.FAILED})`              | `FAILED`       | `FAILED` | OK     |
| `report_outcome(result=False)`          | `step.report_outcome("the_check", False, "did not match")`             | `FAILED`       | `FAILED` | OK     |
| `measure(...)` out-of-bounds            | `step.measure(name="m", value=10.0, bounds={"min": 0.0, "max": 5.0})`  | `FAILED`       | `FAILED` | OK     |

## Out of scope for this characterization run

- **Timeout** — needs `pytest-timeout` or a manual signal harness. Add as a
  follow-up once the audit picks a timeout strategy.
- **Signal (SIGKILL / SIGTERM)** — cannot be caught from inside the process;
  needs a subprocess-level harness.
- **`pytest.exit("...")`** — niche; the "aborts subsequent tests" behavior
  is hard to characterize cleanly because each `pytester` invocation is its
  own session. Document the expectation alongside `SystemExit`.
- **`os._exit()`** — bypasses Python cleanup entirely; can't be tested
  in-process because it would kill the outer pytest run. Document as a
  guaranteed data-loss case alongside `SystemExit` / `SIGKILL`.
- **Parametrize-level marks** (`pytest.param(..., marks=pytest.mark.xfail / skip)`)
  — routes through a different selection path but produces the same
  `report.outcome`, so behavior should match the function-level marks
  already covered above. Add only if the plugin's eventual phase-aware
  handler diverges between the two.
- **Import error / syntax error / `conftest.py` error** — these fail
  collection entirely; no `item` is produced and no plugin hook fires.
  Document explicitly that no Sift step is recorded.
- **No-data / indeterminate** — tracked separately as part of the sibling
  status-semantics work.

## How to refresh this table

Run the suite locally:

```
pytest lib/sift_client/_tests/util/test_step_status_states.py -v
```

Every "Gap" row corresponds to a `# AUDIT:` comment in the test file naming
the target status. When the plugin fix lands, the regression edit is
mechanical: flip the assertion in each gap row to its target, then update
the **Observed today** column here to match.
