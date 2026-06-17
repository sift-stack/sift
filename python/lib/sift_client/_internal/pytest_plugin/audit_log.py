"""DEBUG audit trace for the pytest plugin (file) plus WARNING echo (stdout).

On by default: every session attaches two handlers to the ``sift_client`` root
logger so plugin-behavior modules AND high-value SDK call sites land in one file
(in the run's ``--sift-output-dir``, or a temp dir), with warnings also echoed
to stdout. Pass ``--no-sift-audit-log`` (or set ``sift_audit_log = false``) to
turn it off. The replay subprocess gets its own sibling file via
``replay_audit_path``.

Handlers are removed at session end (``pytest_unconfigure`` ->
``detach_audit_handlers``) so a process that runs many pytest sessions — the
plugin's own test suite drives nested in-process sessions — doesn't accumulate
handlers or leak one session's file into the next.

TODO: levels are fixed (DEBUG file / WARNING stdout) and output is plain text.
A configurable level, JSON lines, or rotation are follow-ups gated on real need.
"""

from __future__ import annotations

import logging
import sys
import tempfile
from pathlib import Path
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    import pytest

ROOT_LOGGER = "sift_client"
# Columnar line for easy parsing: fixed-width timestamp, level, and namespace
# columns, then the message — which every plugin call starts with a
# left-justified ``EVENT_WIDTH`` event token followed by space-separated
# key=value fields (incl. the full test ``path=``). Fields never contain spaces
# (lists are ``/``-joined, free text is quoted), so the line tokenizes cleanly
# on whitespace then ``=``.
FILE_FORMAT = "%(asctime)s %(levelname)-7s %(namespace)-34s %(message)s"
STDOUT_FORMAT = "Sift audit %(levelname)s: %(message)s"
# Width the leading event token is padded to, so the key=value columns align.
EVENT_WIDTH = 16
# Tag so a re-entered pytest_configure (or both processes) doesn't double-attach.
HANDLER_TAG = "sift_audit"
# Field names whose values should be redacted in audit logs to avoid logging
# sensitive data. This is a best-effort blacklist; callers should avoid passing
# sensitive values in the first place.
SENSITIVE_FIELD_NAMES = frozenset(
    {
        "password",
        "passwd",
        "pwd",
        "token",
        "secret",
        "api_key",
        "apikey",
        "api-key",
        "access_token",
        "refresh_token",
        "private_key",
        "auth",
        "credential",
        "credentials",
    }
)


def _fmt_kv(key: str, value: object) -> str:
    """Format a key-value pair for audit log output.

    If ``key`` is in ``SENSITIVE_FIELD_NAMES``, the value is redacted as
    ``[REDACTED]`` to avoid logging sensitive data.
    """
    if key.lower() in SENSITIVE_FIELD_NAMES:
        value = "[REDACTED]"
    return f"{key}={value}"


def log_event(logger: logging.Logger, level: int, event: str, **fields: object) -> None:
    """Emit one columnar audit line: ``<event padded> key=value key=value``.

    Centralizes the event-token padding and value quoting so call sites read as
    ``log_event(logger, logging.DEBUG, "step.open", name=…, path=…)``. Guarded by
    ``isEnabledFor`` so nothing is formatted when audit logging is off.

    Values for field names in ``SENSITIVE_FIELD_NAMES`` are redacted as
    ``[REDACTED]`` to avoid logging sensitive data.
    """
    if not logger.isEnabledFor(level):
        return
    body = " ".join(_fmt_kv(key, value) for key, value in fields.items())
    logger.log(level, "%-*s %s", EVENT_WIDTH, event, body)


class ColumnFormatter(logging.Formatter):
    """Formatter for the columnar file log.

    Adds a ``namespace`` field (the logger name with the redundant
    ``sift_client.`` prefix trimmed) as its own aligned column, without mutating
    ``record.name`` — other handlers (e.g. pytest's log capture) see the record
    unchanged.
    """

    def format(self, record: logging.LogRecord) -> str:
        name = record.name
        record.namespace = name[len("sift_client.") :] if name.startswith("sift_client.") else name
        return super().format(record)


def replay_audit_path(main_path: Path) -> Path:
    """Sibling path for the replay subprocess: ``foo.log`` -> ``foo.replay.log``."""
    return main_path.with_suffix(".replay" + main_path.suffix)


def _make_session_dir(base: Path | None = None) -> Path:
    """Create and return ``<base>/<random>/`` for this run's artifacts.

    ``base`` is the ``--sift-output-dir`` directory when set, else
    ``<tmpdir>/sift_test_results``. Each run gets its own random subfolder (from
    ``tempfile.mkdtemp``) so repeated or concurrent runs never collide, and all
    of a run's artifacts (JSONL log, tracking sidecar, audit log, replay audit
    log) land inside it together.
    """
    parent = base if base is not None else Path(tempfile.gettempdir()) / "sift_test_results"
    parent.mkdir(parents=True, exist_ok=True)
    return Path(tempfile.mkdtemp(dir=parent, prefix=""))


def default_audit_path(session_dir: Path | None = None) -> Path:
    """A unique temp file for the default-on trace.

    When ``session_dir`` is provided the audit log is placed inside it as
    ``<session_dir.name>-audit.log`` so all session artifacts share one dir.
    Without it a standalone temp file is created (legacy / no-session-dir path).
    """
    if session_dir is not None:
        return session_dir / f"{session_dir.name}-audit.log"
    tmp = tempfile.NamedTemporaryFile(prefix="sift-audit-", suffix=".log", delete=False)
    tmp.close()
    return Path(tmp.name)


