"""Tests for web-app URL derivation (``_internal/urls.py`` and ``SiftClient.app_url``)."""

from __future__ import annotations

import pytest

from sift_client import SiftClient, SiftConnectionConfig
from sift_client._internal.urls import frontend_origin_for_api


class TestFrontendOriginForApi:
    @pytest.mark.parametrize(
        ("api_base_url", "expected"),
        [
            ("https://api.siftstack.com", "https://app.siftstack.com"),
            ("https://gov.api.siftstack.com", "https://gov.siftstack.com"),
            # Bare host (no scheme) resolves the same as the full URL.
            ("api.siftstack.com", "https://app.siftstack.com"),
        ],
    )
    def test_known_hosts(self, api_base_url: str, expected: str) -> None:
        assert frontend_origin_for_api(api_base_url) == expected

    def test_unknown_host_returns_none(self) -> None:
        assert frontend_origin_for_api("https://api.acme.example.com") is None

    def test_empty_returns_none(self) -> None:
        assert frontend_origin_for_api("") is None

    def test_override_wins_over_derivation(self) -> None:
        # Override applies even for a known host.
        assert (
            frontend_origin_for_api("https://api.siftstack.com", override="https://app.acme.test")
            == "https://app.acme.test"
        )

    def test_override_normalizes_bare_host(self) -> None:
        assert (
            frontend_origin_for_api("https://api.acme.example.com", override="sift.acme.test")
            == "https://sift.acme.test"
        )


class TestSiftClientAppUrl:
    def _client(self, rest_url: str, app_url: str | None = None) -> SiftClient:
        return SiftClient(
            connection_config=SiftConnectionConfig(
                api_key="k",
                grpc_url="grpc-api.siftstack.com:443",
                rest_url=rest_url,
            ),
            app_url=app_url,
        )

    def test_derives_from_known_rest_host(self) -> None:
        assert self._client("https://api.siftstack.com").app_url == "https://app.siftstack.com"

    def test_unknown_host_without_override_is_none(self) -> None:
        assert self._client("https://api.acme.example.com").app_url is None

    def test_override_used_for_unknown_host(self) -> None:
        client = self._client("https://api.acme.example.com", app_url="https://sift.acme.test")
        assert client.app_url == "https://sift.acme.test"

    def test_override_from_connection_config(self) -> None:
        client = SiftClient(
            connection_config=SiftConnectionConfig(
                api_key="k",
                grpc_url="grpc-api.siftstack.com:443",
                rest_url="https://api.acme.example.com",
                app_url="https://sift.acme.test",
            )
        )
        assert client.app_url == "https://sift.acme.test"
