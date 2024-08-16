"""
Entities represent things that files can be attached to.
"""

from __future__ import annotations

from enum import Enum


class Entity:
    entity_id: str
    entity_type: EntityType

    def __init__(self, entity_id: str, entity_type: EntityType):
        self.entity_id = entity_id
        self.entity_type = entity_type


class EntityType(Enum):
    RUN = "runs"
    ANNOTATION = "annotations"
    ANNOTATION_LOG = "annotation_logs"
