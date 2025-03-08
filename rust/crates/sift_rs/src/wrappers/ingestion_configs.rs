use super::ResourceIdentifier;
use crate::ingestion_configs::v2::{
    ingestion_config_service_client::IngestionConfigServiceClient,
    CreateIngestionConfigFlowsRequest, CreateIngestionConfigRequest, FlowConfig,
    GetIngestionConfigRequest, IngestionConfig, ListIngestionConfigFlowsRequest,
    ListIngestionConfigFlowsResponse, ListIngestionConfigsRequest,
};
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use std::ops::{Deref, DerefMut};

/// A convience wrapper around [IngestionConfigServiceClient].
pub struct IngestionConfigServiceWrapper(IngestionConfigServiceClient<SiftChannel>);

impl IngestionConfigServiceWrapper {
    pub fn new(grpc_channel: SiftChannel) -> Self {
        Self(IngestionConfigServiceClient::new(grpc_channel))
    }

    /// Create an ingestion config.
    pub async fn try_create_ingestion_config<S: AsRef<str>>(
        &mut self,
        asset_name: S,
        client_key: S,
        flows: &[FlowConfig],
    ) -> Result<IngestionConfig> {
        let asset_name = asset_name.as_ref().to_string();
        let client_key = client_key.as_ref().to_string();
        let flows = flows.to_vec();

        if asset_name.is_empty() {
            return Err(Error::new_arg_error("asset name cannot be blank"));
        }
        if client_key.is_empty() {
            return Err(Error::new_arg_error(
                "ingestion config client key cannot be blank",
            ));
        }
        if flows.is_empty() {
            return Err(Error::new_arg_error(
                "cannot create an ingestion config with no flows",
            ));
        }

        self.create_ingestion_config(CreateIngestionConfigRequest {
            asset_name,
            client_key,
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
    pub async fn try_get_ingestion_config_by_id<S: AsRef<str>>(
        &mut self,
        id: S,
    ) -> Result<IngestionConfig> {
        let id = id.as_ref().to_string();

        if id.is_empty() {
            return Err(Error::new_arg_error("ingestion config ID cannot be blank"));
        }
        self.try_get_ingestion_config(ResourceIdentifier::Id(id))
            .await
    }

    /// Retrieve ingestion config by client key.
    pub async fn try_get_ingestion_config_by_client_key<S: AsRef<str>>(
        &mut self,
        client_key: S,
    ) -> Result<IngestionConfig> {
        let client_key = client_key.as_ref().to_string();

        if client_key.is_empty() {
            return Err(Error::new_msg(
                ErrorKind::ArgumentValidationError,
                "ingestion config client key cannot be blank",
            ));
        }
        self.try_get_ingestion_config(ResourceIdentifier::ClientKey(client_key))
            .await
    }

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
                let filter = format!("client_key = '{client_key}'");
                let conf = self
                    .list_ingestion_configs(ListIngestionConfigsRequest { filter, page_size: 1, ..Default::default() })
                    .await
                    .map(|res| res.into_inner().ingestion_configs)
                    .map_err(|e| Error::new(ErrorKind::RetrieveIngestionConfigError, e))
                    .context("failed to try_get ingestion config")
                    .help("ensure that the provided client key is valid")?;

                conf.first()
                    .cloned()
                    .ok_or_else(|| Error::new_msg(ErrorKind::NotFoundError, "no ingestion config found with provided client key"))
            }
        }
    }

    /// Create [FlowConfig]s for a given ingestion config. If this function does not return an
    /// error, then it is safe to assume that all [FlowConfig]s in `configs` was created.
    pub async fn try_create_flows(
        &mut self,
        ingestion_config_id: &str,
        configs: &[FlowConfig],
    ) -> Result<()> {
        let _ = self
            .create_ingestion_config_flows(CreateIngestionConfigFlowsRequest {
                ingestion_config_id: ingestion_config_id.to_string(),
                flows: configs.to_vec(),
            })
            .await
            .map_err(|e| Error::new(ErrorKind::CreateFlowError, e))?;
        Ok(())
    }

    /// Retrieve all flows that satisfy the provided filter.
    pub async fn try_filter_flows(
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

impl Deref for IngestionConfigServiceWrapper {
    type Target = IngestionConfigServiceClient<SiftChannel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IngestionConfigServiceWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
