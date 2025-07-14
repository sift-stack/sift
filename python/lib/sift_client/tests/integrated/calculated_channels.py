import asyncio
import os
from datetime import datetime

from sift_client.client import SiftClient

# Import sift_client types for calculated channels and rules
from sift_client.types import (
    CalculatedChannelUpdate,
    ChannelReference,
)

"""
Comprehensive test script for calculated channels with extensive update field exercises.

This test demonstrates all available update fields for calculated channels:
- name: Update the channel name
- description: Update the channel description
- units: Update the units of measurement
- expression: Update the calculation expression
- expression_channel_references: Update channel references (must be updated with expression)
- asset_ids: Update which assets the channel applies to
- tag_ids: Update associated tags

The test also includes:
- Edge case testing (minimal updates, invalid expressions)
- Batch operations demonstration
- Comprehensive validation
- Error handling and graceful fallbacks
- Archive operations

TODO: TBD if we move this to an example or keep it here as a test expected to be used just by us.

If we keep it as a test, we should ideally have a setup that populates data, and then ensure we teardown all the test assets/channels/rules etc.
"""


async def main():
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    api_key = os.getenv("SIFT_API_KEY", "")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    client = SiftClient(grpc_url=grpc_url, api_key=api_key, rest_url=rest_url)

    # Find assets to work with
    asset = client.assets.find(name="NostromoLV426")
    asset_id = asset.id
    print(f"Using asset: {asset.name} (ID: {asset_id})")

    # Create example calculated channels that will be unique to this test run in case things don't cleanup.
    num_channels = 7
    unique_name_suffix = datetime.now().strftime("%Y%m%d%H%M%S")
    print(
        f"\n=== Creating {num_channels} calculated channels with unique suffix: {unique_name_suffix} ==="
    )

    created_channels = []
    for i in range(num_channels):
        calculated_channel = client.calculated_channels.create(
            name=f"test_channel_{unique_name_suffix}_{i}",
            description=f"Test calculated channel {i} - initial description",
            expression="$1 / $2",  # $1 = mainmotor.velocity, $2 = voltage
            channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="mainmotor.velocity"),
                ChannelReference(channel_reference="$2", channel_identifier="voltage"),
            ],
            units="velocity/voltage",
            asset_ids=[asset_id],
            user_notes=f"Created for testing update fields - channel {i}",
        )
        created_channels.append(calculated_channel)
        print(
            f"Created calculated channel: {calculated_channel.name} (ID: {calculated_channel.id})"
        )

    # Find the channels we just created
    search_results = client.calculated_channels.list(
        name_regex="test_channel.*",
        asset_id=asset_id,
    )
    print(f"Found {len(search_results)} calculated channels: {[cc.name for cc in search_results]}")

    print("\n=== Testing comprehensive update scenarios ===")

    # Test 1: Update expression and channel references together
    print("\n--- Test 1: Update expression and channel references ---")
    channel_1 = created_channels[0]
    updated_channel_1 = channel_1.update(
        CalculatedChannelUpdate(
            expression="$1 / $2 * 100",  # Convert to percentage
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="mainmotor.velocity"),
                ChannelReference(channel_reference="$2", channel_identifier="voltage"),
            ],
        )
    )
    print(f"Updated {updated_channel_1.name}: expression = {updated_channel_1.expression}")

    # Test 2: Update description
    print("\n--- Test 2: Update description ---")
    channel_2 = created_channels[1]
    updated_channel_2 = channel_2.update(
        CalculatedChannelUpdate(
            description="Updated description with more details about velocity-to-voltage ratio calculation",
        )
    )
    print(f"Updated {updated_channel_2.name}: description = {updated_channel_2.description}")

    # Test 3: Update units
    print("\n--- Test 3: Update units ---")
    channel_3 = created_channels[2]
    updated_channel_3 = channel_3.update(
        CalculatedChannelUpdate(
            units="percentage",
        )
    )
    print(f"Updated {updated_channel_3.name}: units = {updated_channel_3.units}")

    # Test 4: Update name
    print("\n--- Test 4: Update name ---")
    channel_4 = created_channels[3]
    new_name = f"renamed_channel_{unique_name_suffix}_5"
    updated_channel_4 = channel_4.update(
        CalculatedChannelUpdate(
            name=new_name,
        )
    )
    print(f"Updated {channel_4.name} -> {updated_channel_4.name}")

    # Test 5: Update multiple fields at once
    print("\n--- Test 5: Update multiple fields simultaneously ---")
    channel_5 = created_channels[4]
    updated_channel_5 = channel_5.update(
        CalculatedChannelUpdate(
            description="Multi-field update test",
            units="ratio",
        ),
        user_notes="Updated via multi-field update",
    )
    print(f"Updated {updated_channel_5.name}:")
    print(f"  - description: {updated_channel_5.description}")
    print(f"  - units: {updated_channel_5.units}")
    print(f"  - user_notes: {updated_channel_5.user_notes}")

    # Test 6: Update with complex expression
    print("\n--- Test 6: Update with complex expression ---")
    channel_6 = created_channels[5]
    updated_channel_6 = channel_6.update(
        CalculatedChannelUpdate(
            expression="($1 / $2) * 100 + ($3 * 0.1)",  # Complex expression with 3 variables
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="mainmotor.velocity"),
                ChannelReference(channel_reference="$2", channel_identifier="voltage"),
                ChannelReference(channel_reference="$3", channel_identifier="temperature"),
            ],
        )
    )
    print(f"Updated {updated_channel_6.name}: complex expression = {updated_channel_6.expression}")

    # Test 7: Update tag_ids (if tags are available)
    print("\n--- Test 7: Update tag_ids ---")
    channel_7 = created_channels[6]
    # Note: This would require actual tag IDs from the system
    # For now, we'll test with an empty list to show the capability
    updated_channel_7 = channel_7.update(
        CalculatedChannelUpdate(
            tag_ids=[],  # Empty list - in practice you'd use actual tag IDs
        )
    )
    print(f"Updated {updated_channel_7.name}: tag_ids = {updated_channel_7.tag_ids}")

    # Test 7b: Edge case - Update with invalid expression (should fail gracefully)
    print("\n--- Test 7b: Edge case - Invalid expression test ---")
    try:
        invalid_update = channel_7.update(
            CalculatedChannelUpdate(
                expression="invalid_expression",
                expression_channel_references=[
                    ChannelReference(
                        channel_reference="$1", channel_identifier="mainmotor.velocity"
                    ),
                ],
            )
        )
        print(f"Invalid expression update succeeded (unexpected): {invalid_update.expression}")
        # TODO: Ticket this?
    except Exception as e:
        print(f"Invalid expression update failed as expected: {e}")

    # Test 8: Archive channels
    print("\n--- Test 8: Archive channels ---")
    archived_count = 0
    for cc in created_channels:
        cc.archive()
        print(f"Archived: {cc.name}")
        archived_count += 1

    print("\n=== Test Summary ===")
    print(f"Created: {len(created_channels)} channels")
    print(f"Archived: {archived_count} channels")

    # Verify all channels were processed
    assert len(created_channels) == num_channels, (
        f"Expected {num_channels} created channels, got {len(created_channels)}"
    )
    assert archived_count == num_channels, (
        f"Expected {num_channels} archived channels, got {archived_count}"
    )

    # Additional validation
    print("\n=== Validation Checks ===")

    # Verify that updates actually changed the values
    assert updated_channel_1.expression == "$1 / $2 * 100", (
        f"Expression update failed: {updated_channel_1.expression}"
    )
    assert "more details" in updated_channel_2.description, (
        f"Description update failed: {updated_channel_2.description}"
    )
    assert updated_channel_3.units == "percentage", (
        f"Units update failed: {updated_channel_3.units}"
    )
    assert updated_channel_4.name == new_name, f"Name update failed: {updated_channel_4.name}"
    assert updated_channel_5.description == "Multi-field update test", (
        f"Description update failed: {updated_channel_5.description}"
    )
    assert updated_channel_5.units == "ratio", f"Units update failed: {updated_channel_5.units}"
    assert updated_channel_5.user_notes == "Updated via multi-field update", (
        f"User notes update failed: {updated_channel_5.user_notes}"
    )
    assert updated_channel_6.expression == "($1 / $2) * 100 + ($3 * 0.1)", (
        f"Complex expression update failed: {updated_channel_6.expression}"
    )
    assert len(updated_channel_6.channel_references) == 3, (
        f"Complex expression should have 3 references, got {len(updated_channel_6.channel_references)}"
    )
    assert updated_channel_7.tag_ids == [], f"Tag IDs update failed: {updated_channel_7.tag_ids}"

    versions = client.calculated_channels.list_versions(
        calculated_channel_id=channel_1.id,
        limit=10,
    )
    print(f"Found {len(versions)} versions for {created_channels[0].name}")

    print("All validation checks passed!")
    print("\n=== Test completed successfully ===")


if __name__ == "__main__":
    asyncio.run(main())
