"""Tests for audit log redaction of sensitive field values."""

import logging
from io import StringIO

from sift_client._internal.pytest_plugin.audit_log import (
    _fmt_kv,
    log_event,
)


def test_fmt_kv_redacts_sensitive_field_names():
    """Field names in SENSITIVE_FIELD_NAMES should be redacted."""
    assert _fmt_kv("password", "secret123") == "password=[REDACTED]"
    assert _fmt_kv("token", "token123") == "token=[REDACTED]"
    assert _fmt_kv("api_key", "key1") == "api_key=[REDACTED]"
    assert _fmt_kv("apikey", "key2") == "apikey=[REDACTED]"
    assert _fmt_kv("api-key", "key3") == "api-key=[REDACTED]"
    assert _fmt_kv("name", "secret123") == "name=secret123"  # non-sensitive


def test_log_event_redacts_sensitive_fields():
    """Sensitive field values are redacted in log output."""
    logger = logging.getLogger("sift_client.test")
    logger.setLevel(logging.DEBUG)

    stream = StringIO()
    handler = logging.StreamHandler(stream)
    handler.setLevel(logging.DEBUG)
    logger.addHandler(handler)

    try:
        log_event(
            logger,
            logging.DEBUG,
            "test.event",
            password="secret123",
            token="abc123",
            name="test",
        )
        output = stream.getvalue()
        assert "password=[REDACTED]" in output
        assert "token=[REDACTED]" in output
        assert "secret123" not in output
        assert "abc123" not in output
        assert "name=test" in output
    finally:
        logger.removeHandler(handler)


def test_log_event_case_insensitive_redaction():
    """Redaction works regardless of field name case."""
    logger = logging.getLogger("sift_client.test2")
    logger.setLevel(logging.DEBUG)

    stream = StringIO()
    handler = logging.StreamHandler(stream)
    handler.setLevel(logging.DEBUG)
    logger.addHandler(handler)

    try:
        log_event(
            logger,
            logging.DEBUG,
            "test.event",
            Password="pass1",
            TOKEN="token1",
            apiKey="key1",
            api_key="key2",
        )
        output = stream.getvalue()
        assert "Password=[REDACTED]" in output
        assert "TOKEN=[REDACTED]" in output
        assert "apiKey=[REDACTED]" in output
        assert "api_key=[REDACTED]" in output
        assert "pass1" not in output
        assert "token1" not in output
        assert "key1" not in output
        assert "key2" not in output
    finally:
        logger.removeHandler(handler)
