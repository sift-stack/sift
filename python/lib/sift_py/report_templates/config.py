from __future__ import annotations

from datetime import datetime
from typing import Any, List, Optional

from pydantic import BaseModel, ConfigDict
from typing_extensions import NotRequired, TypedDict

from sift_py._internal.convert.json import AsJson


class ReportTemplateConfig(BaseModel, AsJson):
    """
    Configuration for a report template.

    - `name`: Name of the report template.
    - `template_client_key`: Unique client key to identify the report template.
    - `organization_id`: Organization ID that the report template belongs to.
    - `tags`: Tags to associate with the report template.
    - `description`: Description of the report template.
    - `rule_client_keys`: List of rule client keys associated with the report template.
    - `archived_date`: Date when the report template was archived. Setting this field
        will archive the report template, and unsetting it will unarchive the report template.
    - `archived`: True if the report template is archived, False otherwise.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str
    template_client_key: Optional[str] = None
    template_id: Optional[str] = None
    tags: Optional[List[str]] = None
    description: Optional[str] = None
    rule_client_keys: List[str] = []
    archived_date: Optional[datetime] = None
    archived: bool = False

    def as_json(self) -> Any:
        return self.model_dump_json()


class ReportTemplateUpdate(TypedDict):
    """
    Represents a dictionary for updating properties of a report template. All fields are optional
    and only the provided fields will be updated.

    - `name`: Updated name of the report template.
    - `template_client_key`: Updated unique client key to identify the report template.
    - `organization_id`: Updated organization ID that the report template belongs to.
    - `tags`: Updated tags to associate with the report template.
    - `description`: Updated description of the report template.
    - `rule_client_keys`: Updated list of rule client keys associated with the report template.
    - `archived`: True if the report template is archived, False otherwise.
    """

    name: NotRequired[str]
    template_client_key: NotRequired[str]
    tags: NotRequired[List[str]]
    description: NotRequired[str]
    rule_client_keys: NotRequired[List[str]]
    archived: NotRequired[bool]
