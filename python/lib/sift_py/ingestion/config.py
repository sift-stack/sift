"""
Contains the in memory representation of a telemetry config as well as tools to initialize one
via a YAML file and future file formats in the future.
"""

from __future__ import annotations
from .channel import ChannelDataType, ChannelBitFieldElement, ChannelEnumType
from .error import YamlConfigError
from .flow import ChannelConfig, FlowConfig
from pathlib import Path
from sift_internal.types import any_as
from typing import Any, Dict, List, Optional

import yaml


class TelemetryConfig:
    """
    Configurations necessary to start ingestion.

    Attributes:
      asset_name: The name of the asset that you wish to telemeter data for.
      ingestion_client_key: An arbitrary string completely chosen by the user to uniquely identify
                  this ingestion configuration. It should be unique with respect to your
                  organization.

      flows: The list of `FlowConfig`. A single flow can specify a single channel value
             or a set of channel values, with each value belonging to a different channel. Channels
             that send data at the same frequency and time should be in the same flow.

      organization_id: ID of your organization in Sift. This field is only required if your user
                       belongs to multiple organizations
    """

    asset_name: str
    ingestion_client_key: str
    organization_id: Optional[str]
    flows: List[FlowConfig]

    def __init__(
        self,
        asset_name: str,
        ingestion_client_key: str,
        organization_id: Optional[str] = None,
        flows: List[FlowConfig] = [],
    ):
        self.asset_name = asset_name
        self.ingestion_client_key = ingestion_client_key
        self.organization_id = organization_id
        self.flows = flows


def try_load_from_yaml(config_fs_path: Path) -> TelemetryConfig:
    """
    Loads in YAML config file and deserializes it into an instance of `TelemetryConfig`. If
    the YAML config has any malformed or missing properties than a `YamlConfigError` is raised.
    """

    suffix = config_fs_path.suffix
    if suffix != ".yaml" and suffix != ".yml":
        raise YamlConfigError(f"Unsupported file-type '{suffix}', expected YAML.")

    with open(config_fs_path, "r") as file:
        content = file.read()
        return _try_from_yaml_str(content)


def _try_from_yaml_str(yaml_str: str) -> TelemetryConfig:
    config: Dict[Any, Any] = yaml.safe_load(yaml_str)

    asset_name = any_as(config.get("asset_name"), str)
    if asset_name is None or len(asset_name) == 0:
        raise YamlConfigError("Expected a non-blank string for top-level 'asset_name' property")

    ingestion_client_key = any_as(config.get("ingestion_client_key"), str)
    if ingestion_client_key is None or len(ingestion_client_key) == 0:
        raise YamlConfigError(
            "Expected a non-blank string top-level 'ingestion_client_key' property"
        )

    organization_id = any_as(config.get("organization_id"), str)

    raw_flows = any_as(config.get("flows"), list)
    if raw_flows is None:
        raise YamlConfigError("Expected 'flows' to be a list property")

    return TelemetryConfig(
        asset_name=asset_name,
        ingestion_client_key=ingestion_client_key,
        organization_id=organization_id,
        flows=_deserialize_flows_from_yaml(raw_flows),
    )


def _deserialize_flows_from_yaml(raw_flow_configs: List[Dict]) -> List[FlowConfig]:
    flow_configs = []

    for raw_flow_config in raw_flow_configs:
        flow_name = any_as(raw_flow_config.get("name"), str)
        if flow_name is None or len(flow_name) == 0:
            raise YamlConfigError("Expected flow to have a non-blank 'name' property")

        raw_channel_configs = any_as(raw_flow_config.get("channels"), list)
        if raw_channel_configs is None:
            raise YamlConfigError("Expected 'channels' to be a list property")

        flow_config = FlowConfig(
            name=flow_name, channels=_deserialize_channels_from_yaml(raw_channel_configs)
        )

        flow_configs.append(flow_config)

    return flow_configs


def _deserialize_channels_from_yaml(raw_channel_configs: List[Dict]) -> List[ChannelConfig]:
    channel_configs = []

    for raw_channel_config in raw_channel_configs:
        channel_name = any_as(raw_channel_config.get("name"), str)
        if channel_name is None or len(channel_name) == 0:
            raise YamlConfigError("Expected channel to have a non-blank 'name' property")

        channel_data_type_str = any_as(raw_channel_config.get("data_type"), str)
        if channel_data_type_str is None or len(channel_data_type_str) == 0:
            raise YamlConfigError("Missing property for 'flows.channel.data_type' property")

        channel_data_type = ChannelDataType.from_str(channel_data_type_str)
        if channel_data_type is None:
            raise YamlConfigError("Invalid property for 'flows.channel.data_type' property")

        description = any_as(raw_channel_config.get("description"), str)
        unit = any_as(raw_channel_config.get("unit"), str)
        component = any_as(raw_channel_config.get("component"), str)

        bit_field_elements = []
        raw_bit_field_elements = any_as(raw_channel_config.get("bit_field_elements"), list)
        if raw_bit_field_elements is not None:
            for element in raw_bit_field_elements:
                el = _deserialize_bit_field_element_from_yaml(element)
                bit_field_elements.append(el)

        enum_types = []
        raw_enum_types = any_as(raw_channel_config.get("enum_types"), list)
        if raw_enum_types is not None:
            for enum_type in raw_enum_types:
                etype = _deserialize_enum_type_from_yaml(enum_type)
                enum_types.append(etype)

        channel_config = ChannelConfig(
            name=channel_name,
            data_type=channel_data_type,
            description=description,
            unit=unit,
            component=component,
            bit_field_elements=bit_field_elements,
            enum_types=enum_types,
        )

        channel_configs.append(channel_config)

    return channel_configs


def _deserialize_bit_field_element_from_yaml(bit_field_element: Dict) -> ChannelBitFieldElement:
    name = any_as(bit_field_element.get("name"), str)
    if name is None or len(name) == 0:
        raise YamlConfigError(
            "Expected a non-blank value for 'flows.channels.bit_field_element.name'"
        )

    index = any_as(bit_field_element.get("index"), int)
    if index is None:
        raise YamlConfigError(
            "Expected an integer value for 'flows.channels.bit_field_element.index'"
        )

    bit_count = any_as(bit_field_element.get("bit_count"), int)
    if bit_count is None:
        raise YamlConfigError(
            "Expected an integer value for 'flows.channels.bit_field_element.bit_count'"
        )

    return ChannelBitFieldElement(
        name=name,
        index=index,
        bit_count=bit_count,
    )


def _deserialize_enum_type_from_yaml(enum_type: Any) -> ChannelEnumType:
    name = any_as(enum_type.get("name"), str)
    if name is None or len(name) == 0:
        raise YamlConfigError("Expected a non-blank value for 'flows.channels.enum_types.name'")

    key = any_as(enum_type.get("key"), int)
    if key is None:
        raise YamlConfigError("Expected an integer value for 'flows.channels.enum_types.key'")

    return ChannelEnumType(
        name=name,
        key=key,
    )
