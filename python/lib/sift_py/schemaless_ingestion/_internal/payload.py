from typing import List, Optional, TypedDict

from sift_py.schemaless_ingestion.data import SchemalessChannel


class SchemalessDataPayload(TypedDict):
    timestamp: str
    values: List[SchemalessChannel]


class SchemalessPayload(TypedDict):
    asset_name: str
    run_id: Optional[str]
    organization_id: Optional[str]
    data: List[SchemalessDataPayload]
