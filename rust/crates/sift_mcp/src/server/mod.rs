use rmcp::{
    RoleServer, ServerHandler,
    handler::server::router::prompt::PromptRouter,
    handler::server::tool::ToolRouter,
    model::{GetPromptRequestParams, GetPromptResult, ListPromptsResult, PaginatedRequestParams},
    prompt_handler,
    service::RequestContext,
    tool_handler,
};
use sift_rs::SiftChannel;

use crate::service::{
    assets::AssetService, channels::ChannelService, data::DataService, ingest::IngestService,
    runs::RunService,
};

#[derive(Clone)]
pub struct SiftMcpServer {
    pub tool_router: ToolRouter<Self>,
    pub prompt_router: PromptRouter<Self>,

    pub asset_service: AssetService,
    pub channel_service: ChannelService,
    pub data_service: DataService,
    pub ingest_service: IngestService,
    pub run_service: RunService,
}

#[tool_handler(
    router = self.tool_router,
    name = "SiftMcp",
    version = "0.1.0",
    instructions = "Sift MCP Server",
)]
#[prompt_handler(router = self.prompt_router)]
impl ServerHandler for SiftMcpServer {}

impl SiftMcpServer {
    pub fn new(channel: SiftChannel) -> Self {
        // Add more routers here as new tool groups are introduced, e.g.
        //   tool_router.merge(Self::ingestion_router())
        let mut tool_router = Self::list_router();
        tool_router.merge(Self::data_router());

        let prompt_router = Self::prompt_router();

        let asset_service = AssetService::new(channel.clone());
        let data_service = DataService::new(channel.clone());
        let channel_service = ChannelService::new(channel.clone());
        let ingest_service = IngestService::new(channel.clone());
        let run_service = RunService::new(channel.clone());

        Self {
            asset_service,
            channel_service,
            data_service,
            ingest_service,
            run_service,
            tool_router,
            prompt_router,
        }
    }
}
