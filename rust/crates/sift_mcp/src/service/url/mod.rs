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
}

#[derive(Clone)]
pub struct UrlService {
    app_uri: String,
}

impl UrlService {
    pub fn new(app_uri: String) -> Self {
        Self { app_uri }
    }

    pub fn build_explore_url(&self, request: ExploreUrlRequest) -> Result<String, ErrorData> {
        let ExploreUrlRequest {
            assets,
            runs,
            channels,
            panel_type,
            start_time_unix_nanos,
            end_time_unix_nanos,
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

        let host = self.app_host()?;

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

    pub fn build_report_url(&self, report_id: &str) -> Result<String, ErrorData> {
        let host = self.app_host()?;
        Ok(format!("{host}/reports/{}", encode_value(report_id)))
    }

    #[cfg(feature = "test-reports")]
    pub fn build_test_report_url(&self, test_report_id: &str) -> Result<String, ErrorData> {
        let host = self.app_host()?;
        Ok(format!(
            "{host}/test-results/{}",
            encode_value(test_report_id)
        ))
    }

    pub fn build_rule_url(&self, rule_id: &str) -> Result<String, ErrorData> {
        let host = self.app_host()?;
        Ok(format!("{host}/rules/{}", encode_value(rule_id)))
    }

    pub fn build_annotation_url(&self, annotation_id: &str) -> Result<String, ErrorData> {
        let host = self.app_host()?;
        Ok(format!("{host}/annotation/{}", encode_value(annotation_id)))
    }

    pub fn build_asset_url(&self, asset_id: &str) -> Result<String, ErrorData> {
        let host = self.app_host()?;
        Ok(format!("{host}/asset/{}", encode_value(asset_id)))
    }

    pub fn build_run_url(&self, run_id: &str) -> Result<String, ErrorData> {
        let host = self.app_host()?;
        Ok(format!("{host}/run/{}", encode_value(run_id)))
    }

    fn app_host(&self) -> Result<&str, ErrorData> {
        let host = self.app_uri.trim().trim_end_matches('/');
        (!host.is_empty()).then_some(host).ok_or_else(|| {
            ErrorData::invalid_params(
                "could not build a Sift web URL because `app_uri` is not configured in the \
                     selected sift-cli profile",
                None,
            )
        })
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
