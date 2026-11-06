use super::IngestionConfigForm;
use crate::stream::{
    flow::{add_new_flows, find_equivalent_flow, validate_flows},
    retry::control_plane_retry,
};
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    assets::v1::Asset,
    ingestion_configs::v2::{FlowConfig, IngestionConfig},
    retry::RateLimitPolicy,
    wrappers::{
        ServiceOptions,
        assets::{AssetServiceWrapper, new_asset_service},
        ingestion_configs::{
            IngestionConfigServiceWrapper, new_ingestion_config_service_with_options,
        },
    },
};

/// A flow to register in the local flow cache.
#[derive(Debug)]
pub struct LoadedFlow {
    /// The flow name callers use in [`Flow::flow_name`](crate::Flow::flow_name).
    pub key: String,
    /// The flow's channels as the caller declared them, under the flow name Sift stores. The flow
    /// name is sent on ingest requests; the channel names are what callers set values with.
    pub config: FlowConfig,
}

impl LoadedFlow {
    fn with_sift_name(local: FlowConfig, sift_name: String) -> Self {
        Self {
            key: local.name,
            config: FlowConfig {
                name: sift_name,
                channels: local.channels,
            },
        }
    }
}

impl From<FlowConfig> for LoadedFlow {
    fn from(config: FlowConfig) -> Self {
        Self {
            key: config.name.clone(),
            config,
        }
    }
}

