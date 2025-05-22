from unittest.mock import MagicMock, patch

import pytest
from sift.reports.v1.reports_pb2 import (
    GetReportResponse,
    Report,
    ReportRuleStatus,
    ReportRuleSummary,
)
from sift_py._internal.test_util.channel import MockChannel
from sift_py.report.service import ReportService


@pytest.fixture
def mock_channel():
    return MockChannel()


@pytest.fixture
def report_service(mock_channel):
    return ReportService(mock_channel, report_id="test-report-id")


def test_wait_until_done(report_service):
    mock_summary = ReportRuleSummary(status=ReportRuleStatus.REPORT_RULE_STATUS_FINISHED)
    mock_response = GetReportResponse(report=Report(summaries=[mock_summary]))

    mock_stub = MagicMock()
    mock_stub.GetReport = MagicMock()
    mock_stub.GetReport.return_value = mock_response

    report_service._report_stub = mock_stub

    with patch("time.sleep", return_value=None):
        with patch("time.time", side_effect=[0, 1]):
            result = report_service.wait_until_done(timeout=10)

    assert result is True
    assert mock_stub.GetReport.call_count == 1


def test_wait_until_done_timeout(report_service):
    mock_summary = ReportRuleSummary(status=ReportRuleStatus.REPORT_RULE_STATUS_CREATED)
    mock_response = GetReportResponse(report=Report(summaries=[mock_summary]))

    mock_stub = MagicMock()
    mock_stub.GetReport = MagicMock()
    mock_stub.GetReport.return_value = mock_response

    report_service._report_stub = mock_stub

    with patch("time.sleep", return_value=None):
        with patch("time.time", side_effect=[0, 5, 7, 11]):
            result = report_service.wait_until_done(timeout=10)

    assert result is False
    assert mock_stub.GetReport.call_count == 2
