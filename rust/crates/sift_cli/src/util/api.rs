use std::collections::HashMap;

use anyhow::Result;
use reqwest::{ClientBuilder, header::AUTHORIZATION};
use sift_rs::{Credentials, SiftChannel, SiftChannelBuilder};

use crate::cmd::Context;

pub fn create_grpc_channel(
    Context {
        grpc_uri,
        api_key,
        disable_tls,
        ..
    }: &Context,
) -> Result<SiftChannel> {
    let mut builder = SiftChannelBuilder::new(Credentials::Config {
        uri: grpc_uri.into(),
        apikey: api_key.into(),
    });
    if *disable_tls {
        builder = builder.use_tls(false)
    }
    Ok(builder.build()?)
}

pub fn create_rest_client(Context { api_key, .. }: &Context) -> Result<reqwest::Client> {
    let mut http_headers = HashMap::<String, String>::new();
    http_headers.insert(AUTHORIZATION.to_string(), format!("Bearer {api_key}"));

    let rest_client = ClientBuilder::new()
        .default_headers((&http_headers).try_into()?)
        .build()?;

    Ok(rest_client)
}
