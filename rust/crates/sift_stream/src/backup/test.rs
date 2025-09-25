use super::{BackupsManager, DiskBackupPolicy, DiskBackupsManager, InMemoryBackupsManager};
use crate::TimeValue;
use crate::backup::disk::AsyncBackupsManager;
use sift_error::ErrorKind;
use sift_rs::ingest::v1::{
    IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest,
    ingest_with_config_data_channel_value::Type,
};
use std::fs;
use tempdir::TempDir;

#[tokio::test]
async fn test_disk_backups_manager_retrieve_data_with_graceful_termination() {
    let backups_dir = uuid::Uuid::new_v4().to_string();
    let backup_prefix = "test_disk_backups_manager_retrieve_data_with_graceful_termination";

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

    let mut backups_manager = DiskBackupsManager::<IngestWithConfigDataStreamRequest>::new(
        Some(tmp_dir_path.to_path_buf()),
        &backups_dir,
        backup_prefix,
        None,
    )
    .expect("failed top start backups manager");

    assert!(
        fs::exists(&backups_manager.backup_file).expect("perhaps a permission denied error"),
        "backup file should exist",
    );

    let mut expected = Vec::new();

    for data in test_data {
        expected.push(data.clone());

        backups_manager
            .send(data)
            .await
            .expect("failed to send data to backup task");
    }

    let backup_data = backups_manager
        .get_backup_data()
        .await
        .expect("failed to get backup data");

    // Make sure backup data and data sent match
    for (lhs, rhs) in expected.into_iter().zip(backup_data) {
        assert_eq!(lhs, rhs.unwrap(), "data sent and backup data don't match");
    }
    let backup_file_path = backups_manager.backup_file.clone();
    drop(backups_manager);
    assert!(
        !fs::exists(backup_file_path).unwrap(),
        "backup file should have been cleaned up",
    );
}

#[tokio::test]
async fn test_in_memory_backups_manager_retrieve_data() {
    let test_data = (0..100).map(|i| IngestWithConfigDataStreamRequest {
        ingestion_config_id: format!("{i}"),
        flow: String::from("some_flow"),
        timestamp: Some(*TimeValue::now()),
        channel_values: vec![IngestWithConfigDataChannelValue {
            r#type: Some(Type::Int32(i)),
        }],
        ..Default::default()
    });

    let mut test_data_iter = test_data.into_iter();

    let max_buffer_size = 10;
    let mut backups_manager =
        InMemoryBackupsManager::<IngestWithConfigDataStreamRequest>::new(Some(max_buffer_size));

    let mut expected = Vec::with_capacity(10);
    for _ in 0..10 {
        let data = test_data_iter.next().unwrap();
        backups_manager
            .send(data.clone())
            .await
            .expect("failed to send data to backup manager");
        expected.push(data);
    }

    let backups = backups_manager.get_backup_data().await.unwrap();
    for (lhs, rhs) in expected.clone().into_iter().zip(backups) {
        assert_eq!(lhs, rhs.unwrap(), "backups don't match actual data sent");
    }

    let data_point = test_data_iter.next().unwrap();

    assert!(
        backups_manager
            .send(data_point)
            .await
            .is_err_and(|e| e.kind() == ErrorKind::BackupLimitReached),
    );
    assert!(backups_manager.finish().await.is_ok());
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
