from __future__ import annotations

import os
import warnings
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from types import SimpleNamespace
from typing import TYPE_CHECKING, Any, Generator, Tuple

import pytest

from sift_client import SiftClient, SiftConnectionConfig
from sift_client._internal.pyproject_config import load_tool_sift
from sift_client.errors import SiftWarning
from sift_client.sift_types.test_report import ErrorInfo, TestStatus
from sift_client.util.test_results import ReportContext
from sift_client.util.test_results.context_manager import (
    _git_metadata,
    _quiet_fork_stderr,
    format_assertion_message,
    format_truncated_traceback,
)


class SiftPytestPluginWarning(SiftWarning):
    """Base warning for issues raised by the Sift pytest plugin."""


class SiftPytestStepDrainWarning(SiftPytestPluginWarning):
    """A step's ``__exit__`` raised while the plugin was draining its stack.

    Surfaced at module-teardown or session-end so the drain can continue and
    pytest test outcomes stay unaffected; the underlying exception is included
    in the message for debugging.
    """


class SiftPytestStepDrainError(RuntimeError):
    """Raised when mid-session drain fails — signals a likely upstream invariant break."""


if TYPE_CHECKING:
    from sift_client.util.test_results.context_manager import NewStep

REPORT_CONTEXT: Any = None

# Set at session end with the resolved (real) report id/URL when online and
# uploaded. Read from a project's conftest in a later hook (e.g.
# ``pytest_unconfigure``) to post the link, write a file, etc.
SIFT_REPORT_ID_STASH_KEY = pytest.StashKey[str]()
SIFT_REPORT_URL_STASH_KEY = pytest.StashKey[str]()

_STASH_MISSING = object()

_PARAMETRIZE_PATH_KEY = pytest.StashKey[Tuple[str, ...]]()
# Each frame: (path_key, open step). Frames are shared across sibling test items
# and drained at session end.
_PARAMETRIZE_STACK: list[tuple[str, Any]] = []

_HIERARCHY_KEY = pytest.StashKey[Tuple[Tuple[str, str, "str | None", bool], ...]]()
# Outer-to-inner frames for the item's collection-tree ancestors. Each chain
# entry is ``(identity, name, doc, rendered)``:
#   - ``identity``: a globally-unique key (``node.nodeid``) used for diff
#     comparison. Two ancestors at the same depth with the same display name
#     but reached via different paths (e.g., ``proj_a/utils`` and
#     ``proj_b/utils`` in a monorepo) get distinct identities, so they never
#     silently merge in the diff.
#   - ``name``: the human-readable step name used when ``rendered`` opens the
#     Sift step.
#   - ``doc``: docstring used for the step description if rendered.
#   - ``rendered``: True iff the corresponding ``sift_*_step`` ini flag is on.
#     Non-rendered frames participate in the diff but do not call
#     ``rc.new_step(...)`` — they appear with ``ns=None`` in the stack.
#
# Stack entries: ``(identity, name, open_step_or_None)``. Frames are shared
# across sibling test items and drained at session end. Drained AFTER
# _PARAMETRIZE_STACK since parametrize parents nest inside hierarchy parents.
_HIERARCHY_STACK: list[tuple[str, str, Any]] = []


def _drain_step_stack(stack: list, *, swallow_errors: bool = True) -> None:
    """Pop and close every frame.

    With ``swallow_errors=True`` (default, used at teardown / session end),
    per-frame failures are surfaced as ``SiftPytestStepDrainWarning`` so a
    single misbehaving ``__exit__`` can't block the rest of the stack from
    cleaning up or cascade out of pytest's finalizer chain.

    With ``swallow_errors=False`` (mid-session, when a class transition forces
    parametrize parents to close), the stack is still fully drained but the
    first per-frame exception is re-raised at the end as a
    ``SiftPytestStepDrainError`` so a real upstream invariant violation
    surfaces as a test error instead of a silenceable warning.
    """
    errors: list[tuple[str, BaseException]] = []
    while stack:
        entry = stack.pop()
        # Tolerate either ``(name, ns)`` (parametrize stack) or
        # ``(identity, name, ns)`` (hierarchy stack) entries.
        name, ns = entry[-2], entry[-1]
        if ns is None:
            # Non-rendered diff-only frame (e.g. a Package frame when
            # ``sift_package_step=false``); nothing to close.
            continue
        try:
            ns.__exit__(None, None, None)
        except Exception as exc:
            if swallow_errors:
                warnings.warn(
                    f"Sift plugin: closing step {name!r} during drain raised "
                    f"{type(exc).__name__}: {exc}",
                    SiftPytestStepDrainWarning,
                    stacklevel=2,
                )
            else:
                errors.append((name, exc))
    if errors:
        first_name, first_exc = errors[0]
        raise SiftPytestStepDrainError(
            f"Sift plugin: {len(errors)} step(s) raised while draining mid-session; "
            f"first failure on {first_name!r}: {type(first_exc).__name__}: {first_exc}"
        ) from first_exc


def _drain_parametrize_stack(*, swallow_errors: bool = True) -> None:
    _drain_step_stack(_PARAMETRIZE_STACK, swallow_errors=swallow_errors)


def _drain_hierarchy_stack(*, swallow_errors: bool = True) -> None:
    _drain_step_stack(_HIERARCHY_STACK, swallow_errors=swallow_errors)


def _close_frame(name: str, ns: Any) -> None:
    """Close a single frame, warning on per-frame failure.

    Used by the mid-session hierarchy-stack pop and the rollback paths so a
    misbehaving ``__exit__`` neither shadows the original exception nor leaks
    sibling frames. ``ns=None`` indicates a non-rendered diff-only frame; skip.
    """
    if ns is None:
        return
    try:
        ns.__exit__(None, None, None)
    except Exception as exc:
        warnings.warn(
            f"Sift plugin: closing step {name!r} raised {type(exc).__name__}: {exc}",
            SiftPytestStepDrainWarning,
            stacklevel=2,
        )


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


def _build_hierarchy_chain(
    item: pytest.Item | pytest.Collector,
    config: pytest.Config,
) -> tuple[tuple[str, str, str | None, bool], ...]:
    """Outer-to-inner ``(identity, name, docstring, rendered)`` for collection ancestors.

    Walks ``item.parent`` upward and ALWAYS collects every ``pytest.Package``,
    ``pytest.Module``, and ``pytest.Class`` ancestor — they all participate in
    the diff that keeps the report tree coherent across tests, so two
    same-named ancestors reached via different paths (e.g., ``proj_a/utils``
    and ``proj_b/utils`` in a monorepo where the ``proj_*`` dirs are
    ``pytest.Dir`` nodes the walker skips) cannot silently merge.

    The ``identity`` field is ``node.nodeid`` — globally unique per collected
    node. The diff compares on identity, not the display ``name``.

    The ``rendered`` flag is True iff the layer's ini flag is on
    (``sift_package_step`` / ``sift_module_step`` / ``sift_class_step``).
    Non-rendered frames participate in the diff for identity but don't open a
    Sift step.

    The ``node.obj`` access is a pytest property that imports the underlying
    Python object and can raise *any* exception (ImportError, custom
    metaclass errors, descriptor ``__doc__`` properties that throw). Guard
    broadly so a misbehaving collector doesn't abort the whole collection
    phase — that frame's docstring just becomes ``None``.
    """
    include_package = bool(_PACKAGE_STEP.resolve(config))
    include_module = bool(_MODULE_STEP.resolve(config))
    include_class = bool(_CLASS_STEP.resolve(config))

    chain: list[tuple[str, str, str | None, bool]] = []
    # ``node.parent`` is typed as the internal ``_pytest.nodes.Node`` which
    # isn't part of pytest's public API; widen to ``Any`` for the walk.
    node: Any = item
    while node is not None:
        if isinstance(node, pytest.Class):
            rendered = include_class
        elif isinstance(node, pytest.Module):
            rendered = include_module
        elif isinstance(node, pytest.Package):
            rendered = include_package
        else:
            node = node.parent
            continue
        try:
            doc = (
                (getattr(node, "obj", None) and getattr(node.obj, "__doc__", None)) or ""
            ).strip() or None
        except Exception:
            doc = None
        chain.append((node.nodeid, node.name, doc, rendered))
        node = node.parent
    return tuple(reversed(chain))


