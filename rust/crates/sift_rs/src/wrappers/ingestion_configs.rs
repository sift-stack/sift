use super::ResourceIdentifier;
use crate::ingestion_configs::v2::{
    CreateIngestionConfigFlowsRequest, CreateIngestionConfigRequest, FlowConfig,
    GetIngestionConfigRequest, IngestionConfig, ListIngestionConfigFlowsRequest,
    ListIngestionConfigFlowsResponse, ListIngestionConfigsRequest,
    ingestion_config_service_client::IngestionConfigServiceClient,
};
use async_trait::async_trait;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use std::ops::{Deref, DerefMut};

/// Creates a new ingestion config service wrapper.
///
/// Returns an implementation of [`IngestionConfigServiceWrapper`] which also exposes
/// methods from the raw [`IngestionConfigServiceClient`] via `Deref` and `DerefMut`.
///
/// # Arguments
///
/// * `grpc_channel` - The gRPC channel to use for communication
///
/// # Example
///
/// ```no_run
/// use sift_rs::wrappers::ingestion_configs::{new_ingestion_config_service, IngestionConfigServiceWrapper};
/// use sift_connect::{Credentials, SiftChannelBuilder};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let credentials = Credentials::Config {
///     uri: "https://api.siftstack.com".to_string(),
///     apikey: "your-api-key".to_string(),
/// };
/// let channel = SiftChannelBuilder::new(credentials).build()?;
/// let mut service = new_ingestion_config_service(channel);
///
/// let config = service.try_get_ingestion_config_by_client_key("my-config").await?;
/// # Ok(())
/// # }
/// ```
pub fn new_ingestion_config_service(
    grpc_channel: SiftChannel,
) -> impl IngestionConfigServiceWrapper {
    IngestionConfigServiceImpl(IngestionConfigServiceClient::new(grpc_channel))
}

/// Convenience methods for working with Sift's IngestionConfig service.
///
/// This trait provides simplified methods that return [`sift_error::Result`] instead
/// of raw gRPC responses. The underlying [`IngestionConfigServiceClient`] is accessible
/// via `Deref` and `DerefMut` for advanced use cases.
#[async_trait]
pub trait IngestionConfigServiceWrapper:
    Clone + Deref<Target = IngestionConfigServiceClient<SiftChannel>> + DerefMut
{
    /// Creates an ingestion config.
    ///
    /// # Arguments
    ///
    /// * `asset_name` - The name of the asset this config is for
    /// * `client_key` - A unique identifier for this ingestion config
    /// * `flows` - The flow configurations to include
    ///
    /// # Returns
    ///
    /// The created ingestion config, or an error if creation fails.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::ArgumentValidationError`] if `asset_name` or `client_key`
    /// is empty. Returns [`ErrorKind::CreateIngestionConfigError`] if creation fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_rs::wrappers::ingestion_configs::IngestionConfigServiceWrapper;
    /// use sift_rs::ingestion_configs::v2::FlowConfig;
    ///
    /// # async fn example(mut service: impl IngestionConfigServiceWrapper) -> Result<(), Box<dyn std::error::Error>> {
    /// let flows = vec![/* FlowConfig instances */];
    /// let config = service.try_create_ingestion_config("MyAsset", "config-v1", &flows).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn try_create_ingestion_config(
        &mut self,
        asset_name: &str,
        client_key: &str,
        flows: &[FlowConfig],
    ) -> Result<IngestionConfig>;

    /// Retrieves an ingestion config by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the ingestion config to retrieve
    ///
    /// # Returns
    ///
    /// The requested ingestion config, or an error if it doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::ArgumentValidationError`] if `id` is empty.
    /// Returns [`ErrorKind::RetrieveIngestionConfigError`] if retrieval fails.
    async fn try_get_ingestion_config_by_id(&mut self, id: &str) -> Result<IngestionConfig>;

    /// Retrieves an ingestion config by client key.
    ///
    /// # Arguments
    ///
    /// * `client_key` - The client key of the ingestion config to retrieve
    ///
    /// # Returns
    ///
    /// The requested ingestion config, or an error if it doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::ArgumentValidationError`] if `client_key` is empty.
    /// Returns [`ErrorKind::RetrieveIngestionConfigError`] if retrieval fails.
    /// Returns [`ErrorKind::NotFoundError`] if no config with the given client key exists.
    async fn try_get_ingestion_config_by_client_key(
        &mut self,
        client_key: &str,
    ) -> Result<IngestionConfig>;

    /// Creates flow configs for a given ingestion config.
    ///
    /// If this function does not return an error, then it is safe to assume that
    /// all [`FlowConfig`]s in `configs` were created.
    ///
    /// # Arguments
    ///
    /// * `ingestion_config_id` - The ID of the ingestion config to add flows to
    /// * `configs` - The flow configs to create (can be any type that converts to `Vec<FlowConfig>`)
    ///
    /// # Returns
    ///
    /// `Ok(())` if all flows were created successfully.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::AlreadyExistsError`] if a flow with the same name already exists.
    /// Returns [`ErrorKind::CreateFlowError`] if creation fails for other reasons.
    async fn try_create_flows<I>(&mut self, ingestion_config_id: &str, configs: I) -> Result<()>
    where
        I: Into<Vec<FlowConfig>> + Send;

    /// Retrieves all flows that satisfy the provided filter.
    ///
    /// This method handles pagination automatically and returns all matching flows.
    ///
    /// # Arguments
    ///
    /// * `ingestion_config_id` - The ID of the ingestion config to filter flows from
    /// * `filter` - A filter expression (e.g., `"name == 'my-flow'"`)
    ///
    /// # Returns
    ///
    /// A vector of all flows matching the filter, or an error if retrieval fails.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::RetrieveIngestionConfigError`] if retrieval fails.
    async fn try_filter_flows(
        &mut self,
        ingestion_config_id: &str,
        filter: &str,
    ) -> Result<Vec<FlowConfig>>;
}

