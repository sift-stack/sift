use rmcp::model::{CallToolResult, ErrorCode};
use std::fmt::{self, Display};
use tonic::{Code, Status};

#[cfg(test)]
mod test;

pub type McpResult = Result<CallToolResult, rmcp::ErrorData>;

/// gRPC codes that warrant a "stopped — do not retry in this prompt" message
/// to the agent. Classification follows AIP-194: only `Unavailable` is retried
/// upstream by `with_retry`; every other variant here surfaces immediately.
///
/// `Debug` yields the machine-readable reason (the variant name).
/// `Display` yields the human-readable guidance.
/// `Unclassified` is the catch-all for codes that aren't soft-signals and
/// should pass through to the generic error path in `from_anyhow`.
#[derive(Debug)]
enum AgentSignal {
    BackendUnreachable,
    RateLimited,
    QueryTooExpensive,
    BackendError,
    Conflict,
    AlreadyExists,
    PermissionDenied,
    Unauthenticated,
    Cancelled,
    Unclassified,
}

impl From<Code> for AgentSignal {
    fn from(code: Code) -> Self {
        match code {
            Code::Unavailable => Self::BackendUnreachable,
            Code::ResourceExhausted => Self::RateLimited,
            Code::DeadlineExceeded => Self::QueryTooExpensive,
            Code::Internal => Self::BackendError,
            Code::Aborted => Self::Conflict,
            Code::AlreadyExists => Self::AlreadyExists,
            Code::PermissionDenied => Self::PermissionDenied,
            Code::Unauthenticated => Self::Unauthenticated,
            Code::Cancelled => Self::Cancelled,
            _ => Self::Unclassified,
        }
    }
}

impl Display for AgentSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackendUnreachable => write!(
                f,
                "The Sift backend was unreachable after multiple attempts. \
                 Do not retry this tool in this prompt. Inform the user that the backend appears to be down."
            ),
            Self::RateLimited => write!(
                f,
                "The Sift backend is rate-limiting requests. \
                 Do not retry this tool in this prompt. Either wait, narrow your query (shorter time \
                 range or more specific filter), or ask the user to refine their request."
            ),
            Self::QueryTooExpensive => write!(
                f,
                "The query exceeded the server's deadline. \
                 Do not retry this tool with the same parameters. Narrow your query (shorter time range, \
                 more specific filter, or smaller limit) before trying again."
            ),
            Self::BackendError => write!(
                f,
                "The Sift backend returned an internal error. \
                 Do not retry this tool in this prompt. Surface this to the user; the issue is on the backend side."
            ),
            Self::Conflict => write!(
                f,
                "The Sift backend reported a conflict (concurrent modification or constraint violation). \
                 Do not retry this tool in this prompt. Surface this to the user so they can resolve the conflicting state."
            ),
            Self::AlreadyExists => write!(
                f,
                "The resource already exists. \
                 Do not retry; the operation cannot succeed as-is. If the user intended to update an existing resource, surface this distinction to them."
            ),
            Self::PermissionDenied => write!(
                f,
                "The caller lacks permission for this operation. \
                 Do not retry; permission will not change within this prompt. Surface this to the user."
            ),
            Self::Unauthenticated => write!(
                f,
                "Authentication is missing or invalid. \
                 Do not retry; the user must re-authenticate. Surface this to the user."
            ),
            Self::Cancelled => write!(
                f,
                "The request was cancelled. \
                 Do not retry; the cancellation is intentional. Surface this to the user."
            ),
            Self::Unclassified => write!(f, "Unclassified gRPC error."),
        }
    }
}

pub fn from_anyhow(error: anyhow::Error) -> rmcp::ErrorData {
    if let Some(status) = error.downcast_ref::<Status>() {
        let signal = AgentSignal::from(status.code());
        if !matches!(signal, AgentSignal::Unclassified) {
            let reason = format!("{signal:?}");
            let guidance = signal.to_string();
            return rmcp::ErrorData {
                code: ErrorCode::INTERNAL_ERROR,
                message: guidance.clone().into(),
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