# Settings-reference categories. Each maps to a docs subsection and, in the
# renderer, to the column subset that category actually uses.
_CAT_BEHAVIOR = "Pytest behavior"
_CAT_CONNECTION = "Connection"
_CAT_REPORT = "Report content"
_CATEGORIES = (_CAT_BEHAVIOR, _CAT_CONNECTION, _CAT_REPORT)

_TOOL_SIFT_KEY = pytest.StashKey[dict]()


def _tool_sift(config: pytest.Config | None) -> dict[str, Any]:
    """Session-cached ``[tool.sift]`` table.

    Every option that reads TOML, plus the typo detector, would otherwise
    re-parse pyproject.toml on the session-start path — and re-emit the
    malformed-file warning each time. Parse once per session via the config
    stash; ``load_tool_sift`` stays the uncached parser for direct callers.
    """
    if config is None:
        return {}
    cached = config.stash.get(_TOOL_SIFT_KEY, None)
    if cached is None:
        cached = load_tool_sift(config)
        config.stash[_TOOL_SIFT_KEY] = cached
    return cached


@dataclass(frozen=True)
class _Option:
    """One setting, declared once across every surface it exposes.

    Declare only the surfaces the setting uses. ``pytest_addoption``, the
    resolvers (:meth:`resolve` / :meth:`resolve_merged`), the docs renderer
    (:func:`_render_settings_reference`), and the typo detector all read from
    the same registry, so adding or changing a setting is one edit.

    Two shapes:

    - **Scalar** (default): :meth:`resolve` walks env > cli > ini > toml > None
      following only the surfaces declared. In practice no current option uses
      both env and cli, so the chain isn't ambiguous.
    - **Free-form dict** (``merge=True``): a TOML table whose keys are
      user-defined. :meth:`resolve_merged` returns the table with non-scalar
      values dropped, and the typo detector leaves its keys unvalidated. Used
      for ``metadata``.

    Surface fields:

    - ``cli`` / ``cli_action``: CLI flag (e.g. ``"--sift-offline"``) and
      argparse action; ``cli_dest`` is derived from the flag.
    - ``ini`` / ``ini_type`` / ``ini_default``: pytest ini key under
      ``[tool.pytest.ini_options]`` and its pytest type + default.
    - ``toml``: tuple path under ``[tool.sift...]``, e.g.
      ``("pytest", "report", "name")`` -> ``tool.sift.pytest.report.name``.
    - ``env``: full env var name, e.g. ``"SIFT_API_KEY"``.

    ``category`` groups the option in the docs settings reference (one of
    ``_CATEGORIES``).
    """

    name: str
    help: str
    category: str
    cli: str | None = None
    cli_action: str | None = None
    ini: str | None = None
    ini_type: str | None = None
    ini_default: Any = None
    toml: tuple[str, ...] | None = None
    env: str | None = None
    merge: bool = False

    @property
    def cli_dest(self) -> str:
        """Argparse ``dest`` for the option.

        When the option has both a CLI flag and an ini key, the dest matches
        the ini name so ``config.getoption(ini_name)`` returns the CLI value
        (and falls through to ``config.getini(ini_name)`` when the flag wasn't
        passed). Without an ini key, the dest derives from the flag name.
        """
        if self.ini:
            return self.ini
        if self.cli is None:
            return self.name
        return self.cli.lstrip("-").replace("-", "_")

    def __post_init__(self) -> None:
        if self.cli_action and not self.cli:
            raise ValueError(f"_Option({self.name!r}): cli_action requires cli")
        if self.ini_type and not self.ini:
            raise ValueError(f"_Option({self.name!r}): ini_type requires ini")
        if self.merge and not self.toml:
            raise ValueError(f"_Option({self.name!r}): merge=True needs toml")
        if not any([self.cli, self.ini, self.toml, self.env]):
            raise ValueError(f"_Option({self.name!r}): declares no surfaces")
        if self.category not in _CATEGORIES:
            raise ValueError(f"_Option({self.name!r}): category must be one of {_CATEGORIES}")

    def resolve(self, config: pytest.Config | None) -> Any:
        """First set value from declared surfaces; ``None`` when unset everywhere.

        Walk order is env > cli > ini > toml. No current option declares both
        env and cli, so the chain isn't ambiguous in practice.
        ``getini`` returns the typed default for unset bool/list keys, so this
        only returns ini values for booleans (always meaningful), non-empty
        strings, and non-empty lists.
        """
        if self.env:
            env_value = os.getenv(self.env)
            if env_value not in (None, ""):
                return env_value
        if config is None:
            return None
        if self.cli:
            cli_value = config.getoption(self.cli_dest, default=None)
            if cli_value is not None:
                return cli_value
        if self.ini:
            try:
                ini_value = config.getini(self.ini)
            except (KeyError, ValueError):
                ini_value = None
            if isinstance(ini_value, bool):
                return ini_value
            if isinstance(ini_value, str) and ini_value:
                return ini_value
            if isinstance(ini_value, list) and ini_value:
                return ini_value
        if self.toml:
            toml_value = _walk_toml(_tool_sift(config), self.toml)
            if toml_value not in (None, ""):
                return toml_value
        return None

    def resolve_merged(self, config: pytest.Config | None) -> dict[str, str | float | bool]:
        """For ``merge=True`` dict-shape settings: the free-form TOML table.

        TOML values that don't fit ``dict[str, str | float | bool]`` (nested
        tables, lists, ``None``) are dropped with a warning so a malformed
        entry can't crash report creation.
        """
        result: dict[str, str | float | bool] = {}
        if config is not None and self.toml:
            base = _walk_toml(_tool_sift(config), self.toml)
            if isinstance(base, dict):
                for key, value in base.items():
                    if not isinstance(key, str):
                        continue
                    if isinstance(value, (bool, str, int, float)):
                        # ``bool`` first since ``isinstance(True, int)`` is True.
                        result[key] = value  # type: ignore[assignment]
                        continue
                    warnings.warn(
                        f"[tool.sift.{'.'.join(self.toml)}] entry {key!r} ignored: "
                        f"unsupported type {type(value).__name__}.",
                        SiftPytestPluginWarning,
                        stacklevel=2,
                    )
        return result


def _walk_toml(data: dict[str, Any], path: tuple[str, ...]) -> Any:
    """Walk a parsed TOML tree along ``path``; return None on any missing key."""
    cur: Any = data
    for key in path:
        if not isinstance(cur, dict):
            return None
        cur = cur.get(key)
        if cur is None:
            return None
    return cur


# ---------------------------------------------------------------------------
# Settings registry.
#
# Add new options here. The registry drives `pytest_addoption`, resolution,
# the docs settings-reference table, and the unknown-key typo detector, so a
# setting is declared once instead of wired up in several places.
#
# Where each setting lives follows a few principles:
#   - Secrets (the API key) come from environment variables only, never a
#     committed file.
#   - Pytest behavior lives in [tool.pytest.ini_options] so it integrates with
#     `pytest --help` / `--co` / `--trace-config`.
#   - Sift report content lives in [tool.sift.pytest.report.*].
#   - Non-secret endpoints take an env var plus one static home (ini or toml,
#     not both).
#   - A CLI flag is added only when there is a real per-run override workflow;
#     stable project config stays in ini/toml.
#   - Dynamic per-run values are injected via environment variables (pytest-dotenv
#     loads .env for local dev; CI sets the same names from its secret store).
# ---------------------------------------------------------------------------

