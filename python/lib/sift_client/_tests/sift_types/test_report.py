"""Tests for sift_types.Report model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types.report import (
    Report,
    ReportRuleStatus,
    ReportRuleSummary,
    ReportUpdate,
)


class TestReportUpdate:
    """Unit tests for ReportUpdate model - tests _to_proto_helpers."""

    def test_metadata_converter(self):
        """Test that metadata is converted using _to_proto_helpers."""
        metadata = {"key1": "value1", "key2": 42.5, "key3": True}
        update = ReportUpdate(metadata=metadata)
        update.resource_id = "test_report_id"

        proto, mask = update.to_proto_with_mask()

        assert proto.report_id == "test_report_id"
        # Verify metadata was converted using the helper (returns a list)
        assert len(proto.metadata) == 3

        # Find each metadata value in the list
        metadata_dict = {md.key.name: md for md in proto.metadata}
        assert metadata_dict["key1"].string_value == "value1"
        assert metadata_dict["key2"].number_value == 42.5
        assert metadata_dict["key3"].boolean_value is True
        assert "metadata" in mask.paths

    def test_is_archive(self, mock_report, mock_client):
        """Test that is_archived field is properly set."""
        archived_report = MagicMock()
        archived_report.is_archived = True
        mock_client.reports.archive.return_value = archived_report
        with MagicMock() as mock_update:
            mock_report._update = mock_update
            result = mock_report.archive()
            mock_client.reports.archive.assert_called_once_with(report=mock_report)
            mock_update.assert_called_once_with(archived_report)
            assert result is mock_report

    def test_unarchive(self, mock_report, mock_client):
        """Test that unarchive() calls client.reports.unarchive and calls _update."""
        unarchived_report = MagicMock()
        unarchived_report.is_archived = False
        mock_client.reports.unarchive.return_value = unarchived_report
        with MagicMock() as mock_update:
            mock_report._update = mock_update
            result = mock_report.unarchive()
            mock_client.reports.unarchive.assert_called_once_with(report=mock_report)
            mock_update.assert_called_once_with(unarchived_report)
            assert result is mock_report

    def test_metadata_and_is_archived_update(self):
        """Test updating multiple fields at once."""
        metadata = {"key": "value"}
        update = ReportUpdate(
            metadata=metadata,
            is_archived=True,
        )
        update.resource_id = "test_report_id"

        proto, mask = update.to_proto_with_mask()

        assert proto.report_id == "test_report_id"
        assert proto.is_archived is True
        assert len(proto.metadata) == 1
        assert "is_archived" in mask.paths
        assert "metadata" in mask.paths


@pytest.fixture
def mock_report_rule_summary():
    """Create a mock ReportRuleSummary instance for testing."""
    return ReportRuleSummary(
        id_="summary_id",
        rule_id="rule1",
        rule_client_key="rule_key",
        rule_version_id="version1",
        rule_version_number=1,
        report_rule_version_id="report_version1",
        num_open=5,
        num_failed=2,
        num_passed=3,
        status=ReportRuleStatus.FINISHED,
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        asset_id="asset1",
        deleted_date=None,
    )


@pytest.fixture
def mock_report(mock_client, mock_report_rule_summary):
    """Create a mock Report instance for testing."""
    report = Report(
        proto=MagicMock(),
        id_="test_report_id",
        report_template_id="template1",
        run_id="run1",
        organization_id="org1",
        name="test_report",
        description="test description",
        created_by_user_id="user1",
        modified_by_user_id="user1",
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        summaries=[mock_report_rule_summary],
        tags=["tag1", "tag2"],
        rerun_from_report_id=None,
        metadata={"key": "value"},
        job_id="job1",
        archived_date=None,
        is_archived=False,
    )
    report._apply_client_to_instance(mock_client)
    return report


class TestReport:
    """Unit tests for Report model - tests properties and methods."""

    def test_report_properties(self, mock_report):
        """Test that Report properties are accessible."""
        assert mock_report.id_ == "test_report_id"
        assert mock_report.name == "test_report"
        assert mock_report.description == "test description"
        assert mock_report.report_template_id == "template1"
        assert mock_report.run_id == "run1"
        assert mock_report.organization_id == "org1"
        assert mock_report.tags == ["tag1", "tag2"]
        assert mock_report.metadata == {"key": "value"}
        assert mock_report.is_archived is False
        assert len(mock_report.summaries) == 1

    def test_report_to_proto(self, mock_report):
        """Test that Report can be converted to proto."""
        proto = mock_report.to_proto()

        assert proto.report_id == "test_report_id"
        assert proto.name == "test_report"
        assert proto.description == "test description"
        assert proto.report_template_id == "template1"
        assert proto.run_id == "run1"
        assert proto.organization_id == "org1"
        assert proto.is_archived is False
        assert len(proto.tags) == 2
        assert proto.tags[0].tag_name == "tag1"
        assert proto.tags[1].tag_name == "tag2"
        assert len(proto.summaries) == 1


class TestReportRuleSummary:
    """Unit tests for ReportRuleSummary model."""

    def test_report_rule_summary_properties(self, mock_report_rule_summary):
        """Test that ReportRuleSummary properties are accessible."""
        assert mock_report_rule_summary.id_ == "summary_id"
        assert mock_report_rule_summary.rule_id == "rule1"
        assert mock_report_rule_summary.rule_client_key == "rule_key"
        assert mock_report_rule_summary.rule_version_id == "version1"
        assert mock_report_rule_summary.rule_version_number == 1
        assert mock_report_rule_summary.report_rule_version_id == "report_version1"
        assert mock_report_rule_summary.num_open == 5
        assert mock_report_rule_summary.num_failed == 2
        assert mock_report_rule_summary.num_passed == 3
        assert mock_report_rule_summary.status == ReportRuleStatus.FINISHED
        assert mock_report_rule_summary.asset_id == "asset1"
        assert mock_report_rule_summary.deleted_date is None

    def test_report_rule_summary_to_proto(self, mock_report_rule_summary):
        """Test that ReportRuleSummary can be converted to proto."""
        proto = mock_report_rule_summary.to_proto()

        assert proto.rule_id == "rule1"
        assert proto.rule_client_key == "rule_key"
        assert proto.rule_version_id == "version1"
        assert proto.rule_version_number == 1
        assert proto.report_rule_version_id == "report_version1"
        assert proto.num_open == 5
        assert proto.num_failed == 2
        assert proto.num_passed == 3
        assert proto.status == ReportRuleStatus.FINISHED.value
        assert proto.asset_id == "asset1"
