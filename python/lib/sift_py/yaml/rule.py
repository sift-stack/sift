from __future__ import annotations

import re
from pathlib import Path
from typing import Any, Dict, List, Literal, Union, cast

import yaml
from typing_extensions import NotRequired, TypedDict

from sift_py.ingestion.config.yaml.error import YamlConfigError
from sift_py.rule.config import RuleActionAnnotationKind
from sift_py.yaml.channel import (
    ChannelConfigYamlSpec,
    _validate_channel_reference,
)
from sift_py.yaml.utils import _handle_subdir, _type_fqn

_SUB_EXPRESSION_REGEX = re.compile(r"^\$[a-zA-Z_]+$")


def load_named_expression_modules(paths: List[Path]) -> Dict[str, str]:
    """
    Takes in a list of paths to YAML files which contains named expressions and processes them into a `dict`.
    The key is the name of the expression and the value is the expression itself. For more information on
    named expression modules see `sift_py/yaml/rule.py`.
    """

    named_expressions = {}

    for path in paths:
        named_expr_module = _read_named_expression_module_yaml(path)

        for name, expr in named_expr_module.items():
            if name in named_expressions:
                raise YamlConfigError(
                    f"Encountered expressions with identical names being loaded, '{name}'."
                )
            named_expressions[name] = expr

    return named_expressions


def load_rule_modules(paths: List[Path]) -> List[RuleYamlSpec]:
    """
    Takes in a list of paths which may either be directories or files containing rule module YAML files,
    and processes them into a `list`. For more information on rule modules see
    RulemoduleYamlSpec in `sift_py/yaml/rule.py`.
    """

    rule_modules: List[RuleYamlSpec] = []

    def update_rule_modules(rule_module_path: Path):
        rule_module = _read_rule_module_yaml(rule_module_path)
        rule_modules.extend(rule_module)

    for path in paths:
        if path.is_dir():
            _handle_subdir(path, update_rule_modules)
        elif path.is_file():
            update_rule_modules(path)

    return rule_modules


def _read_named_expression_module_yaml(path: Path) -> Dict[str, str]:
    with open(path, "r") as f:
        named_expressions = cast(Dict[Any, Any], yaml.safe_load(f.read()))

        for key, value in named_expressions.items():
            if not isinstance(key, str):
                raise YamlConfigError(
                    f"Expected '{key}' to be a string in named expression module '{path}'."
                )
            if not isinstance(value, str):
                raise YamlConfigError(
                    f"Expected expression of '{key}' to be a string in named expression module '{path}'."
                )

        return cast(Dict[str, str], named_expressions)


def _read_rule_module_yaml(path: Path) -> List[RuleYamlSpec]:
    with open(path, "r") as f:
        module_rules = cast(Dict[Any, Any], yaml.safe_load(f.read()))
        rules = module_rules.get("rules")
        if not isinstance(rules, list):
            raise YamlConfigError(
                f"Expected '{rules}' to be a list in rule module yaml: '{path}'"
                f"{_type_fqn(RuleYamlSpec)}"
            )

        for rule in cast(List[Any], rules):
            _validate_rule(rule)

        return cast(List[RuleYamlSpec], rules)


def _validate_rule(val: Any):
    rule = cast(Dict[Any, Any], val)

    name = rule.get("name")

    if not isinstance(name, str):
        raise YamlConfigError._invalid_property(name, "- name", "str", ["rules"])

    channel_references = rule.get("channel_references")

    if channel_references is not None:
        if not isinstance(channel_references, list):
            raise YamlConfigError._invalid_property(
                channel_references,
                "- channel_references",
                f"List[Dict[str, {_type_fqn(ChannelConfigYamlSpec)}]]",
                ["rules"],
            )

        for channel_reference in cast(List[Any], channel_references):
            _validate_channel_reference(channel_reference)

    rule_client_key = rule.get("rule_client_key")
    description = rule.get("description")
    expression = rule.get("expression")
    rule_type = rule.get("type")
    assignee = rule.get("assignee")
    tags = rule.get("tags")
    sub_expressions = rule.get("sub_expressions")
    asset_names = rule.get("asset_names")
    tag_names = rule.get("tag_names")

    if rule_client_key is not None and not isinstance(rule_client_key, str):
        raise YamlConfigError._invalid_property(
            rule_client_key, "- rule_client_key", "str", ["rules"]
        )

    if description is not None and not isinstance(description, str):
        raise YamlConfigError._invalid_property(description, "- description", "str", ["rules"])

    if isinstance(expression, dict):
        expression_name = cast(Dict[Any, Any], expression).get("name")

        if not isinstance(expression_name, str):
            raise YamlConfigError._invalid_property(
                expression_name,
                "name",
                "str",
                ["rules", "- expression"],
            )

    elif not isinstance(expression, str):
        raise YamlConfigError._invalid_property(
            expression,
            "- expression",
            "<class 'str'> | <class 'dict'>",
            ["rules"],
        )

    valid_rule_types = [kind.value for kind in RuleActionAnnotationKind]

    if rule_type not in valid_rule_types:
        raise YamlConfigError._invalid_property(
            rule_type,
            "- type",
            " | ".join(valid_rule_types),
            ["rules"],
        )

    if assignee is not None and not isinstance(assignee, str):
        raise YamlConfigError._invalid_property(
            assignee,
            "- assignee",
            "str",
            ["rules"],
        )

    if tags is not None and not isinstance(tags, list):
        raise YamlConfigError._invalid_property(
            tags,
            "- tags",
            "List[str]",
            ["rules"],
        )

    if sub_expressions is not None:
        if not isinstance(channel_references, list):
            raise YamlConfigError._invalid_property(
                channel_references,
                "- sub_expressions",
                "List[Dict[str, List[Dict[str, str]]]]",
                ["rules"],
            )

        for sub_expression in cast(List[Any], sub_expressions):
            _validate_sub_expression(sub_expression)

    if asset_names is not None and not isinstance(asset_names, list):
        raise YamlConfigError._invalid_property(
            asset_names,
            "- asset_names",
            "List[str]",
            ["rules"],
        )

    if tag_names is not None and not isinstance(tag_names, list):
        raise YamlConfigError._invalid_property(
            tag_names,
            "- tag_names",
            "List[str]",
            ["rules"],
        )


