use super::ResourceIdentifier;
use crate::{
    metadata::v1::MetadataValue,
    runs::v2::{
        CreateRunRequest, GetRunRequest, ListRunsRequest, Run, UpdateRunRequest,
        run_service_client::RunServiceClient,
    },
};
use async_trait::async_trait;
use pbjson_types::FieldMask;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use std::ops::{Deref, DerefMut};

/// Creates a new run service wrapper.
///
/// Returns an implementation of [`RunServiceWrapper`] which also exposes methods
/// from the raw [`RunServiceClient`] via `Deref` and `DerefMut`.
///
/// # Arguments
///
/// * `grpc_channel` - The gRPC channel to use for communication
///
/// # Example
///
/// ```no_run
/// use sift_rs::wrappers::runs::{new_run_service, RunServiceWrapper};
/// use sift_connect::{Credentials, SiftChannelBuilder};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let credentials = Credentials::Config {
///     uri: "https://api.siftstack.com".to_string(),
///     apikey: "your-api-key".to_string(),
/// };
/// let channel = SiftChannelBuilder::new(credentials).build()?;
/// let mut service = new_run_service(channel);
///
/// let run = service.try_get_run_by_id("run-123").await?;
/// # Ok(())
/// # }
/// ```
pub fn new_run_service(grpc_channel: SiftChannel) -> impl RunServiceWrapper {
    RunServiceWrapperImpl(RunServiceClient::new(grpc_channel))
}

/// Convenience methods for working with Sift's Run service.
///
/// This trait provides simplified methods that return [`sift_error::Result`] instead
/// of raw gRPC responses. The underlying [`RunServiceClient`] is accessible via
/// `Deref` and `DerefMut` for advanced use cases.
#[async_trait]
pub trait RunServiceWrapper:
    Clone + Deref<Target = RunServiceClient<SiftChannel>> + DerefMut
{
    /// Creates a run.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the run
    /// * `client_key` - A unique identifier for this run
    /// * `description` - A description of the run
    /// * `tags` - Tags to associate with the run
    /// * `metadata` - Metadata key-value pairs to associate with the run
    ///
    /// # Returns
    ///
    /// The created run, or an error if creation fails.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::ArgumentValidationError`] if `name` or `client_key`
    /// is empty. Returns [`ErrorKind::CreateRunError`] if creation fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_rs::wrappers::runs::RunServiceWrapper;
    ///
    /// # async fn example(mut service: impl RunServiceWrapper) -> Result<(), Box<dyn std::error::Error>> {
    /// let run = service.try_create_run(
    ///     "My Run",
    ///     "run-v1",
    ///     "Test run",
    ///     &["test".to_string()],
    ///     &[],
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn try_create_run(
        &mut self,
        name: &str,
        client_key: &str,
        description: &str,
        tags: &[String],
        metadata: &[MetadataValue],
    ) -> Result<Run>;

    /// Updates a run.
    ///
    /// The `updated_run` is expected to contain the `run_id` or `client_key` used to
    /// identify the run to update. The `update_mask` is a list of snake_cased field names
    /// used to indicate which fields should actually be updated. A list of valid field names
    /// can be found at [this link](https://docs.siftstack.com/docs/api/grpc/protocol-buffers/runs#updaterunrequest).
    ///
    /// If `update_mask` is empty, then no update is required and the `updated_run` is
    /// simply returned.
    ///
    /// # Arguments
    ///
    /// * `updated_run` - The run with updated fields (must include `run_id` or `client_key`)
    /// * `update_mask` - List of snake_cased field names to update
    ///
    /// # Returns
    ///
    /// The updated run, or an error if the update fails.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::UpdateRunError`] if the update fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_rs::wrappers::runs::RunServiceWrapper;
    ///
    /// # async fn example(mut service: impl RunServiceWrapper, mut run: sift_rs::runs::v2::Run) -> Result<(), Box<dyn std::error::Error>> {
    /// run.name = "Updated Name".to_string();
    /// let updated = service.try_update_run(run, &["name".to_string()]).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn try_update_run(&mut self, updated_run: Run, update_mask: &[String]) -> Result<Run>;

    /// Retrieves a run by ID.
    ///
    /// # Arguments
    ///
    /// * `run_id` - The ID of the run to retrieve
    ///
    /// # Returns
    ///
    /// The requested run, or an error if it doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::RetrieveRunError`] if retrieval fails.
    async fn try_get_run_by_id(&mut self, run_id: &str) -> Result<Run>;

    /// Retrieves a run by client key.
    ///
    /// # Arguments
    ///
    /// * `client_key` - The client key of the run to retrieve
    ///
    /// # Returns
    ///
    /// The requested run, or an error if it doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::RetrieveRunError`] if retrieval fails.
    /// Returns [`ErrorKind::NotFoundError`] if no run with the given client key exists.
    async fn try_get_run_by_client_key(&mut self, client_key: &str) -> Result<Run>;
}

