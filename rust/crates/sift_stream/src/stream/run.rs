use super::builder::RunForm;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    runs::v2::Run,
    wrappers::runs::{RunServiceWrapper, new_run_service},
};
use std::collections::HashSet;

pub enum RunSelector {
    ById(String),
    ByForm(RunForm),
}

/// Retrieves a run by run ID.
pub(super) async fn load_run_by_id(grpc_channel: SiftChannel, run_id: &str) -> Result<Run> {
    let mut run_service = new_run_service(grpc_channel);
    let run = run_service.try_get_run_by_id(run_id).await?;

    #[cfg(feature = "tracing")]
    tracing::info!(
        run_id = run.run_id,
        run_name = run.name,
        "successfully retrieve run by ID",
    );

    Ok(run)
}

/// Retrieves a run or creates a run. If the run exists, this method will also update the run
/// if the `run_form` has changed since the last time it was used.
pub(super) async fn load_run_by_form(grpc_channel: SiftChannel, run_form: RunForm) -> Result<Run> {
    #[cfg(feature = "tracing")]
    tracing::info_span!("load_run_by_form");

    let mut run_service = new_run_service(grpc_channel);

    let RunForm {
        name,
        description,
        tags,
        client_key,
    } = run_form;

    match run_service.try_get_run_by_client_key(&client_key).await {
        Err(e) if e.kind() == ErrorKind::NotFoundError => {
            let run = run_service
                .try_create_run(
                    &name,
                    &client_key,
                    &description.unwrap_or_default(),
                    tags.unwrap_or_default().as_slice(),
                )
                .await?;

            #[cfg(feature = "tracing")]
            tracing::info!(run_id = run.run_id, run_name = run.name, "created new run");

            Ok(run)
        }
        Err(e) => Err(e),

        Ok(mut run) => {
            #[cfg(feature = "tracing")]
            tracing::info!(
                run_id = run.run_id,
                run_name = run.name,
                "an existing run was found with the provided client-key"
            );

            // An existing run was found; see if we need to update it.
            let mut update_mask = Vec::new();

            if name != run.name {
                update_mask.push("name".to_string());
                run.name = name;
            }

            if description.as_ref().is_some_and(|d| d != &run.description) {
                update_mask.push("description".to_string());
                run.description = description.unwrap_or_default();
            }
            match tags {
                Some(new_tags) if run.tags.is_empty() => {
                    update_mask.push("tags".to_string());
                    run.tags = new_tags;
                }
                Some(new_tags) => {
                    let new_tags_set = HashSet::<&String>::from_iter(new_tags.iter());
                    let current_tags_set = HashSet::from_iter(run.tags.iter());
                    let difference = new_tags_set.difference(&current_tags_set);

                    if difference.count() == 0 {
                        update_mask.push("tags".to_string());
                        run.tags = new_tags;
                    }
                }
                _ => (),
            }

            if update_mask.is_empty() {
                return Ok(run);
            }

            #[cfg(feature = "tracing")]
            tracing::info!(
                "updating run fields as some fields have changed: {}",
                update_mask.join(", ")
            );

            let updated_run = run_service.try_update_run(run, &update_mask).await?;

            #[cfg(feature = "tracing")]
            tracing::info!("successfully updated run");

            Ok(updated_run)
        }
    }
}
