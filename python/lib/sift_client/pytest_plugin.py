from __future__ import annotations

import os
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING, Any, Generator

import pytest

from sift_client import SiftClient, SiftConnectionConfig
from sift_client.sift_types.test_report import TestStatus
from sift_client.util.test_results import ReportContext

if TYPE_CHECKING:
    from sift_client.util.test_results.context_manager import NewStep

REPORT_CONTEXT: Any = None

_PARAMETRIZE_PATH_KEY = pytest.StashKey[tuple[str, ...]]()
# Each frame: (path_key, open step). Frames are shared across sibling test items
# and drained at module-substep teardown / session end.
_PARAMETRIZE_STACK: list[tuple[str, Any]] = []


def _drain_parametrize_stack() -> None:
    while _PARAMETRIZE_STACK:
        _, ns = _PARAMETRIZE_STACK.pop()
        ns.__exit__(None, None, None)


def _build_parametrize_path(item: pytest.Item) -> tuple[str, ...]:
    """Outer-to-inner step display names for a parametrized item.

    Pytest stores ``callspec.params`` with the BOTTOM decorator's axis first;
    the Sift step tree treats the TOP decorator as outermost, so we reverse.
    """
    callspec = getattr(item, "callspec", None)
    if callspec is None or not callspec.params:
        return ()
    originalname = getattr(item, "originalname", item.name)
    frames: list[str] = [originalname]
    for name, value in reversed(callspec.params.items()):
        frames.append(f"{name}={value!r}")
    return tuple(frames)


@dataclass(frozen=True)
class _Option:
    """A single Sift plugin setting, registered as a CLI flag and/or an ini key.

    ``ini_name`` is used as both the ini key and the CLI ``dest``, so a value
    set either way lands on the same config slot. ``cli_flag=None`` makes the
    option ini-only (e.g. the URI fallbacks).
    """

    ini_name: str
    ini_help: str
    cli_flag: str | None = None
    cli_help: str | None = None
    action: str | None = None
    ini_type: str | None = None
    ini_default: Any = None


_LOG_FILE = _Option(
    cli_flag="--sift-log-file",
    ini_name="sift_log_file",
    cli_help="Path to write the Sift test result log file. "
    "Use 'true' (default) to auto-create a temp file, "
    "False, 'false', or 'none' to disable logging, "
    "or a file path to write to a specific location.",
    ini_help="Default value for --sift-log-file. Same values accepted as "
    "the CLI flag (path, 'true', 'false', 'none').",
)

_GIT_METADATA = _Option(
    cli_flag="--no-sift-git-metadata",
    ini_name="sift_git_metadata",
    action="store_false",
    cli_help="Exclude git metadata from the Sift test results. "
    "Git metadata (repo, branch, commit) is included by default.",
    ini_help="Include git repo/branch/commit in the report (true/false). "
    "Defaults to true. The --no-sift-git-metadata CLI flag overrides "
    "this when passed.",
    ini_type="bool",
    ini_default=True,
)

_OFFLINE = _Option(
    cli_flag="--sift-offline",
    ini_name="sift_offline",
    action="store_true",
    cli_help="Run without contacting Sift. All create/update calls are written "
    "to a JSONL log file for later replay via `import-test-result-log`. "
    "No session-start ping is attempted.",
    ini_help="When true, run in offline mode (same effect as --sift-offline). Defaults to false.",
    ini_type="bool",
    ini_default=False,
)

_DISABLED = _Option(
    cli_flag="--sift-disabled",
    ini_name="sift_disabled",
    action="store_true",
    cli_help="Disable Sift integration entirely. Nothing contacts the API "
    "and no log file is written. `step.measure(...)` still returns real "
    "pass/fail booleans. Returned entities expose `is_simulated == True`. "
    "Also honored via the `SIFT_DISABLED` env var. Supersedes every other "
    "flag.",
    ini_help="When true, run in disabled mode (same effect as --sift-disabled). "
    "Also honored via the SIFT_DISABLED env var. Supersedes every other "
    "setting. Defaults to false.",
    ini_type="bool",
    ini_default=False,
)

_GRPC_URI = _Option(
    ini_name="sift_grpc_uri",
    ini_help="Sift gRPC endpoint URI. The default `sift_client` fixture "
    "prefers the SIFT_GRPC_URI environment variable and falls back to "
    "this ini value.",
)

_REST_URI = _Option(
    ini_name="sift_rest_uri",
    ini_help="Sift REST endpoint URI. The default `sift_client` fixture "
    "prefers the SIFT_REST_URI environment variable and falls back to "
    "this ini value.",
)

