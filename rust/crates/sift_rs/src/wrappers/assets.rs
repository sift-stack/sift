use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use pbjson_types::FieldMask;
use sift_connect::SiftChannel;
use sift_error::prelude::*;

use crate::assets::v1::{
    Asset, GetAssetRequest, UpdateAssetRequest, asset_service_client::AssetServiceClient,
};

/// Creates a new asset service wrapper.
///
/// Returns an implementation of [`AssetServiceWrapper`] which also exposes methods
/// from the raw [`AssetServiceClient`] via `Deref` and `DerefMut`.
///
/// # Arguments
///
/// * `grpc_channel` - The gRPC channel to use for communication
///
/// # Example
///
/// ```no_run
/// use sift_rs::wrappers::assets::{new_asset_service, AssetServiceWrapper};
/// use sift_connect::{Credentials, SiftChannelBuilder};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let credentials = Credentials::Config {
///     uri: "https://api.siftstack.com".to_string(),
///     apikey: "your-api-key".to_string(),
/// };
/// let channel = SiftChannelBuilder::new(credentials).build()?;
/// let mut asset_service = new_asset_service(channel);
///
/// let asset = asset_service.try_get_asset_by_id("asset-123").await?;
/// # Ok(())
/// # }
/// ```
pub fn new_asset_service(grpc_channel: SiftChannel) -> impl AssetServiceWrapper {
    AssetServiceImpl(AssetServiceClient::new(grpc_channel))
}

/// Convenience methods for working with Sift's Asset service.
///
/// This trait provides simplified methods that return [`sift_error::Result`] instead
/// of raw gRPC responses. The underlying [`AssetServiceClient`] is accessible via
/// `Deref` and `DerefMut` for advanced use cases.
#[async_trait]
pub trait AssetServiceWrapper:
    Clone + Deref<Target = AssetServiceClient<SiftChannel>> + DerefMut
{
    /// Retrieves an asset by ID.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The ID of the asset to retrieve
    ///
    /// # Returns
    ///
    /// The requested asset, or an error if the asset doesn't exist or the request fails.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::RetrieveAssetError`] if the request fails or the asset
    /// doesn't exist. Returns [`ErrorKind::EmptyResponseError`] if the response is empty.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_rs::wrappers::assets::AssetServiceWrapper;
    ///
    /// # async fn example(mut service: impl AssetServiceWrapper) -> Result<(), Box<dyn std::error::Error>> {
    /// let asset = service.try_get_asset_by_id("asset-123").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn try_get_asset_by_id(&mut self, asset_id: &str) -> Result<Asset>;

    /// Updates an asset.
    ///
    /// # Arguments
    ///
    /// * `asset` - The asset to update (must include the asset ID)
    /// * `update_mask` - List of field paths to update (in snake_case)
    ///
    /// # Returns
    ///
    /// The updated asset, or an error if the update fails.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::UpdateAssetError`] if the update fails. Returns
    /// [`ErrorKind::EmptyResponseError`] if the response is empty.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sift_rs::wrappers::assets::AssetServiceWrapper;
    ///
    /// # async fn example(mut service: impl AssetServiceWrapper, mut asset: sift_rs::assets::v1::Asset) -> Result<(), Box<dyn std::error::Error>> {
    /// asset.name = "Updated Name".to_string();
    /// let updated = service.try_update_asset(asset, vec!["name".to_string()]).await?;
    /// # Ok(())
    /// # }
    /// ```
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
