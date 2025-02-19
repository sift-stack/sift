from pathlib import Path
from typing import Any, Dict, List, cast

import yaml

from sift_py.calculated_channels.config import CalculatedChannelConfig
from sift_py.ingestion.config.yaml.error import YamlConfigError
from sift_py.yaml.utils import _handle_subdir


def load_calculated_channels(paths: List[Path]) -> List[CalculatedChannelConfig]:
    """
    Takes in a list of paths to YAML files which contains calculated channel configs and processes them into a list of
    `CalculatedChannelConfig` objects. For more information on report templates see
    `sift_py.report_templates.config.CalculatedChannelConfig`.
    """
    calculated_channel_configs: List[CalculatedChannelConfig] = []

    def update_calculated_channels(path: Path):
        calculated_channel_configs.extend(_read_calculated_channels_yaml(path))

    for path in paths:
        if path.is_dir():
            _handle_subdir(path, update_calculated_channels)
        elif path.is_file():
            update_calculated_channels(path)
    return calculated_channel_configs


def _read_calculated_channels_yaml(path: Path) -> List[CalculatedChannelConfig]:
    calculated_channel_configs = []
    with open(path, "r") as f:
        channel_config_yaml = cast(Dict[str, Any], yaml.safe_load(f.read()))

        calculated_channel_list = channel_config_yaml.get("calculated_channels", [])
        for calc_channel in calculated_channel_list:
            if not isinstance(calc_channel, dict):
                raise YamlConfigError(
                    f"Expected 'calculated_channels' to be a list of dictionaries in yaml: '{path}'"
                )
            for channel_ref in calc_channel.get("channel_references", []):
                parsed_channel_refs = []
                if not isinstance(channel_ref, dict):
                    raise YamlConfigError(
                        f"Expected 'channel_references' to be a list of dictionaries in yaml: '{path}'"
                    )
                if "channel_reference" not in channel_ref:
                    for k, v in channel_ref.items():
                        parsed_channel_refs.append(dict(channel_reference=k, channel_identifier=v))
                else:
                    parsed_channel_refs.append(channel_ref)
                calc_channel["channel_references"] = parsed_channel_refs

        if not isinstance(calculated_channel_list, list):
            raise YamlConfigError(f"Expected 'calculated_channels' to be a list in yaml: '{path}'")

        for calc_channel in calculated_channel_list:
            try:
                calc_channel_cfg = CalculatedChannelConfig(**calc_channel)
                calculated_channel_configs.append(calc_channel_cfg)
            except Exception as e:
                raise YamlConfigError(f"Error parsing calculated channel '{calc_channel}'") from e

        return calculated_channel_configs
