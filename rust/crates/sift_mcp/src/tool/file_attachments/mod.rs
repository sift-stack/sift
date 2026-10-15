use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::CallToolResult,
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::remote_files::AttachmentEntityType,
    tool::common::{list_body, to_values},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListFileAttachmentsParams {
    entity_ids: Option<Vec<String>>,
    entity_type: Option<String>,
    filter: String,
    limit: Option<u32>,
    fields: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DownloadFileAttachmentParams {
    remote_file_id: String,
}

/// Accepts the singular name, the plural the backend filters on, or the proto
/// name rows carry (`ENTITY_TYPE_RUN`), in any case.
fn parse_entity_type(value: Option<String>) -> Result<Option<AttachmentEntityType>, ErrorData> {
    let Some(value) = value else {
        return Ok(None);
    };
    let lowered = value.trim().to_ascii_lowercase();
    if lowered.is_empty() {
        return Ok(None);
    }
    let name = lowered.strip_prefix("entity_type_").unwrap_or(&lowered);
    AttachmentEntityType::ALL
        .into_iter()
        .find(|entity_type| name == entity_type.name() || name == entity_type.filter_value())
        .map(Some)
        .ok_or_else(|| {
            let expected = AttachmentEntityType::ALL
                .map(|entity_type| format!("`{}`", entity_type.name()))
                .join(", ");
            ErrorData::invalid_params(
                format!("unknown `entity_type` `{value}`; expected one of {expected}"),
                None,
            )
        })
}

/// An empty array is an error, not "no scope": it usually means an earlier
/// lookup found nothing, and listing every attachment in the organization
/// would answer a different question.
fn parse_entity_ids(entity_ids: Option<Vec<String>>) -> Result<Vec<String>, ErrorData> {
    let Some(entity_ids) = entity_ids else {
        return Ok(Vec::new());
    };
    if entity_ids.is_empty() {
        return Err(ErrorData::invalid_params(
            "`entity_ids` is empty. Resolve at least one run or asset id first, or omit \
             `entity_ids` only when the user wants files across every entity.",
            None,
        ));
    }
    if entity_ids.iter().any(|id| id.trim().is_empty()) {
        return Err(ErrorData::invalid_params(
            "`entity_ids` must not contain an empty id",
            None,
        ));
    }
    Ok(entity_ids)
}

#[tool_router(router = file_attachments_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_file_attachments",
        description = "
            List the files attached to runs, assets, annotations and their comments, test reports, and test
            steps.

            Each row is one file's metadata. Bytes are not returned; call `download_file_attachment` with a
            row's `remote_file_id` for a download URL. Artifact files are not attachments and never appear
            here; use the artifact tools for those.

            Output:
              - `{ \"file_attachments\": [RemoteFile, ...] }`. Each item includes `remote_file_id`,
                `entity_id`, `entity_type`, `file_name`, `file_mime_type`, `file_size` (bytes, as a
                string), `description`, `metadata_values`, `remote_file_type`, `created_by_user_id`, and
                `created_date`. `remote_file_type` is `REMOTE_FILE_TYPE_TELEMETRY` for the source file a
                data import read. Rows come back sorted by file name.
              - `count`: how many items THIS response carries — read it instead of
                counting the array yourself. It is the size of the page you got back, not
                how many files match.
              - `has_more`: `true` when the service hit `limit` with matches left over, so
                this page is not the whole set. Never report `count` as a total while
                `has_more` is `true` — raise `limit` or narrow the scope and ask again. `count` can be
                below `limit` while `has_more` is `true`, because the page drops files the caller can't view.

            Parameters:
              - `entity_ids`: optional ids of the runs, assets, or other entities whose files to list.
                Resolve a run or asset by name with `list_runs` or `list_assets` first. Must not be an
                empty array; omit it to list across every entity.
              - `entity_type`: optional; `run`, `asset`, `annotation`, `annotation_log` (a comment on an
                annotation), `test_report`, or `test_step`. Scope by type here, never in `filter`: the
                filter takes a different spelling from the `entity_type` rows carry and rejects theirs.
              - `filter`: CEL expression ANDed with the scope above. Pass an empty string for none.
                Filterable fields are `remote_file_id`, `entity_id`, `file_name`, and `metadata[\"<key>\"]`.
                When filtering or searching, use `file_name.matches(\"(?i)flight\")`, not `==`. Use `==` only
                for an exact value from a prior result. `contains`/`startsWith`/`endsWith` are case-SENSITIVE:
                `contains(\"Flight\")` silently misses `flight_log.csv`. `.` is a regex wildcard, so use
                `contains` to match a full literal file name.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `fields`: optional array of field names to keep on each item, e.g.
                `[\"file_name\"]`. Omit it for the full object. Names match case-insensitively
                and ignore underscores and hyphens, so `remote_file_id`, `remoteFileId` and
                `remote-file-id` all work. Any name that matched nothing on any returned item
                is listed in `unmatched_fields`; an empty page reports none.

            Errors:
              - `INVALID_PARAMS` for an unknown `entity_type`, an empty `entity_ids` array or id, or an
                invalid `filter`, including an `entity_type` value copied from a row into it.
              - `INTERNAL_ERROR` for upstream failures.
        ",
        annotations(
            title = "file_attachments/list_file_attachments",
            read_only_hint = true
        )
    )]
    pub async fn list_file_attachments(
        &self,
        params: Parameters<ListFileAttachmentsParams>,
    ) -> error::McpResult {
        let Parameters(ListFileAttachmentsParams {
            entity_ids,
            entity_type,
            filter,
            limit,
            fields,
        }) = params;

        let entity_ids = parse_entity_ids(entity_ids)?;
        let entity_type = parse_entity_type(entity_type)?;

        let page = self
            .remote_file_service
            .list_attachments(entity_ids, entity_type, filter, limit)
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(list_body(
            "file_attachments",
            to_values(&page.items)?,
            fields,
            page.has_more,
        )))
    }

    #[tool(
        name = "download_file_attachment",
        description = "
            Get a signed download URL for one file from `list_file_attachments`.

            Output:
              - `{ \"remote_file_id\": string, \"download_url\": string }`. `download_url` is a short-lived
                signed URL that needs no Sift credentials, so any HTTP client can fetch the file from it.

            Parameters:
              - `remote_file_id`: required, from a `list_file_attachments` row.

            Errors:
              - `INVALID_PARAMS` if `remote_file_id` is empty.
              - `RESOURCE_NOT_FOUND` if the file does not exist.
              - `INTERNAL_ERROR` for upstream failures, including a stop signal when the caller cannot view
                the file.

            Guidance:
              - Fetch `download_url` yourself when you need the file's contents.
              - When the user wants the file, give them `download_url` and tell them it expires soon. Call
                this tool again for a fresh URL rather than reusing an old one.
              - For an artifact's file, call `download_artifact` instead.
        ",
        annotations(
            title = "file_attachments/download_file_attachment",
            read_only_hint = true
        )
    )]
    pub async fn download_file_attachment(
        &self,
        params: Parameters<DownloadFileAttachmentParams>,
    ) -> error::McpResult {
        let Parameters(DownloadFileAttachmentParams { remote_file_id }) = params;
        let remote_file_id = remote_file_id.trim().to_owned();
        if remote_file_id.is_empty() {
            return Err(ErrorData::invalid_params(
                "`remote_file_id` must not be empty",
                None,
            ));
        }

        let download_url = self
            .remote_file_service
            .download_url(remote_file_id.clone())
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(serde_json::json!({
            "remote_file_id": remote_file_id,
            "download_url": download_url,
        })))
    }
}
