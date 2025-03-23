use sift_rs::{
    ping::v1::{ping_service_client::PingServiceClient, PingRequest},
    Credentials, SiftChannelBuilder,
};
use std::env;

#[tokio::main]
async fn main() {
    let credentials = Credentials::Config {
        apikey: env::var("SIFT_API_KEY").unwrap(),
        uri: env::var("SIFT_URI").unwrap(),
    };

    let conn = SiftChannelBuilder::new(credentials).build().unwrap();
    let mut ping_service = PingServiceClient::new(conn);
    let ping_response = ping_service.ping(PingRequest::default()).await.unwrap();

    println!("{}", ping_response.into_inner().response);
}
