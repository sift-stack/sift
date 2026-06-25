use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};

const VALUE_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~')
    .remove(b':');

fn encode(s: &str) -> String {
    utf8_percent_encode(s, VALUE_ENCODE_SET).to_string()
}

/// Build a Sift Explore deep-link from the user-configured `app_uri`. Returns
/// `None` when `app_uri` is unset or empty — Sift URLs are not deterministic
/// from the API host (e.g. `gov.siftstack.com` / `gov.grpc-api.siftstack.com`),
/// so the user must configure the web app URL explicitly in their profile.
pub fn build_explore_url(
    app_uri: Option<&str>,
    asset_name: &str,
    run_name: Option<&str>,
) -> Option<String> {
    let host = app_uri.map(str::trim).filter(|s| !s.is_empty())?;
    let host = host.trim_end_matches('/');

    let mut url = format!("{host}/explore?method=single&assets={}", encode(asset_name));
    if let Some(run) = run_name {
        url.push_str(&format!("&runs={}", encode(run)));
    }
    Some(url)
}
