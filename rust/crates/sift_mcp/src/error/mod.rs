use rmcp::model::{CallToolResult, ErrorCode};
use tonic::{Code, Status};

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
