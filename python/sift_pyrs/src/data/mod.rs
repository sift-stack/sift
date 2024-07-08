use crate::grpc::use_channel;
use prost::Message;
use pyo3::{
    exceptions::PyValueError,
    {pyclass, pymethods, PyResult},
};
use sift::gen::sift::data::v1::{data_service_client::DataServiceClient, GetDataRequest, GetDataResponse};
use std::thread;
use tokio::{
    task::JoinSet,
    runtime::{self, Runtime},
};

#[pyclass]
pub struct DataService {
    runtime: Runtime,
    uri: String,
    apikey: String,
}

#[pymethods]
impl DataService {
    #[new]
    pub fn new(uri: &str, apikey: &str, num_threads: Option<usize>) -> PyResult<Self> {
        let num_cpu = num_threads.or_else(|| thread::available_parallelism().map(usize::from).ok());

        let rt = match num_cpu {
            Some(num) if num > 1 => runtime::Builder::new_multi_thread()
                .worker_threads(usize::from(num))
                .build()?,
            _ => runtime::Builder::new_current_thread().build()?,
        };

        Ok(Self { uri: uri.to_string(), apikey: apikey.to_string(), runtime: rt })
    }

    pub fn get_data(&self, get_data_request: &[u8]) -> PyResult<()> {
        let apikey = self.apikey.to_string();
        let uri = self.uri.to_string();

        self.runtime.block_on(async move {
            let grpc_channel = use_channel(&uri, &apikey)
                .map_err(|e| PyValueError::new_err(e.to_string()))?;

            // Cache this bad boi
            let stub = DataServiceClient::new(grpc_channel);

            let req = GetDataRequest::decode(get_data_request).map_err(|e| {
                let msg = format!("Failed to decode wire format: {e}");
                PyValueError::new_err(msg)
            })?;

            let mut futures = JoinSet::new();

            for channel in req.queries {
                let request = GetDataRequest {
                    start_time: req.start_time.clone(),
                    end_time: req.end_time.clone(),
                    sample_ms: req.sample_ms,
                    page_size: req.page_size,
                    page_token: req.page_token.clone(),
                    queries: vec![channel],
                };

                let mut stub = stub.clone();

                futures.spawn(async move {
                    (request.clone(), stub.get_data(request).await)
                });
            }

            while let Some(Ok((mut req, res))) = futures.join_next().await {
                let GetDataResponse { next_page_token, data } = res
                    .map(|r| r.into_inner())
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;

                if !next_page_token.is_empty() {
                    req.page_token = next_page_token;

                    let mut stub = stub.clone();

                    futures.spawn(async move {
                        (req.clone(), stub.get_data(req).await)
                    });
                }

                println!("{data:?}")
            }

            Ok(())
        })
    }
}
