use std::{str::FromStr, time::Duration};
use tonic::{metadata::MetadataValue, service::Interceptor, Request, Status};

#[derive(Clone)]
pub struct AuthInterceptor {
    pub apikey: String,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let auth_token = format!("Bearer {}", &self.apikey);
        let apikey = MetadataValue::from_str(&auth_token)
            .map_err(|e| Status::invalid_argument(format!("Failed to parse API key: {e}")))?;

        request.metadata_mut().insert("authorization", apikey);
        Ok(request)
    }
}

#[derive(Clone)]
pub struct SetTimeoutInterceptor;

impl Interceptor for SetTimeoutInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request.set_timeout(Duration::from_secs(super::REQUEST_TIMEOUT_SECS));
        Ok(request)
    }
}
