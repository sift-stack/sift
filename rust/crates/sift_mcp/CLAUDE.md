# sift_mcp — guidance for building tools

This file is the playbook for adding a tool to the Sift MCP server. Follow it top to bottom.
The steps are ordered; do not skip ahead. Anyone driving an agent against this crate should be
able to ship a correct, tested, well-described tool by working through these steps in order.

## Orientation

The crate has two layers. Keep them separate.

- **`service/<name>/`** is the business logic. It is the only place that talks to the `sift_rs`
  gRPC clients. It owns pagination, retries, and serialization, and returns `anyhow::Result<T>`
  of plain domain types. Services are unit-tested with mocks.
- **`tool/<group>/`** is the transport layer. Keep it thin. A tool validates its flat input
  parameters, calls one or more services, shapes the result into JSON, attaches `next_step`
  guidance for the calling agent, maps errors with `from_anyhow`, and returns `error::McpResult`.
  "Thin" means the tool holds no business logic of its own, not that it calls a single service.
  For tasks that span several services, see "Orchestrating multiple services" under Step 4.

Where things live:

| Path | Purpose |
| --- | --- |
| `src/service/<name>/mod.rs` | Service logic; one module per Sift resource/domain. |
| `src/service/<name>/test.rs` | Service unit tests (required). |
| `src/service/common/mod.rs` | Shared helpers: `paging`, timestamp conversion, `ColumnName`. |
| `src/tool/<group>/mod.rs` | Tool handlers, grouped into one `#[tool_router]` block. |
| `src/server/mod.rs` | `SiftMcpServer` struct; service construction and router merging. |
| `src/error/mod.rs` | gRPC-to-MCP error mapping and agent guidance (`from_anyhow`). |
| `src/policy/mod.rs` | `RetryPolicy` and `with_retry`. |
| `src/prompt/mod.rs` | Multi-step workflow prompts. |
| `rust/crates/sift_test_util/src/` | gRPC test channel and `mockall` service mocks. |

Non-negotiables. These hold regardless of context pressure:

1. Tool parameters are flat: scalars, arrays of scalars, or `Option`s of either. No nested
   objects or enums. (See "Keep parameters flat" below for why.)
2. Every service function gets a test. Testing tools and other code is encouraged.
3. Do not write retry logic anywhere except `policy::with_retry`. Call it; don't reinvent it.
4. Set `read_only_hint` honestly. Read tools are `true`; anything that writes is `false`.
5. Do not mirror the API one-to-one. Run Step 0 before writing anything.

---

## Step 0 — Decide the tool surface (anti-bloat gate)

A good MCP server is not a 1:1 wrapper of the gRPC API. Map tools to what a user wants to *do*,
not to individual RPCs. Fewer, task-shaped tools beat many thin ones: they keep the agent's tool
list legible and reduce wrong-tool selection.

Apply these rules before writing any code:

- **Prefer one `list_*` tool over `Get*` + `List*`.** A list with a filter subsumes a get
  (filter by id). Example: `protos/sift/webhooks/v1/webhooks.proto` exposes both `GetWebhook`
  and `ListWebhooks`. Build a `list_webhooks` tool over `ListWebhooks` only; skip `GetWebhook`.
- **Skip RPCs an agent has no conversational use for.** Internal, admin, batch, and plumbing
  RPCs (for example `BatchCreateWebhookLogs`, signature-key rotation) stay out unless the user
  explicitly asks for them.
- **Collapse a multi-RPC task into one tool.** When the user's intent is a single task, do the
  whole task in one tool rather than exposing each RPC. `get_data` (`tool/data/mod.rs`) is the
  model: one tool resolves the asset, run, and channels, then pulls data. A task that spans
  several services is still one tool; see "Orchestrating multiple services" under Step 4.
- **Default writes and destructive operations to OFF.** Add them only when the user explicitly
  asks. A write tool must set `read_only_hint = false` and use its `next_step` to make the agent
  confirm the destination with the user before acting. See the `sql` and `upload_dataset`
  confirmation pattern in `tool/data/mod.rs`.

Output of this step: a short list of the tool(s) to build and which RPC(s) each one wraps. If
you cannot justify a tool against these rules, do not build it.

---

## Step 1 — Find the proto and the generated client

