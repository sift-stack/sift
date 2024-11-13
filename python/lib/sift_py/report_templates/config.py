from __future__ import annotations

from typing import Any, Dict, List, Optional

from pydantic import BaseModel, ConfigDict, model_validator
from pydantic_core import PydanticCustomError
from typing_extensions import Self

from sift_py.ingestion.config.yaml.spec import RuleYamlSpec
from sift_py.ingestion.rule.config import RuleConfig


class ReportTemplateConfig(BaseModel):
    """
    TODO: A nice doc
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str
    template_client_key: str
    organization_id: str = ""
    tags: Optional[List[str]] = None
    description: Optional[str] = None
    rules: List[RuleConfig] = []
    namespaces: Dict[str, List[RuleYamlSpec]] = {}

    @model_validator(mode="after")
    def validate_config(self) -> Self:
        if not self.name:
            raise PydanticCustomError("invalid_config_error", "Empty 'name'")
        if not self.template_client_key:
            raise PydanticCustomError("invalid_config_error", "Empty 'template_client_key'")
        return self

    def as_json(self) -> Any:
        return self.model_dump_json()

    def to_dict(self) -> Dict[str, Any]:
        return self.model_dump()

    def from_dict(self, data: Dict[str, Any]) -> ReportTemplateConfig:
        return self.parse_obj(data)
