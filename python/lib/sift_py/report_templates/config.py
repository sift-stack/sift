from __future__ import annotations

from typing import Any, Dict, List, Optional

from pydantic import BaseModel, ConfigDict

from sift_py.ingestion.config.yaml.spec import RuleYamlSpec
from sift_py.ingestion.rule.config import RuleConfig


class ReportTemplateConfig(BaseModel):
    """
    Configuration for a report template.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str
    template_client_key: str
    organization_id: str = ""
    tags: Optional[List[str]] = None
    description: Optional[str] = None
    rules: List[RuleConfig] = []
    namespaces: Dict[str, List[RuleYamlSpec]] = {}

    def as_json(self) -> Any:
        return self.model_dump_json()

    def to_dict(self) -> Dict[str, Any]:
        return self.model_dump()
