#!/usr/bin/env python3
"""This test demonstrates the usage of the Runs API.

It creates a new run, updates it, and associates assets with it.
It also lists runs, filters them, and deletes the run.

It uses the SiftClient to interact with the API.
"""

import asyncio
import os

from sift_client import SiftClient, SiftConnectionConfig
from sift_client.sift_types import (
    ChannelReference,
    ReportRuleStatus,
    RuleAction,
    RuleAnnotationType,
)


async def main():
    """Main function demonstrating the Runs API usage."""
    # Initialize the client
    # You can set these environment variables or pass them directly
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_LOCAL_API_KEY", "")

    client = SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=api_key,
            grpc_url=grpc_url,
            rest_url=rest_url,
            use_ssl=False,
        ),
    )

    asset = client.assets.find(name="NostromoLV426")
    asset_id = asset.id_
    print(f"Using asset: {asset.name} (ID: {asset_id})")

    # List runs for this asset
    runs = asset.runs
    print(
        f"Found {len(runs)} run(s): {[run.name for run in runs]} for asset {asset.name} (ID: {asset_id})"
    )

    # Pick one.
    run = runs[0]
    run_id = run.id_
    print(f"Using run: {run.name} (ID: {run_id})")

    tags = client.tags.find_or_create(names=["test", "api-created"])
    tag_ids = [tag.id_ for tag in tags]

    rule = client.rules.create(
        name="test_rule",
        description="Test rule",
        expression="$1 > 0.1",
        channel_references=[
            ChannelReference(channel_reference="$1", channel_identifier="mainmotor.velocity"),
        ],
        action=RuleAction.annotation(
            annotation_type=RuleAnnotationType.DATA_REVIEW,
            tags_ids=tag_ids,
            default_assignee_user_id=None,
        ),
        asset_ids=[asset_id],
    )
    print(f"Created rule: {rule.name} (ID: {rule.id_})")

    report_from_rules = client.reports.create_from_rules(
        name="report_from_rules",
        run_id=run_id,
        rule_ids=[rule.id_],
    )
    print(f"Created report: {report_from_rules.name} (ID: {report_from_rules.id_})")
    print(f"Report summaries: {report_from_rules.summaries}")
    print(f"Report tags: {report_from_rules.tags}")

    report_from_applicable_rules_run = client.reports.create_from_applicable_rules(
        organization_id=asset.organization_id,
        name="report_from_applicable_rules_run",
        run_id=run_id,
    )
    print(
        f"Created report: {report_from_applicable_rules_run.name} (ID: {report_from_applicable_rules_run.id_})"
    )
    print(f"Report summaries: {report_from_applicable_rules_run.summaries}")
    print(f"Report tags: {report_from_applicable_rules_run.tags}")

    job_id, rerun_report_id = client.reports.rerun(report=report_from_rules)
    rerun_report = client.reports.get(report_id=rerun_report_id)
    print(f"Rerun report: {rerun_report.name} (ID: {rerun_report.id_})")
    print(f"Report summaries: {rerun_report.summaries}")
    print(f"Report tags: {rerun_report.tags}")

    assert rerun_report.metadata == {}
    updated_report = client.reports.update(
        report=rerun_report,
        update={
            "metadata": {
                "test_type": "ci",
            },
        },
    )
    print(f"Updated report: {updated_report.name} (ID: {updated_report.id_})")

    reports = client.reports.list_(
        run_id=run_id,
        organization_id=asset.organization_id,
    )
    print(f"Found {len(reports)} report(s): {[report.name for report in reports]}")

    try:
        client.reports.find(name=rerun_report.name)
    except ValueError as e:
        assert "Multiple reports found for query" in str(e)

    find_report = client.reports.find(name=rerun_report.name, metadata={"test_type": "ci"})

    job_id, second_rerun_report_id = client.reports.rerun(report=report_from_rules)
    print(f"Second rerun report: {second_rerun_report_id} (ID: {second_rerun_report_id})")

    client.reports.cancel(report=second_rerun_report_id)
    canceled_report = client.reports.get(report_id=second_rerun_report_id)
    print(f"Canceled report: {canceled_report.name} (ID: {canceled_report.id_})")

    client.rules.archive(rule_ids=[rule.id_])

    report_ids = {
        report_from_rules.id_,
        report_from_applicable_rules_run.id_,
        rerun_report.id_,
        second_rerun_report_id,
    }
    for report_id in report_ids:
        client.reports.archive(report=report_id)

    assert rerun_report.rerun_from_report_id == report_from_rules.id_
    assert find_report.id_ == rerun_report.id_
    for summary in canceled_report.summaries:
        assert summary.status == ReportRuleStatus.CANCELED
    print("All tests passed")


if __name__ == "__main__":
    asyncio.run(main())
