# Running modes

The plugin runs in one of three modes, picked at invocation. This page covers
how each mode behaves, the log-file/replay pipeline, and how to replay a saved
log against Sift.

## Running the suite

```bash
# Full run against your Sift tenant
pytest

# Put this run's artifacts (JSONL log, audit trace) in a known directory
pytest --sift-output-dir=./sift-results
```

## The three modes

| Mode | Flag | Network | Log file | `step.measure(...)` | When to use |
|---|---|---|---|---|---|
| Online (default) | _(none)_ | yes (pings at session start, aborts if it fails) | write-through backup, on by default | real measurement against Sift | CI with Sift credentials, local dev hitting your tenant |
| Offline | `--sift-offline` | none | required (the sole sink) | real measurement queued to log | field tests, air-gapped labs, CI without network |
| Disabled | `--sift-disabled` | none | none | bounds eval; returns a real bool | local dev or CI that doesn't have (or want) Sift |

Pass both flags and disabled wins: it skips Sift entirely and supersedes every
other setting.

## Terminal output

Each run prints a header with the SDK version and active mode, and an end-of-run
`Sift report` panel summarizing the outcome. Both are suppressed under `-q`. The
panel is color-coded when the terminal supports it (green pass, red
failure/error, yellow skip, cyan link) and plain text otherwise (`--color=no`,
captured output, CI logs).

The section title carries the report name (truncated if long). The `Steps` row
tallies every step in the report by final status, so it counts substeps and the
package/module/class/parametrize grouping steps too. Its totals are expected to
exceed pytest's own test count. The `Measurements` row tallies recorded
measurements (`step.measure(...)`) and is omitted when there are none. The
`Test case` and `System` rows echo the report's test case, test system, and
operator.

**Online** shows the report metadata, step and measurement breakdowns, and a
clickable link. The web host is derived from the REST URI for known Sift hosts;
for on-prem or custom deployments set `sift_app_url`
(ini) or the `SIFT_APP_URL` env var. Add `--sift-open-report` to
open the report in a browser at session end.

```text
============================= test session starts ==============================
platform linux -- Python 3.11.8, pytest-8.3.2, pluggy-1.5.0
Sift: sift-stack-py 0.18.0 — online mode
collected 12 items

tests/test_battery.py ........                                           [ 66%]
tests/test_thermal.py ....                                               [100%]

================ Sift report · pytest tests/ 2026-05-27T22:44:23Z ==============
  Test case    pytest tests/
  Status       PASSED       online · sift-stack-py 0.18.0
  Steps        14 passed
  Measurements 42 passed
  System       ci-runner-7 · cibot
  Log file     /tmp/sift_test_results/a1b2c3/a1b2c3.jsonl
  Report       https://app.siftstack.com/test-results/0193f1a2-7c44-7e5b-9b1a-2f6c0d9e84aa
============================== 12 passed in 3.45s ==============================
```

If the background uploader doesn't finish, the panel still links the report and
flags that it may be incomplete:

```text
================ Sift report · pytest tests/ 2026-05-27T22:44:23Z ==============
  Test case    pytest tests/
  Status       FAILED       online · sift-stack-py 0.18.0
  Steps        11 passed · 2 failed · 1 error
  Measurements 40 passed · 3 failed
  System       ci-runner-7 · cibot
  Log file     /tmp/sift_test_results/a1b2c3/a1b2c3.jsonl
  Report       https://app.siftstack.com/test-results/0193f1a2-7c44-7e5b-9b1a-2f6c0d9e84aa
               may be incomplete — finish with: import-test-result-log /tmp/sift_test_results/a1b2c3/a1b2c3.jsonl
```

When the web host can't be resolved and no override is set, the `Report` row
shows the report id instead of a link.

**Offline** shows the metadata and breakdowns, then the upload command under a
small rule (the log path is part of the command):

```text
================ Sift report · pytest tests/ 2026-05-27T22:44:23Z ==============
  Test case    pytest tests/
  Status       PASSED       offline · not uploaded
  Steps        14 passed
  Measurements 42 passed
  System       ci-runner-7 · cibot
  Log file     /tmp/sift_test_results/a1b2c3/a1b2c3.jsonl
------------------------------ to upload to Sift -------------------------------
  >> import-test-result-log /tmp/sift_test_results/a1b2c3/a1b2c3.jsonl
```

