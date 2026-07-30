use crate::policy::{RetryPolicy, with_retry};
use crate::service::common;
use anyhow::{Context, Result};
use sift_rs::{
    SiftChannel,
    common::r#type::v1::User,
    me::v2::{GetMeRequest, GetMeResponse, me_service_client::MeServiceClient},
    users::v2::{
        ListActiveUsersRequest, ListActiveUsersResponse, ListUsersRequest, ListUsersResponse,
        user_service_client::UserServiceClient,
    },
};

#[cfg(test)]
mod test;

/// Spans two proto packages: `sift.users.v2` for listing users and
/// `sift.me.v2` for resolving the caller. Both answer questions about users,
/// and `list_users` serves them from one tool.
#[derive(Clone)]
pub struct UserService {
    channel: SiftChannel,
    policy: RetryPolicy,
}

impl UserService {
    pub fn new(channel: SiftChannel, policy: RetryPolicy) -> Self {
        Self { channel, policy }
    }

    /// `include_inactive` selects the RPC: `ListActiveUsers` by default,
    /// `ListUsers` when set, which also returns deactivated accounts.
    pub async fn list_users(
        &self,
        filter: String,
        order_by: Option<String>,
        limit: Option<u32>,
        include_inactive: bool,
    ) -> Result<Vec<User>> {
        let (page_size, record_limit) = common::paging(limit);

        let mut page_token = String::new();
        let mut results = Vec::new();

        let order_by = order_by.unwrap_or_default();

        loop {
            let channel = self.channel.clone();
            let filter = filter.clone();
            let order_by = order_by.clone();
            let token = page_token.clone();

            let (users, next_page_token) = with_retry(&self.policy, move || {
                let channel = channel.clone();
                let filter = filter.clone();
                let order_by = order_by.clone();
                let token = token.clone();
                async move {
                    let mut client = UserServiceClient::new(channel);
                    if include_inactive {
                        client
                            .list_users(ListUsersRequest {
                                filter,
                                page_size,
                                page_token: token,
                                order_by,
                            })
                            .await
                            .map(|resp| {
                                let ListUsersResponse {
                                    users,
                                    next_page_token,
                                } = resp.into_inner();
                                (users, next_page_token)
                            })
                    } else {
                        client
                            .list_active_users(ListActiveUsersRequest {
                                filter,
                                page_size,
                                page_token: token,
                                order_by,
                                organization_id: String::new(),
                            })
                            .await
                            .map(|resp| {
                                let ListActiveUsersResponse {
                                    users,
                                    next_page_token,
                                } = resp.into_inner();
                                (users, next_page_token)
                            })
                    }
                }
            })
            .await
            .context("failed to query users")?;

            if users.is_empty() {
                break;
            }
            results.extend(users);

            if results.len() >= record_limit || next_page_token.is_empty() {
                break;
            }
            page_token = next_page_token;
        }

        results.truncate(record_limit);

        Ok(results)
    }

    /// Resolve the caller via `sift.me.v2 GetMe`, which takes no arguments —
    /// the backend derives the user from the API key on the channel.
    ///
    /// Returned as a [`User`] so `list_users` emits one row shape either way.
    /// `user_email` maps onto `user_name`, the sign-in identifier on the list
    /// side; `is_admin`, `permissions`, and `created_date` are dropped as
    /// authorization details no tool acts on.
    pub async fn get_me(&self) -> Result<User> {
        let channel = self.channel.clone();

        let resp = with_retry(&self.policy, move || {
            let channel = channel.clone();
            async move {
                let mut client = MeServiceClient::new(channel);
                client
                    .get_me(GetMeRequest {})
                    .await
                    .map(|resp| resp.into_inner())
            }
        })
        .await
        .context("failed to resolve the calling user")?;

        let GetMeResponse {
            user_id,
            user_email,
            organizations,
            ..
        } = resp;

        Ok(User {
            user_id,
            user_name: user_email,
            organizations,
        })
    }
}
