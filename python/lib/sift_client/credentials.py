"""Credential resolution shared by ``SiftClient`` and the pytest plugin.

Reads the same ``sift.toml`` profiles that ``sift-cli --profile`` uses, so an
environment configured once for the CLI is available to the Python client
without restating its endpoints. See :func:`resolve_credentials` for the
precedence order.
"""

from __future__ import annotations

from sift_client._internal.credentials import (
    CONFIG_FILE_NAME,
    ENV_CONFIG_FILE,
    ENV_PROFILE,
    ResolvedCredentials,
    config_file_path,
    resolve_credentials,
    user_config_dir,
)

__all__ = [
    "CONFIG_FILE_NAME",
    "ENV_CONFIG_FILE",
    "ENV_PROFILE",
    "ResolvedCredentials",
    "config_file_path",
    "resolve_credentials",
    "user_config_dir",
]
