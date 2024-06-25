from pathlib import Path

from sift_py.ingestion.channel import (
    ChannelBitFieldElement,
    ChannelConfig,
    ChannelDataType,
    ChannelEnumType,
)
from sift_py.ingestion.config.telemetry import FlowConfig, TelemetryConfig
from sift_py.ingestion.config.yaml.load import load_named_expression_modules
from sift_py.ingestion.rule.config import (
    RuleActionCreateDataReviewAnnotation,
    RuleConfig,
)

EXPRESSION_MODULES_DIR = Path().joinpath("expression_modules")


def nostromos_lv_426() -> TelemetryConfig:
    named_expressions = load_named_expression_modules(
        [
            EXPRESSION_MODULES_DIR.joinpath("kinematics.yml"),
            EXPRESSION_MODULES_DIR.joinpath("string.yml"),
        ]
    )

    log_channel = ChannelConfig(
        name="log",
        data_type=ChannelDataType.STRING,
        description="asset logs",
    )
    velocity_channel = ChannelConfig(
        name="velocity",
        data_type=ChannelDataType.DOUBLE,
        description="speed",
        unit="Miles Per Hour",
        component="mainmotor",
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
    gpio_channel = ChannelConfig(
        name="gpio",
        data_type=ChannelDataType.BIT_FIELD,
        description="on/off values for pins on gpio",
        bit_field_elements=[
            ChannelBitFieldElement(name="12v", index=0, bit_count=1),
            ChannelBitFieldElement(name="charge", index=1, bit_count=2),
            ChannelBitFieldElement(name="led", index=3, bit_count=4),
            ChannelBitFieldElement(name="heater", index=7, bit_count=1),
        ],
    )

    return TelemetryConfig(
        asset_name="NostromoLV426",
        ingestion_client_key="nostromo_lv_426",
        rules=[
            RuleConfig(
                name="overheating",
                description="Checks for vehicle overheating",
                expression='$1 == "Accelerating" && $2 > 80',
                channel_references=[
                    # INFO: Can use either "channel_identifier" or "channel_config"
                    {
                        "channel_reference": "$1",
                        "channel_identifier": vehicle_state_channel.fqn(),
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
                description="Tracks high energy output while in motion",
                expression=named_expressions["kinetic_energy_gt"],
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
                description="Checks for failures reported by logs",
                expression=named_expressions["log_substring_contains"],
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
        ],
        flows=[
            FlowConfig(
                name="readings",
                channels=[
                    velocity_channel,
                    voltage_channel,
                    vehicle_state_channel,
                    gpio_channel,
                ],
            ),
            FlowConfig(
                name="voltage",
                channels=[voltage_channel],
            ),
            FlowConfig(
                name="gpio_channel",
                channels=[gpio_channel],
            ),
            FlowConfig(name="logs", channels=[log_channel]),
        ],
    )
