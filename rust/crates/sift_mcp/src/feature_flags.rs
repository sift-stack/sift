use std::{collections::HashMap, time::Duration};

use anyhow::{Context, Result};
use serde::Deserialize;

const FEATURE_FLAGS_PATH: &str = "/api/v1/feature-flags/variants";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

pub(crate) static TOOL_FEATURE_FLAGS: &[(&str, &str)] = &[
    ("list_test_reports", "test-reports"),
    ("list_test_steps", "test-reports"),
    ("list_test_measurements", "test-reports"),
    ("count_test_steps", "test-reports"),
    ("count_test_measurements", "test-reports"),
    ("create_test_report", "test-reports"),
    ("append_test_measurements", "test-reports"),
];

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FeatureFlags {
    #[serde(default)]
    variants: HashMap<String, FeatureFlagVariant>,
}

#[derive(Clone, Debug, Deserialize)]
struct FeatureFlagVariant {
    #[serde(default)]
    value: String,
}

impl FeatureFlags {
    pub fn enabled(&self, flag: &str) -> bool {
        self.variants
            .get(flag)
            .is_some_and(|variant| !variant.value.is_empty() && variant.value != "off")
    }

    pub async fn fetch(rest_uri: &str, api_key: &str) -> Result<Self> {
        let endpoint = format!("{}{FEATURE_FLAGS_PATH}", rest_uri.trim_end_matches('/'));
        reqwest::Client::new()
            .get(endpoint)
            .timeout(REQUEST_TIMEOUT)
            .bearer_auth(api_key)
            .send()
            .await
            .context("feature flag request failed")?
            .error_for_status()
            .context("feature flag request returned an error status")?
            .json()
            .await
            .context("failed to parse feature flag response")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{FeatureFlagVariant, FeatureFlags};
    use crate::client_event::start_http_server;

    fn response(status: &str, body: &str) -> Vec<u8> {
        format!(
            "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        )
        .into_bytes()
    }

    fn flags(value: Option<&str>) -> FeatureFlags {
        let variants = value.map_or_else(HashMap::new, |value| {
            HashMap::from([(
                "test-flag".to_string(),
                FeatureFlagVariant {
                    value: value.to_string(),
                },
            )])
        });
        FeatureFlags { variants }
    }

    #[test]
    fn enabled_requires_a_non_off_variant() {
        assert!(!flags(None).enabled("test-flag"));
        assert!(!flags(Some("off")).enabled("test-flag"));
        assert!(!flags(Some("")).enabled("test-flag"));
        assert!(flags(Some("on")).enabled("test-flag"));
        assert!(flags(Some("experimental")).enabled("test-flag"));
    }

    #[test]
    fn deserializes_feature_flag_response() {
        let flags: FeatureFlags = serde_json::from_str(
            r#"{"variants":{"some-flag":{"value":"on"},"other":{"value":"off"},"bare":{}}}"#,
        )
        .unwrap();

        assert!(flags.enabled("some-flag"));
        assert!(!flags.enabled("other"));
        assert!(!flags.enabled("bare"));
    }

    #[test]
    fn empty_response_disables_all_flags() {
        let flags: FeatureFlags = serde_json::from_str("{}").unwrap();

        assert!(!flags.enabled("test-flag"));
    }

    #[tokio::test]
    async fn fetches_feature_flags_with_the_expected_request() {
        let (rest_uri, server) = start_http_server(response(
            "200 OK",
            r#"{"variants":{"test-reports":{"value":"on"}}}"#,
        ))
        .await;

        let flags = FeatureFlags::fetch(&format!("{rest_uri}/"), "test-key")
            .await
            .unwrap();
        assert!(flags.enabled("test-reports"));

        let request = String::from_utf8(server.await.unwrap()).unwrap();
        let (headers, _) = request.split_once("\r\n\r\n").unwrap();
        assert!(headers.starts_with("GET /api/v1/feature-flags/variants HTTP/1.1"));
        assert!(
            headers
                .lines()
                .any(|line| line.eq_ignore_ascii_case("authorization: Bearer test-key"))
        );

        let (rest_uri, server) =
            start_http_server(response("500 Internal Server Error", "{}")).await;
        assert!(FeatureFlags::fetch(&rest_uri, "test-key").await.is_err());
        server.await.unwrap();
    }
}
