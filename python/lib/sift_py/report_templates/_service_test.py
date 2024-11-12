from unittest import mock

import pytest

from sift_py._internal.test_util.channel import MockChannel
from sift_py.report_templates.config import ReportTemplateConfig
from sift_py.report_templates.service import ReportTemplateService
from sift_py.rule.config import RuleConfig


@pytest.fixture
def report_template_service():
    return ReportTemplateService(MockChannel())


def test_report_template_service_get_report_template_by_client_key(report_template_service):
    report_template_client_key = "report-template-client-key"

    with mock.patch.object(ReportTemplateService, "_get_report_template_by_client_key") as mock_get_report_template_by_client_key:
        report_template_service.get_report_template(client_key=report_template_client_key)
        mock_get_report_template_by_client_key.assert_called_once_with(report_template_client_key)


def test_report_template_service_get_report_template_by_id(report_template_service):
    report_template_id = "report-template-id"

    with mock.patch.object(ReportTemplateService, "_get_report_template_by_id") as mock_get_report_template_by_id:
        report_template_service.get_report_template(report_template_id=report_template_id)
        mock_get_report_template_by_id.assert_called_once_with(report_template_id)


def test_report_template_service_get_report_template_missing_client_key_and_id(report_template_service):
    with pytest.raises(ValueError, match="Either client_key or report_template_id must be provided"):
        report_template_service.get_report_template()


def test_report_template_service_create_report_template(report_template_service):
    report_template_config = ReportTemplateConfig(
        name="report-template",
        template_client_key="template-client-key",
    )

    with mock.patch.object(ReportTemplateService, "_create_report_template") as mock_create_report_template:
        report_template_service.create_or_update_report_template(report_template_config)
        mock_create_report_template.assert_called_once_with(report_template_config)


def test_report_template_service_update_report_template(report_template_service):
    report_template_config = ReportTemplateConfig(
        name="report-template",
        template_client_key="template-client-key",
    )

    report_template_config_update = ReportTemplateConfig(
        name="report-template-updated",
        template_client_key="template-client-key",
    )

    with mock.patch.object(ReportTemplateService, "_update_report_template") as mock_update_report_template:
        with mock.patch.object(ReportTemplateService, "_get_report_template_by_client_key") as mock_get_report_template_by_client_key:
            mock_get_report_template_by_client_key.return_value = report_template_config
            report_template_service.create_or_update_report_template(report_template_config_update)
            mock_update_report_template.assert_called_once_with(report_template_config_update)


def test_report_template_service_missing_template_client_key(report_template_service):
    report_template_config = ReportTemplateConfig(
        name="report-template",
        template_client_key="",
    )

    with pytest.raises(Exception):
        report_template_service.create_or_update_report_template(report_template_config, match="Report template report-template requires a template_client_key")


def test_report_template_service__get_rule_client_keys(report_template_service):
    report_template_config = ReportTemplateConfig(
        name="report-template",
        template_client_key="template-client-key",
        rules=[
            RuleConfig(
                name="rule-1",
                rule_client_key="rule-client-key-1",
                channel_references=[],
            ),
            RuleConfig(
                name="rule-2",
                rule_client_key="rule-client-key-2",
                channel_references=[],
            ),
        ],
    )

    rule_client_keys = report_template_service._get_rule_client_keys(report_template_config)
    assert rule_client_keys == ["rule-client-key-1", "rule-client-key-2"]


def test_report_template_service__get_rule_client_keys_missing_rule_client_key(report_template_service):
    report_template_config = ReportTemplateConfig(
        name="report-template",
        template_client_key="template-client-key",
        rules=[
            RuleConfig(
                name="rule-1",
                rule_client_key="rule-client-key-1",
                channel_references=[],
            ),
            RuleConfig(
                name="rule-2",
                rule_client_key="",
                channel_references=[],
            ),
        ],
    )

    with pytest.raises(Exception):
        report_template_service._get_rule_client_keys(report_template_config, match="rule rule-2 requires a rule_client_key")

