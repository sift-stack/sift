"""Tests for sift_types.Campaign model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import Campaign
from sift_client.sift_types.campaign import (
    CampaignCreate,
    CampaignReportSummary,
    CampaignUpdate,
)

NOW = datetime(2026, 1, 1, tzinfo=timezone.utc)


class TestCampaignReportSummary:
    """Unit tests for CampaignReportSummary."""

    def test_counts_default_to_zero(self):
        report = CampaignReportSummary(report_id="r-1")

        assert report.rule_statistics.annotations == 0
        assert report.report_name == ""

    def test_from_proto(self):
        from sift.campaigns.v1.campaigns_pb2 import CampaignReport as Proto

        report = CampaignReportSummary._from_proto(
            Proto(report_id="r-1", report_name="nightly", num_failed_rules=2)
        )

        assert report.report_id == "r-1"
        assert report.report_name == "nightly"
        assert report.rule_statistics.failed == 2


class TestCampaignCreate:
    """Unit tests for CampaignCreate - tests _to_proto_helpers."""

    def test_minimal_create(self):
        proto = CampaignCreate(name="Q1 regression").to_proto()

        assert proto.name == "Q1 regression"

    def test_client_key(self):
        proto = CampaignCreate(name="Q1", client_key="q1_2026").to_proto()

        assert proto.client_key == "q1_2026"

    def test_metadata_converter(self):
        proto = CampaignCreate(name="Q1", metadata={"owner": "ops", "n": 3.0}).to_proto()

        by_key = {m.key.name: m for m in proto.metadata}
        assert by_key["owner"].string_value == "ops"
        assert by_key["n"].number_value == 3.0


class TestCampaignUpdate:
    """Unit tests for CampaignUpdate - tests field masks."""

    def test_update_mask_only_includes_set_fields(self):
        update = CampaignUpdate(name="renamed", description="why")
        update.resource_id = "c-1"

        proto, mask = update.to_proto_with_mask()

        assert proto.campaign_id == "c-1"
        assert proto.name == "renamed"
        assert set(mask.paths) == {"name", "description"}

    def test_reports_takes_ids(self):
        """`reports` takes report IDs; the read-only summary type never appears here."""
        update = CampaignUpdate(reports=["r-1", "r-2"])
        update.resource_id = "c-1"

        proto, mask = update.to_proto_with_mask()

        assert mask.paths == []
        assert update.reports == ["r-1", "r-2"]

    def test_archive_update(self):
        update = CampaignUpdate(is_archived=True)
        update.resource_id = "c-1"

        proto, mask = update.to_proto_with_mask()

        assert proto.is_archived is True
        assert mask.paths == ["is_archived"]

    def test_requires_resource_id(self):
        with pytest.raises(ValueError, match="Resource ID must be set"):
            CampaignUpdate(name="renamed").to_proto_with_mask()


@pytest.fixture
def mock_campaign(mock_client):
    """Create a mock Campaign instance for testing."""
    campaign = Campaign(
        proto=MagicMock(),
        id_="c-1",
        name="Q1 regression",
        organization_id="org1",
        report_summaries=[
            CampaignReportSummary(report_id="r-1"),
            CampaignReportSummary(report_id="r-2"),
        ],
        tags=[],
        metadata={},
        created_date=NOW,
        modified_date=NOW,
        created_by_user_id="user1",
        modified_by_user_id="user1",
        is_archived=False,
        reports_include_summaries=False,
        description=None,
        client_key="q1_2026",
        created_from_campaign_id=None,
        archived_date=None,
    )
    campaign._apply_client_to_instance(mock_client)
    return campaign


class TestCampaign:
    """Unit tests for Campaign model - tests properties and methods."""

    def test_report_summaries(self, mock_campaign):
        assert [r.report_id for r in mock_campaign.report_summaries] == ["r-1", "r-2"]

    def test_reports_calls_client(self, mock_campaign, mock_client):
        mock_client.reports.list_.return_value = []

        _ = mock_campaign.reports

        mock_client.reports.list_.assert_called_once_with(report_ids=["r-1", "r-2"])

    def test_update_calls_client_and_updates_self(self, mock_campaign, mock_client):
        updated = MagicMock()
        mock_client.campaigns.update.return_value = updated

        with MagicMock() as mock_update:
            mock_campaign._update = mock_update

            update = CampaignUpdate(name="renamed")
            result = mock_campaign.update(update)

            mock_client.campaigns.update.assert_called_once_with(
                campaign=mock_campaign, update=update
            )
            mock_update.assert_called_once_with(updated)
            assert result is mock_campaign

    def test_archive_calls_client(self, mock_campaign, mock_client):
        mock_client.campaigns.archive.return_value = MagicMock()

        with MagicMock() as mock_update:
            mock_campaign._update = mock_update
            result = mock_campaign.archive()

            mock_client.campaigns.archive.assert_called_once_with(campaign=mock_campaign)
            assert result is mock_campaign

    def test_unarchive_calls_client(self, mock_campaign, mock_client):
        mock_client.campaigns.unarchive.return_value = MagicMock()

        with MagicMock() as mock_update:
            mock_campaign._update = mock_update
            result = mock_campaign.unarchive()

            mock_client.campaigns.unarchive.assert_called_once_with(campaign=mock_campaign)
            assert result is mock_campaign


class TestRunCreateDefaultReport:
    """A run needs a default report before it can join a campaign."""

    def test_create_default_report_reaches_the_proto(self):
        from sift_client.sift_types.run import RunCreate

        proto = RunCreate(name="r", create_default_report=True).to_proto()

        assert proto.create_default_report is True

    def test_unset_by_default(self):
        from sift_client.sift_types.run import RunCreate

        proto = RunCreate(name="r").to_proto()

        assert proto.HasField("create_default_report") is False
