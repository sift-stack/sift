from __future__ import annotations

from abc import ABC, abstractmethod
from enum import Enum
from typing import Any, Dict, List, Optional, Tuple, TypedDict, Union, cast

from sift.annotations.v1.annotations_pb2 import AnnotationType
from sift.rules.v1.rules_pb2 import ActionKind

from sift_py._internal.convert.json import AsJson
from sift_py.ingestion.channel import ChannelConfig


class RuleConfig(AsJson):
    """
    Defines a rule to be used during ingestion. If a rule's expression validates to try, then
    a specific action will take place as specified by the `kind` attribute.

    - `name`: Name of the rule.
    - `description`: Description of the rule.
    - `expression`: A CEL string expression, that, when evaluated to a truthy value, executes the `action`.
    - `action`: The action to execute if the result of an `expression` evaluates to a truthy value.
    - `channel_references`: Reference to channel. If an expression is "$1 < 10", then "$1" is the reference and thus should the key in the dict.
    """

    name: str
    description: Optional[str]
    expression: Optional[str]
    action: Optional[RuleAction]
    channel_references: List[ExpressionChannelReference]

    def __init__(
        self,
        name: str,
        channel_references: List[
            Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]
        ],
        description: Optional[str] = "",
        expression: Optional[str] = "",
        action: Optional[RuleAction] = None,
        sub_expressions: Optional[Dict[str, Any]] = {},
        namespace: Optional[str] = "",
        namespace_rules: Optional[Dict[str, List[Dict]]] = {},
    ):
        self.channel_references = []

        for channel_reference in channel_references:
            config = channel_reference.get("channel_config")

            if config is not None:
                config = cast(ChannelConfig, config)

                self.channel_references.append(
                    {
                        "channel_reference": channel_reference["channel_reference"],
                        "channel_identifier": config.fqn(),
                    }
                )
            else:
                channel_ref = cast(ExpressionChannelReference, channel_reference)

                self.channel_references.append(
                    {
                        "channel_reference": channel_ref["channel_reference"],
                        "channel_identifier": channel_ref["channel_identifier"],
                    }
                )

        self.name = name

        if namespace:
            description, expression, action = self.__class__.interpolate_namespace_rule(
                namespace, namespace_rules
            )

        self.action = action
        self.description = description
        self.expression = self.__class__.interpolate_sub_expressions(expression, sub_expressions)

    def as_json(self) -> Any:
        """
        Produces the appropriate JSON structure that's suitable for the Rules API.
        """

        hash_map: Dict[str, Union[List[ExpressionChannelReference], str, List[str], None]] = {
            "name": self.name,
            "description": self.description,
            "expression": self.expression,
        }

        hash_map["expression_channel_references"] = self.channel_references

        if isinstance(self.action, RuleActionCreateDataReviewAnnotation):
            hash_map["type"] = RuleActionAnnotationKind.REVIEW.value
            hash_map["assignee"] = self.action.assignee

            if self.action.assignee is not None and len(self.action.assignee) > 0:
                hash_map["assignee"] = self.action.assignee

            if self.action.tags is not None and len(self.action.tags) > 0:
                hash_map["tags"] = self.action.tags

        elif isinstance(self.action, RuleActionCreatePhaseAnnotation):
            hash_map["type"] = RuleActionAnnotationKind.PHASE.value

            if self.action.tags is not None and len(self.action.tags) > 0:
                hash_map["tags"] = self.action.tags
        else:
            kind = self.action.kind() if self.action else self.action
            raise TypeError(f"Unsupported rule action '{kind}'.")

        return hash_map

    @staticmethod
    def interpolate_sub_expressions(
        expression: Optional[str], sub_expressions: Optional[Dict[str, str]]
    ) -> Optional[str]:
        if expression and sub_expressions:
            for ref, expr in sub_expressions.items():
                if ref not in expression:
                    raise ValueError(f"Couldn't find '{ref}' in expression '{expression}'.")
                if isinstance(expr, str):
                    expression = expression.replace(ref, f'"{expr}"')
                else:
                    expression = expression.replace(ref, str(expr))

        return expression

    @staticmethod
    def interpolate_namespace_rule(
        namespace: str, namespace_rules: Optional[Dict[str, List[Dict]]]
    ) -> Tuple[str, str, RuleAction]:
        if not namespace_rules:
            raise ValueError(
                f"Namespace rules must be provided with namespace key. Got: {namespace_rules}"
            )

        rule_list = namespace_rules.get(namespace)
        if not rule_list:
            raise ValueError(
                f"Couldn't find namespace '{namespace}' in namespace_rules: {namespace_rules}"
            )

        for rule in rule_list:
            name = rule.get("name")
            if name:
                description = rule.get("description", "")
                expression = rule.get("expression", "")
                type = rule.get("type", "")
                tags = rule.get("tags")
                action: RuleAction = RuleActionCreatePhaseAnnotation(tags)
                if RuleActionAnnotationKind.from_str(type) == RuleActionAnnotationKind.REVIEW:
                    action = RuleActionCreateDataReviewAnnotation(
                        assignee=rule.get("assignee"), tags=tags
                    )

        if not name:
            raise ValueError(f"Couldn't find rule name '{name}' in rule_list: {rule_list}")

        return description, expression, action


class RuleAction(ABC):
    @abstractmethod
    def kind(self) -> RuleActionKind:
        pass


class RuleActionCreateDataReviewAnnotation(RuleAction):
    """
    Action to create a data-review annotation when a rule evaluates to a truthy value.

    - `tags`: List of tag names to associate with the newly created data-review annotation.
    - `assignee`: Email of user in organization to assign the newly created data-review annotation.
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

    - `tags`: List of tag names to associate with the newly created data-review annotation.
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


class ExpressionChannelReference(TypedDict):
    """
    `channel_reference`: The channel reference (e.g. '$1') used in the expression.
    `channel_identifier`: The fully qualified channel name. See `sift_py.ingestion.channel.channel_fqn`.
    """

    channel_reference: str
    channel_identifier: str


class ExpressionChannelReferenceChannelConfig(TypedDict):
    """
    `channel_reference`: The channel reference (e.g. '$1') used in the expression.
    `channel_config`: Instance of `sift_py.ingestion.channel.ChannelConfig`.
    """

    channel_reference: str
    channel_config: ChannelConfig
