"""Unit tests for the test-results log lock + offload behavior.

These cover the hang fix directly: the sidecar FileLock now has a finite
timeout, and ``log_request_to_file`` / ``_read_log_lines`` offload the blocking
lock + file I/O off the event loop internally, so a contended or stale lock can
no longer freeze every synchronous API call.
"""

from __future__ import annotations

import asyncio
import threading
from unittest.mock import MagicMock

import pytest
from filelock import FileLock
from sift.test_reports.v1.test_reports_pb2 import CreateTestReportRequest

from sift_client._internal.low_level_wrappers import _test_results_log as trl
from sift_client._internal.low_level_wrappers._test_results_log import (
    LOG_LOCK_TIMEOUT_SECONDS,
    _read_log_lines,
    log_request_to_file,
    parse_log_data_lines,
)
from sift_client._internal.low_level_wrappers.test_results import TestResultsLowLevelClient


def _lock_path(log_file) -> str:
    return str(log_file.with_name(log_file.name + ".lock"))


class TestLogLockTimeout:
    """A held lock surfaces a clear TimeoutError instead of blocking forever."""

    def test_writer_raises_timeout_when_lock_held(self, tmp_path):
        log_file = tmp_path / "log.jsonl"
        held = FileLock(_lock_path(log_file))
        held.acquire()
        try:
            with pytest.raises(TimeoutError, match="test-results log lock"):
                asyncio.run(
                    log_request_to_file(
                        log_file, "CreateTestReport", CreateTestReportRequest(), timeout=0.2
                    )
                )
        finally:
            held.release()

    def test_reader_raises_timeout_when_lock_held(self, tmp_path):
        log_file = tmp_path / "log.jsonl"
        log_file.write_text("[CreateTestReport] {}\n")
        held = FileLock(_lock_path(log_file))
        held.acquire()
        try:
            with pytest.raises(TimeoutError, match="test-results log lock"):
                asyncio.run(_read_log_lines(log_file, timeout=0.2))
        finally:
            held.release()

    def test_default_lock_timeout_is_finite_and_positive(self):
        # Guards against an accidental None/inf default that would reintroduce
        # the unbounded wait.
        assert 0 < LOG_LOCK_TIMEOUT_SECONDS < float("inf")


class TestLogParsing:
    """The split read/parse helpers preserve the original snapshot semantics."""

    def test_read_and_parse_round_trip(self, tmp_path):
        log_file = tmp_path / "log.jsonl"

        async def drive() -> list:
            await log_request_to_file(
                log_file, "CreateTestReport", CreateTestReportRequest(), response_id="r1"
            )
            await log_request_to_file(log_file, "UpdateTestReport", CreateTestReportRequest())
            return list(parse_log_data_lines(await _read_log_lines(log_file)))

        parsed = asyncio.run(drive())

        assert [p[0] for p in parsed] == ["CreateTestReport", "UpdateTestReport"]
        assert parsed[0][1] == "r1"
        assert parsed[1][1] is None

    def test_parse_skips_start_line_and_blank_lines(self):
        raw = ["[CreateTestReport] {}\n", "\n", "[UpdateTestReport] {}\n"]

        parsed = list(parse_log_data_lines(raw, start_line=1))

        assert [p[0] for p in parsed] == ["UpdateTestReport"]

    def test_parse_raises_on_malformed_line(self):
        with pytest.raises(ValueError, match="Invalid log line"):
            list(parse_log_data_lines(["not a valid log line\n"]))


class TestLogOffload:
    """The blocking log I/O runs off the loop, so a stuck lock cannot cascade."""

    def test_import_does_not_block_event_loop_while_lock_held(self, tmp_path):
        """A held lock parks the import in the executor, leaving the loop free.

        Without the offload the log read would run on the shared loop thread and
        the unrelated ``ping`` coroutine below would never complete.
        """
        log_file = tmp_path / "log.jsonl"
        log_file.write_text("")
        client = TestResultsLowLevelClient(grpc_client=MagicMock())

        loop = asyncio.new_event_loop()
        loop_thread = threading.Thread(target=loop.run_forever, daemon=True)
        loop_thread.start()

        held = FileLock(_lock_path(log_file))
        held.acquire()
        try:
            importing = asyncio.run_coroutine_threadsafe(
                client._batch_import_log_file(log_file), loop
            )

            async def ping() -> str:
                await asyncio.sleep(0)
                return "pong"

            quick = asyncio.run_coroutine_threadsafe(ping(), loop)
            assert quick.result(timeout=2.0) == "pong"
            # Still parked on the held lock in the executor, not on the loop.
            assert not importing.done()
        finally:
            held.release()

        # Once the lock frees, the read returns and the empty log raises.
        with pytest.raises(ValueError, match="No CreateTestReport"):
            importing.result(timeout=5.0)

        loop.call_soon_threadsafe(loop.stop)
        loop_thread.join(timeout=2.0)

    def test_write_and_read_run_on_dedicated_executor(self, tmp_path, monkeypatch):
        """The async wrappers run their blocking cores on the sift-log-io pool.

        Guards against a refactor moving the offload back onto the loop thread
        or the shared default pool.
        """
        seen: list[str] = []
        real_write = trl._log_request_to_file_sync
        real_read = trl._read_log_lines_sync

        def spy_write(*args, **kwargs):
            seen.append(threading.current_thread().name)
            return real_write(*args, **kwargs)

        def spy_read(*args, **kwargs):
            seen.append(threading.current_thread().name)
            return real_read(*args, **kwargs)

        monkeypatch.setattr(trl, "_log_request_to_file_sync", spy_write)
        monkeypatch.setattr(trl, "_read_log_lines_sync", spy_read)

        log_file = tmp_path / "log.jsonl"

        async def drive():
            await log_request_to_file(log_file, "CreateTestReport", CreateTestReportRequest())
            await _read_log_lines(log_file)

        asyncio.run(drive())

        assert len(seen) == 2
        assert all(name.startswith("sift-log-io") for name in seen)