/// A convience wrapper around [RunServiceClient].
#[derive(Clone)]
struct RunServiceWrapperImpl(RunServiceClient<SiftChannel>);

impl RunServiceWrapperImpl {
    /// Retrieve runs by client-key or ID.
    async fn try_get_run(&mut self, identifier: ResourceIdentifier) -> Result<Run> {
        match identifier {
            ResourceIdentifier::Id(run_id) => self
                .get_run(GetRunRequest { run_id })
                .await
                .map(|res| res.into_inner().run)
                .map_err(|e| Error::new(ErrorKind::RetrieveRunError, e))
                .context("failed to retrieve run")
                .help("ensure that the provided run ID is valid")?
                .ok_or_else(|| {
                    Error::new_empty_response("unexpected empty response from RunService/GetRun")
                }),
            ResourceIdentifier::ClientKey(client_key) => {
                let filter = format!("client_key == '{client_key}'");
                let runs = self
                    .list_runs(ListRunsRequest {
                        filter,
                        page_size: 1,
                        ..Default::default()
                    })
                    .await
                    .map(|res| res.into_inner().runs)
                    .map_err(|e| Error::new(ErrorKind::RetrieveRunError, e))
                    .context("failed to retrieve run")
                    .help("ensure that the provided client key is valid")?;

                runs.first().cloned().ok_or_else(|| {
                    Error::new_msg(
                        ErrorKind::NotFoundError,
                        "no run found with provided client key",
                    )
                })
            }
        }
    }
}

#[async_trait]
impl RunServiceWrapper for RunServiceWrapperImpl {
    /// Creates a run
    async fn try_create_run(
        &mut self,
        name: &str,
        client_key: &str,
        description: &str,
        tags: &[String],
        metadata: &[MetadataValue],
    ) -> Result<Run> {
        let tags = tags.to_vec();
        let metadata = metadata.to_vec();

        if name.is_empty() {
            return Err(Error::new_arg_error("run name cannot be blank"));
        }
        if client_key.is_empty() {
            return Err(Error::new_arg_error("run client-key cannot be blank"));
        }

        let run = self
            .create_run(CreateRunRequest {
                name: name.to_string(),
                description: description.to_string(),
                tags,
                client_key: Some(client_key.to_string()),
                metadata,
                ..Default::default()
            })
            .await
            .map(|res| res.into_inner().run)
            .map_err(|e| Error::new(ErrorKind::CreateRunError, e))
            .context("failed to create run")?;

        run.ok_or_else(|| {
            Error::new_empty_response("unexpected empty response from RunService/CreateRun")
        })
    }

    /// Update a run. The `updated_run` is expected to contain the `run_id` or `client_key` used to
    /// identify the run to update. The `update_mask` is a list of snake_cased field names used to
    /// indicate which fields should actually be updated. A list of valid field names can be found
    /// at [`this link`]. The [Run] returned is the updated run. If `update_mask` is empty, then no
    /// update is required and the `updated_run` is simply returned.
    ///
    /// [`this link`]: https://docs.siftstack.com/docs/api/grpc/protocol-buffers/runs#updaterunrequest
    async fn try_update_run(&mut self, updated_run: Run, update_mask: &[String]) -> Result<Run> {
        if update_mask.is_empty() {
            return Ok(updated_run);
        }

        let run = self
            .update_run(UpdateRunRequest {
                update_mask: Some(FieldMask {
                    paths: update_mask.to_vec(),
                }),
                run: Some(updated_run),
            })
            .await
            .map(|res| res.into_inner().run)
            .map_err(|e| Error::new(ErrorKind::UpdateRunError, e))
            .context("failed to update run")?;

        run.ok_or_else(|| {
            Error::new_empty_response("unexpected empty response from RunService/UpdateRun")
        })
    }

    /// Retrieve a run by ID.
    async fn try_get_run_by_id(&mut self, run_id: &str) -> Result<Run> {
        self.try_get_run(ResourceIdentifier::Id(run_id.to_string()))
            .await
    }

    /// Retrieve a run by client key.
    async fn try_get_run_by_client_key(&mut self, client_key: &str) -> Result<Run> {
        self.try_get_run(ResourceIdentifier::ClientKey(client_key.to_string()))
            .await
    }
}

impl Deref for RunServiceWrapperImpl {
    type Target = RunServiceClient<SiftChannel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RunServiceWrapperImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
