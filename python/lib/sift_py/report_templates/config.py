from __future__ import annotations

from typing import Any, Dict, List, Optional

from sift_py._internal.convert.json import AsJson
from sift_py.ingestion.config.yaml.spec import RuleYamlSpec
from sift_py.ingestion.rule.config import RuleConfig


class ReportTemplateConfig(AsJson):
    """
    TODO: A nice doc
    """
    name: str
    template_client_key: str
    organization_id: str = ""
    tags: Optional[List[str]]
    description: Optional[str]
    rules: List[RuleConfig]
    namespaces: Dict[str, List[RuleYamlSpec]]

    def __init__(
        self,
        name: str,
        template_client_key: str,
        organization_id: str = "",
        tags: Optional[List[str]] = None,
        description: Optional[str] = None,
        rules: List[RuleConfig] = [],
        namespaces: Dict[str, List[RuleYamlSpec]] = {},
    ):
        self.name = name
        self.template_client_key = template_client_key
        self.organization_id = organization_id
        self.tags = tags
        self.description = description
        self.rules = rules
        self.namespaces = namespaces

    def as_json(self) -> Any:
        hash_map: Dict[str, Any] = {  # TODO how to confirm the format here
            "name": self.name,
            "template_client_key": self.template_client_key,
            "organization_id": self.organization_id,
            "tags": self.tags,
            "description": self.description,
            "rules": [rule.as_json() for rule in self.rules],
            "namespaces": self.namespaces,
        }
        return hash_map
