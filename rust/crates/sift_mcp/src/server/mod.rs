use rmcp::{ServerHandler, handler::server::tool::ToolRouter, tool_handler};
use sift_rs::SiftChannel;

#[derive(Clone)]
pub struct SiftMcpServer {
    pub(crate) channel: SiftChannel,
    pub tool_router: ToolRouter<Self>,
}

#[tool_handler(
    router = self.tool_router,
    name = "SiftMcp",
    version = "0.1.0",
    instructions = "Sift MCP Server",
)]
impl ServerHandler for SiftMcpServer {}

impl SiftMcpServer {
    pub fn new(channel: SiftChannel) -> Self {
        // Add more routers here as new tool groups are introduced, e.g.
        //   Self::resource_router().merge(Self::ingestion_router())

        let tool_router = Self::resource_router();
        Self {
            channel,
            tool_router,
        }
    }
}
