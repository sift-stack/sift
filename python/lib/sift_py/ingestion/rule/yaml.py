from __future__ import annotations
from .config import (
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleConfig,
    RuleActionAnnotationKind,
)
from ..error import YamlConfigError
from sift_internal.types import any_as
from typing import Dict, List, Literal, TypedDict
from typing_extensions import NotRequired


class RulesYamlSpec(TypedDict):
    rules: List[RuleYamlSpec]


class RuleYamlSpec(TypedDict):
    """
    The formal definition of what a single rule looks like in YAML.
    """

    name: str
    description: NotRequired[str]
    expression: str | NamedExpressionYamlSpec
    annotation_type: Literal["phase"] | Literal["review"]
    assignee: NotRequired[str]
    tags: NotRequired[List[str]]


class NamedExpressionYamlSpec(TypedDict):
    """
    A named, reusable expression. This class is the formal definition
    of what a named expression should look like in YAML.
    """

    name: str
    identifiers: Dict[str, str]


"""
NamedExpressionsYamlSpec is a type alias for a dictionary where both keys and values are strings.
Note the pluralization in the name to distinguish it from `NamedExpressionYamlSpec`.

This alias serves as a formal definition for a YAML file that solely contains named expressions.
See `sift_py.ingestion.rule.yaml_test.py` for examples.

Named expressions are generic expressions that contain placeholders instead of identifiers. They can
be loaded at runtime and referenced in telemetry configs to facilitate reuse.
"""
NamedExpressionsYamlSpec = Dict[str, str]


def rule_config_from_yaml(
    rule_yaml: RuleYamlSpec,
    named_expressions: Dict[str, str] = {},
) -> RuleConfig:
    rule_name = any_as(rule_yaml.get("name"), str)
    if rule_name is None or len(rule_name) == 0:
        raise YamlConfigError("Expected rule to have a 'name' property.")

    description = any_as(rule_yaml.get("description"), str) or ""

    raw_annotation_type = any_as(rule_yaml.get("type"), str)
    if raw_annotation_type is None:
        raise YamlConfigError(f"Expected ruled '{rule_name} to have a 'type' property.")

    annotation_type = RuleActionAnnotationKind.from_str(raw_annotation_type)

    expression = rule_yaml.get("expression")

    if expression is None:
        raise YamlConfigError(f"Expected rule '{rule_name}' to have an expression.")

    if isinstance(expression, str):
        if annotation_type == RuleActionAnnotationKind.REVIEW:
            return RuleConfig(
                name=rule_name,
                description=description,
                expression=expression,
                action=RuleActionCreateDataReviewAnnotation(
                    assignee=rule_yaml.get("assignee"),
                    tags=rule_yaml.get("tags"),
                ),
            )
        else:
            return RuleConfig(
                name=rule_name,
                description=description,
                expression=expression,
                action=RuleActionCreatePhaseAnnotation(
                    tags=rule_yaml.get("tags"),
                ),
            )
    elif isinstance(expression, dict):
        expression_name = expression.get("name")
        if expression_name is None:
            raise YamlConfigError("Expected named expression to have a 'name' property.")

        named_expression = named_expressions.get(expression_name)
        if named_expression is None:
            raise YamlConfigError(
                f"Failed to find named expression '{expression_name}' for rule '{rule_name}'."
            )

        ident_map = expression.get("identifiers", {})

        if annotation_type == RuleActionAnnotationKind.REVIEW:
            return RuleConfig(
                name=rule_name,
                description=description,
                expression=named_expression,
                action=RuleActionCreateDataReviewAnnotation(
                    assignee=rule_yaml.get("assignee"),
                    tags=rule_yaml.get("tags"),
                ),
                ident_map=ident_map,
            )
        else:
            return RuleConfig(
                name=rule_name,
                description=description,
                expression=named_expression,
                action=RuleActionCreatePhaseAnnotation(
                    tags=rule_yaml.get("tags"),
                ),
                ident_map=ident_map,
            )
    else:
        raise YamlConfigError(
            f"Expected rule '{rule_name}' 'expression' property to be a string or have properties."
        )
