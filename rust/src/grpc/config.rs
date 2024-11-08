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
