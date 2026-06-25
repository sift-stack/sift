use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tonic::{Code, Status};

#[cfg(test)]
mod test;

/// Percent-encode a query-parameter value (encodes all non-alphanumerics,
/// including spaces and `/`, which the server decodes).
fn encode(s: &str) -> String {
    percent_encoding::utf8_percent_encode(s, percent_encoding::NON_ALPHANUMERIC).to_string()
}

/// One search hit. The search endpoint now embeds the first page of the doc
/// (the same payload `read_doc` returns for that path) so a hit can be read
/// without a second request. Field names tolerate both the camelCase (protojson
/// default) and snake_case forms the gateway may emit; the struct serializes
/// back out in snake_case to match `read_doc`'s output.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DocHit {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub score: i32,
    /// 1-indexed line in the page where the query matched.
    #[serde(default, alias = "matchLine")]
    pub match_line: i32,
    /// Total number of lines in the page; page further with `read_doc`.
    #[serde(default, alias = "totalLines")]
    pub total_lines: i32,
    /// First page of the page's markdown, each line prefixed with `<line>\t`.
    #[serde(default)]
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchDocsResponse {
    #[serde(default)]
    pub hits: Vec<DocHit>,
    #[serde(default, alias = "total_scanned")]
    pub total_scanned: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadDocResponse {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub title: String,
    #[serde(default, alias = "total_lines")]
    pub total_lines: i32,
    #[serde(default, alias = "start_line")]
    pub start_line: i32,
    #[serde(default)]
    pub content: String,
}

/// Read-only client for Sift's documentation HTTP API (`/api/v1/docs:*`),
/// authenticated with the same bearer token used for the rest of the Sift API.
#[derive(Clone)]
pub struct DocsService {
    base_url: String,
    api_key: String,
    client: Client,
}

impl DocsService {
    pub fn new(rest_uri: String, api_key: String) -> Self {
        // Bound the request so a slow or hung docs gateway surfaces as an error
        // rather than wedging the MCP tool call indefinitely (reqwest applies no
        // timeout by default).
        let client = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("failed to build reqwest client");
        Self {
            base_url: rest_uri.trim_end_matches('/').to_string(),
            api_key,
            client,
        }
    }

    pub async fn search_docs(
        &self,
        query: String,
        max_results: Option<u32>,
    ) -> Result<SearchDocsResponse> {
        let mut url = format!(
            "{}/api/v1/docs:search?query={}",
            self.base_url,
            encode(&query)
        );
        // Enforce the documented contract here rather than trusting the gateway:
        // default to 10 hits, cap at 25.
        let max_results = max_results.unwrap_or(10).min(25);
        url.push_str(&format!("&max_results={max_results}"));

        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .context("docs search request failed")?;

        Self::decode(resp)
            .await
            .context("failed to read docs search response")
    }

    pub async fn read_doc(
        &self,
        path: String,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ReadDocResponse> {
        let mut url = format!("{}/api/v1/docs:read?path={}", self.base_url, encode(&path));
        if let Some(offset) = offset {
            url.push_str(&format!("&offset={offset}"));
        }
        if let Some(limit) = limit {
            url.push_str(&format!("&limit={limit}"));
        }

        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .context("docs read request failed")?;

        Self::decode(resp)
            .await
            .context("failed to read docs read response")
    }

    /// Map a non-success HTTP status onto a `tonic::Status` so `from_anyhow`
    /// classifies it (e.g. 400 -> INVALID_PARAMS, 404 -> RESOURCE_NOT_FOUND),
    /// then deserialize the JSON body.
    async fn decode<T: serde::de::DeserializeOwned>(resp: reqwest::Response) -> Result<T> {
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            let code = match status.as_u16() {
                400 => Code::InvalidArgument,
                404 => Code::NotFound,
                _ => Code::Internal,
            };
            return Err(Status::new(code, format!("docs API returned {status}: {body}")).into());
        }

        let value = resp
            .json::<T>()
            .await
            .context("failed to deserialize docs API response")?;
        Ok(value)
    }
}
