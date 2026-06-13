use async_trait::async_trait;
use mockall::mock;
use sift_rs::user_defined_functions::v1::{
    CheckUpdatableFieldsRequest, CheckUpdatableFieldsResponse, CreateUserDefinedFunctionRequest,
    CreateUserDefinedFunctionResponse, GetUserDefinedFunctionDependentsRequest,
    GetUserDefinedFunctionDependentsResponse, GetUserDefinedFunctionRequest,
    GetUserDefinedFunctionResponse, GetUserDefinedFunctionVersionRequest,
    GetUserDefinedFunctionVersionResponse, GetUserDefinedFunctionVersionsRequest,
    GetUserDefinedFunctionVersionsResponse, ListUserDefinedFunctionVersionsRequest,
    ListUserDefinedFunctionVersionsResponse, ListUserDefinedFunctionsRequest,
    ListUserDefinedFunctionsResponse, UpdateUserDefinedFunctionRequest,
    UpdateUserDefinedFunctionResponse, ValidateUserDefinedFunctionRequest,
    ValidateUserDefinedFunctionResponse,
    user_defined_function_service_server::UserDefinedFunctionService,
};
use tonic::{Request, Response, Status};

mock! {
    pub UserDefinedFunctionServiceImpl {}

    #[async_trait]
    impl UserDefinedFunctionService for UserDefinedFunctionServiceImpl {
        async fn get_user_defined_function(
            &self,
            request: Request<GetUserDefinedFunctionRequest>,
        ) -> std::result::Result<Response<GetUserDefinedFunctionResponse>, Status>;
        async fn get_user_defined_function_version(
            &self,
            request: Request<GetUserDefinedFunctionVersionRequest>,
        ) -> std::result::Result<Response<GetUserDefinedFunctionVersionResponse>, Status>;
        async fn get_user_defined_function_versions(
            &self,
            request: Request<GetUserDefinedFunctionVersionsRequest>,
        ) -> std::result::Result<Response<GetUserDefinedFunctionVersionsResponse>, Status>;
        async fn get_user_defined_function_dependents(
            &self,
            request: Request<GetUserDefinedFunctionDependentsRequest>,
        ) -> std::result::Result<Response<GetUserDefinedFunctionDependentsResponse>, Status>;
        async fn create_user_defined_function(
            &self,
            request: Request<CreateUserDefinedFunctionRequest>,
        ) -> std::result::Result<Response<CreateUserDefinedFunctionResponse>, Status>;
        async fn validate_user_defined_function(
            &self,
            request: Request<ValidateUserDefinedFunctionRequest>,
        ) -> std::result::Result<Response<ValidateUserDefinedFunctionResponse>, Status>;
        async fn update_user_defined_function(
            &self,
            request: Request<UpdateUserDefinedFunctionRequest>,
        ) -> std::result::Result<Response<UpdateUserDefinedFunctionResponse>, Status>;
        async fn check_updatable_fields(
            &self,
            request: Request<CheckUpdatableFieldsRequest>,
        ) -> std::result::Result<Response<CheckUpdatableFieldsResponse>, Status>;
        async fn list_user_defined_functions(
            &self,
            request: Request<ListUserDefinedFunctionsRequest>,
        ) -> std::result::Result<Response<ListUserDefinedFunctionsResponse>, Status>;
        async fn list_user_defined_function_versions(
            &self,
            request: Request<ListUserDefinedFunctionVersionsRequest>,
        ) -> std::result::Result<Response<ListUserDefinedFunctionVersionsResponse>, Status>;
    }
}
