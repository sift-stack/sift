import pytest

from unittest import mock

from sift_py._internal.test_util.channel import MockChannel
from sift_py.report_templates.config import ReportTemplateConfig
from sift_py.report_templates.service import ReportTemplateService

"""
TODO:
- Create template, called once
- Update template, called once
- Missing key

- Get rule client keys from config
- Some rules missing keys
"""

@pytest.fixture
def report_template_service():
    return ReportTemplateService(MockChannel())


def test_report_template_service_create_report_template(report_template_service):  #, mock_create_report_template): #, mock_get_report_template_by_client_key):
    report_template_config = ReportTemplateConfig(
        name="report-template",
        template_client_key="template-client-key",
    )

    with mock.patch.object(ReportTemplateService, "_create_report_template") as mock_create_report_template:
        report_template_service.create_or_update_report_template(report_template_config)
        mock_create_report_template.assert_called_once_with(report_template_config)
