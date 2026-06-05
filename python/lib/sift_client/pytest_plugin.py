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
    finalize_parents,
    get_or_create_parent_chain,
    hierarchy_key,
    parametrize_path_key,
    release_finished_leaf,
    resolve_parent_chain_in_context,
    tally_expected_parents,
)
from sift_client._internal.pytest_plugin.terminal import (
    maybe_open_report,
    write_disabled_summary,
    write_report_summary,
)
from sift_client.errors import SiftWarning
from sift_client.sift_types.test_report import TestStatus

if TYPE_CHECKING:
    from sift_client.util.test_results import ReportContext
    from sift_client.util.test_results.context_manager import NewStep

__all__ = [
    "REPORT_CONTEXT",
    "SIFT_REPORT_ID_STASH_KEY",
    "SIFT_REPORT_URL_STASH_KEY",
    "SiftPytestPluginWarning",
    "SiftPytestStepDrainWarning",
    "client_has_connection",
    "report_context",
    "sift_client",
    "step",
]


# ---------------------------------------------------------------------------
# Public warnings.
# ---------------------------------------------------------------------------


class SiftPytestPluginWarning(SiftWarning):
    """Base warning for issues raised by the Sift pytest plugin."""


class SiftPytestStepDrainWarning(SiftPytestPluginWarning):
    """A parent step's ``__exit__`` raised while the plugin was closing it.

    Surfaced when a parent step is closed (early as its subtree finishes, or at
    session end) so the close can continue and pytest test outcomes stay
    unaffected; the underlying exception is included in the message for debugging.
    """


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
    inside the gated ``step`` and ``_sift_parents`` fixtures. If every test in
    the session is excluded via the marker gate, this fixture is never resolved
    and no ReportContext (or teardown subprocess) is created.

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
    _sift_parents: None,
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
def _sift_parents(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
) -> None:
    """Resolve (get-or-create) the report-tree parent for the current item.

    Builds the item's hierarchy (packages / modules / classes) and parametrize
    parents via ``get_or_create_parent_chain`` and stashes the innermost one on
    the node as ``_sift_parent`` for the ``step`` fixture to nest the leaf under.
    Parents are keyed by identity and reused across sibling items in any order, so
    no reordering of test items is needed.

    Gated off when the item is excluded so excluded items never eagerly create
    ``report_context`` (preserving its lazy, first-gated-test creation).
    """
    if not gate_enabled(request.node, pytestconfig):
        return
    request.node._sift_parent = get_or_create_parent_chain(request.node, pytestconfig, request)


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


def pytest_itemcollected(item: pytest.Item) -> None:
    """Cache each test item's hierarchy chain and parametrize path at collection.

    This is a per-item hook, not ``pytest_collection_modifyitems`` — the plugin
    never touches the ``items`` list or its order, so it cannot conflict with a
    user's (or another plugin's) collection-ordering hook. The report tree is
    built from an identity-keyed registry (see ``get_or_create_parent_chain``),
    so item order is irrelevant to nesting; ``pytest-randomly``,
    ``pytest-ordering``, and pytest's own fixture-scope reordering are all
    preserved untouched.

    The stash is a cache the autouse fixtures read back; both keys have an
    on-demand recompute fallback, so an item a later hook injects without going
    through this hook still resolves correctly.
    """
    item.stash[hierarchy_key] = build_hierarchy_chain(item, item.config)
    item.stash[parametrize_path_key] = build_parametrize_path(item)


def pytest_collection_finish(session: pytest.Session) -> None:
    """Tally each parent's descendant leaves so parents can close mid-session.

    Delegates to ``tally_expected_parents``; runs after deselection so the counts
    reflect only the selected, gated-in items. See ``release_finished_leaf``.
    """
    tally_expected_parents(session)


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
        # Nest the inline step under the same registry parents a running sibling
        # would use. The autouse ``_sift_parents`` fixture never ran for a
        # marker-skipped item, and the report-tree parents live off the step
        # stack, so without resolving the parent here the step lands at the
        # report root instead of under its module/class.
        parent_ns = resolve_parent_chain_in_context(item, item.config, REPORT_CONTEXT)
        parent_step = parent_ns.current_step if parent_ns is not None else None
        with REPORT_CONTEXT.new_step(name=item.name, parent=parent_step) as inline_step:
            inline_step.current_step.update({"status": TestStatus.SKIPPED})

    if report.when == "teardown":
        finalize_after_teardown(item, report)


def pytest_runtest_logfinish(nodeid: str, location: tuple[str, int | None, str]) -> None:
    """Close report-tree parents whose subtree finished with this item.

    Fires once per item (pass / fail / skip / error); delegates to
    ``release_finished_leaf``, which decrements the item's parents' remaining-leaf
    counts and closes any that reach zero — so containers resolve progressively
    rather than all at session end.
    """
    release_finished_leaf(nodeid)


def pytest_sessionfinish(session: pytest.Session, exitstatus: int) -> None:
    """Close any report-tree parents still open at session end (innermost first).

    Normally a no-op: ``report_context_impl`` finalizes the parents inside the
    ``ReportContext`` block so their updates reach the log before the import
    worker drains, and most parents already closed early as their subtrees
    finished. This is the idempotent backstop for anything still open.
    """
    finalize_parents()


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
