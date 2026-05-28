"""Helpers for turning Sift API endpoints into web-app (frontend) URLs.

The Sift frontend can be hosted on several domains and the backend exposes no
field for its own URL, so the frontend origin is derived client-side from the
API host. This table mirrors the canonical mapping used by the Grafana
datasource (sift-stack/sift-grafana-datasource,
``src/components/sharelink/getFrontendHostnameDefaults.ts``). Hosts outside the
table (on-prem and custom deployments) require an explicit override.
"""

from __future__ import annotations

from urllib.parse import urlparse

# API host (host[:port], no scheme) -> frontend origin (with scheme).
_API_HOST_TO_FRONTEND_ORIGIN: dict[str, str] = {
    "api.siftstack.com": "https://app.siftstack.com",
    "gov.api.siftstack.com": "https://gov.siftstack.com",
}


def _origin(url: str) -> str:
    """Normalize a URL or bare host into a ``scheme://host[:port]`` origin.

    Bare hosts (no scheme) are assumed to be ``https``.
    """
    candidate = url if "://" in url else f"https://{url}"
    parsed = urlparse(candidate)
    return f"{parsed.scheme}://{parsed.netloc}".rstrip("/")


def _host(url: str) -> str:
    """Extract ``host[:port]`` from a URL or bare host string."""
    candidate = url if "://" in url else f"https://{url}"
    return urlparse(candidate).netloc


def frontend_origin_for_api(api_base_url: str, override: str | None = None) -> str | None:
    """Return the Sift web-app origin for a given API base URL.

    Args:
        api_base_url: The REST API base URL (e.g. ``https://api.siftstack.com``).
        override: An explicit frontend origin (host or full URL) to use instead
            of the derived value. Set this for on-prem or custom deployments
            whose API host isn't in the built-in mapping.

    Returns:
        The frontend origin (e.g. ``https://app.siftstack.com``), or ``None``
        when no override is given and the API host isn't recognized.
    """
    if override:
        return _origin(override)
    if not api_base_url:
        return None
    return _API_HOST_TO_FRONTEND_ORIGIN.get(_host(api_base_url))
