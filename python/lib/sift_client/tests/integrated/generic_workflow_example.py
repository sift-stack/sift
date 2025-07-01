import asyncio
import os
from datetime import datetime, timedelta

from dotenv import load_dotenv
from sift_client.client import SiftClient

# Import sift_client types for calculated channels and rules
from sift_client.types import (
    CalculatedChannelUpdate,
    ChannelReference,
    RuleAction,
    RuleAnnotationType,
    RuleUpdate,
)
from sift_py.asset.service import AssetService

# TODO: Eventually these should all use sift_client replacements
from sift_py.data.service import DataService
from sift_py.grpc.transport import SiftChannelConfig, use_sift_async_channel, use_sift_channel

"""
Test script for generic workflow.

TODO: TBD if we move this to an example or keep it here as a test expected to be used just by us.

If we keep it as a test, we should ideally have a setup that populates data, and then ensure we teardown all the test assets/channels/rules etc.
"""


# TODO: Make sync
async def main():
    load_dotenv()
    grpc_url = os.getenv("BASE_URI", "localhost:50051")
    api_key = os.getenv("SIFT_API_KEY", "")
    rest_url = os.getenv("REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_DEV_API_KEY", "")
    rest_url = "https://api.development.siftstack.com"
    grpc_url = "grpc-api.development.siftstack.com"
    organization_id = "org-1234567890"
    channel_config: SiftChannelConfig = {
        "apikey": api_key,
        "uri": grpc_url,
    }
    channel_config["use_ssl"] = True
    # TODO: Add organization_id to client
    client = SiftClient(grpc_url=grpc_url, api_key=api_key, rest_url=rest_url)
    sift_sync_channel = use_sift_channel(channel_config)

    async with use_sift_async_channel(channel_config) as sift_connection:
        data_service = DataService(sift_connection)
        asset_service = AssetService(sift_sync_channel)
        # TODO: Replace this w/ new Asset type + resource
        assets = asset_service.list_assets(names=["NostromoLV426"])
        asset_id = assets[0].asset_id
        print("Found assets", [asset.name for asset in assets])

        # TODO: Replace w/ a search for asset based on part of name.

        # TODO: Search for channel based on part of name.
        # velocity_channel = None
        # voltage_channel = None
        # for channel in result.channels():
        #     if channel.name.contains("velocity"):
        #         velocity_channel = channel
        #     elif channel.name.contains("voltage"):
        #         voltage_channel = channel
        # if not velocity_channel or not voltage_channel:
        #     raise ValueError("Velocity and voltage channels not found")

        calculated_channels = client.calculated_channels.list(
            name_regex="velocity_per.*",
            asset_id=asset_id,
        )
        updated = False
        if calculated_channels:
            print(f"Found calculated channels: {[cc.name for cc in calculated_channels]}")
            for cc in calculated_channels:
                if cc.name == "velocity_per_voltage":
                    calculated_channel = cc.update(
                        CalculatedChannelUpdate(
                            expression="$1 / $2 + 0.1",
                            expression_channel_references=cc.expression_channel_references,
                        )
                    )
                    updated = True
                print("Updated", calculated_channel)
        else:
            # Create a calculated channel that divides mainmotor.velocity by voltage
            print("\nCreating calculated channel...")
            calculated_channel = await client.calculated_channels.create(
                name="velocity_per_voltage",
                description="Ratio of mainmotor velocity to voltage",
                expression="$1 / $2",  # $1 = mainmotor.velocity, $2 = voltage
                expression_channel_references=[
                    ChannelReference(reference_key="$1", channel_name="mainmotor.velocity"),
                    ChannelReference(reference_key="$2", channel_name="voltage"),
                ],
                units="velocity/voltage",
                asset_ids=[asset_id],
                user_notes="Created to monitor velocity-to-voltage ratio",
            )
            print(
                f"Created calculated channel: {calculated_channel.name} (ID: {calculated_channel.calculated_channel_id})"
            )
        # TODO: Check calculated channel data?

        # Create a rule that creates an annotation when the ratio is above 0.1
        rule_search = "high_velocity_voltage"
        print(f"Looking for rule containing {rule_search}")
        rules = client.rules.list(
            name_contains=rule_search,
        )
        # TODO: This doesn't work
        # rules = client.rules.search(
        #     name_contains=rule_search,
        #     asset_ids=[asset_id],
        # )
        if rules:
            print(f"Found rules: {[rule.name for rule in rules]}")
            # Example of batch get if you just had the rule ids:
            rules = client.rules.batch_get(rule_ids=[rule.rule_id for rule in rules])
            print(f"Batch get on IDs also works: {[rule.name for rule in rules]}")

            rule = rules[0]
            print(f"Updating rule: {rule.name}")
            rule = await client.rules.update(rule, RuleUpdate(description=f"Alert when velocity-to-voltage ratio exceeds 0.1 (Updated at {datetime.now().isoformat()})", asset_ids=[asset_id]))
        else:
            print(f"No rules found for {rule_search}")
            rules = client.rules.search(
                asset_ids=[asset_id],
            )
            if rules:
                print(f"However these rules do exist: {[rule.name for rule in rules]}")
            print("Attempting to create rule for high_velocity_voltage_ratio_alert")
            rule = client.rules.create(
                name="high_velocity_voltage_ratio_alert",
                description="Alert when velocity-to-voltage ratio exceeds 0.1",
                expression="$1 > 0.1",
                channel_references=[
                    ChannelReference(
                        channel_reference="$1", channel_identifier=calculated_channel.name
                    ),
                ],
                action=RuleAction.annotation(
                    annotation_type=RuleAnnotationType.DATA_REVIEW,
                    tags=["high_ratio", "alert"],
                    assignee=None,  # You can set a user ID here if needed
                ),
            )
            print(f"Created rule: {rule.name} (ID: {rule.rule_id})")

        # TODO: Update calculated channel
        # TODO: Update rule

        if updated:
            print("Second run through, deleting rule and calculated channel")
            # await client.rules.delete(rule.rule_id)
            await client.calculated_channels.delete(calculated_channel.calculated_channel_id)


if __name__ == "__main__":
    asyncio.run(main())
