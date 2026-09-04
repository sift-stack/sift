"""Resolution of Sift credentials from arguments, environment, and ``sift.toml``.

``sift-cli`` stores one or more named profiles in a ``sift.toml`` under the
user's config directory, and selects between them with ``--profile``. This
module lets the Python client read that same file, so a developer who already
runs ``sift-cli --profile staging`` gets the same endpoints from
``SiftClient(profile="staging")`` without restating them.

The resolution order, highest precedence first:

1. Inline arguments (``SiftClient(api_key=...)``), per field.
2. The fields of a profile named explicitly via ``profile=``.
3. Per-field environment variables (``SIFT_API_KEY``, ``SIFT_GRPC_URI``,
   ``SIFT_REST_URI``, ``SIFT_APP_URL``).
4. The fields of the profile named by ``SIFT_PROFILE``.
5. The default (top-level) table of the config file.

Naming a profile explicitly outranks the ambient environment variables so that
an argument beats a shell that was pointed somewhere else; ``SIFT_PROFILE``
does not, so per-field environment overrides still work in CI.

At most one profile table is ever consulted. A named profile does not inherit
missing fields from the top-level table, matching ``sift-cli``: a profile that
omits ``api_key`` is an error rather than a silent fall back to the default
profile's key.
"""

from __future__ import annotations

import os
import sys
import warnings
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Mapping
from urllib.parse import urlparse

# Shared with the ``[tool.sift]`` loader so the 3.8-3.10 ``tomli`` fallback is
# declared once.
from sift_client._internal.pyproject_config import tomllib
from sift_client.errors import SiftCredentialsError, SiftWarning

CONFIG_FILE_NAME = "sift.toml"

ENV_PROFILE = "SIFT_PROFILE"
ENV_CONFIG_FILE = "SIFT_CONFIG_FILE"
ENV_API_KEY = "SIFT_API_KEY"
ENV_GRPC_URI = "SIFT_GRPC_URI"
ENV_REST_URI = "SIFT_REST_URI"
ENV_APP_URL = "SIFT_APP_URL"

# Accepted TOML keys -> (public field name, environment variable). The first
# TOML key is canonical, the rest are accepted spellings kept so files written by
# earlier releases keep working. The env spellings are the ones the pytest plugin
# already ships; they differ from the TOML keys for the app URL and both stay.
_FIELDS = (
    (("grpc_uri",), "grpc_url", ENV_GRPC_URI),
    (("rest_uri",), "rest_url", ENV_REST_URI),
    (("app_uri",), "app_url", ENV_APP_URL),
    (("api_key", "apikey"), "api_key", ENV_API_KEY),
)

#: Every ``SIFT_*`` variable this module reads. The pytest plugin unions this
#: with its own registry so its unknown-variable warning doesn't flag one of
#: these as a typo.
CREDENTIAL_ENV_VARS = (ENV_PROFILE, ENV_CONFIG_FILE, *(env for _, _, env in _FIELDS))

_REQUIRED = ("grpc_url", "rest_url", "api_key")

_CLI_NAME = "sift-cli"


@dataclass(frozen=True)
class ResolvedCredentials:
    """Credentials resolved from arguments, environment, and config file.

    ``sources`` maps each field name to the layer that supplied it: ``"arg"``,
    ``"profile:<name>"``, ``"env"``, ``"default"``, or ``"unset"``. It answers
    "which environment am I actually talking to" without re-deriving the
    precedence by hand.
    """

    api_key: str
    grpc_url: str
    rest_url: str
    app_url: str | None
    use_ssl: bool
    profile: str | None
    sources: Mapping[str, str]


def user_config_dir(env: Mapping[str, str] | None = None) -> Path | None:
    """The directory ``sift-cli`` stores ``sift.toml`` in.

    Mirrors Rust's ``dirs::config_dir()``, which is what ``sift-cli`` uses:
    ``%APPDATA%`` on Windows, ``~/Library/Application Support`` on macOS, and
    ``$XDG_CONFIG_HOME`` (when absolute) or ``~/.config`` elsewhere. Returns
    ``None`` when the home directory cannot be determined.

    This is deliberately hand-rolled rather than delegated to ``platformdirs``,
    whose default app-name suffix would put the file somewhere ``sift-cli``
    never looks.
    """
    environ = os.environ if env is None else env

    if sys.platform == "win32":
        appdata = environ.get("APPDATA")
        return Path(appdata) if appdata else None

    if sys.platform == "darwin":
        home = _home(environ)
        return home / "Library" / "Application Support" if home else None

    xdg = environ.get("XDG_CONFIG_HOME")
    if xdg and os.path.isabs(xdg):
        return Path(xdg)
    home = _home(environ)
    return home / ".config" if home else None


