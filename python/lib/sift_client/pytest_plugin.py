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

_ENV_VARS = ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI")


def pytest_addoption(parser: pytest.Parser) -> None:
    """Register Sift-specific command-line options."""
    group = parser.getgroup("sift", description="Sift test results")
    group.addoption(
        "--sift-offline",
        action="store_true",
        default=False,
        help="Run without contacting Sift. All create/update calls are written "
        "to a JSONL log file for later replay via `import-test-result-log`. "
        "No session-start ping is attempted.",
    )
    group.addoption(
        "--sift-log-file",
        default=None,
        help="Path to write the Sift JSONL log file. In online mode this acts "
        "as a write-through backup; in offline mode it is the sole sink. "
        "When unset, a temporary file is created and its path logged.",
    )
    group.addoption(
        "--no-sift-log-file",
        action="store_true",
        default=False,
        help="Disable the JSONL log file (online mode only). Create/update "
        "calls run synchronously against the API instead of being deferred "
        "through the import worker.",
    )
    group.addoption(
        "--sift-no-git-metadata",
        action="store_true",
        default=False,
        help="Exclude git repo/branch/commit from report metadata. Git metadata "
        "is captured by default when run from inside a git repository.",
    )


def _is_offline(pytestconfig: pytest.Config | None) -> bool:
    if pytestconfig is None:
        return False
    return bool(pytestconfig.getoption("sift_offline", default=False))


def _resolve_log_file(pytestconfig: pytest.Config) -> str | Path | bool | None:
    """Resolve the log_file argument for ReportContext from CLI options.

    Returns ``True`` to request a temp file, a ``Path`` for a pinned location,
    or ``None`` to disable the log file entirely.
    """
    if pytestconfig.getoption("no_sift_log_file", default=False):
        if _is_offline(pytestconfig):
            raise pytest.UsageError(
                "--no-sift-log-file is incompatible with --sift-offline; offline "
                "mode requires a log file. Pin one with --sift-log-file=<path> "
                "or drop --no-sift-log-file to use a temp file."
            )
        return None
    raw = pytestconfig.getoption("sift_log_file", default=None)
    if raw is None:
        return True
    return Path(raw)


def _missing_env_vars() -> list[str]:
    return [name for name in _ENV_VARS if not os.getenv(name)]


@pytest.hookimpl(tryfirst=True, hookwrapper=True)
def pytest_runtest_makereport(item: pytest.Item, call: pytest.CallInfo[Any]):
    """You should import this hook to capture any AssertionErrors that occur during the test. If not included, any assert failures in a test will not automatically fail the step."""
    outcome = yield
    report = outcome.get_result()
    if report.outcome == "skipped":
        # Skipped steps won't invoke the method/fixtures at all, so we need to manually record a step.
        if REPORT_CONTEXT:
            with REPORT_CONTEXT.new_step(name=item.name) as new_step:
                new_step.current_step.update({"status": TestStatus.SKIPPED})
    setattr(item, "rep_" + report.when, call)


@pytest.fixture(scope="session")
def sift_client(pytestconfig: pytest.Config) -> SiftClient:
    """Default ``SiftClient`` resolved from environment variables.

    Reads ``SIFT_API_KEY``, ``SIFT_GRPC_URI``, and ``SIFT_REST_URI``. Projects
    that need custom construction (TLS toggles, custom timeouts, etc.) can
    override this fixture by defining their own ``sift_client`` in their
    ``conftest.py``; pytest fixture resolution prefers the local definition.

    Missing environment variables raise ``pytest.UsageError`` naming the
    variables so the failure is actionable. In ``--sift-offline`` mode the
    check is skipped and placeholder credentials are used; nothing is ever
    sent to Sift, so the values don't matter.
    """
    if _is_offline(pytestconfig):
        return SiftClient(
            connection_config=SiftConnectionConfig(
                api_key=os.getenv("SIFT_API_KEY") or "offline",
                grpc_url=os.getenv("SIFT_GRPC_URI") or "offline.invalid:0",
                rest_url=os.getenv("SIFT_REST_URI") or "http://offline.invalid",
            )
        )
    missing = _missing_env_vars()
    if missing:
        raise pytest.UsageError(
            "Sift environment variables not set: "
            + ", ".join(missing)
            + ". Set them, or override the `sift_client` fixture in your conftest, "
            "or pass --sift-offline to run without contacting Sift."
        )
    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=os.getenv("SIFT_API_KEY"),
            grpc_url=os.getenv("SIFT_GRPC_URI"),
            rest_url=os.getenv("SIFT_REST_URI"),
        )
    )


