use rmcp::{
    handler::server::wrapper::Parameters,
    model::{PromptMessage, PromptMessageRole},
    prompt, prompt_router,
    schemars::{self, JsonSchema},
};
use serde::Deserialize;

use crate::server::SiftMcpServer;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExploreAssetArgs {
    asset: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AnalyzeRunArgs {
    asset: String,
    run: String,
    channels: Option<String>,
    question: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeriveAndUploadArgs {
    source_asset: String,
    source_run: String,
    transform: String,
    target_asset: Option<String>,
    target_run: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MaterializeExternalRulesArgs {
    asset: String,
    run: Option<String>,
    archive_originals: Option<bool>,
    dry_run: Option<bool>,
}

#[prompt_router(vis = "pub(crate)")]
impl SiftMcpServer {
    #[prompt(
        name = "explore_asset",
        description = "Discover a Sift asset along with its recent runs and channel inventory. Read-only starting point for a session."
    )]
    pub async fn explore_asset(&self, params: Parameters<ExploreAssetArgs>) -> Vec<PromptMessage> {
        let Parameters(ExploreAssetArgs { asset }) = params;

        let body = format!(
            "Help the user explore a Sift asset and everything recorded against it. The user referred \
             to the asset as: \"{asset}\".\n\n\
             Use the Sift MCP tools as follows:\n\
             1. Resolve the asset with `list_assets`. Try an exact match first \
             (`name == \"{asset}\"`); if nothing comes back, fall back to a substring match \
             (`name.matches(\"{asset}\")`). When several assets match, ask the user which one they \
             mean before continuing.\n\
             2. List recent runs with `list_runs` filtered by `asset_id == \"<resolved asset_id>\"`, \
             ordered `start_time desc`. Run and channel namespaces are per-asset, so always scope by \
             `asset_id` instead of listing everything.\n\
             3. List channels with `list_channels` filtered by `asset_id == \"<resolved asset_id>\"`.\n\
             4. Summarize for the user: the resolved asset (name and id), its most recent runs (name, \
             start/stop, duration), and the channel inventory grouped by data type. Surface the exact \
             run and channel names so they can be reused with `get_data`.\n\n\
             This step is discovery only. Do not pull sample data."
        );

        vec![PromptMessage::new_text(PromptMessageRole::User, body)]
    }

    #[prompt(
        name = "analyze_run",
        description = "Pull a run's channel data and produce a per-channel statistical summary. Optionally targets specific channels and answers a question."
    )]
    pub async fn analyze_run(&self, params: Parameters<AnalyzeRunArgs>) -> Vec<PromptMessage> {
        let Parameters(AnalyzeRunArgs {
            asset,
            run,
            channels,
            question,
        }) = params;

        let channels_line = match channels {
            Some(c) => format!("the following channels: {c}"),
            None => "all channels on the asset (you choose a sensible subset)".to_string(),
        };
        let question_line = match question {
            Some(q) => format!("Answer this question for the user: \"{q}\".\n"),
            None => String::new(),
        };

        let body = format!(
            "Help the user analyze a single run on a Sift asset.\n\n\
             Asset: \"{asset}\"\nRun: \"{run}\"\nChannels: {channels_line}\n\n\
             {question_line}\
             Steps:\n\
             1. Resolve the asset and run. Use `list_assets` (`name == \"{asset}\"`) for the \
             asset_id, then `list_runs` (`name == \"{run}\" && asset_id == \"...\"`) to confirm the run.\n\
             2. Confirm the target channels exist with `list_channels` scoped by `asset_id`. If no \
             channels were named, choose a sensible set and tell the user which you picked.\n\
             3. Pull data with `get_data`. Pass `run_name` so the run's start/stop bounds apply \
             automatically; do not hand-compute timestamps. Use `channel_names` with exact \
             channel names. Choose `sample_ms` to suit the run length: decimate (e.g. 100-1000 ms) \
             for long runs and use 0 only when raw fidelity is required. Write to a Parquet path in a \
             working directory.\n\
             4. Summarize with `sql` against the `get_data` output: per-channel row count, min/max/mean, \
             and null rate, plus anything needed to answer the user's question. Keep \
             `timestamp_unix_nanos` in the projection in case the result is uploaded later.\n\
             5. Report the findings and surface the Parquet paths so the work can be continued."
        );

        vec![PromptMessage::new_text(PromptMessageRole::User, body)]
    }

    #[prompt(
        name = "derive_and_upload",
        description = "Derive a new dataset from an existing run via SQL and upload it back to Sift. Confirms the write destination before uploading."
    )]
    pub async fn derive_and_upload(
        &self,
        params: Parameters<DeriveAndUploadArgs>,
    ) -> Vec<PromptMessage> {
        let Parameters(DeriveAndUploadArgs {
            source_asset,
            source_run,
            transform,
            target_asset,
            target_run,
        }) = params;

        let target_asset_line = match target_asset {
            Some(a) => format!("Target asset: \"{a}\""),
            None => {
                "Target asset: not given - propose a default and confirm with the user".to_string()
            }
        };
        let target_run_line = match target_run {
            Some(r) => format!("Target run: \"{r}\""),
            None => "Target run: not given - ask whether to create one".to_string(),
        };

        let body = format!(
            "Help the user derive a new dataset from existing Sift data and upload it back to Sift. \
             The upload is a write, so confirm the destination before running it.\n\n\
             Source asset: \"{source_asset}\"\nSource run: \"{source_run}\"\n\
             Transform: \"{transform}\"\n{target_asset_line}\n{target_run_line}\n\n\
             Steps:\n\
             1. Resolve the source asset and run with `list_assets` and `list_runs`. Identify the \
             channels the transform needs via `list_channels` scoped by `asset_id`.\n\
             2. Extract with `get_data`, passing `run_name` so the run bounds apply. Choose \
             `channel_names` and a `sample_ms` suited to the transform.\n\
             3. Apply the transform with `sql`. CRITICAL: column 0 of any dataset uploaded to Sift \
             MUST be `timestamp_unix_nanos` (Int64, non-null). Project it first in the SELECT and never \
             rename or drop it. For aggregations that collapse rows, bucket on a time expression \
             derived from `timestamp_unix_nanos` or emit `MIN(timestamp_unix_nanos)` so every output \
             row still carries a timestamp.\n\
             4. Before uploading, CONFIRM with the user: (a) the target `asset` (suggest \
             \"{source_asset}-derived\" or similar if none was given, but let them override), \
             (b) whether to create a `run_name` (required if any tags or metadata are wanted), and \
             (c) any tags or metadata to attach. Do not silently default these.\n\
             5. Upload with `upload_dataset`, passing the `sql` output as `input`. After it returns, \
             tell the user where the data landed and offer to verify via `list_runs` or \
             `list_channels`."
        );

        vec![PromptMessage::new_text(PromptMessageRole::User, body)]
    }

    #[prompt(
        name = "materialize_external_rules",
        description = "Find external rules on an asset and clone them as internal rules so they appear in reports. Optionally archive the originals and attach the clones to a report on a given run."
    )]
    pub async fn materialize_external_rules(
        &self,
        params: Parameters<MaterializeExternalRulesArgs>,
    ) -> Vec<PromptMessage> {
        let Parameters(MaterializeExternalRulesArgs {
            asset,
            run,
            archive_originals,
            dry_run,
        }) = params;

        let archive_originals = archive_originals.unwrap_or(false);
        let dry_run = dry_run.unwrap_or(false);

        let run_line = match run.as_deref() {
            Some(r) => {
                format!("Target run for the final report: \"{r}\".\n")
            }
            None => "Target run: not provided - skip the final report step.\n".to_string(),
        };

        let mode_line = if dry_run {
            "MODE: dry run. List what would be materialized. Do NOT call `create_rule` or \
             `archive_rule`."
                .to_string()
        } else if archive_originals {
            "MODE: materialize + archive originals. Confirm each write with the user before it \
             runs."
                .to_string()
        } else {
            "MODE: materialize only. Do NOT archive the external originals - the user can clean \
             them up later."
                .to_string()
        };

        let body = format!(
            "Help the user materialize external Sift rules on an asset so they appear in reports. \
             External rules (`is_external = true`) are excluded from report evaluation by default, \
             so the fix is to clone each one as an internal rule that reports can see.\n\n\
             Asset: \"{asset}\"\n{run_line}\
             {mode_line}\n\n\
             Steps:\n\
             1. Resolve the asset. Use `list_assets` with `name == \"{asset}\"`. If nothing hits, \
             fall back to a substring match (`name.matches(\"{asset}\")`). If several match, ask \
             the user which one before continuing.\n\
             2. List external rules on that asset. Call `list_rules` with the filter \
             `is_external == true && asset_id == \"<resolved_id>\"`. Show the user the list \
             (name, description, brief condition summary). If nothing comes back but the user \
             believes external rules exist, warn that the current MCP `list_rules` may filter \
             them out server-side and stop - the fix would need an `include_external` param on \
             `list_rules`.\n\
             3. Confirm with the user which rules to materialize. Default is all; the user can \
             subset. Do not silently default.\n\
             4. For each selected rule: extract its full definition (`name`, `description`, \
             `conditions`, `actions`, `asset_configuration`, `tags`, `metadata`) and call \
             `create_rule` with that content packed into `rule_json`. DROP the `client_key` from \
             the clone to avoid uniqueness collisions. The new rule defaults to \
             `is_external = false` because `create_rule` does not accept that flag. Collect the \
             (old_external_id -> new_internal_id) mapping and surface it to the user.\n\
             5. If archive-originals mode is on, confirm once more with the user, then call \
             `archive_rule` on each original external `rule_id`. Warn the user that archiving \
             stops live evaluation - if any external system references these rules by \
             `client_key`, those references now hit an inactive rule.\n\
             6. If a target run was provided, call `create_report` with `rule_ids` set to the \
             new internal ids, `run_id` set to the resolved run, and a `name` that references \
             the materialization (e.g. \"Materialized external rules for {asset}\"). Surface the \
             `report_url` so the user can open the report and confirm the previously-invisible \
             rules now show.\n\n\
             Safety:\n\
             - Every `create_rule`, `archive_rule`, and `create_report` is a write. Confirm the \
             exact list with the user before each batch.\n\
             - If any single `create_rule` fails, stop the batch, report which rule failed, and \
             do NOT proceed to archive originals - a partial clone must not be paired with an \
             archive.\n\
             - In dry-run mode, execute steps 1-3 only; skip 4-6 entirely and tell the user what \
             would happen."
        );

        vec![PromptMessage::new_text(PromptMessageRole::User, body)]
    }
}
