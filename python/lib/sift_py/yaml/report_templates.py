from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, cast

import yaml
from typing_extensions import NotRequired, TypedDict

from sift_py.ingestion.config.yaml.error import YamlConfigError
from sift_py.report_templates.config import ReportTemplateConfig
from sift_py.yaml.utils import _handle_subdir


def load_report_templates(paths: List[Path]) -> List[ReportTemplateConfig]:
    """
    Takes in a list of paths to YAML files which contains report templates and processes them into a list of
    `ReportTemplateConfig` objects. For more information on report templates see
    `sift_py.report_templates.config.ReportTemplateConfig`.
    """
    report_templates: List[ReportTemplateConfig] = []

    def update_report_templates(path: Path):
        report_templates.extend(_read_report_template_yaml(path))

    for path in paths:
        if path.is_dir():
            _handle_subdir(path, update_report_templates)
        elif path.is_file():
            update_report_templates(path)
    return report_templates


def _read_report_template_yaml(path: Path) -> List[ReportTemplateConfig]:
    report_templates = []
    with open(path, "r") as f:
        report_templates_yaml = cast(Dict[str, Any], yaml.safe_load(f.read()))

        report_template_list = report_templates_yaml.get("report_templates")
        if not isinstance(report_template_list, list):
            raise YamlConfigError(
                f"Expected 'report_templates' to be a list in report template yaml: '{path}'"
            )

        for report_template in report_template_list:
            try:
                report_template_config = ReportTemplateConfig(**report_template)
                report_templates.append(report_template_config)
            except Exception as e:
                raise YamlConfigError(f"Error parsing report template '{report_template}'") from e

        return report_templates


class ReportTemplateYamlSpec(TypedDict):
    """
    Formal spec for a report template.

    `name`: Name of the report template.
    `template_client_key`: Unique client key to identify the report template.
    `organization_id`: Organization ID that the report template belongs to.
    `tags`: Tags to associate with the report template.
    `description`: Description of the report template.
    `rule_client_keys`: List of rule client keys associated with the report template.
    `archived_date`: Date when the report template was archived. Setting this field
      will archive the report template, and unsetting it will unarchive the report template.
    """

    name: str
    template_client_key: str
    organization_id: NotRequired[str]
    tags: NotRequired[List[str]]
    description: NotRequired[str]
    rule_client_keys: List[str]
    archived_date: NotRequired[datetime]
