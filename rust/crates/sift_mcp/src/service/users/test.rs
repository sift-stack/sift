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

use super::UserService;
use crate::policy::RetryPolicy;
use crate::service::common::DEFAULT_LIMIT;

fn user(id: &str, name: &str) -> User {
    User {
        user_id: id.into(),
        user_name: name.into(),
        ..Default::default()
    }
}

async fn service_with_mock(mock: MockUserServiceImpl) -> (UserService, JoinHandle<()>) {
    service_with_mocks(mock, MockMeServiceImpl::new()).await
}

async fn service_with_me_mock(mock: MockMeServiceImpl) -> (UserService, JoinHandle<()>) {
    service_with_mocks(MockUserServiceImpl::new(), mock).await
}

/// Both proto services are registered on one in-memory server, matching how
/// `UserService` spans them in production.
async fn service_with_mocks(
    user_mock: MockUserServiceImpl,
    me_mock: MockMeServiceImpl,
) -> (UserService, JoinHandle<()>) {
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

    (UserService::new(channel, RetryPolicy::default()), handle)
}

#[tokio::test]
async fn list_users_returns_single_page() {
    let mut mock = MockUserServiceImpl::new();
    mock.expect_list_active_users()
        .times(1)
        .withf(|req| req.get_ref().filter == "name == \"jane@siftstack.com\"")
        .returning(|_| {
            Ok(Response::new(ListActiveUsersResponse {
                users: vec![user("u1", "jane@siftstack.com")],
                next_page_token: String::new(),
            }))
        });

    let (service, _h) = service_with_mock(mock).await;

    let users = service
        .list_users(
            "name == \"jane@siftstack.com\"".to_string(),
            None,
            None,
            false,
        )
        .await
        .expect("list_users failed");

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].user_id, "u1");
}

#[tokio::test]
async fn list_users_defaults_to_active_users_rpc() {
    let mut mock = MockUserServiceImpl::new();
    mock.expect_list_active_users().times(1).returning(|req| {
        // `organization_id` is not exposed as a tool parameter; the caller's
        // API key already scopes the listing.
        assert!(req.get_ref().organization_id.is_empty());
        Ok(Response::new(ListActiveUsersResponse {
            users: vec![user("u1", "jane@siftstack.com")],
            next_page_token: String::new(),
        }))
    });
    // No expectation on `list_users`: it must not be reached.

    let (service, _h) = service_with_mock(mock).await;

    let users = service
        .list_users(String::new(), None, None, false)
        .await
        .expect("list_users failed");

    assert_eq!(users.len(), 1);
}

#[tokio::test]
async fn list_users_include_inactive_uses_all_users_rpc() {
    let mut mock = MockUserServiceImpl::new();
    mock.expect_list_users().times(1).returning(|_| {
        Ok(Response::new(ListUsersResponse {
            users: vec![
                user("u1", "jane@siftstack.com"),
                user("u2", "former@siftstack.com"),
            ],
            next_page_token: String::new(),
        }))
    });
    // No expectation on `list_active_users`: it must not be reached.

    let (service, _h) = service_with_mock(mock).await;

    let users = service
        .list_users(String::new(), None, None, true)
        .await
        .expect("list_users failed");

    let ids: Vec<&str> = users.iter().map(|u| u.user_id.as_str()).collect();
    assert_eq!(ids, vec!["u1", "u2"]);
}

