use std::fs;
use std::time::Duration;

use crate::TimeValue;
use crate::backup::DiskBackupPolicy;
use crate::{
    ChannelValue, Flow, IngestionConfigForm, RecoveryStrategy, RunForm, SiftStreamBuilder,
};
use tempdir::TempDir;

#[tokio::test]
async fn test_sift_stream_builder_backup_manager_directory_naming_with_run() {
    let backups_dir = uuid::Uuid::new_v4().to_string();

    let tmp_dir = TempDir::new(&backups_dir).expect("failed to creat tempdir");
    let tmp_dir_path = tmp_dir.path();

    let ingestion_config = IngestionConfigForm {
        asset_name: "test_asset".to_string(),
        client_key: "test_client_key".to_string(),
        flows: vec![],
    };
    let run = RunForm {
        name: "test_run".to_string(),
        client_key: "test_client_key".to_string(),
        description: None,
        tags: None,
        metadata: None,
    };

    let disk_backup_policy = DiskBackupPolicy {
        backups_dir: Some(tmp_dir_path.to_path_buf()),
        retain_backups: true,
        ..Default::default()
    };
    let retry_policy = crate::RetryPolicy::default();
    let (grpc_channel, _mock_service) = crate::test::create_mock_grpc_channel_with_service().await;

    let mut sift_stream = SiftStreamBuilder::from_channel(grpc_channel)
        .ingestion_config(ingestion_config)
        .attach_run(run)
        .recovery_strategy(RecoveryStrategy::RetryWithBackups {
            retry_policy,
            disk_backup_policy,
        })
        .build()
        .await
        .expect("failed to build sift stream");

    for data in 0..100 {
        sift_stream
            .send(Flow::new(
                "some_flow",
                TimeValue::now(),
                &[ChannelValue::new("some_channel", data)],
            ))
            .await
            .expect("failed to send data to backup task");
    }

    // Finish the stream to ensure that the backup manager is shutdown and the backup files are processed.
    tokio::time::timeout(Duration::from_secs(10), async {
        assert!(
            sift_stream.finish().await.is_ok(),
            "failed to finish sift stream"
        );
    })
    .await
    .expect("timeout waiting for sift stream to finish");

    let test_dir = fs::read_dir(tmp_dir_path)
        .expect("failed to read backups directory")
        .collect::<Vec<_>>();
    assert_eq!(test_dir.len(), 1);

    // The first subdirectory should be the asset name.
    let asset_dir = test_dir[0].as_ref().expect("failed to get file");
    assert!(asset_dir.path().is_dir(), "expected file to be a directory");

    let asset_dir_path = asset_dir.path();
    let asset_dir_file_name = asset_dir_path.file_name().expect("failed to get file name");
    assert_eq!(asset_dir_file_name, "test_asset");

    // The next subdirectory in the asset directory should be the run name.
    let asset_dir_contents = fs::read_dir(asset_dir_path)
        .expect("failed to read asset directory")
        .collect::<Vec<_>>();
    assert_eq!(asset_dir_contents.len(), 1);

    let run_dir = asset_dir_contents[0].as_ref().expect("failed to get file");
    assert!(run_dir.path().is_dir(), "expected file to be a directory");

    let run_dir_path = run_dir.path();
    let run_dir_name = run_dir_path.file_name().expect("failed to get file name");
    assert_eq!(run_dir_name, "test_run");
}

#[tokio::test]
async fn test_sift_stream_builder_backup_manager_directory_naming_no_run() {
    let backups_dir = uuid::Uuid::new_v4().to_string();

    let tmp_dir = TempDir::new(&backups_dir).expect("failed to creat tempdir");
    let tmp_dir_path = tmp_dir.path();

    let ingestion_config = IngestionConfigForm {
        asset_name: "test_asset".to_string(),
        client_key: "test_client_key".to_string(),
        flows: vec![],
    };
    let disk_backup_policy = DiskBackupPolicy {
        backups_dir: Some(tmp_dir_path.to_path_buf()),
        retain_backups: true,
        ..Default::default()
    };
    let retry_policy = crate::RetryPolicy::default();
    let (grpc_channel, _mock_service) = crate::test::create_mock_grpc_channel_with_service().await;

    let mut sift_stream = SiftStreamBuilder::from_channel(grpc_channel)
        .ingestion_config(ingestion_config)
        .recovery_strategy(RecoveryStrategy::RetryWithBackups {
            retry_policy,
            disk_backup_policy,
        })
        .build()
        .await
        .expect("failed to build sift stream");

    for data in 0..100 {
        sift_stream
            .send(Flow::new(
                "some_flow",
                TimeValue::now(),
                &[ChannelValue::new("some_channel", data)],
            ))
            .await
            .expect("failed to send data to backup task");
    }

    // Finish the stream to ensure that the backup manager is shutdown and the backup files are processed.
    tokio::time::timeout(Duration::from_secs(10), async {
        assert!(
            sift_stream.finish().await.is_ok(),
            "failed to finish sift stream"
        );
    })
    .await
    .expect("timeout waiting for sift stream to finish");

    let test_dir = fs::read_dir(tmp_dir_path)
        .expect("failed to read backups directory")
        .collect::<Vec<_>>();
    assert_eq!(test_dir.len(), 1);

    // The first subdirectory should be the asset name.
    let asset_dir = test_dir[0].as_ref().expect("failed to get file");
    assert!(asset_dir.path().is_dir(), "expected file to be a directory");

    let asset_dir_path = asset_dir.path();
    let asset_dir_file_name = asset_dir_path.file_name().expect("failed to get file name");
    assert_eq!(asset_dir_file_name, "test_asset");

    // Since there was no run provided, there are no subdirectories in the asset directory.
    let asset_dir_contents = fs::read_dir(asset_dir_path)
        .expect("failed to read asset directory")
        .collect::<Vec<_>>();
    assert_eq!(asset_dir_contents.len(), 1);
    assert!(
        asset_dir_contents[0]
            .as_ref()
            .expect("failed to get file")
            .path()
            .is_file(),
        "expected to be a file",
    );
}
