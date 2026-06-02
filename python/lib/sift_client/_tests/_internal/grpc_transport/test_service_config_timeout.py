from __future__ import annotations

import json

from sift_client._internal.grpc_transport._retry import RetryPolicy
from sift_client._internal.grpc_transport.transport import (
    DEFAULT_REQUEST_TIMEOUT_SECONDS,
    _compute_channel_options,
)


def _service_config(options) -> dict:
    for key, value in options:
        if key == "grpc.service_config":
            return json.loads(value)
    raise AssertionError("grpc.service_config not present in channel options")


def test_default_policy_has_no_timeout():
    config = RetryPolicy.default().config
    assert "timeout" not in config["methodConfig"][0]


def test_timeout_is_formatted_as_duration():
    config = RetryPolicy.default(timeout_seconds=60.0).config
    assert config["methodConfig"][0]["timeout"] == "60s"
    # Retry policy is preserved alongside the timeout.
    assert "retryPolicy" in config["methodConfig"][0]


def test_fractional_timeout_trims_trailing_zeros():
    config = RetryPolicy.default(timeout_seconds=0.5).config
    assert config["methodConfig"][0]["timeout"] == "0.5s"


def test_channel_options_apply_default_timeout():
    options = _compute_channel_options({"uri": "x", "apikey": "y"})
    service_config = _service_config(options)
    expected = f"{int(DEFAULT_REQUEST_TIMEOUT_SECONDS)}s"
    assert service_config["methodConfig"][0]["timeout"] == expected


def test_channel_options_respect_explicit_timeout():
    options = _compute_channel_options({"uri": "x", "apikey": "y", "request_timeout": 5.0})
    service_config = _service_config(options)
    assert service_config["methodConfig"][0]["timeout"] == "5s"


def test_channel_options_omit_timeout_when_disabled():
    options = _compute_channel_options({"uri": "x", "apikey": "y", "request_timeout": None})
    service_config = _service_config(options)
    assert "timeout" not in service_config["methodConfig"][0]
