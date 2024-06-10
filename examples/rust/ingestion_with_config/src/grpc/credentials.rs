use anyhow::{Context, Result};
use std::env;

pub struct Credentials {
    pub uri: String,
    pub apikey: String,
}

impl Credentials {
    pub fn new() -> Result<Self> {
        let uri = env::var("SIFT_URI").context("Expected 'SIFT_URI' to be present in .env")?;
        let apikey =
            env::var("SIFT_API_KEY").context("Expected 'SIFT_API_KEY' to be present in .env")?;
        Ok(Self { uri, apikey })
    }
}
