use rmcp::model::{CallToolResult, ErrorCode};
use serde_json::json;
use tonic::{Code, Status};

pub type McpResult = Result<CallToolResult, rmcp::ErrorData>;

pub fn from_grpc_status(status: Status) -> rmcp::ErrorData {
    let code = from_grpc_code(status.code());
    let message = status.message().to_string();
    let data = Some(json!({
        "grpc_code": status.code().to_string(),
    }));

    rmcp::ErrorData {
        code,
        message: message.into(),
        data,
    }
}

pub fn from_anyhow(error: anyhow::Error) -> rmcp::ErrorData {
    let code = ErrorCode::INTERNAL_ERROR;
    let message = format!("{error:?}");

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
