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

    rule_configs = []
    
    # Test assignees to cycle through - this will trigger get_active_users calls
    test_assignees = [
        "adhi@siftstack.com",
        "jon@siftstack.com",
        "jon.deng@siftstack.com",
    ]
    
    # Generate 850 rules with variations
    for i in range(850):
        rule_num = i + 1
        
        # Cycle through assignees to trigger get_active_users for each rule
        assignee = test_assignees[i % len(test_assignees)]
        
        # Create different rule types based on modulo to vary the patterns
        rule_type = i % 4
        
        if rule_type == 0:
            # Overheating-style rules with varying thresholds
            threshold = 50 + (i % 100)  # Thresholds from 50 to 149
            state = ["Accelerating", "Decelerating", "Stopped"][i % 3]
            rule_configs.append(
                RuleConfig(
                    name=f"overheating_rule_{rule_num:03d}",
                    description=f"Checks for vehicle overheating (rule {rule_num})",
                    expression=f'$1 == "{state}" && $2 > {threshold}',
                    # Note: External rules don't use rule_client_key
                    asset_names=["NostromoLV426"],
                    channel_references=[
                        {
                            "channel_reference": "$1",
                            "channel_identifier": vehicle_state_channel.identifier,
                        },
                        {
                            "channel_reference": "$2",
                            "channel_config": voltage_channel,
                        },
                    ],
                    action=RuleActionCreateDataReviewAnnotation(
                        assignee=assignee,
                        tags=[f"rule_{rule_num}", "overheating"],
                    ),
                )
            )
        elif rule_type == 1:
            # Kinetic energy-style rules with varying parameters
            mass = 5 + (i % 20)  # Mass from 5 to 24
            threshold = 200 + (i % 500)  # Threshold from 200 to 699
            rule_configs.append(
                RuleConfig(
                    name=f"kinetic_energy_rule_{rule_num:03d}",
                    description=f"Tracks high energy output while in motion (rule {rule_num})",
                    expression="0.5 * $mass * $1 * $1 > $threshold",
                    # Note: External rules don't use rule_client_key
                    asset_names=["NostromoLV426"],
                    channel_references=[
                        {
                            "channel_reference": "$1",
                            "channel_config": voltage_channel,
                        },
                    ],
                    sub_expressions={
                        "$mass": mass,
                        "$threshold": threshold,
                    },
                    action=RuleActionCreateDataReviewAnnotation(
                        assignee=assignee,
                        tags=[f"rule_{rule_num}", "kinetic_energy"],
                    ),
                )
            )
        elif rule_type == 2:
            # Failure detection rules with varying search strings
            search_strings = ["failure", "error", "warning", "critical", "alert", "fault", "issue", "problem"]
            search_string = search_strings[i % len(search_strings)]
            rule_configs.append(
                RuleConfig(
                    name=f"failure_rule_{rule_num:03d}",
                    description=f"Checks for {search_string} reported by logs (rule {rule_num})",
                    expression="contains($1, $sub_string)",
                    # Note: External rules don't use rule_client_key
                    asset_names=["NostromoLV426"],
                    contextual_channels=[vehicle_state_channel.name],
                    channel_references=[
                        {
                            "channel_reference": "$1",
                            "channel_config": log_channel,
                        },
                    ],
                    sub_expressions={
                        "$sub_string": search_string,
                    },
                    action=RuleActionCreateDataReviewAnnotation(
                        assignee=assignee,
                        tags=[f"rule_{rule_num}", "failure", search_string],
                    ),
                )
            )
        else:  # rule_type == 3
            # Voltage threshold rules with varying comparisons
            threshold = 30 + (i % 120)  # Thresholds from 30 to 149
            comparison_ops = [">", ">=", "<", "<="]
            op = comparison_ops[i % len(comparison_ops)]
            rule_configs.append(
                RuleConfig(
                    name=f"voltage_rule_{rule_num:03d}",
                    description=f"Monitors voltage levels (rule {rule_num})",
                    expression=f"$1 {op} {threshold}",
                    # Note: External rules don't use rule_client_key
                    asset_names=["NostromoLV426"],
                    channel_references=[
                        {
                            "channel_reference": "$1",
                            "channel_config": voltage_channel,
                        },
                    ],
                    action=RuleActionCreateDataReviewAnnotation(
                        assignee=assignee,
                        tags=[f"rule_{rule_num}", "voltage"],
                    ),
                )
            )

    # Set these as external rules (required for batch API)
    for rule_config in rule_configs:
        rule_config.is_external = True

    return rule_configs

