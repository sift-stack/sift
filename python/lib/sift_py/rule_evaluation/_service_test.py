from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest
from sift.rule_evaluation.v1.rule_evaluation_pb2 import (
    EvaluateRulesResponse,
)
from sift.rules.v1.rules_pb2 import BatchUpdateRulesResponse
from sift_py._internal.test_util.channel import MockChannel
from sift_py.report_templates.config import ReportTemplateConfig
from sift_py.rule.config import (
    RuleActionCreateDataReviewAnnotation,
    RuleConfig,
)
from sift_py.rule.service import RuleIdentifier
from sift_py.rule_evaluation.service import RuleEvaluationService


@pytest.fixture
def mock_channel():
    return MockChannel()


@pytest.fixture
def rule_evaluation_service(mock_channel):
    return RuleEvaluationService(mock_channel)


def test_evaluate_and_preview_rule_identifiers_against_run(rule_evaluation_service):
    mock_stub = MagicMock()
    rule_evaluation_service._rule_evaluation_stub = mock_stub
    mock_stub.EvaluateRules.return_value = EvaluateRulesResponse(report_id="test_report_id")

    run_id = "test_run_id"
    rule_identifiers = [
        RuleIdentifier(rule_id="rule-id1", name="rule1"),
        RuleIdentifier(rule_id="rule-id2", name="rule2"),
    ]
    report_name = "test_report"

    report = rule_evaluation_service.evaluate_against_run(run_id, rule_identifiers, report_name)
    request = mock_stub.mock_calls[0].args[0]
    assert report.report_id == "test_report_id"
    assert request.report_name == report_name
    assert request.run.id == run_id
    assert request.rules.rules.ids.ids[0] == rule_identifiers[0].rule_id
    assert request.rules.rules.ids.ids[1] == rule_identifiers[1].rule_id
    assert request.run_time_range.start_time.seconds == 0
    assert request.run_time_range.end_time.seconds == 0

    rule_evaluation_service.preview_against_run(run_id, rule_identifiers)
    request = mock_stub.mock_calls[1].args[0]
    assert request.run.id == run_id
    assert request.rules.rules.ids.ids[0] == rule_identifiers[0].rule_id
    assert request.rules.rules.ids.ids[1] == rule_identifiers[1].rule_id
    assert request.run_time_range.start_time.seconds == 0
    assert request.run_time_range.end_time.seconds == 0


def test_evaluate_and_preview_report_template_against_run(rule_evaluation_service):
    mock_stub = MagicMock()
    rule_evaluation_service._rule_evaluation_stub = mock_stub
    mock_stub.EvaluateRules.return_value = EvaluateRulesResponse(report_id="test_report_id")

    run_id = "test_run_id"
    report_name = "test_report"
    report_template = ReportTemplateConfig(name=report_name, template_id="template-id")

    report = rule_evaluation_service.evaluate_against_run(run_id, report_template, report_name)
    request = mock_stub.mock_calls[0].args[0]
    assert report.report_id == "test_report_id"
    assert request.report_name == report_name
    assert request.run.id == run_id
    assert request.report_template.report_template.id == report_template.template_id
    assert request.run_time_range.start_time.seconds == 0
    assert request.run_time_range.end_time.seconds == 0

    rule_evaluation_service.preview_against_run(run_id, report_template)
    request = mock_stub.mock_calls[1].args[0]
    assert request.run.id == run_id
    assert request.report_template.report_template.id == report_template.template_id
    assert request.run_time_range.start_time.seconds == 0
    assert request.run_time_range.end_time.seconds == 0


def test_evaluate_and_preview_report_template_against_run_with_start_end_times(
    rule_evaluation_service,
):
    mock_stub = MagicMock()
    rule_evaluation_service._rule_evaluation_stub = mock_stub
    mock_stub.EvaluateRules.return_value = EvaluateRulesResponse(report_id="test_report_id")

    run_id = "test_run_id"
    report_name = "test_report"
    report_template = ReportTemplateConfig(name=report_name, template_id="template-id")
    start_time = datetime(2025, 1, 1, 1, 1, 1, tzinfo=timezone.utc)
    end_time = datetime(2025, 1, 2, 1, 1, 59, tzinfo=timezone.utc)

    report = rule_evaluation_service.evaluate_against_run(
        run_id,
        report_template,
        report_name,
        start_time=start_time,
        end_time=end_time,
    )
    request = mock_stub.mock_calls[0].args[0]
    assert report.report_id == "test_report_id"
    assert request.report_name == report_name
    assert request.run_time_range.run.id == run_id
    assert request.run_time_range.start_time.seconds == int(start_time.timestamp())
    assert request.run_time_range.end_time.seconds == int(end_time.timestamp())
    assert request.report_template.report_template.id == report_template.template_id

    rule_evaluation_service.preview_against_run(run_id, report_template, start_time, end_time)
    request = mock_stub.mock_calls[1].args[0]
    assert request.report_template.report_template.id == report_template.template_id
    assert request.run_time_range.run.id == run_id
    assert request.run_time_range.start_time.seconds == int(start_time.timestamp())
    assert request.run_time_range.end_time.seconds == int(end_time.timestamp())


