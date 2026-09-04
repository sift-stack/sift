"""Tests for credential resolution across arguments, environment, and sift.toml."""

from __future__ import annotations

import sys

import pytest

from sift_client._internal.credentials import (
    config_file_path,
    resolve_credentials,
    user_config_dir,
)
from sift_client.errors import SiftCredentialsError

CONFIG = """\
grpc_uri = "https://grpc.default.example"
rest_uri = "https://rest.default.example"
app_uri = "https://app.default.example"
apikey = "default-key"

[staging]
grpc_uri = "https://grpc.staging.example"
rest_uri = "https://rest.staging.example"
app_uri = "https://app.staging.example"
apikey = "staging-key"

[localdev]
grpc_uri = "http://localhost:50051"
rest_uri = "http://localhost:8080"
apikey = "local-key"

[partial]
grpc_uri = "https://grpc.partial.example"
"""


@pytest.fixture
def config_file(tmp_path):
    path = tmp_path / "sift.toml"
    path.write_text(CONFIG)
    return str(path)


def resolve(config_file=None, env=None, **kwargs):
    """Resolve against an isolated environment so the host's config never leaks in."""
    return resolve_credentials(config_path=config_file, env=env or {}, **kwargs)


class TestProfileSelection:
    def test_default_profile_is_top_level_table(self, config_file):
        creds = resolve(config_file)
        assert creds.grpc_url == "https://grpc.default.example"
        assert creds.api_key == "default-key"
        assert creds.profile is None
        assert creds.sources["api_key"] == "default"

    def test_named_profile(self, config_file):
        creds = resolve(config_file, profile="staging")
        assert creds.grpc_url == "https://grpc.staging.example"
        assert creds.api_key == "staging-key"
        assert creds.profile == "staging"
        assert creds.sources["api_key"] == "profile:staging"

    def test_profile_from_env(self, config_file):
        creds = resolve(config_file, env={"SIFT_PROFILE": "staging"})
        assert creds.api_key == "staging-key"
        assert creds.profile == "staging"

    def test_profile_argument_beats_env_profile(self, config_file):
        creds = resolve(config_file, profile="localdev", env={"SIFT_PROFILE": "staging"})
        assert creds.profile == "localdev"
        assert creds.api_key == "local-key"

    def test_named_profile_does_not_inherit_from_default(self, config_file):
        """A profile missing a key is an error, not a silent fall back to the default's."""
        with pytest.raises(SiftCredentialsError) as exc:
            resolve(config_file, profile="partial")
        assert "SIFT_REST_URI" in str(exc.value)
        assert "SIFT_API_KEY" in str(exc.value)

    def test_unknown_profile_lists_the_known_ones(self, config_file):
        with pytest.raises(SiftCredentialsError) as exc:
            resolve(config_file, profile="nope")
        message = str(exc.value)
        assert "'nope' was not found" in message
        assert "localdev, partial, staging" in message

    def test_unknown_profile_without_config_file_says_so(self, tmp_path):
        with pytest.raises(SiftCredentialsError) as exc:
            resolve(str(tmp_path / "absent.toml"), profile="staging")
        assert "no Sift config file was found" in str(exc.value)


class TestPrecedence:
    def test_arguments_win_over_everything(self, config_file):
        creds = resolve(
            config_file,
            profile="staging",
            env={"SIFT_API_KEY": "env-key"},
            api_key="arg-key",
        )
        assert creds.api_key == "arg-key"
        assert creds.sources["api_key"] == "arg"

    def test_named_profile_beats_environment(self, config_file):
        """Naming a profile is explicit, so it outranks an ambient env var."""
        creds = resolve(
            config_file,
            profile="staging",
            env={"SIFT_GRPC_URI": "https://grpc.env.example"},
        )
        assert creds.grpc_url == "https://grpc.staging.example"
        assert creds.sources["grpc_url"] == "profile:staging"

    def test_environment_beats_env_named_profile(self, config_file):
        """SIFT_PROFILE is ambient too, so per-field env vars still override it."""
        creds = resolve(
            config_file,
            env={"SIFT_PROFILE": "staging", "SIFT_GRPC_URI": "https://grpc.env.example"},
        )
        assert creds.grpc_url == "https://grpc.env.example"
        assert creds.sources["grpc_url"] == "env"
        assert creds.api_key == "staging-key"
        assert creds.sources["api_key"] == "profile:staging"

    def test_environment_beats_default_profile(self, config_file):
        creds = resolve(config_file, env={"SIFT_API_KEY": "env-key"})
        assert creds.api_key == "env-key"
        assert creds.sources["api_key"] == "env"
        assert creds.grpc_url == "https://grpc.default.example"

    def test_partial_environment_override_keeps_profile_fields(self, config_file):
        """The CI case: profile endpoints, key injected from a secret store."""
        creds = resolve(config_file, profile="staging", env={"SIFT_API_KEY": "ci-key"})
        assert creds.grpc_url == "https://grpc.staging.example"
        assert creds.api_key == "staging-key"

        creds = resolve(config_file, env={"SIFT_PROFILE": "staging", "SIFT_API_KEY": "ci-key"})
        assert creds.grpc_url == "https://grpc.staging.example"
        assert creds.api_key == "ci-key"

    def test_app_url_environment_name_differs_from_toml_key(self, config_file):
        creds = resolve(config_file, env={"SIFT_APP_URL": "https://app.env.example"})
        assert creds.app_url == "https://app.env.example"
        creds = resolve(config_file)
        assert creds.app_url == "https://app.default.example"

    def test_empty_values_are_treated_as_absent(self, config_file):
        creds = resolve(config_file, api_key="", env={"SIFT_API_KEY": ""})
        assert creds.api_key == "default-key"
        assert creds.sources["api_key"] == "default"


