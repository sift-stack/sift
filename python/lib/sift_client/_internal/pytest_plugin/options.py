"""Declarative settings registry for the Sift pytest plugin.

Every plugin setting is declared once as an :class:`Option` in the ``PLUGIN_OPTIONS``
registry. That single registry drives ``pytest_addoption``, value resolution,
the docs settings-reference table, and the unknown-key typo detector, so a
setting is added or changed in one place instead of wired up across several.
"""

from __future__ import annotations

import logging
import os
import warnings
from dataclasses import dataclass
from typing import Any

import pytest

from sift_client._internal.pytest_plugin.audit_log import log_event

logger = logging.getLogger(__name__)

from sift_client._internal.pyproject_config import load_tool_sift

# Settings-reference categories. Each maps to a docs subsection and, in the
# renderer, to the column subset that category actually uses.
CAT_BEHAVIOR = "Pytest behavior"
CAT_CONNECTION = "Connection"
CAT_REPORT = "Report content"
CATEGORIES = (CAT_BEHAVIOR, CAT_CONNECTION, CAT_REPORT)

tool_sift_key = pytest.StashKey[dict]()


def tool_sift(config: pytest.Config | None) -> dict[str, Any]:
    """Session-cached ``[tool.sift]`` table.

    Every option that reads TOML, plus the typo detector, would otherwise
    re-parse pyproject.toml on the session-start path, and re-emit the
    malformed-file warning each time. Parse once per session via the config
    stash; ``load_tool_sift`` stays the uncached parser for direct callers.
    """
    if config is None:
        return {}
    cached = config.stash.get(tool_sift_key, None)
    if cached is None:
        cached = load_tool_sift(config)
        config.stash[tool_sift_key] = cached
    return cached


