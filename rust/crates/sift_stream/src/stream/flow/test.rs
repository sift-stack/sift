use super::validate_flows;
use sift_rs::ingestion_configs::v2::{ChannelConfig, FlowConfig};

#[test]
fn test_validate_flows_compatible_flows() {
    let user_flows = vec![
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![
                ChannelConfig {
                    name: String::from("foo"),
                    description: String::from("foo"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("bar"),
                    description: String::from("bar"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
    ];

    // Contains all the flows `user_flows` expects but with two `wheel` flows and different order.
    let sift_flows = vec![
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![
                ChannelConfig {
                    name: String::from("foo"),
                    description: String::from("foo"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("bar"),
                    description: String::from("bar"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![ChannelConfig {
                name: String::from("foo_v2"),
                description: String::from("foo"),
                ..Default::default()
            }],
        },
    ];

    assert!(
        validate_flows(&user_flows, &sift_flows).is_ok(),
        "validation should have succeeded"
    );
}

#[test]
fn test_validate_flows_incompatible_flows() {
    let user_flows = vec![
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![
                ChannelConfig {
                    name: String::from("foo"),
                    description: String::from("foo"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("bar"),
                    description: String::from("bar"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
    ];

    // 'wheel' flow in Sift is different
    let sift_flows = vec![
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![ChannelConfig {
                name: String::from("foo_v2"),
                description: String::from("foo"),
                ..Default::default()
            }],
        },
    ];

    assert!(
        validate_flows(&user_flows, &sift_flows).is_err(),
        "validation should have failed because 'wheel' flows differ."
    );
}

#[test]
fn test_validate_flows_missing_flows() {
    let user_flows = vec![
        FlowConfig {
            name: String::from("wheel"),
            channels: vec![
                ChannelConfig {
                    name: String::from("foo"),
                    description: String::from("foo"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("bar"),
                    description: String::from("bar"),
                    ..Default::default()
                },
            ],
        },
        FlowConfig {
            name: String::from("radiator"),
            channels: vec![
                ChannelConfig {
                    name: String::from("baz"),
                    description: String::from("baz"),
                    ..Default::default()
                },
                ChannelConfig {
                    name: String::from("barbaz"),
                    description: String::from("barbaz"),
                    ..Default::default()
                },
            ],
        },
    ];

    assert!(
        validate_flows(&user_flows, &Vec::new()).is_err(),
        "validation should fail because Sift is missing all specified flows",
    );
}
