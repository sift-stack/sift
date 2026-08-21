use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result, anyhow};
use pbjson_types::FieldMask;
use sift_rs::{
    SiftChannel,
    common::r#type::v1::{FunctionInput, UserDefinedFunction},
    metadata::v1::MetadataValue,
    user_defined_functions::v1::{
        CreateUserDefinedFunctionRequest, ListUserDefinedFunctionVersionsRequest,
        ListUserDefinedFunctionVersionsResponse, ListUserDefinedFunctionsRequest,
        ListUserDefinedFunctionsResponse, UpdateUserDefinedFunctionRequest,
        user_defined_function_service_client::UserDefinedFunctionServiceClient,
    },
};

#[cfg(test)]
mod test;

/// A partial set of changes to apply to an existing user defined function.
/// `None` means "leave unchanged" — only the fields set here reach the update
/// mask. Archive state has its own entry point
/// ([`UserDefinedFunctionService::set_user_defined_function_archived`]) so the
/// archive flip stays a separate, gated operation.
#[derive(Debug, Default)]
pub struct UdfUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub expression: Option<String>,
    pub function_inputs: Option<Vec<FunctionInput>>,
    pub metadata: Option<Vec<MetadataValue>>,
}

#[derive(Clone)]
pub struct UserDefinedFunctionService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl UserDefinedFunctionService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    /// Lists the latest version of each user defined function.
    pub async fn list_user_defined_functions(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<UserDefinedFunction>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = UserDefinedFunctionServiceClient::new(channel);
                    client
                        .list_user_defined_functions(ListUserDefinedFunctionsRequest {
                            page_size,
                            page_token: token,
                            filter,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query user defined functions")?;

            let ListUserDefinedFunctionsResponse {
                user_defined_functions,
                next_page_token,
            } = resp;
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

    /// Lists the version history of one user defined function. The caller
    /// guarantees exactly one of `user_defined_function_id` or `name` is
    /// non-empty; the proto ignores `name` when the id is present.
    pub async fn list_user_defined_function_versions(
        &self,
        user_defined_function_id: String,
        name: String,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<UserDefinedFunction>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let user_defined_function_id = user_defined_function_id.clone();
            let name = name.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let resp = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let user_defined_function_id = user_defined_function_id.clone();
                let name = name.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = UserDefinedFunctionServiceClient::new(channel);
                    client
                        .list_user_defined_function_versions(
                            ListUserDefinedFunctionVersionsRequest {
                                user_defined_function_id,
                                name,
                                page_size,
                                page_token: token,
                                filter,
                                order_by,
                            },
                        )
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query user defined function versions")?;

            let ListUserDefinedFunctionVersionsResponse {
                user_defined_functions,
                next_page_token,
            } = resp;
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

    /// Creates a user defined function at version 1 and returns it.
    pub async fn create_user_defined_function(
        &self,
        name: String,
        description: Option<String>,
        expression: String,
        function_inputs: Vec<FunctionInput>,
        user_notes: Option<String>,
        metadata: Vec<MetadataValue>,
    ) -> Result<UserDefinedFunction> {
        let channel = self.channel.clone();

        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let name = name.clone();
            let description = description.clone();
            let expression = expression.clone();
            let function_inputs = function_inputs.clone();
            let user_notes = user_notes.clone();
            let metadata = metadata.clone();
            async move {
                let mut client = UserDefinedFunctionServiceClient::new(channel);
                client
                    .create_user_defined_function(CreateUserDefinedFunctionRequest {
                        name,
                        description,
                        expression,
                        function_inputs,
                        user_notes,
                        metadata,
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to create user defined function")?;

        resp.user_defined_function.ok_or_else(|| {
            anyhow!("create_user_defined_function response missing user defined function")
        })
    }

    /// Updates a user defined function through the field mask. Per
    /// `protos/sift/user_defined_functions/v1/user_defined_functions.proto::UpdateUserDefinedFunctionRequest`
    /// the updatable fields are `name`, `archived_date`, `is_archived`,
    /// `description`, `expression`, `function_inputs`, and `metadata`; archive
    /// state goes through [`Self::set_user_defined_function_archived`].
    ///
    /// The RPC has no version precondition. Every accepted update creates a new
    /// version and returns it, so the request carries only the id and the masked
    /// fields — never a version read earlier by the caller.
    pub async fn update_user_defined_function(
        &self,
        user_defined_function_id: String,
        changes: UdfUpdate,
    ) -> Result<UserDefinedFunction> {
        let mut function = UserDefinedFunction {
            user_defined_function_id,
            ..Default::default()
        };
        let mut paths = Vec::new();

        let UdfUpdate {
            name,
            description,
            expression,
            function_inputs,
            metadata,
        } = changes;

        if let Some(v) = name {
            function.name = v;
            paths.push("name".to_string());
        }
        if let Some(v) = description {
            function.description = v;
            paths.push("description".to_string());
        }
        if let Some(v) = expression {
            function.expression = v;
            paths.push("expression".to_string());
        }
        if let Some(v) = function_inputs {
            function.function_inputs = v;
            paths.push("function_inputs".to_string());
        }
        if let Some(v) = metadata {
            function.metadata = v;
            paths.push("metadata".to_string());
        }

        self.send_update(function, paths).await
    }

    /// Archives or unarchives a user defined function. There is no dedicated
    /// archive RPC: the proto sets `is_archived` through the update mask.
    pub async fn set_user_defined_function_archived(
        &self,
        user_defined_function_id: String,
        is_archived: bool,
    ) -> Result<UserDefinedFunction> {
        let function = UserDefinedFunction {
            user_defined_function_id,
            is_archived,
            ..Default::default()
        };

        self.send_update(function, vec!["is_archived".to_string()])
            .await
    }

    async fn send_update(
        &self,
        function: UserDefinedFunction,
        paths: Vec<String>,
    ) -> Result<UserDefinedFunction> {
        let channel = self.channel.clone();
        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            let function = function.clone();
            let paths = paths.clone();
            async move {
                let mut client = UserDefinedFunctionServiceClient::new(channel);
                client
                    .update_user_defined_function(UpdateUserDefinedFunctionRequest {
                        user_defined_function: Some(function),
                        update_mask: Some(FieldMask { paths }),
                    })
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to update user defined function")?;

        resp.user_defined_function.ok_or_else(|| {
            anyhow!("update_user_defined_function response missing user defined function")
        })
    }
}