# Pytest behavior. The CLI flag survives because the per-run override is real.
_LOG_FILE = _Option(
    name="log_file",
    category=_CAT_BEHAVIOR,
    help="Path to the JSONL log of create/update calls (path | true | false | none).",
    cli="--sift-log-file",
    ini="sift_log_file",
)
_GIT_METADATA = _Option(
    name="git_metadata",
    category=_CAT_BEHAVIOR,
    help="Capture git repo/branch/commit on the report.",
    cli="--no-sift-git-metadata",
    cli_action="store_false",
    ini="sift_git_metadata",
    ini_type="bool",
    ini_default=True,
)
_OFFLINE = _Option(
    name="offline",
    category=_CAT_BEHAVIOR,
    help="Skip the session-start ping; route create/update through the JSONL log.",
    cli="--sift-offline",
    cli_action="store_true",
    ini="sift_offline",
    ini_type="bool",
    ini_default=False,
)
_DISABLED = _Option(
    name="disabled",
    category=_CAT_BEHAVIOR,
    help="Disable Sift entirely (no API calls, no log file). Supersedes --sift-offline.",
    cli="--sift-disabled",
    cli_action="store_true",
    ini="sift_disabled",
    ini_type="bool",
    ini_default=False,
)

_OPEN = _Option(
    name="open_report",
    category=_CAT_BEHAVIOR,
    help="Open the resulting report in a browser at session end (online only; "
    "no-op when the report URL can't be resolved).",
    cli="--sift-open-report",
    cli_action="store_true",
    ini="sift_open_report",
    ini_type="bool",
    ini_default=False,
)

# Pytest behavior: set-once project defaults (no CLI flag — no per-run override).
_AUTOUSE = _Option(
    name="autouse",
    category=_CAT_BEHAVIOR,
    help="Default for the Sift autouse fixtures (report_context, step, hierarchy/parametrize parents).",
    ini="sift_autouse",
    ini_type="bool",
    ini_default=True,
)
_PACKAGE_STEP = _Option(
    name="package_step",
    category=_CAT_BEHAVIOR,
    help="Open a parent step for each Python package in the test path.",
    ini="sift_package_step",
    ini_type="bool",
    ini_default=True,
)
_MODULE_STEP = _Option(
    name="module_step",
    category=_CAT_BEHAVIOR,
    help="Open a parent step for each test module.",
    ini="sift_module_step",
    ini_type="bool",
    ini_default=True,
)
_CLASS_STEP = _Option(
    name="class_step",
    category=_CAT_BEHAVIOR,
    help="Open per-class parent steps, including nested classes.",
    ini="sift_class_step",
    ini_type="bool",
    ini_default=True,
)
_PARAMETRIZE_NESTING = _Option(
    name="parametrize_nesting",
    category=_CAT_BEHAVIOR,
    help="Cluster parametrized tests under shared parent steps (e.g. test_a -> v=1, v=2).",
    ini="sift_parametrize_nesting",
    ini_type="bool",
    ini_default=True,
)

# Credentials. The API key is env-only; the URIs accept env + ini.
_API_KEY = _Option(
    name="api_key",
    category=_CAT_CONNECTION,
    help="Sift API key (secret, env-only).",
    env="SIFT_API_KEY",
)
_GRPC_URI = _Option(
    name="grpc_uri",
    category=_CAT_CONNECTION,
    help="Sift gRPC endpoint URI.",
    env="SIFT_GRPC_URI",
    ini="sift_grpc_uri",
)
_REST_URI = _Option(
    name="rest_uri",
    category=_CAT_CONNECTION,
    help="Sift REST endpoint URI.",
    env="SIFT_REST_URI",
    ini="sift_rest_uri",
)
_APP_URL = _Option(
    name="app_url",
    category=_CAT_CONNECTION,
    help="Sift web-app origin for the report link in the terminal footer (e.g. "
    "https://app.siftstack.com). When unset, the link is derived from the REST URI "
    "for known Sift hosts.",
    env="SIFT_APP_URL",
    ini="sift_app_url",
)

# Report content. Project defaults in [tool.sift.pytest.report]; CI injects
# per-run values via SIFT_REPORT_* env vars (pytest-dotenv handles .env files
# for local dev).
_REPORT_NAME = _Option(
    name="report_name",
    category=_CAT_REPORT,
    help="Template for the report display name. Placeholders: {target}, {command}, {args}, "
    "{rootdir}, {timestamp}, {count}, {git_repo}, {git_branch}, {git_commit}.",
    toml=("pytest", "report", "name"),
)
_TEST_CASE = _Option(
    name="test_case",
    category=_CAT_REPORT,
    help="Template for the report's test_case field (same placeholders as report_name).",
    toml=("pytest", "report", "test_case"),
)
_TEST_SYSTEM_NAME = _Option(
    name="test_system_name",
    category=_CAT_REPORT,
    help="Name of the test system / rig. Defaults to the host's name.",
    env="SIFT_REPORT_TEST_SYSTEM_NAME",
    toml=("pytest", "report", "test_system_name"),
)
_SYSTEM_OPERATOR = _Option(
    name="system_operator",
    category=_CAT_REPORT,
    help="Operator running the test. Defaults to the OS user.",
    env="SIFT_REPORT_SYSTEM_OPERATOR",
    toml=("pytest", "report", "system_operator"),
)
_SERIAL_NUMBER = _Option(
    name="serial_number",
    category=_CAT_REPORT,
    help="Serial number of the unit under test.",
    env="SIFT_REPORT_SERIAL_NUMBER",
    toml=("pytest", "report", "serial_number"),
)
_PART_NUMBER = _Option(
    name="part_number",
    category=_CAT_REPORT,
    help="Part number of the unit under test.",
    env="SIFT_REPORT_PART_NUMBER",
    toml=("pytest", "report", "part_number"),
)
_METADATA = _Option(
    name="metadata",
    category=_CAT_REPORT,
    help="Free-form report metadata, as a TOML table of scalar values. For "
    "dynamic per-run keys, attach them in conftest via the report_context fixture.",
    toml=("pytest", "report", "metadata"),
    merge=True,
)

_OPTIONS: tuple[_Option, ...] = (
    _LOG_FILE,
    _GIT_METADATA,
    _OFFLINE,
    _DISABLED,
    _OPEN,
    _AUTOUSE,
    _PACKAGE_STEP,
    _MODULE_STEP,
    _CLASS_STEP,
    _PARAMETRIZE_NESTING,
    _API_KEY,
    _GRPC_URI,
    _REST_URI,
    _APP_URL,
    _REPORT_NAME,
    _TEST_CASE,
    _TEST_SYSTEM_NAME,
    _SYSTEM_OPERATOR,
    _SERIAL_NUMBER,
    _PART_NUMBER,
    _METADATA,
)


def pytest_addoption(parser: pytest.Parser) -> None:
    """Register every CLI flag and pytest ini key declared in ``_OPTIONS``.

    One loop drives both surfaces — adding a setting is one entry in the
    registry, not three edits across this function and a docs table.
    """
    group = parser.getgroup("sift", description="Sift test results")
    for opt in _OPTIONS:
        if opt.cli is not None:
            cli_kwargs: dict[str, Any] = {
                "dest": opt.cli_dest,
                "default": None,
                "help": opt.help,
            }
            if opt.cli_action is not None:
                cli_kwargs["action"] = opt.cli_action
            group.addoption(opt.cli, **cli_kwargs)
        if opt.ini is not None:
            ini_kwargs: dict[str, Any] = {"help": opt.help, "default": opt.ini_default}
            if opt.ini_type is not None:
                ini_kwargs["type"] = opt.ini_type
            parser.addini(opt.ini, **ini_kwargs)


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
    _warn_on_unknown_env_vars()
    _warn_on_unknown_toml_keys(config)


