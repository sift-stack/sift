"""Tests for the ``import-test-result-log`` command's reporting."""

from __future__ import annotations

import sys
import warnings
from unittest.mock import MagicMock

import pytest

from sift_client.errors import SiftReplayReconciledWarning
from sift_client.scripts import import_test_result_log


def _run_main(monkeypatch, tmp_path, *, reconciled_count: int) -> None:
    log_file = tmp_path / "log.jsonl"
    log_file.write_text("")
    result = MagicMock(reconciled_count=reconciled_count, steps=[], measurements=[])
    result.report = None
    client = MagicMock()
    client.test_results.import_log_file.return_value = result
    monkeypatch.setattr(import_test_result_log, "SiftClient", lambda **_: client)
    monkeypatch.setattr(import_test_result_log, "cleanup_temp_log", lambda _: None)
    monkeypatch.setattr(
        sys,
        "argv",
        [
            "import-test-result-log",
            str(log_file),
            "--grpc-url",
            "test.invalid:0",
            "--rest-url",
            "http://test.invalid",
            "--api-key",
            "test",
        ],
    )
    import_test_result_log.main()


def test_reconciled_measurements_are_reported_as_a_warning(monkeypatch, tmp_path) -> None:
    with pytest.warns(SiftReplayReconciledWarning, match="reconciled 2 measurement"):
        _run_main(monkeypatch, tmp_path, reconciled_count=2)


def test_nothing_reconciled_stays_silent(monkeypatch, tmp_path) -> None:
    with warnings.catch_warnings():
        warnings.simplefilter("error", SiftReplayReconciledWarning)
        _run_main(monkeypatch, tmp_path, reconciled_count=0)
