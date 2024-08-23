"""
Entities represent things that files can be attached to.
"""

from __future__ import annotations

from enum import Enum


class Entity:
    """
    An abstract entity that represents the thing that we want to attach files to.
    """

    entity_id: str
    entity_type: EntityType

    def __init__(self, entity_id: str, entity_type: EntityType):
        self.entity_id = entity_id
        self.entity_type = entity_type


class EntityType(Enum):
    """
    Represents the types of entities that supports file attachments.
    """

    RUN = "runs"
    ANNOTATION = "annotations"
    ANNOTATION_LOG = "annotation_logs"