def _render_settings_reference() -> str:
    """Render the Markdown settings reference from ``_OPTIONS``.

    One ``### <category>`` subsection per category, each table showing only the
    columns that category uses (so no dead all-``—`` columns). The plugin docs
    at ``docs/guides/pytest_plugin/configuration.md`` embed this output verbatim
    so the registry and the docs can't drift;
    ``test_settings_reference_docs_in_sync`` is the guard rail. Regenerate with::

        uv run python -c "from sift_client.pytest_plugin import _render_settings_reference; print(_render_settings_reference())"
    """

    def _cli_cell(opt: _Option) -> str:
        return f"`{opt.cli}`" if opt.cli else "—"

    def _ini_cell(opt: _Option) -> str:
        return f"`{opt.ini}`" if opt.ini else "—"

    def _toml_cell(opt: _Option) -> str:
        if not opt.toml:
            return "—"
        if opt.merge:
            return f"`[tool.sift.{'.'.join(opt.toml)}]` (table)"
        section = ".".join(opt.toml[:-1])
        return f"`[tool.sift.{section}] {opt.toml[-1]}`"

    def _env_cell(opt: _Option) -> str:
        if opt.env:
            return f"`{opt.env}`"
        return "—"

    # Per-category column layout: only the surfaces that category actually uses.
    # Each column is (header, cell-renderer).
    columns_by_category = {
        _CAT_BEHAVIOR: [
            ("CLI flag", _cli_cell),
            ("Ini (`[tool.pytest.ini_options]`)", _ini_cell),
        ],
        _CAT_CONNECTION: [
            ("Ini (`[tool.pytest.ini_options]`)", _ini_cell),
            ("Env var", _env_cell),
        ],
        _CAT_REPORT: [
            ("TOML (`[tool.sift...]`)", _toml_cell),
            ("Env var", _env_cell),
        ],
    }

    def _escape(cell: str) -> str:
        # Literal pipes inside a Markdown table cell need backslash escaping or
        # they'd be parsed as column separators.
        return cell.replace("|", "\\|")

    blocks: list[str] = []
    for category in _CATEGORIES:
        opts = [o for o in _OPTIONS if o.category == category]
        if not opts:
            continue
        columns = columns_by_category[category]
        headers = ["Setting", *(h for h, _ in columns)]
        lines = [
            f"### {category}",
            "",
            "| " + " | ".join(headers) + " |",
            "|" + "|".join(["---"] * len(headers)) + "|",
        ]
        for opt in opts:
            cells = [opt.help, *(render(opt) for _, render in columns)]
            lines.append("| " + " | ".join(_escape(c) for c in cells) + " |")
        blocks.append("\n".join(lines))
    return "\n\n".join(blocks)


def _warn_on_unknown_env_vars() -> None:
    """Emit a warning for any ``SIFT_*`` env var not declared in the registry.

    The registry declares each env var by its full name (``opt.env``); a
    ``SIFT_*`` var that matches none of them is almost always a typo.
    """
    import difflib

    known_full = {opt.env for opt in _OPTIONS if opt.env}
    suggestion_pool = sorted(known_full)
    for name in sorted(os.environ):
        if not name.startswith("SIFT_"):
            continue
        if name in known_full:
            continue
        close = difflib.get_close_matches(name, suggestion_pool, n=1, cutoff=0.6)
        hint = f" (did you mean `{close[0]}`?)" if close else ""
        warnings.warn(
            f"Unknown SIFT_* env var `{name}`{hint}; ignored.",
            SiftPytestPluginWarning,
            stacklevel=2,
        )


def _warn_on_unknown_toml_keys(config: pytest.Config) -> None:
    """Walk ``[tool.sift.pytest.*]`` in pyproject.toml and warn on keys outside the registry.

    Only the ``tool.sift.pytest`` subtree is checked. Other ``tool.sift.*``
    subtrees are reserved for non-pytest Sift tooling (e.g. ``tool.sift.extras``
    is consumed by this repo's extras-generation script) and aren't our
    concern. Free-form subtrees (``merge=True`` options like ``metadata``)
    stop the walk — their keys are user-defined and not validated.
    """
    import difflib

    data = _tool_sift(config)
    pytest_table = (data or {}).get("pytest")
    if not isinstance(pytest_table, dict):
        return
    # Build leaf/free-form/prefix sets relative to the ``("pytest", ...)`` root
    # the registry already uses, so the walk runs on the table we just sliced.
    leaves = {opt.toml for opt in _OPTIONS if opt.toml and not opt.merge}
    free_form = {opt.toml for opt in _OPTIONS if opt.toml and opt.merge}
    prefixes: set[tuple[str, ...]] = set()
    for full in leaves | free_form:
        for i in range(len(full)):
            prefixes.add(full[:i])

    def _walk(node: Any, base: tuple[str, ...]) -> None:
        if base in free_form or not isinstance(node, dict):
            return
        for key, value in node.items():
            path = (*base, str(key))
            if path in leaves or path in free_form:
                continue
            if path in prefixes:
                _walk(value, path)
                continue
            full_name = "tool.sift." + ".".join(path)
            same_depth = [
                ".".join(p) for p in (leaves | free_form | prefixes) if len(p) == len(path)
            ]
            close = difflib.get_close_matches(".".join(path), same_depth, n=1, cutoff=0.6)
            hint = f" (did you mean `tool.sift.{close[0]}`?)" if close else ""
            warnings.warn(
                f"Unknown sift config key `{full_name}`{hint}; ignored.",
                SiftPytestPluginWarning,
                stacklevel=2,
            )

    _walk(pytest_table, ("pytest",))


def pytest_collection_modifyitems(config: pytest.Config, items: list[pytest.Item]) -> None:
    """Stash each item's class chain + parametrize path and cluster siblings.

    Sorts by ``(file_path, hierarchy_chain, parametrize_path)`` so sibling
    items under a shared parent (package, module, class, or parametrize axis)
    stay contiguous — otherwise a free function sorting between two class
    methods would tear down + re-open the class step, producing duplicate
    parents in the report tree.
    """
    for item in items:
        item.stash[_HIERARCHY_KEY] = _build_hierarchy_chain(item, config)
        item.stash[_PARAMETRIZE_PATH_KEY] = _build_parametrize_path(item)
    # Use ``.get(...)`` defensively: a third-party hook may inject items after
    # our stashing loop runs, and we'd rather sort them at the tail than
    # KeyError out of collection.
    items.sort(
        key=lambda i: (
            str(i.path),
            tuple(identity for identity, _, _, _ in i.stash.get(_HIERARCHY_KEY, ())),
            i.stash.get(_PARAMETRIZE_PATH_KEY, ()),
        )
    )


def pytest_sessionfinish(session: pytest.Session, exitstatus: int) -> None:
    """Drain any parent steps still open at session end (innermost first).

    Wrapped so a failure in the inner drain does not prevent the outer one
    from running. With ``module_substep`` removed, this is the sole place
    where hierarchy parents close — they persist across all tests and only
    drain when the session ends.
    """
    try:
        _drain_parametrize_stack()
    finally:
        _drain_hierarchy_stack()


def _is_offline(pytestconfig: pytest.Config | None) -> bool:
    return bool(_OFFLINE.resolve(pytestconfig))


def _is_disabled(pytestconfig: pytest.Config | None) -> bool:
    return bool(_DISABLED.resolve(pytestconfig))


def _sdk_version() -> str:
    """Return the installed ``sift_stack_py`` version, or ``"unknown"``."""
    from importlib.metadata import PackageNotFoundError, version

    try:
        return version("sift_stack_py")
    except PackageNotFoundError:
        return "unknown"


def _mode_label(config: pytest.Config) -> str:
    """Resolve the active mode for the terminal header: disabled > offline > online."""
    if _is_disabled(config):
        return "disabled"
    if _is_offline(config):
        return "offline"
    return "online"