**Disabled** notes that no report was created:

```text
===================================== Sift =====================================
Sift disabled — no test report created.
```

## Online mode (default)

`report_context` resolves `client_has_connection` at session start. The default
implementation calls `sift_client.ping.ping()`. A failed ping aborts the whole
session with `pytest.UsageError` and points at `--sift-offline` and
`--sift-disabled` as escape hatches.

This is loud on purpose. A CI run that silently no-ops on a flaky network won't
get noticed until somebody goes looking for the report, which is usually weeks
later, which is usually too late.

With the JSONL log on by default, create/update calls are written to a log file
in the run's output directory during the run, and an
`import-test-result-log --incremental` worker replays them against Sift in the
background. If the worker crashes mid-session (connection failure, API error) or
is still draining its backlog at session end, the failure is logged at session
end with a `replay-test-result-log` command for manual recovery. Test outcomes
are unaffected and the local log file is preserved. Pass `--no-sift-log-file` to
make every create/update synchronous against the API instead.

### Overriding the connection check

Override `client_has_connection` when ping isn't the right signal, for example a
token cache that's only warm when authenticated:

```python title="conftest.py"
from pathlib import Path

import pytest


@pytest.fixture(scope="session")
def client_has_connection(sift_client) -> bool:
    return Path("~/.sift-token-cache").expanduser().is_file()
```

The override is ignored under `--sift-offline` and `--sift-disabled`.

## Offline mode (`--sift-offline`)

Same fixtures, same `step.measure(...)` semantics as online. The difference is
where the writes go: every create/update lands in a JSONL log file instead of
hitting the Sift API. The session-start ping is skipped, missing `SIFT_*` env
vars are tolerated (placeholders are filled), and the replay worker
(`import-test-result-log --incremental`) does not get spawned at session end.

```bash
pytest --sift-offline --sift-output-dir=./offline-runs
```

The summary panel prints the exact log path and the replay command. Once you
have connectivity, replay it:

```bash
import-test-result-log ./offline-runs/a1b2c3/a1b2c3.jsonl
```

That replay creates the report, steps, and measurements against Sift. See
[Replaying a saved log file](#replaying-a-saved-log-file) for cleanup and the
incremental flag.

`--no-sift-log-file` is rejected when offline is set, since the log is the only
sink in offline mode and without it the results are gone.

!!! note "Finding the log path"
    Without `--sift-output-dir`, offline mode writes the log to a random
    subfolder under the system temp directory. Either way the end-of-run panel
    prints the exact path and the `import-test-result-log` command. Pass
    `--sift-output-dir=<dir>` to put the run's artifacts somewhere you choose.

## Disabled mode (`--sift-disabled`)

The plugin stays loaded with the same fixtures and markers as the other modes.
Nothing contacts Sift, no log file is written, and no `SIFT_*` env vars are
required. `step.measure(...)`, `step.measure_avg(...)`, `step.measure_all(...)`,
`step.substep(...)`, and `report_context.report.update({...})` all behave
normally: bounds evaluate and you get a real pass/fail boolean back.

Entities returned in disabled mode report `is_simulated == True` (on
`TestReport`, `TestStep`, `TestMeasurement`, and `ReportContext`) so consumers
and tests can branch on provenance. Offline-mode entities also report
`is_simulated == True`.

How to turn it on, in the order most projects pick:

```bash
# Per-invocation kill-switch
pytest --sift-disabled
```

```toml
# Per-project default (uncommon; online is usually the right default)
# pyproject.toml:
[tool.pytest.ini_options]
sift_disabled = true
```

Good fit for local dev without Sift credentials, for library consumers who don't
have a Sift tenant, and for CI runs that shouldn't add noise to the report
stream, like a PR job re-running the same suite five times in a row.

## Replaying a saved log file

When the worker doesn't finish cleanly the plugin will print a hint mentioning
`import-test-result-log`. To import:

```bash
import-test-result-log <path-to-log.jsonl>
```

That replays the saved JSONL log as a single batch (no `--incremental`) and
deletes the file when it lives under the system temp dir.