@dataclass(frozen=True)
class Option:
    """A single setting plus the logic to resolve it from wherever it can be set.

    A setting may come from an env var, a CLI flag, a pytest ini key, or a
    ``[tool.sift...]`` TOML path. :meth:`resolve` walks the declared surfaces in
    env > cli > ini > toml order; ``metadata`` (``merge=True``) is the one
    free-form table, resolved by :meth:`resolve_merged`. The single ``PLUGIN_OPTIONS``
    registry of these drives ``pytest_addoption``, the resolvers, the docs
    settings-reference table, and the typo detector.

    Declare only the surface fields a setting uses:

    - ``cli`` / ``cli_action``: CLI flag and argparse action (``cli_dest`` derived).
    - ``ini`` / ``ini_type`` / ``ini_default``: pytest ini key + type/default.
    - ``toml``: tuple path under ``[tool.sift...]``, e.g.
      ``("pytest", "report", "name")`` -> ``tool.sift.pytest.report.name``.
    - ``env``: full env var name, e.g. ``"SIFT_API_KEY"``.

    ``category`` groups the option in the docs reference (one of ``CATEGORIES``).
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
            raise ValueError(f"Option({self.name!r}): cli_action requires cli")
        if self.ini_type and not self.ini:
            raise ValueError(f"Option({self.name!r}): ini_type requires ini")
        if self.merge and not self.toml:
            raise ValueError(f"Option({self.name!r}): merge=True needs toml")
        if not any([self.cli, self.ini, self.toml, self.env]):
            raise ValueError(f"Option({self.name!r}): declares no surfaces")
        if self.category not in CATEGORIES:
            raise ValueError(f"Option({self.name!r}): category must be one of {CATEGORIES}")

    def resolve(self, config: pytest.Config | None) -> Any:
        """First set value from declared surfaces; ``None`` when unset everywhere.

        Walk order is env > cli > ini > toml. No current option declares both
        env and cli, so the chain isn't ambiguous in practice.
        ``getini`` returns the typed default for unset bool/list keys, so this
        only returns ini values for booleans (always meaningful), non-empty
        strings, and non-empty lists.
        """
        return self.resolve_with_source(config)[0]

    def resolve_with_source(self, config: pytest.Config | None) -> tuple[Any, str]:
        """Like :meth:`resolve`, but also reports which surface set the value.

        Returns ``(value, source)`` where ``source`` is one of
        ``env``/``cli``/``ini``/``toml``, or ``default`` when nothing set it
        (``value`` is then ``None``). Used by the audit log's settings snapshot.
        """
        if self.env:
            env_value = os.getenv(self.env)
            if env_value not in (None, ""):
                return env_value, "env"
        if config is None:
            return None, "default"
        if self.cli:
            cli_value = config.getoption(self.cli_dest, default=None)
            if cli_value is not None:
                return cli_value, "cli"
        if self.ini:
            try:
                ini_value = config.getini(self.ini)
            except (KeyError, ValueError):
                ini_value = None
            if isinstance(ini_value, bool):
                return ini_value, "ini"
            if isinstance(ini_value, str) and ini_value:
                return ini_value, "ini"
            if isinstance(ini_value, list) and ini_value:
                return ini_value, "ini"
        if self.toml:
            toml_value = _walk_toml(tool_sift(config), self.toml)
            if toml_value not in (None, ""):
                return toml_value, "toml"
        return None, "default"

    def resolve_merged(self, config: pytest.Config | None) -> dict[str, str | float | bool]:
        """For ``merge=True`` dict-shape settings: the free-form TOML table.

        TOML values that don't fit ``dict[str, str | float | bool]`` (nested
        tables, lists, ``None``) are dropped with a warning so a malformed
        entry can't crash report creation.
        """
        from sift_client.pytest_plugin import SiftPytestPluginWarning

        result: dict[str, str | float | bool] = {}
        if config is not None and self.toml:
            base = _walk_toml(tool_sift(config), self.toml)
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
LOG_FILE_OPTION = Option(
    name="log_file",
    category=CAT_BEHAVIOR,
    help="Path to the JSONL log of create/update calls (path | true | false | none).",
    cli="--sift-log-file",
    ini="sift_log_file",
)
AUDIT_LOG_OPTION = Option(
    name="audit_log",
    category=CAT_BEHAVIOR,
    help="DEBUG-level audit trace of plugin behavior (path | true | false). On by "
    "default to a temp file, with warnings echoed to stdout; set a path to pin the "
    "file, or false to disable.",
    cli="--sift-audit-log",
    ini="sift_audit_log",
)
GIT_METADATA_OPTION = Option(
    name="git_metadata",
    category=CAT_BEHAVIOR,
    help="Capture git repo/branch/commit on the report.",
    cli="--no-sift-git-metadata",
    cli_action="store_false",
    ini="sift_git_metadata",
    ini_type="bool",
    ini_default=True,
)
OFFLINE_OPTION = Option(
    name="offline",
    category=CAT_BEHAVIOR,
    help="Skip the session-start ping; route create/update through the JSONL log.",
    cli="--sift-offline",
    cli_action="store_true",
    ini="sift_offline",
    ini_type="bool",
    ini_default=False,
)
DISABLED_OPTION = Option(
    name="disabled",
    category=CAT_BEHAVIOR,
    help="Disable Sift entirely (no API calls, no log file). Supersedes --sift-offline.",
    cli="--sift-disabled",
    cli_action="store_true",
    ini="sift_disabled",
    ini_type="bool",
    ini_default=False,
)

OPEN_OPTION = Option(
    name="open_report",
    category=CAT_BEHAVIOR,
    help="Open the resulting report in a browser at session end (online only; "
    "no-op when the report URL can't be resolved).",
    cli="--sift-open-report",
    cli_action="store_true",
    ini="sift_open_report",
    ini_type="bool",
    ini_default=False,
)

# Pytest behavior: set-once project defaults (no CLI flag, no per-run override).
AUTOUSE_OPTION = Option(
    name="autouse",
    category=CAT_BEHAVIOR,
    help="Default for the Sift autouse fixtures (report_context, step, hierarchy/parametrize parents).",
    ini="sift_autouse",
    ini_type="bool",
    ini_default=True,
)
PACKAGE_STEP_OPTION = Option(
    name="package_step",
    category=CAT_BEHAVIOR,
    help="Open a parent step for each Python package in the test path.",
    ini="sift_package_step",
    ini_type="bool",
    ini_default=True,
)
MODULE_STEP_OPTION = Option(
    name="module_step",
    category=CAT_BEHAVIOR,
    help="Open a parent step for each test module.",
    ini="sift_module_step",
    ini_type="bool",
    ini_default=True,
)
CLASS_STEP_OPTION = Option(
    name="class_step",
    category=CAT_BEHAVIOR,
    help="Open per-class parent steps, including nested classes.",
    ini="sift_class_step",
    ini_type="bool",
    ini_default=True,
)
PARAMETRIZE_NESTING_OPTION = Option(
    name="parametrize_nesting",
    category=CAT_BEHAVIOR,
    help="Cluster parametrized tests under shared parent steps (e.g. test_a -> v=1, v=2).",
    ini="sift_parametrize_nesting",
    ini_type="bool",
    ini_default=True,
)

# Credentials. The API key is env-only; the URIs accept env + ini.
API_KEY_OPTION = Option(
    name="api_key",
    category=CAT_CONNECTION,
    help="Sift API key (secret, env-only).",
    env="SIFT_API_KEY",
)
GRPC_URI_OPTION = Option(
    name="grpc_uri",
    category=CAT_CONNECTION,
    help="Sift gRPC endpoint URI.",
    env="SIFT_GRPC_URI",
    ini="sift_grpc_uri",
)
REST_URI_OPTION = Option(
    name="rest_uri",
    category=CAT_CONNECTION,
    help="Sift REST endpoint URI.",
    env="SIFT_REST_URI",
    ini="sift_rest_uri",
)
APP_URL_OPTION = Option(
    name="app_url",
    category=CAT_CONNECTION,
    help="Sift web-app origin for the report link in the terminal footer (e.g. "
    "https://app.siftstack.com). When unset, the link is derived from the REST URI "
    "for known Sift hosts.",
    env="SIFT_APP_URL",
    ini="sift_app_url",
)

# Report content. Project defaults in [tool.sift.pytest.report]; CI injects
# per-run values via SIFT_REPORT_* env vars (pytest-dotenv handles .env files
# for local dev).
REPORT_NAME_OPTION = Option(
    name="report_name",
    category=CAT_REPORT,
    help="Template for the report display name. Placeholders: {target}, {command}, {args}, "
    "{rootdir}, {timestamp}, {count}, {git_repo}, {git_branch}, {git_commit}.",
    toml=("pytest", "report", "name"),
)
TEST_CASE_OPTION = Option(
    name="test_case",
    category=CAT_REPORT,
    help="Template for the report's test_case field (same placeholders as report_name).",
    toml=("pytest", "report", "test_case"),
)
TEST_SYSTEM_NAME_OPTION = Option(
    name="test_system_name",
    category=CAT_REPORT,
    help="Name of the test system / rig. Defaults to the host's name.",
    env="SIFT_REPORT_TEST_SYSTEM_NAME",
    toml=("pytest", "report", "test_system_name"),
)
SYSTEM_OPERATOR_OPTION = Option(
    name="system_operator",
    category=CAT_REPORT,
    help="Operator running the test. Defaults to the OS user.",
    env="SIFT_REPORT_SYSTEM_OPERATOR",
    toml=("pytest", "report", "system_operator"),
)
SERIAL_NUMBER_OPTION = Option(
    name="serial_number",
    category=CAT_REPORT,
    help="Serial number of the unit under test.",
    env="SIFT_REPORT_SERIAL_NUMBER",
    toml=("pytest", "report", "serial_number"),
)
PART_NUMBER_OPTION = Option(
    name="part_number",
    category=CAT_REPORT,
    help="Part number of the unit under test.",
    env="SIFT_REPORT_PART_NUMBER",
    toml=("pytest", "report", "part_number"),
)
METADATA_OPTION = Option(
    name="metadata",
    category=CAT_REPORT,
    help="Free-form report metadata, as a TOML table of scalar values. For "
    "dynamic per-run keys, attach them in conftest via the report_context fixture.",
    toml=("pytest", "report", "metadata"),
    merge=True,
)

PLUGIN_OPTIONS: tuple[Option, ...] = (
    LOG_FILE_OPTION,
    AUDIT_LOG_OPTION,
    GIT_METADATA_OPTION,
    OFFLINE_OPTION,
    DISABLED_OPTION,
    OPEN_OPTION,
    AUTOUSE_OPTION,
    PACKAGE_STEP_OPTION,
    MODULE_STEP_OPTION,
    CLASS_STEP_OPTION,
    PARAMETRIZE_NESTING_OPTION,
    API_KEY_OPTION,
    GRPC_URI_OPTION,
    REST_URI_OPTION,
    APP_URL_OPTION,
    REPORT_NAME_OPTION,
    TEST_CASE_OPTION,
    TEST_SYSTEM_NAME_OPTION,
    SYSTEM_OPERATOR_OPTION,
    SERIAL_NUMBER_OPTION,
    PART_NUMBER_OPTION,
    METADATA_OPTION,
)


def resolved_settings(config: pytest.Config | None) -> list[tuple[str, Any, str]]:
    """Every option's resolved ``(name, value, source)`` for the audit snapshot.

    The API key is the only secret in the registry; its value is redacted to
    ``"***"`` so the snapshot is safe to write to the log file.
    """
    rows: list[tuple[str, Any, str]] = []
    for opt in PLUGIN_OPTIONS:
        value, source = opt.resolve_with_source(config)
        if opt.name == "api_key" and value:
            value = "***"
        rows.append((opt.name, value, source))
    return rows


def register_options(parser: pytest.Parser) -> None:
    """Register every option's CLI flag and ini key on the pytest parser.

    One loop drives both surfaces, so adding a setting is one entry in
    ``PLUGIN_OPTIONS``, not edits scattered across the ``pytest_addoption`` hook.
    """
    group = parser.getgroup("sift", description="Sift test results")
    for opt in PLUGIN_OPTIONS:
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


def render_settings_reference() -> str:
    """Render the Markdown settings reference from ``PLUGIN_OPTIONS``.

    One ``### <category>`` subsection per category, each table showing only the
    columns that category uses (so no dead all-``—`` columns). The plugin docs
    at ``docs/guides/pytest_plugin/configuration.md`` embed this output verbatim
    so the registry and the docs can't drift;
    ``test_settings_reference_docs_in_sync`` is the guard rail. Regenerate with::

        uv run python -c "from sift_client._internal.pytest_plugin.options import render_settings_reference; print(render_settings_reference())"
    """

    def _cli_cell(opt: Option) -> str:
        return f"`{opt.cli}`" if opt.cli else "—"

    def _ini_cell(opt: Option) -> str:
        return f"`{opt.ini}`" if opt.ini else "—"

    def _toml_cell(opt: Option) -> str:
        if not opt.toml:
            return "—"
        if opt.merge:
            return f"`[tool.sift.{'.'.join(opt.toml)}]` (table)"
        section = ".".join(opt.toml[:-1])
        return f"`[tool.sift.{section}] {opt.toml[-1]}`"

    def _env_cell(opt: Option) -> str:
        if opt.env:
            return f"`{opt.env}`"
        return "—"

    # Per-category column layout: only the surfaces that category actually uses.
    # Each column is (header, cell-renderer).
    columns_by_category = {
        CAT_BEHAVIOR: [
            ("CLI flag", _cli_cell),
            ("Ini (`[tool.pytest.ini_options]`)", _ini_cell),
        ],
        CAT_CONNECTION: [
            ("Ini (`[tool.pytest.ini_options]`)", _ini_cell),
            ("Env var", _env_cell),
        ],
        CAT_REPORT: [
            ("TOML (`[tool.sift...]`)", _toml_cell),
            ("Env var", _env_cell),
        ],
    }

    def _escape(cell: str) -> str:
        # Literal pipes inside a Markdown table cell need backslash escaping or
        # they'd be parsed as column separators.
        return cell.replace("|", "\\|")

    blocks: list[str] = []
    for category in CATEGORIES:
        opts = [o for o in PLUGIN_OPTIONS if o.category == category]
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


def warn_on_unknown_env_vars() -> None:
    """Emit a warning for any ``SIFT_*`` env var not declared in the registry.

    The registry declares each env var by its full name (``opt.env``); a
    ``SIFT_*`` var that matches none of them is almost always a typo.
    """
    import difflib

    from sift_client.pytest_plugin import SiftPytestPluginWarning

    known_full = {opt.env for opt in PLUGIN_OPTIONS if opt.env}
    suggestion_pool = sorted(known_full)
    for name in sorted(os.environ):
        if not name.startswith("SIFT_"):
            continue
        if name in known_full:
            continue
        close = difflib.get_close_matches(name, suggestion_pool, n=1, cutoff=0.6)
        hint = f" (did you mean `{close[0]}`?)" if close else ""
        log_event(
            logger,
            logging.WARNING,
            "config.unknown",
            kind="env",
            name=name,
            suggestion=close[0] if close else "-",
        )
        warnings.warn(
            f"Unknown SIFT_* env var `{name}`{hint}; ignored.",
            SiftPytestPluginWarning,
            stacklevel=2,
        )


def warn_on_unknown_toml_keys(config: pytest.Config) -> None:
    """Walk ``[tool.sift.pytest.*]`` in pyproject.toml and warn on keys outside the registry.

    Only the ``tool.sift.pytest`` subtree is checked. Other ``tool.sift.*``
    subtrees are reserved for non-pytest Sift tooling (e.g. ``tool.sift.extras``
    is consumed by this repo's extras-generation script) and aren't our
    concern. Free-form subtrees (``merge=True`` options like ``metadata``)
    stop the walk; their keys are user-defined and not validated.
    """
    import difflib

    from sift_client.pytest_plugin import SiftPytestPluginWarning

    data = tool_sift(config)
    pytest_table = (data or {}).get("pytest")
    if not isinstance(pytest_table, dict):
        return
    # Build leaf/free-form/prefix sets relative to the ``("pytest", ...)`` root
    # the registry already uses, so the walk runs on the table we just sliced.
    leaves = {opt.toml for opt in PLUGIN_OPTIONS if opt.toml and not opt.merge}
    free_form = {opt.toml for opt in PLUGIN_OPTIONS if opt.toml and opt.merge}
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
            log_event(
                logger,
                logging.WARNING,
                "config.unknown",
                kind="toml",
                name=full_name,
                suggestion=f"tool.sift.{close[0]}" if close else "-",
            )
            warnings.warn(
                f"Unknown sift config key `{full_name}`{hint}; ignored.",
                SiftPytestPluginWarning,
                stacklevel=2,
            )

    _walk(pytest_table, ("pytest",))
