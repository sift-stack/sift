use rmcp::ErrorData;

#[cfg(test)]
mod test;

const KNOWN_PANEL_TYPES: &[&str] = &[
    "timeseries",
    "histogram",
    "table",
    "fft",
    "metrics",
    "scatter-plot",
    "geo-map",
];

const VALUE_ENCODE_SET: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~')
    .remove(b':');

#[derive(Debug, Default)]
pub struct ExploreUrlRequest {
    pub assets: Option<Vec<String>>,
    pub runs: Option<Vec<String>>,
    pub channels: Option<Vec<String>>,
    pub panel_type: Option<String>,
    pub start_time_unix_nanos: Option<i64>,
    pub end_time_unix_nanos: Option<i64>,
    pub explore_host: Option<String>,
}

#[derive(Clone)]
pub struct ExploreService {
    rest_uri: String,
}

impl ExploreService {
    pub fn new(rest_uri: String) -> Self {
        Self { rest_uri }
    }

    pub fn build_url(&self, request: ExploreUrlRequest) -> Result<String, ErrorData> {
        let ExploreUrlRequest {
            assets,
            runs,
            channels,
            panel_type,
            start_time_unix_nanos,
            end_time_unix_nanos,
            explore_host,
        } = request;

        let no_selection = assets.as_ref().is_none_or(|v| v.is_empty())
            && runs.as_ref().is_none_or(|v| v.is_empty())
            && channels.as_ref().is_none_or(|v| v.is_empty())
            && panel_type.is_none()
            && start_time_unix_nanos.is_none()
            && end_time_unix_nanos.is_none();
        if no_selection {
            return Err(ErrorData::invalid_params(
                "at least one of `assets`, `runs`, `channels`, `panel_type`, \
                 `start_time_unix_nanos`, or `end_time_unix_nanos` must be set",
                None,
            ));
        }

        if let (Some(start), Some(end)) = (start_time_unix_nanos, end_time_unix_nanos)
            && end < start
        {
            return Err(ErrorData::invalid_params(
                "`end_time_unix_nanos` must be greater than or equal to `start_time_unix_nanos`",
                None,
            ));
        }

        if let Some(ref p) = panel_type
            && !KNOWN_PANEL_TYPES.contains(&p.as_str())
        {
            return Err(ErrorData::invalid_params(
                format!(
                    "unknown `panel_type` `{p}`; expected one of: {}",
                    KNOWN_PANEL_TYPES.join(", ")
                ),
                None,
            ));
        }

        let host = match explore_host {
            Some(h) => h,
            None => derive_explore_host(&self.rest_uri)?,
        };

        let mut query = String::from("method=single");
        if let Some(v) = assets.as_ref().filter(|v| !v.is_empty()) {
            query.push_str("&assets=");
            query.push_str(&join_encoded(v));
        }
        if let Some(v) = runs.as_ref().filter(|v| !v.is_empty()) {
            query.push_str("&runs=");
            query.push_str(&join_encoded(v));
        }
        if let Some(v) = channels.as_ref().filter(|v| !v.is_empty()) {
            query.push_str("&channels=");
            query.push_str(&join_encoded(v));
        }
        if let Some(p) = panel_type {
            query.push_str("&panelType=");
            query.push_str(&encode_value(&p));
        }
        if let Some(start) = start_time_unix_nanos {
            query.push_str("&startTime=");
            query.push_str(&encode_value(
                &chrono::DateTime::from_timestamp_nanos(start)
                    .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            ));
        }
        if let Some(end) = end_time_unix_nanos {
            query.push_str("&endTime=");
            query.push_str(&encode_value(
                &chrono::DateTime::from_timestamp_nanos(end)
                    .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            ));
        }

        Ok(format!("{host}/explore?{query}"))
    }
}

fn encode_value(s: &str) -> String {
    percent_encoding::utf8_percent_encode(s, VALUE_ENCODE_SET).to_string()
}

fn join_encoded(values: &[String]) -> String {
    values
        .iter()
        .map(|v| encode_value(v))
        .collect::<Vec<_>>()
        .join(",")
}

fn derive_explore_host(rest_uri: &str) -> Result<String, ErrorData> {
    let swapped = rest_uri.replacen("://api.", "://app.", 1);
    if swapped == rest_uri {
        return Err(ErrorData::invalid_params(
            "could not derive Sift web host from `rest_uri` (no `api.` subdomain); pass \
             `explore_host` explicitly",
            None,
        ));
    }
    Ok(swapped.split('/').take(3).collect::<Vec<_>>().join("/"))
}
