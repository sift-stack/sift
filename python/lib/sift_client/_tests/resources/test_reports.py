import pytest

from sift_client.resources import ReportsAPI, ReportsAPIAsync
from sift_client.sift_types import (
    ChannelReference,
    ReportRuleStatus,
    RuleAction,
    RuleAnnotationType,
)


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


@pytest.mark.integration
class TestReports:
    def test_create_from_rules(self, nostromo_run, test_rule, sift_client):
        pending = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert pending is not None
        report = sift_client.reports.get(report_id=pending.report_id)
        assert report.run_id == nostromo_run.id_

    @pytest.mark.asyncio
    async def test_wait_until_complete(self, nostromo_run, test_rule, sift_client):
        """Create a report and wait for its job to complete via async wait_until_complete."""
        pending = sift_client.reports.create_from_rules(
            name="report_wait_until_complete",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert pending is not None
        assert pending.job_id

        completed_report = await sift_client.async_.reports.wait_until_complete(
            pending_report=pending,
            polling_interval_secs=2,
            timeout_secs=120,
        )

        assert completed_report is not None
        assert completed_report.id_ == pending.report_id
        assert completed_report.job_id == pending.job_id

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
        pending = sift_client.reports.create_from_applicable_rules(
            name="report_from_applicable_rules_run",
            run=nostromo_run,
            organization_id=nostromo_run.organization_id,
        )
        assert pending is not None
        report = sift_client.reports.get(report_id=pending.report_id)
        assert report.run_id == nostromo_run.id_

    def test_list(self, nostromo_asset, nostromo_run, tags, sift_client):
        reports = sift_client.reports.list_(
            run=nostromo_run,
            organization_id=nostromo_asset.organization_id,
        )
        assert len(reports) > 0

    def test_rerun(self, nostromo_asset, nostromo_run, test_rule, sift_client):
        pending = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert pending is not None
        rerun_pending = sift_client.reports.rerun(report=pending)
        assert rerun_pending is not None
        assert rerun_pending.report_id
        assert rerun_pending.job_id
        rerun_report = sift_client.reports.get(report_id=rerun_pending.report_id)
        assert rerun_report is not None
        assert rerun_report.run_id == nostromo_run.id_
        assert rerun_report.rerun_from_report_id == pending.report_id

    def test_update(self, nostromo_asset, nostromo_run, test_rule, sift_client):
        pending = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert pending is not None
        report = sift_client.reports.get(report_id=pending.report_id)
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
        pending = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert pending is not None
        second_rerun_pending = sift_client.reports.rerun(report=pending)
        assert second_rerun_pending is not None
        sift_client.reports.cancel(report=second_rerun_pending)
        canceled_report = sift_client.reports.find(report_ids=[second_rerun_pending.report_id])
        assert canceled_report is not None
        for summary in canceled_report.summaries:
            # Sometimes the report finishes before it can be canceled.
            assert summary.status in [ReportRuleStatus.CANCELED, ReportRuleStatus.FINISHED]

    def test_archive(self, nostromo_run, test_rule, sift_client):
        pending = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert pending is not None
        report = pending.wait_until_complete(polling_interval_secs=2, timeout_secs=120)
        archived_report = sift_client.reports.archive(report=report)
        assert archived_report is not None
        assert archived_report.is_archived == True

    def test_unarchive(self, sift_client):
        reports_from_rules = sift_client.reports.list_(
            name="report_from_rules", include_archived=True
        )
        report_from_rules = None
        for report_from_rules in reports_from_rules:
            if report_from_rules.is_archived:
                report_from_rules = report_from_rules
                break
        assert report_from_rules is not None
        assert report_from_rules.is_archived == True
        unarchived_report = sift_client.reports.unarchive(report=report_from_rules)
        assert unarchived_report is not None
        assert unarchived_report.is_archived == False
