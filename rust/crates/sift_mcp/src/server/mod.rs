use sift_rs::SiftChannel;

use crate::tool::resource::ResourceTool;

#[derive(Clone)]
pub struct SiftMcpServer {
    pub resource_tool: ResourceTool<SiftChannel>
}