class TestUseSsl:
    def test_https_profile_uses_tls(self, config_file):
        assert resolve(config_file, profile="staging").use_ssl is True

    def test_http_profile_disables_tls(self, config_file):
        """Without this the transport would dial a plaintext port over TLS."""
        creds = resolve(config_file, profile="localdev")
        assert creds.use_ssl is False

    def test_bare_host_keeps_the_tls_default(self, config_file):
        creds = resolve(config_file, grpc_url="grpc.example:443", rest_url="rest.example")
        assert creds.use_ssl is True

    def test_mismatched_schemes_warn_and_follow_grpc(self, config_file):
        from sift_client.errors import SiftWarning

        with pytest.warns(SiftWarning, match="REST URL"):
            creds = resolve(
                config_file,
                grpc_url="http://localhost:50051",
                rest_url="https://rest.example",
            )
        assert creds.use_ssl is False


class TestMissingAndMalformed:
    def test_missing_everything_names_the_variables_and_the_fix(self, tmp_path):
        with pytest.raises(SiftCredentialsError) as exc:
            resolve(str(tmp_path / "absent.toml"))
        message = str(exc.value)
        assert "SIFT_GRPC_URI" in message
        assert "SIFT_REST_URI" in message
        assert "SIFT_API_KEY" in message
        assert "sift-cli config update" in message

    def test_missing_message_lists_available_profiles(self, config_file, tmp_path):
        stripped = tmp_path / "profiles-only.toml"
        stripped.write_text('[staging]\ngrpc_uri = "https://g.example"\n')
        with pytest.raises(SiftCredentialsError) as exc:
            resolve(str(stripped))
        assert "Named profiles in that file: staging" in str(exc.value)

    def test_require_false_leaves_fields_empty(self, tmp_path):
        creds = resolve(str(tmp_path / "absent.toml"), require=False)
        assert creds.api_key == ""
        assert creds.grpc_url == ""
        assert creds.sources["api_key"] == "unset"

    def test_malformed_toml_raises_rather_than_falling_through(self, tmp_path):
        bad = tmp_path / "sift.toml"
        bad.write_text("grpc_uri = \nnot valid")
        with pytest.raises(SiftCredentialsError, match="not valid TOML"):
            resolve(str(bad), env={"SIFT_API_KEY": "k"})

    def test_missing_file_is_not_an_error_when_env_supplies_everything(self, tmp_path):
        creds = resolve(
            str(tmp_path / "absent.toml"),
            env={
                "SIFT_API_KEY": "k",
                "SIFT_GRPC_URI": "https://g.example",
                "SIFT_REST_URI": "https://r.example",
            },
        )
        assert creds.api_key == "k"
        assert creds.sources["grpc_url"] == "env"


class TestConfigDiscovery:
    def test_explicit_path_wins(self, config_file):
        assert str(config_file_path(config_file, {})) == config_file

    def test_env_var_overrides_the_config_directory(self, tmp_path):
        target = tmp_path / "elsewhere.toml"
        found = config_file_path(None, {"SIFT_CONFIG_FILE": str(target)})
        assert found == target

    def test_cwd_is_not_searched(self, tmp_path, monkeypatch):
        """A sift.toml in a checkout must not be able to supply an API key."""
        monkeypatch.chdir(tmp_path)
        (tmp_path / "sift.toml").write_text('apikey = "from-cwd"\n')
        found = config_file_path(None, {"HOME": str(tmp_path / "home")})
        assert found != tmp_path / "sift.toml"

    @pytest.mark.skipif(sys.platform != "linux", reason="XDG layout is Linux-only")
    def test_linux_uses_xdg_config_home_when_absolute(self, tmp_path):
        assert user_config_dir({"XDG_CONFIG_HOME": str(tmp_path)}) == tmp_path

    @pytest.mark.skipif(sys.platform != "linux", reason="XDG layout is Linux-only")
    def test_linux_ignores_relative_xdg_config_home(self, tmp_path):
        found = user_config_dir({"XDG_CONFIG_HOME": "relative/path", "HOME": str(tmp_path)})
        assert found == tmp_path / ".config"

    @pytest.mark.skipif(sys.platform != "darwin", reason="macOS layout")
    def test_macos_uses_application_support(self, tmp_path):
        found = user_config_dir({"HOME": str(tmp_path)})
        assert found == tmp_path / "Library" / "Application Support"

    @pytest.mark.skipif(sys.platform != "win32", reason="Windows layout")
    def test_windows_uses_appdata(self, tmp_path):
        assert user_config_dir({"APPDATA": str(tmp_path)}) == tmp_path
