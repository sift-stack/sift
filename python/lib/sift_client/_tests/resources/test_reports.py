from unittest.mock import AsyncMock, MagicMock, patch

import pytest

from sift_client.resources import ReportsAPI, ReportsAPIAsync
from sift_client.sift_types import (
    ChannelReference,
    ReportRuleStatus,
    RuleAction,
    RuleAnnotationType,
)
from sift_client.sift_types.job import RuleEvaluationDetails


@pytest.fixture(scope="session")
def tags(sift_client, test_tag, ci_pytest_tag):
    tags = sift_client.tags.find_or_create(names=[test_tag.name, ci_pytest_tag.name])
    return tags


@pytest.fixture(scope="session")
def test_rule(sift_client, nostromo_asset, ci_pytest_tag):
    rule = sift_client.rules.find(name="test_rule")
    created_rule = None
    if not rule:
        created_rule = sift_client.rules.create(
            {
                "name": "test_rule",
                "description": "Test rule",
                "expression": "$1 > 0.1",
                "asset_ids": [nostromo_asset._id_or_error],
                "channel_references": [
                    ChannelReference(
                        channel_reference="$1", channel_identifier="mainmotor.velocity"
                    ),
                ],
                "action": RuleAction.annotation(
                    annotation_type=RuleAnnotationType.DATA_REVIEW,
                    tags=[ci_pytest_tag],
                ),
            },
        )
        rule = created_rule
    if rule.is_archived:
        rule = rule.unarchive()
    yield rule
    if created_rule:
        created_rule.archive()


def test_client_binding(sift_client):
    assert sift_client.reports
    assert isinstance(sift_client.reports, ReportsAPI)
    assert sift_client.async_.reports
    assert isinstance(sift_client.async_.reports, ReportsAPIAsync)


@pytest.fixture
def reports_api_async_mock_client(mock_client):
    """ReportsAPIAsync with a mock client for unit testing wait_until_complete."""
    mock_client.async_.jobs = MagicMock()
    mock_client.async_.jobs.get = AsyncMock()
    mock_client.async_.jobs.wait_until_complete = AsyncMock()
    return ReportsAPIAsync(mock_client)


