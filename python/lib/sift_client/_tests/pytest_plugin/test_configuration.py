"""Tests for the plugin's CLI/ini configuration surface.

Covers flag parsing, ini-key resolution, CLI-over-ini precedence, and the
defaults that apply when nothing is set. Credentials are tested in
``test_credentials.py``.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable

if TYPE_CHECKING:
    from pathlib import Path

    import pytest


class TestIniConfiguration:
    """`addini` keys configure the plugin via pyproject.toml / pytest.ini."""

    def test_ini_log_file_none(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _resolve_log_file
            print("RESOLVED:", _resolve_log_file(config))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_test_results_log_file = "none"
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest("-s", "--co")
        result.stdout.fnmatch_lines(["RESOLVED: None"])

    def test_ini_log_file_path(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        log_path = tmp_path / "sift-run.jsonl"
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _resolve_log_file
            print("RESOLVED:", _resolve_log_file(config))
            """,
        )
        pytester.makepyprojecttoml(
            f"""
            [tool.pytest.ini_options]
            sift_test_results_log_file = "{log_path}"
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest("-s", "--co")
        result.stdout.fnmatch_lines([f"RESOLVED: {log_path}"])

    def test_ini_check_connection_true(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _check_connection_enabled
            print("CHECK:", _check_connection_enabled(config))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_test_results_check_connection = true
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest("-s", "--co")
        result.stdout.fnmatch_lines(["CHECK: True"])

    def test_ini_git_metadata_false(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            print("INI_GIT:", config.getini("sift_test_results_git_metadata"))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_test_results_git_metadata = false
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest("-s", "--co")
        result.stdout.fnmatch_lines(["INI_GIT: False"])

    def test_cli_overrides_ini(
        self,
        pytester: pytest.Pytester,
        tmp_path: Path,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """A CLI flag takes precedence over the matching ini key."""
        cli_path = tmp_path / "cli-wins.jsonl"
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _resolve_log_file
            print("RESOLVED:", _resolve_log_file(config))
            """,
        )
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_test_results_log_file = "none"
            """
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest("-s", "--co", f"--sift-test-results-log-file={cli_path}")
        result.stdout.fnmatch_lines([f"RESOLVED: {cli_path}"])

    def test_cli_check_connection_flag(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """The ``--sift-test-results-check-connection`` CLI flag flips the resolver to True."""
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import _check_connection_enabled
            print("CHECK:", _check_connection_enabled(config))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest("-s", "--co", "--sift-test-results-check-connection")
        result.stdout.fnmatch_lines(["CHECK: True"])

    def test_cli_no_git_metadata_flag(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        """The ``--no-sift-test-results-git-metadata`` CLI flag flips git_metadata to False.

        Guards the negation flag's ``dest`` binding: the flag name doesn't match
        the ini key, so a broken ``dest`` would silently fall back to the ini
        default and pass every other test in this file.
        """
        write_probe_conftest(
            """
            print("CLI_GIT:", config.getoption("sift_test_results_git_metadata"))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest("-s", "--co", "--no-sift-test-results-git-metadata")
        result.stdout.fnmatch_lines(["CLI_GIT: False"])

    def test_defaults_when_neither_set(
        self,
        pytester: pytest.Pytester,
        write_probe_conftest: Callable[[str], None],
    ) -> None:
        write_probe_conftest(
            """
            from sift_client.pytest_plugin import (
                _check_connection_enabled,
                _resolve_log_file,
            )
            print("RESOLVED:", _resolve_log_file(config))
            print("CHECK:", _check_connection_enabled(config))
            print("INI_GIT:", config.getini("sift_test_results_git_metadata"))
            """,
        )
        pytester.makepyfile("def test_noop(): pass")
        result = pytester.runpytest("-s", "--co")
        result.stdout.fnmatch_lines(
            [
                "RESOLVED: True",
                "CHECK: False",
                "INI_GIT: True",
            ]
        )
