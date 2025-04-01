use sift_rs::{
    Credentials, SiftChannelBuilder,
    assets::v1::{ListAssetsRequest, asset_service_client::AssetServiceClient},
};
use std::env;

#[tokio::main]
async fn main() {
    let credentials = Credentials::Config {
        apikey: env::var("SIFT_API_KEY").unwrap(),
        uri: env::var("SIFT_URI").unwrap(),
    };

    let conn = SiftChannelBuilder::new(credentials).build().unwrap();
    let mut asset_service = AssetServiceClient::new(conn);

    let response = asset_service
        .list_assets(ListAssetsRequest {
            filter: "name.matches('falcon$')".into(),
            ..Default::default()
        })
        .await
        .unwrap()
        .into_inner();

    println!("found {} assets", response.assets.len());

    for asset in response.assets {
        println!("{}", asset.name);
    }
}
