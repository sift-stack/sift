use super::SiftStreamMetrics;
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock}
};
use tokio::sync::RwLock;

static METRICS: LazyLock<RwLock<HashMap<String, Arc<SiftStreamMetrics>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub(crate) async fn register_metrics(uuid: String, metrics: Arc<SiftStreamMetrics>) {
    let mut global_metrics_guard = METRICS.write().await;
    global_metrics_guard.insert(uuid, metrics);
}

pub async fn start_metrics_server(port: u16) {
    use hyper::{server::conn::http1, service::service_fn};
    use hyper::{Request, Response, Method};
    use hyper_util::rt::TokioIo;
    use std::{convert::Infallible, net::SocketAddr};
    use tokio::net::TcpListener;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    async fn metrics_handle(req: Request<hyper::body::Incoming>) -> Result<Response<String>, Infallible> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") | (&Method::GET, "/metrics") => {
                let metrics_json = {
                    let metrics_guard = METRICS.read().await;
                    let serializable_map: HashMap<_, _> = metrics_guard
                        .iter()
                        .map(|(k, v)| (k, &**v))
                        .collect();
                    serde_json::to_string(&serializable_map).unwrap()
                };

                Ok(Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(metrics_json)
                    .unwrap()
                )
            }
            _ => Ok(Response::builder()
                .status(404)
                .body(String::new())
                .unwrap()
            )
        }
    }

    let listener = TcpListener::bind(addr).await.unwrap();

    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(metrics_handle))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    });
}