def pytest_report_header(config: pytest.Config) -> str | None:
    """Emit a session-start header with the SDK version and active mode.

    Suppressed under ``-q`` (negative verbosity), matching how pytest hides its
    own platform/plugin header.
    """
    if config.get_verbosity() < 0:
        return None
    return f"Sift: sift-stack-py {_sdk_version()} — {_mode_label(config)} mode"


def _resolve_real_report_id(context: Any) -> str | None:
    """Resolve the real server-side report id for the online footer link.

    In synchronous online mode (``--sift-log-file=false``) the report is created
    directly against the API, so ``report.id_`` is already the real id. In the
    default incremental mode the report is created through the simulate path
    (a client-side UUID) and the background worker maps it to the real id on
    replay, recording it in the ``<log>.tracking`` sidecar's ``id_map``. By the
    time this footer runs the session-scoped report context has torn down and
    the worker has drained, so the sidecar is final.

    Returns ``None`` when the worker never mapped the report (e.g. it died before
    replaying the create), meaning no real report exists to link.
    """
    report = context.report
    if not report.id_:
        # No id was ever assigned (unset/empty); nothing to link.
        return None
    sim_id = str(report.id_)
    if not getattr(report, "is_simulated", False):
        return sim_id
    log_file = getattr(context, "log_file", None)
    if log_file is None:
        return None
    from sift_client._internal.low_level_wrappers._test_results_log import LogTracking

    return LogTracking.load(log_file).id_map.get(sim_id)


_LABEL_WIDTH = 13


def _sift_kv(terminalreporter: Any, label: str, value: str, **value_markup: bool) -> None:
    """Write an indented ``label  value`` row, bolding the label.

    ``value_markup`` (e.g. ``green=True``, ``cyan=True``) styles only the value.
    Color is dropped automatically when the terminal has no markup (not a TTY or
    ``--color=no``), so captured/CI output stays plain text.
    """
    terminalreporter.write("  ")
    terminalreporter.write(f"{label:<{_LABEL_WIDTH}}", bold=True)
    terminalreporter.write_line(value, **value_markup)


# Step-count breakdown order and labels for the footer's "Steps" row.
_STEP_COUNT_ORDER: tuple[tuple[TestStatus, str], ...] = (
    (TestStatus.PASSED, "passed"),
    (TestStatus.FAILED, "failed"),
    (TestStatus.ERROR, "error"),
    (TestStatus.ABORTED, "aborted"),
    (TestStatus.SKIPPED, "skipped"),
    (TestStatus.IN_PROGRESS, "in progress"),
)


# Per-status color for the footer's step breakdown: green pass, red
# failure/error/abort, yellow skip; in-progress (and anything else) stays plain.
_STEP_STATUS_MARKUP: dict[TestStatus, dict[str, bool]] = {
    TestStatus.PASSED: {"green": True},
    TestStatus.FAILED: {"red": True},
    TestStatus.ERROR: {"red": True},
    TestStatus.ABORTED: {"red": True},
    TestStatus.SKIPPED: {"yellow": True},
}


def _step_count_segments(counts: Any) -> list[tuple[str, dict[str, bool]]]:
    """Build ``(text, markup)`` segments for a step tally, non-zero only."""
    return [
        (f"{counts.get(status, 0)} {label}", _STEP_STATUS_MARKUP.get(status, {}))
        for status, label in _STEP_COUNT_ORDER
        if counts.get(status, 0)
    ]


def _measurement_segments(counts: Any) -> list[tuple[str, dict[str, bool]]]:
    """Build ``(text, markup)`` segments for a measurement tally, non-zero only."""
    segments: list[tuple[str, dict[str, bool]]] = []
    if counts.get(True, 0):
        segments.append((f"{counts[True]} passed", {"green": True}))
    if counts.get(False, 0):
        segments.append((f"{counts[False]} failed", {"red": True}))
    return segments


def _write_count_row(
    terminalreporter: Any, label: str, segments: list[tuple[str, dict[str, bool]]]
) -> None:
    """Write a ``label  a · b · c`` row, applying each segment's color markup."""
    terminalreporter.write("  ")
    terminalreporter.write(f"{label:<{_LABEL_WIDTH}}", bold=True)
    for index, (text, markup) in enumerate(segments):
        if index:
            terminalreporter.write(" · ")
        terminalreporter.write(text, **markup)
    terminalreporter.write_line("")


def _report_panel_title(report: Any, terminalreporter: Any) -> str:
    """``Sift report · <name>`` for the section rule, truncated to the terminal width.

    The report name embeds a timestamp (and, for invocation-based runs, the
    pytest args), so a long name is truncated with an ellipsis to keep the
    separator line from wrapping.
    """
    base = "Sift report"
    name = getattr(report, "name", None)
    if not name:
        return base
    title = f"{base} · {name}"
    fullwidth = getattr(getattr(terminalreporter, "_tw", None), "fullwidth", 80)
    # Reserve room for the separator characters and spaces write_sep adds.
    limit = max(len(base), fullwidth - 8)
    if len(title) > limit:
        title = title[: limit - 1] + "…"
    return title


def _maybe_open_report(url: str) -> None:
    """Best-effort open the report URL in a browser (for ``--sift-open-report``).

    Skipped on CI or non-interactive sessions so a committed ``sift_open_report``
    setting can't spawn a browser on a headless agent; the flag is meant for
    local development.
    """
    import sys
    import webbrowser

    if os.environ.get("CI") or not sys.stdout.isatty():
        return
    try:
        # webbrowser.open forks/execs the platform opener while the gRPC client's
        # background threads are live; redirect fd 2 across the fork to swallow
        # gRPC's prefork notice (same treatment as the plugin's other fork sites).
        with _quiet_fork_stderr():
            webbrowser.open(url)
    except Exception:
        # Headless / no browser available: opening is a convenience, never fatal.
        pass


