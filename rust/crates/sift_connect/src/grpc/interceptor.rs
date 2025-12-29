use std::str::FromStr;
use tonic::{Request, Status, metadata::MetadataValue, service::Interceptor};

/// Interceptor that adds authentication headers to gRPC requests.
///
/// This interceptor automatically adds a `Bearer` token authorization header
/// to all outgoing gRPC requests using the provided API key.
///
/// # Example
///
/// ```
/// use sift_connect::grpc::AuthInterceptor;
///
/// let interceptor = AuthInterceptor {
///     apikey: "your-api-key".to_string(),
/// };
/// ```
#[derive(Clone)]
pub struct AuthInterceptor {
    /// The API key to use for authentication.
    pub apikey: String,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let auth_token = format!("Bearer {}", &self.apikey);
        let apikey = MetadataValue::from_str(&auth_token)
            .map_err(|e| Status::invalid_argument(format!("failed to parse API key: {e}")))?;

        request.metadata_mut().insert("authorization", apikey);
        Ok(request)
    }
}
