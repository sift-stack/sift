use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    handler::server::router::prompt::PromptRouter,
    handler::server::tool::ToolRouter,
    model::{
        GetPromptRequestParams, GetPromptResult, ListPromptsResult, ListToolsResult,
        PaginatedRequestParams,
    },
    prompt_handler,
    service::RequestContext,
    tool_handler,
};
use sift_rs::SiftChannel;

use crate::policy::RetryPolicy;
use crate::service::{
    annotations::AnnotationService, assets::AssetService, channels::ChannelService,
    data::DataService, docs::DocsService, ingest::IngestService, ping::PingService,
    reports::ReportService, rules::RuleService, runs::RunService, test_reports::TestReportService,
    url::UrlService,
};

#[derive(Clone)]
pub struct SiftMcpServer {
    pub tool_router: ToolRouter<Self>,
    pub prompt_router: PromptRouter<Self>,

    pub annotation_service: AnnotationService,
    pub asset_service: AssetService,
    pub channel_service: ChannelService,
    pub data_service: DataService,
    pub url_service: UrlService,
    pub ingest_service: IngestService,
    pub ping_service: PingService,
    pub run_service: RunService,
    pub report_service: ReportService,
    pub rule_service: RuleService,
    pub test_report_service: TestReportService,
    pub docs_service: DocsService,
}

#[tool_handler(
    router = self.tool_router,
    name = "SiftMcp",
    version = "0.1.0",
    instructions = "Sift MCP Server",
)]
#[prompt_handler(router = self.prompt_router)]
impl ServerHandler for SiftMcpServer {
    // Override rmcp's default `list_tools` (which sorts by tool `name`) so
    // tools group by domain in `tools/list`. Because our titles are
    // `<domain>/<tool>`, sorting by title alphabetically clusters all tools
    // of the same domain together. Falls back to `name` for any tool that
    // omits a title.
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let mut tools = self.tool_router.list_all();
        tools.sort_by(|a, b| {
            let a_key: &str = a
                .annotations
                .as_ref()
                .and_then(|ann| ann.title.as_deref())
                .unwrap_or(a.name.as_ref());
            let b_key: &str = b
                .annotations
                .as_ref()
                .and_then(|ann| ann.title.as_deref())
                .unwrap_or(b.name.as_ref());
            a_key.cmp(b_key)
        });
        Ok(ListToolsResult {
            tools,
            meta: None,
            next_cursor: None,
        })
    }
}

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
        tool_router.merge(Self::annotations_router());
        tool_router.merge(Self::test_reports_router());
        tool_router.merge(Self::docs_router());

        let prompt_router = Self::prompt_router();

        let retry_policy = RetryPolicy::default();

        let annotation_service = AnnotationService::new(channel.clone(), retry_policy.clone());
        let asset_service = AssetService::new(channel.clone(), retry_policy.clone());
        let data_service = DataService::new(channel.clone(), retry_policy.clone());
        let channel_service = ChannelService::new(channel.clone(), retry_policy.clone());
        let url_service = UrlService::new(rest_uri);
        let ingest_service = IngestService::new(channel.clone());
        let ping_service = PingService::new(channel.clone(), retry_policy.clone());
        let run_service = RunService::new(channel.clone(), retry_policy.clone());
        let report_service = ReportService::new(channel.clone(), retry_policy.clone());
        let rule_service = RuleService::new(channel.clone(), retry_policy.clone());
        let test_report_service = TestReportService::new(channel.clone(), retry_policy.clone());
        let docs_service = DocsService::new(channel.clone(), retry_policy);

        Self {
            annotation_service,
            asset_service,
            channel_service,
            data_service,
            url_service,
            ingest_service,
            ping_service,
            run_service,
            report_service,
            rule_service,
            test_report_service,
            docs_service,
            tool_router,
            prompt_router,
        }
    }
}
