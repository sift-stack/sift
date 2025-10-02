use super::SiftStreamMetrics;
use sift_error::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tokio::sync::RwLock;

#[cfg(feature = "metrics-unstable")]
use std::net::SocketAddr;

#[cfg(feature = "metrics-unstable")]
static METRICS: LazyLock<RwLock<HashMap<String, Arc<SiftStreamMetrics>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[cfg(feature = "metrics-unstable")]
pub(crate) async fn register_metrics(uuid: String, metrics: Arc<SiftStreamMetrics>) {
    let mut global_metrics_guard = METRICS.write().await;
    global_metrics_guard.insert(uuid, metrics);
}

#[cfg(feature = "metrics-unstable")]
/// Builder for a light weight HTTP metrics server.
///
/// Defaults to 127.0.0.1:8080
///
/// Once started, the server exposes metrics as JSON at the `/` and `/metrics` endpoints.
/// On a GET request, a snapshot metrics for all SiftStream instances are returned
/// in a JSON format, organized by sift_stream_id
///
/// Metric snapshots are taken at the moment the GET request is receieved.
pub struct MetricsServerBuilder {
    socket_addr: SocketAddr,
}

#[cfg(feature = "metrics-unstable")]
impl MetricsServerBuilder {
    /// Create a new MetricsServerBuilder, with a default address of 127.0.0.1:8080
    pub fn new() -> MetricsServerBuilder {
        MetricsServerBuilder {
            socket_addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
        }
    }

    /// Set the socket address of the server
    pub fn socket(mut self, socket_addr: SocketAddr) -> MetricsServerBuilder {
        self.socket_addr = socket_addr;
        self
    }

    /// Start up the metrics server
    pub async fn start_metrics_server(self) -> Result<()> {
        use hyper::{Method, Request, Response};
        use hyper::{server::conn::http1, service::service_fn};
        use hyper_util::rt::TokioIo;
        use std::convert::Infallible;
        use tokio::net::TcpListener;

        async fn metrics_handle(
            req: Request<hyper::body::Incoming>,
        ) -> std::result::Result<Response<String>, Infallible> {
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") | (&Method::GET, "/metrics") => {
                    let metrics_json = {
                        let metrics_guard = METRICS.read().await;
                        let serializable_map: HashMap<_, _> = metrics_guard
                            .iter()
                            .map(|(k, v)| (k, v.snapshot()))
                            .collect();
                        match serde_json::to_string(&serializable_map) {
                            Ok(json) => json,
                            Err(e) => {
                                #[cfg(feature = "tracing")]
                                tracing::error!("Failed to serialize metrics: {:?}", e);
                                "{}".to_string()
                            }
                        }
                    };

                    Ok(Response::builder()
                        .status(200)
                        .header("Content-Type", "application/json")
                        .body(metrics_json)
                        .unwrap_or_else(|_| {
                            #[cfg(feature = "tracing")]
                            tracing::error!("Failed to build response");
                            Response::new(String::new())
                        }))
                }
                _ => Ok(Response::builder()
                    .status(404)
                    .body(String::new())
                    .expect("Unable to build 404 response")),
            }
        }

        let listener = TcpListener::bind(self.socket_addr)
            .await
            .map_err(|e| Error::new(ErrorKind::SiftStreamMetricsServerError, e))
            .context("unable to bind socket address")?;

        tokio::spawn(async move {
            loop {
                let accept_result = listener.accept().await;
                let stream = match accept_result {
                    Ok((stream, _)) => stream,
                    Err(e) => {
                        #[cfg(feature = "tracing")]
                        tracing::error!("Failed to accept connection: {:?}", e);
                        continue;
                    }
                };
                let io = TokioIo::new(stream);

                tokio::task::spawn(async move {
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(io, service_fn(metrics_handle))
                        .await
                    {
                        #[cfg(feature = "tracing")]
                        tracing::error!("Error serving connection: {:?}", err);
                    }
                });
            }
        });

        Ok(())
    }
}

#[cfg(feature = "metrics-unstable")]
impl Default for MetricsServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
