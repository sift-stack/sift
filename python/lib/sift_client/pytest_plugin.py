"""Sift pytest plugin: records each test as a step in a Sift test report.

Load it from a project's ``conftest.py``::

    pytest_plugins = ["sift_client.pytest_plugin"]

This module holds only the plugin's public surface: the catchable warnings,
the session-state globals a conftest may read, the fixtures a project can
request or override, and pytest's hook entry points. The implementation
(settings registry, step stacks, report construction, terminal formatting)
lives under ``sift_client._internal.pytest_plugin``.
"""

from __future__ import annotations

from types import SimpleNamespace
from typing import TYPE_CHECKING, Any, Generator

import pytest

from sift_client import SiftClient, SiftConnectionConfig
from sift_client._internal.pytest_plugin.modes import (
    gate_enabled,
    is_disabled,
    is_offline,
    mode_label,
    sdk_version,
)
from sift_client._internal.pytest_plugin.options import (
    API_KEY_OPTION,
    APP_URL_OPTION,
    GRPC_URI_OPTION,
    OPEN_OPTION,
    REST_URI_OPTION,
    register_options,
    warn_on_unknown_env_vars,
    warn_on_unknown_toml_keys,
)
from sift_client._internal.pytest_plugin.report import (
    OFFLINE_DEFAULTS,
    build_disabled_client,
    finalize_after_teardown,
    report_context_impl,
    resolve_report_link,
    step_impl,
)
from sift_client._internal.pytest_plugin.steps import (
    build_hierarchy_chain,
    build_parametrize_path,
    drain_hierarchy_stack,
    drain_parametrize_stack,
    hierarchy_key,
    parametrize_path_key,
    reconcile_hierarchy,
    reconcile_parametrize,
)
from sift_client._internal.pytest_plugin.terminal import (
    maybe_open_report,
    write_disabled_summary,
    write_report_summary,
)
from sift_client.errors import SiftWarning
from sift_client.sift_types.test_report import TestStatus
from sift_client.util.test_results import ReportContext
from sift_client.util.test_results.context_manager import NewStep

__all__ = [
    "REPORT_CONTEXT",
    "SIFT_REPORT_ID_STASH_KEY",
    "SIFT_REPORT_URL_STASH_KEY",
    "SiftPytestPluginWarning",
    "SiftPytestStepDrainError",
    "SiftPytestStepDrainWarning",
    "client_has_connection",
    "report_context",
    "sift_client",
    "step",
    "NewStep",
    "ReportContext",
]


# ---------------------------------------------------------------------------
# Public warnings.
# ---------------------------------------------------------------------------


class SiftPytestPluginWarning(SiftWarning):
    """Base warning for issues raised by the Sift pytest plugin."""


class SiftPytestStepDrainWarning(SiftPytestPluginWarning):
    """A step's ``__exit__`` raised while the plugin was draining its stack.

    Surfaced at module-teardown or session-end so the drain can continue and
    pytest test outcomes stay unaffected; the underlying exception is included
    in the message for debugging.
    """


class SiftPytestStepDrainError(RuntimeError):
    """Raised when mid-session drain fails, signaling a likely upstream invariant break."""


# ---------------------------------------------------------------------------
# Public session state and stash keys.
# ---------------------------------------------------------------------------

REPORT_CONTEXT: Any = None

# Set at session end with the resolved (real) report id/URL when online and
# uploaded. Read from a project's conftest in a later hook (e.g.
# ``pytest_unconfigure``) to post the link, write a file, etc.
SIFT_REPORT_ID_STASH_KEY = pytest.StashKey[str]()
SIFT_REPORT_URL_STASH_KEY = pytest.StashKey[str]()


# ---------------------------------------------------------------------------
# Fixtures.
# ---------------------------------------------------------------------------