- **Proto:** `protos/sift/<service>/<version>/<service>.proto`. Read the message comments
  first. They are the source of truth for filterable and orderable fields, default sort, and
  page-size caps. Step 5 depends on them.
- **Generated client:** `sift_rs::<service>::<version>::<svc>_service_client::<Svc>ServiceClient`.
  Request and response types live in `sift_rs::<service>::<version>`. Example for assets:
  `sift_rs::assets::v1::{Asset, ListAssetsRequest, ListAssetsResponse, asset_service_client::AssetServiceClient}`.

---

## Step 2 — Write the service

Create `src/service/<name>/mod.rs` and declare it in `src/service/mod.rs` (`pub mod <name>;`).

The struct holds a `SiftChannel` and a `RetryPolicy`, derives `Clone`, and has a
`new(channel, policy)`. `service/assets/mod.rs` is the canonical paginated example;
`service/ping/mod.rs` is the canonical simple (non-paginated) one.

Rules for the call itself:

- Construct the client **inside** the `with_retry` closure (`<Svc>ServiceClient::new(channel)`).
  A retry needs a fresh client, so clone every captured value per attempt.
- Finish the RPC with `.await.map(|resp| resp.into_inner())`.
- Wrap the whole call in `with_retry(&self.policy, ...)` and add `.context("...")`.
- For `List` RPCs, page with `common::paging(limit)`, which returns `(page_size, record_limit)`.
  Loop until the page is empty or `next_page_token` is empty or you hit `record_limit`, then
  `results.truncate(record_limit)`.

Paginated skeleton (mirror `service/assets/mod.rs::list_assets`):

```rust
#[derive(Clone)]
pub struct WebhookService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl WebhookService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    pub async fn list_webhooks(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
    ) -> Result<Vec<Webhook>> {
        let (page_size, record_limit) = common::paging(limit);
        let order_by = order_by.unwrap_or_default();
        let mut page_token = String::new();
        let mut results = Vec::new();

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
                    let mut client = WebhookServiceClient::new(channel);
                    client
                        .list_webhooks(ListWebhooksRequest {
                            filter,
                            page_size,
                            page_token: token,
                            order_by,
                        })
                        .await
                        .map(|resp| resp.into_inner())
                }
            })
            .await
            .context("failed to query webhooks")?;

            let ListWebhooksResponse { webhooks, next_page_token } = resp;
            if webhooks.is_empty() {
                break;
            }
            results.extend(webhooks);
            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);
        Ok(results)
    }
}
```

For a non-paginated RPC, drop the loop and return the single response field. See
`service/ping/mod.rs`.

---

## Step 3 — Register the service and merge the router

In `src/server/mod.rs`:

1. Add the service field to `SiftMcpServer`.
2. Construct it in `new()` with `channel.clone()` and `retry_policy.clone()` (the last
   constructed service can take `retry_policy` by move).
3. Merge the tool router: `tool_router.merge(Self::<group>_router());`.

The existing `new()` shows all three insertion points:

```rust
let mut tool_router = Self::list_router();
tool_router.merge(Self::data_router());
tool_router.merge(Self::explore_router());
tool_router.merge(Self::ping_router());
// merge your new router here

let asset_service = AssetService::new(channel.clone(), retry_policy.clone());
// construct your new service here
```

If the tool belongs in an existing group (a new `list_*` tool belongs in `list_router`), add it
to that module instead of creating a new router.

---

## Step 4 — Write the tool

Create or extend `src/tool/<group>/mod.rs`. Declare new modules in `src/tool/mod.rs`.

