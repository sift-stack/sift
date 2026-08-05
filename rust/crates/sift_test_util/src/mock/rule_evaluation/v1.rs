use async_trait::async_trait;
use mockall::mock;
use sift_rs::rule_evaluation::v1::{
    EvaluateRulesPreviewRequest, EvaluateRulesPreviewResponse, EvaluateRulesRequest,
    EvaluateRulesResponse, rule_evaluation_service_server::RuleEvaluationService,
};
use tonic::{Request, Response, Status};

mock! {
    pub RuleEvaluationServiceImpl {}

    #[async_trait]
    impl RuleEvaluationService for RuleEvaluationServiceImpl {
        async fn evaluate_rules(
            &self,
            request: Request<EvaluateRulesRequest>,
        ) -> std::result::Result<
            Response<EvaluateRulesResponse>,
            Status,
        >;
        async fn evaluate_rules_preview(
            &self,
            request: Request<EvaluateRulesPreviewRequest>,
        ) -> std::result::Result<
            Response<EvaluateRulesPreviewResponse>,
            Status,
        >;
    }
}
