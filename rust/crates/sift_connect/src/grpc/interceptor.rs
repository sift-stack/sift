use std::str::FromStr;
use tonic::{Request, Status, metadata::MetadataValue, service::Interceptor};

/// The header that names the organization a request runs in, as the Sift web
/// app sends it when a platform admin selects an organization.
pub const ORGANIZATION_ID_HEADER: &str = "current-organization-id";

/// Interceptor that adds authentication headers to gRPC requests.
///
/// This interceptor automatically adds a `Bearer` token authorization header
/// to all outgoing gRPC requests using the provided API key. When
/// `organization_id` is set, it also sends [`ORGANIZATION_ID_HEADER`].
///
/// # Example
///
/// ```
/// use sift_connect::grpc::AuthInterceptor;
///
/// let interceptor = AuthInterceptor {
///     apikey: "your-api-key".to_string(),
///     organization_id: None,
/// };
/// ```
#[derive(Clone)]
pub struct AuthInterceptor {
    /// The API key to use for authentication.
    pub apikey: String,
    /// The organization every request runs in, sent as [`ORGANIZATION_ID_HEADER`].
    pub organization_id: Option<String>,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let auth_token = format!("Bearer {}", &self.apikey);
        let apikey = MetadataValue::from_str(&auth_token)
            .map_err(|e| Status::invalid_argument(format!("failed to parse API key: {e}")))?;

        request.metadata_mut().insert("authorization", apikey);

        if let Some(organization_id) = &self.organization_id {
            let organization_id = MetadataValue::from_str(organization_id).map_err(|e| {
                Status::invalid_argument(format!("failed to parse organization ID: {e}"))
            })?;
            request
                .metadata_mut()
                .insert(ORGANIZATION_ID_HEADER, organization_id);
        }
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intercept(organization_id: Option<&str>) -> Result<Request<()>, Status> {
        AuthInterceptor {
            apikey: "test-key".to_string(),
            organization_id: organization_id.map(str::to_string),
        }
        .call(Request::new(()))
    }

    #[test]
    fn sends_the_organization_header_when_set() {
        let request = intercept(Some("org-123")).unwrap();
        let metadata = request.metadata();
        assert_eq!(metadata.get("authorization").unwrap(), "Bearer test-key");
        assert_eq!(metadata.get(ORGANIZATION_ID_HEADER).unwrap(), "org-123");
    }

    #[test]
    fn omits_the_organization_header_when_unset() {
        let request = intercept(None).unwrap();
        assert!(request.metadata().get(ORGANIZATION_ID_HEADER).is_none());
    }

    #[test]
    fn rejects_an_organization_id_that_is_not_a_valid_header_value() {
        let status = intercept(Some("org\n123")).unwrap_err();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
    }
}
