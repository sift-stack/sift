# Configuration & Defaults

This page is the full reference for everything the plugin exposes: fixtures, CLI
flags, ini options, credential handling, and the markers that control which
tests report.

!!! info "Where the plugin lives"
    The plugin lives at `sift_client.pytest_plugin`. It is **not** registered as
    a `pytest11` entry point. Projects opt in with a `pytest_plugins` declaration
    in their top-level `conftest.py`. Pytest then loads the module as a real
    plugin: the fixtures, CLI options, and `pytest_runtest_makereport` hook all
    register through standard pytest machinery, so `pytest --trace-config` lists
    it and `pytest -p no:sift_client.pytest_plugin` disables it.

## Credentials

Set the connection details in a `.env` next to your tests:

```bash
SIFT_API_KEY="your-api-key"
SIFT_GRPC_URI="..."
SIFT_REST_URI="..."
```

The `SIFT_GRPC_URI` and `SIFT_REST_URI` are the gRPC and REST endpoints for your
Sift organization. You can find these on the Sift Manage page as well as
generate an API key.

The default `sift_client` fixture reads its two URIs from the environment
first, then from the `sift_grpc_uri` / `sift_rest_uri` ini keys.
`SIFT_API_KEY` is intentionally env-only, so keep it out of source control (see
[API key handling](#api-key-handling) below). There are no CLI flags for
credentials.

| Setting | Where | Notes |
|---|---|---|
| `SIFT_API_KEY` | env var only | Inject from your secret store in CI; for local dev use a `.env` (see below). Never read from a committed file. |
| `SIFT_GRPC_URI` | env > `sift_grpc_uri` ini | Stable per-org gRPC endpoint; safe to commit. |
| `SIFT_REST_URI` | env > `sift_rest_uri` ini | Stable per-org REST endpoint; safe to commit. |

### API key handling

`SIFT_API_KEY` is read from the process environment only — the plugin never
reads it from a committed file. How you get it into the environment is up to
you:

- **CI:** set `SIFT_API_KEY` directly via your provider's secret manager.
- **Local dev:** keep the values in a `.env` (gitignored) and let
  [`pytest-dotenv`](https://pypi.org/project/pytest-dotenv/) load them — it is
  not bundled with `sift-stack-py`, so install it explicitly:

    ```bash
    pip install pytest-dotenv
    ```

    ```bash title=".env"
    SIFT_API_KEY=sk-...your-key...
    SIFT_GRPC_URI=your-org.grpc.example.com
    SIFT_REST_URI=https://your-org.rest.example.com
    ```

    Once installed, pytest-dotenv auto-loads `.env` from the rootdir before
    tests run — no `conftest.py` glue and no `load_dotenv()` call. (Point it at
    a different file with the `env_files` ini key if you prefer.)

Prefer real environment variables (shell exports, CI secrets) for anything you
can't keep in a local file.

!!! warning "FedRAMP / shared environments"
    Pass `--sift-log-file=false` (or set the ini key to `"false"`) to skip the
    temp file + worker pipeline. Create/update calls then run inline against the
    API instead of being deferred through a subprocess.

## Wire the plugin into `conftest.py`

A single `pytest_plugins` declaration in your top-level `conftest.py` is all
that's required. The plugin ships a default `sift_client` fixture that reads
`SIFT_API_KEY`, `SIFT_GRPC_URI`, and `SIFT_REST_URI` from the environment.

```python title="conftest.py"
pytest_plugins = ["sift_client.pytest_plugin"]
```

That's the whole setup. Every test in the session will now create a step on a
single shared `TestReport`.

### Customizing the `SiftClient`

To construct the client differently (custom TLS, timeouts, alternate
credentials, etc.), override the `sift_client` fixture in your conftest. The
plugin's default falls away in favor of your definition.

```python title="conftest.py"
import os

import pytest

from sift_client import SiftClient, SiftConnectionConfig

pytest_plugins = ["sift_client.pytest_plugin"]


@pytest.fixture(scope="session")
def sift_client() -> SiftClient:
    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=os.getenv("SIFT_API_KEY"),
            grpc_url=os.getenv("SIFT_GRPC_URI"),
            rest_url=os.getenv("SIFT_REST_URI"),
            use_ssl=False,
        )
    )
```

## Plugin provided fixtures

| Name | Kind | Scope | Purpose |
|---|---|---|---|
| `report_context` | fixture (autouse) | session | The `ReportContext` backing the run's `TestReport`. Use it to attach metadata or open ad-hoc steps. |
| `step` | fixture (autouse) | function | A `NewStep` created for the current test function. Exposes `measure*`, `substep`, `report_outcome`, `pytest_fail_if_step_failed`, and `current_step`. |
| `_sift_parents` | internal fixture (autouse) | function | Resolves the report-tree parents for the current test: a parent step for each `pytest.Package`, `pytest.Module`, and `pytest.Class` ancestor, then one per `@pytest.mark.parametrize` axis (and fixture parametrization) nested inside them. Parents are created once and reused across tests in any order, so test execution order is never changed. Each layer is gated independently; see [settings reference](#settings-reference). |
| `client_has_connection` | fixture | session | Calls `sift_client.ping.ping()`; consulted by `report_context` at session start in online mode (the default). Override to skip the ping or use a different reachability signal. |

## Settings reference

Every setting the plugin reads, grouped by the three config kinds. Within a
group, a `—` means the setting can't be set from that surface.

Each kind has a home chosen for a specific workflow:

- **Pytest behavior** lives in `[tool.pytest.ini_options]` (log/offline/disabled/git/`*_step`/autouse/parametrize). A CLI flag exists for the ones with a real ad-hoc override workflow.
- **Connection** comes from the environment first, falling back to the ini keys; the API key is env-only so secrets stay out of committed files.
- **Report content** takes static defaults from `[tool.sift.pytest.report]` and per-run dynamic values from `SIFT_REPORT_*` env vars (CI builds, hardware cycling, anything `.env`-driven; pytest-dotenv loads `.env` for local dev).

**Precedence within a setting:** env > CLI flag > ini key > TOML > built-in
default. No setting exposes both env and CLI, so the chain isn't ambiguous in
practice.

The plugin scans `SIFT_*` env vars and `[tool.sift.pytest.*]` keys at session
start; anything outside these tables fires a warning with a closest-match
suggestion, so typos like `SIFT_REPORT_SERIALNUM` surface immediately.

<!-- BEGIN settings-reference (auto-generated from PLUGIN_OPTIONS in sift_client/_internal/pytest_plugin/options.py; regenerate via test_settings_reference_docs_in_sync) -->
### Pytest behavior

| Setting | CLI flag | Ini (`[tool.pytest.ini_options]`) |
|---|---|---|
| Path to the JSONL log of create/update calls (path \| true \| false \| none). | `--sift-log-file` | `sift_log_file` |
| Capture git repo/branch/commit on the report. | `--no-sift-git-metadata` | `sift_git_metadata` |
| Skip the session-start ping; route create/update through the JSONL log. | `--sift-offline` | `sift_offline` |
| Disable Sift entirely (no API calls, no log file). Supersedes --sift-offline. | `--sift-disabled` | `sift_disabled` |
| Open the resulting report in a browser at session end (online only; no-op when the report URL can't be resolved). | `--sift-open-report` | `sift_open_report` |
| Default for the Sift autouse fixtures (report_context, step, hierarchy/parametrize parents). | — | `sift_autouse` |
| Open a parent step for each Python package in the test path. | — | `sift_package_step` |
| Open a parent step for each test module. | — | `sift_module_step` |
| Open per-class parent steps, including nested classes. | — | `sift_class_step` |
| Cluster parametrized tests under shared parent steps (e.g. test_a -> v=1, v=2). | — | `sift_parametrize_nesting` |

### Connection

| Setting | Ini (`[tool.pytest.ini_options]`) | Env var |
|---|---|---|
| Sift API key (secret, env-only). | — | `SIFT_API_KEY` |
| Sift gRPC endpoint URI. | `sift_grpc_uri` | `SIFT_GRPC_URI` |
| Sift REST endpoint URI. | `sift_rest_uri` | `SIFT_REST_URI` |
| Sift web-app origin for the report link in the terminal footer (e.g. https://app.siftstack.com). When unset, the link is derived from the REST URI for known Sift hosts. | `sift_app_url` | `SIFT_APP_URL` |

### Report content

| Setting | TOML (`[tool.sift...]`) | Env var |
|---|---|---|
| Template for the report display name. Placeholders: {target}, {command}, {args}, {rootdir}, {timestamp}, {count}, {git_repo}, {git_branch}, {git_commit}. | `[tool.sift.pytest.report] name` | — |
| Template for the report's test_case field (same placeholders as report_name). | `[tool.sift.pytest.report] test_case` | — |
| Name of the test system / rig. Defaults to the host's name. | `[tool.sift.pytest.report] test_system_name` | `SIFT_REPORT_TEST_SYSTEM_NAME` |
| Operator running the test. Defaults to the OS user. | `[tool.sift.pytest.report] system_operator` | `SIFT_REPORT_SYSTEM_OPERATOR` |
| Serial number of the unit under test. | `[tool.sift.pytest.report] serial_number` | `SIFT_REPORT_SERIAL_NUMBER` |
| Part number of the unit under test. | `[tool.sift.pytest.report] part_number` | `SIFT_REPORT_PART_NUMBER` |
| Free-form report metadata, as a TOML table of scalar values. For dynamic per-run keys, attach them in conftest via the report_context fixture. | `[tool.sift.pytest.report.metadata]` (table) | — |
<!-- END settings-reference -->

### Quick-start examples

```toml title="pyproject.toml"
[tool.pytest.ini_options]
sift_offline = true
sift_git_metadata = false
sift_grpc_uri = "your-org.sift.example:443"
sift_rest_uri = "https://your-org.sift.example"

[tool.sift.pytest.report]
name = "{rootdir} ({count} tests) {timestamp}"
test_system_name = "rig-7"

[tool.sift.pytest.report.metadata]
build_id = "v1.2.3"
```

```bash title="CI env (set by your runner)"
SIFT_API_KEY=...                    # from a secret manager
SIFT_REPORT_SYSTEM_OPERATOR=ci-bot
SIFT_REPORT_SERIAL_NUMBER=$UNIT_SN  # cycles per matrix job
```

```ini title="pytest.ini (alternative — pytest-execution flags only)"
[pytest]
sift_offline = true
sift_git_metadata = false
sift_grpc_uri = your-org.sift.example:443
sift_rest_uri = https://your-org.sift.example
```

CLI flags can be made permanent via `addopts`:

```ini title="pytest.ini"
[pytest]
addopts = --sift-offline
```

## Report content in depth

The [settings reference](#settings-reference) above maps each report-content
field to its `[tool.sift.pytest.report]` key and `SIFT_REPORT_*` env var. This
section covers the two template fields and the metadata table in more detail.

```toml title="pyproject.toml — static project defaults"
[tool.sift.pytest.report]
name             = "{rootdir} {git_branch} ({count} tests) {timestamp}"
test_case        = "{rootdir}-{git_branch}"
test_system_name = "rig-7"
system_operator  = "ci-bot"
serial_number    = "SN-001"
part_number      = "PN-9000"
```

```bash title="Per-run overrides — CI or hardware-bench shell"
SIFT_REPORT_SERIAL_NUMBER=$UNIT_SN \
SIFT_REPORT_SYSTEM_OPERATOR=$CI_ACTOR \
pytest tests/
```

### `name` vs `test_case`

The two fields look similar but serve opposite purposes:

- **`name`** is the report's **per-run display label** — what you see in the
  Test Results list. It should be unique per run, which is why its default ends
  in `{timestamp}`.
- **`test_case`** is the **cross-run grouping key** — reports that share a
  `test_case` are treated as runs of the *same* case, so Sift can track its
  pass/fail history over time. It should be stable across runs, which is why
  its default has **no** timestamp.

By default both derive from the same `{target}` (what ran), and the timestamp
is the only difference: `name` = `{target} {timestamp}` (distinct each run),
`test_case` = `{target}` (identical across runs of the same target, so they
group together). Set either explicitly to override — a static `test_case` like
`"{rootdir}"` is common when you want every run of a project to group under one
case regardless of which subset ran.

### Templates for `name` and `test_case`

`name` and `test_case` accept the same f-string-style placeholders:

| Placeholder | Value |
|---|---|
| `{target}` | What ran, derived from the collected tests (not the command line) and anchored to the project name: `project/tests/test_x.py::test_y` for a single test (the `[param]` suffix is stripped), `project/tests/test_x.py` for a single file, `project/tests/motor` for several files' common directory, or just `project` for a whole-suite run. |
| `{command}` | The full pytest invocation, e.g. `pytest tests/ -k smoke`. |
| `{args}` | The invocation arguments without the leading `pytest`. |
| `{rootdir}` | The pytest rootdir name (typically the project directory). |
| `{timestamp}` | The report start time in ISO 8601 (UTC). |
| `{count}` | The number of collected tests in the run. |
| `{git_repo}` | The `origin` remote URL, or empty when not in a git repo. |
| `{git_branch}` | The current branch, or empty when not in a git repo. |
| `{git_commit}` | The current commit (`git describe --always --dirty`), or empty when not in a git repo. |

**Defaults when unset.** Because `{target}` is derived from the collected
tests, the defaults reflect what actually ran and don't change with flag order
or `-k` / `-m` filters:

(`<project>` below is the rootdir directory name.)

| Invocation | default `name` | default `test_case` |
|---|---|---|
| `pytest tests/test_motor.py::test_spin[12V]` | `<project>/tests/test_motor.py::test_spin 2026-...` | `<project>/tests/test_motor.py::test_spin` |
| `pytest -v tests/test_motor.py` | `<project>/tests/test_motor.py 2026-...` | `<project>/tests/test_motor.py` |
| `pytest -k motor` (hits `tests/motor/`) | `<project>/tests/motor 2026-...` | `<project>/tests/motor` |
| `pytest` (whole suite) | `<project> 2026-...` | `<project>` |

The git placeholders are resolved independently of `--no-sift-git-metadata`
(which only controls whether git values are stored on the report metadata) and
render empty outside a git checkout. An unknown placeholder is reported as a
warning and the value falls back to the default rather than failing the run.

Regardless of the name, the full pytest command is always preserved on the
report's metadata under the `pytest_command` key, so the exact invocation stays
queryable and viewable in the report detail.

### Report metadata

`[tool.sift.pytest.report.metadata]` is a TOML table whose typed values land
on the report's metadata alongside the git fields and the auto-recorded
`pytest_command`. Use it for build IDs, fixture identifiers, shift labels,
and any key/value data not otherwise modeled.

```toml title="pyproject.toml — static metadata defaults"
[tool.sift.pytest.report.metadata]
build_id = "v1.2.3"
fixture  = "PSU-A"
shift    = "night"
lane     = 2          # ints, floats, and bools come through with their TOML type
verbose  = true
```

For per-run dynamic entries (CI build IDs, cycling serial numbers), attach them
in your `conftest.py` through the `report_context` fixture rather than the TOML
table.

Nested tables, lists, and `null` values in
`[tool.sift.pytest.report.metadata]` are skipped with a warning since the
report's metadata is a flat `dict[str, str | float | bool]`.

## Controlling which tests produce reports

By default every test in the session produces a Sift step. Two markers and one
ini key let you narrow that to a specific set of tests, which is useful when a
repo holds tests that you don't want included in the Sift test report.

| Setting                                                 | Effect                                                                                       |
|---------------------------------------------------------|----------------------------------------------------------------------------------------------|
| `sift_autouse = false` in `pyproject.toml` | Flip the project-wide default off. Tests no longer produce steps unless explicitly opted in. |
| `@pytest.mark.sift_include` on a test, class, or module | Force reporting on for that scope, regardless of the project default.                        |
| `@pytest.mark.sift_exclude` on a test, class, or module | Force reporting off for that scope, regardless of the project default.                       |

Closest marker determines setting. `sift_exclude` beats `sift_include` when both apply.
`pytestmark` at the class or module level inherits to every test in scope.

### Bulk-applying a marker to a directory

To opt an entire directory in (or out) without editing each file, hook
`pytest_collection_modifyitems` in the directory's `conftest.py`:

```python title="tests/example/conftest.py"
from pathlib import Path

import pytest

_HERE = Path(__file__).parent


def pytest_collection_modifyitems(config, items):
    for item in items:
        try:
            item.path.relative_to(_HERE)
        except ValueError:
            continue
        item.add_marker(pytest.mark.sift_include)
```

This applies `sift_include` to every test collected under `tests/example/`.
Combine with `sift_autouse = false` in `pyproject.toml` for opting in to
specific directories.

`pytest_collection_modifyitems` receives every item in the session, not just
this directory's, so the `relative_to` filter is what scopes the marker.
