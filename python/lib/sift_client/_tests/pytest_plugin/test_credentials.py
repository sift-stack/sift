"""Tests for the default ``sift_client`` fixture's credential resolution.

Covers the env-var-then-ini fallback for URIs, the env-only handling of
``SIFT_API_KEY``, and the error path that names missing credentials.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable

if TYPE_CHECKING:
    import pytest


class TestCredentials:
    """The default ``sift_client`` fixture's resolution of env vars and ini keys."""

    def test_uris_from_ini(
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """The default sift_client fixture reads URI credentials from ini when env vars are unset."""
        monkeypatch.setenv("SIFT_API_KEY", "env-key")
        monkeypatch.delenv("SIFT_GRPC_URI", raising=False)
        monkeypatch.delenv("SIFT_REST_URI", raising=False)
        write_plugin_conftest()
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
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """When both env var and ini set a URI, the env var wins."""
        monkeypatch.setenv("SIFT_API_KEY", "env-key")
        monkeypatch.setenv("SIFT_GRPC_URI", "env-grpc:9999")
        monkeypatch.delenv("SIFT_REST_URI", raising=False)
        write_plugin_conftest()
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
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """`sift_api_key` is not registered as an ini key; the fixture refuses to use it."""
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            monkeypatch.delenv(name, raising=False)
        write_plugin_conftest()
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
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """A missing credential aborts with all missing names listed."""
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            monkeypatch.delenv(name, raising=False)
        write_plugin_conftest()
        pytester.makepyfile("def test_should_not_run(): pass")
        result = pytester.runpytest()
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            assert name in combined, combined
