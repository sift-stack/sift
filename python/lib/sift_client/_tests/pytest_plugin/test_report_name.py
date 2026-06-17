"""Tests for report display-name templating.

The report ``name`` is rendered from a template set under
``[tool.sift.pytest.report] name`` and defaults to ``"{target} {timestamp}"``.
The full pytest invocation is preserved on the report's metadata under
``pytest_command``. These tests drive offline-mode inner sessions and inspect
the JSONL ``CreateTestReport`` line for the rendered values.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable

from sift_client._tests.pytest_plugin._step_status_capture import run_jsonl

if TYPE_CHECKING:
    from pathlib import Path

    import pytest


def _create_report_line(content: str) -> str:
    """Return the ``[CreateTestReport:...]`` JSONL line from a log file."""
    for line in content.splitlines():
        if line.startswith("[CreateTestReport:"):
            return line
    raise AssertionError(f"no CreateTestReport line in log:\n{content}")


class TestReportName:
    def test_toml_template(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """``[tool.sift.pytest.report] name`` renders placeholders into the report name."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            name = "TomlReport-{count}"
            """
        )
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        line = _create_report_line(log_path.read_text())
        assert '"name":"TomlReport-1"' in line, line

    def test_full_command_preserved_in_metadata(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """The full pytest invocation is stored on the report metadata."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        line = _create_report_line(log_path.read_text())
        assert '"pytest_command"' in line, line
        # The recorded command reflects the actual invocation.
        assert "--sift-offline" in line, line

    def test_git_placeholders_render_empty_outside_repo(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """Git placeholders are recognized and render empty when not in a repo.

        The inner pytester session runs in a temp dir that is not a git
        checkout, so ``{git_branch}`` resolves to an empty string rather than
        triggering the unknown-placeholder fallback.
        """
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            name = "R-{git_branch}-{count}"
            """
        )
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        combined = "\n".join(result.outlines + result.errlines)
        assert "Invalid sift_report_name template" not in combined, combined
        line = _create_report_line(log_path.read_text())
        assert '"name":"R--1"' in line, line

    def test_invalid_template_falls_back_and_warns(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """An unknown placeholder warns and falls back without aborting the session."""
        out_dir = tmp_path / "sift-out"
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.sift.pytest.report]
            name = "{nope}"
            """
        )
        pytester.makepyfile("def test_one(step): pass")
        result = pytester.runpytest_subprocess("--sift-offline", f"--sift-output-dir={out_dir}")
        log_path = run_jsonl(out_dir)
        result.assert_outcomes(passed=1)
        combined = "\n".join(result.outlines + result.errlines)
        assert "Invalid sift_report_name template" in combined, combined
        # The report is still created despite the bad template.
        _create_report_line(log_path.read_text())
