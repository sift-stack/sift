"""Tests for the plugin's terminal output (session header + report footer).

Driven through inner pytester sessions. Online output is exercised by the
``SiftClient.app_url`` unit tests (``_tests/test_urls.py``) since a live link
needs a real backend; here we cover the deterministic disabled/offline footers
and the ``-q`` suppression both share.
"""

from __future__ import annotations

from collections import Counter
from types import SimpleNamespace
from typing import TYPE_CHECKING, Callable

from sift_client._internal.low_level_wrappers._test_results_log import LogTracking
from sift_client.pytest_plugin import (
    _measurement_segments,
    _resolve_real_report_id,
    _step_count_segments,
)
from sift_client.sift_types.test_report import TestStatus

if TYPE_CHECKING:
    from pathlib import Path

    import pytest


class TestStepCountSegments:
    def test_lists_nonzero_statuses_in_order_with_color(self) -> None:
        counts = Counter({TestStatus.PASSED: 4, TestStatus.FAILED: 2, TestStatus.SKIPPED: 1})
        assert _step_count_segments(counts) == [
            ("4 passed", {"green": True}),
            ("2 failed", {"red": True}),
            ("1 skipped", {"yellow": True}),
        ]

    def test_error_and_aborted_are_red(self) -> None:
        counts = Counter({TestStatus.ERROR: 1, TestStatus.ABORTED: 1})
        assert _step_count_segments(counts) == [
            ("1 error", {"red": True}),
            ("1 aborted", {"red": True}),
        ]

    def test_empty_is_empty(self) -> None:
        assert _step_count_segments(Counter()) == []


class TestMeasurementSegments:
    def test_passed_green_failed_red(self) -> None:
        assert _measurement_segments(Counter({True: 2, False: 1})) == [
            ("2 passed", {"green": True}),
            ("1 failed", {"red": True}),
        ]

    def test_empty_is_empty(self) -> None:
        assert _measurement_segments(Counter()) == []


class TestResolveRealReportId:
    """``_resolve_real_report_id`` maps the footer to the real server report id."""

    def test_synchronous_online_uses_report_id_directly(self) -> None:
        # No log file, non-simulated report (``--sift-log-file=false`` path).
        context = SimpleNamespace(
            report=SimpleNamespace(id_="real-123", is_simulated=False),
            log_file=None,
        )
        assert _resolve_real_report_id(context) == "real-123"

    def test_incremental_resolves_via_sidecar(self, tmp_path: Path) -> None:
        log_file = tmp_path / "run.jsonl"
        log_file.write_text("")
        LogTracking(id_map={"sim-1": "real-1"}).save(log_file)
        context = SimpleNamespace(
            report=SimpleNamespace(id_="sim-1", is_simulated=True),
            log_file=log_file,
        )
        assert _resolve_real_report_id(context) == "real-1"

    def test_empty_report_id_returns_none(self) -> None:
        # An unset/empty id must not produce a ``/test-results/`` link.
        context = SimpleNamespace(
            report=SimpleNamespace(id_="", is_simulated=False),
            log_file=None,
        )
        assert _resolve_real_report_id(context) is None

    def test_incremental_unmapped_returns_none(self, tmp_path: Path) -> None:
        # Worker died before mapping the report: no sidecar entry.
        log_file = tmp_path / "run.jsonl"
        log_file.write_text("")
        context = SimpleNamespace(
            report=SimpleNamespace(id_="sim-1", is_simulated=True),
            log_file=log_file,
        )
        assert _resolve_real_report_id(context) is None


class TestHeader:
    def test_header_shows_version_and_mode(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """The session header reports the SDK version and the active mode."""
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(step): step.measure(name='v', value=1.0)")
        result = pytester.runpytest_subprocess("--sift-disabled")
        result.assert_outcomes(passed=1)
        result.stdout.fnmatch_lines(["*sift-stack-py*disabled mode*"])

    def test_header_suppressed_under_quiet(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """``-q`` suppresses the header, matching pytest's own platform header."""
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(step): step.measure(name='v', value=1.0)")
        result = pytester.runpytest_subprocess("--sift-disabled", "-q")
        result.assert_outcomes(passed=1)
        result.stdout.no_fnmatch_line("*sift-stack-py*")


class TestDisabledFooter:
    def test_footer_notes_no_report(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(step): step.measure(name='v', value=1.0)")
        result = pytester.runpytest_subprocess("--sift-disabled")
        result.assert_outcomes(passed=1)
        result.stdout.fnmatch_lines(["*Sift disabled*no test report created*"])

    def test_footer_suppressed_under_quiet(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(step): step.measure(name='v', value=1.0)")
        result = pytester.runpytest_subprocess("--sift-disabled", "-q")
        result.assert_outcomes(passed=1)
        result.stdout.no_fnmatch_line("*Sift disabled*")


class TestOfflineFooter:
    def test_footer_shows_log_path_and_replay_command(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Offline footer points at the saved log file and the replay command."""
        log_path = tmp_path / "run.jsonl"
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(step): step.measure(name='v', value=1.0)")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-log-file={log_path}")
        result.assert_outcomes(passed=1)
        result.stdout.fnmatch_lines(
            [
                "*Test case*",
                "*Status*offline*not uploaded*",
                "*Steps*passed*",
                "*Measurements*1 passed*",
                "*System*",
                f"*Log file*{log_path}",
                "*to upload to Sift*",
                f"*import-test-result-log {log_path}",
            ]
        )

    def test_sift_open_report_flag_is_accepted_offline(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """``--sift-open-report`` is a no-op offline (no resolvable URL) and never errors."""
        log_path = tmp_path / "run.jsonl"
        write_plugin_conftest()
        pytester.makepyfile("def test_runs(step): step.measure(name='v', value=1.0)")
        result = pytester.runpytest_subprocess(
            "--sift-offline", f"--sift-log-file={log_path}", "--sift-open-report"
        )
        result.assert_outcomes(passed=1)
