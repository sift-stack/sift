/// Parameters used to configure a connection to Sift over gRPC. The `uri` field
/// must include the scheme i.e. `https://`.
pub struct SiftChannelConfig {
    pub uri: String,
    pub apikey: String,
}

impl SiftChannelConfig {
    pub fn new(uri: &str, apikey: &str) -> Self {
        Self {
            uri: uri.to_string(),
            apikey: apikey.to_string(),
        }
    }
}
