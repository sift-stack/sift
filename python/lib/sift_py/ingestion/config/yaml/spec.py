"""
Formal specification of the types that `sift_py` expects when loading
a telemetry config from a YAML file.
"""

from __future__ import annotations

from pathlib import Path
from typing import Dict, List, Literal, TypedDict

from typing_extensions import NotRequired, TypeAlias


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

Named expressions are expressions that contain placeholders which are replaced with actual
expressions at runtime.
"""
NamedExpressionsYamlSpec: TypeAlias = Dict[str, str]


class YamlConfigError(Exception):
    """
    When the YAML config has missing or invalid properties.
    """

    message: str

    def __init__(self, message: str):
        super().__init__(message)
