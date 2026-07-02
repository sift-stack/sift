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
};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchDocsParams {
    /// Keyword query to search the docs (search mode), e.g. `"asset channels CEL"`.
    /// Provide EITHER `query` or `path`, never both.
    pub query: Option<String>,
    /// Doc path from a prior search hit (read mode), e.g.
    /// `"api/reference/protocol-buffers/assets.mdx"`.
    pub path: Option<String>,
    /// Search mode only: cap on hits. Defaults to 10; values above 25 are coerced to 25.
    pub max_results: Option<u32>,
    /// Read mode only: 1-indexed start line. Defaults to 1.
    pub index: Option<u32>,
    /// Read mode only: number of lines to return. Defaults to all remaining lines.
    pub lines: Option<u32>,
}

#[tool_router(router = docs_router, vis = "pub(crate)")]
impl SiftMcpServer {
    #[tool(
        name = "search_docs",
        description = "
            Search and read Sift's product documentation (docs.siftstack.com), including the REST/gRPC API
            reference. One tool, two modes — the entry point for any question about how Sift works, the API,
            ingestion, channels, rules, CEL, calculated channels, or user-defined functions. Search first;
            each hit already carries the start of the page, so a top hit is usually answerable without a
            follow-up read.

            Provide EXACTLY ONE of `query` or `path`:

            SEARCH MODE — pass `query` (keywords). Output:
              - `{ \"mode\": \"search\", \"hits\": [DocHit, ...], \"total_scanned\": <int> }`. Each `DocHit` is
                `{ \"path\", \"title\", \"score\", \"match_line\", \"total_lines\", \"content\" }`. `content` is
                the first page of that doc (the same payload READ MODE returns), each line prefixed with
                `<line_number>\\t`; `match_line` is the line the `query` matched; `total_lines` is the full
                page length. `score` ranks relevance (higher is better). Use READ MODE with the hit's `path`
                only to page past `content` when `total_lines` exceeds what `content` covers. `max_results`
                caps hits (default 10, max 25).

            READ MODE — pass `path` (a path from a search hit, e.g.
            `\"api/reference/protocol-buffers/assets.mdx\"`). Output:
              - `{ \"mode\": \"read\", \"path\", \"title\", \"total_lines\", \"start_line\", \"content\" }`.
                `content` is the page markdown with each line prefixed by `<line_number>\\t`. For long pages,
                page through with `index` (1-indexed start line) and `lines` (count) using `total_lines`.

            Errors:
              - `INVALID_PARAMS` if both `query` and `path` are set, if neither is set, or if the docs
                service rejects the request.
              - `RESOURCE_NOT_FOUND` if `path` does not match a doc page.
        ",
        annotations(title = "docs_router/search_docs", read_only_hint = true)
    )]
    pub async fn search_docs(&self, params: Parameters<SearchDocsParams>) -> error::McpResult {
        let Parameters(SearchDocsParams {
            query,
            path,
            max_results,
            index,
            lines,
        }) = params;

        match (query, path) {
            (Some(query), None) => {
                let out = self
                    .docs_service
                    .search_docs(query, max_results)
                    .await
                    .map(|resp| {
                        // Build snake_case hits explicitly; the generated DocHit
                        // serializes as camelCase (pbjson), but this tool's
                        // contract is snake_case, matching READ MODE.
                        let hits: Vec<_> = resp
                            .hits
                            .iter()
                            .map(|h| {
                                serde_json::json!({
                                    "path": h.path,
                                    "title": h.title,
                                    "score": h.score,
                                    "match_line": h.match_line,
                                    "total_lines": h.total_lines,
                                    "content": h.content,
                                })
                            })
                            .collect();
                        serde_json::json!({
                            "mode": "search",
                            "hits": hits,
                            "total_scanned": resp.total_scanned,
                        })
                    })
                    .map_err(from_anyhow)?;
                Ok(CallToolResult::structured(out))
            }
            (None, Some(path)) => {
                let out = self
                    .docs_service
                    .read_doc(path, index, lines)
                    .await
                    .map(|resp| {
                        serde_json::json!({
                            "mode": "read",
                            "path": resp.path,
                            "title": resp.title,
                            "total_lines": resp.total_lines,
                            "start_line": resp.start_line,
                            "content": resp.content,
                        })
                    })
                    .map_err(from_anyhow)?;
                Ok(CallToolResult::structured(out))
            }
            (Some(_), Some(_)) => Err(ErrorData::invalid_params(
                "provide exactly one of `query` (to search) or `path` (to read a page), not both",
                None,
            )),
            (None, None) => Err(ErrorData::invalid_params(
                "provide either `query` (to search the docs) or `path` (to read a page from a search hit)",
                None,
            )),
        }
    }
}
