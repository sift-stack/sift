use crate::service::common;
use anyhow::{Context, Result};
use pbjson_types::FieldMask;
use sift_rs::{
    SiftChannel,
    common::r#type::v1::{FunctionInput, UserDefinedFunction},
    metadata::v1::MetadataValue,
    user_defined_functions::v1::{
        CreateUserDefinedFunctionRequest, CreateUserDefinedFunctionResponse,
        GetUserDefinedFunctionRequest, GetUserDefinedFunctionResponse,
        ListUserDefinedFunctionsRequest, ListUserDefinedFunctionsResponse,
        UpdateUserDefinedFunctionRequest, UpdateUserDefinedFunctionResponse,
        user_defined_function_service_client::UserDefinedFunctionServiceClient,
    },
};
use tonic::Status;

#[cfg(test)]
mod test;

/// Fields a caller may change via [`UserDefinedFunctionService::update_user_defined_function`].
/// `None` means "leave unchanged"; `Some` sets the field and adds it to the update mask.
#[derive(Default)]
pub struct UserDefinedFunctionUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub expression: Option<String>,
    pub function_inputs: Option<Vec<FunctionInput>>,
    pub metadata: Option<Vec<MetadataValue>>,
}

#[derive(Clone)]
pub struct UserDefinedFunctionService {
    channel: SiftChannel,
}

impl UserDefinedFunctionService {
    pub fn new(channel: SiftChannel) -> Self {
        Self { channel }
    }

    pub async fn list_user_defined_functions(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<UserDefinedFunction>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut client = UserDefinedFunctionServiceClient::new(self.channel.clone());
        let mut page_token = String::new();
        let mut results = Vec::new();

        loop {
            let resp = client
                .list_user_defined_functions(ListUserDefinedFunctionsRequest {
                    filter: filter.clone(),
                    page_size,
                    page_token,
                    order_by: order_by.clone().unwrap_or_default(),
                })
                .await
                .context("failed to query user-defined functions")?;

            let ListUserDefinedFunctionsResponse {
                user_defined_functions,
                next_page_token,
            } = resp.into_inner();
            if user_defined_functions.is_empty() {
                break;
            }
            results.extend(user_defined_functions);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    /// Fetch a single function by id or by exact name. Exactly one of `id` /
    /// `name` should be non-empty; the caller (tool layer) enforces that.
    pub async fn get_user_defined_function(
        &self,
        id: String,
        name: String,
    ) -> Result<UserDefinedFunction> {
        let mut client = UserDefinedFunctionServiceClient::new(self.channel.clone());

        let resp = client
            .get_user_defined_function(GetUserDefinedFunctionRequest {
                user_defined_function_id: id,
                name,
            })
            .await
            .context("failed to get user-defined function")?;

        let GetUserDefinedFunctionResponse {
            user_defined_function,
        } = resp.into_inner();

        user_defined_function
            .ok_or_else(|| Status::not_found("user-defined function not found").into())
    }

    pub async fn create_user_defined_function(
        &self,
        name: String,
        description: Option<String>,
        expression: String,
        function_inputs: Vec<FunctionInput>,
        user_notes: Option<String>,
        metadata: Vec<MetadataValue>,
    ) -> Result<UserDefinedFunction> {
        let mut client = UserDefinedFunctionServiceClient::new(self.channel.clone());

        let resp = client
            .create_user_defined_function(CreateUserDefinedFunctionRequest {
                name,
                description,
                expression,
                function_inputs,
                user_notes,
                metadata,
            })
            .await
            .context("failed to create user-defined function")?;

        let CreateUserDefinedFunctionResponse {
            user_defined_function,
        } = resp.into_inner();

        user_defined_function
            .ok_or_else(|| Status::internal("create returned no user-defined function").into())
    }

    /// Read-modify-write: fetch the current function, overlay the provided
    /// fields, and send an update with a mask covering exactly those fields.
    pub async fn update_user_defined_function(
        &self,
        id: String,
        update: UserDefinedFunctionUpdate,
    ) -> Result<UserDefinedFunction> {
        let mut current = self.get_user_defined_function(id, String::new()).await?;

        let mut paths = Vec::new();
        if let Some(name) = update.name {
            current.name = name;
            paths.push("name".to_string());
        }
        if let Some(description) = update.description {
            current.description = description;
            paths.push("description".to_string());
        }
        if let Some(expression) = update.expression {
            current.expression = expression;
            paths.push("expression".to_string());
        }
        if let Some(function_inputs) = update.function_inputs {
            current.function_inputs = function_inputs;
            paths.push("function_inputs".to_string());
        }
        if let Some(metadata) = update.metadata {
            current.metadata = metadata;
            paths.push("metadata".to_string());
        }

        if paths.is_empty() {
            return Err(Status::invalid_argument(
                "no updatable fields provided; set at least one of name, description, expression, function_inputs, metadata",
            )
            .into());
        }

        let mut client = UserDefinedFunctionServiceClient::new(self.channel.clone());
        let resp = client
            .update_user_defined_function(UpdateUserDefinedFunctionRequest {
                user_defined_function: Some(current),
                update_mask: Some(FieldMask { paths }),
            })
            .await
            .context("failed to update user-defined function")?;

        let UpdateUserDefinedFunctionResponse {
            user_defined_function,
        } = resp.into_inner();

        user_defined_function
            .ok_or_else(|| Status::internal("update returned no user-defined function").into())
    }
}
