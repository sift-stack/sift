use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::artifacts::AuthoringKind,
    tool::common::{list_body, to_values},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArtifactListParams {
    conversation_id: Option<String>,
    include_archived: Option<bool>,
    limit: Option<u32>,
    fields: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetArtifactParams {
    artifact_id: String,
    artifact_version_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateArtifactParams {
    title: Option<String>,
    summary: Option<String>,
    conversation_id: Option<String>,
    artifact_id: Option<String>,
    authoring_kind: Option<String>,
}

fn parse_authoring_kind(value: Option<String>) -> Result<AuthoringKind, ErrorData> {
    match value.as_deref().map(str::trim).unwrap_or("user") {
        "" | "user" => Ok(AuthoringKind::User),
        "agent" => Ok(AuthoringKind::Agent),
        other => Err(ErrorData::invalid_params(
            format!("unknown `authoring_kind` `{other}`; expected `user` or `agent`"),
            None,
        )),
    }
}

#[tool_router(router = artifacts_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "list_artifacts",
        description = "
            List artifacts in the caller's organization, optionally restricted to those linked to one conversation.

            Artifacts are first-class versioned documents. Each list entry is the latest version of one
            artifact. Bytes are not returned; use `get_artifact` for a download URL when a version has
            uploaded files.

            Output:
              - `{ \"artifacts\": [Artifact, ...] }`. Each item includes `artifact_id`, `artifact_version_id`,
                `version`, `title`, `summary`, `authoring_kind`, `file_name`, `file_mime_type`, `remote_file_id`,
                `created_date`, and `archived_date` when set.
              - `count`: how many items THIS response carries — read it instead of
                counting the array yourself. It is the size of the page you got back, not
                how many artifacts exist in the organization.
              - `has_more`: `true` when the service hit `limit` with matches left over, so
                this page is not the whole set. Never report `count` as a total while
                `has_more` is `true` — raise `limit` or scope with `conversation_id` and ask again.

            Parameters:
              - `conversation_id`: optional. When set, only artifacts linked to that conversation. When omitted,
                every artifact in the caller's organization.
              - `include_archived`: optional. Default `false` omits archived artifacts. Set `true` only when the
                user asks for archived ones.
              - `limit`: max items to return. Start at 50 and only raise it if the result is capped
                and you still need more. Values are clamped to `1..=200`; omitting it defaults to 50.
              - `fields`: optional array of field names to keep on each item, e.g.
                `[\"title\"]`. Omit it for the full object. Names match case-insensitively
                and ignore underscores and hyphens, so `artifact_id`, `artifactId` and
                `artifact-id` all work. Any name that matched nothing on any returned item
                is listed in `unmatched_fields`; an empty page reports none, since it
                says nothing about whether a name was spelled right.
                Reach for this whenever you need only a few fields: full objects are wide,
                and a large listing can exceed the response size limit without it.

            Errors:
              - `INVALID_PARAMS` if `conversation_id` is empty when set.
              - `RESOURCE_NOT_FOUND` if `conversation_id` does not exist in the caller's organization.
              - `INTERNAL_ERROR` for upstream failures.

            Guidance:
              - Prefer scoping by `conversation_id` when the user is talking about one chat.
              - This list has no CEL `filter`; narrow with `conversation_id` or follow up with `get_artifact`.
        ",
        annotations(title = "artifacts/list_artifacts", read_only_hint = true)
    )]
    pub async fn list_artifacts(&self, params: Parameters<ArtifactListParams>) -> error::McpResult {
        let Parameters(ArtifactListParams {
            conversation_id,
            include_archived,
            limit,
            fields,
        }) = params;

        if let Some(id) = conversation_id.as_deref()
            && id.trim().is_empty()
        {
            return Err(ErrorData::invalid_params(
                "`conversation_id` must not be empty when set",
                None,
            ));
        }

        let page = self
            .artifact_service
            .list_artifacts(conversation_id, include_archived.unwrap_or(false), limit)
            .await
            .map_err(from_anyhow)?;

        let artifacts = to_values(&page.items)?;

        Ok(CallToolResult::structured(list_body(
            "artifacts",
            artifacts,
            fields,
            page.has_more,
        )))
    }

    #[tool(
        name = "get_artifact",
        description = "
            Get one artifact by `artifact_id`, resolved to the latest version unless `artifact_version_id` pins one.

            Output:
              - `{ \"artifact\": Artifact }`. Same chrome as `list_artifacts`, plus `download_url` when the
                version has uploaded bytes. `download_url` is a short-lived signed URL; fetch it only when the
                user needs the body.

            Parameters:
              - `artifact_id`: required stable container id.
              - `artifact_version_id`: optional pin. Omit to get the latest version.

            Errors:
              - `INVALID_PARAMS` if `artifact_id` is empty, or `artifact_version_id` is empty when set.
              - `RESOURCE_NOT_FOUND` if the artifact (or pinned version) does not exist in the caller's organization.
              - `INTERNAL_ERROR` for upstream failures.

            Guidance:
              - Use this when you need a specific version or a download URL. Use `list_artifacts` to discover ids.
        ",
        annotations(title = "artifacts/get_artifact", read_only_hint = true)
    )]
    pub async fn get_artifact(&self, params: Parameters<GetArtifactParams>) -> error::McpResult {
        let Parameters(GetArtifactParams {
            artifact_id,
            artifact_version_id,
        }) = params;

        if artifact_id.trim().is_empty() {
            return Err(ErrorData::invalid_params(
                "`artifact_id` must not be empty",
                None,
            ));
        }
        if let Some(id) = artifact_version_id.as_deref()
            && id.trim().is_empty()
        {
            return Err(ErrorData::invalid_params(
                "`artifact_version_id` must not be empty when set",
                None,
            ));
        }

        let artifact = self
            .artifact_service
            .get_artifact(artifact_id, artifact_version_id)
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "artifact": artifact }),
        ))
    }

    #[tool(
        name = "create_artifact",
        description = "
            Create a new artifact, or append a version to an existing one. This writes artifact metadata
            only; version bytes live in remote_files and are not uploaded by this tool.

            Output:
              - `{ \"artifact\": Artifact, \"next_step\": string }`. The returned artifact is the created or
                appended version, including `artifact_id`, `artifact_version_id`, and `version`.

            Parameters:
              - `title`: optional display title stored on the version.
              - `summary`: optional short description stored on the version.
              - `conversation_id`: optional. Legal only when creating a new artifact (not when appending).
                Links the new artifact to that conversation. The caller must be the conversation's author.
              - `artifact_id`: optional. Set to append a new version to an existing artifact. Omit to create
                a new artifact. `conversation_id` must be omitted when this is set.
              - `authoring_kind`: optional; `user` (default) or `agent`. Use `agent` when a Sift agent is
                producing the artifact during a turn.

            Errors:
              - `INVALID_PARAMS` if `authoring_kind` is not `user` or `agent`, if `conversation_id` is set
                while appending, or if `artifact_id` / `conversation_id` is empty when set.
              - `RESOURCE_NOT_FOUND` if the conversation or existing artifact is not visible to the caller.
              - `INTERNAL_ERROR` for upstream failures.

            Guidance:
              - This is a write. CONFIRM the title and destination conversation with the user before invoking.
              - Edits always create a new version; there is no edit-in-place path.
        ",
        annotations(
            title = "artifacts/create_artifact",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
        )
    )]
    pub async fn create_artifact(
        &self,
        params: Parameters<CreateArtifactParams>,
    ) -> error::McpResult {
        self.require_create()?;

        let Parameters(CreateArtifactParams {
            title,
            summary,
            conversation_id,
            artifact_id,
            authoring_kind,
        }) = params;

        if let Some(id) = artifact_id.as_deref()
            && id.trim().is_empty()
        {
            return Err(ErrorData::invalid_params(
                "`artifact_id` must not be empty when set",
                None,
            ));
        }
        if let Some(id) = conversation_id.as_deref()
            && id.trim().is_empty()
        {
            return Err(ErrorData::invalid_params(
                "`conversation_id` must not be empty when set",
                None,
            ));
        }
        if artifact_id.is_some() && conversation_id.is_some() {
            return Err(ErrorData::invalid_params(
                "`conversation_id` is legal only when creating an artifact; omit it when appending a version",
                None,
            ));
        }

        let authoring_kind = parse_authoring_kind(authoring_kind)?;
        let artifact = self
            .artifact_service
            .create_artifact(title, summary, conversation_id, artifact_id, authoring_kind)
            .await
            .map_err(from_anyhow)?;

        let next_step = format!(
            "Created artifact {} version {}. Confirm the title and destination with the user before further edits.",
            artifact.inner.artifact_id, artifact.inner.version
        );
        let mut result = CallToolResult::structured(serde_json::json!({
            "artifact": artifact,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}
