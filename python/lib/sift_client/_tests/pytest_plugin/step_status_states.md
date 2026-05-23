# Pytest-plugin step-status: test scenarios

Reference for the pass/fail scenarios covered by
[`test_pass_fail.py`](test_pass_fail.py). Each row pairs a scenario with the
`TestStatus` the plugin records, and maps to the user-facing contract in
[`docs/guides/pytest_plugin/pass_fail_behavior.md`](../../../../docs/guides/pytest_plugin/pass_fail_behavior.md).

`TestStatus` values come from `sift_client.sift_types.test_report.TestStatus`:
`PASSED`, `FAILED`, `ERROR`, `SKIPPED`, `ABORTED`, `IN_PROGRESS`. Hard process
exits the plugin can observe (`SystemExit`, `KeyboardInterrupt` when pytest
delivers a call-phase report) map to `ABORTED`. A session-aborting interrupt
that fires before the plugin sees it leaves the step in `IN_PROGRESS`.

## Case ID scheme

Each scenario has a stable case ID of the form `PREFIX-NN`. Tests in
`test_pass_fail.py` reference their case ID in a leading comment so a test can
be traced back to its row here without rereading the scenario:

| Prefix  | Section                                  |
| ------- | ---------------------------------------- |
| `CALL`  | Call-phase exit paths                    |
| `SKIP`  | Skip paths                               |
| `XFAIL` | xfail / xpass                            |
| `PHASE` | Setup / teardown phases                  |
| `COLL`  | Collection / fixture-resolution failures |
| `API`   | Plugin-API exit paths                    |


## Call-phase exit paths

| Case      | Scenario                        | Trigger                              | Outcome                                                                                                  |
| --------- | ------------------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `CALL-01` | Test passes                     | function body returns cleanly        | `PASSED`                                                                                                 |
| `CALL-02` | Assert failure in call phase    | `assert 1 == 2`                      | `FAILED`                                                                                                 |
| `CALL-03` | Generic exception in call phase | `raise ValueError("boom")`           | `ERROR`                                                                                                  |
| `CALL-04` | `pytest.fail("...")` from body  | `pytest.fail("intentional failure")` | `FAILED`                                                                                                 |
| `CALL-05` | `SystemExit` from the test body | `sys.exit(1)`                        | `ABORTED`                                                                                                |
| `CALL-06` | `KeyboardInterrupt` in body     | `raise KeyboardInterrupt`            | `IN_PROGRESS` — session aborts before the plugin sees the interrupt; `ABORTED` if the plugin does see it |

## Skip paths

| Case      | Scenario                         | Trigger                                      | Outcome                                                                  |
| --------- | -------------------------------- | -------------------------------------------- | ------------------------------------------------------------------------ |
| `SKIP-01` | Collection-time skip             | `@pytest.mark.skip(reason=...)`              | `SKIPPED` — only the makereport hook records a step; no autouse step ran |
| `SKIP-02` | Conditional collection-time skip | `@pytest.mark.skipif(True, reason=...)`      | `SKIPPED` — same route as `@pytest.mark.skip`                            |
| `SKIP-03` | Runtime skip in body             | `pytest.skip("...")`                         | Outer step `SKIPPED`; no duplicate nested step                           |
| `SKIP-04` | Skip raised inside a fixture     | `@pytest.fixture` calls `pytest.skip("...")` | Outer step `SKIPPED` (setup-phase skip); no duplicate nested step        |

## xfail / xpass

| Case       | Scenario                                  | Trigger                                                    | Outcome                                                  |
| ---------- | ----------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------- |
| `XFAIL-01` | xfail-marked test that fails              | `@pytest.mark.xfail` + `assert 1 == 2`                     | `PASSED` — test fulfilled the xfail expectation          |
| `XFAIL-02` | Strict xfail that unexpectedly passes     | `@pytest.mark.xfail(strict=True)` + `assert True`          | `FAILED` — mark no longer matches reality                |
| `XFAIL-03` | Non-strict xfail that unexpectedly passes | `@pytest.mark.xfail()` + `assert True`                     | `PASSED` — `strict=False` doesn't insist on the failure  |
| `XFAIL-04` | `xfail(raises=...)` with wrong exception  | `@pytest.mark.xfail(raises=ValueError)` + `raise KeyError` | `FAILED` — `raises=` mismatch is a real test failure     |
| `XFAIL-05` | `xfail(run=False)`                        | `@pytest.mark.xfail(run=False)` (body never executed)      | `SKIPPED` — the test never ran                           |