def _home(environ: Mapping[str, str]) -> Path | None:
    home = environ.get("HOME") or environ.get("USERPROFILE")
    if home:
        return Path(home)
    try:
        return Path.home()
    except (RuntimeError, OSError):
        return None


def config_file_path(
    config_path: str | None = None,
    env: Mapping[str, str] | None = None,
) -> Path | None:
    """Where to look for ``sift.toml``.

    An explicit path wins, then ``SIFT_CONFIG_FILE``, then the user config
    directory. There is deliberately no search of the current working
    directory: a ``sift.toml`` inside a checkout would let a cloned repository
    supply an API key, which needs its own decision before it ships.
    """
    if config_path is not None:
        return Path(config_path)
    environ = os.environ if env is None else env
    from_env = environ.get(ENV_CONFIG_FILE)
    if from_env:
        return Path(from_env)
    base = user_config_dir(environ)
    return base / CONFIG_FILE_NAME if base else None


def _load_config(path: Path | None) -> dict[str, Any]:
    """Parse the config file, or return ``{}`` when there isn't one.

    A missing file is not an error on its own, since arguments or environment
    variables may supply everything. A file that exists but cannot be read or
    parsed does raise: silently ignoring it would surface later as a confusing
    "credentials missing" rather than the syntax error it is.
    """
    if path is None:
        return {}
    try:
        with path.open("rb") as fh:
            return tomllib.load(fh)
    except FileNotFoundError:
        return {}
    except OSError as exc:
        raise SiftCredentialsError(f"Failed to read Sift config file '{path}': {exc}") from exc
    except tomllib.TOMLDecodeError as exc:
        raise SiftCredentialsError(
            f"Sift config file '{path}' is not valid TOML: {exc}. "
            f"Run `{_CLI_NAME} config show` to inspect it."
        ) from exc


def _profile_names(config: Mapping[str, Any]) -> list[str]:
    return sorted(key for key, value in config.items() if isinstance(value, dict))


def _profile_table(
    config: Mapping[str, Any],
    name: str,
    path: Path | None,
) -> Mapping[str, Any]:
    table = config.get(name)
    if isinstance(table, dict):
        return table

    location = f"'{path}'" if path else "the Sift config file"
    if not config:
        raise SiftCredentialsError(
            f"Profile '{name}' was requested but no Sift config file was found at {location}. "
            f"Create one with `{_CLI_NAME} config create`, then "
            f"`{_CLI_NAME} config update --profile {name}`."
        )
    available = _profile_names(config)
    known = ", ".join(available) if available else "none"
    raise SiftCredentialsError(
        f"Profile '{name}' was not found in {location}. Profiles defined there: {known}. "
        f"Add it with `{_CLI_NAME} config update --profile {name}`."
    )


def _first_present(table: Mapping[str, Any], keys: tuple[str, ...]) -> Any:
    """The value of the first of ``keys`` set in ``table``, canonical spelling first."""
    for key in keys:
        value = table.get(key)
        if _str_or_none(value) is not None:
            return value
    return None


def _str_or_none(value: Any) -> str | None:
    """Coerce a layer's value, treating empty and non-string values as absent."""
    if isinstance(value, str) and value:
        return value
    return None


def _derive_use_ssl(grpc_url: str, rest_url: str) -> bool:
    """Infer transport security from the gRPC URL's scheme.

    ``sift_py`` strips the scheme off the URI and decides plaintext vs TLS from
    ``use_ssl`` alone, so a profile's ``http://localhost:50051`` would otherwise
    be dialed over TLS and fail. A bare host with no scheme keeps the TLS
    default.
    """
    grpc_scheme = urlparse(grpc_url).scheme
    rest_scheme = urlparse(rest_url).scheme
    use_ssl = grpc_scheme != "http"

    if grpc_scheme and rest_scheme and grpc_scheme != rest_scheme:
        warnings.warn(
            f"Sift gRPC URL uses '{grpc_scheme}://' but the REST URL uses "
            f"'{rest_scheme}://'. Both connections will use "
            f"{'TLS' if use_ssl else 'plaintext'}, following the gRPC URL.",
            SiftWarning,
            stacklevel=3,
        )
    return use_ssl


def _select_profile(
    profile: str | None,
    environ: Mapping[str, str],
) -> tuple[str | None, bool]:
    """The profile to read, and whether it was named explicitly rather than by env."""
    if profile:
        return profile, True
    return environ.get(ENV_PROFILE) or None, False