def test_evaluate_and_preview_rule_configs_against_run(rule_evaluation_service):
    mock_stub = MagicMock()
    rule_evaluation_service._rule_evaluation_stub = mock_stub
    mock_stub.EvaluateRules.return_value = EvaluateRulesResponse(report_id="test_report_id")

    run_id = "test_run_id"
    report_name = "test_report"
    rule_configs = [
        RuleConfig(
            name="rule1",
            rule_client_key="key1",
            channel_references=[],
            expression="$1 == 1",
            action=RuleActionCreateDataReviewAnnotation(),
        ),
        RuleConfig(
            name="rule2",
            rule_client_key="key2",
            channel_references=[],
            expression="$2 == 2",
            action=RuleActionCreateDataReviewAnnotation(),
        ),
    ]

    report = rule_evaluation_service.evaluate_against_run(run_id, rule_configs, report_name)
    request = mock_stub.mock_calls[0].args[0]
    assert report.report_id == "test_report_id"
    assert request.report_name == report_name
    assert request.run.id == run_id
    assert request.rules.rules.client_keys.client_keys[0] == rule_configs[0].rule_client_key
    assert request.rules.rules.client_keys.client_keys[1] == rule_configs[1].rule_client_key
    assert request.run_time_range.start_time.seconds == 0
    assert request.run_time_range.end_time.seconds == 0

    rule_evaluation_service.preview_against_run(run_id, rule_configs)
    request = mock_stub.mock_calls[1].args[0]
    assert request.run.id == run_id
    assert request.rules.rules.client_keys.client_keys[0] == rule_configs[0].rule_client_key
    assert request.rules.rules.client_keys.client_keys[1] == rule_configs[1].rule_client_key
    assert request.run_time_range.start_time.seconds == 0
    assert request.run_time_range.end_time.seconds == 0


def test_evaluate_and_preview_rule_configs_against_run_with_start_end_times(
    rule_evaluation_service,
):
    mock_stub = MagicMock()
    rule_evaluation_service._rule_evaluation_stub = mock_stub
    mock_stub.EvaluateRules.return_value = EvaluateRulesResponse(report_id="test_report_id")

    run_id = "test_run_id"
    report_name = "test_report"
    rule_configs = [
        RuleConfig(
            name="rule1",
            rule_client_key="key1",
            channel_references=[],
            expression="$1 == 1",
            action=RuleActionCreateDataReviewAnnotation(),
        ),
        RuleConfig(
            name="rule2",
            rule_client_key="key2",
            channel_references=[],
            expression="$2 == 2",
            action=RuleActionCreateDataReviewAnnotation(),
        ),
    ]
    start_time = datetime(2025, 1, 1, 1, 1, 1, tzinfo=timezone.utc)
    end_time = datetime(2025, 1, 2, 1, 1, 59, tzinfo=timezone.utc)

    report = rule_evaluation_service.evaluate_against_run(
        run_id, rule_configs, report_name, start_time, end_time
    )
    request = mock_stub.mock_calls[0].args[0]
    assert report.report_id == "test_report_id"
    assert request.report_name == report_name
    assert request.rules.rules.client_keys.client_keys[0] == rule_configs[0].rule_client_key
    assert request.rules.rules.client_keys.client_keys[1] == rule_configs[1].rule_client_key
    assert request.run_time_range.run.id == run_id
    assert request.run_time_range.start_time.seconds == int(start_time.timestamp())
    assert request.run_time_range.end_time.seconds == int(end_time.timestamp())

    rule_evaluation_service.preview_against_run(run_id, rule_configs, start_time, end_time)
    request = mock_stub.mock_calls[1].args[0]
    assert request.rules.rules.client_keys.client_keys[0] == rule_configs[0].rule_client_key
    assert request.rules.rules.client_keys.client_keys[1] == rule_configs[1].rule_client_key
    assert request.run_time_range.run.id == run_id
    assert request.run_time_range.start_time.seconds == int(start_time.timestamp())
    assert request.run_time_range.end_time.seconds == int(end_time.timestamp())


