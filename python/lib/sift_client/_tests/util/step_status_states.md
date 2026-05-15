# Pytest-plugin step-status: observed vs. target

Companion document to `test_step_status_states.py`. Each row corresponds to
one scenario in that suite. The **target** column is the contract the suite
asserts (sourced from
[`docs/guides/pytest_plugin/pass_fail_behavior.md`](../../../../docs/guides/pytest_plugin/pass_fail_behavior.md));
the **observed today** column records what the plugin actually produces
right now. Rows marked `Gap` are scenarios where the test fails today and
the plugin needs to be fixed to match the contract.

`TestStatus` values referenced below come from
`sift_client.sift_types.test_report.TestStatus`: `PASSED`, `FAILED`, `ERROR`,
`SKIPPED`. The targets below map every scenario onto these four existing
statuses. An `ABORTED` status for hard process exits (`SystemExit`,
`KeyboardInterrupt`, signals) is a planned future addition; until it lands
those cases baseline against `ERROR`. The user-facing contract these
targets describe is documented in
[`docs/guides/pytest_plugin/pass_fail_behavior.md`](../../../../docs/guides/pytest_plugin/pass_fail_behavior.md).

## Case ID scheme

Each scenario has a stable case ID of the form `PREFIX-NN`, where the
prefix names its section. Tests in `test_step_status_states.py` reference
their case ID in a leading comment so a failing test can be traced back to
this table without rereading the scenario:

| Prefix  | Section                                  |
| ------- | ---------------------------------------- |
| `CALL`  | Call-phase exit paths                    |
| `SKIP`  | Skip paths                               |
| `XFAIL` | xfail / xpass                            |
| `PHASE` | Setup / teardown phases                  |
| `COLL`  | Collection / fixture-resolution failures |
| `API`   | Plugin-API exit paths                    |

IDs are stable: a new scenario in a section takes the next free number for
that prefix; numbers are never reused or shifted when other sections grow.

## Call-phase exit paths

| Case      | Scenario                                | Trigger                                       | Observed today              | Target                                     | Status |
| --------- | --------------------------------------- | --------------------------------------------- | --------------------------- | ------------------------------------------ | ------ |
| `CALL-01` | Test passes                             | function body returns cleanly                 | `PASSED`                    | `PASSED`                                   | OK     |
| `CALL-02` | Assert failure in call phase            | `assert 1 == 2`                               | `FAILED`                    | `FAILED`                                   | OK     |
| `CALL-03` | Generic exception in call phase         | `raise ValueError("boom")`                    | `ERROR`                     | `ERROR`                                    | OK     |
| `CALL-04` | `pytest.fail("...")` from body          | `pytest.fail("intentional failure")`          | `ERROR`                     | `FAILED`                                   | Gap    |
| `CALL-05` | `SystemExit` from the test body         | `sys.exit(1)`                                 | `ERROR`                     | `ERROR` (baseline; `ABORTED` planned later) | OK    |
| `CALL-06` | `KeyboardInterrupt` in body             | `raise KeyboardInterrupt`                     | `PASSED` (session aborts before the plugin sees the interrupt) | `ERROR` when the plugin sees the interrupt; document that a session-aborting interrupt may leave the step in `IN_PROGRESS` | Gap |

## Skip paths

| Case      | Scenario                                | Trigger                                       | Observed today                                                              | Target                                                          | Status |
| --------- | --------------------------------------- | --------------------------------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------- | ------ |
| `SKIP-01` | Collection-time skip                    | `@pytest.mark.skip(reason=...)`               | `SKIPPED` (only the makereport hook records a step; no autouse step ran)    | `SKIPPED`                                                       | OK     |
| `SKIP-02` | Conditional collection-time skip        | `@pytest.mark.skipif(True, reason=...)`       | `SKIPPED` (same route as `@pytest.mark.skip`)                               | `SKIPPED`                                                       | OK     |
| `SKIP-03` | Runtime skip in body                    | `pytest.skip("...")`                          | Outer step `ERROR`; a nested step with the same name records `SKIPPED`      | Outer step `SKIPPED`; no duplicate nested step                  | Gap    |
| `SKIP-04` | Skip raised inside a fixture            | `@pytest.fixture` calls `pytest.skip("...")`  | Outer step `PASSED`; a nested `SKIPPED` step is created by the makereport hook | Outer step `SKIPPED` (setup-phase skip); no duplicate nested step | Gap |

## xfail / xpass