#[tokio::test]
async fn list_users_paginates_until_token_empty() {
    let mut mock = MockUserServiceImpl::new();
    mock.expect_list_active_users().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, DEFAULT_LIMIT);
        let (users, next) = match req.page_token.as_str() {
            "" => (vec![user("u1", "a@siftstack.com")], "page-2".to_string()),
            "page-2" => (vec![user("u2", "b@siftstack.com")], "page-3".to_string()),
            "page-3" => (vec![user("u3", "c@siftstack.com")], String::new()),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListActiveUsersResponse {
            users,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let users = service
        .list_users(String::new(), None, None, false)
        .await
        .expect("list_users failed");

    let ids: Vec<&str> = users.iter().map(|u| u.user_id.as_str()).collect();
    assert_eq!(ids, vec!["u1", "u2", "u3"]);
}

#[tokio::test]
async fn list_users_truncates_to_limit_across_pages() {
    let mut mock = MockUserServiceImpl::new();
    mock.expect_list_active_users().returning(|req| {
        let req = req.into_inner();
        assert_eq!(req.page_size, 3);
        let (users, next) = match req.page_token.as_str() {
            "" => (
                vec![user("u1", "a@siftstack.com"), user("u2", "b@siftstack.com")],
                "page-2".to_string(),
            ),
            "page-2" => (
                vec![user("u3", "c@siftstack.com"), user("u4", "d@siftstack.com")],
                String::new(),
            ),
            other => return Err(Status::invalid_argument(format!("bad token: {other}"))),
        };
        Ok(Response::new(ListActiveUsersResponse {
            users,
            next_page_token: next,
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let users = service
        .list_users(String::new(), None, Some(3), false)
        .await
        .expect("list_users failed");

    let ids: Vec<&str> = users.iter().map(|u| u.user_id.as_str()).collect();
    assert_eq!(ids, vec!["u1", "u2", "u3"]);
}

#[tokio::test]
async fn list_users_breaks_on_empty_page() {
    let mut mock = MockUserServiceImpl::new();
    mock.expect_list_active_users().times(1).returning(|_| {
        Ok(Response::new(ListActiveUsersResponse {
            users: vec![],
            next_page_token: "ignored".into(),
        }))
    });

    let (service, _h) = service_with_mock(mock).await;

    let users = service
        .list_users(String::new(), None, None, false)
        .await
        .expect("list_users failed");

    assert!(users.is_empty());
}

#[tokio::test]
async fn list_users_propagates_grpc_error() {
    let mut mock = MockUserServiceImpl::new();
    mock.expect_list_active_users()
        .returning(|_| Err(Status::invalid_argument("bad filter")));

    let (service, _h) = service_with_mock(mock).await;

    let err = service
        .list_users("nope".to_string(), None, None, false)
        .await
        .expect_err("expected error");

    assert!(err.to_string().contains("failed to query users"));
}

#[tokio::test]
async fn get_me_maps_email_onto_user_name() {
    let mut mock = MockMeServiceImpl::new();
    mock.expect_get_me().times(1).returning(|_| {
        Ok(Response::new(GetMeResponse {
            user_id: "u1".into(),
            user_email: "jane@siftstack.com".into(),
            organizations: vec![Organization {
                organization_id: "org1".into(),
                organization_name: "Sift".into(),
                ..Default::default()
            }],
            is_admin: true,
            ..Default::default()
        }))
    });

    let (service, _h) = service_with_me_mock(mock).await;

    let user = service.get_me().await.expect("get_me failed");

    assert_eq!(user.user_id, "u1");
    assert_eq!(user.user_name, "jane@siftstack.com");
    assert_eq!(user.organizations.len(), 1);
    assert_eq!(user.organizations[0].organization_id, "org1");
}

#[tokio::test]
async fn get_me_sends_an_empty_request() {
    // Identity comes from the API key on the channel, so the request carries
    // nothing.
    let mut mock = MockMeServiceImpl::new();
    mock.expect_get_me().times(1).returning(|req| {
        let _: sift_rs::me::v2::GetMeRequest = req.into_inner();
        Ok(Response::new(GetMeResponse {
            user_id: "u1".into(),
            user_email: "jane@siftstack.com".into(),
            ..Default::default()
        }))
    });

    let (service, _h) = service_with_me_mock(mock).await;

    let user = service.get_me().await.expect("get_me failed");

    assert_eq!(user.user_id, "u1");
    assert!(user.organizations.is_empty());
}

#[tokio::test]
async fn get_me_propagates_grpc_error() {
    let mut mock = MockMeServiceImpl::new();
    mock.expect_get_me()
        .returning(|_| Err(Status::unauthenticated("bad api key")));

    let (service, _h) = service_with_me_mock(mock).await;

    let err = service.get_me().await.expect_err("expected error");

    assert!(
        err.to_string()
            .contains("failed to resolve the calling user")
    );
}
