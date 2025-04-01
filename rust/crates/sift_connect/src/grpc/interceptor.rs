use std::str::FromStr;
use tonic::{Request, Status, metadata::MetadataValue, service::Interceptor};

#[derive(Clone)]
pub struct AuthInterceptor {
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
