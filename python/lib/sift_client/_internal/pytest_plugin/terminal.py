"""Terminal-summary formatting for the session-end Sift report panel.

Row writers and colored count/measurement segments used by
``pytest_terminal_summary``, plus the best-effort browser opener for
``--sift-open-report``. Color is dropped automatically when the terminal has no
markup (not a TTY or ``--color=no``), so captured/CI output stays plain text.
"""

from __future__ import annotations

import os
from typing import Any

from sift_client._internal.pytest_plugin.modes import mode_label, sdk_version
from sift_client.sift_types.test_report import TestStatus
from sift_client.util.test_results.context_manager import _quiet_fork_stderr

LABEL_WIDTH = 13


def sift_kv(terminalreporter: Any, label: str, value: str, **value_markup: bool) -> None:
    """Write an indented ``label  value`` row, bolding the label.

    ``value_markup`` (e.g. ``green=True``, ``cyan=True``) styles only the value.
    Color is dropped automatically when the terminal has no markup (not a TTY or
    ``--color=no``), so captured/CI output stays plain text.
    """
    terminalreporter.write("  ")
    terminalreporter.write(f"{label:<{LABEL_WIDTH}}", bold=True)
    terminalreporter.write_line(value, **value_markup)


# Step-count breakdown order and labels for the footer's "Steps" row.
STEP_COUNT_ORDER: tuple[tuple[TestStatus, str], ...] = (
    (TestStatus.PASSED, "passed"),
    (TestStatus.FAILED, "failed"),
    (TestStatus.ERROR, "error"),
    (TestStatus.ABORTED, "aborted"),
    (TestStatus.SKIPPED, "skipped"),
    (TestStatus.IN_PROGRESS, "in progress"),
)


# Per-status color for the footer's step breakdown: green pass, red
# failure/error/abort, yellow skip; in-progress (and anything else) stays plain.
STEP_STATUS_MARKUP: dict[TestStatus, dict[str, bool]] = {
    TestStatus.PASSED: {"green": True},
    TestStatus.FAILED: {"red": True},
    TestStatus.ERROR: {"red": True},
    TestStatus.ABORTED: {"red": True},
    TestStatus.SKIPPED: {"yellow": True},
}


def step_count_segments(counts: Any) -> list[tuple[str, dict[str, bool]]]:
    """Build ``(text, markup)`` segments for a step tally, non-zero only."""
    return [
        (f"{counts.get(status, 0)} {label}", STEP_STATUS_MARKUP.get(status, {}))
        for status, label in STEP_COUNT_ORDER
        if counts.get(status, 0)
    ]


def measurement_segments(counts: Any) -> list[tuple[str, dict[str, bool]]]:
    """Build ``(text, markup)`` segments for a measurement tally, non-zero only."""
    segments: list[tuple[str, dict[str, bool]]] = []
    if counts.get(True, 0):
        segments.append((f"{counts[True]} passed", {"green": True}))
    if counts.get(False, 0):
        segments.append((f"{counts[False]} failed", {"red": True}))
    return segments


def write_count_row(
    terminalreporter: Any, label: str, segments: list[tuple[str, dict[str, bool]]]
) -> None:
    """Write a ``label  a · b · c`` row, applying each segment's color markup."""
    terminalreporter.write("  ")
    terminalreporter.write(f"{label:<{LABEL_WIDTH}}", bold=True)
    for index, (text, markup) in enumerate(segments):
        if index:
            terminalreporter.write(" · ")
        terminalreporter.write(text, **markup)
    terminalreporter.write_line("")


def report_panel_title(report: Any, terminalreporter: Any) -> str:
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


def maybe_open_report(url: str) -> None:
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


def write_disabled_summary(terminalreporter: Any) -> None:
    """Print the one-line panel shown in ``--sift-disabled`` mode."""
    terminalreporter.write_sep("=", "Sift", cyan=True, bold=True)
    terminalreporter.write_line("Sift disabled — no test report created.")


def write_report_summary(
    terminalreporter: Any,
    context: Any,
    config: Any,
    report_id: str | None,
    report_url: str | None,
    offline: bool,
) -> None:
    """Print the session-end report panel: outcome, tallies, provenance, action.

    ``report_id`` / ``report_url`` come from ``resolve_report_link``. The action
    row is a clickable link (online), the upload command (offline), or a replay
    hint when the report never uploaded.
    """
    log_file = getattr(context, "log_file", None)

    failed = bool(getattr(context, "any_failures", False))
    status_word, status_markup = (
        ("FAILED", {"red": True, "bold": True})
        if failed
        else ("PASSED", {"green": True, "bold": True})
    )
    # Offline results live only in the local log until replayed, so the status
    # row calls that out instead of repeating the version (already in the header).
    status_context = (
        f"{mode_label(config)} · not uploaded"
        if offline
        else f"{mode_label(config)} · sift-stack-py {sdk_version()}"
    )

    report = context.report

    terminalreporter.write_sep(
        "=", report_panel_title(report, terminalreporter), cyan=True, bold=True
    )

    # Identity row: the test case (test path or pytest invocation).
    if report.test_case:
        sift_kv(terminalreporter, "Test case", str(report.test_case))

    # Status row: colored outcome, then compact mode context.
    terminalreporter.write("  ")
    terminalreporter.write(f"{'Status':<{LABEL_WIDTH}}", bold=True)
    terminalreporter.write(status_word, **status_markup)
    terminalreporter.write_line(f"      {status_context}")

    # Step + measurement tallies (green pass, red failure, yellow skip).
    write_count_row(
        terminalreporter,
        "Steps",
        step_count_segments(context.step_status_counts) or [("no steps", {})],
    )
    measurements = measurement_segments(context.measurement_counts)
    if measurements:
        write_count_row(terminalreporter, "Measurements", measurements)

    # Provenance row: test system and operator.
    system = " · ".join(part for part in (report.test_system_name, report.system_operator) if part)
    if system:
        sift_kv(terminalreporter, "System", system)

    # Local log file (write-through backup online, sole sink offline).
    if log_file is not None:
        sift_kv(terminalreporter, "Log file", str(log_file))

    if offline:
        if log_file is not None:
            terminalreporter.write_sep("-", "to upload to Sift")
            terminalreporter.write_line(f"  >> import-test-result-log {log_file}", cyan=True)
    else:
        if not report_id:
            # Incremental upload never mapped the report (the worker died before
            # replaying the create), so there's no real report to link.
            sift_kv(
                terminalreporter,
                "Report",
                f"not uploaded — replay with: import-test-result-log {log_file}",
                yellow=True,
            )
        elif report_url is not None:
            sift_kv(terminalreporter, "Report", report_url, cyan=True)
        else:
            sift_kv(
                terminalreporter,
                "Report",
                f"id {report_id}  (set sift_app_url for a clickable link)",
            )

        if report_id and getattr(context, "replay_incomplete", False) and log_file is not None:
            sift_kv(
                terminalreporter,
                "",
                f"may be incomplete — finish with: import-test-result-log {log_file}",
                yellow=True,
            )

    # Audit log: its own section after the upload/report block. The main-process
    # trace plus, online, the replay worker's sibling file.
    audit_log = getattr(context, "audit_log", None)
    if audit_log is not None:
        from sift_client._internal.pytest_plugin.audit_log import replay_audit_path

        terminalreporter.write_sep("-", "audit log")
        sift_kv(terminalreporter, "File", str(audit_log))
        if not offline:
            sift_kv(terminalreporter, "Replay", str(replay_audit_path(audit_log)))
