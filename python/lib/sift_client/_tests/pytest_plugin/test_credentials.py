"""Tests for the default ``sift_client`` fixture's credential resolution.

Covers the env-var-then-ini fallback for URIs, the env-only handling of
``SIFT_API_KEY``, and the error path that names missing credentials.
"""

from __future__ import annotations

from typing import Callable

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
            sift_offline = true

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
        result = pytester.runpytest_subprocess()
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
            sift_offline = true

            """
        )
        pytester.makepyfile(
            """
            def test_env_wins(sift_client):
                assert "env-grpc:9999" in sift_client.grpc_client._config.uri
            """
        )
        result = pytester.runpytest_subprocess()
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
        result = pytester.runpytest_subprocess()
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
        result = pytester.runpytest_subprocess()
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            assert name in combined, combined


_PROFILE_CONFIG = """\
grpc_uri = "https://grpc.default.example"
rest_uri = "https://rest.default.example"
apikey = "default-key"

[staging]
grpc_uri = "https://grpc.staging.example"
rest_uri = "https://rest.staging.example"
apikey = "staging-key"

[other]
grpc_uri = "https://grpc.other.example"
rest_uri = "https://rest.other.example"
apikey = "other-key"
"""


class TestProfiles:
    """The fixture's use of ``sift.toml`` profiles via ``--sift-profile``."""

    @staticmethod
    def _write_config(pytester: pytest.Pytester, monkeypatch: pytest.MonkeyPatch) -> None:
        config = pytester.path / "sift.toml"
        config.write_text(_PROFILE_CONFIG)
        monkeypatch.setenv("SIFT_CONFIG_FILE", str(config))
        for name in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI", "SIFT_PROFILE"):
            monkeypatch.delenv(name, raising=False)

    @pytest.mark.parametrize(
        ("extra_env", "cli_args", "expected_key"),
        [
            pytest.param({}, ("--sift-profile", "staging"), "staging-key", id="named-profile"),
            pytest.param({}, (), "default-key", id="default-profile"),
            pytest.param(
                {"SIFT_API_KEY": "ci-key"},
                ("--sift-profile", "staging"),
                "ci-key",
                id="env-beats-profile",
            ),
            pytest.param(
                {"SIFT_PROFILE": "staging"},
                ("--sift-profile", "other"),
                "other-key",
                id="cli-beats-env-profile",
            ),
        ],
    )
    def test_profile_selection(
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
        extra_env: dict[str, str],
        cli_args: tuple[str, ...],
        expected_key: str,
    ) -> None:
        """Which profile supplies the API key, across the surfaces that can name one.

        ``env-beats-profile`` is the plugin's deliberate difference from
        ``SiftClient``: a key injected by CI outranks the profile, so a profile
        on the runner can never silently replace it.
        """
        self._write_config(pytester, monkeypatch)
        for name, value in extra_env.items():
            monkeypatch.setenv(name, value)
        write_plugin_conftest()
        pytester.makepyfile(
            f"""
            def test_key(sift_client):
                assert sift_client.grpc_client._config.api_key == {expected_key!r}
            """
        )
        result = pytester.runpytest_subprocess(*cli_args, "--sift-offline")
        result.assert_outcomes(passed=1)

    def test_profile_from_ini_key(
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """`sift_profile` in pyproject selects the profile without a CLI flag."""
        self._write_config(pytester, monkeypatch)
        write_plugin_conftest()
        pytester.makepyprojecttoml(
            """
            [tool.pytest.ini_options]
            sift_profile = "staging"
            sift_offline = true
            """
        )
        pytester.makepyfile(
            """
            def test_from_ini_profile(sift_client):
                assert sift_client.grpc_client._config.api_key == "staging-key"
            """
        )
        result = pytester.runpytest_subprocess()
        result.assert_outcomes(passed=1)

    def test_unknown_profile_is_a_usage_error(
        self,
        pytester: pytest.Pytester,
        monkeypatch: pytest.MonkeyPatch,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """A named profile that the file does not define aborts and lists the ones it does."""
        self._write_config(pytester, monkeypatch)
        write_plugin_conftest()
        pytester.makepyfile("def test_should_not_run(): pass")
        result = pytester.runpytest_subprocess("--sift-profile", "nope", "--sift-offline")
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        assert "staging" in combined, combined
