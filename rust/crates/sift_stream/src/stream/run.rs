use super::builder::RunForm;
use sift_connect::SiftChannel;
use sift_error::prelude::*;
use sift_rs::{
    metadata::v1::metadata_value::Value,
    retry::{RetryConfig, RetryExt},
    runs::v2::Run,
    wrappers::runs::{RunServiceWrapper, new_run_service},
};
use std::collections::{HashMap, HashSet};

pub enum RunSelector {
    ById(String),
    ByForm(RunForm),
}

/// Retrieves a run by run ID.
pub(super) async fn load_run_by_id(grpc_channel: SiftChannel, run_id: &str) -> Result<Run> {
    let run_service_wrapper = new_run_service(grpc_channel);
    let retrying_run = run_service_wrapper.retrying(RetryConfig::default());
    let run = retrying_run
        .call(|mut w| async move { w.try_get_run_by_id(run_id).await })
        .await?;

    #[cfg(feature = "tracing")]
    tracing::info!(
        run_id = run.run_id,
        run_name = run.name,
        "successfully retrieve run by ID",
    );

    Ok(run)
}

/// Retrieves a run or creates a run. If the run exists, this method will also update the run
/// if the `run_form` has changed since the last time it was used.
pub(super) async fn load_run_by_form(grpc_channel: SiftChannel, run_form: RunForm) -> Result<Run> {
    #[cfg(feature = "tracing")]
    tracing::info_span!("load_run_by_form");

    let run_service_wrapper = new_run_service(grpc_channel);
    let retrying_run = run_service_wrapper.retrying(RetryConfig::default());

    let RunForm {
        name,
        description,
        tags,
        metadata,
        client_key,
    } = run_form;

    let client_key_clone = client_key.clone();
    match retrying_run
        .call(|mut w| {
            let client_key = client_key_clone.clone();
            async move { w.try_get_run_by_client_key(&client_key).await }
        })
        .await
    {
        Err(e) if e.kind() == ErrorKind::NotFoundError => {
            let description_str = description.unwrap_or_default();
            let tags_vec = tags.unwrap_or_default();
            let metadata_vec = metadata.unwrap_or_default();
            let name_clone = name.clone();
            let client_key_clone2 = client_key.clone();
            let run = retrying_run
                .call(|mut w| {
                    let name = name_clone.clone();
                    let client_key = client_key_clone2.clone();
                    let description_str = description_str.clone();
                    let tags = tags_vec.clone();
                    let metadata = metadata_vec.clone();
                    async move {
                        w.try_create_run(
                            &name,
                            &client_key,
                            &description_str,
                            tags.as_slice(),
                            metadata.as_slice(),
                        )
                        .await
                    }
                })
                .await?;

            #[cfg(feature = "tracing")]
            tracing::info!(run_id = run.run_id, run_name = run.name, "created new run");

            Ok(run)
        }
        Err(e) => Err(e),

        Ok(mut run) => {
            #[cfg(feature = "tracing")]
            tracing::info!(
                run_id = run.run_id,
                run_name = run.name,
                "an existing run was found with the provided client-key"
            );

            // An existing run was found; see if we need to update it.
            let mut update_mask = Vec::new();

            if name != run.name {
                update_mask.push("name".to_string());
                run.name = name;
            }

            if description.as_ref().is_some_and(|d| d != &run.description) {
                update_mask.push("description".to_string());
                run.description = description.unwrap_or_default();
            }
            match tags {
                Some(new_tags) if run.tags.is_empty() => {
                    update_mask.push("tags".to_string());
                    run.tags = new_tags;
                }
                Some(new_tags) => {
                    let new_tags_set = HashSet::<&String>::from_iter(new_tags.iter());
                    let current_tags_set = HashSet::from_iter(run.tags.iter());
                    let difference = new_tags_set.difference(&current_tags_set);

                    if difference.count() > 0 {
                        update_mask.push("tags".to_string());
                        run.tags = new_tags;
                    }
                }
                _ => (),
            }
            match metadata {
                Some(new_metadata) if run.metadata.is_empty() => {
                    update_mask.push("metadata".to_string());
                    run.metadata = new_metadata;
                }
                Some(new_metadata) => {
                    let new_metadata_map: HashMap<String, Value> = HashMap::from_iter(
                        new_metadata
                            .iter()
                            .filter_map(|item| {
                                if let Some(key) = &item.key
                                    && let Some(value) = &item.value
                                {
                                    Some((key.name.clone(), value.clone()))
                                } else {
                                    None
                                }
                            })
                            .collect::<HashMap<String, Value>>(),
                    );

                    let current_metadata_map: HashMap<String, Value> = HashMap::from_iter(
                        run.metadata
                            .iter()
                            .filter_map(|item| {
                                if let Some(key) = &item.key
                                    && let Some(value) = &item.value
                                {
                                    Some((key.name.clone(), value.clone()))
                                } else {
                                    None
                                }
                            })
                            .collect::<HashMap<String, Value>>(),
                    );

                    if new_metadata_map != current_metadata_map {
                        update_mask.push("metadata".to_string());
                        run.metadata = new_metadata;
                    }
                }
                _ => (),
            }

            if update_mask.is_empty() {
                return Ok(run);
            }

            #[cfg(feature = "tracing")]
            tracing::info!(
                "updating run fields as some fields have changed: {}",
                update_mask.join(", ")
            );

            let update_mask_clone = update_mask.clone();
            let run_clone = run.clone();
            let updated_run = retrying_run
                .call(|mut w| {
                    let run = run_clone.clone();
                    let update_mask = update_mask_clone.clone();
                    async move { w.try_update_run(run, update_mask.as_slice()).await }
                })
                .await?;

            #[cfg(feature = "tracing")]
            tracing::info!("successfully updated run");

            Ok(updated_run)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper_util::rt::TokioIo;
    use sift_connect::{SiftChannel, grpc::interceptor::AuthInterceptor};
    use sift_rs::{
        metadata::v1::{MetadataKey, MetadataValue, metadata_value::Value},
        runs::v2::{
            CreateAdhocRunRequest, CreateAdhocRunResponse,
            CreateAutomaticRunAssociationForAssetsRequest,
            CreateAutomaticRunAssociationForAssetsResponse, CreateRunRequest, GetRunRequest,
            ListRunsRequest, StopRunRequest, StopRunResponse, UpdateRunRequest,
        },
    };
    use std::sync::{Arc, Mutex};
    use tokio::task::JoinHandle;
    use tonic::{
        Request, Response, Status,
        transport::{Endpoint, Server, Uri},
    };
    use tower::{ServiceBuilder, service_fn};
    /// Mock run service that captures calls to verify correct data is sent to try_update_run
    #[derive(Clone)]
    struct MockRunService {
        runs: Arc<Mutex<HashMap<String, Run>>>,    // client_key -> Run
        update_calls: Arc<Mutex<Vec<UpdateCall>>>, // Track update calls
    }

    #[derive(Debug, Clone)]
    struct UpdateCall {
        #[allow(dead_code)]
        run_name: String,
        update_mask: Vec<String>,
        updated_run: Run,
    }

    impl MockRunService {
        fn new() -> MockRunService {
            MockRunService {
                runs: Arc::new(Mutex::new(HashMap::new())),
                update_calls: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn add_existing_run(&self, client_key: &str, mut run: Run) {
            run.client_key = Some(client_key.to_string());
            self.runs
                .lock()
                .unwrap()
                .insert(client_key.to_string(), run);
        }

        fn get_update_calls(&self) -> Vec<UpdateCall> {
            self.update_calls.lock().unwrap().clone()
        }
    }

    #[tonic::async_trait]
    impl sift_rs::runs::v2::run_service_server::RunService for MockRunService {
        async fn create_run(
            &self,
            request: Request<CreateRunRequest>,
        ) -> std::result::Result<Response<sift_rs::runs::v2::CreateRunResponse>, Status> {
            let req = request.into_inner();
            let run_id = format!("run_id_{}", req.name.replace(' ', "_"));
            let client_key = req.client_key.clone().unwrap_or_default();

            let run = Run {
                run_id: run_id.clone(),
                name: req.name,
                description: req.description,
                tags: req.tags,
                metadata: req.metadata,
                client_key: req.client_key,
                ..Default::default()
            };

            self.runs.lock().unwrap().insert(client_key, run.clone());

            Ok(Response::new(sift_rs::runs::v2::CreateRunResponse {
                run: Some(run),
            }))
        }

        async fn get_run(
            &self,
            _request: Request<GetRunRequest>,
        ) -> std::result::Result<Response<sift_rs::runs::v2::GetRunResponse>, Status> {
            Err(Status::unimplemented("get_run not needed for these tests"))
        }

        async fn list_runs(
            &self,
            request: Request<ListRunsRequest>,
        ) -> std::result::Result<Response<sift_rs::runs::v2::ListRunsResponse>, Status> {
            let req = request.into_inner();

            // Parse the filter to extract client_key
            if let Some(client_key) = parse_client_key_filter(&req.filter)
                && let Some(run) = self.runs.lock().unwrap().get(&client_key)
            {
                return Ok(Response::new(sift_rs::runs::v2::ListRunsResponse {
                    runs: vec![run.clone()],
                    ..Default::default()
                }));
            }

            Ok(Response::new(sift_rs::runs::v2::ListRunsResponse {
                runs: vec![],
                ..Default::default()
            }))
        }

        async fn update_run(
            &self,
            request: Request<UpdateRunRequest>,
        ) -> std::result::Result<Response<sift_rs::runs::v2::UpdateRunResponse>, Status> {
            let req = request.into_inner();
            let updated_run = req
                .run
                .ok_or_else(|| Status::invalid_argument("run is required"))?;
            let update_mask = req.update_mask.unwrap_or_default();

            // Capture the update call for verification
            let update_call = UpdateCall {
                run_name: updated_run.name.clone(),
                update_mask: update_mask.paths.clone(),
                updated_run: updated_run.clone(),
            };

            self.update_calls.lock().unwrap().push(update_call);

            // Update the run in our storage
            if let Some(client_key) = &updated_run.client_key {
                self.runs
                    .lock()
                    .unwrap()
                    .insert(client_key.clone(), updated_run.clone());
            }

            Ok(Response::new(sift_rs::runs::v2::UpdateRunResponse {
                run: Some(updated_run),
            }))
        }

        async fn delete_run(
            &self,
            _request: Request<sift_rs::runs::v2::DeleteRunRequest>,
        ) -> std::result::Result<Response<sift_rs::runs::v2::DeleteRunResponse>, Status> {
            Err(Status::unimplemented(
                "delete_run not needed for these tests",
            ))
        }

        async fn create_adhoc_run(
            &self,
            _request: Request<CreateAdhocRunRequest>,
        ) -> std::result::Result<Response<CreateAdhocRunResponse>, Status> {
            Err(Status::unimplemented(
                "create_adhoc_run not needed for these tests",
            ))
        }

        async fn stop_run(
            &self,
            _request: Request<StopRunRequest>,
        ) -> std::result::Result<Response<StopRunResponse>, Status> {
            Err(Status::unimplemented("stop_run not needed for these tests"))
        }

        async fn create_automatic_run_association_for_assets(
            &self,
            _request: Request<CreateAutomaticRunAssociationForAssetsRequest>,
        ) -> std::result::Result<Response<CreateAutomaticRunAssociationForAssetsResponse>, Status>
        {
            Err(Status::unimplemented(
                "create_automatic_run_association_for_assets not needed for these tests",
            ))
        }
    }

    fn parse_client_key_filter(filter: &str) -> Option<String> {
        if let Some(start) = filter.find("client_key == '") {
            let start_idx = start + "client_key == '".len();
            if let Some(end_idx) = filter[start_idx..].find('\'') {
                return Some(filter[start_idx..start_idx + end_idx].to_string());
            }
        }
        None
    }

    async fn start_mock_run_service(service: MockRunService) -> (SiftChannel, JoinHandle<()>) {
        use std::io::Error as IoError;

        let (client, server) = tokio::io::duplex(1024);

        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(sift_rs::runs::v2::run_service_server::RunServiceServer::new(service))
                .serve_with_incoming(tokio_stream::once(Ok::<_, IoError>(server)))
                .await
                .unwrap();
        });

        let mut client = Some(client);
        let channel = Endpoint::try_from("http://[::]:50051")
            .unwrap()
            .connect_with_connector(service_fn(move |_: Uri| {
                let client = client.take();
                async move {
                    if let Some(client) = client {
                        Ok(TokioIo::new(client))
                    } else {
                        Err(std::io::Error::other("Client already taken"))
                    }
                }
            }))
            .await
            .unwrap();

        let sift_channel: SiftChannel = ServiceBuilder::new()
            .layer(tonic::service::interceptor(AuthInterceptor {
                apikey: "test_api_key".to_string(),
            }))
            .service(channel);

        (sift_channel, server)
    }

    fn create_metadata_value(key: &str, string_value: &str) -> MetadataValue {
        MetadataValue {
            key: Some(MetadataKey {
                name: key.to_string(),
                ..Default::default()
            }),
            value: Some(Value::StringValue(string_value.to_string())),
            archived_date: None,
            is_archived: false,
        }
    }

    #[tokio::test]
    async fn test_load_run_by_form_adds_tags_when_none_exist() {
        let mock_service = MockRunService::new();

        // Add an existing run with no tags
        let existing_run = Run {
            run_id: "test_run_1".to_string(),
            name: "Test Run".to_string(),
            description: "Original description".to_string(),
            tags: vec![], // No existing tags
            metadata: vec![],
            client_key: Some("test_client_key".to_string()),
            ..Default::default()
        };

        mock_service.add_existing_run("test_client_key", existing_run);

        let (channel, server) = start_mock_run_service(mock_service.clone()).await;

        let run_form = RunForm {
            name: "Test Run".to_string(),
            client_key: "test_client_key".to_string(),
            description: Some("Updated description".to_string()),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            metadata: None,
        };

        let result = load_run_by_form(channel, run_form).await;
        assert!(result.is_ok());

        let updated_run = result.unwrap();
        assert_eq!(updated_run.tags, vec!["tag1", "tag2"]);

        // Verify update_run was called with correct field mask
        let update_calls = mock_service.get_update_calls();
        assert_eq!(update_calls.len(), 1);
        assert!(update_calls[0].update_mask.contains(&"tags".to_string()));
        assert!(
            update_calls[0]
                .update_mask
                .contains(&"description".to_string())
        );
        assert_eq!(update_calls[0].updated_run.tags, vec!["tag1", "tag2"]);

        server.abort();
    }

    #[tokio::test]
    async fn test_load_run_by_form_adds_metadata_when_none_exist() {
        let mock_service = MockRunService::new();

        // Add an existing run with no metadata
        let existing_run = Run {
            run_id: "test_run_2".to_string(),
            name: "Test Run Metadata".to_string(),
            description: "Original description".to_string(),
            tags: vec![],
            metadata: vec![], // No existing metadata
            client_key: Some("metadata_test_key".to_string()),
            ..Default::default()
        };

        mock_service.add_existing_run("metadata_test_key", existing_run);

        let (channel, server) = start_mock_run_service(mock_service.clone()).await;

        let run_form = RunForm {
            name: "Test Run Metadata".to_string(),
            client_key: "metadata_test_key".to_string(),
            description: None,
            tags: None,
            metadata: Some(vec![
                create_metadata_value("env", "production"),
                create_metadata_value("version", "1.0"),
            ]),
        };

        let result = load_run_by_form(channel, run_form).await;
        assert!(result.is_ok());

        let updated_run = result.unwrap();
        assert_eq!(updated_run.metadata.len(), 2);

        // Verify update_run was called with correct field mask
        let update_calls = mock_service.get_update_calls();
        assert_eq!(update_calls.len(), 1);
        assert!(
            update_calls[0]
                .update_mask
                .contains(&"metadata".to_string())
        );
        assert_eq!(update_calls[0].updated_run.metadata.len(), 2);

        server.abort();
    }

    #[tokio::test]
    async fn test_load_run_by_form_same_tags_no_update() {
        let mock_service = MockRunService::new();

        // Add an existing run with same tags
        let existing_run = Run {
            run_id: "test_run_3".to_string(),
            name: "Test Run Same Tags".to_string(),
            description: "Original description".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            metadata: vec![],
            client_key: Some("same_tags_key".to_string()),
            ..Default::default()
        };

        mock_service.add_existing_run("same_tags_key", existing_run);

        let (channel, server) = start_mock_run_service(mock_service.clone()).await;

        let run_form = RunForm {
            name: "Test Run Same Tags".to_string(),
            client_key: "same_tags_key".to_string(),
            description: Some("Original description".to_string()),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]), // Same tags
            metadata: None,
        };

        let result = load_run_by_form(channel, run_form).await;
        assert!(result.is_ok());

        // Verify NO update_run was called since nothing changed
        let update_calls = mock_service.get_update_calls();
        assert_eq!(update_calls.len(), 0);

        server.abort();
    }

    #[tokio::test]
    async fn test_load_run_by_form_same_metadata_no_update() {
        let mock_service = MockRunService::new();

        // Add an existing run with same metadata
        let existing_run = Run {
            run_id: "test_run_4".to_string(),
            name: "Test Run Same Metadata".to_string(),
            description: "Original description".to_string(),
            tags: vec![],
            metadata: vec![
                create_metadata_value("env", "production"),
                create_metadata_value("version", "1.0"),
            ],
            client_key: Some("same_metadata_key".to_string()),
            ..Default::default()
        };

        mock_service.add_existing_run("same_metadata_key", existing_run);

        let (channel, server) = start_mock_run_service(mock_service.clone()).await;

        let run_form = RunForm {
            name: "Test Run Same Metadata".to_string(),
            client_key: "same_metadata_key".to_string(),
            description: Some("Original description".to_string()),
            tags: None,
            metadata: Some(vec![
                create_metadata_value("env", "production"),
                create_metadata_value("version", "1.0"),
            ]), // Same metadata
        };

        let result = load_run_by_form(channel, run_form).await;
        assert!(result.is_ok());

        // Verify NO update_run was called since nothing changed
        let update_calls = mock_service.get_update_calls();
        assert_eq!(update_calls.len(), 0);

        server.abort();
    }

    #[tokio::test]
    async fn test_load_run_by_form_replaces_existing_tags() {
        let mock_service = MockRunService::new();

        // Add an existing run with existing tags
        let existing_run = Run {
            run_id: "test_run_5".to_string(),
            name: "Test Run Replace Tags".to_string(),
            description: "Original description".to_string(),
            tags: vec!["old_tag1".to_string(), "old_tag2".to_string()],
            metadata: vec![],
            client_key: Some("replace_tags_key".to_string()),
            ..Default::default()
        };

        mock_service.add_existing_run("replace_tags_key", existing_run);

        let (channel, server) = start_mock_run_service(mock_service.clone()).await;

        let run_form = RunForm {
            name: "Test Run Replace Tags".to_string(),
            client_key: "replace_tags_key".to_string(),
            description: Some("Original description".to_string()),
            tags: Some(vec![
                "new_tag1".to_string(),
                "new_tag2".to_string(),
                "new_tag3".to_string(),
            ]),
            metadata: None,
        };

        let result = load_run_by_form(channel, run_form).await;
        assert!(result.is_ok());

        let updated_run = result.unwrap();
        // Verify that the old tags are completely replaced with new tags
        assert_eq!(updated_run.tags, vec!["new_tag1", "new_tag2", "new_tag3"]);
        assert!(!updated_run.tags.contains(&"old_tag1".to_string()));
        assert!(!updated_run.tags.contains(&"old_tag2".to_string()));

        // Verify update_run was called with correct field mask and data
        let update_calls = mock_service.get_update_calls();
        assert_eq!(update_calls.len(), 1);
        assert!(update_calls[0].update_mask.contains(&"tags".to_string()));
        assert_eq!(
            update_calls[0].updated_run.tags,
            vec!["new_tag1", "new_tag2", "new_tag3"]
        );

        server.abort();
    }

    #[tokio::test]
    async fn test_load_run_by_form_replaces_existing_metadata() {
        let mock_service = MockRunService::new();

        // Add an existing run with existing metadata
        let existing_run = Run {
            run_id: "test_run_6".to_string(),
            name: "Test Run Replace Metadata".to_string(),
            description: "Original description".to_string(),
            tags: vec![],
            metadata: vec![
                create_metadata_value("old_key1", "old_value1"),
                create_metadata_value("old_key2", "old_value2"),
            ],
            client_key: Some("replace_metadata_key".to_string()),
            ..Default::default()
        };

        mock_service.add_existing_run("replace_metadata_key", existing_run);

        let (channel, server) = start_mock_run_service(mock_service.clone()).await;

        let run_form = RunForm {
            name: "Test Run Replace Metadata".to_string(),
            client_key: "replace_metadata_key".to_string(),
            description: Some("Original description".to_string()),
            tags: None,
            metadata: Some(vec![
                create_metadata_value("new_key1", "new_value1"),
                create_metadata_value("new_key2", "new_value2"),
                create_metadata_value("new_key3", "new_value3"),
            ]),
        };

        let result = load_run_by_form(channel, run_form).await;
        assert!(result.is_ok());

        let updated_run = result.unwrap();
        assert_eq!(updated_run.metadata.len(), 3);

        // Verify that the old metadata is completely replaced with new metadata
        let metadata_keys: Vec<String> = updated_run
            .metadata
            .iter()
            .filter_map(|m| m.key.as_ref().map(|k| k.name.clone()))
            .collect();

        assert!(metadata_keys.contains(&"new_key1".to_string()));
        assert!(metadata_keys.contains(&"new_key2".to_string()));
        assert!(metadata_keys.contains(&"new_key3".to_string()));
        assert!(!metadata_keys.contains(&"old_key1".to_string()));
        assert!(!metadata_keys.contains(&"old_key2".to_string()));

        // Verify update_run was called with correct field mask and data
        let update_calls = mock_service.get_update_calls();
        assert_eq!(update_calls.len(), 1);
        assert!(
            update_calls[0]
                .update_mask
                .contains(&"metadata".to_string())
        );
        assert_eq!(update_calls[0].updated_run.metadata.len(), 3);

        server.abort();
    }

    #[tokio::test]
    async fn test_load_run_by_form_metadata_key_value_change() {
        let mock_service = MockRunService::new();

        // Add an existing run with existing metadata
        let existing_run = Run {
            run_id: "test_run_8".to_string(),
            name: "Test Run Metadata Change".to_string(),
            description: "Original description".to_string(),
            tags: vec![],
            metadata: vec![
                create_metadata_value("env", "staging"),
                create_metadata_value("version", "1.0"),
                create_metadata_value("region", "us-east-1"),
            ],
            client_key: Some("metadata_change_key".to_string()),
            ..Default::default()
        };

        mock_service.add_existing_run("metadata_change_key", existing_run);

        let (channel, server) = start_mock_run_service(mock_service.clone()).await;

        let run_form = RunForm {
            name: "Test Run Metadata Change".to_string(),
            client_key: "metadata_change_key".to_string(),
            description: Some("Original description".to_string()),
            tags: None,
            metadata: Some(vec![
                create_metadata_value("env", "production"), // Same key, different value
                create_metadata_value("version", "1.0"),    // Same key-value pair
                create_metadata_value("build", "123"),      // New key-value pair
            ]),
        };

        let result = load_run_by_form(channel, run_form).await;
        assert!(result.is_ok());

        let updated_run = result.unwrap();
        assert_eq!(updated_run.metadata.len(), 3);

        // Verify that metadata is completely replaced
        let metadata_map: std::collections::HashMap<String, String> = updated_run
            .metadata
            .iter()
            .filter_map(|m| {
                if let (Some(key), Some(Value::StringValue(s))) = (&m.key, &m.value) {
                    Some((key.name.clone(), s.clone()))
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(metadata_map.get("env"), Some(&"production".to_string()));
        assert_eq!(metadata_map.get("version"), Some(&"1.0".to_string()));
        assert_eq!(metadata_map.get("build"), Some(&"123".to_string()));
        assert!(!metadata_map.contains_key("region")); // Old key should be gone

        // Verify update_run was called due to different metadata
        let update_calls = mock_service.get_update_calls();
        assert_eq!(update_calls.len(), 1);
        assert!(
            update_calls[0]
                .update_mask
                .contains(&"metadata".to_string())
        );

        server.abort();
    }

    #[tokio::test]
    async fn test_load_run_by_form_add_both_tags_and_metadata_to_existing() {
        let mock_service = MockRunService::new();

        // Add an existing run with both tags and metadata
        let existing_run = Run {
            run_id: "test_run_9".to_string(),
            name: "Test Run Both Fields".to_string(),
            description: "Original description".to_string(),
            tags: vec!["old_tag".to_string()],
            metadata: vec![create_metadata_value("old_key", "old_value")],
            client_key: Some("both_fields_key".to_string()),
            ..Default::default()
        };

        mock_service.add_existing_run("both_fields_key", existing_run);

        let (channel, server) = start_mock_run_service(mock_service.clone()).await;

        let run_form = RunForm {
            name: "Test Run Both Fields".to_string(),
            client_key: "both_fields_key".to_string(),
            description: Some("Updated description".to_string()),
            tags: Some(vec!["new_tag1".to_string(), "new_tag2".to_string()]),
            metadata: Some(vec![
                create_metadata_value("new_key1", "new_value1"),
                create_metadata_value("new_key2", "new_value2"),
            ]),
        };

        let result = load_run_by_form(channel, run_form).await;
        assert!(result.is_ok());

        let updated_run = result.unwrap();
        assert_eq!(updated_run.tags, vec!["new_tag1", "new_tag2"]);
        assert_eq!(updated_run.metadata.len(), 2);
        assert_eq!(updated_run.description, "Updated description");

        // Verify update_run was called with all three field masks
        let update_calls = mock_service.get_update_calls();
        assert_eq!(update_calls.len(), 1);
        assert!(update_calls[0].update_mask.contains(&"tags".to_string()));
        assert!(
            update_calls[0]
                .update_mask
                .contains(&"metadata".to_string())
        );
        assert!(
            update_calls[0]
                .update_mask
                .contains(&"description".to_string())
        );

        server.abort();
    }
}
