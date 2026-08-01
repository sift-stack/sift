use crossterm::style::Stylize;
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

pub struct ImportTarget {
    pub location: String,
    pub explore_url: Option<String>,
}

pub fn import_target(
    asset: &str,
    run_name: Option<&str>,
    run_id: Option<&str>,
    app_uri: Option<&str>,
) -> ImportTarget {
    let run = run_id.or(run_name);
    let location = run.map_or_else(
        || format!("asset '{}'", asset.cyan()),
        |r| format!("run '{}'", r.cyan()),
    );
    let explore_url = build_explore_url(app_uri, asset, run);
    ImportTarget {
        location,
        explore_url,
    }
}

pub fn explore_or_note(explore_url: Option<&str>) -> String {
    match explore_url {
        Some(url) => format!("\nView in Sift: {url}"),
        None => "\nOpen your Sift web app and copy its URL origin. Then run `sift-cli config \
                 update --app-uri <SIFT_WEB_ORIGIN>`. Add `--profile <name>` when you use a \
                 named profile."
            .to_string(),
    }
}

pub fn pending_import_tip(location: &str, explore_url: Option<&str>) -> String {
    let mut tip =
        format!("Once processing is complete the data will be available on the {location}.");
    tip.push_str(&explore_or_note(explore_url));
    tip
}

pub fn build_explore_url(
    app_uri: Option<&str>,
    asset_name: &str,
    run: Option<&str>,
) -> Option<String> {
    let host = app_uri.map(str::trim).filter(|s| !s.is_empty())?;
    let host = host.trim_end_matches('/');

    let mut url = format!("{host}/explore?method=single&assets={}", encode(asset_name));
    if let Some(run) = run {
        url.push_str(&format!("&runs={}", encode(run)));
    }
    Some(url)
}