/// A convience wrapper around [IngestionConfigServiceClient].
#[derive(Clone)]
struct IngestionConfigServiceImpl(IngestionConfigServiceClient<SiftChannel>);

#[async_trait]
impl IngestionConfigServiceWrapper for IngestionConfigServiceImpl {
    /// Create an ingestion config.
    async fn try_create_ingestion_config(
        &mut self,
        asset_name: &str,
        client_key: &str,
        flows: &[FlowConfig],
    ) -> Result<IngestionConfig> {
        let flows = flows.to_vec();

        if asset_name.is_empty() {
            return Err(Error::new_arg_error("asset name cannot be blank"));
        }
        if client_key.is_empty() {
            return Err(Error::new_arg_error(
                "ingestion config client key cannot be blank",
            ));
        }

        self.create_ingestion_config(CreateIngestionConfigRequest {
            asset_name: asset_name.to_string(),
            client_key: client_key.to_string(),
            flows,
            ..Default::default()
        })
        .await
        .map(|res| res.into_inner().ingestion_config)
        .map_err(|e| Error::new(ErrorKind::CreateIngestionConfigError, e))?
        .ok_or_else(|| {
            Error::new_empty_response(
                "unexpected empty response from IngestionConfigService/CreateIngestionConfig",
            )
        })
    }

    /// Retrieve ingestion config by ID.
    async fn try_get_ingestion_config_by_id(&mut self, id: &str) -> Result<IngestionConfig> {
        if id.is_empty() {
            return Err(Error::new_arg_error("ingestion config ID cannot be blank"));
        }
        self.try_get_ingestion_config(ResourceIdentifier::Id(id.to_string()))
            .await
    }

    /// Retrieve ingestion config by client key.
    async fn try_get_ingestion_config_by_client_key(
        &mut self,
        client_key: &str,
    ) -> Result<IngestionConfig> {
        if client_key.is_empty() {
            return Err(Error::new_msg(
                ErrorKind::ArgumentValidationError,
                "ingestion config client key cannot be blank",
            ));
        }
        self.try_get_ingestion_config(ResourceIdentifier::ClientKey(client_key.to_string()))
            .await
    }

