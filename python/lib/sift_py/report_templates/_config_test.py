import pytest
from pydantic_core import ValidationError

from sift_py.report_templates.config import ReportTemplateConfig


@pytest.fixture
def report_template_dict() -> dict:
    return {
        "name": "report-template",
        "template_client_key": "template-client-key",
    }


def test_report_template_config(report_template_dict):
    report_template_config = ReportTemplateConfig(**report_template_dict)
    assert report_template_config.name == "report-template"
    assert report_template_config.template_client_key == "template-client-key"
    assert report_template_config.tags is None
    assert report_template_config.description is None
    assert report_template_config.rule_client_keys == []
    assert report_template_config.archived_date is None


def test_report_template_config_validation(report_template_dict):
    report_template_dict.pop("name")
    with pytest.raises(ValidationError, match="Field required"):
        ReportTemplateConfig(**report_template_dict)
