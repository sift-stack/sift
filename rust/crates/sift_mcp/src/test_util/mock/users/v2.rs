use async_trait::async_trait;
use mockall::mock;
use sift_rs::users::v2::{
    GetUserRequest, GetUserResponse, ListActiveUsersRequest, ListActiveUsersResponse,
    ListUsersRequest, ListUsersResponse, UpdateUserOrganizationActiveRequest,
    UpdateUserOrganizationActiveResponse, user_service_server::UserService,
};
use tonic::{Request, Response, Status};

mock! {
    pub UserServiceImpl {}

    #[async_trait]
    impl UserService for UserServiceImpl {
        async fn update_user_organization_active(
            &self,
            request: Request<UpdateUserOrganizationActiveRequest>,
        ) -> std::result::Result<
            Response<UpdateUserOrganizationActiveResponse>,
            Status,
        >;
        async fn get_user(
            &self,
            request: Request<GetUserRequest>,
        ) -> std::result::Result<
            Response<GetUserResponse>,
            Status,
        >;
        async fn list_active_users(
            &self,
            request: Request<ListActiveUsersRequest>,
        ) -> std::result::Result<
            Response<ListActiveUsersResponse>,
            Status,
        >;
        async fn list_users(
            &self,
            request: Request<ListUsersRequest>,
        ) -> std::result::Result<
            Response<ListUsersResponse>,
            Status,
        >;
    }
}
