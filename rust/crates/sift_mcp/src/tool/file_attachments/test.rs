use rmcp::{handler::server::wrapper::Parameters, model::ErrorCode};
use serde_json::json;
use sift_rs::remote_files::v1::{
    EntityType, GetRemoteFileDownloadUrlResponse, ListRemoteFilesResponse, RemoteFile,
    remote_file_service_server::RemoteFileServiceServer,
};
use sift_test_util::{
    grpc::memory_sift_channel, mock::remote_files::v1::MockRemoteFileServiceImpl,
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use super::{DownloadFileAttachmentParams, ListFileAttachmentsParams, parse_entity_type};
use crate::{
    server::SiftMcpServer, service::remote_files::AttachmentEntityType,
    tool::common::test_support::structured,
};

const ATTACHMENT_TYPES: &str = "(entity_type == \"runs\" || entity_type == \"assets\" || \
     entity_type == \"annotations\" || entity_type == \"annotation_logs\" || \
     entity_type == \"test_reports\" || entity_type == \"test_steps\")";

async fn server_with_mock(mock: MockRemoteFileServiceImpl) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(RemoteFileServiceServer::new(mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(
            channel,
            String::from("https://app.test.local"),
            false,
            false,
        ),
        handle,
    )
}

fn list_params(
    entity_ids: Option<&[&str]>,
    entity_type: Option<&str>,
    filter: &str,
) -> Parameters<ListFileAttachmentsParams> {
    Parameters(ListFileAttachmentsParams {
        entity_ids: entity_ids.map(|ids| ids.iter().map(|id| (*id).to_string()).collect()),
        entity_type: entity_type.map(str::to_string),
        filter: filter.into(),
        limit: None,
        fields: None,
    })
}

#[test]
fn entity_type_accepts_singular_plural_and_proto_names() {
    for value in ["run", "runs", "RUN", " Run ", "ENTITY_TYPE_RUN"] {
        assert_eq!(
            parse_entity_type(Some(value.into())).unwrap(),
            Some(AttachmentEntityType::Run)
        );
    }
    for value in [
        "annotation_log",
        "annotation_logs",
        "ENTITY_TYPE_ANNOTATION_LOG",
    ] {
        assert_eq!(
            parse_entity_type(Some(value.into())).unwrap(),
            Some(AttachmentEntityType::AnnotationLog)
        );
    }
    for entity_type in AttachmentEntityType::ALL {
        assert_eq!(
            parse_entity_type(Some(entity_type.name().into())).unwrap(),
            Some(entity_type)
        );
    }
    for value in [None, Some(String::new()), Some("  ".into())] {
        assert_eq!(parse_entity_type(value).unwrap(), None);
    }
}

#[test]
fn entity_type_rejects_non_attachment_types() {
    for value in ["artifact_versions", "chat_conversations", "canvas_version"] {
        let err = parse_entity_type(Some(value.into())).unwrap_err();
        assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    }
}

#[tokio::test]
async fn list_file_attachments_sends_the_scoped_filter() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_list_remote_files()
        .withf(|req| {
            req.get_ref().filter
                == "entity_id in [\"run-1\"] && entity_type == \"runs\" && (file_name.matches(\"(?i)log\"))"
        })
        .times(1)
        .returning(|_| {
            Ok(Response::new(ListRemoteFilesResponse {
                remote_files: vec![RemoteFile {
                    remote_file_id: "rf-1".into(),
                    entity_id: "run-1".into(),
                    entity_type: EntityType::Run as i32,
                    file_name: "flight_log.csv".into(),
                    file_size: 42,
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;
    let result = server
        .list_file_attachments(list_params(
            Some(&["run-1"]),
            Some("run"),
            "file_name.matches(\"(?i)log\")",
        ))
        .await
        .expect("list");

    let body = structured(result);
    assert_eq!(body["count"], json!(1));
    assert_eq!(body["has_more"], json!(false));
    let file = &body["file_attachments"][0];
    assert_eq!(file["remoteFileId"], json!("rf-1"));
    assert_eq!(file["entityType"], json!("ENTITY_TYPE_RUN"));
    assert_eq!(file["fileName"], json!("flight_log.csv"));
}

#[tokio::test]
async fn list_file_attachments_projects_fields() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_list_remote_files()
        .withf(|req| req.get_ref().filter == ATTACHMENT_TYPES)
        .returning(|_| {
            Ok(Response::new(ListRemoteFilesResponse {
                remote_files: vec![RemoteFile {
                    remote_file_id: "rf-1".into(),
                    file_name: "notes.pdf".into(),
                    ..Default::default()
                }],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;
    let mut params = list_params(None, None, "");
    params.0.fields = Some(vec!["file_name".into()]);
    let body = structured(server.list_file_attachments(params).await.expect("list"));
    assert_eq!(
        body["file_attachments"],
        json!([{ "fileName": "notes.pdf" }])
    );
}

#[tokio::test]
async fn list_file_attachments_rejects_bad_scope_before_calling_the_service() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_list_remote_files().never();

    let (server, _h) = server_with_mock(mock).await;
    let err = server
        .list_file_attachments(list_params(Some(&["run-1", " "]), None, ""))
        .await
        .expect_err("empty id");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);

    // An empty array must not widen the listing to the whole organization.
    let err = server
        .list_file_attachments(list_params(Some(&[]), None, ""))
        .await
        .expect_err("empty entity_ids");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);

    let err = server
        .list_file_attachments(list_params(None, Some("artifact"), ""))
        .await
        .expect_err("unknown type");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn list_file_attachments_maps_an_invalid_filter() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_list_remote_files()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mock(mock).await;
    let err = server
        .list_file_attachments(list_params(None, None, "file_name =="))
        .await
        .expect_err("invalid filter");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn download_file_attachment_returns_the_signed_url() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_get_remote_file_download_url()
        .withf(|req| req.get_ref().remote_file_id == "rf-1")
        .times(1)
        .returning(|_| {
            Ok(Response::new(GetRemoteFileDownloadUrlResponse {
                download_url: "https://files.test.local/rf-1?sig=abc".into(),
            }))
        });

    let (server, _h) = server_with_mock(mock).await;
    let result = server
        .download_file_attachment(Parameters(DownloadFileAttachmentParams {
            remote_file_id: " rf-1 ".into(),
        }))
        .await
        .expect("download");
    assert_eq!(
        structured(result),
        json!({
            "remote_file_id": "rf-1",
            "download_url": "https://files.test.local/rf-1?sig=abc",
        })
    );
}

#[tokio::test]
async fn download_file_attachment_rejects_an_empty_id() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_get_remote_file_download_url().never();

    let (server, _h) = server_with_mock(mock).await;
    let err = server
        .download_file_attachment(Parameters(DownloadFileAttachmentParams {
            remote_file_id: "  ".into(),
        }))
        .await
        .expect_err("empty id");
    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
}

#[tokio::test]
async fn download_file_attachment_maps_not_found() {
    let mut mock = MockRemoteFileServiceImpl::new();
    mock.expect_get_remote_file_download_url()
        .returning(|_| Err(Status::not_found("remote file not found")));

    let (server, _h) = server_with_mock(mock).await;
    let err = server
        .download_file_attachment(Parameters(DownloadFileAttachmentParams {
            remote_file_id: "missing".into(),
        }))
        .await
        .expect_err("not found");
    assert_eq!(err.code, ErrorCode::RESOURCE_NOT_FOUND);
}