Parameter struct — flat fields only:

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListParams {
    filter: String,
    order_by: Option<String>,
    limit: Option<u32>,
}
```

Handler — destructure, validate early, call the service(s), map errors, return structured JSON:

```rust
pub async fn list_webhooks(&self, params: Parameters<ListParams>) -> error::McpResult {
    let Parameters(ListParams { filter, order_by, limit }) = params;

    let out = self
        .webhook_service
        .list_webhooks(filter, order_by, limit)
        .await
        .map(|webhooks| serde_json::json!({ "webhooks": webhooks }))
        .map_err(from_anyhow)?;

    Ok(CallToolResult::structured(out))
}
```

- Validate before calling the service. Return `ErrorData::invalid_params(...)` or
  `ErrorData::resource_not_found(...)` for bad input. `get_data` shows multi-field validation.
- Always return `CallToolResult::structured(json!({ ... }))`.
- Annotate: `annotations(title = "<group>_router/<tool>", read_only_hint = <bool>)`.
- **`next_step`.** When the tool writes a file, opens a follow-on workflow, or performs a write,
  set both the structured `next_step` field and `result.content = vec![Content::text(next_step)]`
  so the calling agent sees it. `get_data` and `explore_url` are the reference patterns. Write
  tools use `next_step` to force a confirmation before the agent proceeds.

### Writing the description

The description is read by another agent at call time. It is the *only* documentation the
calling LLM gets, so optimize for an agent deciding under context pressure, not for a human
reading source.

Use this section order. Skip a section only when it has no content; do not reorder.

1. **One-line purpose.** First sentence states what the tool does and where the output goes
   ("Retrieve X and write to Y", "List Z filtered by W"). The agent should match intent from
   this line alone.
2. **Output schema.** When the tool returns structured data or writes a file, describe the shape:
   column names, types, what null means, where metadata lives.
3. **Parameters.** One bullet per parameter, in declaration order. Spell out:
   - Whether matching is exact or pattern-based.
   - Conditional requirements ("required when X is omitted").
   - Sentinel values and their meaning (e.g. `sample_ms = 0` is raw samples).
   - Mutually exclusive choices.
   - Side effects on the filesystem or external state (truncate mode, idempotency).
4. **Errors.** Name the actual `ErrorData` variants the tool returns (`RESOURCE_NOT_FOUND`,
   `INVALID_PARAMS`, etc.) and what triggers each, so the agent can recover with different input
   instead of treating every failure as terminal.
5. **Guidance.** Load-bearing advice only: performance characteristics, recommended call
   patterns, when to chunk, when to prefer one parameter shape over another.

Style rules:

- Direct voice. "Retrieve …", not "This tool retrieves …".
- Backtick parameter names, enum variants, and field names so they survive Markdown rendering
  on the client.
- Escape inner double quotes as `\"`. The description is a Rust string literal inside the
  `#[tool(...)]` attribute.
- Bullets over paragraphs for output schema, parameters, and errors. Paragraphs hide structure.
- Don't restate the type signature. The name and type already say it's a `String` or
  `Option<i64>`; the description adds semantics, constraints, and defaults.
- No marketing or filler. Every line should change what the agent does.
- Cap around 30–40 lines. Beyond that, agents start truncating mentally; trim guidance first.

`tool/data/mod.rs::get_data` is the canonical example. Mirror its layout.

### Keep parameters flat

Tool parameters MUST be scalars, arrays of scalars, or `Option`s of either. Do not use nested
enums or nested objects as the value of a parameter field.

Some MCP clients JSON-stringify object-typed argument values before transport. The server then
receives the value as a `Value::String` instead of `Value::Object`, and `serde_json`'s
`deserialize_enum` treats the entire JSON string as the variant name. It surfaces as
``unknown variant `{"Names":[...]}`, expected `Names` or `Regex` ``. Scalar and array params
round-trip cleanly, so the failure is specific to nested-enum/object params.

When a parameter has mutually exclusive shapes, flatten it into sibling `Option<...>` fields and
validate "exactly one is set" in the handler. Return `INVALID_PARAMS` if zero or both are set.
`tool/data/mod.rs::get_data` (`channel_names` / `channel_regex`) is the reference.

If a tagged variant is genuinely unavoidable, use `#[serde(tag = "type")]` so the discriminator
is a sibling scalar field rather than the parent key. That survives the stringification bug.

### Orchestrating multiple services

Some tools span several services. A tool that migrates a channel from one asset to another, for
example, reads with `DataService` and writes through the ingest path. "Thin" means the tool
holds no business logic, not that it calls a single service. Decide where the orchestration
lives:

- **Light, sequential fan-out** (resolve names to ids, then perform one action): orchestrate in
  the tool handler. `get_data` (`tool/data/mod.rs`) is the model. Its handler calls
  `asset_service`, `run_service`, `channel_service`, then `data_service` in sequence. Keep this
  to resolution plus a single real action; no transforms or multi-write logic in the handler.
