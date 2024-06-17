from __future__ import annotations
from abc import ABC, abstractmethod
from enum import Enum
from sift.annotations.v1.annotations_pb2 import AnnotationType
from sift.rules.v1.rules_pb2 import ActionKind
from sift_internal.convert.json import AsJson
from typing import Any, Dict, List, Optional

import json


class RuleConfig(AsJson):
    """
    Defines a rule to be used during ingestion. If a rule's expression validates to try, then
    a specific action will take place as specified by the `kind` attribute.

    Attributes:
      `name`: Name of the rule.
      `description`: Description of the rule.
      `expression`: A CEL string expression, that, when evaluated to a truthy value, executes the `action`.
      `action`: The action to execute if the result of an `expression` evaluates to a truthy value.
    """

    name: str
    description: str
    expression: str
    action: RuleAction

    def __init__(
        self,
        name: str,
        description: str,
        expression: str,
        action: RuleAction,
        ident_map: Optional[Dict[str, Any]] = None,
    ):
        """
        Initializes an instance of `RuleConfig`.

        The `ident_map` is used when loading in a re-usable expression that contains placeholder identifiers
        as opposed to channel names. The `ident_map` is used to map the placeholder to an actual channel identifier
        or an arbitrary expression.Note that the value of each key will be passed to the builtin `str` function. If
        you're concerned about what the output will be when passed to `str`, prefer to create a `str` value yourself
        before passing it into the `ident_map`.

        A channel identifier is the channel's fully qualified name.

        For a channel whose name is 'voltage' that doesn't belong to a component, the fully qualified name is
        'voltage'. If a channel's name is 'temperature' and it belongs to the 'thruster' component, then the fully
        qualified name is 'thruster.component'. You may use `sift_py.ingestion.channel.channel_fqn` to  generate
        fully qualified channel names. Failure to provide fully-qualified names may produce downstream errors.

        If no `ident_map` is provided, it is assumed that the `expression` argument is fine as is.

        See `sift_py.ingestion.rule.config_test` for examples.
        """
        self.name = name
        self.description = description
        self.action = action

        if ident_map is None or len(ident_map) == 0:
            self.expression = expression
        else:
            self.expression = self.__class__.generate_complete_expression(expression, ident_map)

    def as_json(self) -> str:
        """
        Produces the appropriate JSON structure that's suitable for the Rules API.
        """

        hash_map: Dict[str, str | List[str] | None] = {
            "name": self.name,
            "description": self.description,
            "expression": self.expression,
        }

        if isinstance(self.action, RuleActionCreateDataReviewAnnotation):
            hash_map["type"] = RuleActionAnnotationKind.REVIEW.value
            hash_map["tags"] = self.action.tags
            hash_map["assignee"] = self.action.assignee
        elif isinstance(self.action, RuleActionCreatePhaseAnnotation):
            hash_map["type"] = RuleActionAnnotationKind.PHASE.value
            hash_map["tags"] = self.action.tags
        else:
            raise TypeError(f"Unsupported rule action '{self.action.kind()}'.")

        return json.dumps(hash_map)

    @staticmethod
    def generate_complete_expression(expression: str, ident_map: Dict[str, str]) -> str:
        """
        Takes a generic expression with placeholders and generates a complete expression given the `ident_map`.

        If for example, we had a generic `expression`, `$1 > $2` with an `ident_map` of `{ "$1": "pressure", "$2": 10 }`,
        then this function will output `pressure > 10`.
        """

        for ident, expr in ident_map.items():
            if ident not in expression:
                raise ValueError(f"Couldn't find '{ident}' in expression '{expression}'.")
            expression = expression.replace(ident, str(expr))

        return expression


class RuleAction(ABC):
    @abstractmethod
    def kind(self) -> RuleActionKind:
        pass


class RuleActionCreateDataReviewAnnotation(RuleAction):
    """
    Action to create a data-review annotation when a rule evaluates to a truthy value.

    Attributes:
      `tags`: List of tag names to associate with the newly created data-review annotation.
      `assignee`: Email of user in organization to assign the newly created data-review annotation.
    """

    tags: Optional[List[str]]
    assignee: Optional[str]

    def __init__(self, assignee: Optional[str] = None, tags: Optional[List[str]] = None):
        self.assignee = assignee
        self.tags = tags

    def kind(self) -> RuleActionKind:
        return RuleActionKind.ANNOTATION


class RuleActionCreatePhaseAnnotation(RuleAction):
    """
    Action to create a phase annotation when a rule evaluates to a truthy value.

    Attributes:
      `tags`: List of tag names to associate with the newly created data-review annotation.
    """

    tags: Optional[List[str]]

    def __init__(self, tags: Optional[List[str]] = None):
        self.tags = tags

    def kind(self) -> RuleActionKind:
        return RuleActionKind.ANNOTATION


class RuleActionAnnotationKind(Enum):
    REVIEW = "review"
    PHASE = "phase"

    @classmethod
    def from_annotation_type(cls, annotation_type: AnnotationType) -> "RuleActionAnnotationKind":
        if annotation_type == AnnotationType.ANNOTATION_TYPE_PHASE:
            return cls.PHASE
        return cls.PHASE

    @classmethod
    def from_str(cls, val: str) -> "RuleActionAnnotationKind":
        if val == cls.REVIEW.value:
            return cls.REVIEW
        elif val == cls.PHASE.value:
            return cls.PHASE
        else:
            raise ValueError("Argument 'val' is not a valid annotation kind.")


class RuleActionKind(Enum):
    NOTIFICATION = ActionKind.NOTIFICATION
    ANNOTATION = ActionKind.ANNOTATION

    @classmethod
    def from_str(cls, val: str) -> Optional["RuleActionKind"]:
        if val == "ACTION_KIND_NOTIFICATION" or val == RuleActionKindStrRep.NOTIFICATION.value:
            return cls.NOTIFICATION
        elif val == "ACTION_KIND_ANNOTATION" or val == RuleActionKindStrRep.ANNOTATION.value:
            return cls.ANNOTATION

        return None


class RuleActionKindStrRep(Enum):
    NOTIFICATION = "notification"
    ANNOTATION = "annotation"
