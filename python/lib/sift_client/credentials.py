"""Credential resolution shared by ``SiftClient`` and the pytest plugin.

This module reads the same ``sift.toml`` profiles that ``sift-cli --profile``
uses. An environment that you configure once for the CLI is then available to
the Python client, and you do not restate its endpoints. See
:func:`resolve_credentials` for the precedence order. The Credentials and
profiles guide describes the config file.

This module is the public surface. The implementation is in
``sift_client._internal.credentials``.
"""

from __future__ import annotations

from sift_client._internal.credentials import ResolvedCredentials, resolve_credentials

__all__ = [
    "ResolvedCredentials",
    "resolve_credentials",
]
