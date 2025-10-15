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
        report_from_rules = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert report_from_rules is not None
        assert report_from_rules.run_id == nostromo_run.id_

    def test_create_from_applicable_rules(
        self, test_rule, nostromo_asset, nostromo_run, sift_client
    ):
        if not test_rule.asset_ids:
            # Test rule may exist but be in a state where it no longer applies to the asset associated w/ the run so re-attach it if necessary.
            test_rule = test_rule.update(update={"asset_ids": [nostromo_asset._id_or_error]})
        report_from_applicable_rules = sift_client.reports.create_from_applicable_rules(
            name="report_from_applicable_rules_run",
            run=nostromo_run,
            organization_id=nostromo_run.organization_id,
        )
        assert report_from_applicable_rules is not None
        assert report_from_applicable_rules.run_id == nostromo_run.id_

    def test_list(self, nostromo_asset, nostromo_run, tags, sift_client):
        reports = sift_client.reports.list_(
            run=nostromo_run,
            organization_id=nostromo_asset.organization_id,
        )
        assert len(reports) > 0

    def test_rerun(self, nostromo_asset, nostromo_run, test_rule, sift_client):
        report_from_rules = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert report_from_rules is not None
        job_id, rerun_report_id = sift_client.reports.rerun(report=report_from_rules)
        rerun_report = sift_client.reports.get(report_id=rerun_report_id)
        assert rerun_report is not None
        assert rerun_report.run_id == nostromo_run.id_
        assert rerun_report.rerun_from_report_id == report_from_rules.id_

    def test_update(self, nostromo_asset, nostromo_run, test_rule, sift_client):
        report_from_rules = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert report_from_rules is not None
        updated_report = sift_client.reports.update(
            report=report_from_rules,
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
        report_from_rules = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert report_from_rules is not None
        job_id, second_rerun_report_id = sift_client.reports.rerun(report=report_from_rules)
        assert second_rerun_report_id is not None
        sift_client.reports.cancel(report=second_rerun_report_id)
        canceled_report = sift_client.reports.find(report_ids=[second_rerun_report_id])
        assert canceled_report is not None
        for summary in canceled_report.summaries:
            # Sometimes the report finishes before it can be canceled.
            assert summary.status in [ReportRuleStatus.CANCELED, ReportRuleStatus.FINISHED]

    def test_archive(self, nostromo_run, test_rule, sift_client):
        report_from_rules = sift_client.reports.create_from_rules(
            name="report_from_rules",
            run=nostromo_run,
            rules=[test_rule],
        )
        assert report_from_rules is not None
        archived_report = sift_client.reports.archive(report=report_from_rules)
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
