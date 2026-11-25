"""Tests for the ``import-test-result-log`` command's reporting."""

from __future__ import annotations

import sys
import warnings
from unittest.mock import MagicMock

import pytest

from sift_client.errors import SiftCredentialsError, SiftReplayReconciledWarning
from sift_client.scripts import import_test_result_log

_FLAGS = ("--grpc-url", "http://test.invalid:0", "--rest-url", "http://test.invalid")


def _run_main(monkeypatch, tmp_path, *flags: str, reconciled_count: int = 0) -> dict:
    """Run ``main`` with ``flags`` and return the arguments it gave ``SiftClient``."""
    log_file = tmp_path / "log.jsonl"
    log_file.write_text("")
    result = MagicMock(reconciled_count=reconciled_count, steps=[], measurements=[])
    result.report = None
    client = MagicMock()
    client.test_results.import_log_file.return_value = result
    captured: dict = {}

    def fake_client(**kwargs):
        captured.update(kwargs)
        return client

    monkeypatch.setattr(import_test_result_log, "SiftClient", fake_client)
    monkeypatch.setattr(import_test_result_log, "cleanup_temp_log", lambda _: None)
    monkeypatch.setattr(sys, "argv", ["import-test-result-log", str(log_file), *flags])
    import_test_result_log.main()
    return captured


def test_reconciled_measurements_are_reported_as_a_warning(monkeypatch, tmp_path) -> None:
    with pytest.warns(SiftReplayReconciledWarning, match="reconciled 2 measurement"):
        _run_main(monkeypatch, tmp_path, *_FLAGS, "--api-key", "test", reconciled_count=2)


def test_nothing_reconciled_stays_silent(monkeypatch, tmp_path) -> None:
    with warnings.catch_warnings():
        warnings.simplefilter("error", SiftReplayReconciledWarning)
        _run_main(monkeypatch, tmp_path, *_FLAGS, "--api-key", "test")


def test_profile_flag_is_passed_to_the_client(monkeypatch, tmp_path) -> None:
    kwargs = _run_main(monkeypatch, tmp_path, "--profile", "staging")
    assert kwargs["profile"] == "staging"


def test_environment_is_passed_as_explicit_arguments(monkeypatch, tmp_path) -> None:
    # Explicit arguments outrank a named profile in the resolver, so passing the
    # environment this way keeps a key that CI injects in effect, as in the plugin.
    monkeypatch.setenv("SIFT_API_KEY", "env-key")
    kwargs = _run_main(monkeypatch, tmp_path, "--profile", "staging")
    assert kwargs["api_key"] == "env-key"


def test_flag_outranks_environment(monkeypatch, tmp_path) -> None:
    monkeypatch.setenv("SIFT_API_KEY", "env-key")
    kwargs = _run_main(monkeypatch, tmp_path, "--api-key", "flag-key")
    assert kwargs["api_key"] == "flag-key"


def test_credentials_error_is_a_usage_error(monkeypatch, tmp_path, capsys) -> None:
    def failing_client(**_):
        raise SiftCredentialsError("Profile 'missing' was not found")

    monkeypatch.setattr(import_test_result_log, "SiftClient", failing_client)
    monkeypatch.setattr(sys, "argv", ["import-test-result-log", str(tmp_path / "log.jsonl")])
    with pytest.raises(SystemExit) as exc_info:
        import_test_result_log.main()
    assert exc_info.value.code == 2
    assert "Profile 'missing' was not found" in capsys.readouterr().err