@pytest.fixture(scope="session")
def client_has_connection(sift_client: SiftClient) -> bool:
    """Verify the ``SiftClient`` can reach Sift via ``/ping``.

    Consulted at session start when running in online mode. Override this
    fixture in your conftest to use a different reachability signal (e.g. a
    cached auth token) for environments where pinging is the wrong check.
    """
    sift_client.ping.ping()
    return True


def _report_context_impl(
    sift_client: SiftClient,
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
) -> Generator[ReportContext, None, None]:
    args = request.config.invocation_params.args
    test_path = Path(args[0]) if args else None
    if test_path is not None and test_path.exists():
        base_name = test_path.name
        test_case: Path | str = test_path
    else:
        base_name = "pytest " + " ".join(args) if args else "pytest"
        test_case = base_name
    include_git_metadata = not pytestconfig.getoption("sift_no_git_metadata", default=False)
    log_file = _resolve_log_file(pytestconfig)
    offline = _is_offline(pytestconfig)
    with ReportContext(
        sift_client,
        name=f"{base_name} {datetime.now(timezone.utc).isoformat()}",
        test_case=str(test_case),
        log_file=log_file,
        include_git_metadata=include_git_metadata,
        offline=offline,
    ) as context:
        # Set a global so we can access this in pytest hooks.
        global REPORT_CONTEXT
        REPORT_CONTEXT = context
        yield context


@pytest.fixture(scope="session", autouse=True)
def report_context(
    sift_client: SiftClient, request: pytest.FixtureRequest, pytestconfig: pytest.Config
) -> Generator[ReportContext, None, None]:
    """Create a report context for the session.

    In online mode (default) the plugin verifies connectivity to Sift via the
    ``client_has_connection`` fixture before creating the report; a failed
    ping aborts the session with ``pytest.UsageError``. Pass ``--sift-offline``
    to skip the ping and route all writes through the JSONL log file.

    The log-file destination is controlled by ``--sift-log-file`` (a path),
    ``--no-sift-log-file`` (disable, online only), or left unset for a temp file.
    """
    if not _is_offline(pytestconfig):
        try:
            request.getfixturevalue("client_has_connection")
        except pytest.UsageError:
            raise
        except Exception as exc:
            grpc_url = getattr(
                getattr(sift_client, "grpc_client", None), "_config", None
            )
            grpc_url = getattr(grpc_url, "uri", "<unknown>")
            raise pytest.UsageError(
                f"Sift ping failed against {grpc_url}: {exc}. "
                "Pass --sift-offline to run without contacting Sift."
            ) from exc
    yield from _report_context_impl(sift_client, request, pytestconfig)


def _step_impl(
    report_context: ReportContext, request: pytest.FixtureRequest
) -> Generator[NewStep, None, None]:
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
    report_context: ReportContext,
    request: pytest.FixtureRequest,
) -> Generator[NewStep, None, None]:
    """Create an outer step for each test function."""
    yield from _step_impl(report_context, request)


@pytest.fixture(scope="module", autouse=True)
def module_substep(
    report_context: ReportContext,
    request: pytest.FixtureRequest,
) -> Generator[NewStep, None, None]:
    """Create a step per module."""
    yield from _step_impl(report_context, request)
