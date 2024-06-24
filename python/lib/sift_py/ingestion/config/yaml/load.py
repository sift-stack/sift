import re
from pathlib import Path
from typing import Any, Dict, List, Type, cast

import yaml

from sift_py.ingestion.channel import ChannelDataTypeStrRep
from sift_py.ingestion.config.yaml.error import YamlConfigError
from sift_py.ingestion.config.yaml.spec import (
    ChannelBitFieldElementYamlSpec,
    ChannelConfigYamlSpec,
    ChannelEnumTypeYamlSpec,
    FlowYamlSpec,
    RuleYamlSpec,
    TelemetryConfigYamlSpec,
)
from sift_py.ingestion.rule.config import RuleActionAnnotationKind

_CHANNEL_REFERENCE_REGEX = re.compile(r"^\$\d+$")
_SUB_EXPRESSION_REGEX = re.compile(r"^\$[a-zA-Z_]+$")


def read_and_validate(path: Path) -> TelemetryConfigYamlSpec:
    """
    Reads in the telemetry config YAML file found at `path` and validates it. Any errors that may occur at the parsing
    step will return an error whose source is the `yaml` package. Any errors that may occur during the
    validation step will return a `sift_py.ingestion.config.yaml.error.YamlConfigError`.
    """
    raw_config = _read_yaml(path)
    return _validate_yaml(raw_config)


