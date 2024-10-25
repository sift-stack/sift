from __future__ import annotations

from typing import Dict, List, Optional

from sift_py._internal.convert.json import AsJson
from sift_py.ingestion.config.yaml.spec import RuleYamlSpec
from sift_py.ingestion.rule.config import RuleConfig


class ReportTemplateConfig(AsJson):
    """
    TODO: A nice doc
    """
    name: str
    template_client_key: str
    tags: Optional[List[str]]
    description: Optional[str]
    rules: List[RuleConfig]
    namespaces: Dict[str, List[RuleYamlSpec]]
