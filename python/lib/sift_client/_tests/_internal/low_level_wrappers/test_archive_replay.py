"""Replay keeps ``is_archived`` from a log written by ``archive_on_create``.

Batch replay collapses the log into one create. ``CreateTestReportRequest``
has no archive field, so the collapsed flag has to go out as a follow-up
update. Incremental replay sends the logged update as its own line.
"""

from __future__ import annotations

from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock

import pytest

from sift_client._internal.low_level_wrappers.test_results import (
    TestResultsLowLevelClient as ResultsLowLevelClient,
)
from sift_client.sift_types.test_report import (
    TestReport,
    TestReportCreate,
    TestReportUpdate,
    TestStatus,
)

T0 = datetime(2026, 1, 1, tzinfo=timezone.utc)


def _make_report(id_: str, *, is_archived: bool = False) -> TestReport:
    return TestReport(
        id_=id_,
        status=TestStatus.PASSED,
        name="n",
        test_system_name="s",
        test_case="c",
        start_time=T0,
        end_time=T0,
        metadata={},
        is_archived=is_archived,
    )


def _report_create() -> TestReportCreate:
    return TestReportCreate(
        status=TestStatus.IN_PROGRESS,
        name="n",
        test_system_name="s",
        test_case="c",
        start_time=T0,
        end_time=T0,
    )


def _install_create_spy(client: ResultsLowLevelClient) -> None:
    """Real creates return a stand-in. Simulate and log writes stay on the client."""
    original = client.create_test_report

    async def create_spy(*args, **kwargs):
        if kwargs.get("simulate") or kwargs.get("log_file") is not None:
            return await original(*args, **kwargs)
        return _make_report("real-report")

    client.create_test_report = create_spy  # type: ignore[method-assign]


async def _write_archived_log(log_file, client: ResultsLowLevelClient) -> None:
    report = await client.create_test_report(test_report=_report_create(), log_file=log_file)
    update = TestReportUpdate(is_archived=True)
    update.resource_id = report.id_
    await client.update_test_report(update=update, log_file=log_file)


@pytest.mark.asyncio
async def test_batch_replay_archives_after_create(tmp_path):
    """The default upload path archives the report it just created."""
    log_file = tmp_path / "archived.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    await _write_archived_log(log_file, client)

    original_update = client.update_test_report
    real_updates: list[TestReportUpdate] = []
    _install_create_spy(client)

    async def update_spy(*args, **kwargs):
        if kwargs.get("simulate") or kwargs.get("log_file") is not None:
            return await original_update(*args, **kwargs)
        real_updates.append(args[0])
        return _make_report("real-report", is_archived=True)

    client.update_test_report = update_spy  # type: ignore[method-assign]

    result = await client.import_log_file(log_file)

    assert len(real_updates) == 1
    assert real_updates[0].is_archived is True
    assert real_updates[0].resource_id == "real-report"
    assert result.report is not None
    assert result.report.is_archived is True


@pytest.mark.asyncio
async def test_batch_replay_skips_archive_when_unset(tmp_path):
    """A log with no archive update does not send one."""
    log_file = tmp_path / "plain.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    await client.create_test_report(test_report=_report_create(), log_file=log_file)

    client.update_test_report = AsyncMock()  # type: ignore[method-assign]
    _install_create_spy(client)

    result = await client.import_log_file(log_file)

    client.update_test_report.assert_not_called()
    assert result.report is not None
    assert result.report.is_archived is False


@pytest.mark.asyncio
async def test_incremental_replay_sends_archive_update(tmp_path):
    """Line-by-line replay forwards the logged archive update."""
    log_file = tmp_path / "incremental.jsonl"
    client = ResultsLowLevelClient(grpc_client=MagicMock())
    await _write_archived_log(log_file, client)

    archived = _make_report("real-report", is_archived=True)
    client.create_test_report = AsyncMock(return_value=_make_report("real-report"))  # type: ignore[method-assign]
    client.update_test_report = AsyncMock(return_value=archived)  # type: ignore[method-assign]

    await client.import_log_file(log_file, incremental=True)

    sent = client.update_test_report.await_args.kwargs["request"]
    assert "is_archived" in sent.update_mask.paths
    assert sent.test_report.is_archived is True
