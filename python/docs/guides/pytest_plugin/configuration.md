# Configuration & Defaults

This page is the full reference for everything the plugin exposes: fixtures, CLI
flags, ini options, credential handling, and the markers that control which
tests report. It opens with an audit that classifies every option so you know
what belongs in the happy path and what is reserved for specific needs.

## Defaults audit

Every knob, marker, flag, and fixture, classified by how it should be used:

- **Default**: part of the [Overview](index.md) happy path. You get it with
  zero config.
- **Advanced**: documented here and in the other guides, kept off the happy
  path. Reach for it when you have a specific need.
- **Removed**: none. Removing an option is a code change outside the scope of
  the docs; see [Candidates for removal](#candidates-for-removal) below.

| Option | Surface | Classification | Rationale |
|---|---|---|---|
| `pytest_plugins = ["sift_client.pytest_plugin"]` | `conftest.py` declaration | Default | The one required setup step. |
| `step` | fixture (autouse) | Default | Core API for measurements and substeps. |
| `step.measure(...)` | step method | Default | Primary way to record a measurement. |
| `SIFT_API_KEY` | env var | Default | Required credential in online mode. |
| `SIFT_GRPC_URI` | env var / `sift_grpc_uri` ini | Default | Required endpoint in online mode. |
| `SIFT_REST_URI` | env var / `sift_rest_uri` ini | Default | Required endpoint in online mode. |
| `sift_package_step` | ini (default `true`) | Default | Full step tree works out of the box. |
| `sift_module_step` | ini (default `true`) | Default | Full step tree works out of the box. |
| `sift_class_step` | ini (default `true`) | Default | Full step tree works out of the box. |
| `sift_parametrize_nesting` | ini (default `true`) | Default | Clustered parametrize tree works out of the box. |
| `sift_autouse` | ini (default `true`) | Default on; Advanced to override | On by default every test reports; set `false` only to opt in per test. |
| `sift_git_metadata` | ini (default `true`) | Default on; Advanced to override | Git metadata captured automatically; toggle off to suppress. |
| online mode | default | Default | Zero-config sends results to your tenant. |
| `report_context` | fixture (autouse) | Advanced | Only needed to set report metadata or link a Run. |
| `--sift-offline` / `sift_offline` | CLI / ini | Advanced | Field tests, air-gapped labs, CI without network. |
| `--sift-disabled` / `sift_disabled` / `SIFT_DISABLED` | CLI / ini / env | Advanced | Skip Sift entirely. |
| `--sift-log-file` / `sift_log_file` | CLI / ini | Advanced | Tune the replay sink; the default temp file is fine. |
| `--no-sift-git-metadata` | CLI | Advanced | Opt out of git metadata capture. |
| `sift_grpc_uri` / `sift_rest_uri` | ini | Advanced | Commit stable endpoints instead of using env vars. |
| `@pytest.mark.sift_include` | marker | Advanced | Force reporting on for a scope. |
| `@pytest.mark.sift_exclude` | marker | Advanced | Force reporting off for a scope. |
| `sift_client` override | fixture | Advanced | Custom client construction (TLS, timeouts, credentials). |
| `client_has_connection` override | fixture | Advanced | Custom reachability signal. |
| `pytest_collection_modifyitems` | hook | Advanced | Bulk-apply a marker to a directory. |

### Candidates for removal

No options are removed by this documentation. Each one above maps to live plugin
behavior, and removing a knob is a code change that belongs in a separate
follow-up. Revisit the "Advanced" rows if usage shows a knob is never changed
from its default.

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

The default `sift_client` fixture reads its two URIs from environment first and
falls back to ini keys when the env vars are unset. `SIFT_API_KEY` is
intentionally env-only, so keep it out of source control and supply it through
`pytest-dotenv` (see [API key handling](#api-key-handling) below). The env var
wins when both are set, so secrets injected into a CI environment continue to
override values committed to `pyproject.toml`. There are no CLI flags for
credentials.

| Ini key | Environment variable | Notes |
|---|---|---|
| _(none)_ | `SIFT_API_KEY` | Env-only. Use `.env` + `pytest-dotenv` locally; inject from your secret store in CI. |
| `sift_grpc_uri` | `SIFT_GRPC_URI` | Stable per-org gRPC endpoint; safe to commit. |
| `sift_rest_uri` | `SIFT_REST_URI` | Stable per-org REST endpoint; safe to commit. |

### API key handling

`SIFT_API_KEY` is deliberately read from the process environment only. The
recommended workflow uses the
[`pytest-dotenv`](https://pypi.org/project/pytest-dotenv/) plugin (already a
dependency of `sift-stack-py`), which loads variables from a `.env` file into
`os.environ` before tests run.

1. Add `.env` to `.gitignore`.
2. Drop your key into `.env` at the project root:

    ```bash title=".env"
    SIFT_API_KEY=sk-...your-key...
    ```

3. In CI, set `SIFT_API_KEY` directly via your provider's secret manager
   instead of committing a `.env` file.

`pytest-dotenv` picks the file up automatically; no `pytest_configure` glue is
needed.

!!! warning "FedRAMP / shared environments"
    Pass `--sift-log-file=false` (or set the ini key to `"false"`) to skip the
    temp file + worker pipeline. Create/update calls then run inline against the
    API instead of being deferred through a subprocess.

## Wire the plugin into `conftest.py`

A single `pytest_plugins` declaration in your top-level `conftest.py` is all
that's required. The plugin ships a default `sift_client` fixture that reads
`SIFT_API_KEY`, `SIFT_GRPC_URI`, and `SIFT_REST_URI` from the environment.

```python title="conftest.py"
from dotenv import load_dotenv

load_dotenv()

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
from dotenv import load_dotenv

from sift_client import SiftClient, SiftConnectionConfig

load_dotenv()

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
| `step` | fixture (autouse) | function | A `NewStep` created for the current test function. Exposes `measure*`, `substep`, `report_outcome`, `fail_if_measurements_failed`, and `current_step`. |
| `_hierarchy_parents` | internal fixture (autouse) | function | Opens a parent step for each `pytest.Package`, `pytest.Module`, and `pytest.Class` ancestor of the current test. Each layer is gated independently; see [ini options](#ini-options). |
| `_parametrize_parents` | internal fixture (autouse) | function | Opens a parent step for each `@pytest.mark.parametrize` axis (and fixture parametrization), nested inside the hierarchy parents. |
| `client_has_connection` | fixture | session | Calls `sift_client.ping.ping()`; consulted by `report_context` at session start in online mode (the default). Override to skip the ping or use a different reachability signal. |

## CLI options

| Flag | Default | Effect |
|---|---|---|
| `--sift-offline` | off (online) | Skip the session-start ping and don't contact Sift. All create/update calls go to the JSONL log file for later replay via `import-test-result-log`. Missing `SIFT_*` env vars are tolerated; placeholders are filled. |
| `--sift-disabled` | off | Skip Sift entirely. Nothing contacts the API and no log file is written; `step.measure(...)` still evaluates bounds and returns a real pass/fail boolean. Also honored via `SIFT_DISABLED=1`. Supersedes every other flag (disabled wins over offline). |
| `--sift-log-file=<path\|true\|false>` | temp file | Where the JSONL log of create/update calls goes. With a log file set, the plugin spawns an `import-test-result-log --incremental` worker that polls the file and replays entries against Sift while the run is in flight. Pass `false` to disable the file entirely; create/update calls then go straight to the API synchronously during tests. Incompatible with `--sift-offline` since offline mode needs the log file as its sole sink. |
| `--no-sift-git-metadata` | git metadata on | Skip capturing git repo/branch/commit on the report's metadata. |

These can be passed permanently via `addopts`:

```ini title="pytest.ini"
[pytest]
addopts = --sift-offline
```

## Ini options

Set the matching ini key directly (recommended for stable per-project
configuration). Each CLI flag has a corresponding key under
`[tool.pytest.ini_options]` in `pyproject.toml` or `[pytest]` in `pytest.ini`.
CLI flags, when passed, override the ini values.

| Ini key | Type | Equivalent CLI flag |
|---|---|---|
| `sift_log_file` | string (`true` / `false` / `none` / path) | `--sift-log-file=<value>` |
| `sift_git_metadata` | bool (default `true`) | `--no-sift-git-metadata` (sets to `false`) |
| `sift_offline` | bool (default `false`) | `--sift-offline` |
| `sift_disabled` | bool (default `false`) | `--sift-disabled` (also honors `SIFT_DISABLED` env var) |
| `sift_autouse` | bool (default `true`) | _(no CLI flag; controls the marker gate below)_ |
| `sift_package_step` | bool (default `true`) | _(ini-only)_. Opens a parent step for each Python package (directory with `__init__.py`) in the test path. |
| `sift_module_step` | bool (default `true`) | _(ini-only)_. Opens a parent step for each test module (file). |
| `sift_class_step` | bool (default `true`) | _(ini-only)_. Opens a parent step for each test class, including nested classes. |
| `sift_parametrize_nesting` | bool (default `true`) | _(ini-only)_. Clusters parametrized tests under shared parents (`test_x`, `axis=value`) instead of flat leaves (`test_x[value]`). |

```toml title="pyproject.toml"
[tool.pytest.ini_options]
sift_offline = true
sift_git_metadata = false
sift_grpc_uri = "your-org.sift.example:443"
sift_rest_uri = "https://your-org.sift.example"
```

```ini title="pytest.ini"
[pytest]
sift_offline = true
sift_git_metadata = false
sift_grpc_uri = your-org.sift.example:443
sift_rest_uri = https://your-org.sift.example
```

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
