from pathlib import Path
from typing import Any, Dict, List, cast

import yaml

import sift_py.yaml.rule as rule_yaml
from sift_py.ingestion.config.yaml.error import YamlConfigError
from sift_py.ingestion.config.yaml.spec import (
    FlowYamlSpec,
    TelemetryConfigYamlSpec,
)
from sift_py.yaml.channel import ChannelConfigYamlSpec, _validate_channel, _validate_channel_anchor
from sift_py.yaml.rule import RuleYamlSpec
from sift_py.yaml.utils import _type_fqn

load_named_expression_modules = rule_yaml.load_named_expression_modules


def read_and_validate(path: Path) -> TelemetryConfigYamlSpec:
    """
    Reads in the telemetry config YAML file found at `path` and validates it. Any errors that may occur at the parsing
    step will return an error whose source is the `yaml` package. Any errors that may occur during the
    validation step will return a `sift_py.ingestion.config.yaml.error.YamlConfigError`.
    """
    raw_config = _read_yaml(path)
    return _validate_yaml(raw_config)


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
                rules,
                "rules",
                f"List[{_type_fqn(RuleYamlSpec)}]",
                None,
            )

        for rule in cast(List[Any], rules):
            rule_yaml._validate_rule(rule)

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