class TestReportsWaitUntilComplete:
    """Unit tests for ReportsAPIAsync.wait_until_complete validation and input handling."""

    @pytest.mark.asyncio
    async def test_raises_when_neither_report_nor_job_provided(self, reports_api_async_mock_client):
        with pytest.raises(ValueError, match="either report or job must be provided"):
            await reports_api_async_mock_client.wait_until_complete()

    @pytest.mark.asyncio
    async def test_raises_when_both_report_and_job_provided(self, reports_api_async_mock_client):
        mock_report = MagicMock()
        mock_report.job_id = "job-1"
        mock_report.id_ = "report-1"
        mock_job = MagicMock()
        mock_job.id_ = "job-1"
        mock_job.job_details = RuleEvaluationDetails(report_id="report-1")

        with pytest.raises(
            ValueError, match="exactly one of report or report_job must be provided"
        ):
            await reports_api_async_mock_client.wait_until_complete(
                report=mock_report, job=mock_job
            )

    @pytest.mark.asyncio
    async def test_valid_report_object_waits_and_returns_report(
        self, reports_api_async_mock_client
    ):
        mock_report = MagicMock()
        mock_report.job_id = "job-1"
        mock_report.id_ = "report-1"
        completed_report = MagicMock()
        completed_report.id_ = "report-1"

        with patch.object(
            reports_api_async_mock_client,
            "get",
            new_callable=AsyncMock,
            return_value=completed_report,
        ) as mock_get:
            result = await reports_api_async_mock_client.wait_until_complete(report=mock_report)

        assert result is completed_report
        reports_api_async_mock_client.client.async_.jobs.wait_until_complete.assert_awaited_once_with(
            job="job-1", polling_interval_secs=5, timeout_secs=None
        )
        mock_get.assert_awaited_once_with(report_id="report-1")

    @pytest.mark.asyncio
    async def test_valid_report_id_str_fetches_report_then_waits_and_returns(
        self, reports_api_async_mock_client
    ):
        report_id = "report-1"
        mock_report_from_get = MagicMock()
        mock_report_from_get.job_id = "job-1"
        mock_report_from_get.id_ = report_id
        completed_report = MagicMock()
        completed_report.id_ = report_id

        with patch.object(
            reports_api_async_mock_client,
            "get",
            new_callable=AsyncMock,
            side_effect=[mock_report_from_get, completed_report],
        ) as mock_get:
            result = await reports_api_async_mock_client.wait_until_complete(report=report_id)

        assert result is completed_report
        assert mock_get.await_count == 2
        reports_api_async_mock_client.client.async_.jobs.wait_until_complete.assert_awaited_once_with(
            job="job-1", polling_interval_secs=5, timeout_secs=None
        )

    @pytest.mark.asyncio
    async def test_valid_job_object_rule_evaluation_waits_and_returns_report(
        self, reports_api_async_mock_client
    ):
        mock_job = MagicMock()
        mock_job.id_ = "job-1"
        mock_job.job_details = RuleEvaluationDetails(report_id="report-1")
        completed_report = MagicMock()
        completed_report.id_ = "report-1"

        with patch.object(
            reports_api_async_mock_client,
            "get",
            new_callable=AsyncMock,
            return_value=completed_report,
        ) as mock_get:
            result = await reports_api_async_mock_client.wait_until_complete(job=mock_job)

        assert result is completed_report
        reports_api_async_mock_client.client.async_.jobs.wait_until_complete.assert_awaited_once_with(
            job="job-1", polling_interval_secs=5, timeout_secs=None
        )
        mock_get.assert_awaited_once_with(report_id="report-1")

    @pytest.mark.asyncio
    async def test_valid_job_id_str_fetches_job_then_waits_and_returns_report(
        self, reports_api_async_mock_client
    ):
        job_id = "job-1"
        report_id = "report-1"
        mock_job_from_get = MagicMock()
        mock_job_from_get.id_ = job_id
        mock_job_from_get.job_details = RuleEvaluationDetails(report_id=report_id)
        reports_api_async_mock_client.client.async_.jobs.get = AsyncMock(
            return_value=mock_job_from_get
        )
        completed_report = MagicMock()
        completed_report.id_ = report_id

        with patch.object(
            reports_api_async_mock_client,
            "get",
            new_callable=AsyncMock,
            return_value=completed_report,
        ) as mock_get:
            result = await reports_api_async_mock_client.wait_until_complete(job=job_id)

        assert result is completed_report
        reports_api_async_mock_client.client.async_.jobs.get.assert_awaited_once_with(job_id)
        reports_api_async_mock_client.client.async_.jobs.wait_until_complete.assert_awaited_once_with(
            job=job_id, polling_interval_secs=5, timeout_secs=None
        )
        mock_get.assert_awaited_once_with(report_id=report_id)

    @pytest.mark.asyncio
    async def test_raises_when_job_object_not_rule_evaluation(self, reports_api_async_mock_client):
        mock_job = MagicMock()
        mock_job.id_ = "job-1"
        mock_job.job_details = None  # not RuleEvaluationDetails

        with pytest.raises(ValueError, match="job is not a rule evaluation job"):
            await reports_api_async_mock_client.wait_until_complete(job=mock_job)

        reports_api_async_mock_client.client.async_.jobs.wait_until_complete.assert_not_awaited()

    @pytest.mark.asyncio
    async def test_raises_when_job_id_str_fetches_non_rule_evaluation_job(
        self, reports_api_async_mock_client
    ):
        job_id = "job-1"
        mock_job_from_get = MagicMock()
        mock_job_from_get.id_ = job_id
        mock_job_from_get.job_details = None  # e.g. data import job
        reports_api_async_mock_client.client.async_.jobs.get = AsyncMock(
            return_value=mock_job_from_get
        )

        with pytest.raises(ValueError, match="job is not a rule evaluation job"):
            await reports_api_async_mock_client.wait_until_complete(job=job_id)

        reports_api_async_mock_client.client.async_.jobs.get.assert_awaited_once_with(job_id)
        reports_api_async_mock_client.client.async_.jobs.wait_until_complete.assert_not_awaited()


