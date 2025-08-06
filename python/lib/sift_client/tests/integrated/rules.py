import os
from datetime import datetime

from sift_client.client import SiftClient

# Import sift_client types for calculated channels and rules
from sift_client.types import (
    ChannelReference,
    RuleAction,
    RuleAnnotationType,
    RuleUpdate,
)

"""
Comprehensive test script for rules with extensive update field exercises.

This test demonstrates all available update fields for rules:
- name: Update the rule name
- description: Update the rule description
- expression: Update the rule expression
- channel_references: Update channel references (must be updated with expression)
- action: Update the rule action (annotation, notification, webhook)
- tag_ids: Update associated tags (TBD)
- contextual_channels: Update contextual channels
- version_notes: Update version notes

The test also includes:
- Edge case testing (invalid expressions)
- Batch operations demonstration
- Comprehensive validation
- Archive operations


If we keep it as a test, we should ideally have a setup that populates data, and then ensure we teardown all the test assets/channels/rules etc.
"""


def main():
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    api_key = os.getenv("SIFT_API_KEY", "")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    client = SiftClient(grpc_url=grpc_url, api_key=api_key, rest_url=rest_url)

    asset = client.assets.find(name="NostromoLV426")
    asset_id = asset.id
    print(f"Using asset: {asset.name} (ID: {asset_id})")

    unique_name_suffix = datetime.now().strftime("%Y%m%d%H%M%S")
    num_rules = 8
    print(f"\n=== Creating {num_rules} rules with unique suffix: {unique_name_suffix} ===")
    created_rules = []
    for i in range(num_rules):
        rule = client.rules.create(
            name=f"test_rule_{unique_name_suffix}_{i}",
            description=f"Test rule {i} - initial description",
            expression="$1 > 0.1",  # Simple threshold check
            channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="mainmotor.velocity"),
            ],
            action=RuleAction.annotation(
                annotation_type=RuleAnnotationType.DATA_REVIEW,
                tags=["test", "initial"],
                default_assignee_user_id=None,
            ),
            asset_ids=[asset_id],
        )
        created_rules.append(rule)
        print(f"Created rule: {rule.name} (ID: {rule.rule_id})")

    # Find the rules we just created
    search_results = client.rules.list(
        name_regex=f"test_rule_{unique_name_suffix}.*",
    )
    assert len(search_results) == num_rules, (
        f"Expected {num_rules} created rules, got {len(search_results)}"
    )

    print("\n=== Testing comprehensive update scenarios ===")

    # Test 1: Update expression and channel references together
    print("\n--- Test 1: Update expression and channel references ---")
    rule_1 = created_rules[0]
    rule_1_model_dump = rule_1.model_dump()
    updated_rule_1 = rule_1.update(
        RuleUpdate(
            expression="$1 > 0.5",  # Higher threshold
            channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="mainmotor.velocity"),
            ],
        )
    )
    updated_rule_1_model_dump = updated_rule_1.model_dump()
    print(f"Updated {updated_rule_1.name}: expression = {updated_rule_1.expression}")

    # Test 2: Update description
    print("\n--- Test 2: Update description ---")
    rule_2 = created_rules[1]
    updated_rule_2 = rule_2.update(
        RuleUpdate(
            description="Updated description with more details about velocity-to-voltage ratio monitoring",
        )
    )
    print(f"Updated {updated_rule_2.name}: description = {updated_rule_2.description}")

    # Test 3: Update action (change annotation type and tags)
    print("\n--- Test 3: Update action ---")
    rule_3 = created_rules[2]
    updated_rule_3 = rule_3.update(
        RuleUpdate(
            action=RuleAction.annotation(
                annotation_type=RuleAnnotationType.PHASE,
                tags=["updated", "phase", "alert"],
                default_assignee_user_id=rule_3.created_by_user_id,
            ),
        )
    )
    print(f"Updated {updated_rule_3.name}: action type = {updated_rule_3.action.action_type}")
    print(f"  - annotation type: {updated_rule_3.action.annotation_type}")
    print(f"  - tags: {updated_rule_3.action.tags}")
    print(f"  - assignee: {updated_rule_3.action.default_assignee_user_id}")

    # Test 4: Update name
    print("\n--- Test 4: Update name ---")
    rule_4 = created_rules[3]
    new_name = f"renamed_rule_{unique_name_suffix}_4"
    updated_rule_4 = rule_4.update(
        RuleUpdate(
            name=new_name,
        )
    )
    print(f"Updated {rule_4.name} -> {updated_rule_4.name}")

    # Test 5: Update multiple fields at once
    print("\n--- Test 5: Update multiple fields simultaneously ---")
    rule_5 = created_rules[4]
    updated_rule_5 = rule_5.update(
        RuleUpdate(
            description="Multi-field update test",
        ),
        version_notes="Updated via multi-field update",
    )
    print(f"Updated {updated_rule_5.name}:")
    print(f"  - description: {updated_rule_5.description}")
    print(
        f"  - version_notes: {updated_rule_5.rule_version.version_notes if updated_rule_5.rule_version else None}"
    )

    # Test 6: Update with complex expression
    print("\n--- Test 6: Update with complex expression ---")
    rule_6 = created_rules[5]
    updated_rule_6 = rule_6.update(
        RuleUpdate(
            expression="$1 > 0.3 && $1 < 0.8",  # Range check
            channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier="mainmotor.velocity"),
            ],
        )
    )
    print(f"Updated {updated_rule_6.name}: complex expression = {updated_rule_6.expression}")

    # Test 7: Update action to notification type
    print("\n--- Test 7: Update action to notification ---")
    rule_7 = created_rules[6]
    updated_rule_7 = rule_7
    # Note: Notification actions are not supported yet.
    # updated_rule_7 = rule_7.update(
    #     RuleUpdate(
    #         action=RuleAction.notification(
    #             notify_recipients=[rule_7.created_by_user_id]
    #         ),
    #     )
    # )
    # print(f"Updated {updated_rule_7.name}: action type = {updated_rule_7.action.action_type}")
    # print(f"  - notification recipients: {updated_rule_7.action.notification_recipients}")

    # Test 8: Update tag_ids and contextual_channels
    print("\n--- Test 8: Update tag_ids and contextual_channels ---")
    rule_8 = created_rules[7]
    updated_rule_8 = rule_8.update(
        RuleUpdate(
            # tag_ids=["tag-123", "tag-456"],  # Example tag IDs # TODO: Where are these IDs supposed to come from? They're supposed to be uuids? {grpc_message:"invalid argument: invalid input syntax for type uuid: \"tag-123\"
            contextual_channels=["temperature", "pressure"],  # Example contextual channels
        )
    )
    print(f"Updated {updated_rule_8.name}:")
    print(f"  - asset_tag_ids: {updated_rule_8.asset_tag_ids}")
    print(f"  - contextual_channels: {updated_rule_8.contextual_channels}")

    # Test 8b: Edge case - Update with invalid expression (should fail gracefully)
    print("\n--- Test 8b: Edge case - Invalid expression test ---")
    try:
        invalid_update = rule_8.update(
            RuleUpdate(
                expression="invalid_expression",
                channel_references=[
                    ChannelReference(
                        channel_reference="$1", channel_identifier="mainmotor.velocity"
                    ),
                ],
            )
        )
        print(f"Invalid expression update succeeded (unexpected): {invalid_update.expression}")
    except Exception as e:
        print(f"Invalid expression update failed as expected: {e}")

    # Test 9: Batch operations demonstration
    print("\n--- Test 9: Batch operations demonstration ---")
    all_updated_rules = [
        updated_rule_1,
        updated_rule_2,
        updated_rule_3,
        updated_rule_4,
        updated_rule_5,
        updated_rule_6,
        updated_rule_7,
        updated_rule_8,
    ]

    # Batch get the updated rules
    rule_ids = [rule.rule_id for rule in all_updated_rules]
    batch_rules = client.rules.batch_get(rule_ids=rule_ids)
    print(f"Batch retrieved {len(batch_rules)} rules:")
    for rule in batch_rules:
        print(f"  - {rule.name}: {rule.expression}")

    # Test 10: Archive rules
    print("\n--- Test 10: Archive rules ---")
    client.rules.archive(rules=created_rules)

    print("\n=== Test Summary ===")
    print(f"Created: {len(created_rules)} rules")
    print(f"Updated: {len(all_updated_rules)} rules")

    # Verify all rules were processed
    assert len(created_rules) == num_rules, (
        f"Expected {num_rules} created rules, got {len(created_rules)}"
    )
    assert len(all_updated_rules) == num_rules, (
        f"Expected {num_rules} updated rules, got {len(all_updated_rules)}"
    )

    # Additional validation
    print("\n=== Validation Checks ===")

    # Verify that updates actually changed the values
    assert updated_rule_1.expression == "$1 > 0.5", (
        f"Expression update failed: {updated_rule_1.expression}"
    )
    # For update 1, also verify that the fields that were not updated are not reset.
    assert updated_rule_1_model_dump["description"] == rule_1_model_dump["description"], (
        f"Expected no description change, got {rule_1_model_dump['description']} -> {updated_rule_1.description}"
    )
    assert (
        updated_rule_1_model_dump["channel_references"] == rule_1_model_dump["channel_references"]
    ), (
        f"Expected no channel references change, got {rule_1_model_dump['channel_references']} -> {updated_rule_1.channel_references}"
    )
    assert updated_rule_1_model_dump["asset_ids"] == rule_1_model_dump["asset_ids"], (
        f"Expected no asset IDs change, got {rule_1_model_dump['asset_ids']} -> {updated_rule_1.asset_ids}"
    )
    assert updated_rule_1_model_dump["asset_tag_ids"] == rule_1_model_dump["asset_tag_ids"], (
        f"Expected no tag IDs change, got {rule_1_model_dump['asset_tag_ids']} -> {updated_rule_1.asset_tag_ids}"
    )
    assert (
        updated_rule_1_model_dump["contextual_channels"] == rule_1_model_dump["contextual_channels"]
    ), f"Contextual channels update failed: {updated_rule_1.contextual_channels}"
    assert "more details" in updated_rule_2.description, (
        f"Description update failed: {updated_rule_2.description}"
    )
    assert updated_rule_3.action.annotation_type == RuleAnnotationType.PHASE, (
        f"Action update failed: {updated_rule_3.action.annotation_type}"
    )
    assert updated_rule_4.name == new_name, f"Name update failed: {updated_rule_4.name}"

    assert updated_rule_6.expression == "$1 > 0.3 && $1 < 0.8", (
        f"Complex expression update failed: {updated_rule_6.expression}"
    )
    # assert updated_rule_7.action.action_type == RuleActionType.NOTIFICATION, f"Action type update failed: {updated_rule_7.action.action_type}"
    # assert len(updated_rule_8.tag_ids) == 2, f"Tag IDs update failed: {updated_rule_8.tag_ids}"
    assert len(updated_rule_8.contextual_channels) == 2, (
        f"Contextual channels update failed: {updated_rule_8.contextual_channels}"
    )

    print("All validation checks passed!")
    print("\n=== Test completed successfully ===")


if __name__ == "__main__":
    main()