def load_named_expression_modules(paths: List[Path]) -> Dict[str, str]:
    """
    Takes in a list of paths to YAML files which contains named expressions and processes them into a `dict`.
    The key is the name of the expression and the value is the expression itself. For more information on
    named expression modules see `sift_py.ingestion/config/yaml/spec.py
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


def _validate_yaml(raw_config: Dict[Any, Any]) -> TelemetryConfigYamlSpec:
    asset_name = raw_config.get("asset_name")

    if not isinstance(asset_name, str):
        raise YamlConfigError._invalid_property(asset_name, "asset_name", "str")

    ingestion_client_key = raw_config.get("ingestion_client_key")

    if not isinstance(ingestion_client_key, str):
        raise YamlConfigError._invalid_property(ingestion_client_key, "ingestion_client_key", "str")

    organization_id = raw_config.get("organization_id")

    if organization_id is not None and not isinstance(organization_id, str):
        raise YamlConfigError._invalid_property(ingestion_client_key, "organization_id", "str")

    channels = raw_config.get("channels")

    if channels is not None:
        if not isinstance(channels, dict):
            raise YamlConfigError._invalid_property(
                channels,
                "channels",
                f"Dict[str, {ChannelConfigYamlSpec}]",
                None,
            )

        for anchor, channel_config in cast(Dict[Any, Any], channels).items():
            _validate_channel_anchor(anchor)
            _validate_channel(channel_config)

    rules = raw_config.get("rules")

    if rules is not None:
        if not isinstance(rules, list):
            raise YamlConfigError._invalid_property(
                channels,
                "channels",
                f"List[{_type_fqn(RuleYamlSpec)}]",
                None,
            )

        for rule in cast(List[Any], rules):
            _validate_rule(rule)

    flows = raw_config.get("flows")

    if flows is not None:
        if not isinstance(flows, list):
            raise YamlConfigError._invalid_property(
                flows,
                "flows",
                f"List[{_type_fqn(FlowYamlSpec)}]",
                None,
            )

        for flow in cast(List[Any], flows):
            _validate_flow(flow)

    return cast(TelemetryConfigYamlSpec, raw_config)


def _read_yaml(path: Path) -> Dict[Any, Any]:
    with open(path, "r") as f:
        return cast(Dict[Any, Any], yaml.safe_load(f.read()))


def _validate_channel_anchor(val: Any):
    if not isinstance(val, str):
        raise YamlConfigError._invalid_property(
            val,
            "<str>",
            "&str",
            ["channels"],
        )


def _validate_channel(val: Any):
    channel = cast(Dict[Any, Any], val)

    name = channel.get("name")

    if not isinstance(name, str):
        raise YamlConfigError._invalid_property(name, "- name", "str", ["channels"])

    description = channel.get("description")

    if description is not None and not isinstance(description, str):
        raise YamlConfigError._invalid_property(description, "- description", "str", ["channels"])

    unit = channel.get("unit")

    if unit is not None and not isinstance(unit, str):
        raise YamlConfigError._invalid_property(unit, "- unit", "str", ["channels"])

    component = channel.get("component")

    if component is not None and not isinstance(component, str):
        raise YamlConfigError._invalid_property(component, "- component", "str", ["channels"])

    data_type = channel.get("data_type")
    valid_data_type_values = [v.value for v in ChannelDataTypeStrRep]

    if not data_type in valid_data_type_values:
        raise YamlConfigError._invalid_property(
            data_type,
            "- data_type",
            " | ".join(valid_data_type_values),
            ["channels"],
        )

    if data_type == ChannelDataTypeStrRep.ENUM.value:
        enum_types = channel.get("enum_types")

        if not isinstance(enum_types, list):
            raise YamlConfigError._invalid_property(
                enum_types,
                "- enum_types",
                f"List<{_type_fqn(ChannelEnumTypeYamlSpec)}>",
                ["channels"],
            )

        for enum_type in cast(List[Any], enum_types):
            _validate_enum_type(enum_type)

    elif data_type == ChannelDataTypeStrRep.BIT_FIELD.value:
        bit_field_elements = channel.get("bit_field_elements")

        if not isinstance(bit_field_elements, list):
            raise YamlConfigError._invalid_property(
                bit_field_elements,
                "- bit_field_elements",
                f"List<{_type_fqn(ChannelBitFieldElementYamlSpec)}>",
            )

        for bit_field_element in cast(List[Any], bit_field_elements):
            _validate_bit_field_element(bit_field_element)

    else:
        enum_types = channel.get("enum_types")

        if enum_types is not None:
            raise YamlConfigError(
                f"Channel of data-type '{data_type}' should not have 'enum_types' set."
            )

        bit_field_elements = channel.get("bit_field_elements")

        if bit_field_elements is not None:
            raise YamlConfigError(
                f"Channel of data-type '{data_type}' should not have 'bit_field_elements' set."
            )


def _validate_enum_type(val: Any):
    enum_type = cast(Dict[Any, Any], val)

    name = enum_type.get("name")

    if not isinstance(name, str):
        raise YamlConfigError._invalid_property(
            name,
            "- name",
            "str",
            ["channels", "- enum_type"],
        )

    key = enum_type.get("key")

    if not isinstance(key, int):
        raise YamlConfigError._invalid_property(
            key,
            "- key",
            "int",
            ["channels", "- enum_type"],
        )


def _validate_bit_field_element(val: Any):
    bit_field_element = cast(Dict[Any, Any], val)

    name = bit_field_element.get("name")

    if not isinstance(name, str):
        raise YamlConfigError._invalid_property(
            name, "- name", "str", ["channels", "- bit_field_elements"]
        )

    index = bit_field_element.get("index")

    if not isinstance(index, int):
        raise YamlConfigError._invalid_property(
            name, "- index", "int", ["channels", "- bit_field_elements"]
        )

    bit_count = bit_field_element.get("bit_count")

    if not isinstance(bit_count, int):
        raise YamlConfigError._invalid_property(
            name, "- bit_count", "int", ["channels", "- bit_field_elements"]
        )


def _validate_rule(val: Any):
    rule = cast(Dict[Any, Any], val)

    name = rule.get("name")

    if not isinstance(name, str):
        raise YamlConfigError._invalid_property(name, "- name", "str", ["rules"])

    description = rule.get("description")

    if description is not None and not isinstance(description, str):
        raise YamlConfigError._invalid_property(description, "- description", "str", ["rules"])

    expression = rule.get("expression")

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

    rule_type = rule.get("type")
    valid_rule_types = [kind.value for kind in RuleActionAnnotationKind]

    if rule_type not in valid_rule_types:
        raise YamlConfigError._invalid_property(
            rule_type,
            "- type",
            " | ".join(valid_rule_types),
            ["rules"],
        )

    assignee = rule.get("assignee")

    if assignee is not None and not isinstance(assignee, str):
        raise YamlConfigError._invalid_property(
            assignee,
            "- assignee",
            "str",
            ["rules"],
        )

    tags = rule.get("tags")

    if tags is not None and not isinstance(tags, list):
        raise YamlConfigError._invalid_property(
            tags,
            "- tags",
            "List[str]",
            ["rules"],
        )

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

    sub_expressions = rule.get("sub_expressions")

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


def _validate_channel_reference(val: Any):
    channel_reference = cast(Dict[Any, Any], val)

    for key, value in channel_reference.items():
        if not isinstance(key, str):
            raise YamlConfigError._invalid_property(
                channel_reference,
                "- <str>",
                f"Dict[str, {_type_fqn(ChannelConfigYamlSpec)}]",
                ["rules", "- channel_references"],
            )

        if _CHANNEL_REFERENCE_REGEX.match(key) is None:
            raise YamlConfigError(
                f"Invalid channel reference key '{key}'. Expected an integer prefixed with '$' e.g. '$1', '$2', and so on."
            )

        try:
            _validate_channel(value)
        except YamlConfigError as err:
            raise YamlConfigError(f"Rule '{key}' contains an invalid channel reference:\n{err}")


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


def _validate_flow(val: Any):
    flow = cast(Dict[Any, Any], val)

    name = flow.get("name")

    if not isinstance(name, str):
        raise YamlConfigError._invalid_property(
            name,
            "- name",
            "str",
            ["flows"],
        )

    channels = flow.get("channels")

    if channels is not None:
        if not isinstance(channels, list):
            raise YamlConfigError._invalid_property(
                channels,
                "channels",
                f"List<{ChannelConfigYamlSpec}>",
                ["flows"],
            )

        for channel in cast(List[Any], channels):
            try:
                _validate_channel(channel)
            except YamlConfigError as err:
                raise YamlConfigError(
                    f"Flow '{name}' contains an invalid channel reference:\n{err}"
                )


def _type_fqn(typ: Type) -> str:
    return f"{typ.__module__}.{typ.__name__}"