def test_evaluate_rules_against_assets(rule_evaluation_service):
    mock_stub = MagicMock()
    rule_evaluation_service._rule_evaluation_stub = mock_stub
    mock_stub.EvaluateRules.return_value = EvaluateRulesResponse(report_id="test_report_id")

    asset_names = ["asset1", "asset2"]
    start_time = datetime(2025, 1, 1, 1, 1, 1, tzinfo=timezone.utc)
    end_time = datetime(2025, 1, 2, 1, 1, 59, tzinfo=timezone.utc)
    rules = [
        RuleIdentifier(rule_id="rule-id1", name="rule1"),
        RuleIdentifier(rule_id="rule-id2", name="rule2"),
    ]
    report_name = "test_report"

    report = rule_evaluation_service.evaluate_against_assets(
        asset_names, start_time, end_time, rules, report_name
    )
    request = mock_stub.mock_calls[0].args[0]
    assert report.report_id == "test_report_id"
    assert request.report_name == report_name
    assert request.assets.assets.names.names[0] == asset_names[0]
    assert request.assets.assets.names.names[1] == asset_names[1]
    assert request.assets.start_time.seconds == int(start_time.timestamp())
    assert request.assets.end_time.seconds == int(end_time.timestamp())
    assert request.rules.rules.ids.ids[0] == rules[0].rule_id
    assert request.rules.rules.ids.ids[1] == rules[1].rule_id


def test_evaluate_and_preview_external_rules(mocker, rule_evaluation_service):
    run_id = "test_run_id"
    rules = [
        RuleConfig(
            name="rule1",
            channel_references=[],
            expression="$1 == 1",
            action=RuleActionCreateDataReviewAnnotation(),
            is_external=True,
        ),
        RuleConfig(
            name="rule2",
            channel_references=[],
            expression="$2 == 2",
            action=RuleActionCreateDataReviewAnnotation(),
            is_external=True,
        ),
    ]
    report_name = "test_report"

    mock_stub = MagicMock()
    mock_stub.EvaluateRules.return_value = EvaluateRulesResponse(report_id="test_report_id")
    rule_evaluation_service._rule_evaluation_stub = mock_stub

    rule_identifiers = [
        BatchUpdateRulesResponse.RuleIdentifiers(rule_id=f"{r.name}-id", name=r.name) for r in rules
    ]
    mock_batch_update_rules = mocker.patch.object(
        rule_evaluation_service._rule_service._rule_service_stub, "BatchUpdateRules"
    )
    mock_batch_update_rules.return_value = BatchUpdateRulesResponse(
        created_rule_identifiers=rule_identifiers,
        success=True,
    )

    report = rule_evaluation_service.evaluate_external_rules(run_id, rules, report_name)
    request = mock_stub.mock_calls[0].args[0]
    assert report.report_id == "test_report_id"
    assert request.report_name == report_name
    assert request.run.id == run_id
    assert request.rules.rules.ids.ids[0] == rule_identifiers[0].rule_id
    assert request.rules.rules.ids.ids[1] == rule_identifiers[1].rule_id

    rule_evaluation_service.preview_external_rules(run_id, rules)
    request = mock_stub.mock_calls[1].args[0]
    assert request.run.id == run_id
    assert request.rules.rules.ids.ids[0] == rule_identifiers[0].rule_id
    assert request.rules.rules.ids.ids[1] == rule_identifiers[1].rule_id


def test_invalid_rules(rule_evaluation_service):
    run_id = "test_run_id"
    report_name = "test_report"

    with pytest.raises(ValueError, match="Rule set is empty"):
        rule_evaluation_service.evaluate_against_run(run_id, [], report_name)

    with pytest.raises(ValueError, match="requires a rule_client_key"):
        invalid_rule_config = RuleConfig(
            name="rule2",
            channel_references=[],
            expression="$2 == 2",
            action=RuleActionCreateDataReviewAnnotation(),
        )
        rule_evaluation_service.evaluate_against_run(run_id, [invalid_rule_config], report_name)