def pytest_terminal_summary(terminalreporter: Any, exitstatus: int, config: pytest.Config) -> None:
    """Emit a session-end Sift report summary, adapting per mode.

    The printed panel is suppressed under ``-q``, but programmatic side effects
    (stashing the report ref for ``conftest.py``, ``--sift-open-report``) still run so
    other plugins and CI steps can consume the result. The panel shows the
    outcome (green/red), step and measurement tallies, and a per-mode action: a
    report link (online), the upload command (offline), or a disabled note.
    """
    quiet = config.get_verbosity() < 0

    if _is_disabled(config):
        if not quiet:
            terminalreporter.write_sep("=", "Sift", cyan=True, bold=True)
            terminalreporter.write_line("Sift disabled — no test report created.")
        return

    context = REPORT_CONTEXT
    if context is None:
        # No gated test ran, so no report context was created. Nothing to show.
        return

    log_file = getattr(context, "log_file", None)
    offline = _is_offline(config)

    # Resolve the report link first so stashing and --sift-open-report run even under
    # -q (programmatic consumers don't care about verbosity). Truthiness, not
    # ``is not None``: a resolved-but-empty id (degenerate sidecar mapping, unset
    # proto field) must fall through to the "not uploaded" path, not produce a
    # ``/test-results/`` link.
    report_id = None if offline else _resolve_real_report_id(context)
    report_url = (
        f"{context.client.app_url}/test-results/{report_id}"
        if report_id and context.client.app_url
        else None
    )
    if report_id:
        config.stash[SIFT_REPORT_ID_STASH_KEY] = report_id
    if report_url is not None:
        config.stash[SIFT_REPORT_URL_STASH_KEY] = report_url
        if _OPEN.resolve(config):
            _maybe_open_report(report_url)

    if quiet:
        return

    failed = bool(getattr(context, "any_failures", False))
    status_word, status_markup = (
        ("FAILED", {"red": True, "bold": True})
        if failed
        else ("PASSED", {"green": True, "bold": True})
    )
    # Offline results live only in the local log until replayed, so the status
    # row calls that out instead of repeating the version (already in the header).
    status_context = (
        f"{_mode_label(config)} · not uploaded"
        if offline
        else f"{_mode_label(config)} · sift-stack-py {_sdk_version()}"
    )

    report = context.report

    terminalreporter.write_sep(
        "=", _report_panel_title(report, terminalreporter), cyan=True, bold=True
    )

    # Identity row: the test case (test path or pytest invocation).
    if report.test_case:
        _sift_kv(terminalreporter, "Test case", str(report.test_case))

    # Status row: colored outcome, then compact mode context.
    terminalreporter.write("  ")
    terminalreporter.write(f"{'Status':<{_LABEL_WIDTH}}", bold=True)
    terminalreporter.write(status_word, **status_markup)
    terminalreporter.write_line(f"      {status_context}")

    # Step + measurement tallies (green pass, red failure, yellow skip).
    _write_count_row(
        terminalreporter,
        "Steps",
        _step_count_segments(context.step_status_counts) or [("no steps", {})],
    )
    measurement_segments = _measurement_segments(context.measurement_counts)
    if measurement_segments:
        _write_count_row(terminalreporter, "Measurements", measurement_segments)

    # Provenance row: test system and operator.
    system = " · ".join(part for part in (report.test_system_name, report.system_operator) if part)
    if system:
        _sift_kv(terminalreporter, "System", system)

    # Local log file (write-through backup online, sole sink offline).
    if log_file is not None:
        _sift_kv(terminalreporter, "Log file", str(log_file))

    if offline:
        if log_file is not None:
            terminalreporter.write_sep("-", "to upload to Sift")
            terminalreporter.write_line(f"  >> import-test-result-log {log_file}", cyan=True)
        return

    if not report_id:
        # Incremental upload never mapped the report (the worker died before
        # replaying the create), so there's no real report to link.
        _sift_kv(
            terminalreporter,
            "Report",
            f"not uploaded — replay with: import-test-result-log {log_file}",
            yellow=True,
        )
    elif report_url is not None:
        _sift_kv(terminalreporter, "Report", report_url, cyan=True)
    else:
        _sift_kv(
            terminalreporter,
            "Report",
            f"id {report_id}  (set sift_app_url for a clickable link)",
        )

    if report_id and getattr(context, "replay_incomplete", False) and log_file is not None:
        _sift_kv(
            terminalreporter,
            "",
            f"may be incomplete — finish with: import-test-result-log {log_file}",
            yellow=True,
        )


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
    raw = _LOG_FILE.resolve(pytestconfig)
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


def _error_info_from_longrepr(longrepr: Any) -> ErrorInfo:
    """Fall back to the report's longrepr when no Python exception is available."""
    return ErrorInfo(error_code=1, error_message=str(longrepr) if longrepr is not None else "")


def _resolve_initial_status(new_step: NewStep, item: pytest.Item) -> None:
    """Resolve the function step's status from pytest's per-phase reports.

    Reads ``_sift_phase_setup`` / ``_sift_phase_call`` and the test's xfail marker,
    then mutates ``new_step.current_step`` in place and flips
    ``new_step._sift_managed_externally`` so ``NewStep.__exit__`` emits the
    resolved status without re-classifying.

    When the call phase reports ``passed`` and no override is needed (i.e. the
    test's own status or substep failures should drive the result), this leaves
    the step alone so the default ``__exit__`` resolution stays in charge.
    """
    current_step = new_step.current_step
    if current_step is None:
        # The step never opened (the autouse fixture short-circuited or was
        # disabled). Nothing to resolve.
        return
    setup_phase = getattr(item, "_sift_phase_setup", None)
    call_phase = getattr(item, "_sift_phase_call", None)
    xfail_marker = item.get_closest_marker("xfail")
    xfail_runs = xfail_marker.kwargs.get("run", True) if xfail_marker is not None else True

    status: TestStatus | None = None
    error_info: ErrorInfo | None = None
    keep_managed = False

    if setup_phase is not None and setup_phase.report.outcome == "failed":
        status = TestStatus.ERROR
        excinfo = setup_phase.call.excinfo
        if excinfo is not None:
            error_info = format_truncated_traceback(excinfo.type, excinfo.value, excinfo.tb)
        else:
            error_info = _error_info_from_longrepr(setup_phase.report.longrepr)
    elif setup_phase is not None and setup_phase.report.outcome == "skipped":
        status = TestStatus.SKIPPED
    elif call_phase is None:
        # Setup completed but the call-phase report never fired — the inner
        # pytester session was aborted (e.g. by KeyboardInterrupt) before the
        # plugin could observe the outcome. Leave the step at IN_PROGRESS so
        # the report does not lie about a clean pass.
        keep_managed = True
    else:
        wasxfail = getattr(call_phase.report, "wasxfail", None)
        if wasxfail is not None:
            if call_phase.report.outcome == "failed":
                # Strict xpass: pytest synthesizes a failure when an xfail(strict=True)
                # test unexpectedly passes. The xfail mark no longer matches reality.
                status = TestStatus.FAILED
            elif call_phase.report.outcome == "skipped":
                if xfail_marker is not None and xfail_runs is False:
                    # xfail(run=False): the test body never executed.
                    status = TestStatus.SKIPPED
                else:
                    # xfail + expected failure: the test fulfilled its xfail expectation.
                    status = TestStatus.PASSED
            else:
                # Non-strict xpass: passes that weren't required to fail.
                status = TestStatus.PASSED
        elif call_phase.report.outcome == "passed":
            # Default __exit__ resolves PASSED/FAILED from open_step_results and any
            # status the test code may have set. Don't override it here.
            return
        elif call_phase.report.outcome == "skipped":
            status = TestStatus.SKIPPED
        elif call_phase.report.outcome == "failed":
            excinfo = call_phase.call.excinfo
            children_passed = new_step.report_context.open_step_results.get(
                current_step.step_path, True
            )
            if excinfo is None:
                status = TestStatus.FAILED
            elif isinstance(excinfo.value, AssertionError):
                status = TestStatus.FAILED
                error_info = format_assertion_message(excinfo.type, excinfo.value)
            elif isinstance(excinfo.value, pytest.fail.Exception):
                status = TestStatus.FAILED
            elif isinstance(excinfo.value, (KeyboardInterrupt, SystemExit)):
                # Hard exits the plugin can observe: pytest converted the
                # raise into a call-phase report. The session-aborting variant
                # (call_phase is None) lands earlier and stays IN_PROGRESS.
                status = TestStatus.ABORTED
                error_info = format_truncated_traceback(excinfo.type, excinfo.value, excinfo.tb)
            elif xfail_marker is not None:
                # xfail(raises=X) with a non-matching exception: the contract failed.
                status = TestStatus.FAILED
                error_info = format_truncated_traceback(excinfo.type, excinfo.value, excinfo.tb)
            elif not children_passed:
                # A substep already recorded the error and carries the traceback;
                # the test step only inherits the child-failed signal.
                status = TestStatus.FAILED
            else:
                status = TestStatus.ERROR
                error_info = format_truncated_traceback(excinfo.type, excinfo.value, excinfo.tb)

    if status is None and not keep_managed:
        return

    if status is not None:
        # BaseType is frozen; mutate via __dict__ the same way _apply_client_to_instance does.
        current_step.__dict__["status"] = status
        if error_info is not None:
            current_step.__dict__["error_info"] = error_info
    new_step._sift_managed_externally = True


def _finalize_after_teardown(item: pytest.Item, teardown_report: pytest.TestReport) -> None:
    """Upgrade a closed step to FAILED when the teardown phase failed.

    The autouse step fixture has already exited by the time the teardown
    makereport hook fires, so call ``step.update`` again to override the status
    server-side and propagate the failure to the still-open parent step.
    """
    step: NewStep | None = getattr(item, "_sift_step", None)
    if step is None:
        return
    current_step = step.current_step
    if current_step is None:
        return
    if teardown_report.outcome == "failed" and current_step.status == TestStatus.PASSED:
        current_step.update({"status": TestStatus.FAILED})
        step.report_context.mark_step_failed_after_close(current_step)