## Setup / teardown phases

| Case       | Scenario                                     | Trigger                                                            | Outcome                                                                                                                          |
| ---------- | -------------------------------------------- | ------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- |
| `PHASE-01` | Setup-phase fixture failure (RuntimeError)   | `@pytest.fixture` raises before `yield`; test body never runs      | `ERROR` — plugin reads the setup-phase report and maps `failed` → `ERROR` (a `phase=setup` annotation is a planned follow-up)    |
| `PHASE-02` | Teardown-phase fixture failure               | `@pytest.fixture` raises after `yield`; test body passed           | `FAILED` — plugin upgrades a passed step when the teardown report shows `failed` (a `phase=teardown` annotation is a planned follow-up) |
| `PHASE-03` | Call-phase fail **plus** teardown-phase fail | `assert 1 == 2` in body AND `@pytest.fixture` raises after `yield` | `FAILED` — call-phase failure dominates; surfacing the teardown error alongside is a planned follow-up                           |

## Collection / fixture-resolution failures

| Case      | Scenario        | Trigger                            | Outcome                                                                                                            |
| --------- | --------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `COLL-01` | Missing fixture | `def test_x(nonexistent_fixture):` | `ERROR` — missing fixture surfaces as a setup-phase failure (a `phase=setup` annotation is a planned follow-up)    |

## Plugin-API exit paths (in-test mutations)

| Case     | Scenario                          | Trigger                                                                   | Outcome                                                                                                                     |
| -------- | --------------------------------- | ------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `API-01` | Manual status override            | `step.current_step.update({"status": TestStatus.FAILED})`                 | `FAILED`                                                                                                                    |
| `API-02` | `report_outcome(result=False)`    | `step.report_outcome("the_check", False, "did not match")`                | `FAILED`                                                                                                                    |
| `API-03` | `measure(...)` out-of-bounds      | `step.measure(name="m", value=10.0, bounds={"min": 0.0, "max": 5.0})`     | `FAILED`                                                                                                                    |
| `API-04` | Failed measurement on a substep   | `with step.substep(...) as s: s.measure(... out-of-bounds)`               | `FAILED` — propagates from substep to parent                                                                                |
| `API-05` | Manually-skipped substep          | `with step.substep(...) as s: s.current_step.update({"status": SKIPPED})` | Parent step `PASSED` — skip does not propagate as a failure                                                                 |
| `API-06` | Hard exit inside a nested substep | `with step.substep(...) as s: with s.substep(...): sys.exit(1)`           | Every open step on the unwind path records `ABORTED`; a sibling substep that closed before the abort keeps its prior status |

## Out of scope

Scenarios deliberately not covered by this suite:

- **Timeout** — needs `pytest-timeout` or a manual signal harness.
- **Signal (SIGKILL / SIGTERM)** — cannot be caught from inside the process;
  needs a subprocess-level harness.
- **`pytest.exit("...")`** — niche; the "aborts subsequent tests" behavior
  is hard to characterize cleanly because each `pytester` invocation is
  its own session.
- **`os._exit()`** — bypasses Python cleanup entirely; can't be tested
  in-process because it would kill the outer pytest run. Guaranteed
  data-loss case alongside `SystemExit` / `SIGKILL`.
- **Parametrize-level marks** (`pytest.param(..., marks=pytest.mark.xfail / skip)`)
  — routes through a different selection path but produces the same
  `report.outcome`, so behavior matches the function-level marks already
  covered above.
- **Import error / syntax error / `conftest.py` error** — these fail
  collection entirely; no `item` is produced and no plugin hook fires, so
  no Sift step is recorded.
