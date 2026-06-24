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

pub fn build_explore_url(
    rest_uri: String,
    asset_name: String,
    run_name: Option<String>,
) -> Option<String> {
    let swapped = rest_uri.replacen("://api", "://app", 1);
    if swapped == rest_uri {
        return None;
    }
    let host = swapped.split('/').take(3).collect::<Vec<_>>().join("/");

    let mut url = format!("{host}/explore?method=single&assets={}", encode(&asset_name));
    if let Some(run) = run_name {
        url.push_str(&format!("&runs={}", encode(&run)));
    }
    Some(url)
}
