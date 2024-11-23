from __future__ import annotations

from typing import Any, Dict, List, Optional

from pydantic import BaseModel, ConfigDict

from sift_py.ingestion.config.yaml.spec import RuleYamlSpec
from sift_py.ingestion.rule.config import RuleConfig


class ReportTemplateConfig(BaseModel):
    """
    Configuration for a report template.

    - `name`: Name of the report template.
    - `template_client_key`: Unique client key to identify the report template.
    - `organization_id`: Organization ID that the report template belongs to.
    - `tags`: Tags to associate with the report template.
    - `description`: Description of the report template.
    - `rule_client_keys`: List of rule client keys associated with the report template.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str
    template_client_key: str
    organization_id: str = ""
    tags: Optional[List[str]] = None
    description: Optional[str] = None
    rule_client_keys: List[str] = []

    def as_json(self) -> Any:
        return self.model_dump_json()

    def to_dict(self) -> Dict[str, Any]:
        return self.model_dump()
