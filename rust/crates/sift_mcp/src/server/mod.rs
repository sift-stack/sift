use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    handler::server::router::prompt::PromptRouter,
    handler::server::tool::{ToolCallContext, ToolRouter},
    model::{
        CacheScope, CallToolRequestParams, CallToolResponse, Implementation, ListToolsResult,
        PaginatedRequestParams, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    prompt_handler,
    service::RequestContext,
    tool_handler,
};
use sift_rs::SiftChannel;
#[cfg(test)]
use tokio::sync::watch;

#[cfg(test)]
use crate::UpdateCheck;
use crate::{UpdateCheckReceiver, client_event::ClientEventReporter, policy::RetryPolicy};

#[cfg(test)]
mod test;

const UPDATE_CHECK_INSTRUCTIONS: &str = concat!(
    "Call `check_for_updates` once at the start of each session, before you call any other Sift tool. ",
    "If it reports `update_available`, relay its `message` and exact ",
    "`install_command` to the user. If the check is unavailable, continue with ",
    "the requested Sift task. "
);
pub(crate) const BASE_INSTRUCTIONS: &str = concat!(
    "Use Sift tools for telemetry discovery, analysis, ",
    "and ingestion. Run `sift-cli agent doctor` for read-only integration ",
    "diagnosis, `sift-cli agent install` for first setup, and `sift-cli agent ",
    "update` to refresh every detected client together. Never enable destructive ",
    "tools without explicit user approval. Result objects follow proto3 JSON ",
    "rules: fields at their default value (false, 0, empty string/list) are ",
    "omitted, so a missing boolean key means false, not unknown."
);
#[cfg(feature = "test-reports")]
use crate::service::test_reports::TestReportService;
use crate::service::{
    annotations::AnnotationService, assets::AssetService,
    calculated_channels::CalculatedChannelService, channels::ChannelService, data::DataService,
    docs::DocsService, ingest::IngestService, ping::PingService,
    report_templates::ReportTemplateService, reports::ReportService,
    rule_evaluation::RuleEvaluationService, rules::RuleService, runs::RunService, url::UrlService,
    users::UserService,
};

#[derive(Clone)]
pub struct SiftMcpServer {
    pub tool_router: ToolRouter<Self>,
    pub prompt_router: PromptRouter<Self>,

    pub annotation_service: AnnotationService,
    pub asset_service: AssetService,
    pub calculated_channel_service: CalculatedChannelService,
    pub channel_service: ChannelService,
    pub data_service: DataService,
    pub url_service: UrlService,
    pub ingest_service: IngestService,
    pub ping_service: PingService,
    pub run_service: RunService,
    pub report_service: ReportService,
    pub report_template_service: ReportTemplateService,
    pub rule_service: RuleService,
    pub rule_evaluation_service: RuleEvaluationService,
    #[cfg(feature = "test-reports")]
    pub test_report_service: TestReportService,
    pub docs_service: DocsService,
    pub user_service: UserService,

    pub allow_create: bool,
    pub allow_destructive: bool,
    cli_version: String,
    pub update_check: Option<UpdateCheckReceiver>,
    client_event_reporter: ClientEventReporter,
}

#[tool_handler(router = self.tool_router)]
#[prompt_handler(router = self.prompt_router)]
impl ServerHandler for SiftMcpServer {
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let _ = self.client_event_reporter.send(request.name.as_ref()).await;

        let context = ToolCallContext::new(self, request, context);
        self.tool_router.call(context).await
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
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
        let mut result = ListToolsResult::with_all_items(tools);
        if context
            .protocol_version()
            .is_some_and(|version| version >= ProtocolVersion::V_2026_07_28)
        {
            result = result.with_ttl_ms(0).with_cache_scope(CacheScope::Public);
        }
        Ok(result)
    }

    fn get_info(&self) -> ServerInfo {
        let instructions = match &self.update_check {
            Some(receiver) => {
                let update_check = receiver.borrow();
                let base = format!("{UPDATE_CHECK_INSTRUCTIONS}{BASE_INSTRUCTIONS}");
                match update_check.update_message() {
                    Some(message) => format!("{message}\n\n{base}"),
                    None => base,
                }
            }
            None => BASE_INSTRUCTIONS.to_string(),
        };

        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
        )
        .with_server_info(Implementation::new("SiftMcp", self.cli_version.clone()))
        .with_instructions(instructions)
    }
}

impl SiftMcpServer {
    #[cfg(test)]
    pub fn new(
        channel: SiftChannel,
        app_uri: String,
        allow_create: bool,
        allow_destructive: bool,
    ) -> Self {
        let version = env!("CARGO_PKG_VERSION").to_string();
        let update_check = watch::channel(UpdateCheck::Current {
            current_version: version.clone(),
            latest_version: version.clone(),
        })
        .1;
        Self::new_with_client_events(
            channel,
            app_uri,
            allow_create,
            allow_destructive,
            version,
            Some(update_check),
            ClientEventReporter::default(),
        )
    }

