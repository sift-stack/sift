"""
Example demonstrating how to use the Rule types in sift_client.

This example shows how to create rule configurations and work with rule actions.
"""

from sift_client.types import (
    ExpressionChannelReference,
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreateNotification,
    RuleActionCreatePhaseAnnotation,
    RuleConfig,
)


def create_data_review_rule():
    """Create a rule that creates data review annotations."""

    # Define channel references
    channel_references = [
        ExpressionChannelReference(channel_reference="$1", channel_identifier="temperature"),
        ExpressionChannelReference(channel_reference="$2", channel_identifier="pressure"),
    ]

    # Create the action
    action = RuleActionCreateDataReviewAnnotation(
        assignee="engineer@company.com", tags=["high_temp", "alert"]
    )

    # Create the rule configuration
    rule_config = RuleConfig(
        name="High Temperature Alert",
        description="Alert when temperature exceeds threshold",
        expression="$1 > 100 && $2 < 50",
        action=action,
        channel_references=channel_references,
        rule_client_key="high_temp_alert_001",
        asset_names=["engine_001", "engine_002"],
        contextual_channels=["ambient_temp"],
        is_external=False,
    )

    return rule_config


def create_phase_annotation_rule():
    """Create a rule that creates phase annotations."""

    channel_references = [
        ExpressionChannelReference(channel_reference="$1", channel_identifier="speed")
    ]

    action = RuleActionCreatePhaseAnnotation(tags=["phase_change", "acceleration"])

    rule_config = RuleConfig(
        name="Speed Phase Change",
        description="Mark phase changes when speed changes significantly",
        expression="$1 > 0 && $1 < 10",
        action=action,
        channel_references=channel_references,
        rule_client_key="speed_phase_001",
        asset_names=["vehicle_001"],
        is_external=False,
    )

    return rule_config


def create_notification_rule():
    """Create a rule that sends notifications."""

    channel_references = [
        ExpressionChannelReference(channel_reference="$1", channel_identifier="battery_level")
    ]

    action = RuleActionCreateNotification(recipient_user_ids=["user_123", "user_456"])

    rule_config = RuleConfig(
        name="Low Battery Alert",
        description="Notify when battery level is low",
        expression="$1 < 20",
        action=action,
        channel_references=channel_references,
        rule_client_key="low_battery_001",
        asset_names=["device_001"],
        is_external=False,
    )

    return rule_config


def demonstrate_rule_actions():
    """Demonstrate different rule action types."""

    # Data review annotation
    data_review_action = RuleActionCreateDataReviewAnnotation(
        assignee="analyst@company.com", tags=["review", "manual_check"]
    )
    print(f"Data review action kind: {data_review_action.kind()}")
    print(f"Data review action assignee: {data_review_action.assignee}")

    # Phase annotation
    phase_action = RuleActionCreatePhaseAnnotation(tags=["phase", "automated"])
    print(f"Phase action kind: {phase_action.kind()}")
    print(f"Phase action tags: {phase_action.tags}")

    # Notification
    notification_action = RuleActionCreateNotification(recipient_user_ids=["admin@company.com"])
    print(f"Notification action kind: {notification_action.kind()}")
    print(f"Notification recipients: {notification_action.recipient_user_ids}")


def demonstrate_rule_config_json():
    """Demonstrate converting rule config to JSON."""

    rule_config = create_data_review_rule()
    json_data = rule_config.as_json()

    print("Rule config as JSON:")
    import json

    print(json.dumps(json_data, indent=2))


if __name__ == "__main__":
    print("=== Rule Types Example ===\n")

    print("1. Creating different types of rules:")
    data_review_rule = create_data_review_rule()
    phase_rule = create_phase_annotation_rule()
    notification_rule = create_notification_rule()

    print(f"   - Data review rule: {data_review_rule.name}")
    print(f"   - Phase rule: {phase_rule.name}")
    print(f"   - Notification rule: {notification_rule.name}\n")

    print("2. Demonstrating rule actions:")
    demonstrate_rule_actions()
    print()

    print("3. Converting rule config to JSON:")
    demonstrate_rule_config_json()
