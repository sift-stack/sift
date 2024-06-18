from __future__ import annotations

from collections.abc import Iterable
from pathlib import Path
from typing import Dict, List, Literal, Optional, TypedDict, cast

import yaml
from typing_extensions import NotRequired

from ..channel import ChannelBitFieldElement, ChannelDataType, ChannelEnumType, channel_fqn
from ..flow import ChannelConfig, FlowConfig
from ..rule.config import (
    RuleActionAnnotationKind,
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleConfig,
)
from .telemetry import TelemetryConfig


class TelemetryConfigYamlSpec(TypedDict):
    """
    Formal spec that defines what the telemetry config should look like in YAML.
    """

    asset_name: str
    ingestion_client_key: str
    organization_id: NotRequired[str]
    channels: Dict[str, ChannelConfigYamlSpec]
    rules: NotRequired[List[RuleYamlSpec]]
    flows: NotRequired[List[FlowYamlSpec]]


class ChannelConfigYamlSpec(TypedDict):
    """
    Formal spec that defines what a channel should look like in YAML.
    """

    name: str
    description: NotRequired[str]
    unit: NotRequired[str]
    component: NotRequired[str]
    data_type: (
        Literal["double"]
        | Literal["string"]
        | Literal["enum"]
        | Literal["bit_field"]
        | Literal["bool"]
        | Literal["float"]
        | Literal["int32"]
        | Literal["int64"]
        | Literal["uint32"]
        | Literal["uint64"]
    )
    enum_types: NotRequired[List[ChannelEnumTypeYamlSpec]]
    bit_field_elements: NotRequired[List[ChannelBitFieldElementYamlSpec]]


class ChannelEnumTypeYamlSpec(TypedDict):
    """
    Formal spec that defines what a channel enum type should look like in YAML.
    """

    name: str
    key: int


class ChannelBitFieldElementYamlSpec(TypedDict):
    """
    Formal spec that defines what a bit-field element should look like in YAML.
    """

    name: str
    index: int
    bit_count: int


class FlowYamlSpec(TypedDict):
    """
    Formal spec that defines what a flow should look like in YAML.
    """

    name: str
    channels: List[ChannelConfigYamlSpec]


class YamlLoadOptions(TypedDict):
    """
    Options to use when loading a telemetry config form YAML.

    Attributes:
        `named_expressions`:
            A list of look up paths for YAML files containing named expressions. Could also just be a YAML str.
    """

    named_expressions: List[Path | str]


class RulesYamlSpec(TypedDict):
    rules: List[RuleYamlSpec]


class RuleYamlSpec(TypedDict):
    """
    The formal definition of what a single rule looks like in YAML.
    """

    name: str
    description: NotRequired[str]
    expression: str | NamedExpressionYamlSpec
    type: Literal["phase"] | Literal["review"]
    assignee: NotRequired[str]
    tags: NotRequired[List[str]]
    channel_references: NotRequired[List[Dict[str, ChannelConfigYamlSpec]]]
    sub_expressions: NotRequired[List[Dict[str, str]]]


class NamedExpressionYamlSpec(TypedDict):
    """
    A named, reusable expression. This class is the formal definition
    of what a named expression should look like in YAML.
    """

    name: str


"""
NamedExpressionsYamlSpec is a type alias for a dictionary where both keys and values are strings.
Note the pluralization in the name to distinguish it from `NamedExpressionYamlSpec`.

This alias serves as a formal definition for a YAML file that solely contains named expressions.
See `sift_py.ingestion.rule.yaml_test.py` for examples.

Named expressions are generic expressions that contain placeholders instead of identifiers. They can
be loaded at runtime and referenced in telemetry configs to facilitate reuse.
"""
NamedExpressionsYamlSpec = Dict[str, str]

class YamlConfigError(Exception):
    """
    When the YAML config has missing or invalid properties.
    """
    message: str

    def __init__(self, message: str):
        super().__init__(message)


def try_load_from_yaml(
    config_fs_path: Path, opts: Optional[YamlLoadOptions] = None
) -> TelemetryConfig:
    """
    Loads in YAML config file and deserializes it into an instance of `TelemetryConfig`. If
    the YAML config has any malformed or missing properties than a `YamlConfigError` is raised.
    """

    suffix = config_fs_path.suffix
    if suffix != ".yaml" and suffix != ".yml":
        raise YamlConfigError(f"Unsupported file-type '{suffix}', expected YAML.")

    with open(config_fs_path, "r") as file:
        content = file.read()
        return _try_from_yaml_str(content, opts)


