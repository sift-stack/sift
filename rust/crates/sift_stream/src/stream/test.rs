use std::fs;
use std::time::Duration;

use crate::TimeValue;
use crate::backup::DiskBackupPolicy;
use crate::{
    ChannelValue, Flow, FlowBuilder, IngestionConfigForm, RecoveryStrategy, RunForm,
    SiftStreamBuilder,
};
use sift_rs::common::r#type::v1::ChannelDataType;
use sift_rs::ingestion_configs::v2::{ChannelConfig, FlowConfig};
use tempdir::TempDir;
use tracing_test::traced_test;

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
        .metrics_streaming_interval(None)
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
    assert_eq!(test_dir.len(), 1, "{:?}", test_dir);

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
        .metrics_streaming_interval(None)
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

#[tokio::test]
#[traced_test]
async fn test_sift_stream_drop_without_finish() {
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

    let sift_stream = SiftStreamBuilder::from_channel(grpc_channel)
        .ingestion_config(ingestion_config)
        .attach_run(run)
        .recovery_strategy(RecoveryStrategy::RetryWithBackups {
            retry_policy,
            disk_backup_policy,
        })
        .build()
        .await
        .expect("failed to build sift stream");

    drop(sift_stream);

    let final_check = async move {
        while !logs_contain("ingestion task shutting down")
            && !logs_contain("re-ingestion task shutting down")
            && !logs_contain("backup manager shutting down")
        {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    };

    tokio::time::timeout(Duration::from_secs(10), final_check)
        .await
        .expect("timeout waiting for tasks to shutdown");
}

#[tokio::test]
async fn test_sift_stream_builder_load_ingestion_config_with_no_flows() {
    let backups_dir = uuid::Uuid::new_v4().to_string();

    let tmp_dir = TempDir::new(&backups_dir).expect("failed to creat tempdir");
    let tmp_dir_path = tmp_dir.path();

    let ingestion_config = IngestionConfigForm {
        asset_name: "already_exists_asset".to_string(),
        client_key: "already_exists_client_key".to_string(),
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

    // The mock sift server should have returned 1 flow.
    let flows = sift_stream.get_flows();
    assert_eq!(flows.len(), 1);

    let existing_flow = FlowConfig {
        name: "already_exists_flow".to_string(),
        channels: vec![ChannelConfig {
            name: "channel1".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    };

    // Add the existing flow again to ensure it is not added again.
    assert!(sift_stream.add_new_flows(&[existing_flow]).await.is_ok());
    let flows = sift_stream.get_flows();
    assert_eq!(flows.len(), 1);

    sift_stream
        .finish()
        .await
        .expect("failed to finish sift stream");
}

#[tokio::test]
async fn test_sift_stream_builder_load_ingestion_config_with_flows() {
    let backups_dir = uuid::Uuid::new_v4().to_string();

    let tmp_dir = TempDir::new(&backups_dir).expect("failed to creat tempdir");
    let tmp_dir_path = tmp_dir.path();

    let existing_flow = FlowConfig {
        name: "already_exists_flow".to_string(),
        channels: vec![ChannelConfig {
            name: "channel1".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    };

    let ingestion_config = IngestionConfigForm {
        asset_name: "test_asset".to_string(),
        client_key: "test_client_key".to_string(),
        flows: vec![existing_flow.clone()],
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

    // The mock sift server should have returned 1 flow.
    let flows = sift_stream.get_flows();
    assert_eq!(flows.len(), 1);

    // Add the existing flow again to ensure it is not added again.
    assert!(sift_stream.add_new_flows(&[existing_flow]).await.is_ok());
    let flows = sift_stream.get_flows();
    assert_eq!(flows.len(), 1);
}

#[tokio::test]
async fn test_sift_stream_builder_load_ingestion_config_with_new_flows() {
    let backups_dir = uuid::Uuid::new_v4().to_string();

    let tmp_dir = TempDir::new(&backups_dir).expect("failed to creat tempdir");
    let tmp_dir_path = tmp_dir.path();

    let new_flow = FlowConfig {
        name: "new_flow".to_string(),
        channels: vec![ChannelConfig {
            name: "channel-new".to_string(),
            data_type: ChannelDataType::Uint32.into(),
            ..Default::default()
        }],
    };

    let ingestion_config = IngestionConfigForm {
        asset_name: "test_asset".to_string(),
        client_key: "test_client_key".to_string(),
        flows: vec![new_flow.clone()],
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

    // The mock sift server should have returned 1 flow.
    let flows = sift_stream.get_flows();
    assert_eq!(flows.len(), 1);

    // Add the existing flow again to ensure it is not added again.
    assert!(sift_stream.add_new_flows(&[new_flow]).await.is_ok());
    let flows = sift_stream.get_flows();
    assert_eq!(flows.len(), 1);

    // Add another new flow to ensure it is added.
    let new_flow2 = FlowConfig {
        name: "new_flow2".to_string(),
        channels: vec![ChannelConfig {
            name: "channel-new2".to_string(),
            data_type: ChannelDataType::Uint32.into(),
            ..Default::default()
        }],
    };
    assert!(sift_stream.add_new_flows(&[new_flow2]).await.is_ok());
    let flows = sift_stream.get_flows();
    assert_eq!(flows.len(), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn test_sift_stream_ingestion_and_backup_channels_fill_up() {
    let backups_dir = uuid::Uuid::new_v4().to_string();

    let tmp_dir = TempDir::new(&backups_dir).expect("failed to creat tempdir");
    let tmp_dir_path = tmp_dir.path();

    let existing_flow = FlowConfig {
        name: "already_exists_flow".to_string(),
        channels: vec![ChannelConfig {
            name: "channel1".to_string(),
            data_type: ChannelDataType::Double.into(),
            ..Default::default()
        }],
    };

    let ingestion_config = IngestionConfigForm {
        asset_name: "test_asset".to_string(),
        client_key: "test_client_key".to_string(),
        flows: vec![existing_flow],
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
        .metrics_streaming_interval(None)
        .ingestion_data_channel_capacity(1)
        .backup_data_channel_capacity(1)
        .build()
        .await
        .expect("failed to build sift stream");

    let descriptor = sift_stream
        .get_flow_descriptor("already_exists_flow")
        .expect("failed to get flow descriptor");

    // Send a burst of messages that will cause the ingestion and backup channels to fill up.
    //
    // Since this test is running in single-threded mode, and `send_requests_nonblocking` is not async,
    // sending all the messages should occur before the background tasks have a chance to run
    // and create space.
    for data in 0..100 {
        let mut builder = FlowBuilder::new(&descriptor);
        assert!(builder.set_with_key("channel1", data as f64).is_ok());

        assert!(
            sift_stream
                .send_requests(vec![builder.request(TimeValue::now())])
                .await
                .is_ok(),
            "failed to send request"
        );
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
}
