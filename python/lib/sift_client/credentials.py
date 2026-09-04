"""Credential resolution shared by ``SiftClient`` and the pytest plugin.

Reads the same ``sift.toml`` profiles that ``sift-cli --profile`` uses, so an
environment configured once for the CLI is available to the Python client
without restating its endpoints. See :func:`resolve_credentials` for the
precedence order, and the Credentials & Profiles guide for the config file's
shape.

This is the public surface; the implementation lives in
``sift_client._internal.credentials``.
"""

from __future__ import annotations

from sift_client._internal.credentials import ResolvedCredentials, resolve_credentials

__all__ = [
    "ResolvedCredentials",
    "resolve_credentials",
]
