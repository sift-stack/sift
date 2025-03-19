use super::{BackupsManager, Message};
use sift_rs::ingest::v1::IngestWithConfigDataStreamRequest;
use std::{fs, sync::atomic::Ordering};
use tempdir::TempDir;

#[tokio::test]
async fn test_backups_manager_retrieve_data_with_graceful_termination() {
    let backups_dir = "my-backups-v1";
    let backup_prefix = "my-run";

    let tmp_dir = TempDir::new(backups_dir).expect("failed to creat tempdir");
    let tmp_dir_path = tmp_dir.path();

    let test_data = (0..100).map(|i| IngestWithConfigDataStreamRequest {
        ingestion_config_id: format!("{i}"),
        ..Default::default()
    });

    let mut backups_manager = BackupsManager::<IngestWithConfigDataStreamRequest>::new(
        Some(tmp_dir_path.to_path_buf()),
        backups_dir,
        backup_prefix,
    )
    .expect("failed top start backups manager");

    assert!(
        fs::exists(&backups_manager.backup_file).expect("perhaps a permission denied error"),
        "backup file should exist",
    );
    assert!(
        !backups_manager.is_reading.load(Ordering::Relaxed),
        "this should be initialized to false"
    );

    let mut expected = Vec::new();

    for data in test_data {
        expected.push(data.clone());
        let message = Message::Data(data);

        backups_manager
            .send(message)
            .expect("failed to send data to backup task");
    }

    let backup_data = backups_manager
        .get_backup_data()
        .await
        .expect("failed to get backup data");

    // Make sure backup data and data sent match
    for (lhs, rhs) in expected.into_iter().zip(backup_data) {
        assert_eq!(lhs, rhs, "data sent and backup data don't match");
    }

    backups_manager
        .checkpoint()
        .await
        .expect("checkpoint should have succeeded");

    let md = fs::metadata(&backups_manager.backup_file).expect("failed to read backup metadata");
    assert_eq!(
        0,
        md.len(),
        "backup file should have been truncated after checkpoint signal"
    );

    let backup_file_path = backups_manager.backup_file.clone();
    backups_manager
        .finish()
        .await
        .expect("failed to gracefully terminate backups manager");

    assert!(
        !fs::exists(backup_file_path).unwrap(),
        "backup file should have been cleaned up",
    );
}

#[tokio::test]
async fn test_backups_manager_retrieve_data_with_hard_termination() {
    let backups_dir = "my-backups-v2";
    let backup_prefix = "my-run";

    let tmp_dir = TempDir::new(backups_dir).expect("failed to create tempdir");
    let tmp_dir_path = tmp_dir.path();

    let test_data = (0..100).map(|i| IngestWithConfigDataStreamRequest {
        ingestion_config_id: format!("{i}"),
        ..Default::default()
    });

    let backups_manager = BackupsManager::<IngestWithConfigDataStreamRequest>::new(
        Some(tmp_dir_path.to_path_buf()),
        backups_dir,
        backup_prefix,
    )
    .expect("failed top start backups manager");

    assert!(
        fs::exists(&backups_manager.backup_file).expect("perhaps a permission denied error"),
        "backup file should exist",
    );
    assert!(
        !backups_manager.is_reading.load(Ordering::Relaxed),
        "this should be initialized to false"
    );

    let mut expected = Vec::new();

    for data in test_data {
        expected.push(data.clone());
        let message = Message::Data(data);

        backups_manager
            .send(message)
            .expect("failed to send data to backup task");
    }

    let backup_data = backups_manager
        .get_backup_data()
        .await
        .expect("failed to get backup data");

    // Make sure backup data and data sent match
    for (lhs, rhs) in expected.into_iter().zip(backup_data) {
        assert_eq!(lhs, rhs, "data sent and backup data don't match");
    }

    let backup_file_path = backups_manager.backup_file.clone();
    backups_manager
        .terminate()
        .await
        .expect("termination should have succeeded");

    let md = fs::metadata(&backup_file_path).expect("termination shouldn't remove backup file");
    assert!(
        md.len() > 0,
        "backup file should still exist and have data in it"
    );
}