| Case       | Scenario                                  | Trigger                                                | Observed today                                                                                  | Target                                                | Status |
| ---------- | ----------------------------------------- | ------------------------------------------------------ | ----------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ------ |
| `XFAIL-01` | xfail-marked test that fails              | `@pytest.mark.xfail` + `assert 1 == 2`                 | Outer step `FAILED`; nested `SKIPPED` substep from the makereport hook                          | Outer step `PASSED` (test fulfilled the xfail expectation); no duplicate nested step | Gap    |
| `XFAIL-02` | Strict xfail that unexpectedly passes     | `@pytest.mark.xfail(strict=True)` + `assert True`      | Outer step `PASSED` (plugin never sees pytest's "strict xpass" failure attached to the report)  | Outer step `FAILED` (mark no longer matches reality — either the bug was fixed or the test stopped testing what it claimed) | Gap    |
| `XFAIL-03` | Non-strict xfail that unexpectedly passes | `@pytest.mark.xfail()` + `assert True`                 | Outer step `PASSED` (pytest reports outcome="passed" with `wasxfail` set; plugin ignores it)    | Outer step `PASSED` (`strict=False` doesn't insist on the failure) | OK    |
| `XFAIL-04` | `xfail(raises=...)` with wrong exception  | `@pytest.mark.xfail(raises=ValueError)` + `raise KeyError` | Outer step `ERROR` (treated as a generic non-assertion exception)                           | `FAILED` (the `raises=` mismatch is a real test failure) | Gap |
| `XFAIL-05` | `xfail(run=False)`                        | `@pytest.mark.xfail(run=False)` (body never executed)  | `SKIPPED` (only the makereport hook records a step)                                              | `SKIPPED` (the test never ran)                      | OK    |

## Setup / teardown phases

| Case       | Scenario                                   | Trigger                                                              | Observed today                                                                                                                            | Target                                                  | Status |
| ---------- | ------------------------------------------ | -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------ |
| `PHASE-01` | Setup-phase fixture failure (RuntimeError) | `@pytest.fixture` raises before `yield`; test body never runs        | Outer step does not exist or lands `PASSED`; the plugin does not consult `report.when`                                                    | `ERROR` with `phase=setup` annotation                   | Gap    |
| `PHASE-02` | Teardown-phase fixture failure             | `@pytest.fixture` raises after `yield`; test body passed             | Outer step `PASSED` — it closes before the failing teardown runs, so the error is invisible                                              | `FAILED` with `phase=teardown` annotation               | Gap    |
| `PHASE-03` | Call-phase fail **plus** teardown-phase fail | `assert 1 == 2` in body AND `@pytest.fixture` raises after `yield` | Outer step `FAILED` (the call-phase failure dominates); the teardown error is silently lost                                              | `FAILED` with a `phase=teardown` annotation so the teardown error is also visible | Gap |

## Collection / fixture-resolution failures

| Case      | Scenario                                | Trigger                                       | Observed today                                                                                                                                  | Target                                                  | Status |
| --------- | --------------------------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------ |
| `COLL-01` | Missing fixture                         | `def test_x(nonexistent_fixture):`            | Outer step `PASSED` — the autouse `step` fixture's setup still runs before pytest detects the missing fixture; the user sees a green step for a test that never executed | `ERROR` with `phase=setup`                              | Gap    |

## Plugin-API exit paths (in-test mutations)

| Case     | Scenario                                | Trigger                                                                | Observed today                                              | Target   | Status |
| -------- | --------------------------------------- | ---------------------------------------------------------------------- | ----------------------------------------------------------- | -------- | ------ |
| `API-01` | Manual status override                  | `step.current_step.update({"status": TestStatus.FAILED})`              | `FAILED`                                                    | `FAILED` | OK     |
| `API-02` | `report_outcome(result=False)`          | `step.report_outcome("the_check", False, "did not match")`             | `FAILED`                                                    | `FAILED` | OK     |
| `API-03` | `measure(...)` out-of-bounds            | `step.measure(name="m", value=10.0, bounds={"min": 0.0, "max": 5.0})`  | `FAILED`                                                    | `FAILED` | OK     |
| `API-04` | Failed measurement on a substep         | `with step.substep(...) as s: s.measure(... out-of-bounds)`            | `FAILED` (propagates from substep to parent)                | `FAILED` | OK     |
| `API-05` | Manually-skipped substep                | `with step.substep(...) as s: s.current_step.update({"status": SKIPPED})` | Parent step `PASSED` (skip does not propagate as a failure) | `PASSED` | OK     |

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

Gap rows fail today. When the plugin fix for a row lands, the matching
test turns green; update the **Observed today** column here to match the
target and flip the row's status to **OK**.