    /// Create [FlowConfig]s for a given ingestion config. If this function does not return an
    /// error, then it is safe to assume that all [FlowConfig]s in `configs` was created.
    async fn try_create_flows<I>(&mut self, ingestion_config_id: &str, configs: I) -> Result<()>
    where
        I: Into<Vec<FlowConfig>> + Send,
    {
        let _ = self
            .create_ingestion_config_flows(CreateIngestionConfigFlowsRequest {
                ingestion_config_id: ingestion_config_id.to_string(),
                flows: configs.into(),
            })
            .await
            .map_err(|e| {
                if e.code() == tonic::Code::AlreadyExists {
                    Error::new(ErrorKind::AlreadyExistsError, e)
                } else {
                    Error::new(ErrorKind::CreateFlowError, e)
                }
            })?;
        Ok(())
    }

    /// Retrieve all flows that satisfy the provided filter.
    async fn try_filter_flows(
        &mut self,
        ingestion_config_id: &str,
        filter: &str,
    ) -> Result<Vec<FlowConfig>> {
        let mut request = ListIngestionConfigFlowsRequest {
            filter: filter.to_string(),
            ingestion_config_id: ingestion_config_id.to_string(),
            page_size: 1_000,
            page_token: String::new(),
        };

        let mut filtered_flows = Vec::new();

        loop {
            let ListIngestionConfigFlowsResponse {
                flows,
                next_page_token,
            } = self
                .list_ingestion_config_flows(request)
                .await
                .map(|res| res.into_inner())
                .map_err(|e| Error::new(ErrorKind::RetrieveIngestionConfigError, e))
                .context("something went wrong while filtering flows")?;

            if flows.is_empty() {
                break;
            }
            filtered_flows.extend_from_slice(&flows);

            if next_page_token.is_empty() {
                break;
            }
            request = ListIngestionConfigFlowsRequest {
                filter: filter.to_string(),
                ingestion_config_id: ingestion_config_id.to_string(),
                page_size: 1_000,
                page_token: next_page_token,
            };
        }
        Ok(filtered_flows)
    }
}

impl IngestionConfigServiceImpl {
    /// Retrieves an ingestion config by ID or client-key.
    async fn try_get_ingestion_config(
        &mut self,
        identifier: ResourceIdentifier,
    ) -> Result<IngestionConfig> {
        match identifier {
            ResourceIdentifier::Id(ingestion_config_id) => {
                self.get_ingestion_config(GetIngestionConfigRequest { ingestion_config_id })
                    .await
                    .map(|res| res.into_inner().ingestion_config)
                    .map_err(|e| Error::new(ErrorKind::RetrieveIngestionConfigError, e))
                    .context("failed to try_get ingestion config")
                    .help("make sure that the provided ingestion config ID is valid")?
                    .ok_or_else(|| Error::new_empty_response("unexpected empty response from IngestionConfigService/GetIngestionConfigRequest"))
            }

            ResourceIdentifier::ClientKey(client_key) => {
                let filter = format!("client_key == '{client_key}'");
                let conf = self
                    .list_ingestion_configs(ListIngestionConfigsRequest { filter, page_size: 1, ..Default::default() })
                    .await
                    .map(|res| res.into_inner().ingestion_configs)
                    .map_err(|e| Error::new(ErrorKind::RetrieveIngestionConfigError, e))
                    .context("failed try_get_ingestion_config")
                    .help("ensure that the provided client key is valid")?;

                conf.first()
                    .cloned()
                    .ok_or_else(|| Error::new_msg(ErrorKind::NotFoundError, "no ingestion config found with provided client key"))
            }
        }
    }
}

impl Deref for IngestionConfigServiceImpl {
    type Target = IngestionConfigServiceClient<SiftChannel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IngestionConfigServiceImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