    pub(crate) fn new_with_client_events(
        channel: SiftChannel,
        app_uri: String,
        allow_create: bool,
        allow_destructive: bool,
        cli_version: String,
        update_check: Option<UpdateCheckReceiver>,
        client_event_reporter: ClientEventReporter,
    ) -> Self {
        // Add more routers here as new tool groups are introduced, e.g.
        //   tool_router.merge(Self::ingestion_router())
        let mut tool_router = Self::assets_router();
        tool_router.merge(Self::runs_router());
        tool_router.merge(Self::channels_router());
        tool_router.merge(Self::calculated_channels_router());
        tool_router.merge(Self::reports_router());
        tool_router.merge(Self::report_templates_router());
        tool_router.merge(Self::data_router());
        tool_router.merge(Self::explore_router());
        tool_router.merge(Self::ping_router());
        tool_router.merge(Self::rules_router());
        tool_router.merge(Self::rule_evaluation_router());
        tool_router.merge(Self::annotations_router());
        #[cfg(feature = "test-reports")]
        tool_router.merge(Self::test_reports_router());
        tool_router.merge(Self::docs_router());
        tool_router.merge(Self::users_router());
        if update_check.is_some() {
            tool_router.merge(Self::update_router());
        }

        let prompt_router = Self::prompt_router();

        let retry_policy = RetryPolicy::default();

        let annotation_service = AnnotationService::new(channel.clone(), retry_policy.clone());
        let asset_service = AssetService::new(channel.clone(), retry_policy.clone());
        let calculated_channel_service =
            CalculatedChannelService::new(channel.clone(), retry_policy.clone());
        let data_service = DataService::new(channel.clone(), retry_policy.clone());
        let channel_service = ChannelService::new(channel.clone(), retry_policy.clone());
        let url_service = UrlService::new(app_uri);
        let ingest_service = IngestService::new(channel.clone());
        let ping_service = PingService::new(channel.clone(), retry_policy.clone());
        let run_service = RunService::new(channel.clone(), retry_policy.clone());
        let report_service = ReportService::new(channel.clone(), retry_policy.clone());
        let report_template_service =
            ReportTemplateService::new(channel.clone(), retry_policy.clone());
        let rule_service = RuleService::new(channel.clone(), retry_policy.clone());
        let rule_evaluation_service =
            RuleEvaluationService::new(channel.clone(), retry_policy.clone());
        #[cfg(feature = "test-reports")]
        let test_report_service = TestReportService::new(channel.clone(), retry_policy.clone());
        let docs_service = DocsService::new(channel.clone(), retry_policy.clone());
        let user_service = UserService::new(channel.clone(), retry_policy);

        Self {
            annotation_service,
            asset_service,
            calculated_channel_service,
            channel_service,
            data_service,
            url_service,
            ingest_service,
            ping_service,
            run_service,
            report_service,
            report_template_service,
            rule_service,
            rule_evaluation_service,
            #[cfg(feature = "test-reports")]
            test_report_service,
            docs_service,
            user_service,
            tool_router,
            prompt_router,
            allow_create,
            allow_destructive,
            cli_version,
            update_check,
            client_event_reporter,
        }
    }

    pub(crate) fn require_create(&self) -> Result<(), ErrorData> {
        if self.allow_create || self.allow_destructive {
            return Ok(());
        }
        Err(ErrorData::invalid_request(
            "This is a create tool. The MCP server was launched without \
             `--allow-create`, so tools that create new resources are disabled. \
             Ask the user for explicit approval to enable them. If they approve, \
             run `sift-cli agent update --allow-create` to update every detected \
             client together, then ask the user to reload or restart their MCP \
             client. Do not retry until they confirm the client has restarted.",
            Some(serde_json::json!({
                "status": "stopped",
                "reason": "CreateToolsDisabled",
                "requires_user_approval": true,
                "remediation_command": "sift-cli agent update --allow-create",
                "restart_required": true,
            })),
        ))
    }

    pub(crate) fn require_destructive(&self) -> Result<(), ErrorData> {
        if self.allow_destructive {
            return Ok(());
        }
        Err(ErrorData::invalid_request(
            "This is a destructive tool. The MCP server was launched without \
             `--allow-destructive`, so tools that modify or archive resources \
             are disabled. Ask the user for explicit approval to enable them. \
             If they approve, run `sift-cli agent update --allow-destructive` \
             to update every detected client together, then ask the user to \
             reload or restart their MCP client. Do not retry until they \
             confirm the client has restarted.",
            Some(serde_json::json!({
                "status": "stopped",
                "reason": "DestructiveToolsDisabled",
                "requires_user_approval": true,
                "remediation_command": "sift-cli agent update --allow-destructive",
                "restart_required": true,
            })),
        ))
    }
}
