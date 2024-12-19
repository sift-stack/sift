"""
Formal specification of the types that `sift_py` expects when loading a telemetry config from a YAML file.
"""

from __future__ import annotations

from typing import Dict, List

from typing_extensions import NotRequired, TypedDict

import sift_py.yaml.channel as channel_yaml
import sift_py.yaml.rule as rule_yaml

RuleYamlSpec = rule_yaml.RuleYamlSpec
NamedExpressionYamlSpec = rule_yaml.NamedExpressionYamlSpec
ChannelConfigYamlSpec = channel_yaml.ChannelConfigYamlSpec
ChannelEnumTypeYamlSpec = channel_yaml.ChannelEnumTypeYamlSpec
ChannelBitFieldElementYamlSpec = channel_yaml.ChannelBitFieldElementYamlSpec


class TelemetryConfigYamlSpec(TypedDict):
    """
    Formal spec that defines what the telemetry config should look like in YAML.

    `asset_name`: The name of the asset to telemeter.
    `ingestion_client_key`: User-defined string-key that uniquely identifies this telemetry config.
    `organization_id`: Optional ID of user's organization. Required if user belongs to multiple orgs.
    `channels`: Sensors that send the data.
    `rules`: Rules that, when evaluated to a true, will perform some sort of acction.
    `flows`: A list of named groups of channels that send data together.
    """

    asset_name: str
    ingestion_client_key: str
    organization_id: NotRequired[str]
    channels: Dict[str, ChannelConfigYamlSpec]
    rules: NotRequired[List[RuleYamlSpec]]
    flows: NotRequired[List[FlowYamlSpec]]


class FlowYamlSpec(TypedDict):
    """
    Formal spec that defines what a flow should look like in YAML.
    """

    name: str
    channels: List[ChannelConfigYamlSpec]


class YamlConfigError(Exception):
    """
    When the YAML config has missing or invalid properties.
    """

    message: str

    def __init__(self, message: str):
        super().__init__(message)
