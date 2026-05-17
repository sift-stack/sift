from __future__ import annotations

import os
from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING, Any, Generator

import pytest

from sift_client import SiftClient, SiftConnectionConfig
from sift_client.sift_types.test_report import TestStatus
from sift_client.util.test_results import ReportContext

if TYPE_CHECKING:
    from sift_client.util.test_results.context_manager import NewStep

REPORT_CONTEXT: ReportContext | None = None

# Ini keys that mirror the registered CLI options. Centralized so renames stay in sync.
_INI_LOG_FILE = "sift_test_results_log_file"
_INI_GIT_METADATA = "sift_test_results_git_metadata"
_INI_CHECK_CONNECTION = "sift_test_results_check_connection"
_INI_GRPC_URI = "sift_grpc_uri"
_INI_REST_URI = "sift_rest_uri"


def pytest_addoption(parser: pytest.Parser) -> None:
    """Register Sift-specific command-line options and ini keys.

    Each option can be set on the command line or under ``[tool.pytest.ini_options]``
    in ``pyproject.toml`` (or ``[pytest]`` in ``pytest.ini``). CLI values take
    precedence over ini values, which take precedence over the built-in default.
    """
    group = parser.getgroup("sift", description="Sift test results")
    group.addoption(
        "--sift-test-results-log-file",
        default=None,
        help="Path to write the Sift test result log file. "
        "Use 'true' (default) to auto-create a temp file, "
        "False, 'false', or 'none' to disable logging, "
        "or a file path to write to a specific location.",
    )
    parser.addini(
        _INI_LOG_FILE,
        help="Default value for --sift-test-results-log-file. Same values "
        "accepted as the CLI flag (path, 'true', 'false', 'none').",
        default=None,
    )
    group.addoption(
        "--no-sift-test-results-git-metadata",
        action="store_false",
        dest="sift_test_results_git_metadata",
        default=None,
        help="Exclude git metadata from the Sift test results. "
        "Git metadata (repo, branch, commit) is included by default.",
    )
    parser.addini(
        _INI_GIT_METADATA,
        help="Include git repo/branch/commit in the report (true/false). "
        "Defaults to true. The --no-sift-test-results-git-metadata CLI flag "
        "overrides this when passed.",
        type="bool",
        default=True,
    )
    group.addoption(
        "--sift-test-results-check-connection",
        action="store_true",
        default=None,
        help="Skip the sift test-result fixtures (report_context, step, module_substep) "
        "when the Sift client has no connection to the server. Requires a "
        "`client_has_connection` fixture to be available in the test session.",
    )
    parser.addini(
        _INI_CHECK_CONNECTION,
        help="When true, skip the sift test-result fixtures if the client has "
        "no connection (same effect as --sift-test-results-check-connection). "
        "Defaults to false.",
        type="bool",
        default=False,
    )
    parser.addini(
        _INI_GRPC_URI,
        help="Sift gRPC endpoint URI. The default `sift_client` fixture "
        "prefers the SIFT_GRPC_URI environment variable and falls back to "
        "this ini value.",
        default=None,
    )
    parser.addini(
        _INI_REST_URI,
        help="Sift REST endpoint URI. The default `sift_client` fixture "
        "prefers the SIFT_REST_URI environment variable and falls back to "
        "this ini value.",
        default=None,
    )


def _option_or_ini(pytestconfig: pytest.Config | None, name: str) -> Any:
    """Resolve a Sift plugin setting from CLI > ini > None.

    The ``addoption`` registrations use ``default=None`` so we can tell whether
    the CLI was actually used. When the CLI didn't set a value, fall back to
    the matching ``addini`` key.
    """
    if pytestconfig is None:
        return None
    cli = pytestconfig.getoption(name, default=None)
    if cli is not None:
        return cli
    try:
        return pytestconfig.getini(name)
    except (KeyError, ValueError):
        return None


def _resolve_log_file(pytestconfig: pytest.Config | None) -> str | Path | bool | None:
    """Determine log_file value from CLI flag or ini key."""
    raw = _option_or_ini(pytestconfig, _INI_LOG_FILE)
    if not raw:
        # None, empty string from ini, or False — treat as "use temp file default".
        return True
    lower = str(raw).lower()
    if lower in ("true", "1"):
        return True
    if lower in ("false", "none"):
        return None
    return Path(raw)


@pytest.hookimpl(tryfirst=True, hookwrapper=True)
def pytest_runtest_makereport(item: pytest.Item, call: pytest.CallInfo[Any]):
    """Capture pytest outcomes so assertion failures and skips land on the Sift step."""
    outcome = yield
    report = outcome.get_result()
    if report.outcome == "skipped":
        # Skipped tests bypass the autouse `step` fixture, so we record the step manually here.
        if REPORT_CONTEXT:
            with REPORT_CONTEXT.new_step(name=item.name) as new_step:
                new_step.current_step.update({"status": TestStatus.SKIPPED})
    setattr(item, "rep_" + report.when, call)


def _report_context_impl(
    sift_client: SiftClient,
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config | None = None,
) -> Generator[ReportContext | None, None, None]:
    args = request.config.invocation_params.args
    test_path = Path(args[0]) if args else None
    if test_path is not None and test_path.exists():
        base_name = test_path.name
        test_case: Path | str = test_path
    else:
        base_name = "pytest " + " ".join(args) if args else "pytest"
        test_case = base_name
    log_file = _resolve_log_file(pytestconfig)
    git_metadata = _option_or_ini(pytestconfig, _INI_GIT_METADATA)
    include_git_metadata = True if git_metadata is None else bool(git_metadata)
    with ReportContext(
        sift_client,
        name=f"{base_name} {datetime.now(timezone.utc).isoformat()}",
        test_case=str(test_case),
        log_file=log_file,
        include_git_metadata=include_git_metadata,
    ) as context:
        global REPORT_CONTEXT
        REPORT_CONTEXT = context
        yield context


