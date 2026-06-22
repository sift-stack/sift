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

use crate::policy::RetryPolicy;
use crate::service::{
    assets::AssetService, channels::ChannelService, data::DataService, explore::ExploreService,
    ingest::IngestService, ping::PingService, reports::ReportService, rules::RuleService,
    runs::RunService,
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
    pub ping_service: PingService,
    pub run_service: RunService,
    pub report_service: ReportService,
    pub rule_service: RuleService,
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
        let mut tool_router = Self::assets_router();
        tool_router.merge(Self::runs_router());
        tool_router.merge(Self::channels_router());
        tool_router.merge(Self::reports_router());
        tool_router.merge(Self::data_router());
        tool_router.merge(Self::explore_router());
        tool_router.merge(Self::ping_router());
        tool_router.merge(Self::rules_router());

        let prompt_router = Self::prompt_router();

        let retry_policy = RetryPolicy::default();

        let asset_service = AssetService::new(channel.clone(), retry_policy.clone());
        let data_service = DataService::new(channel.clone(), retry_policy.clone());
        let channel_service = ChannelService::new(channel.clone(), retry_policy.clone());
        let explore_service = ExploreService::new(rest_uri);
        let ingest_service = IngestService::new(channel.clone());
        let ping_service = PingService::new(channel.clone(), retry_policy.clone());
        let run_service = RunService::new(channel.clone(), retry_policy.clone());
        let report_service = ReportService::new(channel.clone(), retry_policy.clone());
        let rule_service = RuleService::new(channel.clone(), retry_policy);

        Self {
            asset_service,
            channel_service,
            data_service,
            explore_service,
            ingest_service,
            ping_service,
            run_service,
            report_service,
            rule_service,
            tool_router,
            prompt_router,
        }
    }
}
