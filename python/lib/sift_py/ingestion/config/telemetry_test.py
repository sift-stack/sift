import pytest
from sift_py.ingestion.channel import ChannelConfig, ChannelDataType
from sift_py.ingestion.config.telemetry import TelemetryConfig, TelemetryConfigValidationError
from sift_py.ingestion.flow import FlowConfig
from sift_py.ingestion.rule.config import RuleActionCreateDataReviewAnnotation, RuleConfig


def test_telemetry_config_validations_duplicate_rules():
    channel = ChannelConfig(
        name="my_channel",
        data_type=ChannelDataType.DOUBLE,
    )

    rule_on_my_channel_a = RuleConfig(
        name="rule_a",
        description="",
        expression="$1 > 10",
        channel_references={
            "$1": channel,
        },
        action=RuleActionCreateDataReviewAnnotation(
            assignee="bob@example.com",
            tags=["barometer"],
        ),
    )

    another_rule_on_my_channel_a = RuleConfig(
        name="rule_a",  # same name
        description="",
        expression="$1 > 11",
        channel_references={
            "$1": channel,
        },
        action=RuleActionCreateDataReviewAnnotation(
            assignee="bob@example.com",
            tags=["barometer"],
        ),
    )

    with pytest.raises(TelemetryConfigValidationError, match="Can't have two rules"):
        TelemetryConfig(
            asset_name="my_asset",
            ingestion_client_key="my_asset_key",
            organization_id="my_organization_id",
            flows=[
                FlowConfig(
                    name="my_flow",
                    channels=[channel],
                )
            ],
            rules=[rule_on_my_channel_a, another_rule_on_my_channel_a],
        )


def test_telemetry_config_validations_duplicate_channels():
    channel = ChannelConfig(
        name="my_channel",
        data_type=ChannelDataType.DOUBLE,
    )

    with pytest.raises(TelemetryConfigValidationError, match="Can't have two identical channels"):
        TelemetryConfig(
            asset_name="my_asset",
            ingestion_client_key="my_asset_key",
            organization_id="my_organization_id",
            flows=[
                FlowConfig(
                    name="my_flow",
                    channels=[
                        channel,
                        channel,
                    ],
                )
            ],
        )


def test_telemetry_config_validations_flows_with_same_name():
    channel = ChannelConfig(
        name="my_channel",
        data_type=ChannelDataType.DOUBLE,
    )

    channel_b = ChannelConfig(
        name="my_other_channel",
        data_type=ChannelDataType.DOUBLE,
    )

    with pytest.raises(TelemetryConfigValidationError, match="Can't have two flows"):
        TelemetryConfig(
            asset_name="my_asset",
            ingestion_client_key="my_asset_key",
            organization_id="my_organization_id",
            flows=[
                FlowConfig(
                    name="my_flow",
                    channels=[channel],
                ),
                FlowConfig(
                    name="my_flow",
                    channels=[channel_b],
                ),
            ],
        )