@pytest.hookimpl(tryfirst=True, hookwrapper=True)
def pytest_runtest_makereport(item: pytest.Item, call: pytest.CallInfo[Any]):
    """Capture per-phase reports and finalize step status after teardown.

    Stashes both ``rep_<when>`` (the ``CallInfo``, kept for pytest plugins that
    expect that conventional attribute) and ``_sift_phase_<when>`` (a
    ``SimpleNamespace(call, report)`` used by ``_resolve_initial_status``). The
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
        _finalize_after_teardown(item, report)


def _relativize(path: Path, rootpath: Path) -> str:
    """Path relative to rootdir, or the basename when it sits outside the tree."""
    try:
        rel = str(path.relative_to(rootpath))
    except ValueError:
        return path.name
    return "" if rel == "." else rel


def _strip_param(nodeid: str) -> str:
    """Drop the trailing ``[param]`` from a nodeid, keeping ``file::Class::func``.

    The parametrize id is a variation of the test, not its identity — leaving it
    in would make a re-parametrization silently shift the grouping key. Splits on
    the last ``::`` segment and cuts at its first ``[``; class/function names
    never contain ``[``, so nested brackets in a param value can't confuse it.
    """
    head, sep, leaf = nodeid.rpartition("::")
    leaf = leaf.split("[", 1)[0]
    return f"{head}{sep}{leaf}"


def _derive_target(request: pytest.FixtureRequest, args: tuple[str, ...]) -> str:
    """Describe what was run, from the collected items rather than the command line.

    Collection is the ground truth of selection — independent of flag order,
    ``-k`` / ``-m`` filters, or which path form was typed. Every value is
    anchored to the rootdir (project) name so the shape is uniform; granularity
    narrows with the selection:

    * a single test -> ``project/tests/test_motor.py::test_spin`` (param stripped)
    * a single file -> ``project/tests/test_motor.py``
    * many files    -> their common directory, ``project/tests/motor``
    * whole tree / nothing collected / paths outside rootdir -> ``project``

    The report is session-level and individual tests are its steps, so the
    file/directory grain is the natural unit of "what ran" for the report
    itself. The verbatim invocation stays available via ``{command}`` and the
    ``pytest_command`` metadata key.
    """
    rootpath = request.config.rootpath
    root = rootpath.name

    def _anchor(rel: str) -> str:
        return f"{root}/{rel}" if rel else root

    items = list(getattr(request.session, "items", ()) or ())
    if not items:
        return root
    if len(items) == 1:
        return _anchor(_strip_param(items[0].nodeid))
    paths = {p for p in (getattr(i, "path", None) for i in items) if p is not None}
    if not paths:
        return root
    if len(paths) == 1:
        return _anchor(_relativize(next(iter(paths)), rootpath))
    try:
        common = Path(os.path.commonpath([str(p) for p in paths]))
    except ValueError:
        # e.g. paths on different drives (Windows); fall back to the project.
        return root
    return _anchor(_relativize(common, rootpath))


def _build_template_fields(
    target: str,
    command: str,
    args: tuple[str, ...],
    request: pytest.FixtureRequest,
) -> dict[str, Any]:
    """Build the placeholder mapping shared by the name and test_case templates."""
    items = getattr(request.session, "items", ()) or ()
    git = _git_metadata() or {}
    return {
        "target": target,
        "command": command,
        "args": " ".join(args),
        "rootdir": request.config.rootpath.name,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "count": len(items),
        "git_repo": git.get("git_repo", ""),
        "git_branch": git.get("git_branch", ""),
        "git_commit": git.get("git_commit", ""),
    }


def _format_template(
    template: str,
    fields: dict[str, Any],
    *,
    fallback: str,
    option_label: str,
) -> str:
    """Format ``template`` with ``fields``; on bad input, warn and return ``fallback``.

    A bad template should never block test results from being recorded, so the
    rendering errors collapse to a warning + fallback rather than aborting the
    session.
    """
    try:
        return template.format(**fields)
    except (KeyError, IndexError, ValueError) as exc:
        warnings.warn(
            f"Invalid {option_label} template {template!r} ({exc}); using fallback.",
            SiftPytestPluginWarning,
            stacklevel=2,
        )
        return fallback


def _report_context_impl(
    sift_client: SiftClient,
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config | None = None,
) -> Generator[ReportContext, None, None]:
    args = request.config.invocation_params.args
    # ``target`` is "what ran", derived from the collected items (see
    # _derive_target) — invocation-independent, unlike parsing the command
    # line. Both the display name and test_case default to it; the verbatim
    # command stays available via {command} and the pytest_command metadata.
    target = _derive_target(request, args)
    command = "pytest " + " ".join(args) if args else "pytest"
    fields = _build_template_fields(target, command, args, request)
    name_template = _REPORT_NAME.resolve(pytestconfig) or "{target} {timestamp}"
    name = _format_template(
        name_template,
        fields,
        fallback=f"{target} {fields['timestamp']}",
        option_label="sift_report_name",
    )
    test_case_template = _TEST_CASE.resolve(pytestconfig)
    test_case = (
        _format_template(
            test_case_template,
            fields,
            fallback=target,
            option_label="sift_test_case",
        )
        if test_case_template
        else target
    )
    # Metadata starts from the [tool.sift.pytest.report.metadata] TOML table, and
    # the auto-recorded pytest_command layers in last so the user can't
    # accidentally overwrite it.
    report_metadata: dict[str, str | float | bool] = {
        **_METADATA.resolve_merged(pytestconfig),
        "pytest_command": command,
    }
    # Mode → ReportContext flags:
    #   online (default): log_file=<temp or user path>, replay_log_file=True
    #   --sift-offline:   log_file=<temp or user path>, replay_log_file=False
    #   --sift-disabled:  log_file=False,               replay_log_file=False
    disabled = sift_client._simulate
    offline = False if disabled else _is_offline(pytestconfig)
    log_file: str | Path | bool | None = False if disabled else _resolve_log_file(pytestconfig)
    include_git_metadata = bool(_GIT_METADATA.resolve(pytestconfig))
    with ReportContext(
        sift_client,
        name=name,
        test_case=test_case,
        test_system_name=_TEST_SYSTEM_NAME.resolve(pytestconfig) or None,
        system_operator=_SYSTEM_OPERATOR.resolve(pytestconfig) or None,
        serial_number=_SERIAL_NUMBER.resolve(pytestconfig) or None,
        part_number=_PART_NUMBER.resolve(pytestconfig) or None,
        log_file=log_file,
        include_git_metadata=include_git_metadata,
        replay_log_file=not (disabled or offline),
        metadata=report_metadata,
    ) as context:
        global REPORT_CONTEXT
        REPORT_CONTEXT = context
        try:
            yield context
        finally:
            # Drain the hierarchy + parametrize stacks INSIDE the
            # ReportContext's ``with`` block, so the final ``__exit__``
            # update calls for those parent steps are written to the log
            # file BEFORE the import worker drains. Without this, the
            # worker exits with a partial backlog and the parent steps
            # are stuck IN_PROGRESS in the Sift report.
            try:
                _drain_parametrize_stack()
            finally:
                _drain_hierarchy_stack()


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
    resolved = {
        "SIFT_API_KEY": _API_KEY.resolve(pytestconfig),
        "SIFT_GRPC_URI": _GRPC_URI.resolve(pytestconfig),
        "SIFT_REST_URI": _REST_URI.resolve(pytestconfig),
    }
    missing = [env for env, value in resolved.items() if not value]
    if missing and not _is_offline(pytestconfig):
        raise pytest.UsageError(
            "Sift credentials missing: "
            + ", ".join(missing)
            + ". Set the environment variable(s) — pytest-dotenv loads them "
            "from a `.env` file automatically — or set the URIs under "
            "`sift_grpc_uri` / `sift_rest_uri` in `[tool.pytest.ini_options]` "
            "in pyproject.toml, or override the sift_client fixture in your "
            "conftest.py, or pass --sift-offline / --sift-disabled to run "
            "without contacting Sift."
        )
    for env in missing:
        resolved[env] = _OFFLINE_DEFAULTS[env]
    # Web-app origin for the report link: the SIFT_APP_URL env var wins, then the
    # sift_app_url ini key, else host-based derivation in SiftClient.app_url.
    app_url = _APP_URL.resolve(pytestconfig)
    return SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=resolved["SIFT_API_KEY"] or "",
            grpc_url=resolved["SIFT_GRPC_URI"] or "",
            rest_url=resolved["SIFT_REST_URI"] or "",
            app_url=app_url or None,
        )
    )


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
    # by ``_parametrize_parents``. When parametrize-nesting is disabled, fall
    # back to the bracket-mangled pytest name (e.g. ``test_a[1]``) so the leaf
    # remains uniquely identifiable.
    if _PARAMETRIZE_NESTING.resolve(request.config):
        path = node.stash.get(_PARAMETRIZE_PATH_KEY, ())
        name = path[-1] if path else str(node.name)
    else:
        name = str(node.name)
    # ``node.obj`` may not exist (e.g., ``pytest.DoctestItem``) or may raise
    # when accessed — fall back to no description in those cases rather than
    # erroring out a perfectly valid test. ``getattr``'s default only
    # suppresses ``AttributeError``; the try/except catches everything else
    # (RuntimeError from a misbehaving ``__doc__`` descriptor, etc.).
    try:
        existing_docstring = getattr(getattr(node, "obj", None), "__doc__", None) or None
    except Exception:
        existing_docstring = None
    with report_context.new_step(
        name=name, description=existing_docstring, assertion_as_fail_not_error=False
    ) as new_step:
        node._sift_step = new_step
        yield new_step
        _resolve_initial_status(new_step, node)


@pytest.fixture(autouse=True)
def _hierarchy_parents(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
) -> None:
    """Open/close hierarchy parent steps (packages, modules, classes) for the current item.

    Same diff-stack pattern as ``_parametrize_parents`` but operates on
    ``_HIERARCHY_KEY``. The chain is built outer-to-inner from the item's
    collection-tree ancestors; which node types are included is decided at
    build time by ``sift_package_step`` / ``sift_module_step`` /
    ``sift_class_step``. When the chain changes (pop or push), the parametrize
    stack is drained first since parametrize parents nest INSIDE these.

    Gated off when the item is excluded (avoids eager ``report_context`` setup).
    """
    default = bool(_AUTOUSE.resolve(pytestconfig))
    if not _sift_enabled_for(request.node, default):
        return None
    # Fall back to computing the chain on-demand for items that bypassed
    # ``pytest_collection_modifyitems`` (e.g., dynamically inserted by another
    # plugin's later hook). Defaulting to ``()`` would incorrectly drain the
    # entire open hierarchy stack for those items.
    desired = request.node.stash.get(_HIERARCHY_KEY, _STASH_MISSING)
    if desired is _STASH_MISSING:
        desired = _build_hierarchy_chain(request.node, pytestconfig)
    common = 0
    # Compare on identity (nodeid) — same-named ancestors at different paths
    # MUST stay distinct.
    while (
        common < len(_HIERARCHY_STACK)
        and common < len(desired)
        and _HIERARCHY_STACK[common][0] == desired[common][0]
    ):
        common += 1
    # Any change to the hierarchy chain orphans parametrize parents from the
    # previous test — drain them before mutating the hierarchy stack so
    # ReportContext's top-of-stack invariant holds. Strict mode: a per-frame
    # ``__exit__`` failure here signals a real upstream drift between the
    # plugin stacks and ReportContext; raise it as a test error instead of a
    # silenceable warning.
    if common < len(_HIERARCHY_STACK) or common < len(desired):
        _drain_parametrize_stack(swallow_errors=False)
    # Symmetric per-frame guard for the hierarchy pop so one bad ``__exit__``
    # doesn't leave _HIERARCHY_STACK partially drained for every subsequent test.
    while len(_HIERARCHY_STACK) > common:
        _identity, name, ns = _HIERARCHY_STACK.pop()
        _close_frame(name, ns)
    if not desired[common:]:
        return None
    # Fetch ``report_context`` lazily — but only when there's at least one
    # rendered frame to push. Pure diff-only frames (e.g. a Package frame when
    # ``sift_package_step=false``) just update _HIERARCHY_STACK with ns=None.
    rc = None
    # Roll back any partial push so a mid-loop exception doesn't leave half
    # the chain orphaned on the stack. Per-frame guard inside the rollback so
    # a failing ``__exit__`` doesn't shadow the original exception or leak
    # the remaining opened frames.
    opened: list[tuple[str, str, Any]] = []
    try:
        for identity, name, doc, rendered in desired[common:]:
            if rendered:
                if rc is None:
                    rc = request.getfixturevalue("report_context")
                ns = rc.new_step(name=name, description=doc, assertion_as_fail_not_error=False)
                ns.__enter__()
                opened.append((identity, name, ns))
            else:
                opened.append((identity, name, None))
    except BaseException:
        while opened:
            _identity, name, ns = opened.pop()
            _close_frame(name, ns)
        raise
    _HIERARCHY_STACK.extend(opened)
    return None


@pytest.fixture(autouse=True)
def _parametrize_parents(
    request: pytest.FixtureRequest,
    pytestconfig: pytest.Config,
    _hierarchy_parents: None,
) -> None:
    """Open/close shared parametrize parent steps for the current item.

    Diffs the item's desired parametrize path against the open stack: pops the
    stale tail, then opens new parents (everything except the innermost frame —
    the ``step`` fixture creates that as the leaf). Parents persist across
    sibling items so a tree like ``test_x[a=1]`` / ``test_x[a=2]`` shares one
    ``test_x`` container.

    Gated off when the current item is excluded so that excluded items don't
    eagerly request ``report_context`` (which would defeat its lazy creation),
    or when ``sift_parametrize_nesting=false``. Parents persist until the
    diff against a subsequent test's chain pops them, or until
    ``pytest_sessionfinish`` drains anything left at session end.
    """
    default = bool(_AUTOUSE.resolve(pytestconfig))
    if not _sift_enabled_for(request.node, default):
        return None
    if not _PARAMETRIZE_NESTING.resolve(pytestconfig):
        return None
    # Fall back to on-demand computation for dynamically-inserted items;
    # see _hierarchy_parents for the same rationale.
    desired = request.node.stash.get(_PARAMETRIZE_PATH_KEY, _STASH_MISSING)
    if desired is _STASH_MISSING:
        desired = _build_parametrize_path(request.node)
    parents = desired[:-1]
    common = 0
    while (
        common < len(_PARAMETRIZE_STACK)
        and common < len(parents)
        and _PARAMETRIZE_STACK[common][0] == parents[common]
    ):
        common += 1
    # Per-frame guard so one bad ``__exit__`` doesn't leave _PARAMETRIZE_STACK
    # partially drained for every subsequent test.
    while len(_PARAMETRIZE_STACK) > common:
        name, ns = _PARAMETRIZE_STACK.pop()
        _close_frame(name, ns)
    if not parents[common:]:
        return None
    rc = request.getfixturevalue("report_context")
    opened: list[tuple[str, Any]] = []
    try:
        for display in parents[common:]:
            ns = rc.new_step(name=display, assertion_as_fail_not_error=False)
            ns.__enter__()
            opened.append((display, ns))
    except BaseException:
        while opened:
            name, ns = opened.pop()
            _close_frame(name, ns)
        raise
    _PARAMETRIZE_STACK.extend(opened)
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
    default = bool(_AUTOUSE.resolve(pytestconfig))
    if not _sift_enabled_for(request.node, default):
        yield None
        return
    rc = request.getfixturevalue("report_context")
    yield from _step_impl(rc, request)


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
