use chrono::{DateTime, Utc};
use pbjson_types::Timestamp as PbTimestamp;

pub fn pbts_to_rfc3339(pbts: Option<PbTimestamp>) -> String {
    pbts.and_then(|ts| DateTime::<Utc>::from_timestamp(ts.seconds, ts.nanos as u32))
        .unwrap_or_default()
        .to_rfc3339()
}