def resolve_credentials(
    api_key: str | None = None,
    grpc_url: str | None = None,
    rest_url: str | None = None,
    app_url: str | None = None,
    profile: str | None = None,
    config_path: str | None = None,
    env: Mapping[str, str] | None = None,
    require: bool = True,
) -> ResolvedCredentials:
    """Resolve Sift credentials across arguments, environment, and ``sift.toml``.

    Args:
        api_key: Explicit API key, overriding every other layer.
        grpc_url: Explicit gRPC endpoint, overriding every other layer.
        rest_url: Explicit REST endpoint, overriding every other layer.
        app_url: Explicit Sift web-app origin, overriding every other layer.
        profile: Name of a profile in the config file. Outranks the per-field
            environment variables; see the module docstring.
        config_path: Path to a specific config file, bypassing discovery.
        env: Environment mapping to read, defaulting to ``os.environ``.
        require: When ``True``, raise if the API key or either URL is still
            missing. Pass ``False`` to resolve as much as is available and
            leave the rest empty, as the pytest plugin's offline mode does.

    Returns:
        The resolved credentials, including which layer supplied each field.

    Raises:
        SiftCredentialsError: The config file is unreadable or malformed, the
            named profile does not exist, or (when ``require``) a required
            field could not be resolved.
    """
    environ = os.environ if env is None else env
    profile_name, profile_is_explicit = _select_profile(profile, environ)

    path = config_file_path(config_path, environ)
    config = _load_config(path)

    if profile_name is not None:
        table: Mapping[str, Any] = _profile_table(config, profile_name, path)
        file_source = f"profile:{profile_name}"
    else:
        table = config
        file_source = "default"

    # Every layer is keyed by field name, so picking a value never depends on
    # which layer it came from.
    arg_layer = {"grpc_url": grpc_url, "rest_url": rest_url, "app_url": app_url, "api_key": api_key}
    env_layer = {field: environ.get(env_key) for _, field, env_key in _FIELDS}
    file_layer = {field: _first_present(table, toml_keys) for toml_keys, field, _ in _FIELDS}

    # Highest precedence first. A profile named explicitly outranks the ambient
    # environment; one named by SIFT_PROFILE does not.
    if profile_is_explicit:
        layers = [("arg", arg_layer), (file_source, file_layer), ("env", env_layer)]
    else:
        layers = [("arg", arg_layer), ("env", env_layer), (file_source, file_layer)]

    resolved: dict[str, str] = {}
    sources: dict[str, str] = {}
    for _, field_name, _ in _FIELDS:
        for source, layer in layers:
            value = _str_or_none(layer.get(field_name))
            if value is not None:
                resolved[field_name] = value
                sources[field_name] = source
                break
        else:
            resolved[field_name] = ""
            sources[field_name] = "unset"

    if require:
        missing = [name for name in _REQUIRED if not resolved[name]]
        if missing:
            raise SiftCredentialsError(
                _missing_message(missing, profile_name, profile_is_explicit, path, config)
            )

    return ResolvedCredentials(
        api_key=resolved["api_key"],
        grpc_url=resolved["grpc_url"],
        rest_url=resolved["rest_url"],
        app_url=resolved["app_url"] or None,
        use_ssl=_derive_use_ssl(resolved["grpc_url"], resolved["rest_url"]),
        profile=profile_name,
        sources=sources,
    )


def _missing_message(
    missing: list[str],
    profile_name: str | None,
    profile_is_explicit: bool,
    path: Path | None,
    config: Mapping[str, Any],
) -> str:
    """Explain what is missing, where it was looked for, and how to supply it."""
    env_names = {field_name: env_key for _, field_name, env_key in _FIELDS}
    wanted = ", ".join(env_names[name] for name in missing)

    if profile_name is not None:
        origin = "--profile/profile=" if profile_is_explicit else ENV_PROFILE
        looked = f"profile '{profile_name}' (from {origin}) in '{path}'"
        fix = f"`{_CLI_NAME} config update --profile {profile_name}`"
    elif path is not None:
        looked = f"the default profile in '{path}'"
        fix = f"`{_CLI_NAME} config update`"
    else:
        looked = "the environment (no config file location could be determined)"
        fix = f"`{_CLI_NAME} config create`"

    lines = [
        f"Sift credentials incomplete. Missing: {wanted}.",
        f"Looked in: {looked}, then the environment.",
    ]
    if profile_name is None and (available := _profile_names(config)):
        lines.append(
            f"Named profiles in that file: {', '.join(available)}. "
            "Select one with profile=<name> or SIFT_PROFILE=<name>."
        )
    lines.append(
        f"Set them with {fix}, export {wanted}, "
        "or pass api_key/grpc_url/rest_url to SiftClient directly."
    )
    return " ".join(lines)