/// Loads or creates an ingestion config and returns the config, flows, and asset.
/// This is shared logic used by both IngestionConfigMode and FileBackupMode builders.
pub async fn load_ingestion_config(
    grpc_channel: SiftChannel,
    ingestion_config: IngestionConfigForm,
    service_options: ServiceOptions,
    rate_limit_policy: RateLimitPolicy,
) -> Result<(IngestionConfig, Vec<LoadedFlow>, Asset)> {
    #[cfg(feature = "tracing")]
    tracing::info_span!("load_ingestion_config");

    let IngestionConfigForm {
        asset_name,
        client_key,
        flows,
    } = ingestion_config;

    let ingestion_config_service_wrapper =
        new_ingestion_config_service_with_options(grpc_channel.clone(), service_options);
    let retrying_ingestion_config =
        control_plane_retry(ingestion_config_service_wrapper, rate_limit_policy);

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
    let retrying_asset = control_plane_retry(asset_service_wrapper, rate_limit_policy);
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
    if !asset.name.eq_ignore_ascii_case(&asset_name) {
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
        let loaded = flows.into_iter().map(LoadedFlow::from).collect();
        return Ok((ingestion_config, loaded, asset));
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

    let fetch_flows = |filter: String| {
        let ingestion_config_id = ingestion_config.ingestion_config_id.clone();
        let retrying_ingestion_config = &retrying_ingestion_config;
        async move {
            retrying_ingestion_config
                .call(|mut w| {
                    let ingestion_config_id = ingestion_config_id.clone();
                    let filter = filter.clone();
                    async move { w.try_filter_flows(&ingestion_config_id, &filter).await }
                })
                .await
        }
    };

    let existing_flows = fetch_flows(filter.clone()).await?;

    // If no flows were provided, use the existing flows in Sift to populate the local flow cache.
    if flows.is_empty() {
        #[cfg(feature = "tracing")]
        tracing::info!(
            ingestion_config_id = ingestion_config.ingestion_config_id,
            "no flows provided, using existing flows in Sift to populate the local flow cache"
        );
        let loaded = existing_flows.into_iter().map(LoadedFlow::from).collect();
        return Ok((ingestion_config, loaded, asset));
    }

    // Find the local flows that Sift does not yet know about so they can be created.
    let flows_to_create: Vec<FlowConfig> = flows
        .iter()
        .filter(|f| find_equivalent_flow(f, &existing_flows).is_none())
        .cloned()
        .collect();

    let sift_flows = if flows_to_create.is_empty() {
        existing_flows
    } else {
        // Capture names before moving flows_to_create into add_new_flows.
        #[cfg(feature = "tracing")]
        let flow_names: Vec<String> = flows_to_create.iter().map(|f| f.name.clone()).collect();

        let results = add_new_flows(
            grpc_channel.clone(),
            &ingestion_config.ingestion_config_id,
            flows_to_create,
            rate_limit_policy,
        )
        .await;

        let mut flow_create_error = false;
        let mut rate_limit_error = None;
        for (idx, result) in results.into_iter().enumerate() {
            match result {
                Ok(Ok(())) => {}
                // A concurrent SiftStream instance may have won the race to create this flow.
                Ok(Err(e)) if e.kind() == ErrorKind::AlreadyExistsError => {}
                Ok(Err(e)) if e.is_rate_limited() => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("failed to create flow {}: {e}", flow_names[idx]);
                    rate_limit_error.get_or_insert(e);
                }
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

        // Surface the rate limit, with its retry delay, only when it is the sole cause of
        // failure; otherwise retrying after the delay would not be enough to succeed.
        if let Some(e) = rate_limit_error
            && !flow_create_error
        {
            return Err(e).context("ingestion config flow creation was rate limited");
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
        let mut sift_flows = fetch_flows(filter).await?;

        // A local flow whose name differs from an existing flow only by case already exists in
        // Sift, but the name-filtered fetch may not return it. Fetch every flow to find it. This
        // only happens when a name is still missing after creation.
        if flows
            .iter()
            .any(|f| !sift_flows.iter().any(|sf| sf.name == f.name))
        {
            sift_flows = fetch_flows(String::new()).await?;
        }

        validate_flows(&flows, &sift_flows)?;
        sift_flows
    };

    // Every local flow now has an equivalent flow in Sift. Cache the local flows so the channel
    // names callers send match the flow descriptors, under the flow names Sift stores.
    let loaded = flows
        .into_iter()
        .map(|local| {
            let sift_name = find_equivalent_flow(&local, &sift_flows)
                .map(|f| f.name.clone())
                .unwrap_or_else(|| local.name.clone());
            LoadedFlow::with_sift_name(local, sift_name)
        })
        .collect();

    Ok((ingestion_config, loaded, asset))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{
        MockIngestionConfigService, create_mock_grpc_channel_with_ingestion_service,
        create_mock_grpc_channel_with_service, rate_limited_status,
    };
    use sift_rs::common::r#type::v1::{ChannelBitFieldElement, ChannelDataType, ChannelEnumType};
    use sift_rs::ingestion_configs::v2::ChannelConfig;
    use std::sync::atomic::Ordering;
    use std::time::{Duration, Instant};
    use tonic::Status;
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

        let (ingestion_config, returned_flows, asset) = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await
        .unwrap();

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

        let (ingestion_config, returned_flows, asset) = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await
        .unwrap();

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

        let (ingestion_config, returned_flows, _) = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await
        .unwrap();

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

        let (ingestion_config, returned_flows, _) = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await
        .unwrap();

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

        let result = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await;

        // Should fail with IncompatibleIngestionConfigChange error
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::IncompatibleIngestionConfigChange);
    }

    #[tokio::test]
    async fn test_load_ingestion_config_asset_name_differs_only_by_case() {
        let (grpc_channel, _) = create_mock_grpc_channel_with_service().await;
        let client_key = "already_exists_client_key";
        // Same asset as the mock service, differing only in capitalization.
        let asset_name = "Already_Exists_Asset";

        let form = create_test_ingestion_config_form(asset_name, client_key, vec![]);

        let (ingestion_config, _, asset) = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await
        .unwrap();

        assert_eq!(ingestion_config.client_key, client_key);
        // Sift is the source of truth for the name, so the asset keeps its own casing.
        assert_eq!(asset.name, "already_exists_asset");
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

        let (ingestion_config, returned_flows, _) = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await
        .unwrap();

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

        let result = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await;

        assert!(result.is_err());
        assert_ne!(result.unwrap_err().kind(), ErrorKind::AlreadyExistsError);
    }

    // Covers the flow_create_error path where add_new_flows returns a non-AlreadyExists
    // error, which should surface as CreateFlowError.
    #[tokio::test]
    async fn test_load_ingestion_config_flow_creation_failure_returns_error() {
        let service = MockIngestionConfigService::with_flow_create_error(
            sift_error::Error::new_msg(sift_error::ErrorKind::IoError, "io error"),
        );
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

        let result = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await;

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

        let (ingestion_config, returned_flows, asset) = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await
        .unwrap();

        assert_eq!(ingestion_config.client_key, "already_exists_client_key");
        assert_eq!(asset.name, "already_exists_asset");
        assert_eq!(returned_flows.len(), 1);
        assert_eq!(returned_flows[0].key, "race_flow");
    }

    /// Loads the mock's existing ingestion config with `flow_names` as new flows to create.
    async fn load_with_new_flows(
        service: MockIngestionConfigService,
        flow_names: &[&str],
        rate_limit_policy: RateLimitPolicy,
    ) -> Result<(IngestionConfig, Vec<LoadedFlow>, Asset)> {
        let (grpc_channel, _) = create_mock_grpc_channel_with_ingestion_service(service).await;
        let flows = flow_names
            .iter()
            .map(|name| create_test_flow_config(name))
            .collect();
        let form = create_test_ingestion_config_form(
            "already_exists_asset",
            "already_exists_client_key",
            flows,
        );
        load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            rate_limit_policy,
        )
        .await
    }

    // The RetryInfo detail must survive the real gRPC transport and surface as
    // RateLimitedError with the server's delay instead of a generic CreateFlowError.
    #[tokio::test]
    async fn test_load_ingestion_config_rate_limited_flow_creation_returns_retry_delay() {
        let delay = Duration::from_millis(50);
        let service = MockIngestionConfigService::with_flow_create_statuses(vec![(
            "rate_limited_flow",
            rate_limited_status(delay),
            usize::MAX,
        )]);
        let calls = service.create_flows_call_count();

        let err = load_with_new_flows(
            service,
            &["rate_limited_flow"],
            RateLimitPolicy::ReturnError,
        )
        .await
        .unwrap_err();

        assert_eq!(
            err.kind(),
            ErrorKind::RateLimitedError {
                retry_delay: Some(delay)
            }
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_load_ingestion_config_rate_limited_flow_creation_is_retried() {
        let delay = Duration::from_millis(50);
        let service = MockIngestionConfigService::with_flow_create_statuses(vec![(
            "rate_limited_flow",
            rate_limited_status(delay),
            1,
        )]);
        let calls = service.create_flows_call_count();

        let start = Instant::now();
        let (_, flows, _) =
            load_with_new_flows(service, &["rate_limited_flow"], RateLimitPolicy::default())
                .await
                .unwrap();

        assert!(start.elapsed() >= delay);
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert_eq!(flows.len(), 1);
        assert_eq!(flows[0].key, "rate_limited_flow");
    }

    // A rate limit is only surfaced when it is the sole cause of failure; otherwise waiting
    // for the retry delay would not be enough to succeed.
    #[tokio::test]
    async fn test_load_ingestion_config_mixed_flow_failures_return_create_flow_error() {
        let service = MockIngestionConfigService::with_flow_create_statuses(vec![
            (
                "rate_limited_flow",
                rate_limited_status(Duration::from_millis(50)),
                usize::MAX,
            ),
            ("broken_flow", Status::internal("boom"), usize::MAX),
        ]);

        let err = load_with_new_flows(
            service,
            &["rate_limited_flow", "broken_flow"],
            RateLimitPolicy::ReturnError,
        )
        .await
        .unwrap_err();

        assert_eq!(err.kind(), ErrorKind::CreateFlowError);
    }

    // The server's message size limit returns ResourceExhausted without RetryInfo. It is not
    // a rate limit, so it is neither retried nor reported as RateLimitedError.
    #[tokio::test]
    async fn test_load_ingestion_config_resource_exhausted_without_retry_info_is_not_retried() {
        for policy in [RateLimitPolicy::retry(), RateLimitPolicy::ReturnError] {
            let service = MockIngestionConfigService::with_flow_create_statuses(vec![(
                "too_large_flow",
                Status::resource_exhausted(
                    "grpc: received message larger than max for this method",
                ),
                usize::MAX,
            )]);
            let calls = service.create_flows_call_count();

            let err = load_with_new_flows(service, &["too_large_flow"], policy)
                .await
                .unwrap_err();

            assert_eq!(err.kind(), ErrorKind::CreateFlowError, "policy {policy:?}");
            assert_eq!(calls.load(Ordering::SeqCst), 1, "policy {policy:?}");
        }
    }

    // Sift normalizes some variation in how a flow is declared. The tests below seed the mock with
    // a stored flow, then load the existing config with an equivalent declaration of that flow,
    // as a rebuild with an unchanged `IngestionConfigForm` would.

    struct ExistingConfigLoad {
        result: Result<(IngestionConfig, Vec<LoadedFlow>, Asset)>,
        create_calls: usize,
        list_filters: Vec<String>,
    }

    async fn load_existing(sift_flow: FlowConfig, local_flow: FlowConfig) -> ExistingConfigLoad {
        let service = MockIngestionConfigService::with_existing_flows(vec![sift_flow]);
        let create_calls = service.create_flows_call_count();
        let list_filters = service.list_flows_filters();
        let (grpc_channel, _) = create_mock_grpc_channel_with_ingestion_service(service).await;

        let form = create_test_ingestion_config_form(
            "already_exists_asset",
            "already_exists_client_key",
            vec![local_flow],
        );
        let result = load_ingestion_config(
            grpc_channel,
            form,
            ServiceOptions::default(),
            RateLimitPolicy::default(),
        )
        .await;

        ExistingConfigLoad {
            result,
            create_calls: create_calls.load(Ordering::SeqCst),
            list_filters: list_filters.lock().unwrap().clone(),
        }
    }

    fn flow(name: &str, channels: Vec<ChannelConfig>) -> FlowConfig {
        FlowConfig {
            name: name.into(),
            channels,
        }
    }

    fn double_channel(name: &str, unit: &str, description: &str) -> ChannelConfig {
        ChannelConfig {
            name: name.into(),
            unit: unit.into(),
            description: description.into(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }
    }

    fn bit_field_channel(indexes: &[i32]) -> ChannelConfig {
        ChannelConfig {
            name: "status".into(),
            data_type: ChannelDataType::BitField.into(),
            bit_field_elements: indexes
                .iter()
                .map(|&index| ChannelBitFieldElement {
                    name: format!("bits_{index}"),
                    index,
                    bit_count: 4,
                })
                .collect(),
            ..Default::default()
        }
    }

    fn enum_channel(keys: &[u32]) -> ChannelConfig {
        ChannelConfig {
            name: "mode".into(),
            data_type: ChannelDataType::Enum.into(),
            enum_types: keys
                .iter()
                .map(|&key| ChannelEnumType {
                    name: format!("value_{key}"),
                    key,
                    is_signed: false,
                })
                .collect(),
            ..Default::default()
        }
    }

    /// Asserts the load succeeded without creating flows, and returns the single loaded flow.
    fn assert_loaded_unchanged(load: ExistingConfigLoad) -> LoadedFlow {
        let (_, mut flows, _) = load.result.expect("load should succeed");
        assert_eq!(load.create_calls, 0, "no flow should be re-created");
        assert_eq!(flows.len(), 1);
        flows.remove(0)
    }

    #[tokio::test]
    async fn test_load_ingestion_config_accepts_bit_field_elements_sorted_by_sift() {
        let local = flow("flow", vec![bit_field_channel(&[4, 0])]);
        let sift = flow("flow", vec![bit_field_channel(&[0, 4])]);

        let loaded = assert_loaded_unchanged(load_existing(sift, local.clone()).await);
        assert_eq!(loaded.config, local);
    }

    #[tokio::test]
    async fn test_load_ingestion_config_accepts_reordered_enum_types() {
        let local = flow("flow", vec![enum_channel(&[2, 0, 1])]);
        let sift = flow("flow", vec![enum_channel(&[0, 1, 2])]);

        let loaded = assert_loaded_unchanged(load_existing(sift, local.clone()).await);
        assert_eq!(loaded.config, local);
    }

    #[tokio::test]
    async fn test_load_ingestion_config_ignores_channel_descriptions() {
        let local = flow("flow", vec![double_channel("pressure", "psi", "updated")]);
        let sift = flow("flow", vec![double_channel("pressure", "psi", "original")]);

        assert_loaded_unchanged(load_existing(sift, local).await);
    }

    #[tokio::test]
    async fn test_load_ingestion_config_keeps_caller_channel_name_casing() {
        let local = flow("flow", vec![double_channel("Pressure", "psi", "")]);
        let sift = flow("flow", vec![double_channel("pressure", "psi", "")]);

        let loaded = assert_loaded_unchanged(load_existing(sift, local).await);
        // Descriptors are keyed by the caller's channel names, which `Flow` values use.
        assert_eq!(loaded.config.channels[0].name, "Pressure");
    }

    #[tokio::test]
    async fn test_load_ingestion_config_matches_flow_name_case_insensitively() {
        let local = flow("Col_Group=0", vec![double_channel("pressure", "psi", "")]);
        let sift = flow("col_group=0", vec![double_channel("pressure", "psi", "")]);

        let load = load_existing(sift, local).await;
        let list_filters = load.list_filters.clone();
        // The filtered fetch misses the flow, so it is created once and Sift reports it exists.
        let create_calls = load.create_calls;
        let (_, flows, _) = load.result.expect("load should succeed");

        assert_eq!(create_calls, 1);
        assert_eq!(list_filters.last().map(String::as_str), Some(""));
        assert_eq!(flows.len(), 1);
        assert_eq!(
            flows[0].key, "Col_Group=0",
            "callers look flows up by their own name"
        );
        assert_eq!(
            flows[0].config.name, "col_group=0",
            "requests use Sift's flow name"
        );
    }

    #[tokio::test]
    async fn test_load_ingestion_config_skips_unfiltered_fetch_when_flows_match() {
        let local = flow("flow", vec![double_channel("pressure", "psi", "")]);

        let load = load_existing(local.clone(), local).await;
        let list_filters = load.list_filters.clone();
        assert_loaded_unchanged(load);
        assert_eq!(list_filters, vec!["flow_name in ['flow']".to_string()]);
    }

    /// Asserts the load failed as an incompatible change whose message names `difference`.
    fn assert_incompatible(load: ExistingConfigLoad, difference: &str) {
        let err = load.result.expect_err("load should fail");
        assert_eq!(err.kind(), ErrorKind::IncompatibleIngestionConfigChange);
        assert!(
            err.to_string().contains(difference),
            "expected '{difference}' in:\n{err}"
        );
    }

    #[tokio::test]
    async fn test_load_ingestion_config_rejects_changed_unit() {
        let local = flow("flow", vec![double_channel("pressure", "kPa", "")]);
        let sift = flow("flow", vec![double_channel("pressure", "psi", "")]);

        assert_incompatible(
            load_existing(sift, local).await,
            "channel 0 ('pressure'): unit is 'kPa' locally but 'psi' in Sift",
        );
    }

    #[tokio::test]
    async fn test_load_ingestion_config_rejects_added_channel() {
        let local = flow(
            "flow",
            vec![
                double_channel("pressure", "psi", ""),
                double_channel("temperature", "degC", ""),
            ],
        );
        let sift = flow("flow", vec![double_channel("pressure", "psi", "")]);

        assert_incompatible(
            load_existing(sift, local).await,
            "flow has 2 channels locally but 1 in Sift",
        );
    }

    #[tokio::test]
    async fn test_load_ingestion_config_rejects_reordered_channels() {
        let pressure = double_channel("pressure", "psi", "");
        let temperature = double_channel("temperature", "degC", "");
        let local = flow("flow", vec![temperature.clone(), pressure.clone()]);
        let sift = flow("flow", vec![pressure, temperature]);

        assert_incompatible(
            load_existing(sift, local).await,
            "channel 0 ('temperature'): name is 'temperature' locally but 'pressure' in Sift",
        );
    }

    #[tokio::test]
    async fn test_load_ingestion_config_rejects_changed_enum_values() {
        let local = flow("flow", vec![enum_channel(&[0, 1, 3])]);
        let sift = flow("flow", vec![enum_channel(&[0, 1, 2])]);

        assert_incompatible(
            load_existing(sift, local).await,
            "channel 0 ('mode'): enum types differ",
        );
    }
}
