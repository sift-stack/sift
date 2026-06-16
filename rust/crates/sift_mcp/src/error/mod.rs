use rmcp::model::{CallToolResult, Content, ErrorCode};
use tonic::{Code, Status};

#[cfg(test)]
mod test;

pub type McpResult = Result<CallToolResult, rmcp::ErrorData>;

pub fn from_anyhow(error: anyhow::Error) -> rmcp::ErrorData {
    let mut code = ErrorCode::INTERNAL_ERROR;
    let message = format!("{error:?}");

    if let Ok(grpc_status) = error.downcast::<Status>() {
        code = from_grpc_code(grpc_status.code());
    }

    rmcp::ErrorData {
        code,
        message: message.into(),
        data: None,
    }
}

fn from_grpc_code(code: Code) -> ErrorCode {
    match code {
        Code::InvalidArgument | Code::OutOfRange => ErrorCode::INVALID_PARAMS,
        Code::NotFound => ErrorCode::RESOURCE_NOT_FOUND,
        Code::Unimplemented => ErrorCode::METHOD_NOT_FOUND,
        _ => ErrorCode::INTERNAL_ERROR,
    }
}

/// gRPC codes that warrant a structured "stopped, do not retry in this prompt"
/// response to the calling agent instead of an opaque error. Returns the
/// machine-readable reason and the human-readable guidance text shown to the
/// agent.
fn soft_signal_guidance(code: Code) -> Option<(&'static str, &'static str)> {
    match code {
        Code::Unavailable => Some((
            "backend_unreachable",
            "The Sift backend was unreachable after multiple attempts. \
             Do not retry this tool in this prompt. Inform the user that the backend appears to be down.",
        )),
        Code::ResourceExhausted => Some((
            "rate_limited",
            "The Sift backend is rate-limiting requests. \
             Do not retry this tool in this prompt. Either wait, narrow your query (shorter time \
             range or more specific filter), or ask the user to refine their request.",
        )),
        Code::DeadlineExceeded => Some((
            "query_too_expensive",
            "The query exceeded the server's deadline. \
             Do not retry this tool with the same parameters. Narrow your query (shorter time range, \
             more specific filter, or smaller limit) before trying again.",
        )),
        Code::Internal => Some((
            "backend_error",
            "The Sift backend returned an internal error. \
             Do not retry this tool in this prompt. Surface this to the user; the issue is on the backend side.",
        )),
        _ => None,
    }
}

/// Convert an `anyhow::Error` into an `McpResult`. gRPC status codes that map
/// to a soft signal (see [`soft_signal_guidance`]) are returned as a structured
/// `CallToolResult` instructing the agent not to retry; everything else falls
/// back to [`from_anyhow`].
pub fn err_into_mcp_result(err: anyhow::Error) -> McpResult {
    let soft = err
        .downcast_ref::<Status>()
        .and_then(|s| soft_signal_guidance(s.code()));
    if let Some((reason, guidance)) = soft {
        let mut result = CallToolResult::structured(serde_json::json!({
            "status": "stopped",
            "reason": reason,
            "guidance": guidance,
        }));
        result.content = vec![Content::text(guidance.to_string())];
        return Ok(result);
    }
    Err(from_anyhow(err))
}

/// Convert a fallible operation's result into an `McpResult`. `Ok` values are
/// wrapped in [`CallToolResult::structured`]; `Err` values route through
/// [`err_into_mcp_result`].
pub fn into_tool_result(result: Result<serde_json::Value, anyhow::Error>) -> McpResult {
    match result {
        Ok(value) => Ok(CallToolResult::structured(value)),
        Err(err) => err_into_mcp_result(err),
    }
}

/// `?`-like macro for tool handlers: returns the unwrapped value on `Ok`,
/// early-returns an `McpResult` (either a soft-signal `Ok` or a normal `Err`)
/// from the enclosing function on `Err`. Use inside `pub async fn ... -> McpResult`.
#[macro_export]
macro_rules! tool_try {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return $crate::error::err_into_mcp_result(e),
        }
    };
}