_AUTOUSE = _Option(
    ini_name="sift_autouse",
    ini_help="Default for the Sift autouse fixtures (report_context, step, "
    "module_substep). When true (default), tests are included unless marked "
    "with @pytest.mark.sift_exclude. When false, tests are skipped unless "
    "marked with @pytest.mark.sift_include. Bulk-apply markers in a "
    "directory's conftest via `pytest_collection_modifyitems`.",
    ini_type="bool",
    ini_default=True,
)

_OPTIONS: tuple[_Option, ...] = (
    _LOG_FILE,
    _GIT_METADATA,
    _OFFLINE,
    _DISABLED,
    _GRPC_URI,
    _REST_URI,
    _AUTOUSE,
)


def pytest_addoption(parser: pytest.Parser) -> None:
    """Register Sift-specific command-line options and ini keys.

    Each option can be set on the command line or under ``[tool.pytest.ini_options]``
    in ``pyproject.toml`` (or ``[pytest]`` in ``pytest.ini``). CLI values take
    precedence over ini values, which take precedence over the built-in default.
    """
    group = parser.getgroup("sift", description="Sift test results")
    for opt in _OPTIONS:
        if opt.cli_flag is not None:
            cli_kwargs: dict[str, Any] = {
                "dest": opt.ini_name,
                "default": None,
                "help": opt.cli_help,
            }
            if opt.action is not None:
                cli_kwargs["action"] = opt.action
            group.addoption(opt.cli_flag, **cli_kwargs)

        ini_kwargs: dict[str, Any] = {"help": opt.ini_help, "default": opt.ini_default}
        if opt.ini_type is not None:
            ini_kwargs["type"] = opt.ini_type
        parser.addini(opt.ini_name, **ini_kwargs)


def pytest_configure(config: pytest.Config) -> None:
    """Register the Sift gate markers so they show up in `pytest --markers`."""
    config.addinivalue_line(
        "markers",
        "sift_include: force the Sift autouse fixtures to activate for this test "
        "regardless of the `sift_autouse` ini default.",
    )
    config.addinivalue_line(
        "markers",
        "sift_exclude: force the Sift autouse fixtures to skip this test "
        "regardless of the `sift_autouse` ini default.",
    )


def pytest_collection_modifyitems(config: pytest.Config, items: list[pytest.Item]) -> None:
    """Stash each item's parametrize path and cluster siblings by shared prefix."""
    for item in items:
        item.stash[_PARAMETRIZE_PATH_KEY] = _build_parametrize_path(item)
    items.sort(key=lambda i: i.stash[_PARAMETRIZE_PATH_KEY])


def pytest_sessionfinish(session: pytest.Session, exitstatus: int) -> None:
    """Drain any parametrize parents still open (e.g. when module_substep was gated off)."""
    _drain_parametrize_stack()


def _is_offline(pytestconfig: pytest.Config | None) -> bool:
    return bool(_option_or_ini(pytestconfig, _OFFLINE))


def _is_disabled(pytestconfig: pytest.Config | None) -> bool:
    if bool(_option_or_ini(pytestconfig, _DISABLED)):
        return True
    return os.getenv("SIFT_DISABLED", "").lower() in ("1", "true", "yes")


def _sift_enabled_for(node: pytest.Item | pytest.Collector, default: bool) -> bool:
    """Resolve the Sift gate for a node: sift_exclude > sift_include > default.

    `get_closest_marker` walks the node hierarchy upward, so markers applied
    at any level (function, class, module, package, session) are honored.
    """
    if node.get_closest_marker("sift_exclude"):
        return False
    if node.get_closest_marker("sift_include"):
        return True
    return default


def _module_has_included_tests(request: pytest.FixtureRequest, default: bool) -> bool:
    """True when at least one test in `request`'s module is gated on.

    Used by the module-scoped `module_substep` fixture to decide whether to
    activate without triggering `report_context` creation for modules where
    every test is excluded.
    """
    module_path = request.path
    for item in request.session.items:
        if item.path != module_path:
            continue
        if _sift_enabled_for(item, default):
            return True
    return False


def _option_or_ini(pytestconfig: pytest.Config | None, opt: _Option) -> Any:
    """Resolve a Sift plugin setting from CLI > ini > None.

    The ``addoption`` registrations use ``default=None`` so we can tell whether
    the CLI was actually used. When the CLI didn't set a value, fall back to
    the matching ``addini`` key.
    """
    if pytestconfig is None:
        return None
    cli = pytestconfig.getoption(opt.ini_name, default=None)
    if cli is not None:
        return cli
    try:
        return pytestconfig.getini(opt.ini_name)
    except (KeyError, ValueError):
        return None


