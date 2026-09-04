use rmcp::{
    ErrorData,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock},
    schemars::{self, JsonSchema},
    tool, tool_router,
};
use serde::Deserialize;
use sift_rs::artifacts::v1::{
    ArtifactAuthoringKind, ArtifactCreatedVia, ArtifactLinkInput, ArtifactLinkRelation,
    ArtifactStorageClass,
};

use crate::{
    error::{self, from_anyhow},
    server::SiftMcpServer,
    service::artifacts::CreateArtifactInput,
    tool::common::{MetadataEntry, list_body, to_values},
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArtifactListParams {
    conversation_id: Option<String>,
    include_archived: Option<bool>,
    filter: Option<String>,
    order_by: Option<String>,
    limit: Option<u32>,
    fields: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DownloadArtifactParams {
    artifact_id: String,
    artifact_version_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct CreateArtifactParams {
    title: Option<String>,
    summary: Option<String>,
    conversation_id: Option<String>,
    artifact_id: Option<String>,
    authoring_kind: Option<String>,
    storage_class: Option<String>,
    created_via: Option<String>,
    kind: Option<String>,
    payload: Option<serde_json::Value>,
    metadata: Option<Vec<MetadataEntry>>,
    links: Option<Vec<ArtifactLinkParam>>,
    file_path: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ArtifactLinkParam {
    relation: String,
    entity_type: String,
    entity_id: String,
}

/// Also accepts the proto enum names so an agent can echo a value it read from `list_artifacts`.
fn parse_authoring_kind(value: Option<String>) -> Result<ArtifactAuthoringKind, ErrorData> {
    let lowered = value
        .as_deref()
        .map(str::trim)
        .unwrap_or("user")
        .to_ascii_lowercase();
    match lowered.as_str() {
        "" | "user" | "artifact_authoring_kind_user" => Ok(ArtifactAuthoringKind::User),
        "agent" | "artifact_authoring_kind_agent" => Ok(ArtifactAuthoringKind::Agent),
        other => Err(ErrorData::invalid_params(
            format!("unknown `authoring_kind` `{other}`; expected `user` or `agent`"),
            None,
        )),
    }
}

fn parse_storage_class(value: Option<String>) -> Result<Option<ArtifactStorageClass>, ErrorData> {
    let Some(value) = value else {
        return Ok(None);
    };
    let lowered = value.trim().to_ascii_lowercase();
    match lowered.as_str() {
        "" => Ok(None),
        "file" | "artifact_storage_class_file" => Ok(Some(ArtifactStorageClass::File)),
        "structured" | "artifact_storage_class_structured" => {
            Ok(Some(ArtifactStorageClass::Structured))
        }
        "blob" | "artifact_storage_class_blob" => Ok(Some(ArtifactStorageClass::Blob)),
        other => Err(ErrorData::invalid_params(
            format!("unknown `storage_class` `{other}`; expected `file`, `structured`, or `blob`"),
            None,
        )),
    }
}

fn parse_created_via(value: Option<String>) -> Result<Option<ArtifactCreatedVia>, ErrorData> {
    let Some(value) = value else {
        return Ok(None);
    };
    let lowered = value.trim().to_ascii_lowercase();
    match lowered.as_str() {
        "" => Ok(None),
        "chat" | "artifact_created_via_chat" => Ok(Some(ArtifactCreatedVia::Chat)),
        "canvas" | "artifact_created_via_canvas" => Ok(Some(ArtifactCreatedVia::Canvas)),
        "sdk" | "artifact_created_via_sdk" => Ok(Some(ArtifactCreatedVia::Sdk)),
        "upload" | "artifact_created_via_upload" => Ok(Some(ArtifactCreatedVia::Upload)),
        other => Err(ErrorData::invalid_params(
            format!(
                "unknown `created_via` `{other}`; expected `chat`, `canvas`, `sdk`, or `upload`"
            ),
            None,
        )),
    }
}

fn parse_link_relation(value: String) -> Result<ArtifactLinkRelation, ErrorData> {
    let lowered = value.trim().to_ascii_lowercase();
    match lowered.as_str() {
        "attached_to" | "artifact_link_relation_attached_to" => {
            Ok(ArtifactLinkRelation::AttachedTo)
        }
        "source" | "artifact_link_relation_source" => Ok(ArtifactLinkRelation::Source),
        "derived_from" | "artifact_link_relation_derived_from" => {
            Ok(ArtifactLinkRelation::DerivedFrom)
        }
        other => Err(ErrorData::invalid_params(
            format!(
                "unknown link `relation` `{other}`; expected `attached_to`, `source`, or `derived_from`"
            ),
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
            artifact. Bytes are not returned; use `download_artifact` for a download URL when a version has
            uploaded files.

            Use CEL `filter` to narrow results and `order_by` to choose their order. The default order is
            `created_date` ascending. Pass `order_by: \"created_date desc\"` to reach the newest artifacts.

            Output:
              - `{ \"artifacts\": [Artifact, ...] }`. Each item includes `artifact_id`, `artifact_version_id`,
                `version`, `title`, `summary`, `authoring_kind`, `storage_class`, `created_via`, `kind`,
                `payload` for structured artifacts, `metadata`, `file_name`, `file_mime_type`, `remote_file_id`,
                `created_date`, and `archived_date` when set.
              - `count`: how many items THIS response carries — read it instead of
                counting the array yourself. It is the size of the page you got back, not
                how many artifacts the caller has.
              - `has_more`: `true` when the service hit `limit` with matches left over, so
                this page is not the whole set. Never report `count` as a total while
                `has_more` is `true` — raise `limit` or scope with `conversation_id` and ask again.

            Parameters:
              - `conversation_id`: optional. When set, only artifacts linked to that conversation. When omitted,
                every artifact in the caller's organization.
              - `include_archived`: optional. Default `false` omits archived artifacts. Set `true` only when the
                user asks for archived ones.
              - `filter`: optional CEL expression. Omit it or pass an empty string to list everything.
                Filterable fields are `artifact_id`, `organization_id`, `created_by_user_id`, `authoring_kind`,
                `storage_class`, `created_via`, `kind`, `title`, `version`, `created_date`, `archived_date`,
                the `include_archived` directive, `metadata[\"<key>\"]`, and
                `links.exists(l, l.relation == \"ATTACHED_TO\" && l.entity_type == \"conversations\" &&
                l.entity_id == \"<id>\")`. Enum comparisons use proto value names without the prefix, such as
                `storage_class == \"STRUCTURED\"`. Use `created_by_user_id == \"<user id>\"` to narrow to
                one author.
              - `order_by`: optional comma-separated ordering over `created_date`, `archived_date`, `title`,
                `version`, and `kind`. Fields sort ascending by default and accept a `desc` suffix. The default
                is `created_date` ascending.
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
              - `RESOURCE_NOT_FOUND` if `conversation_id` does not exist or is not visible to the caller.
              - `INTERNAL_ERROR` for upstream failures.

            Guidance:
              - Prefer scoping by `conversation_id` when the user is talking about one chat.
              - Use `order_by: \"created_date desc\"` when the newest artifacts matter.
        ",
        annotations(title = "artifacts/list_artifacts", read_only_hint = true)
    )]
    pub async fn list_artifacts(&self, params: Parameters<ArtifactListParams>) -> error::McpResult {
        let Parameters(ArtifactListParams {
            conversation_id,
            include_archived,
            filter,
            order_by,
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
            .list_artifacts(
                conversation_id,
                include_archived.unwrap_or(false),
                filter.unwrap_or_default(),
                order_by,
                limit,
            )
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
        name = "download_artifact",
        description = "
            Get one artifact by `artifact_id`, resolved to the latest version unless `artifact_version_id` pins one.

            Output:
              - `{ \"artifact\": Artifact }`. Same chrome as `list_artifacts`, plus `download_url` when the
                version has uploaded bytes (`remote_file_id` is set). `download_url` is a short-lived signed
                URL; fetch it only when the user needs the body. Structured artifacts carry their JSON `payload`
                directly. When `remote_file_id` is absent, `download_url` is omitted.

            Parameters:
              - `artifact_id`: required stable container id.
              - `artifact_version_id`: optional pin. Omit to get the latest version.

            Errors:
              - `INVALID_PARAMS` if `artifact_id` is empty, or `artifact_version_id` is empty when set.
              - `RESOURCE_NOT_FOUND` if the artifact (or pinned version) does not exist or is not visible to the caller.
              - `INTERNAL_ERROR` for upstream failures, including a failure to mint the download URL for a
                version that has uploaded bytes. The tool does not return a partial artifact in that case.

            Guidance:
              - Use this when you need a specific version or a download URL. Use `list_artifacts` to discover ids.
        ",
        annotations(title = "artifacts/download_artifact", read_only_hint = true)
    )]
    pub async fn download_artifact(
        &self,
        params: Parameters<DownloadArtifactParams>,
    ) -> error::McpResult {
        let Parameters(DownloadArtifactParams {
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
            .download_artifact(artifact_id, artifact_version_id)
            .await
            .map_err(from_anyhow)?;

        Ok(CallToolResult::structured(
            serde_json::json!({ "artifact": artifact }),
        ))
    }

    #[tool(
        name = "create_artifact",
        description = "
            Create a new artifact, or append a version to an existing one. It can carry a local file or a
            structured JSON payload.

            Output:
              - `{ \"artifact\": Artifact, \"next_step\": string }`. The returned artifact is the created or
                appended version, including `artifact_id`, `artifact_version_id`, and `version`. When a
                file was uploaded it also carries `file_name`, `file_mime_type`, `remote_file_id`, and a
                short-lived signed `download_url`.

            Parameters:
              - `title`: optional display title stored on the version.
              - `summary`: optional short description stored on the version.
              - `conversation_id`: optional. Legal only when creating a new artifact (not when appending).
                Links the new artifact to that conversation. The caller must be the conversation's author.
              - `artifact_id`: optional. Set to append a new version to an existing artifact. Omit to create
                a new artifact. `conversation_id` must be omitted when this is set.
              - `authoring_kind`: optional; `user` (default) or `agent`, matched case-insensitively. The
                proto names that `list_artifacts` / `download_artifact` emit (`ARTIFACT_AUTHORING_KIND_USER`,
                `ARTIFACT_AUTHORING_KIND_AGENT`) are also accepted. Use `agent` when a Sift agent is
                producing the artifact during a turn.
              - `storage_class`: optional; `file` (default), `structured`, or `blob`, matched case-insensitively.
                Proto names are also accepted. `structured` requires `payload` and rejects `file_path`. `file`
                and `blob` reject `payload`.
              - `created_via`: optional; `chat`, `canvas`, `sdk` (default), or `upload`, matched
                case-insensitively. Proto names are also accepted.
              - `kind`: optional semantic type label, such as `markdown`, `table`, or `psd`.
              - `payload`: optional JSON object. Required for `storage_class: \"structured\"`; rejected for
                `file` and `blob`. Its serialized form must not exceed 1 MiB.
              - When appending, omit `storage_class`, `created_via`, and `kind` unless you intend to assert they
                match the existing artifact. Appending a JSON payload requires `storage_class: \"structured\"`;
                it must match the existing artifact and lets local validation accept the payload.
              - `metadata`: optional list of `{ \"name\": \"<key>\", \"value\": <scalar> }` entries.
              - `links`: optional list of `{ \"relation\", \"entity_type\", \"entity_id\" }` entries. `relation`
                accepts `attached_to`, `source`, or `derived_from`, plus proto names. `entity_type` and
                `entity_id` must not be empty.
              - `file_path`: optional absolute or relative path of a local file to upload as this
                version's content. The file streams to Sift's file store; its name and extension drive
                the mime type and how the UI previews it. Regular, non-empty files up to 1 GiB.
                Omit it to record metadata only (content can not be attached later to the same version).

            Access:
              - Creating a new artifact needs `--allow-create`.
              - Appending a version to an existing artifact changes what every linked conversation
                resolves to, so it needs `--allow-destructive`.

            Errors:
              - `INVALID_PARAMS` for unknown enum values, invalid storage/payload combinations, a non-object
                `payload`, an empty link entity field, if `conversation_id` is set while appending, or if
                `artifact_id` / `conversation_id` / `file_path` is empty when set.
              - `INVALID_REQUEST` if the server was launched without the flag the call needs (see Access).
              - `RESOURCE_NOT_FOUND` if the conversation or existing artifact is not visible to the caller.
              - `INTERNAL_ERROR` for upstream failures. When the message says the artifact was created but
                the upload failed, the version exists without content — report that to the user instead of
                calling `create_artifact` again, which would mint a duplicate.

            Guidance:
              - This is a write. CONFIRM the title and destination conversation with the user before invoking.
              - Edits always create a new version; there is no edit-in-place path.
              - Use `storage_class: \"structured\"` for computed tables and PSD-like results. Use `blob` for
                opaque intermediates.
              - Set `created_via: \"chat\"` inside a Sift agent session. Use `sdk` otherwise.
              - One artifact per real deliverable. Do not create artifacts for intermediate scratch files.
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
            storage_class,
            created_via,
            kind,
            payload,
            metadata,
            links,
            file_path,
        }) = params;

        // Appending rewrites what every linked conversation resolves to, so it takes the stronger gate.
        if artifact_id.is_some() {
            self.require_destructive()?;
        }

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
        if let Some(path) = file_path.as_deref()
            && path.trim().is_empty()
        {
            return Err(ErrorData::invalid_params(
                "`file_path` must not be empty when set",
                None,
            ));
        }

        let authoring_kind = parse_authoring_kind(authoring_kind)?;
        let storage_class = parse_storage_class(storage_class)?;
        let created_via = parse_created_via(created_via)?;
        let storage_class_for_validation = storage_class.unwrap_or(ArtifactStorageClass::File);
        if storage_class_for_validation == ArtifactStorageClass::Structured && payload.is_none() {
            return Err(ErrorData::invalid_params(
                "`payload` is required when `storage_class` is `structured`",
                None,
            ));
        }
        if storage_class_for_validation == ArtifactStorageClass::Structured && file_path.is_some() {
            return Err(ErrorData::invalid_params(
                "`file_path` is not allowed when `storage_class` is `structured`",
                None,
            ));
        }
        if storage_class_for_validation != ArtifactStorageClass::Structured && payload.is_some() {
            return Err(ErrorData::invalid_params(
                "`payload` is allowed only when `storage_class` is `structured`",
                None,
            ));
        }
        let payload = payload
            .map(|value| {
                if !value.is_object() {
                    return Err(ErrorData::invalid_params(
                        "`payload` must be a JSON object",
                        None,
                    ));
                }
                serde_json::from_value(value).map_err(|error| {
                    ErrorData::invalid_params(format!("invalid `payload`: {error}"), None)
                })
            })
            .transpose()?;
        let links = links
            .unwrap_or_default()
            .into_iter()
            .map(|link| {
                if link.entity_type.trim().is_empty() || link.entity_id.trim().is_empty() {
                    return Err(ErrorData::invalid_params(
                        "link `entity_type` and `entity_id` must not be empty",
                        None,
                    ));
                }
                Ok(ArtifactLinkInput {
                    relation: parse_link_relation(link.relation)? as i32,
                    entity_type: link.entity_type,
                    entity_id: link.entity_id,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let appending = artifact_id.is_some();
        let uploaded = file_path.is_some();
        let artifact = self
            .artifact_service
            .create_artifact(
                CreateArtifactInput {
                    title,
                    summary,
                    conversation_id,
                    artifact_id,
                    authoring_kind,
                    storage_class,
                    created_via,
                    kind,
                    payload,
                    metadata: metadata
                        .unwrap_or_default()
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    links,
                },
                file_path.as_deref().map(std::path::Path::new),
            )
            .await
            .map_err(from_anyhow)?;

        // The refresh and download-link steps after an upload are best-effort, so
        // say only what the returned artifact actually carries.
        let content_note = if uploaded && artifact.download_url.is_some() {
            " Its file content was uploaded and the user can preview and download it."
        } else if uploaded {
            " Its file content was uploaded, but the refreshed artifact or its download link \
             could not be fetched; call `download_artifact` for the link."
        } else if storage_class_for_validation == ArtifactStorageClass::Structured {
            " It carries its JSON payload."
        } else {
            " It has no file content; the user has nothing to preview or download."
        };
        let next_step = if appending {
            format!(
                "Appended version {} to artifact {}.{content_note} Surface the new version to the user \
                 and confirm it matches their intent.",
                artifact.inner.version, artifact.inner.artifact_id
            )
        } else {
            format!(
                "Created artifact {} version {}.{content_note} Surface the title and destination to the \
                 user and confirm they match their intent before further edits.",
                artifact.inner.artifact_id, artifact.inner.version
            )
        };
        let mut result = CallToolResult::structured(serde_json::json!({
            "artifact": artifact,
            "next_step": next_step,
        }));
        result.content = vec![ContentBlock::text(next_step)];
        Ok(result)
    }
}