- **Heavy orchestration** (multiple writes, data transforms, partial-failure handling, anything
  worth unit-testing as a unit): put it in a dedicated composite service method. The tool stays
  thin and calls that one method. `IngestService::upload_dataset` (`service/ingest/mod.rs`) is
  the model: a single service method that internally drives three clients
  (`IngestionConfigServiceClient` to create the config, `RunServiceClient` to create the run,
  and `IngestServiceClient::ingest_with_config_data_stream` to stream the data). For a migrate
  tool, write a `MigrateService` method that composes the read and the write rather than wiring
  several clients together inside the handler.

Cross-service rules:

- **Validate before you write.** Resolve and check every input a later step depends on (source
  exists, target exists, channels line up) before the first write, so a bad target fails fast
  instead of mid-migration.
- **These RPCs are not transactional.** A multi-step write can fail after partial progress. State
  what the tool guarantees in its description and `next_step`, and on failure report what was and
  was not written so the user or agent can recover. Order the steps so the hardest-to-undo step
  runs last.
- **Streaming writes are not retried like unary calls.** `with_retry` covers unary `Unavailable`
  failures. The streaming ingest path (`IngestService` holds only a channel, no `RetryPolicy`)
  deliberately does not wrap the stream in `with_retry`. Do not bolt naive retries onto a
  half-consumed stream; surface the failure instead.

---

## Step 5 — Source `list_*` tools from protos

Tools in `tool/list/` (`list_assets`, `list_runs`, `list_channels`, etc.) are thin wrappers over
`sift_rs::<service>::<version>::List<Resource>Request`. Their parameters and per-parameter
semantics MUST be derived from the proto comments on that message, not invented.

When you add or update a list tool:

1. **Open the matching proto** and find the `message List<Resource>Request { ... }` block:
   - `list_assets` → `protos/sift/assets/v1/assets.proto::ListAssetsRequest`
   - `list_runs` → `protos/sift/runs/v2/runs.proto::ListRunsRequest`
   - `list_channels` → `protos/sift/channels/v3/channels.proto::ListChannelsRequest`
2. **Copy the field comments into the description.** The proto authors curate the filterable and
   orderable field lists, default sort, page-size caps, and metadata syntax. Restating those in
   your own words risks drift; quoting from the proto keeps the tool aligned with the API.
3. **Map every wrapped field to a bullet** under `Parameters:`, following Step 4's structure:
   - Filter: list every filterable field named in the proto's `filter` comment. Preserve
     metadata-key syntax (`metadata.{key}`) and CEL helpers (`duration(...)`).
   - Order-by: list every orderable field, the default sort when the field is empty (assets and
     runs default to `created_date desc`; channels defaults to `created_date` ascending — these
     differ, do not assume), and the `\"FIELD_NAME[ desc],...\"` format.
   - Limit: describe the `1..=1000` cap of `service::common::paging`, which differs from the
     proto's raw `page_size` (it caps higher for some services).
4. **Re-read the proto whenever the resource changes.** If a filterable or orderable field is
   added to the proto, update the tool description in the same change. A stale description is
   worse than a missing one, because agents trust it.

If the proto comments are themselves wrong or incomplete, fix the proto first and regenerate.
The tool description is downstream of it.

---

## Step 6 — Test the service (required)

Every service function needs a test. Create `src/service/<name>/test.rs` and declare it with
`#[cfg(test)] mod test;` in the service `mod.rs`. `service/assets/test.rs` is the template.

Wire a mock gRPC server to an in-memory channel:

```rust
async fn service_with_mock(mock: MockWebhookServiceImpl) -> (WebhookService, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(WebhookServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (WebhookService::new(channel, RetryPolicy::default()), handle)
}
```

Set expectations with `mockall`: `.expect_<rpc>().withf(|req| ...).returning(|_| ...)`, and
`.times(n)` to assert call counts. `sift_test_util/src/mock/test.rs` is the catalog of patterns:
canned responses, request matching, dynamic responses, pagination, and error returns.

If no mock exists for your service, add one:

1. Create `rust/crates/sift_test_util/src/mock/<service>/<version>.rs` with a `mock! { ... }`
   block over the generated server trait. `mock/assets/v1.rs` and `mock/data/v2.rs` are
   templates: list every RPC on the service trait.
2. Declare the module in `rust/crates/sift_test_util/src/mock/mod.rs` (`pub mod <service>;`) and,
   if needed, the version in the service-level mock module.

Minimum coverage to write for a list service:

