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
from sift_py.report_templates.config import ReportTemplateConfig

EXPRESSION_MODULES_DIR = Path().joinpath("expression_modules")


def load_rules() -> List[RuleConfig]:
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

    rules = [
        RuleConfig(
            name="overheating",
            description="Checks for vehicle overheating",
            expression='$1 == "Accelerating" && $2 > 80',
            rule_client_key="overheating-rule",
            asset_names=["NostromoLV2024"],
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
            expression="0.5 * $mass * $1 * $1 > $threshold",
            rule_client_key="kinetic-energy-rule",
            asset_names=["NostromoLV2024"],
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
            expression="contains($1, $sub_string)",
            rule_client_key="failure-rule",
            asset_names=["NostromoLV2024"],
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
    return rules


def nostromos_report_template() -> ReportTemplateConfig:
    return ReportTemplateConfig(
        name="Nostromo Report Template for PR test",
        template_client_key="report-template-test-1001",
        description="A report template for the Nostromo",
        rule_client_keys=[],
    )
