use sift_error::prelude::*;
use sift_rs::ingestion_configs::v2::FlowConfig;

#[cfg(test)]
mod test;

/// Compares two sets of flows and ensures that all flows in `user_specified` has a corresponding
/// equivalent flow in `sift_flows`. If there is no corresponding flow then it either doesn't exist
/// in Sift (this would be a bug) or a user made a backwards incompatible change to their ingestion
/// config.
pub fn validate_flows(user_specified: &[FlowConfig], sift_flows: &[FlowConfig]) -> Result<()> {
    for user_flow in user_specified {
        let num_matches_by_name = sift_flows
            .iter()
            .filter(|f| user_flow.name == f.name)
            .count();
        let num_exact_matches = sift_flows.iter().filter(|f| &user_flow == f).count();

        if num_matches_by_name > 0 && num_exact_matches == 0 {
            return Err(Error::new_msg(ErrorKind::IncompatibleIngestionConfigChange, "incompatible change to ingestion config"))
                .with_context(|| format!("flow(s) with name '{}' exist but their channel configs do not match what the user specified", user_flow.name))
                .help("Did you modify an existing flow? Try updating the the flow's name or the 'client_key' of `sift_stream::IngestionConfigForm`");
        } else if num_exact_matches == 0 {
            return Err(Error::new_msg(ErrorKind::IncompatibleIngestionConfigChange, "incompatible change to ingestion config"))
                .with_context(|| format!("flow(s) with name '{}' not found in Sift", user_flow.name))
                .help("try creating a new ingestion config by providing a new 'client_key' to `sift_stream::IngestionConfigForm` and notify Sift");
        }
    }
    Ok(())
}