- A single page returns its rows.
- Pagination across multiple `next_page_token` values accumulates correctly.
- `limit` truncates the result set.
- A gRPC error (e.g. `Status::not_found`) propagates.

For a composite service or multi-service tool, register every mock on one in-memory server with
repeated `.add_service(...)` calls on the same `Server::builder()`, then build the service (or
services) against the resulting channel. This exercises the real orchestration path end to end,
including the order of calls and a failure injected partway through.

---

## Step 7 — Update the onboarding docs

The MCP server ships as part of `sift-cli`, and its onboarding docs live in
`rust/crates/sift_cli/assets/docs/src/`. A new tool or prompt that is not documented does not
exist as far as users are concerned. Update the docs in the same change.

- **New or changed tool** → `agents/mcp.md`. Add a row to the "Available tools" table with the
  tool name and a one-line purpose drawn from your tool description. If the tool changes the
  typical agent flow, update the flow line beneath the table.
- **New or changed prompt** → `agents/prompts.md`. Add a `## <prompt>` section with a one-line
  summary, an argument table (`Argument` / `Required` / `Description`), and at least one
  invocation example using the `/mcp__sift__<prompt>` slash-command form. Match the format of
  the existing `explore_asset` / `analyze_run` / `derive_and_upload` sections.

These docs are mdBook source. Keep prose in direct voice, concise, and consistent with the
surrounding pages.

Parallel obligation: the agent skill files (`SKILL.md` / `AGENTS.md`) also carry the MCP tool
list. They are governed by `rust/crates/sift_cli/CLAUDE.md`; follow its lockstep rules and
update them in the same change when you add or remove a tool.

---

## Reference — errors and retries

Tools should not handle gRPC status codes themselves. Let the service return the error and let
`from_anyhow` (`error/mod.rs`) classify it. Add `.context("...")` in the service so the `Status`
is preserved in the `anyhow::Error` chain.

- **Retries.** `with_retry` retries **only** `Code::Unavailable`, per AIP-194, with exponential
  backoff plus jitter (default 3 attempts). Every other code returns immediately. Do not add
  retry logic anywhere else.
- **Agent signals.** `from_anyhow` maps codes such as `ResourceExhausted`, `PermissionDenied`,
  and `DeadlineExceeded` to a structured "stopped, do not retry in this prompt" message with
  guidance the agent can act on. Codes like `InvalidArgument` and `NotFound` map to
  `INVALID_PARAMS` and `RESOURCE_NOT_FOUND`. Returning the raw error through `from_anyhow` gives
  the agent the right recovery behavior for free.

---

## Reference — when to add a prompt

Prompts (`prompt/mod.rs`) encode multi-step, multi-tool workflows: discovery, analysis, a
derive-and-upload pipeline. A single-tool action does not need a prompt. Add one only when a
sequence of tool calls recurs and benefits from being scripted for the agent.

Add a `#[prompt(...)]` method with a flat args struct (same flatness rule as tool params) inside
the `#[prompt_router]` block, returning `Vec<PromptMessage>`. `explore_asset`, `analyze_run`,
and `derive_and_upload` are the reference implementations.

---

## Pre-merge checklist

Run through this before declaring the tool done:

- [ ] Step 0 passed: the tool maps to a user task, not just an RPC. No `Get*` tool where a
      `list_*` filter suffices. No unrequested writes.
- [ ] Parameters are flat (scalars, arrays, `Option`s). Mutually exclusive fields validated in
      the handler.
- [ ] Multi-service tasks place orchestration correctly: light fan-out in the handler, heavy
      logic in a composite service. Validation precedes writes; partial-failure behavior is
      stated in the description and `next_step`.
- [ ] Description follows the five-section structure and the style rules. `list_*` descriptions
      match the current proto comments.
- [ ] `read_only_hint` is correct. Write tools confirm the destination via `next_step`.
- [ ] Service registered in `server/mod.rs` and the router merged.
- [ ] Service tests added, covering single page, pagination, `limit`, and an error path. A mock
      was added to `sift_test_util` if one did not exist.
- [ ] Onboarding docs updated: `agents/mcp.md` for a tool, `agents/prompts.md` for a prompt. Skill
      files (`SKILL.md` / `AGENTS.md`) updated per `sift_cli/CLAUDE.md` if the tool list changed.
- [ ] `cargo build -p sift_mcp` and `cargo test -p sift_mcp` both pass.
