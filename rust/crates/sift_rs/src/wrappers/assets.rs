use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use pbjson_types::FieldMask;
use sift_connect::SiftChannel;
use sift_error::prelude::*;

use crate::assets::v1::{
    Asset, GetAssetRequest, UpdateAssetRequest, asset_service_client::AssetServiceClient,
};

/// Return an implementation of [AssetServiceWrapper] which also exposes methods from the
/// raw [AssetServiceClient].
pub fn new_asset_service(grpc_channel: SiftChannel) -> impl AssetServiceWrapper {
    AssetServiceImpl(AssetServiceClient::new(grpc_channel))
}

/// Convenience methods
#[async_trait]
pub trait AssetServiceWrapper:
    Clone + Deref<Target = AssetServiceClient<SiftChannel>> + DerefMut
{
    /// Retrieves an asset by ID
    async fn try_get_asset_by_id(&mut self, asset_id: &str) -> Result<Asset>;

    /// Update an asset
    async fn try_update_asset(&mut self, asset: Asset, update_mask: Vec<String>) -> Result<Asset>;
}

/// A convenience wrapper around [AssetServiceClient].
#[derive(Clone)]
struct AssetServiceImpl(AssetServiceClient<SiftChannel>);

#[async_trait]
impl AssetServiceWrapper for AssetServiceImpl {
    async fn try_get_asset_by_id(&mut self, asset_id: &str) -> Result<Asset> {
        let req = GetAssetRequest {
            asset_id: asset_id.into(),
        };
        let resp = self
            .get_asset(req)
            .await
            .map_err(|e| Error::new(ErrorKind::RetrieveAssetError, e))?;

        resp.into_inner().asset.ok_or_else(|| {
            Error::new_empty_response("unexpected empty response from AssetService/GetAsset")
        })
    }

    async fn try_update_asset(&mut self, asset: Asset, update_mask: Vec<String>) -> Result<Asset> {
        let req = UpdateAssetRequest {
            asset: Some(asset),
            update_mask: Some(FieldMask { paths: update_mask }),
        };
        let resp = self
            .update_asset(req)
            .await
            .map_err(|e| Error::new(ErrorKind::UpdateAssetError, e))?;

        resp.into_inner().asset.ok_or_else(|| {
            Error::new_empty_response("unexpected empty response from AssetService/UpdateAsset")
        })
    }
}

impl Deref for AssetServiceImpl {
    type Target = AssetServiceClient<SiftChannel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AssetServiceImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
