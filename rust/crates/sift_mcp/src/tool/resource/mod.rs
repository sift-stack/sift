use anyhow::Context;
use bytes::Bytes;
use rmcp::{
    ServerHandler, handler::server::wrapper::Parameters, model::CallToolResult, schemars::{self, JsonSchema}, tool, tool_handler, tool_router
};
use serde::Deserialize;
use sift_rs::{
    SiftChannel,
    assets::v1::{ListAssetsRequest, ListAssetsResponse, asset_service_client::AssetServiceClient}, runs::v2::{ListRunsRequest, ListRunsResponse, run_service_client::RunServiceClient},
};
use tonic::{body::Body, client::GrpcService};

use crate::{
    error::{self, BoxedStdErr, McpResult},
    tool::TransportBody,
};

#[derive(Clone)]
pub struct ResourceTool<T> {
    channel: T,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub enum Resource {
    Asset,
    Run
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetParams {
    resource: Resource,
    filter: String,
    limit: Option<u32>,
}

impl From<SiftChannel> for ResourceTool<SiftChannel> {
    fn from(channel: SiftChannel) -> Self {
        Self::new(channel)
    }
}

#[tool_handler(
    name = "ResourceTool",
    version = "0.1.0",
    instructions = "Tool to retrieve information about particular resources in Sift",
)]
impl<T> ServerHandler for ResourceTool<T>
where
    T: GrpcService<Body> + Clone +  Send + Sync + 'static,
    <T as GrpcService<tonic::body::Body>>::Future: Send,
    T::Error: Into<BoxedStdErr>,
    T::ResponseBody: TransportBody<Data = Bytes> + Send + 'static,
    <T::ResponseBody as TransportBody>::Error: Into<BoxedStdErr> + Send,
{}

#[tool_router]
impl<T> ResourceTool<T>
where
    T: GrpcService<Body> + Clone +  Send + Sync + 'static,
    <T as GrpcService<tonic::body::Body>>::Future: Send,
    T::Error: Into<BoxedStdErr>,
    T::ResponseBody: TransportBody<Data = Bytes> + Send + 'static,
    <T::ResponseBody as TransportBody>::Error: Into<BoxedStdErr> + Send,
{
    pub fn new(channel: T) -> Self {
        Self { channel }
    }

    #[tool(
        name = "get",
        description = "Retrieve and filter Sift resources information.",
        annotations(
            title = "ResourceTool/get",
            read_only_hint = true,
        ),
    )]
    pub async fn get(&self, params: Parameters<GetParams>) -> McpResult {
        const PAGE_SIZE: u32 = 1000;

        let Parameters(GetParams {
            resource,
            filter,
            limit
        }) = params;

        let (page_size, record_limit) = {
            if let Some(lim) = limit && lim <= PAGE_SIZE {
                (lim, lim as usize)
            } else {
                // Query everything
                (PAGE_SIZE, usize::MAX)
            }
        };

        match resource {
            Resource::Asset => {
                let mut client = AssetServiceClient::new(self.channel.clone());
                let mut page_token = String::new();
                let mut query_result = Vec::new();

                loop {
                    let resp = client
                        .list_assets(ListAssetsRequest {
                            filter: filter.clone(),
                            page_size,
                            page_token,
                            ..Default::default()
                        })
                        .await
                        .map_err(error::from_grpc_status)?;

                    let ListAssetsResponse {
                        assets,
                        next_page_token,
                    } = resp.into_inner();

                    if query_result.is_empty() {
                        break;
                    }
                    query_result.extend_from_slice(&assets);

                    if next_page_token.is_empty() {
                        break;
                    }
                    page_token = next_page_token;

                    if assets.len() >= record_limit {
                        break;
                    }
                }

                let out = serde_json::to_value(&query_result)
                    .context("failed to serialize assets")
                    .map_err(error::from_anyhow)?;

                Ok(CallToolResult::structured(out))
            }
            Resource::Run => {
                let mut client = RunServiceClient::new(self.channel.clone());
                let mut page_token = String::new();
                let mut query_result = Vec::new();

                loop {
                    let resp = client
                        .list_runs(ListRunsRequest {
                            filter: filter.clone(),
                            page_size,
                            page_token,
                            ..Default::default()
                        })
                        .await
                        .map_err(error::from_grpc_status)?;

                    let ListRunsResponse {
                        runs,
                        next_page_token,
                    } = resp.into_inner();

                    if query_result.is_empty() {
                        break;
                    }
                    query_result.extend_from_slice(&runs);

                    if next_page_token.is_empty() {
                        break;
                    }
                    page_token = next_page_token;

                    if runs.len() >= record_limit {
                        break;
                    }
                }

                let out = serde_json::to_value(&query_result)
                    .context("failed to serialize runs")
                    .map_err(error::from_anyhow)?;

                Ok(CallToolResult::structured(out))
            },
        }
    }
}