def rule_config_from_yaml(
    rule_yaml: RuleYamlSpec,
    named_expressions: Dict[str, str] = {},
) -> RuleConfig:
    """
    Creates a `RuleConfig` from a `rule_yaml` and an optional `named_expressions` dictionary
    if generic named expressions are used.
    """

    rule_name = rule_yaml.get("name")
    if rule_name is None or len(rule_name) == 0:
        raise YamlConfigError("Expected rule to have a 'name' property.")

    description = rule_yaml.get("description") or ""

    raw_annotation_type = rule_yaml.get("type")
    if raw_annotation_type is None:
        raise YamlConfigError(f"Expected ruled '{rule_name} to have a 'type' property.")

    annotation_type = RuleActionAnnotationKind.from_str(raw_annotation_type)

    expression = rule_yaml.get("expression")

    if expression is None:
        raise YamlConfigError(f"Expected rule '{rule_name}' to have an expression.")

    raw_channel_references = rule_yaml.get("channel_references", [])
    channel_references = {}
    for raw_channel_reference in raw_channel_references:
        for reference, channel_config in raw_channel_reference.items():
            channel_references[reference] = _deserialize_channel_from_yaml(channel_config)

    raw_sub_expressions = rule_yaml.get("sub_expressions", [])
    sub_expressions = {}
    for raw_sub_expression in raw_sub_expressions:
        for reference, value in raw_sub_expression.items():
            sub_expressions[reference] = value

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
                channel_references=channel_references,
                sub_expressions=sub_expressions,
            )
        else:
            return RuleConfig(
                name=rule_name,
                description=description,
                expression=expression,
                action=RuleActionCreatePhaseAnnotation(
                    tags=rule_yaml.get("tags"),
                ),
                channel_references=channel_references,
                sub_expressions=sub_expressions,
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

        if annotation_type == RuleActionAnnotationKind.REVIEW:
            return RuleConfig(
                name=rule_name,
                description=description,
                expression=named_expression,
                action=RuleActionCreateDataReviewAnnotation(
                    assignee=rule_yaml.get("assignee"),
                    tags=rule_yaml.get("tags"),
                ),
                channel_references=channel_references,
                sub_expressions=sub_expressions,
            )
        else:
            return RuleConfig(
                name=rule_name,
                description=description,
                expression=named_expression,
                action=RuleActionCreatePhaseAnnotation(
                    tags=rule_yaml.get("tags"),
                ),
                channel_references=channel_references,
                sub_expressions=sub_expressions,
            )
    else:
        raise YamlConfigError(
            f"Expected rule '{rule_name}' 'expression' property to be a string or have properties."
        )


def try_load_named_expressions_from_yaml(
    named_expressions_fs_path: Path,
) -> NamedExpressionsYamlSpec:
    """
    Loads in named expressions from a file.
    """

    suffix = named_expressions_fs_path.suffix
    if suffix != ".yaml" and suffix != ".yml":
        raise YamlConfigError(f"Unsupported file-type '{suffix}', expected YAML.")

    with open(named_expressions_fs_path, "r") as file:
        content = file.read()
        return cast(NamedExpressionsYamlSpec, yaml.safe_load(content))


def _try_from_yaml_str(yaml_str: str, opts: Optional[YamlLoadOptions] = None) -> TelemetryConfig:
    config: TelemetryConfigYamlSpec = yaml.safe_load(yaml_str)

    asset_name = config.get("asset_name")
    if asset_name is None or len(asset_name) == 0:
        raise YamlConfigError("Expected a non-blank string for top-level 'asset_name' property.")

    ingestion_client_key = config.get("ingestion_client_key")
    if ingestion_client_key is None or len(ingestion_client_key) == 0:
        raise YamlConfigError(
            "Expected a non-blank string top-level 'ingestion_client_key' property."
        )

    organization_id = config.get("organization_id")

    raw_channels = config.get("channels")
    if raw_channels is None or len(raw_channels) == 0:
        raise YamlConfigError("Expected a top-level non-empty 'channels' property.")

    channels = [_deserialize_channel_from_yaml(c) for c in raw_channels.values()]
    channels_by_fqn = {channel_fqn(c): c for c in channels}

    raw_flows = config.get("flows")
    if raw_flows is None:
        raise YamlConfigError("Expected 'flows' to be a list property.")

    named_expressions = {}
    if opts is not None:
        for named_expr in opts.get("named_expressions", []):
            named_expressions_from_yaml = {}

            if isinstance(named_expr, str):
                named_expressions_from_yaml = cast(
                    NamedExpressionsYamlSpec, yaml.safe_load(named_expr)
                )
            else:
                named_expressions_from_yaml = try_load_named_expressions_from_yaml(named_expr)

            for name, expression in named_expressions_from_yaml.items():
                if name in named_expressions:
                    raise YamlConfigError(
                        f"Found multiple named expressions with the name '{name}'."
                    )
                named_expressions[name] = expression

    raw_rules = config.get("rules")
    rules = []
    if raw_rules is not None and len(raw_rules) > 0:
        for raw_rule in raw_rules:
            rule = rule_config_from_yaml(raw_rule, named_expressions)
            rules.append(rule)

    return TelemetryConfig(
        asset_name=asset_name,
        ingestion_client_key=ingestion_client_key,
        organization_id=organization_id,
        flows=_deserialize_flows_from_yaml(raw_flows, channels_by_fqn),
        rules=rules,
    )


def _deserialize_flows_from_yaml(
    raw_flow_configs: Iterable[FlowYamlSpec],
    channels_by_fqn: Dict[str, ChannelConfig],
) -> List[FlowConfig]:
    flow_configs = []

    for raw_flow_config in raw_flow_configs:
        flow_name = raw_flow_config.get("name")
        if flow_name is None or len(flow_name) == 0:
            raise YamlConfigError("Expected flow to have a non-blank 'name' property")

        raw_channel_configs = raw_flow_config.get("channels")
        if raw_channel_configs is None:
            raise YamlConfigError("Expected 'channels' to be a list property")

        channels = [_deserialize_channel_from_yaml(c) for c in raw_channel_configs]
        seen_channels = set()

        for channel in channels:
            fqn = channel_fqn(channel)
            if fqn not in channels_by_fqn:
                raise YamlConfigError(
                    f"Flow '{flow_name}' contains channel '{fqn}' that is missing from top-level 'channels' property."
                )
            if fqn in seen_channels:
                raise YamlConfigError(
                    f"Channel '{fqn}' cannot appear more than once for flow '{flow_name}'."
                )
            seen_channels.add(fqn)

        flow_config = FlowConfig(name=flow_name, channels=channels)
        flow_configs.append(flow_config)

    return flow_configs


def _deserialize_channel_from_yaml(
    raw_channel_config: ChannelConfigYamlSpec,
) -> ChannelConfig:
    channel_name = raw_channel_config.get("name")
    if channel_name is None or len(channel_name) == 0:
        raise YamlConfigError("Expected channel to have a non-blank 'name' property")

    channel_data_type_str = raw_channel_config.get("data_type")
    if channel_data_type_str is None or len(channel_data_type_str) == 0:
        raise YamlConfigError("Missing property for 'flows.channel.data_type' property")

    channel_data_type = ChannelDataType.from_str(channel_data_type_str)
    if channel_data_type is None:
        raise YamlConfigError("Invalid property for 'flows.channel.data_type' property")

    description = raw_channel_config.get("description")
    unit = raw_channel_config.get("unit")
    component = raw_channel_config.get("component")

    bit_field_elements = []
    raw_bit_field_elements = raw_channel_config.get("bit_field_elements")
    if raw_bit_field_elements is not None:
        for element in raw_bit_field_elements:
            el = _deserialize_bit_field_element_from_yaml(element)
            bit_field_elements.append(el)

    enum_types = []
    raw_enum_types = raw_channel_config.get("enum_types")
    if raw_enum_types is not None:
        for enum_type in raw_enum_types:
            etype = _deserialize_enum_type_from_yaml(enum_type)
            enum_types.append(etype)

    return ChannelConfig(
        name=channel_name,
        data_type=channel_data_type,
        description=description,
        unit=unit,
        component=component,
        bit_field_elements=bit_field_elements,
        enum_types=enum_types,
    )


def _deserialize_bit_field_element_from_yaml(
    bit_field_element: ChannelBitFieldElementYamlSpec,
) -> ChannelBitFieldElement:
    name = bit_field_element.get("name")
    if name is None or len(name) == 0:
        raise YamlConfigError(
            "Expected a non-blank value for 'flows.channels.bit_field_element.name'"
        )

    index = bit_field_element.get("index")
    if index is None:
        raise YamlConfigError(
            "Expected an integer value for 'flows.channels.bit_field_element.index'"
        )

    bit_count = bit_field_element.get("bit_count")
    if bit_count is None:
        raise YamlConfigError(
            "Expected an integer value for 'flows.channels.bit_field_element.bit_count'"
        )

    return ChannelBitFieldElement(
        name=name,
        index=index,
        bit_count=bit_count,
    )


def _deserialize_enum_type_from_yaml(enum_type: ChannelEnumTypeYamlSpec) -> ChannelEnumType:
    name = enum_type.get("name")
    if name is None or len(name) == 0:
        raise YamlConfigError("Expected a non-blank value for 'flows.channels.enum_types.name'")

    key = enum_type.get("key")
    if key is None:
        raise YamlConfigError("Expected an integer value for 'flows.channels.enum_types.key'")

    return ChannelEnumType(
        name=name,
        key=key,
    )
