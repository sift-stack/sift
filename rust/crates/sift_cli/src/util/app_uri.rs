const PROD_REST_HOST: &str = "api.siftstack.com";
const PROD_APP_URI: &str = "https://app.siftstack.com";
const GOV_REST_HOST: &str = "gov.api.siftstack.com";
const GOV_APP_URI: &str = "https://gov.siftstack.com";

pub fn infer_app_uri(rest_uri: &str) -> Option<&'static str> {
    let rest_uri = rest_uri.trim();
    let authority_and_path = rest_uri
        .strip_prefix("https://")
        .or_else(|| rest_uri.strip_prefix("http://"))?;
    let (authority, path) = authority_and_path
        .split_once('/')
        .unwrap_or((authority_and_path, ""));
    if !path.trim_matches('/').is_empty() {
        return None;
    }

    if authority.eq_ignore_ascii_case(PROD_REST_HOST) {
        Some(PROD_APP_URI)
    } else if authority.eq_ignore_ascii_case(GOV_REST_HOST) {
        Some(GOV_APP_URI)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::infer_app_uri;

    #[test]
    fn infers_public_sift_app_uris() {
        for rest_uri in [
            "http://api.siftstack.com",
            "https://api.siftstack.com",
            "https://api.siftstack.com/",
            "https://API.SIFTSTACK.COM",
        ] {
            assert_eq!(infer_app_uri(rest_uri), Some("https://app.siftstack.com"));
        }
        for rest_uri in [
            "http://gov.api.siftstack.com",
            "https://gov.api.siftstack.com",
            "https://gov.api.siftstack.com/",
        ] {
            assert_eq!(infer_app_uri(rest_uri), Some("https://gov.siftstack.com"));
        }
    }

    #[test]
    fn does_not_infer_non_public_hosts_or_paths() {
        for rest_uri in [
            "https://api.example.net",
            "https://app.siftstack.com",
            "https://api.development.siftstack.com",
            "https://api.staging.internal",
            "https://api.sift.test",
            "https://api.siftstack.com/v1",
            "https://api.siftstack.com?query=value",
            "https://api.siftstack.com#fragment",
            "https://api.siftstack.com:443",
            "https://api.siftstack.com.example.net",
            "api.siftstack.com",
        ] {
            assert_eq!(infer_app_uri(rest_uri), None, "rest_uri: {rest_uri}");
        }
    }
}
