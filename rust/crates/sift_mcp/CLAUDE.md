# sift_mcp — guidance for Claude

## Writing tool descriptions

Tool descriptions in this crate are read by other agents at call time. They are the *only* documentation the calling LLM gets, so optimize for an agent making a decision under context pressure, not for a human reading the source.

### Structure

Use this section ordering. Skip a section only when it has no content; do not reorder.

1. **One-line purpose.** First sentence states what the tool does and where the output goes ("Retrieve X and write to Y", "List Z filtered by W"). The agent should be able to match intent from this line alone.
2. **Output schema.** When the tool returns structured data or writes a file, describe the shape — column names, types, what null means, where metadata lives. The agent will consume this output; don't make it guess.
3. **Parameters.** One bullet per parameter, in declaration order. Spell out:
   - Whether matching is exact or pattern-based.
   - Conditional requirements ("required when X is omitted").
   - Sentinel values and their meaning (e.g. `sample_ms = 0` → raw samples).
   - Mutually exclusive choices (e.g. `Names` vs `Regex` variants).
   - Side effects on the filesystem or external state (truncate mode, idempotency).
4. **Errors.** Name the actual `ErrorData` variants the tool returns (`RESOURCE_NOT_FOUND`, `INVALID_PARAMS`, etc.) and the condition that triggers each. The agent can then recover with different inputs instead of treating every failure as terminal.
5. **Guidance.** Performance characteristics, recommended call patterns, when to chunk, when to prefer one parameter shape over another. Keep this to load-bearing advice — the agent doesn't need general SQL/Arrow background.

### Style rules

- Write in direct voice. "Retrieve …", not "This tool retrieves …".
- Use backticks for parameter names, enum variants, and field names so they survive Markdown rendering on the client.
- Escape inner double quotes as `\"` — the description is a Rust string literal inside the `#[tool(...)]` attribute.
- Prefer bullets over paragraphs for multi-fact sections (output schema, parameters, errors). Paragraphs hide structure.
- Don't restate the obvious from the type signature. The parameter's name and type already tell the agent it's a `String` or `Option<i64>`; the description adds what isn't in the type — semantics, constraints, defaults.
- No marketing or filler ("powerful", "easy to use"). Every line should change what the agent does.
- Cap length around 30–40 lines. Beyond that, agents start truncating mentally; trim the guidance section first.

### Reference

`tool/data/mod.rs::get_data` is the canonical example. Mirror its layout when adding a new tool.

### list_router tools — sourcing from protos

Tools in `tool/list/` (`list_assets`, `list_runs`, `list_channels`, etc.) are thin wrappers over `sift_rs::<service>::<version>::List<Resource>Request`. Their parameters and per-parameter semantics MUST be derived from the proto comments on that message, not invented.

When you add or update a list-router tool:

1. **Open the matching proto.** Path pattern: `protos/sift/<service>/<version>/<service>.proto`. Find the `message List<Resource>Request { ... }` block. Examples:
   - `list_assets` → `protos/sift/assets/v1/assets.proto::ListAssetsRequest`
   - `list_runs` → `protos/sift/runs/v2/runs.proto::ListRunsRequest`
   - `list_channels` → `protos/sift/channels/v3/channels.proto::ListChannelsRequest`
2. **Copy the field comments verbatim into the tool description.** The proto authors curate the filterable/orderable field lists, default sort, page-size caps, and metadata syntax. Re-stating those in your own words risks drift; quoting from the proto keeps the tool spec aligned with the API.
3. **Map every wrapped field to a bullet** under the `Parameters:` section of the description, using the structure in `### Structure` above. Include:
   - Filter: list every filterable field named in the proto's `filter` comment. Preserve metadata-key syntax notes (`metadata.{key}`) and CEL helper notes (`duration(...)`).
   - Order-by: list every orderable field, the default sort if the field is empty (assets/runs default to `created_date desc`; channels defaults to `created_date` ascending — these differ, do not assume), and the `\"FIELD_NAME[ desc],...\"` format.
   - Limit: describe the `1..=1000` cap behavior of `service::common::paging` (different from the proto's raw `page_size`, which caps higher for some services).
4. **Re-read the proto whenever the resource changes.** If a new filterable or orderable field is added to the proto, update the tool description in the same change. Stale descriptions are worse than missing ones because agents will trust them.

If the proto's comments are themselves wrong or incomplete, fix the proto first and regenerate — the tool description is downstream of it.
