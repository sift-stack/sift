use super::ResourceIdentifier;
use crate::runs::v2::{
    run_service_client::RunServiceClient, CreateRunRequest, GetRunRequest, ListRunsRequest, Run,
    UpdateRunRequest,
};
use pbjson_types::FieldMask;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use std::ops::{Deref, DerefMut};

/// A convience wrapper around [RunServiceClient].
pub struct RunServiceWrapper(RunServiceClient<SiftChannel>);

impl RunServiceWrapper {
    pub fn new(grpc_channel: SiftChannel) -> Self {
        Self(RunServiceClient::new(grpc_channel))
    }

    /// Creates a run
    pub async fn try_create_run<S: AsRef<str>>(
        &mut self,
        name: S,
        client_key: S,
        description: S,
        tags: &[String],
    ) -> Result<Run> {
        let name = name.as_ref().to_string();
        let client_key = client_key.as_ref().to_string();
        let description = description.as_ref().to_string();
        let tags = tags.to_vec();

        if name.is_empty() {
            return Err(Error::new_arg_error("run name cannot be blank"));
        }
        if client_key.is_empty() {
            return Err(Error::new_arg_error("run client-key cannot be blank"));
        }

        let run = self
            .create_run(CreateRunRequest {
                name,
                description,
                tags,
                client_key: Some(client_key),
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
    /// identify the run to update. The `field_mask` is a list of snake_cased field names used to
    /// indicate which fields should actually be updated. A list of valid field names can be found
    /// at [`this link`]. The [Run] returned is the updated run. If `field_masks` is empty, then no
    /// update is required and the `updated_run` is simply returned.
    ///
    /// [`this link`]: https://docs.siftstack.com/docs/api/grpc/protocol-buffers/runs#updaterunrequest
    pub async fn try_update_run(
        &mut self,
        updated_run: Run,
        field_masks: &[String],
    ) -> Result<Run> {
        if field_masks.is_empty() {
            return Ok(updated_run);
        }

        let run = self
            .update_run(UpdateRunRequest {
                update_mask: Some(FieldMask {
                    paths: field_masks.to_vec(),
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
    pub async fn try_get_run_by_id<S: AsRef<str>>(&mut self, run_id: S) -> Result<Run> {
        self.try_get_run(ResourceIdentifier::Id(run_id.as_ref().to_string()))
            .await
    }

    /// Retrieve a run by client key.
    pub async fn try_get_run_by_client_key<S: AsRef<str>>(&mut self, client_key: S) -> Result<Run> {
        self.try_get_run(ResourceIdentifier::ClientKey(
            client_key.as_ref().to_string(),
        ))
        .await
    }

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

impl Deref for RunServiceWrapper {
    type Target = RunServiceClient<SiftChannel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RunServiceWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