@pytest.mark.integration
class TestReports:
    def test_create_from_rules(self, nostromo_run, test_rule, sift_client):
        job = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert job is not None
        assert isinstance(job.job_details, RuleEvaluationDetails)
        report = sift_client.reports.get(report_id=job.job_details.report_id)
        assert report.run_id == nostromo_run.id_

    @pytest.mark.asyncio
    async def test_wait_until_complete(self, nostromo_run, test_rule, sift_client):
        """Create a report and wait for its job to complete via jobs.wait_until_complete."""
        job = sift_client.reports.create_from_rules(
            name="report_wait_until_complete",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert job is not None
        assert job.id_
        assert isinstance(job.job_details, RuleEvaluationDetails)

        await sift_client.async_.jobs.wait_until_complete(
            job=job,
            polling_interval_secs=2,
            timeout_secs=120,
        )
        completed_report = await sift_client.async_.reports.get(report_id=job.job_details.report_id)

        assert completed_report is not None
        assert completed_report.id_ == job.job_details.report_id
        assert completed_report.job_id == job.id_

        completed_rule_statuses = (
            ReportRuleStatus.FINISHED,
            ReportRuleStatus.FAILED,
            ReportRuleStatus.CANCELED,
            ReportRuleStatus.ERROR,
        )
        assert len(completed_report.summaries) == 1
        assert any(s.status in completed_rule_statuses for s in completed_report.summaries), (
            "expected rule summary to be in a completed state"
        )

    def test_create_from_applicable_rules(
        self, test_rule, nostromo_asset, nostromo_run, sift_client
    ):
        if not test_rule.asset_ids:
            # Test rule may exist but be in a state where it no longer applies to the asset associated w/ the run so re-attach it if necessary.
            test_rule = test_rule.update(update={"asset_ids": [nostromo_asset._id_or_error]})
        job = sift_client.reports.create_from_applicable_rules(
            name="report_from_applicable_rules_run",
            run=nostromo_run,
            organization_id=nostromo_run.organization_id,
        )
        assert job is not None
        assert isinstance(job.job_details, RuleEvaluationDetails)
        report = sift_client.reports.get(report_id=job.job_details.report_id)
        assert report.run_id == nostromo_run.id_

    def test_list(self, nostromo_asset, nostromo_run, tags, sift_client):
        reports = sift_client.reports.list_(
            run=nostromo_run,
            organization_id=nostromo_asset.organization_id,
        )
        assert len(reports) > 0

    def test_rerun(self, nostromo_asset, nostromo_run, test_rule, sift_client):
        job = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert job is not None
        assert isinstance(job.job_details, RuleEvaluationDetails)
        rerun_job = sift_client.reports.rerun(report=job.job_details.report_id)
        assert rerun_job is not None
        assert rerun_job.id_
        assert isinstance(rerun_job.job_details, RuleEvaluationDetails)
        rerun_report = sift_client.reports.get(report_id=rerun_job.job_details.report_id)
        assert rerun_report is not None
        assert rerun_report.run_id == nostromo_run.id_
        assert rerun_report.rerun_from_report_id == job.job_details.report_id

    def test_update(self, nostromo_asset, nostromo_run, test_rule, sift_client):
        job = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert job is not None
        assert isinstance(job.job_details, RuleEvaluationDetails)
        report = sift_client.reports.get(report_id=job.job_details.report_id)
        updated_report = sift_client.reports.update(
            report=report,
            update={
                "metadata": {
                    "test_type": "ci",
                },
            },
        )
        assert updated_report is not None
        assert updated_report.metadata == {"test_type": "ci"}

    def test_find_multiple(self, sift_client):
        with pytest.raises(ValueError, match="Multiple reports found for query"):
            sift_client.reports.find(name="report_from_rules")

    def test_cancel(self, nostromo_asset, nostromo_run, test_rule, sift_client):
        job = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert job is not None
        assert isinstance(job.job_details, RuleEvaluationDetails)
        second_rerun_job = sift_client.reports.rerun(report=job.job_details.report_id)
        assert second_rerun_job is not None
        assert isinstance(second_rerun_job.job_details, RuleEvaluationDetails)
        sift_client.reports.cancel(report=second_rerun_job.job_details.report_id)
        canceled_report = sift_client.reports.find(
            report_ids=[second_rerun_job.job_details.report_id]
        )
        assert canceled_report is not None
        for summary in canceled_report.summaries:
            # Sometimes the report finishes before it can be canceled.
            assert summary.status in [ReportRuleStatus.CANCELED, ReportRuleStatus.FINISHED]

    def test_archive(self, nostromo_run, test_rule, sift_client):
        job = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert job is not None
        assert isinstance(job.job_details, RuleEvaluationDetails)
        job.wait_until_complete(polling_interval_secs=2, timeout_secs=120)
        report = sift_client.reports.get(report_id=job.job_details.report_id)
        archived_report = sift_client.reports.archive(report=report)
        assert archived_report is not None
        assert archived_report.is_archived == True

    def test_unarchive(self, nostromo_run, test_rule, sift_client):
        # Create, wait for completion, then archive to ensure we have an archived report
        job = sift_client.reports.create_from_rules(
            name="report_from_rules_unarchive",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert job is not None
        assert isinstance(job.job_details, RuleEvaluationDetails)
        report = sift_client.reports.get(report_id=job.job_details.report_id)
        archived_report = sift_client.reports.archive(report=report)
        assert archived_report.is_archived is True
        unarchived_report = sift_client.reports.unarchive(report=archived_report)
        assert unarchived_report is not None
        assert unarchived_report.is_archived is False