def attach_file_handler(path: Path, *, root: str = ROOT_LOGGER) -> None:
    """Attach an idempotent DEBUG FileHandler to ``root``. Shared by both processes."""
    logger = logging.getLogger(root)
    if any(
        getattr(h, HANDLER_TAG, False) and isinstance(h, logging.FileHandler)
        for h in logger.handlers
    ):
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    handler = logging.FileHandler(path, mode="w")
    handler.setLevel(logging.DEBUG)
    handler.setFormatter(ColumnFormatter(FILE_FORMAT))
    setattr(handler, HANDLER_TAG, True)
    logger.addHandler(handler)
    logger.setLevel(logging.DEBUG)
    # Stop records bubbling to the root logger so pytest's log capture (caplog,
    # "Captured log" sections, --log-cli) doesn't surface the plugin's own audit
    # trace. Our handlers are attached directly here, so they still fire.
    logger.propagate = False


def configure_audit_logging(
    config: pytest.Config, *, session_dir: Path | None = None
) -> Path | None:
    """Set up audit logging for the main pytest process. On by default.

    Returns the resolved file path (so the caller can thread the ``.replay``
    sibling to the subprocess and surface paths in the terminal summary), or
    ``None`` when audit logging is explicitly disabled.

    When ``session_dir`` is provided and the audit path is at its default
    (not explicitly set by the user), the audit log is placed inside the
    session dir so all temp artifacts land together.
    """
    from sift_client._internal.pytest_plugin.options import AUDIT_LOG_OPTION

    if not AUDIT_LOG_OPTION.resolve(config):
        return None
    path = default_audit_path(session_dir=session_dir)
    attach_file_handler(path)
    logger = logging.getLogger(ROOT_LOGGER)
    if not any(
        getattr(h, HANDLER_TAG, False)
        and isinstance(h, logging.StreamHandler)
        and not isinstance(h, logging.FileHandler)
        for h in logger.handlers
    ):
        # WARNING echo to stdout. Note: pytest captures stdout per-test, so
        # mid-test warnings land in pytest's captured-output section;
        # session-boundary warnings reach the terminal directly. The file is
        # always the complete record.
        stream = logging.StreamHandler(sys.stdout)
        stream.setLevel(logging.WARNING)
        stream.setFormatter(logging.Formatter(STDOUT_FORMAT))
        setattr(stream, HANDLER_TAG, True)
        logger.addHandler(stream)
    return path


def detach_audit_handlers(*, root: str = ROOT_LOGGER) -> None:
    """Remove and close the audit handlers; reset the logger level.

    Called from ``pytest_unconfigure`` so handlers don't outlive the session
    that created them — important when one process runs many sessions (the
    plugin's own test suite drives nested in-process pytester runs).
    """
    logger = logging.getLogger(root)
    for handler in [h for h in logger.handlers if getattr(h, HANDLER_TAG, False)]:
        handler.close()
        logger.removeHandler(handler)
    logger.setLevel(logging.NOTSET)
    logger.propagate = True


# Width the status column is right-aligned to in the rendered tree.
_TREE_WIDTH = 64


def render_report_tree(created_steps: list[Any], *, mode: str) -> str:
    """Render the final step tree with statuses — the end-state validation view.

    Reconstructs the parent/child structure from each step's dotted numeric
    ``step_path`` (``"1"`` -> ``"1.1"`` -> ``"1.1.2"``), preserving creation
    order, and renders an ASCII tree with a dotted leader to the final status.
    Failed/errored steps are annotated with the first line of their
    ``error_info`` when present, so a reader sees what went wrong inline.
    """
    header = f"Sift report tree ({mode} mode):"
    if not created_steps:
        return f"{header}\n(no steps recorded)"

    by_path = {s.step_path: s for s in created_steps}
    children: dict[str, list[Any]] = {}
    roots: list[Any] = []
    for step in created_steps:
        parent_path = step.step_path.rpartition(".")[0]
        if parent_path and parent_path in by_path:
            children.setdefault(parent_path, []).append(step)
        else:
            roots.append(step)

    lines = [header]

    def walk(step: Any, prefix: str, is_last: bool, is_root: bool) -> None:
        status = step.status.name if step.status is not None else "?"
        if is_root:
            # Roots are the trunk: no branch connector, children start at col 0.
            head = f"{step.name} "
            child_prefix = ""
        else:
            connector = "`- " if is_last else "|- "
            head = f"{prefix}{connector}{step.name} "
            child_prefix = prefix + ("   " if is_last else "|  ")
        leader = "." * max(3, _TREE_WIDTH - len(head))
        line = f"{head}{leader} {status}"
        error = getattr(step, "error_info", None)
        if status in ("FAILED", "ERROR") and error is not None and error.error_message:
            line += f"   <- {error.error_message.strip().splitlines()[0]}"
        lines.append(line)
        kids = children.get(step.step_path, [])
        for index, kid in enumerate(kids):
            walk(kid, child_prefix, index == len(kids) - 1, is_root=False)

    for index, root in enumerate(roots):
        walk(root, "", index == len(roots) - 1, is_root=True)
    return "\n".join(lines)
