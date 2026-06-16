use rmcp::model::{CallToolResult, ErrorCode};
use tonic::{Code, Status};

#[cfg(test)]
mod test;

pub type McpResult = Result<CallToolResult, rmcp::ErrorData>;

/// gRPC codes that warrant a "stopped — do not retry in this prompt" message
/// to the agent. Returns a machine-readable reason and the human-readable
/// guidance.
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

pub fn from_anyhow(error: anyhow::Error) -> rmcp::ErrorData {
    if let Some(status) = error.downcast_ref::<Status>() {
        if let Some((reason, guidance)) = soft_signal_guidance(status.code()) {
            return rmcp::ErrorData {
                code: ErrorCode::INTERNAL_ERROR,
                message: guidance.into(),
                data: Some(serde_json::json!({
                    "status": "stopped",
                    "reason": reason,
                    "guidance": guidance,
                })),
            };
        }
    }

    let message = format!("{error:?}");
    let mut code = ErrorCode::INTERNAL_ERROR;

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