def _validate_sub_expression(val: Any):
    sub_expression = cast(Dict[Any, Any], val)

    for key in sub_expression.keys():
        if not isinstance(key, str):
            raise YamlConfigError._invalid_property(
                sub_expression,
                "- <str>",
                "Dict[str, Any]",
                ["rules", "- sub_expressions"],
            )

        if _SUB_EXPRESSION_REGEX.match(key) is None:
            raise YamlConfigError(
                f"Invalid sub-expression key, '{key}'. Characters must be in the character set [a-zA-Z_] and prefixed with a '$'."
            )


class RuleModuleYamlSpec(TypedDict):
    """
    The formal definition of what a rule module looks like in YAML.

    `rules`: A list of rules that belong to the module.
    """

    rules: List[RuleYamlSpec]


class RuleYamlSpec(TypedDict):
    """
    The formal definition of what a single rule looks like in YAML.

    `name`: Name of the rule.
    `rule_client_key`: User-defined string-key that uniquely identifies this rule config.
    `description`: Description of rule.
    `expression`:
        Either an expression-string or a `sift_py.ingestion.config.yaml.spec.NamedExpressionYamlSpec` referencing a named expression.
    `type`: Determines the action to perform if a rule gets evaluated to true.
    `assignee`: If `type` is `review`, determines who to notify. Expects an email.
    `tags`: Tags to associate with the rule.
    `channel_references`: A list of channel references that maps to an actual channel. More below.
    `sub_expressions`: A list of sub-expressions which is a mapping of place-holders to sub-expressions. Only used if using named expressions.
    `asset_names`: A list of asset names that this rule should be applied to. ONLY VALID if defining rules outside of a telemetry config.
    `tag_names`: A list of tag names that this rule should be applied to. ONLY VALID if defining rules outside of a telemetry config.

    Channel references:
    A channel reference is a string containing a numerical value prefixed with "$". Examples include "$1", "$2", "$11", and so on.
    The channel reference is mapped to an actual channel config. In YAML it would look something like this:

    ```yaml
    channel_references:
      - $1: *vehicle_state_channel
      - $2: *voltage_channel
    ```

    Sub-expressions:
    A sub-expression is made up of two components: A reference and the actual sub-expression. The sub-expression reference is
    a string with a "$" prepended to another string comprised of characters in the following character set: `[a-zA-Z0-9_]`.
    This reference should be mapped to the actual sub-expression. For example, say you have kinematic equations in `kinematics.yml`,
    and the equation you're interested in using looks like the following:

    ```yaml
    kinetic_energy_gt:
      0.5 * $mass * $1 * $1 > $threshold
    ```

    To properly use `kinetic_energy_gt` in your rule, it would look like the following:

    ```yaml
    rules:
      - name: kinetic_energy
        description: Tracks high energy output while in motion
        type: review
        assignee: bob@example.com
        expression:
          name: kinetic_energy_gt
        channel_references:
          - $1: *velocity_channel
        sub_expressions:
          - $mass: 10
          - $threshold: 470
        tags:
            - nostromo
    ```
    """

    name: str
    rule_client_key: NotRequired[str]
    description: NotRequired[str]
    expression: Union[str, NamedExpressionYamlSpec]
    type: Union[Literal["phase"], Literal["review"]]
    assignee: NotRequired[str]
    tags: NotRequired[List[str]]
    channel_references: NotRequired[List[Dict[str, ChannelConfigYamlSpec]]]
    sub_expressions: NotRequired[List[Dict[str, str]]]
    asset_names: NotRequired[List[str]]
    tag_names: NotRequired[List[str]]


class NamedExpressionYamlSpec(TypedDict):
    """
    A named expression. This class is the formal definition of what a named expression
    should look like in YAML. The value of `name` may contain a mix of channel references
    and channel identifiers.

    For a formal definition of channel references and channel identifiers see the following:
    `sift_py.ingestion.config.yaml.spec.RuleYamlSpec`.
    """

    name: str