@pytest.fixture(scope="session")
def sift_client(pytestconfig: pytest.Config) -> SiftClient:
    """Default ``SiftClient`` resolved from environment variables and ini keys.

    Each credential is read from its environment variable first. The URIs
    (``SIFT_GRPC_URI``, ``SIFT_REST_URI``) additionally fall back to the
    ``sift_grpc_uri`` / ``sift_rest_uri`` ini keys, since they are stable
    per-org values that are safe to commit. ``SIFT_API_KEY`` is intentionally
    env-only; use ``pytest-dotenv`` (already a project dependency) to load
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
    if is_disabled(pytestconfig):
        return build_disabled_client()
    resolved = {
        "SIFT_API_KEY": API_KEY_OPTION.resolve(pytestconfig),
        "SIFT_GRPC_URI": GRPC_URI_OPTION.resolve(pytestconfig),
        "SIFT_REST_URI": REST_URI_OPTION.resolve(pytestconfig),
    }
    missing = [env for env, value in resolved.items() if not value]
    if missing and not is_offline(pytestconfig):
        raise pytest.UsageError(
            "Sift credentials missing: "
            + ", ".join(missing)
            + ". Set the environment variable(s) (pytest-dotenv loads them "
            "from a `.env` file automatically), or set the URIs under "
            "`sift_grpc_uri` / `sift_rest_uri` in `[tool.pytest.ini_options]` "
            "in pyproject.toml, or override the sift_client fixture in your "
            "conftest.py, or pass --sift-offline / --sift-disabled to run "
            "without contacting Sift."
        )
    for env in missing:
        resolved[env] = OFFLINE_DEFAULTS[env]
    # Web-app origin for the report link: the SIFT_APP_URL env var wins, then the
    # sift_app_url ini key, else host-based derivation in SiftClient.app_url.
    app_url = APP_URL_OPTION.resolve(pytestconfig)
    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=resolved["SIFT_API_KEY"] or "",
            grpc_url=resolved["SIFT_GRPC_URI"] or "",
            rest_url=resolved["SIFT_REST_URI"] or "",
            app_url=app_url or None,
        )
    )


@pytest.fixture(scope="session")
def client_has_connection(pytestconfig: pytest.Config, request: pytest.FixtureRequest) -> bool:
    """Verify the ``SiftClient`` can reach Sift via ``/ping``.

    Consulted at session start by ``report_context`` in online mode. A failed
    ping aborts the session via ``pytest.exit``. Override this fixture in your
    conftest to use a
    different reachability signal (e.g. a cached auth token) for environments
    where pinging is the wrong check. Returns ``False`` in ``--sift-disabled``
    mode without constructing a client.
    """
    if is_disabled(pytestconfig):
        return False
    sift_client = request.getfixturevalue("sift_client")
    sift_client.ping.ping()
    return True


def _set_report_context(
    contexts: Generator[ReportContext, None, None],
) -> Generator[ReportContext, None, None]:
    """Publish each yielded ReportContext to the module-level ``REPORT_CONTEXT``.

    ``report_context_impl`` stays pure: it builds and yields the context.
    Ownership of the reassignable global lives here so the terminal-summary and
    makereport hooks (which read ``REPORT_CONTEXT``) see it. The global is set
    after the context opens and before tests run, then the impl's ``finally``
    still drains the step stacks before the context exits.
    """
    global REPORT_CONTEXT
    for context in contexts:
        REPORT_CONTEXT = context
        yield context


@pytest.fixture(scope="session")
def report_context(
    request: pytest.FixtureRequest, pytestconfig: pytest.Config
) -> Generator[ReportContext, None, None]:
    """Lazy session-scoped Sift ReportContext.

    The fixture is no longer autouse; it's instantiated on the first call
    to ``request.getfixturevalue("report_context")``, which today happens
    inside the gated ``step``, ``_hierarchy_parents``, and
    ``_parametrize_parents`` fixtures. If every test in the session is
    excluded via the marker gate, this fixture is never resolved and no
    ReportContext (or teardown subprocess) is created.

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
      with ``pytest.exit`` and points at ``--sift-offline`` and
      ``--sift-disabled`` as escape hatches.

    The log-file destination is controlled by
    ``--sift-log-file``; defaults to a temp file when unset.
    """
    if is_disabled(pytestconfig):
        yield from _set_report_context(
            report_context_impl(build_disabled_client(), request, pytestconfig=pytestconfig)
        )
        return
    sift_client = request.getfixturevalue("sift_client")
    if not is_offline(pytestconfig):
        try:
            request.getfixturevalue("client_has_connection")
        except pytest.UsageError:
            raise
        except Exception as exc:
            grpc_config = getattr(getattr(sift_client, "grpc_client", None), "_config", None)
            grpc_url = getattr(grpc_config, "uri", "<unknown>")
            pytest.exit(
                f"Sift ping failed against {grpc_url}: {exc}. "
                "Pass --sift-offline to run without contacting Sift, or "
                "--sift-disabled to skip Sift entirely.",
                returncode=4,
            )
    yield from _set_report_context(
        report_context_impl(sift_client, request, pytestconfig=pytestconfig)
    )


@pytest.fixture(autouse=True)
def step(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
    _parametrize_parents: None,
) -> Generator[NewStep | None, None, None]:
    """Create an outer step for the function when the Sift gate is on.

    Resolves the gate via `gate_enabled`: the `sift_exclude` marker forces off,
    `sift_include` forces on, otherwise the `sift_autouse` ini default applies.
    When on, requests the session `report_context` lazily; the first gated test
    in the session triggers its creation, subsequent gated tests reuse it. In
    ``--sift-disabled`` mode the report context is backed by a
    ``SiftClient(_simulate=True)`` placeholder, so every write returns a
    synthesized response without contacting Sift.
    """
    if not gate_enabled(request.node, pytestconfig):
        yield None
        return
    rc = request.getfixturevalue("report_context")
    yield from step_impl(rc, request)


@pytest.fixture(autouse=True)
def _hierarchy_parents(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
) -> None:
    """Open/close hierarchy parent steps (packages, modules, classes) for the current item.

    Gated off when the item is excluded (avoids eager ``report_context`` setup);
    otherwise delegates to ``reconcile_hierarchy``, which diffs the item's
    ancestor chain against the open stack and opens/closes parents to match.
    """
    if not gate_enabled(request.node, pytestconfig):
        return
    reconcile_hierarchy(request, pytestconfig)


@pytest.fixture(autouse=True)
def _parametrize_parents(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
    _hierarchy_parents: None,
) -> None:
    """Open/close shared parametrize parent steps for the current item.

    Ordered after ``_hierarchy_parents`` so parametrize parents nest inside the
    hierarchy ones. Gated off when the item is excluded (so excluded items don't
    eagerly request ``report_context``); otherwise delegates to
    ``reconcile_parametrize``, which also no-ops when
    ``sift_parametrize_nesting=false``. Parents persist until a later test's
    chain pops them, or until ``pytest_sessionfinish`` drains the rest.
    """
    if not gate_enabled(request.node, pytestconfig):
        return
    reconcile_parametrize(request, pytestconfig)


# ---------------------------------------------------------------------------
# Hooks (in lifecycle fire order).
# ---------------------------------------------------------------------------


def pytest_addoption(parser: pytest.Parser) -> None:
    """Register every CLI flag and pytest ini key declared in ``PLUGIN_OPTIONS``."""
    register_options(parser)


def pytest_configure(config: pytest.Config) -> None:
    """Register the Sift gate markers and warn on unknown ``SIFT_*`` settings."""
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
    # Surface typos in env vars and [tool.sift...] keys at session start so a
    # silent no-op (env var that doesn't match anything, table key the loader
    # ignores) becomes visible. The registry is the source of truth for what's
    # known.
    warn_on_unknown_env_vars()
    warn_on_unknown_toml_keys(config)


def pytest_collection_modifyitems(config: pytest.Config, items: list[pytest.Item]) -> None:
    """Stash each item's class chain + parametrize path and cluster siblings.

    Sorts by ``(file_path, hierarchy_chain, parametrize_path)`` so sibling
    items under a shared parent (package, module, class, or parametrize axis)
    stay contiguous; otherwise a free function sorting between two class
    methods would tear down + re-open the class step, producing duplicate
    parents in the report tree.
    """
    for item in items:
        item.stash[hierarchy_key] = build_hierarchy_chain(item, config)
        item.stash[parametrize_path_key] = build_parametrize_path(item)
    # Use ``.get(...)`` defensively: a third-party hook may inject items after
    # our stashing loop runs, and we'd rather sort them at the tail than
    # KeyError out of collection.
    items.sort(
        key=lambda i: (
            str(i.path),
            tuple(identity for identity, _, _, _ in i.stash.get(hierarchy_key, ())),
            i.stash.get(parametrize_path_key, ()),
        )
    )


@pytest.hookimpl(tryfirst=True, hookwrapper=True)
def pytest_runtest_makereport(item: pytest.Item, call: pytest.CallInfo[Any]):
    """Capture per-phase reports and finalize step status after teardown.

    Stashes both ``rep_<when>`` (the ``CallInfo``, kept for pytest plugins that
    expect that conventional attribute) and ``_sift_phase_<when>`` (a
    ``SimpleNamespace(call, report)`` used by ``resolve_initial_status``). The
    collection-time skip path is strictly gated on ``_sift_step`` being unset
    so it does not duplicate steps the fixture already created.
    """
    outcome = yield
    report = outcome.get_result()
    setattr(item, "rep_" + report.when, call)
    setattr(item, "_sift_phase_" + report.when, SimpleNamespace(call=call, report=report))

    # Collection-time skip (``@pytest.mark.skip`` / ``skipif``): the autouse
    # ``step`` fixture never runs, so the hook is the only place that can
    # record a step. Presence of ``_sift_step`` is the "fixture ran" signal.
    if (
        REPORT_CONTEXT
        and report.when == "setup"
        and report.outcome == "skipped"
        and getattr(item, "_sift_step", None) is None
    ):
        with REPORT_CONTEXT.new_step(name=item.name) as inline_step:
            inline_step.current_step.update({"status": TestStatus.SKIPPED})

    if report.when == "teardown":
        finalize_after_teardown(item, report)


def pytest_sessionfinish(session: pytest.Session, exitstatus: int) -> None:
    """Drain any parent steps still open at session end (innermost first).

    Wrapped so a failure in the inner drain does not prevent the outer one
    from running. With ``module_substep`` removed, this is the sole place
    where hierarchy parents close; they persist across all tests and only
    drain when the session ends.
    """
    try:
        drain_parametrize_stack()
    finally:
        drain_hierarchy_stack()


def pytest_report_header(config: pytest.Config) -> str | None:
    """Emit a session-start header with the SDK version and active mode.

    Suppressed under ``-q`` (negative verbosity), matching how pytest hides its
    own platform/plugin header.
    """
    if config.get_verbosity() < 0:
        return None
    return f"Sift: sift-stack-py {sdk_version()} — {mode_label(config)} mode"


def pytest_terminal_summary(terminalreporter: Any, exitstatus: int, config: pytest.Config) -> None:
    """Emit a session-end Sift report summary, adapting per mode.

    The printed panel is suppressed under ``-q``, but programmatic side effects
    (stashing the report ref for ``conftest.py``, ``--sift-open-report``) still run so
    other plugins and CI steps can consume the result. The panel itself is
    rendered by ``write_report_summary``; this hook handles the side effects.
    """
    quiet = config.get_verbosity() < 0

    if is_disabled(config):
        if not quiet:
            write_disabled_summary(terminalreporter)
        return

    context = REPORT_CONTEXT
    if context is None:
        # No gated test ran, so no report context was created. Nothing to show.
        return

    offline = is_offline(config)
    # Resolve the link first so stashing and --sift-open-report run even under -q;
    # programmatic consumers don't care about verbosity.
    report_id, report_url = resolve_report_link(context, offline)
    if report_id:
        config.stash[SIFT_REPORT_ID_STASH_KEY] = report_id
    if report_url is not None:
        config.stash[SIFT_REPORT_URL_STASH_KEY] = report_url
        if OPEN_OPTION.resolve(config):
            maybe_open_report(report_url)

    if quiet:
        return

    write_report_summary(terminalreporter, context, config, report_id, report_url, offline)
