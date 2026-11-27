use async_trait::async_trait;
use mockall::mock;
use sift_rs::rules::v1::{
    ArchiveRuleRequest, ArchiveRuleResponse, BatchArchiveRulesRequest, BatchArchiveRulesResponse,
    BatchDeleteRulesRequest, BatchDeleteRulesResponse, BatchGetRuleVersionsRequest,
    BatchGetRuleVersionsResponse, BatchGetRulesRequest, BatchGetRulesResponse,
    BatchUnarchiveRulesRequest, BatchUnarchiveRulesResponse, BatchUndeleteRulesRequest,
    BatchUndeleteRulesResponse, BatchUpdateRulesRequest, BatchUpdateRulesResponse,
    CreateRuleRequest, CreateRuleResponse, DeleteRuleRequest, DeleteRuleResponse,
    EvaluateRulesRequest, EvaluateRulesResponse, GetRuleRequest, GetRuleResponse,
    GetRuleVersionRequest, GetRuleVersionResponse, ListRuleVersionsRequest,
    ListRuleVersionsResponse, ListRulesRequest, ListRulesResponse, SearchRulesRequest,
    SearchRulesResponse, UnarchiveRuleRequest, UnarchiveRuleResponse, UndeleteRuleRequest,
    UndeleteRuleResponse, UpdateHumanFriendlyRulesRequest, UpdateHumanFriendlyRulesResponse,
    UpdateJsonRulesRequest, UpdateJsonRulesResponse, UpdateRuleRequest, UpdateRuleResponse,
    ValidateJsonRulesRequest, ValidateJsonRulesResponse, ViewHumanFriendlyRulesRequest,
    ViewHumanFriendlyRulesResponse, ViewJsonRulesRequest, ViewJsonRulesResponse,
    rule_service_server::RuleService,
};
use tonic::{Request, Response, Status};

mock! {
    pub RuleServiceImpl {}

    #[async_trait]
    impl RuleService for RuleServiceImpl {
        async fn search_rules(
            &self,
            request: Request<SearchRulesRequest>,
        ) -> std::result::Result<
            Response<SearchRulesResponse>,
            Status,
        >;
        async fn get_rule(
            &self,
            request: Request<GetRuleRequest>,
        ) -> std::result::Result<
            Response<GetRuleResponse>,
            Status,
        >;
        async fn batch_get_rules(
            &self,
            request: Request<BatchGetRulesRequest>,
        ) -> std::result::Result<
            Response<BatchGetRulesResponse>,
            Status,
        >;
        async fn create_rule(
            &self,
            request: Request<CreateRuleRequest>,
        ) -> std::result::Result<
            Response<CreateRuleResponse>,
            Status,
        >;
        async fn update_rule(
            &self,
            request: Request<UpdateRuleRequest>,
        ) -> std::result::Result<
            Response<UpdateRuleResponse>,
            Status,
        >;
        async fn batch_update_rules(
            &self,
            request: Request<BatchUpdateRulesRequest>,
        ) -> std::result::Result<
            Response<BatchUpdateRulesResponse>,
            Status,
        >;
        async fn delete_rule(
            &self,
            request: Request<DeleteRuleRequest>,
        ) -> std::result::Result<
            Response<DeleteRuleResponse>,
            Status,
        >;
        async fn archive_rule(
            &self,
            request: Request<ArchiveRuleRequest>,
        ) -> std::result::Result<
            Response<ArchiveRuleResponse>,
            Status,
        >;
        async fn batch_delete_rules(
            &self,
            request: Request<BatchDeleteRulesRequest>,
        ) -> std::result::Result<
            Response<BatchDeleteRulesResponse>,
            Status,
        >;
        async fn batch_archive_rules(
            &self,
            request: Request<BatchArchiveRulesRequest>,
        ) -> std::result::Result<
            Response<BatchArchiveRulesResponse>,
            Status,
        >;
        async fn unarchive_rule(
            &self,
            request: Request<UnarchiveRuleRequest>,
        ) -> std::result::Result<
            Response<UnarchiveRuleResponse>,
            Status,
        >;
        async fn batch_unarchive_rules(
            &self,
            request: Request<BatchUnarchiveRulesRequest>,
        ) -> std::result::Result<
            Response<BatchUnarchiveRulesResponse>,
            Status,
        >;
        async fn undelete_rule(
            &self,
            request: Request<UndeleteRuleRequest>,
        ) -> std::result::Result<
            Response<UndeleteRuleResponse>,
            Status,
        >;
        async fn batch_undelete_rules(
            &self,
            request: Request<BatchUndeleteRulesRequest>,
        ) -> std::result::Result<
            Response<BatchUndeleteRulesResponse>,
            Status,
        >;
        async fn evaluate_rules(
            &self,
            request: Request<EvaluateRulesRequest>,
        ) -> std::result::Result<
            Response<EvaluateRulesResponse>,
            Status,
        >;
        async fn view_human_friendly_rules(
            &self,
            request: Request<ViewHumanFriendlyRulesRequest>,
        ) -> std::result::Result<
            Response<ViewHumanFriendlyRulesResponse>,
            Status,
        >;
        async fn view_json_rules(
            &self,
            request: Request<ViewJsonRulesRequest>,
        ) -> std::result::Result<
            Response<ViewJsonRulesResponse>,
            Status,
        >;
        async fn update_human_friendly_rules(
            &self,
            request: Request<UpdateHumanFriendlyRulesRequest>,
        ) -> std::result::Result<
            Response<UpdateHumanFriendlyRulesResponse>,
            Status,
        >;
        async fn validate_json_rules(
            &self,
            request: Request<ValidateJsonRulesRequest>,
        ) -> std::result::Result<
            Response<ValidateJsonRulesResponse>,
            Status,
        >;
        async fn update_json_rules(
            &self,
            request: Request<UpdateJsonRulesRequest>,
        ) -> std::result::Result<
            Response<UpdateJsonRulesResponse>,
            Status,
        >;
        async fn list_rules(
            &self,
            request: Request<ListRulesRequest>,
        ) -> std::result::Result<
            Response<ListRulesResponse>,
            Status,
        >;
        async fn list_rule_versions(
            &self,
            request: Request<ListRuleVersionsRequest>,
        ) -> std::result::Result<
            Response<ListRuleVersionsResponse>,
            Status,
        >;
        async fn get_rule_version(
            &self,
            request: Request<GetRuleVersionRequest>,
        ) -> std::result::Result<
            Response<GetRuleVersionResponse>,
            Status,
        >;
        async fn batch_get_rule_versions(
            &self,
            request: Request<BatchGetRuleVersionsRequest>,
        ) -> std::result::Result<
            Response<BatchGetRuleVersionsResponse>,
            Status,
        >;
    }
}
