use super::IngestionConfigForm;
use crate::stream::flow::validate_flows;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    assets::v1::Asset,
    ingestion_configs::v2::{FlowConfig, IngestionConfig as IngestionConfigPb},
    retry::{RetryConfig, RetryExt},
    wrappers::{
        assets::{AssetServiceWrapper, new_asset_service},
        ingestion_configs::{IngestionConfigServiceWrapper, new_ingestion_config_service},
    },
};

/// Loads or creates an ingestion config and returns the config, flows, and asset.
/// This is shared logic used by both IngestionConfigMode and FileBackupMode builders.
pub async fn load_ingestion_config(
    grpc_channel: SiftChannel,
    ingestion_config: IngestionConfigForm,
) -> Result<(IngestionConfigPb, Vec<FlowConfig>, Asset)> {
    #[cfg(feature = "tracing")]
    tracing::info_span!("load_ingestion_config");

    let IngestionConfigForm {
        asset_name,
        client_key,
        flows,
    } = ingestion_config;

    let ingestion_config_service_wrapper = new_ingestion_config_service(grpc_channel.clone());
    let retrying_ingestion_config =
        ingestion_config_service_wrapper.retrying(RetryConfig::default());

    let client_key_clone = client_key.clone();
    match retrying_ingestion_config
        .call(|mut w| {
            let client_key = client_key_clone.clone();
            async move { w.try_get_ingestion_config_by_client_key(&client_key).await }
        })
        .await
    {
        Err(err) if err.kind() == ErrorKind::NotFoundError => {
            let asset_name_clone = asset_name.clone();
            let client_key_clone = client_key.clone();
            let flows_clone = flows.clone();
            let ingestion_config = retrying_ingestion_config
                .call(|mut w| {
                    let asset_name = asset_name_clone.clone();
                    let client_key = client_key_clone.clone();
                    let flows = flows_clone.clone();
                    async move {
                        w.try_create_ingestion_config(&asset_name, &client_key, &flows)
                            .await
                    }
                })
                .await?;

            let new_flows = {
                if flows.is_empty() {
                    Vec::new()
                } else {
                    let ingestion_config_id_clone = ingestion_config.ingestion_config_id.clone();
                    retrying_ingestion_config
                        .call(|mut w| {
                            let ingestion_config_id = ingestion_config_id_clone.clone();
                            async move { w.try_filter_flows(&ingestion_config_id, "").await }
                        })
                        .await?
                }
            };

            let asset_service_wrapper = new_asset_service(grpc_channel.clone());
            let retrying_asset = asset_service_wrapper.retrying(RetryConfig::default());
            let asset_id_clone = ingestion_config.asset_id.clone();
            let asset = retrying_asset
                .call(|mut w| {
                    let asset_id = asset_id_clone.clone();
                    async move { w.try_get_asset_by_id(&asset_id).await }
                })
                .await
                .context("failed to retrieve asset specified by ingestion config")?;

            #[cfg(feature = "tracing")]
            {
                if !new_flows.is_empty() {
                    let flow_names = new_flows
                        .iter()
                        .map(|f| f.name.as_str())
                        .collect::<Vec<&str>>()
                        .join(",");
                    tracing::info!(
                        ingestion_config_id = ingestion_config.ingestion_config_id,
                        flow_names = flow_names,
                        "created new ingestion config"
                    );
                }
            }
            Ok((ingestion_config, flows, asset))
        }
        Err(err) => Err(err),

        Ok(ingestion_config) => {
            #[cfg(feature = "tracing")]
            tracing::info!(
                ingestion_config_id = ingestion_config.ingestion_config_id,
                "an existing ingestion config was found with the provided client-key"
            );

            let asset_service_wrapper = new_asset_service(grpc_channel.clone());
            let retrying_asset = asset_service_wrapper.retrying(RetryConfig::default());
            let asset_id_clone2 = ingestion_config.asset_id.clone();
            let asset = retrying_asset
                .call(|mut w| {
                    let asset_id = asset_id_clone2.clone();
                    async move { w.try_get_asset_by_id(&asset_id).await }
                })
                .await
                .context("failed to retrieve asset specified by ingestion config")?;

            if asset.name != asset_name {
                return Err(Error::new_msg(
                    ErrorKind::IncompatibleIngestionConfigChange,
                    format!(
                        "local ingestion config references asset '{asset_name}' but this existing config in Sift refers to asset '{}'",
                        asset.name
                    ),
                ));
            }

            let flow_names = flows
                .iter()
                .map(|f| format!("'{}'", f.name))
                .collect::<Vec<String>>()
                .join(",");

            let filter = flow_names
                .is_empty()
                .then(String::new)
                .unwrap_or_else(|| format!("flow_name in [{flow_names}]"));

            let ingestion_config_id_clone = ingestion_config.ingestion_config_id.clone();
            let filter_clone = filter.clone();
            let existing_flows = retrying_ingestion_config
                .call(|mut w| {
                    let ingestion_config_id = ingestion_config_id_clone.clone();
                    let filter = filter_clone.clone();
                    async move { w.try_filter_flows(&ingestion_config_id, &filter).await }
                })
                .await?;

            // If no flows are provided, use the existing flows in Sift to populate the local flow cache.
            if flows.is_empty() {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    ingestion_config_id = ingestion_config.ingestion_config_id,
                    "no flows provided, using existing flows in Sift to populate the local flow cache"
                );
                return Ok((ingestion_config, existing_flows, asset));
            }

            let mut flows_to_create: Vec<FlowConfig> = Vec::new();

            for flow in &flows {
                let mut flow_exists = false;

                for existing_flow in existing_flows.iter().filter(|ef| ef == &flow) {
                    flow_exists = flow
                        .channels
                        .iter()
                        .zip(existing_flow.channels.iter())
                        .all(|(lhs, rhs)| lhs == rhs);

                    if flow_exists {
                        break;
                    }
                }

                if !flow_exists {
                    flows_to_create.push(flow.clone());
                }
            }

            if !flows_to_create.is_empty() {
                let flows_to_create_clone = flows_to_create.clone();
                let ingestion_config_id_clone = ingestion_config.ingestion_config_id.clone();
                let _ = retrying_ingestion_config
                    .call(|mut w| {
                        let ingestion_config_id = ingestion_config_id_clone.clone();
                        let flows = flows_to_create_clone.clone();
                        async move {
                            w.try_create_flows(&ingestion_config_id, flows.as_slice())
                                .await
                        }
                    })
                    .await;

                #[cfg(feature = "tracing")]
                {
                    let new_flow_names = flows_to_create
                        .iter()
                        .map(|f| f.name.as_str())
                        .collect::<Vec<&str>>()
                        .join(",");
                    tracing::info!(
                        ingestion_config_id = ingestion_config.ingestion_config_id,
                        new_flows = new_flow_names,
                        "created new flows for ingestion config"
                    );
                }

                // All the flows Sift sees with the specified names
                let ingestion_config_id_clone2 = ingestion_config.ingestion_config_id.clone();
                let filter_clone2 = filter.clone();
                let sift_flows = retrying_ingestion_config
                    .call(|mut w| {
                        let ingestion_config_id = ingestion_config_id_clone2.clone();
                        let filter = filter_clone2.clone();
                        async move { w.try_filter_flows(&ingestion_config_id, &filter).await }
                    })
                    .await?;

                validate_flows(&flows, &sift_flows)?;

                // Validation succeeded... used the flows we got for confidence in correctness.
                Ok((ingestion_config, sift_flows, asset))
            } else {
                Ok((ingestion_config, existing_flows, asset))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::create_mock_grpc_channel_with_service;
    use sift_rs::common::r#type::v1::ChannelDataType;
    use sift_rs::ingestion_configs::v2::ChannelConfig;
    use uuid::Uuid;

    fn create_test_flow_config(name: &str) -> FlowConfig {
        FlowConfig {
            name: name.to_string(),
            channels: vec![
                ChannelConfig {
                    name: "channel1".to_string(),
                    data_type: ChannelDataType::Double.into(),
                    ..Default::default()
                },
                ChannelConfig {
                    name: "channel2".to_string(),
                    data_type: ChannelDataType::Int32.into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    fn create_test_ingestion_config_form(
        asset_name: &str,
        client_key: &str,
        flows: Vec<FlowConfig>,
    ) -> IngestionConfigForm {
        IngestionConfigForm {
            asset_name: asset_name.to_string(),
            client_key: client_key.to_string(),
            flows,
        }
    }

    #[tokio::test]
    async fn test_load_ingestion_config_creates_new_config() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let asset_name = "test_asset";
        let client_key = format!("new_client_key_{}", Uuid::new_v4());
        let flows = vec![
            create_test_flow_config("test_flow_1"),
            create_test_flow_config("test_flow_2"),
        ];

        let form = create_test_ingestion_config_form(asset_name, &client_key, flows.clone());

        let (ingestion_config, returned_flows, asset) =
            load_ingestion_config(grpc_channel, form).await.unwrap();

        assert_eq!(ingestion_config.client_key, client_key);
        assert_eq!(asset.name, asset_name);
        assert_eq!(returned_flows.len(), flows.len());
    }

    #[tokio::test]
    async fn test_load_ingestion_config_creates_new_config_with_empty_flows() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let asset_name = "test_asset";
        let client_key = format!("new_client_key_{}", Uuid::new_v4());

        let form = create_test_ingestion_config_form(asset_name, &client_key, vec![]);

        let (ingestion_config, returned_flows, asset) =
            load_ingestion_config(grpc_channel, form).await.unwrap();

        assert_eq!(ingestion_config.client_key, client_key);
        assert_eq!(asset.name, asset_name);
        assert_eq!(returned_flows.len(), 0);
    }

    #[tokio::test]
    async fn test_load_ingestion_config_with_existing_config_and_empty_flows() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let client_key = "already_exists_client_key";
        let asset_name = "already_exists_asset";

        let form = create_test_ingestion_config_form(asset_name, client_key, vec![]);

        let (ingestion_config, returned_flows, _) =
            load_ingestion_config(grpc_channel, form).await.unwrap();

        assert_eq!(ingestion_config.client_key, client_key);
        // Should use existing flows from Sift when no flows provided
        assert!(!returned_flows.is_empty());
    }

    #[tokio::test]
    async fn test_load_ingestion_config_with_existing_config_and_new_flows() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let client_key = "already_exists_client_key";
        let asset_name = "already_exists_asset";
        let flows = vec![
            create_test_flow_config("new_flow_1"),
            create_test_flow_config("new_flow_2"),
        ];

        let form = create_test_ingestion_config_form(asset_name, client_key, flows.clone());

        let (ingestion_config, returned_flows, _) =
            load_ingestion_config(grpc_channel, form).await.unwrap();

        assert_eq!(ingestion_config.client_key, client_key);
        // Should return flows (either existing or newly created)
        assert!(!returned_flows.is_empty());
    }

    #[tokio::test]
    async fn test_load_ingestion_config_asset_name_mismatch() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let client_key = "already_exists_client_key";
        // Use a different asset name than what's in the mock service
        let asset_name = "different_asset_name";
        let flows = vec![];

        let form = create_test_ingestion_config_form(asset_name, client_key, flows);

        let result = load_ingestion_config(grpc_channel, form).await;

        // Should fail with IncompatibleIngestionConfigChange error
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::IncompatibleIngestionConfigChange);
    }

    #[tokio::test]
    async fn test_load_ingestion_config_handles_existing_flows() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let client_key = "already_exists_client_key";
        let asset_name = "already_exists_asset";

        // Use the same flow name as the default mock service
        let existing_flows = vec![FlowConfig {
            name: "already_exists_flow".to_string(),
            channels: vec![ChannelConfig {
                name: "channel1".to_string(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            }],
            ..Default::default()
        }];
        let form = create_test_ingestion_config_form(asset_name, client_key, existing_flows);

        let (ingestion_config, returned_flows, _) =
            load_ingestion_config(grpc_channel, form).await.unwrap();

        assert_eq!(ingestion_config.client_key, client_key);
        // Should return existing flows
        assert!(!returned_flows.is_empty());
    }
}
