use super::IngestionConfigForm;
use crate::stream::flow::{add_new_flows, validate_flows};
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    assets::v1::Asset,
    ingestion_configs::v2::{FlowConfig, IngestionConfig},
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
) -> Result<(IngestionConfig, Vec<FlowConfig>, Asset)> {
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

    let ingestion_config_create_res = {
        let asset_name = asset_name.clone();
        let client_key = client_key.clone();
        let flows = flows.clone();
        retrying_ingestion_config
            .call(|mut w| {
                let asset_name = asset_name.clone();
                let client_key = client_key.clone();
                let flows = flows.clone();
                async move {
                    w.try_create_ingestion_config(&asset_name, &client_key, &flows)
                        .await
                }
            })
            .await
    };

    let (ingestion_config, new_config) = match ingestion_config_create_res {
        Ok(ingestion_config) => (ingestion_config, true),
        Err(e) if e.kind() == ErrorKind::AlreadyExistsError => {
            let client_key = client_key.clone();
            let ingestion_config = retrying_ingestion_config
                .call(|mut w| {
                    let client_key = client_key.clone();
                    async move { w.try_get_ingestion_config_by_client_key(&client_key).await }
                })
                .await?;

            #[cfg(feature = "tracing")]
            tracing::info!(
                ingestion_config_id = ingestion_config.ingestion_config_id,
                "an existing ingestion config was found with the provided client-key"
            );
            (ingestion_config, false)
        }
        Err(e) => return Err(e),
    };

    // Get the asset corresponding to the ingestion config.
    let asset_service_wrapper = new_asset_service(grpc_channel.clone());
    let retrying_asset = asset_service_wrapper.retrying(RetryConfig::default());
    let asset = {
        let asset_id = ingestion_config.asset_id.clone();
        retrying_asset
            .call(|mut w| {
                let asset_id = asset_id.clone();
                async move { w.try_get_asset_by_id(&asset_id).await }
            })
            .await
            .context("failed to retrieve asset specified by ingestion config")?
    };

    // Sanity check the Sift asset matches the expected asset-name.
    if asset.name != asset_name {
        return Err(Error::new_msg(
            ErrorKind::IncompatibleIngestionConfigChange,
            format!(
                "local ingestion config references asset '{asset_name}' but this existing config in Sift refers to asset '{}'",
                asset.name
            ),
        ));
    }

    // If the ingestion config was created new, the create call provided all the known ingestion
    // config flows, so no further API calls are required.
    if new_config {
        #[cfg(feature = "tracing")]
        {
            let flow_names = flows
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
        return Ok((ingestion_config, flows, asset));
    }

    // Fetch existing flow configs that match the names of the provided flow configs
    // so we can validate what we are trying to use vs. what Sift expects.
    let flow_names = flows
        .iter()
        .map(|f| format!("'{}'", f.name))
        .collect::<Vec<String>>()
        .join(",");

    let filter = flow_names
        .is_empty()
        .then(String::new)
        .unwrap_or_else(|| format!("flow_name in [{flow_names}]"));

    let existing_flows = {
        let ingestion_config_id = ingestion_config.ingestion_config_id.clone();
        let filter = filter.clone();
        retrying_ingestion_config
            .call(|mut w| {
                let ingestion_config_id = ingestion_config_id.clone();
                let filter = filter.clone();
                async move { w.try_filter_flows(&ingestion_config_id, &filter).await }
            })
            .await?
    };

    // If no flows were provided, use the existing flows in Sift to populate the local flow cache.
    if flows.is_empty() {
        #[cfg(feature = "tracing")]
        tracing::info!(
            ingestion_config_id = ingestion_config.ingestion_config_id,
            "no flows provided, using existing flows in Sift to populate the local flow cache"
        );
        return Ok((ingestion_config, existing_flows, asset));
    }

    // Find the local flows that Sift does not yet know about so they can be created.
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

        // If the flow isn't known to Sift yet, it will need to be created.
        if !flow_exists {
            flows_to_create.push(flow.clone());
        }
    }

    if !flows_to_create.is_empty() {
        // Capture names before moving flows_to_create into add_new_flows.
        #[cfg(feature = "tracing")]
        let flow_names: Vec<String> = flows_to_create.iter().map(|f| f.name.clone()).collect();

        let results = add_new_flows(
            grpc_channel.clone(),
            &ingestion_config.ingestion_config_id,
            flows_to_create,
        )
        .await;

        let mut flow_create_error = false;
        for (idx, result) in results.into_iter().enumerate() {
            match result {
                Ok(Ok(())) => {}
                // A concurrent SiftStream instance may have won the race to create this flow.
                Ok(Err(e)) if e.kind() == ErrorKind::AlreadyExistsError => {}
                Ok(Err(e)) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("failed to create flow {}: {e}", flow_names[idx]);
                    flow_create_error = true;
                }
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("failed to create flow {}: {e}", flow_names[idx]);
                    flow_create_error = true;
                }
            }
        }

        if flow_create_error {
            return Err(Error::new_msg(
                ErrorKind::CreateFlowError,
                "Ingestion config flow creation failed. See logs for details.",
            ));
        }

        #[cfg(feature = "tracing")]
        tracing::info!(
            ingestion_config_id = ingestion_config.ingestion_config_id,
            new_flows = flow_names.join(","),
            "created new flows for ingestion config"
        );

        // Fetch all flows Sift knows about with the specified names for validation.
        let sift_flows = {
            let ingestion_config_id = ingestion_config.ingestion_config_id.clone();
            let filter = filter.clone();
            retrying_ingestion_config
                .call(|mut w| {
                    let ingestion_config_id = ingestion_config_id.clone();
                    let filter = filter.clone();
                    async move { w.try_filter_flows(&ingestion_config_id, &filter).await }
                })
                .await?
        };

        validate_flows(&flows, &sift_flows)?;

        Ok((ingestion_config, sift_flows, asset))
    } else {
        Ok((ingestion_config, existing_flows, asset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{
        FlowCreateError, MockIngestionConfigService,
        create_mock_grpc_channel_with_ingestion_service, create_mock_grpc_channel_with_service,
    };
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
        }];
        let form = create_test_ingestion_config_form(asset_name, client_key, existing_flows);

        let (ingestion_config, returned_flows, _) =
            load_ingestion_config(grpc_channel, form).await.unwrap();

        assert_eq!(ingestion_config.client_key, client_key);
        // Should return existing flows
        assert!(!returned_flows.is_empty());
    }

    // Covers the Err(e) => return Err(e) branch at the ingestion-config create match.
    // An empty asset name triggers ArgumentValidationError from try_create_ingestion_config,
    // which is not AlreadyExistsError and must propagate rather than be swallowed.
    #[tokio::test]
    async fn test_load_ingestion_config_propagates_non_already_exists_create_error() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let form = create_test_ingestion_config_form("", "some_client_key", vec![]);

        let result = load_ingestion_config(grpc_channel, form).await;

        assert!(result.is_err());
        assert_ne!(result.unwrap_err().kind(), ErrorKind::AlreadyExistsError);
    }

    // Covers the flow_create_error path where add_new_flows returns a non-AlreadyExists
    // error, which should surface as CreateFlowError.
    #[tokio::test]
    async fn test_load_ingestion_config_flow_creation_failure_returns_error() {
        let service = MockIngestionConfigService::with_flow_create_error(FlowCreateError::Internal);
        let (grpc_channel, _) = create_mock_grpc_channel_with_ingestion_service(service).await;

        // brand_new_flow is not in the mock's existing flows, so it is queued for creation.
        let flows = vec![FlowConfig {
            name: "brand_new_flow".to_string(),
            channels: vec![ChannelConfig {
                name: "ch".to_string(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            }],
        }];
        let form = create_test_ingestion_config_form(
            "already_exists_asset",
            "already_exists_client_key",
            flows,
        );

        let result = load_ingestion_config(grpc_channel, form).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::CreateFlowError);
    }

    // Covers the AlreadyExistsError arm inside add_new_flows processing (the race
    // condition where a concurrent SiftStream instance created the flow first).
    // The function must succeed and return the flows created by the winning instance.
    #[tokio::test]
    async fn test_load_ingestion_config_race_condition_on_flow_creation_succeeds() {
        let race_flow = FlowConfig {
            name: "race_flow".to_string(),
            channels: vec![ChannelConfig {
                name: "ch".to_string(),
                data_type: ChannelDataType::Double.into(),
                ..Default::default()
            }],
        };
        let service = MockIngestionConfigService::with_race_condition(vec![race_flow.clone()]);
        let (grpc_channel, _) = create_mock_grpc_channel_with_ingestion_service(service).await;

        let form = create_test_ingestion_config_form(
            "already_exists_asset",
            "already_exists_client_key",
            vec![race_flow],
        );

        let (ingestion_config, returned_flows, asset) =
            load_ingestion_config(grpc_channel, form).await.unwrap();

        assert_eq!(ingestion_config.client_key, "already_exists_client_key");
        assert_eq!(asset.name, "already_exists_asset");
        assert_eq!(returned_flows.len(), 1);
        assert_eq!(returned_flows[0].name, "race_flow");
    }
}