def _resolve_log_file(pytestconfig: pytest.Config | None) -> str | Path | bool | None:
    """Determine log_file value from CLI flag or ini key.

    Three signal types arrive here:

    * ``None`` — unset; nothing was passed on the CLI and the ini key is
      absent. Treat as the default "use a temp file."
    * Python ``False`` — an explicit disable, typically set in a conftest via
      ``config.option.sift_log_file = False``. Return ``None`` so
      the rest of the pipeline knows to skip logging entirely.
    * A string (from CLI or ini) — interpret ``"true"`` / ``"1"`` as the temp
      file default, ``"false"`` / ``"none"`` as disable, anything else as a
      file path.

    Rejects ``--sift-log-file=none`` combined with ``--sift-offline`` since
    offline mode needs the log file as its sole sink.
    """
    raw = _option_or_ini(pytestconfig, _LOG_FILE)
    disabled = raw is False or (isinstance(raw, str) and raw.lower() in ("false", "none"))
    if disabled and _is_offline(pytestconfig):
        raise pytest.UsageError(
            "--sift-log-file=none is incompatible with --sift-offline; offline "
            "mode requires a log file. Pin one with --sift-log-file=<path>, or "
            "drop --sift-log-file=none to use a temp file."
        )
    if raw is False:
        return None
    if not raw:
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
) -> Generator[ReportContext, None, None]:
    args = request.config.invocation_params.args
    test_path = Path(args[0]) if args else None
    if test_path is not None and test_path.exists():
        base_name = test_path.name
        test_case: Path | str = test_path
    else:
        base_name = "pytest " + " ".join(args) if args else "pytest"
        test_case = base_name
    # Mode → ReportContext flags:
    #   online (default): log_file=<temp or user path>, replay_log_file=True
    #   --sift-offline:   log_file=<temp or user path>, replay_log_file=False
    #   --sift-disabled:  log_file=False,               replay_log_file=False
    disabled = sift_client._simulate
    offline = False if disabled else _is_offline(pytestconfig)
    log_file: str | Path | bool | None = False if disabled else _resolve_log_file(pytestconfig)
    git_metadata = _option_or_ini(pytestconfig, _GIT_METADATA)
    include_git_metadata = True if git_metadata is None else bool(git_metadata)
    with ReportContext(
        sift_client,
        name=f"{base_name} {datetime.now(timezone.utc).isoformat()}",
        test_case=str(test_case),
        log_file=log_file,
        include_git_metadata=include_git_metadata,
        replay_log_file=not (disabled or offline),
    ) as context:
        global REPORT_CONTEXT
        REPORT_CONTEXT = context
        yield context


_CREDENTIAL_KEYS: tuple[tuple[str, _Option | None], ...] = (
    ("SIFT_API_KEY", None),  # env-only; never read from ini to keep secrets out of source control.
    ("SIFT_GRPC_URI", _GRPC_URI),
    ("SIFT_REST_URI", _REST_URI),
)

# Placeholder credentials used in --sift-offline mode when env/ini values
# are missing. Offline mode never makes network calls, so the values are
# only syntactically required by SiftConnectionConfig.
_OFFLINE_DEFAULTS = {
    "SIFT_API_KEY": "offline",
    "SIFT_GRPC_URI": "offline.invalid:0",
    "SIFT_REST_URI": "http://offline.invalid",
}


def _build_disabled_client() -> SiftClient:
    """Construct a SiftClient for ``--sift-disabled`` mode.

    Tagged with ``_simulate=True`` so test-results writes short-circuit through
    the existing low-level simulate path without contacting Sift. The URLs are
    syntactically valid but unreachable; nothing dials them.
    """
    client = SiftClient(
        connection_config=SiftConnectionConfig(
            api_key="disabled",
            grpc_url="disabled.invalid:0",
            rest_url="http://disabled.invalid",
        )
    )
    client._simulate = True
    return client


