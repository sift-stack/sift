from __future__ import annotations

from abc import ABC, abstractmethod
from enum import Enum
from typing import Any, Dict, List, Optional, Union, cast

from sift.annotations.v1.annotations_pb2 import AnnotationType
from sift.rules.v1.rules_pb2 import ActionKind
from typing_extensions import TypedDict

from sift_py._internal.convert.json import AsJson
from sift_py.ingestion.channel import ChannelConfig


class RuleConfig(AsJson):
    """
    Defines a rule to be used during ingestion. If a rule's expression validates to try, then
    a specific action will take place as specified by the `kind` attribute.

    - `name`: Name of the rule.
    - `description`: Description of the rule.
    - `expression`: A CEL string expression that executes the `action` when evaluated to a truthy value.
    - `action`: The action to execute if the result of an `expression` evaluates to a truthy value.
    - `channel_references`: Reference to channel. If an expression is "$1 < 10", then "$1" is the reference and thus should the key in the dict.
    - `rule_client_key`: User defined unique string that uniquely identifies this rule.
    - `asset_names`: A list of asset names that this rule should be applied to. ONLY VALID if defining rules outside of a telemetry config.
    - `tag_names`: A list of asset names that this rule should be applied to. ONLY VALID if defining rules outside of a telemetry config.
    """

    name: str
    description: str
    expression: str
    action: Optional[RuleAction]
    channel_references: List[ExpressionChannelReference]
    rule_client_key: Optional[str]
    asset_names: List[str]

    def __init__(
        self,
        name: str,
        channel_references: List[
            Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]
        ],
        description: str = "",
        expression: str = "",
        action: Optional[RuleAction] = None,
        rule_client_key: Optional[str] = None,
        asset_names: Optional[List[str]] = None,
        tag_names: Optional[List[str]] = None,
        sub_expressions: Dict[str, Any] = {},
    ):
        self.channel_references = _channel_references_from_dicts(channel_references)

        self.name = name
        self.asset_names = asset_names or []
        self.action = action
        self.rule_client_key = rule_client_key
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
        expression: str, sub_expressions: Optional[Dict[str, str]]
    ) -> str:
        if sub_expressions:
            for ref, expr in sub_expressions.items():
                if ref not in expression:
                    raise ValueError(f"Couldn't find '{ref}' in expression '{expression}'.")
                if isinstance(expr, str):
                    expression = expression.replace(ref, f'"{expr}"')
                else:
                    expression = expression.replace(ref, str(expr))

        return expression


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


class RuleActionAnnotationKind(Enum):
    REVIEW = "review"
    PHASE = "phase"

    @classmethod
    def from_annotation_type(cls, annotation_type: AnnotationType) -> "RuleActionAnnotationKind":
        if annotation_type == AnnotationType.ANNOTATION_TYPE_PHASE:
            return cls.PHASE
        return cls.REVIEW

    @classmethod
    def from_str(cls, val: str) -> "RuleActionAnnotationKind":
        if val == cls.REVIEW.value:
            return cls.REVIEW
        elif val == cls.PHASE.value:
            return cls.PHASE
        else:
            raise ValueError(f"Argument '{val}' is not a valid annotation kind.")


class RuleActionKindStrRep(Enum):
    NOTIFICATION = "notification"
    ANNOTATION = "annotation"


class ExpressionChannelReference(TypedDict):
    """
    `channel_reference`: The channel reference (e.g. '$1') used in the expression.
    `channel_identifier`: The channel name.
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


def _channel_references_from_dicts(
    channel_references: List[
        Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]
    ],
) -> List[ExpressionChannelReference]:
    out: List[ExpressionChannelReference] = []
    for channel_reference in channel_references:
        config = channel_reference.get("channel_config")

        if config is not None:
            config = cast(ChannelConfig, config)

            out.append(
                {
                    "channel_reference": channel_reference["channel_reference"],
                    "channel_identifier": config.fqn(),
                }
            )
        else:
            channel_ref = cast(ExpressionChannelReference, channel_reference)

            out.append(
                {
                    "channel_reference": channel_ref["channel_reference"],
                    "channel_identifier": channel_ref["channel_identifier"],
                }
            )
    return out