def _check_connection_enabled(pytestconfig: pytest.Config | None) -> bool:
    """Return True when the caller opted into the check-connection mode via CLI or ini."""
    return bool(_option_or_ini(pytestconfig, _INI_CHECK_CONNECTION))


def _has_sift_connection(request: pytest.FixtureRequest) -> bool:
    """Resolve the `client_has_connection` fixture lazily; only called when the check is enabled."""
    return bool(request.getfixturevalue("client_has_connection"))


_CREDENTIAL_KEYS: tuple[tuple[str, str | None], ...] = (
    ("SIFT_API_KEY", None),  # env-only; never read from ini to keep secrets out of source control.
    ("SIFT_GRPC_URI", _INI_GRPC_URI),
    ("SIFT_REST_URI", _INI_REST_URI),
)


def _resolve_credential(
    pytestconfig: pytest.Config | None, env_name: str, ini_name: str | None
) -> str | None:
    """Resolve a Sift credential: env var first, then ini key (if registered), else None."""
    env_value = os.getenv(env_name)
    if env_value:
        return env_value
    if ini_name is None or pytestconfig is None:
        return None
    return pytestconfig.getini(ini_name) or None


@pytest.fixture(scope="session")
def sift_client(pytestconfig: pytest.Config) -> SiftClient:
    """Default ``SiftClient`` resolved from environment variables and ini keys.

    Each credential is read from its environment variable first. The URIs
    (``SIFT_GRPC_URI``, ``SIFT_REST_URI``) additionally fall back to the
    ``sift_grpc_uri`` / ``sift_rest_uri`` ini keys, since they are stable
    per-org values that are safe to commit. ``SIFT_API_KEY`` is intentionally
    env-only — use ``pytest-dotenv`` (already a project dependency) to load
    it from a ``.env`` file kept out of version control.

    Projects that need custom construction (TLS toggles, custom timeouts,
    etc.) can override this fixture by defining their own ``sift_client``
    in their ``conftest.py``; pytest fixture resolution prefers the local
    definition.
    """
    resolved = {
        env: _resolve_credential(pytestconfig, env, ini) for env, ini in _CREDENTIAL_KEYS
    }
    missing = [env for env, value in resolved.items() if not value]
    if missing:
        raise pytest.UsageError(
            "Sift credentials missing: "
            + ", ".join(missing)
            + ". Set the environment variable(s) — pytest-dotenv loads them "
            "from a `.env` file automatically — or set the URIs via "
            "`sift_grpc_uri` / `sift_rest_uri` under `[tool.pytest.ini_options]` "
            "in pyproject.toml, or override the sift_client fixture in your "
            "conftest.py."
        )
    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=resolved["SIFT_API_KEY"],
            grpc_url=resolved["SIFT_GRPC_URI"],
            rest_url=resolved["SIFT_REST_URI"],
        )
    )


@pytest.fixture(scope="session", autouse=True)
def report_context(
    sift_client: SiftClient, request: pytest.FixtureRequest, pytestconfig: pytest.Config
) -> Generator[ReportContext | None, None, None]:
    """Create a report context for the session.

    The log file destination is controlled by ``--sift-test-results-log-file``.
    Defaults to a temp file when not set.

    When ``--sift-test-results-check-connection`` is passed, this fixture will no-op
    (yield None) if the Sift client has no connection to the server. That mode
    requires a ``client_has_connection`` fixture to be available in the session.
    """
    if _check_connection_enabled(pytestconfig) and not _has_sift_connection(request):
        yield None
        return
    yield from _report_context_impl(sift_client, request, pytestconfig=pytestconfig)


def _step_impl(
    report_context: ReportContext, request: pytest.FixtureRequest
) -> Generator[NewStep | None, None, None]:
    name = str(request.node.name)
    existing_docstring = request.node.obj.__doc__ or None
    with report_context.new_step(
        name=name, description=existing_docstring, assertion_as_fail_not_error=False
    ) as new_step:
        yield new_step
        if hasattr(request.node, "rep_call") and request.node.rep_call.excinfo:
            new_step.update_step_from_result(
                request.node.rep_call.excinfo,
                request.node.rep_call.excinfo.value,
                request.node.rep_call.excinfo.tb,
            )


@pytest.fixture(autouse=True)
def step(
    report_context: ReportContext | None,
    request: pytest.FixtureRequest,
) -> Generator[NewStep | None, None, None]:
    """Create an outer step for the function. No-ops when ``report_context`` is None."""
    if report_context is None:
        yield None
        return
    yield from _step_impl(report_context, request)


@pytest.fixture(scope="module", autouse=True)
def module_substep(
    report_context: ReportContext | None,
    request: pytest.FixtureRequest,
) -> Generator[NewStep | None, None, None]:
    """Create a step per module. No-ops when ``report_context`` is None."""
    if report_context is None:
        yield None
        return
    yield from _step_impl(report_context, request)


@pytest.fixture(scope="session")
def client_has_connection(sift_client):
    """Check if the SiftClient has a connection to the Sift server.

    Can be used to skip tests that require a connection to the Sift server, and is
    consulted by the Sift fixtures when ``--sift-test-results-check-connection`` is set.
    """
    try:
        sift_client.ping.ping()
        return True
    except Exception:
        return False
