use super::DiskBackupPolicy;
use crate::backup::disk::AsyncBackupsManager;
use crate::metrics::SiftStreamMetrics;
use crate::{TimeValue, backup::sanitize_name};
use sift_rs::ingest::v1::{
    IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest,
    ingest_with_config_data_channel_value::Type,
};
use std::{fs, sync::Arc};
use tempdir::TempDir;

#[test]
fn test_sanitize_name_with_illegal_chars() {
    let illegal_chars = vec![
        ':', '/', '\\', '*', '?', '"', '<', '>', '|', '.', ' ', '\t', '\n', '\r',
    ];
    for char in illegal_chars {
        assert_eq!(sanitize_name(&format!("test{}test", char)), "test_test");
    }
}

#[test]
fn test_sanitize_name_with_legal_chars() {
    assert_eq!(sanitize_name("test"), "test");
    assert_eq!(sanitize_name("test_test"), "test_test");
    assert_eq!(sanitize_name("test-test"), "test-test");
}

#[tokio::test]
async fn test_async_backups_manager_retrieve_data_with_graceful_termination() {
    let backups_dir = uuid::Uuid::new_v4().to_string();
    let backup_prefix = "test_async_backups_manager_retrieve_data_with_graceful_termination";

    let tmp_dir = TempDir::new(&backups_dir).expect("failed to creat tempdir");
    let tmp_dir_path = tmp_dir.path();

    let test_data = (0..100).map(|i| IngestWithConfigDataStreamRequest {
        ingestion_config_id: format!("{i}"),
        flow: String::from("some_flow"),
        timestamp: Some(*TimeValue::now()),
        channel_values: vec![IngestWithConfigDataChannelValue {
            r#type: Some(Type::Int32(i)),
        }],
        ..Default::default()
    });

    let disk_backup_policy = DiskBackupPolicy {
        backups_dir: Some(tmp_dir_path.to_path_buf()),
        ..Default::default()
    };
    let backup_retry_policy = crate::RetryPolicy::default();
    let (grpc_channel, mock_service) = crate::test::create_mock_grpc_channel_with_service().await;

    let mut backups_manager = AsyncBackupsManager::<IngestWithConfigDataStreamRequest>::new(
        &backups_dir,
        backup_prefix,
        disk_backup_policy,
        backup_retry_policy,
        grpc_channel,
        Arc::new(SiftStreamMetrics::new()),
    )
    .expect("failed to start backups manager");

    let mut expected = Vec::new();

    for data in test_data {
        expected.push(data.clone());

        backups_manager
            .send(data)
            .await
            .expect("failed to send data to backup task");
    }

    let captured_data = mock_service.get_captured_data();
    // No data should be sent prior to commanding start_backup_ingestion
    assert!(captured_data.is_empty());

    let file_count = backups_manager.start_backup_ingestion().await;

    // We should have a single file for ingestion
    assert_eq!(file_count, 1);

    // Wait a bit for the ingestion to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Verify that the mock service received the data via gRPC
    let captured_data = mock_service.get_captured_data();
    assert_eq!(
        captured_data.len(),
        expected.len(),
        "gRPC should have received all sent data"
    );

    // Make sure the data sent via gRPC matches what we sent to the backup manager
    for (lhs, rhs) in expected.into_iter().zip(captured_data) {
        assert_eq!(lhs.ingestion_config_id, rhs.ingestion_config_id);
        assert_eq!(lhs.flow, rhs.flow);
        assert_eq!(lhs.channel_values, rhs.channel_values);
    }

    // Graceful termination
    backups_manager
        .finish()
        .await
        .expect("failed to finish backup manager");
}

#[tokio::test]
async fn test_async_backups_manager_discard_data_with_graceful_termination() {
    let backups_dir = uuid::Uuid::new_v4().to_string();
    let backup_prefix = "test_async_backups_manager_discard_data_with_graceful_termination";

    let tmp_dir = TempDir::new(&backups_dir).expect("failed to creat tempdir");
    let tmp_dir_path = tmp_dir.path();

    let test_data = (0..100).map(|i| IngestWithConfigDataStreamRequest {
        ingestion_config_id: format!("{i}"),
        flow: String::from("some_flow"),
        timestamp: Some(*TimeValue::now()),
        channel_values: vec![IngestWithConfigDataChannelValue {
            r#type: Some(Type::Int32(i)),
        }],
        ..Default::default()
    });

    let disk_backup_policy = DiskBackupPolicy {
        backups_dir: Some(tmp_dir_path.to_path_buf()),
        ..Default::default()
    };
    let backup_retry_policy = crate::RetryPolicy::default();
    let (grpc_channel, mock_service) = crate::test::create_mock_grpc_channel_with_service().await;

    let mut backups_manager = AsyncBackupsManager::<IngestWithConfigDataStreamRequest>::new(
        &backups_dir,
        backup_prefix,
        disk_backup_policy,
        backup_retry_policy,
        grpc_channel,
        Arc::new(SiftStreamMetrics::new()),
    )
    .expect("failed to start backups manager");

    let mut expected = Vec::new();

    for data in test_data {
        expected.push(data.clone());

        backups_manager
            .send(data)
            .await
            .expect("failed to send data to backup task");
    }

    backups_manager
        .restart()
        .await
        .expect("Had error restarting");

    // Wait a moment for any possible ingestion to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Verify that the mock service recieved no data, since we didn't need to ingest
    let captured_data = mock_service.get_captured_data();
    assert!(captured_data.is_empty());

    // Graceful termination
    backups_manager
        .finish()
        .await
        .expect("failed to finish backup manager");
}
