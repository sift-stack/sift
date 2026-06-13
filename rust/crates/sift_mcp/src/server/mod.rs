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
    assets::AssetService, calculated_channels::CalculatedChannelService, channels::ChannelService,
    data::DataService, explore::ExploreService, ingest::IngestService, reports::ReportService,
    rules::RuleService, runs::RunService, user_defined_functions::UserDefinedFunctionService,
};

#[derive(Clone)]
pub struct SiftMcpServer {
    pub tool_router: ToolRouter<Self>,
    pub prompt_router: PromptRouter<Self>,

    pub asset_service: AssetService,
    pub channel_service: ChannelService,
    pub data_service: DataService,
    pub explore_service: ExploreService,
    pub ingest_service: IngestService,
    pub run_service: RunService,
    pub report_service: ReportService,
    pub rule_service: RuleService,
    pub udf_service: UserDefinedFunctionService,
    pub calculated_channel_service: CalculatedChannelService,
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
    pub fn new(channel: SiftChannel, rest_uri: String) -> Self {
        // Add more routers here as new tool groups are introduced, e.g.
        //   tool_router.merge(Self::ingestion_router())
        let mut tool_router = Self::list_router();
        tool_router.merge(Self::data_router());
        tool_router.merge(Self::explore_router());
        tool_router.merge(Self::mutate_router());

        let prompt_router = Self::prompt_router();

        let asset_service = AssetService::new(channel.clone());
        let data_service = DataService::new(channel.clone());
        let channel_service = ChannelService::new(channel.clone());
        let explore_service = ExploreService::new(rest_uri);
        let ingest_service = IngestService::new(channel.clone());
        let run_service = RunService::new(channel.clone());
        let report_service = ReportService::new(channel.clone());
        let rule_service = RuleService::new(channel.clone());
        let udf_service = UserDefinedFunctionService::new(channel.clone());
        let calculated_channel_service = CalculatedChannelService::new(channel.clone());

        Self {
            asset_service,
            channel_service,
            data_service,
            explore_service,
            ingest_service,
            run_service,
            report_service,
            rule_service,
            udf_service,
            calculated_channel_service,
            tool_router,
            prompt_router,
        }
    }
}
