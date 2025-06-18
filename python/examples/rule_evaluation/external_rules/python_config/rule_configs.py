import inspect
from pathlib import Path
from typing import List

from sift_py.ingestion.channel import (
    ChannelConfig,
    ChannelDataType,
    ChannelEnumType,
)
from sift_py.ingestion.rule.config import (
    RuleActionCreateDataReviewAnnotation,
    RuleConfig,
)


def load_nostromos_lv_426_rule_configs() -> List[RuleConfig]:
    """Returns a set of RuleConfigs that we want to evaluate."""
    # Filename and linenumber is passed into the Rule description for
    # traceability.
    frame = inspect.currentframe()
    filename = str(Path(inspect.getfile(frame)).resolve())
    filename = filename[filename.index("sift") :]

    log_channel = ChannelConfig(
        name="log",
        data_type=ChannelDataType.STRING,
        description="asset logs",
    )
    voltage_channel = ChannelConfig(
        name="voltage",
        data_type=ChannelDataType.INT_32,
        description="voltage at source",
        unit="Volts",
    )
    vehicle_state_channel = ChannelConfig(
        name="vehicle_state",
        data_type=ChannelDataType.ENUM,
        description="vehicle state",
        enum_types=[
            ChannelEnumType(name="Accelerating", key=0),
            ChannelEnumType(name="Decelerating", key=1),
            ChannelEnumType(name="Stopped", key=2),
        ],
    )

    rule_configs = [
        RuleConfig(
            name="overheating",
            description=f"Checks for vehicle overheating (Created from: {filename}:{frame.f_lineno})",
            expression='$1 == "Accelerating" && $2 > 80',
            asset_names=["NostromoLV426"],
            channel_references=[
                # INFO: Can use either a channel idenfier string or a ChannelConfig
                {
                    "channel_reference": "$1",
                    "channel_identifier": vehicle_state_channel.identifier,
                },
                {
                    "channel_reference": "$2",
                    "channel_config": voltage_channel,
                },
            ],
            action=RuleActionCreateDataReviewAnnotation(),
        ),
        RuleConfig(
            name="kinetic_energy",
            description=f"Tracks high energy output while in motion (Created from: {filename}:{frame.f_lineno})",
            expression="0.5 * $mass * $1 * $1 > $threshold",
            asset_names=["NostromoLV426"],
            channel_references=[
                {
                    "channel_reference": "$1",
                    "channel_config": voltage_channel,
                },
            ],
            sub_expressions={
                "$mass": 10,
                "$threshold": 470,
            },
            action=RuleActionCreateDataReviewAnnotation(
                # User in your organization to notify
                # assignee="ellen.ripley@weylandcorp.com",
                tags=["nostromo"],
            ),
        ),
        RuleConfig(
            name="failure",
            description=f"Checks for failures reported by logs (Created from: {filename}:{frame.f_lineno})",
            expression="contains($1, $sub_string)",
            asset_names=["NostromoLV426"],
            contextual_channels=[vehicle_state_channel.name],
            channel_references=[
                {
                    "channel_reference": "$1",
                    "channel_config": log_channel,
                },
            ],
            sub_expressions={
                "$sub_string": "failure",
            },
            action=RuleActionCreateDataReviewAnnotation(
                # User in your organization to notify
                # assignee="ellen.ripley@weylandcorp.com",
                tags=["nostromo", "failure"],
            ),
        ),
    ]

    # Set these as external rules.
    for rule_config in rule_configs:
        rule_config.is_external = True

    return rule_configs