def _resolve_credential(
    pytestconfig: pytest.Config | None, env_name: str, opt: _Option | None
) -> str | None:
    """Resolve a Sift credential: env var first, then ini key (if registered), else None."""
    env_value = os.getenv(env_name)
    if env_value:
        return env_value
    if opt is None or pytestconfig is None:
        return None
    ini_value = pytestconfig.getini(opt.ini_name)
    return ini_value if isinstance(ini_value, str) and ini_value else None


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

    In ``--sift-offline`` mode the missing-credential check is relaxed:
    real env vars and ini values still win when set (so the client is
    constructible against a real backend even though no calls are made), but
    anything still missing is filled with a placeholder. In ``--sift-disabled``
    mode the credential resolution is skipped entirely and placeholders are
    always used.
    """
    if _is_disabled(pytestconfig):
        return _build_disabled_client()
    resolved = {env: _resolve_credential(pytestconfig, env, opt) for env, opt in _CREDENTIAL_KEYS}
    missing = [env for env, value in resolved.items() if not value]
    if missing and not _is_offline(pytestconfig):
        raise pytest.UsageError(
            "Sift credentials missing: "
            + ", ".join(missing)
            + ". Set the environment variable(s) — pytest-dotenv loads them "
            "from a `.env` file automatically — or set the URIs via "
            "`sift_grpc_uri` / `sift_rest_uri` under `[tool.pytest.ini_options]` "
            "in pyproject.toml, or override the sift_client fixture in your "
            "conftest.py, or pass --sift-offline / --sift-disabled to run "
            "without contacting Sift."
        )
    for env in missing:
        resolved[env] = _OFFLINE_DEFAULTS[env]
    # `or ""` is unreachable in practice since the `missing` check above guarantees
    # non-None values
    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=resolved.get("SIFT_API_KEY") or "",
            grpc_url=resolved.get("SIFT_GRPC_URI") or "",
            rest_url=resolved.get("SIFT_REST_URI") or "",
        )
    )


@pytest.fixture(scope="session")
def report_context(
    request: pytest.FixtureRequest, pytestconfig: pytest.Config
) -> Generator[ReportContext, None, None]:
    """Lazy session-scoped Sift ReportContext.

    The fixture is no longer autouse; it's instantiated on the first call
    to ``request.getfixturevalue("report_context")``, which today happens
    inside the gated ``step`` and ``module_substep`` fixtures. If every
    test in the session is excluded via the marker gate, this fixture is
    never resolved and no ReportContext (or teardown subprocess) is created.

    What gets yielded depends on the mode:

    * ``--sift-disabled``: a real ``ReportContext`` against a placeholder
      ``SiftClient`` with ``_simulate=True``. Every test-results write
      returns a synthesized response without contacting Sift; no log file
      is written; the replay subprocess never spawns. Test code that calls
      ``step.measure(...)`` keeps working because bounds are evaluated as
      usual and routed through the simulate path.
    * ``--sift-offline``: a real ReportContext, but the session-start ping
      is skipped, all create/update calls go to the JSONL log file, and
      the import-test-result-log replay subprocess is not spawned at
      session end.
    * default (online): verify connectivity via ``client_has_connection``
      before constructing the context. A failed ping aborts the session
      with ``pytest.UsageError`` and points at ``--sift-offline`` and
      ``--sift-disabled`` as escape hatches.

    The log-file destination is controlled by
    ``--sift-log-file``; defaults to a temp file when unset.
    """
    if _is_disabled(pytestconfig):
        yield from _report_context_impl(
            _build_disabled_client(), request, pytestconfig=pytestconfig
        )
        return
    sift_client = request.getfixturevalue("sift_client")
    if not _is_offline(pytestconfig):
        try:
            request.getfixturevalue("client_has_connection")
        except pytest.UsageError:
            raise
        except Exception as exc:
            grpc_config = getattr(getattr(sift_client, "grpc_client", None), "_config", None)
            grpc_url = getattr(grpc_config, "uri", "<unknown>")
            raise pytest.UsageError(
                f"Sift ping failed against {grpc_url}: {exc}. "
                "Pass --sift-offline to run without contacting Sift, or "
                "--sift-disabled to skip Sift entirely."
            ) from exc
    yield from _report_context_impl(sift_client, request, pytestconfig=pytestconfig)


def _step_impl(
    report_context: ReportContext, request: pytest.FixtureRequest
) -> Generator[NewStep, None, None]:
    node = request.node
    # Items get a parametrize path stashed in ``pytest_collection_modifyitems``;
    # modules/other nodes fall back to their node name. The leaf frame
    # (``path[-1]``) is the test-specific display name — parents are opened
    # by ``_parametrize_parents``.
    path = node.stash.get(_PARAMETRIZE_PATH_KEY, ())
    name = path[-1] if path else str(node.name)
    existing_docstring = node.obj.__doc__ or None
    with report_context.new_step(
        name=name, description=existing_docstring, assertion_as_fail_not_error=False
    ) as new_step:
        yield new_step
        if hasattr(node, "rep_call") and node.rep_call.excinfo:
            new_step.update_step_from_result(
                node.rep_call.excinfo,
                node.rep_call.excinfo.value,
                node.rep_call.excinfo.tb,
            )


@pytest.fixture(autouse=True)
def _parametrize_parents(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
) -> None:
    """Open/close shared parametrize parent steps for the current item.

    Diffs the item's desired parametrize path against the open stack: pops the
    stale tail, then opens new parents (everything except the innermost frame —
    the ``step`` fixture creates that as the leaf). Parents persist across
    sibling items so a tree like ``test_x[a=1]`` / ``test_x[a=2]`` shares one
    ``test_x`` container.

    Gated off when the current item is excluded so that excluded items don't
    eagerly request ``report_context`` (which would defeat its lazy creation).
    Any parents still open at the end of a module are drained by
    ``module_substep`` teardown; anything left at session end is drained by
    ``pytest_sessionfinish``.
    """
    default = bool(_option_or_ini(pytestconfig, _AUTOUSE))
    if not _sift_enabled_for(request.node, default):
        return None
    desired = request.node.stash.get(_PARAMETRIZE_PATH_KEY, ())
    parents = desired[:-1]
    common = 0
    while (
        common < len(_PARAMETRIZE_STACK)
        and common < len(parents)
        and _PARAMETRIZE_STACK[common][0] == parents[common]
    ):
        common += 1
    while len(_PARAMETRIZE_STACK) > common:
        _, ns = _PARAMETRIZE_STACK.pop()
        ns.__exit__(None, None, None)
    if not parents[common:]:
        return None
    rc = request.getfixturevalue("report_context")
    for display in parents[common:]:
        ns = rc.new_step(name=display, assertion_as_fail_not_error=False)
        ns.__enter__()
        _PARAMETRIZE_STACK.append((display, ns))
    return None


@pytest.fixture(autouse=True)
def step(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
    _parametrize_parents: None,
) -> Generator[NewStep | None, None, None]:
    """Create an outer step for the function when the Sift gate is on.

    Resolves the gate via `_sift_enabled_for(request.node, ini_default)`:
    `sift_exclude` marker forces off, `sift_include` forces on, otherwise the
    `sift_autouse` ini default applies. When on, requests the
    session `report_context` lazily — the first gated test in the session
    triggers its creation, subsequent gated tests reuse it. In
    ``--sift-disabled`` mode the report context is backed by a
    ``SiftClient(_simulate=True)`` placeholder, so every write returns a
    synthesized response without contacting Sift.
    """
    default = bool(_option_or_ini(pytestconfig, _AUTOUSE))
    if not _sift_enabled_for(request.node, default):
        yield None
        return
    rc = request.getfixturevalue("report_context")
    yield from _step_impl(rc, request)


@pytest.fixture(scope="module", autouse=True)
def module_substep(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
) -> Generator[NewStep | None, None, None]:
    """Create a per-module step when at least one test in the module is gated on.

    Inspects the module's collected items rather than gating on a single marker,
    so a module with mixed inclusion/exclusion still produces the module-level
    step (individual `step` fixtures then decide per-test). When every test in
    the module is excluded, the substep is skipped without requesting
    `report_context`.
    """
    default = bool(_option_or_ini(pytestconfig, _AUTOUSE))
    if not _module_has_included_tests(request, default):
        yield None
        return
    rc = request.getfixturevalue("report_context")
    gen = _step_impl(rc, request)
    new_step = next(gen)
    try:
        yield new_step
    finally:
        # Drain parametrize parents nested under this module step before it
        # exits — ReportContext.exit_step asserts the module step is the top.
        _drain_parametrize_stack()
        try:
            next(gen)
        except StopIteration:
            pass


@pytest.fixture(scope="session")
def client_has_connection(pytestconfig: pytest.Config, request: pytest.FixtureRequest) -> bool:
    """Verify the ``SiftClient`` can reach Sift via ``/ping``.

    Consulted at session start by ``report_context`` in online mode. A failed
    ping raises through ``report_context`` and aborts the session with
    ``pytest.UsageError``. Override this fixture in your conftest to use a
    different reachability signal (e.g. a cached auth token) for environments
    where pinging is the wrong check. Returns ``False`` in ``--sift-disabled``
    mode without constructing a client.
    """
    if _is_disabled(pytestconfig):
        return False
    sift_client = request.getfixturevalue("sift_client")
    sift_client.ping.ping()
    return True
