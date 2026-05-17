"""Pytester-based tests for the Sift pytest plugin's configuration surface."""

from __future__ import annotations

import textwrap
from pathlib import Path

import pytest

pytest_plugins = ["pytester"]


def _probe_conftest(pytester: pytest.Pytester, probe_body: str) -> None:
    """Write a conftest that loads the plugin and runs ``probe_body`` in pytest_configure.

    ``probe_body`` is python source that runs at config time with ``config`` in scope;
    use ``print(...)`` calls and capture them via ``result.stdout.fnmatch_lines``.
    """
    pytester.makeconftest(
        'pytest_plugins = ["sift_client.pytest_plugin"]\n\n'
        "def pytest_configure(config):\n" + textwrap.indent(textwrap.dedent(probe_body), "    ")
    )


def _plugin_conftest(pytester: pytest.Pytester) -> None:
    pytester.makeconftest('pytest_plugins = ["sift_client.pytest_plugin"]')


class TestIniConfiguration:
    """`addini` keys configure the plugin via pyproject.toml / pytest.ini."""

    def test_ini_log_file_none(self, pytester: pytest.Pytester) -> None:
        _probe_conftest(
            pytester,
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

    def test_ini_log_file_path(self, pytester: pytest.Pytester, tmp_path: Path) -> None:
        log_path = tmp_path / "sift-run.jsonl"
        _probe_conftest(
            pytester,
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

    def test_ini_check_connection_true(self, pytester: pytest.Pytester) -> None:
        _probe_conftest(
            pytester,
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

    def test_ini_git_metadata_false(self, pytester: pytest.Pytester) -> None:
        _probe_conftest(
            pytester,
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

    def test_cli_overrides_ini(self, pytester: pytest.Pytester, tmp_path: Path) -> None:
        """A CLI flag takes precedence over the matching ini key."""
        cli_path = tmp_path / "cli-wins.jsonl"
        _probe_conftest(
            pytester,
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

    def test_uris_from_ini(
        self, pytester: pytest.Pytester, monkeypatch: pytest.MonkeyPatch
    ) -> None:
        """The default sift_client fixture reads URI credentials from ini when env vars are unset."""
        monkeypatch.setenv("SIFT_API_KEY", "env-key")
        monkeypatch.delenv("SIFT_GRPC_URI", raising=False)
        monkeypatch.delenv("SIFT_REST_URI", raising=False)
        _plugin_conftest(pytester)
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_grpc_uri = "ini-grpc:1234"
            sift_rest_uri = "https://ini-rest"
            sift_test_results_check_connection = true
            sift_test_results_log_file = "false"
            """
        )
        pytester.makepyfile(
            """
            def test_credentials_loaded(sift_client):
                cfg = sift_client.grpc_client._config
                assert cfg.api_key == "env-key"
                assert "ini-grpc:1234" in cfg.uri
            """
        )
        result = pytester.runpytest()
        result.assert_outcomes(passed=1)

    def test_env_var_overrides_ini_uri(
        self, pytester: pytest.Pytester, monkeypatch: pytest.MonkeyPatch
    ) -> None:
        """When both env var and ini set a URI, the env var wins."""
        monkeypatch.setenv("SIFT_API_KEY", "env-key")
        monkeypatch.setenv("SIFT_GRPC_URI", "env-grpc:9999")
        monkeypatch.delenv("SIFT_REST_URI", raising=False)
        _plugin_conftest(pytester)
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_grpc_uri = "ini-grpc:1234"
            sift_rest_uri = "https://ini-rest"
            sift_test_results_check_connection = true
            sift_test_results_log_file = "false"
            """
        )
        pytester.makepyfile(
            """
            def test_env_wins(sift_client):
                assert "env-grpc:9999" in sift_client.grpc_client._config.uri
            """
        )
        result = pytester.runpytest()
        result.assert_outcomes(passed=1)

    def test_api_key_ignored_from_ini(
        self, pytester: pytest.Pytester, monkeypatch: pytest.MonkeyPatch
    ) -> None:
        """`sift_api_key` is not registered as an ini key; the fixture refuses to use it."""
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            monkeypatch.delenv(name, raising=False)
        _plugin_conftest(pytester)
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_api_key = "should-be-ignored"
            sift_grpc_uri = "ini-grpc:1234"
            sift_rest_uri = "https://ini-rest"
            """
        )
        pytester.makepyfile("def test_should_not_run(): pass")
        result = pytester.runpytest()
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        assert "SIFT_API_KEY" in combined, combined

    def test_missing_credentials_named_in_error(
        self, pytester: pytest.Pytester, monkeypatch: pytest.MonkeyPatch
    ) -> None:
        """A missing credential aborts with all missing names listed."""
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            monkeypatch.delenv(name, raising=False)
        _plugin_conftest(pytester)
        pytester.makepyfile("def test_should_not_run(): pass")
        result = pytester.runpytest()
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            assert name in combined, combined

    def test_defaults_when_neither_set(self, pytester: pytest.Pytester) -> None:
        _probe_conftest(
            pytester,
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
