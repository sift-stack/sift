from unittest import mock

import pytest
from sift.report_templates.v1.report_templates_pb2 import ReportTemplate

from sift_py._internal.test_util.channel import MockChannel
from sift_py.report_templates.config import ReportTemplateConfig, ReportTemplateUpdate
from sift_py.report_templates.service import ReportTemplateService


@pytest.fixture
def report_template_service():
    return ReportTemplateService(MockChannel())


def test_report_template_service_get_report_template_by_client_key(report_template_service):
    report_template_client_key = "report-template-client-key"

    with mock.patch.object(
        ReportTemplateService,
        "_get_report_template_by_client_key",
        return_value=ReportTemplate(name="abc"),
    ) as mock_get_report_template_by_client_key:
        report_template_service.get_report_template(client_key=report_template_client_key)
        mock_get_report_template_by_client_key.assert_called_once_with(report_template_client_key)


def test_report_template_service_get_report_template_by_id(report_template_service):
    report_template_id = "report-template-id"

    with mock.patch.object(
        ReportTemplateService, "_get_report_template_by_id", return_value=ReportTemplate(name="abc")
    ) as mock_get_report_template_by_id:
        report_template_service.get_report_template(id=report_template_id)
        mock_get_report_template_by_id.assert_called_once_with(report_template_id)


def test_report_template_service_get_report_template_missing_or_both_client_key_and_id(
    report_template_service,
):
    with pytest.raises(ValueError, match="One of client_key or id must be provided"):
        report_template_service.get_report_template()
    with pytest.raises(ValueError, match="One of client_key or id must be provided"):
        report_template_service.get_report_template(client_key="abc", id="abc")


def test_report_template_service_create_report_template(report_template_service):
    report_template_config = ReportTemplateConfig(
        name="report-template",
        template_client_key="template-client-key",
    )

    with mock.patch.object(
        ReportTemplateService, "_create_report_template"
    ) as mock_create_report_template:
        report_template_service.create_or_update_report_template(report_template_config)
        mock_create_report_template.assert_called_once_with(report_template_config)


def test_report_template_service_update_report_template(report_template_service):
    report_template_config = ReportTemplateConfig(
        name="report-template",
        template_client_key="template-client-key",
    )

    updates = ReportTemplateUpdate(name="report-template-updated")

    with mock.patch.object(
        ReportTemplateService, "_update_report_template"
    ) as mock_update_report_template:
        with mock.patch.object(
            ReportTemplateService, "_get_report_template_by_client_key"
        ) as mock_get_report_template_by_client_key:
            mock_get_report_template_by_client_key.return_value = report_template_config
            report_template_service.create_or_update_report_template(
                report_template_config, updates=updates
            )
            mock_update_report_template.assert_called_once_with(report_template_config, updates)


def test_report_template_service_missing_template_client_key(report_template_service):
    report_template_config = ReportTemplateConfig.model_construct(  # Without model validation
        name="report-template",
        template_client_key="",
    )

    with pytest.raises(Exception):
        report_template_service.create_or_update_report_template(
            report_template_config,
            match="Report template report-template requires a template_client_key",
        )
