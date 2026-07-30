use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::ErrorCode;
use sift_rs::{
    common::r#type::v1::{Organization, User},
    me::v2::{GetMeResponse, me_service_server::MeServiceServer},
    users::v2::{
        ListActiveUsersResponse, ListUsersResponse, user_service_server::UserServiceServer,
    },
};
use sift_test_util::{
    grpc::memory_sift_channel,
    mock::{me::v2::MockMeServiceImpl, users::v2::MockUserServiceImpl},
};
use tokio::task::JoinHandle;
use tonic::{Response, Status, transport::Server};

use crate::{
    server::SiftMcpServer,
    tool::{common::test_support::structured_field, users::ListUsersParams},
};

fn user(id: &str, name: &str) -> User {
    User {
        user_id: id.into(),
        user_name: name.into(),
        ..Default::default()
    }
}

fn params(filter: &str) -> Parameters<ListUsersParams> {
    Parameters(ListUsersParams {
        filter: filter.into(),
        order_by: None,
        limit: None,
        include_inactive: None,
        me: None,
    })
}

fn me_params() -> Parameters<ListUsersParams> {
    Parameters(ListUsersParams {
        filter: String::new(),
        order_by: None,
        limit: None,
        include_inactive: None,
        me: Some(true),
    })
}

async fn server_with_mocks(
    user_mock: MockUserServiceImpl,
    me_mock: MockMeServiceImpl,
) -> (SiftMcpServer, JoinHandle<()>) {
    let (client, server) = tokio::io::duplex(1024);
    let channel = memory_sift_channel(client).await;

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(UserServiceServer::new(user_mock))
            .add_service(MeServiceServer::new(me_mock))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
            .unwrap();
    });

    (
        SiftMcpServer::new(channel, String::from("https://api.test.local"), false),
        handle,
    )
}

#[tokio::test]
async fn list_users_returns_single_page() {
    let mut user_mock = MockUserServiceImpl::new();
    user_mock
        .expect_list_active_users()
        .withf(|req| req.get_ref().filter == "name == \"jane@siftstack.com\"")
        .returning(|_| {
            Ok(Response::new(ListActiveUsersResponse {
                users: vec![user("u1", "jane@siftstack.com")],
                next_page_token: String::new(),
            }))
        });

    let (server, _h) = server_with_mocks(user_mock, MockMeServiceImpl::new()).await;

    let resp = server
        .list_users(params("name == \"jane@siftstack.com\""))
        .await
        .expect("list_users failed");

    let users = structured_field(resp, "users");
    assert_eq!(users.as_array().unwrap().len(), 1);
    assert_eq!(users[0]["userId"], "u1");
    assert_eq!(users[0]["userName"], "jane@siftstack.com");
}

#[tokio::test]
async fn list_users_include_inactive_uses_all_users_rpc() {
    let mut user_mock = MockUserServiceImpl::new();
    user_mock.expect_list_users().times(1).returning(|_| {
        Ok(Response::new(ListUsersResponse {
            users: vec![user("u2", "former@siftstack.com")],
            next_page_token: String::new(),
        }))
    });

    let (server, _h) = server_with_mocks(user_mock, MockMeServiceImpl::new()).await;

    let resp = server
        .list_users(Parameters(ListUsersParams {
            filter: String::new(),
            order_by: None,
            limit: None,
            include_inactive: Some(true),
            me: None,
        }))
        .await
        .expect("list_users failed");

    let users = structured_field(resp, "users");
    assert_eq!(users[0]["userId"], "u2");
}

#[tokio::test]
async fn list_users_propagates_grpc_error() {
    let mut user_mock = MockUserServiceImpl::new();
    user_mock
        .expect_list_active_users()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (server, _h) = server_with_mocks(user_mock, MockMeServiceImpl::new()).await;

    let err = server
        .list_users(params("nope"))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("bad filter"));
}

#[tokio::test]
async fn list_users_me_resolves_caller_via_get_me() {
    let mut me_mock = MockMeServiceImpl::new();
    me_mock.expect_get_me().times(1).returning(|_| {
        Ok(Response::new(GetMeResponse {
            user_id: "u7".into(),
            user_email: "liam@siftstack.com".into(),
            organizations: vec![Organization {
                organization_id: "org1".into(),
                organization_name: "Sift".into(),
                ..Default::default()
            }],
            is_admin: false,
            ..Default::default()
        }))
    });

    let (server, _h) = server_with_mocks(MockUserServiceImpl::new(), me_mock).await;

    let resp = server
        .list_users(me_params())
        .await
        .expect("list_users failed");

    let users = structured_field(resp, "users");
    assert_eq!(users.as_array().unwrap().len(), 1);
    assert_eq!(users[0]["userId"], "u7");
    assert_eq!(users[0]["userName"], "liam@siftstack.com");
    assert_eq!(users[0]["organizations"][0]["organizationId"], "org1");
}

#[tokio::test]
async fn list_users_me_ignores_paging_params() {
    let mut me_mock = MockMeServiceImpl::new();
    me_mock.expect_get_me().times(1).returning(|_| {
        Ok(Response::new(GetMeResponse {
            user_id: "u7".into(),
            user_email: "liam@siftstack.com".into(),
            ..Default::default()
        }))
    });

    let (server, _h) = server_with_mocks(MockUserServiceImpl::new(), me_mock).await;

    let resp = server
        .list_users(Parameters(ListUsersParams {
            filter: String::new(),
            order_by: Some("created_date desc".into()),
            limit: Some(50),
            include_inactive: Some(true),
            me: Some(true),
        }))
        .await
        .expect("list_users failed");

    let users = structured_field(resp, "users");
    assert_eq!(users.as_array().unwrap().len(), 1);
    assert_eq!(users[0]["userId"], "u7");
}

#[tokio::test]
async fn list_users_me_with_filter_is_invalid_params() {
    let (server, _h) =
        server_with_mocks(MockUserServiceImpl::new(), MockMeServiceImpl::new()).await;

    let err = server
        .list_users(Parameters(ListUsersParams {
            filter: "name == \"john@siftstack.com\"".into(),
            order_by: None,
            limit: None,
            include_inactive: None,
            me: Some(true),
        }))
        .await
        .expect_err("expected error");

    assert_eq!(err.code, ErrorCode::INVALID_PARAMS);
    assert!(err.message.contains("mutually exclusive"));
}

#[tokio::test]
async fn list_users_me_propagates_grpc_error() {
    let mut me_mock = MockMeServiceImpl::new();
    me_mock
        .expect_get_me()
        .returning(|_| Err(Status::unauthenticated("bad api key")));

    let (server, _h) = server_with_mocks(MockUserServiceImpl::new(), me_mock).await;

    let err = server
        .list_users(me_params())
        .await
        .expect_err("expected error");

    assert_eq!(
        err.data
            .as_ref()
            .and_then(|v| v.get("reason"))
            .and_then(|v| v.as_str()),
        Some("Unauthenticated")
    );
}
