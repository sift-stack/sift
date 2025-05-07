from __future__ import annotations
from typing import Optional, List, Union, cast, Dict
from sift.rule_evaluation.v1.rule_evaluation_pb2_grpc import RuleEvaluationServiceStub
from sift_py._internal.time import to_timestamp_pb
from sift_py.grpc.transport import SiftChannel
from datetime import datetime
from sift.rule_evaluation.v1.rule_evaluation_pb2 import (
    AssetsTimeRange,
    EvaluateRulesRequest,
    EvaluateRulesResponse,
    EvaluateRulesPreviewRequest,
    EvaluateRulesPreviewResponse,
    EvaluateRulesFromCurrentRuleVersions,
)
from sift_py.report_templates.config import ReportTemplateConfig
from sift_py.rule.config import RuleConfig
from sift_py.rule.service import RuleIdentifier, RuleService
from sift.common.type.v1.resource_identifier_pb2 import ClientKeys, NamedResources, Names, ResourceIdentifier, ResourceIdentifiers, Ids
from pathlib import Path

from sift_py.rule_evalutation.report import Report


class RuleEvaluationService:
    """
    A service for evaluating rules. Provides methods to evaluate rules and perform dry-run evaluations.
    """
    _channel: SiftChannel
    _rule_evaluation_stub: RuleEvaluationServiceStub
    _rule_service: RuleService

    def __init__(self, channel: SiftChannel):
        self._channel = channel
        self._rule_evaluation_stub = RuleEvaluationServiceStub(channel)
        self._rule_service = RuleService(channel)

    def evaluate_against_run(
        self,
        run_id: str,
        rules: Union[ReportTemplateConfig, List[RuleIdentifier], List[RuleConfig]],
        report_name: str = "",
    ) -> Report:
        """
        TODO
        """
        eval_kwargs = self._get_rules_kwargs(rules)

        req = EvaluateRulesRequest(
            report_name=report_name,
            run=ResourceIdentifier(id=run_id),
            **eval_kwargs,
        )

        res = cast(EvaluateRulesResponse, self._rule_evaluation_stub.EvaluateRules(req))
        return Report(self._channel, res.report_id)

    def evaluate_against_assets(
        self,
        asset_names: List[str],
        start_time: datetime,
        end_time: datetime,
        rules: Union[ReportTemplateConfig, List[RuleIdentifier], List[RuleConfig]],
        report_name: str = "",
    ) -> Report:
        """
        TODO
        """
        asset_time_range = AssetsTimeRange(
            assets=NamedResources(names=Names(names=asset_names)),
            start_time=to_timestamp_pb(start_time),
            end_time=to_timestamp_pb(end_time),
        )
        eval_kwargs = self._get_rules_kwargs(rules)

        req = EvaluateRulesRequest(
            report_name=report_name,
            assets=asset_time_range,
            **eval_kwargs,
        )

        res = cast(EvaluateRulesResponse, self._rule_evaluation_stub.EvaluateRules(req))
        return Report(self._channel, res.report_id)

    def evaluate_against_run_preview(
        self,
        run_id: str,
        rules: Union[ReportTemplateConfig, List[RuleIdentifier], List[RuleConfig]],
        report_name: str = "",
    ) -> EvaluateRulesPreviewResponse:
        """
        TODO
        """
        eval_kwargs = self._get_rules_kwargs(rules)

        req = EvaluateRulesPreviewRequest(
            run=ResourceIdentifier(id=run_id),
            **eval_kwargs,
        )

        return self._rule_evaluation_stub.EvaluateRulesPreview(req)

    def evaluate_external_rules(
        self,
        run_id: str,
        rules: List[RuleConfig],
        report_name: str = "",
    ) -> Report:
        """
        TODO
        """
        rule_ids = self._rule_service.create_external_rules(rules)
        return self.evaluate_against_run(run_id, rule_ids, report_name)

    def evaluate_external_rules_from_yaml(
        self,
        run_id: str,
        paths: List[Path],
        named_expressions: Optional[Dict[str, str]] = None,
        report_name: str = "",
    ) -> Report:
        """
        TODO
        """
        rule_ids = self._rule_service.create_external_rules_from_yaml(paths, named_expressions)
        return self.evaluate_against_run(run_id, rule_ids, report_name)

    def evaluate_external_rules_preview(
        self,
        run_id: str,
        rules: List[RuleConfig],
        report_name: str = "",
    ) -> EvaluateRulesResponse:
        rule_ids = self._rule_service.create_external_rules(rules)
        return self.evaluate_against_run_preview(run_id, rule_ids, report_name)

    def evaluate_external_rules_from_yaml_preview(
        self,
        run_id: str,
        paths: List[Path],
        named_expressions: Optional[Dict[str, str]] = None,
        report_name: str = "",
    ) -> EvaluateRulesResponse:
        """
        TODO
        """
        rule_ids = self._rule_service.create_external_rules_from_yaml(paths, named_expressions)
        return self.evaluate_against_run_preview(run_id, rule_ids, report_name)

    def _get_rules_kwargs(self, rules: Union[ReportTemplateConfig, List[RuleIdentifier], List[RuleConfig]]) -> dict:
        """Returns the keyword arguments for a EvalutateRules request based on the input type.
        Currently does not support evaluating rules from a specific version.

        Args:
            rules: Either the ReportTemplateConfig, list of RuleIdentifiers, or list of RuleConfigs.

        Returns:
            dict: The keyword argument.
        """
        if isinstance(rules, ReportTemplateConfig):
            if not rules.template_id:
                raise ValueError("Invalid report template")
            return {"report_template": ResourceIdentifier(id=rules.template_id)}
        else:
            if len(rules) == 0:
                raise ValueError("Rule set is empty")

            if isinstance(rules[0], RuleIdentifier):
                rule_ids = cast(List[RuleIdentifier], rules)

                return {
                    "rules": EvaluateRulesFromCurrentRuleVersions(
                        rules=ResourceIdentifiers(
                            ids=Ids(ids=[r.rule_id for r in rule_ids])
                        ),
                    ),
                }

            elif isinstance(rules[0], RuleConfig):
                rule_configs = cast(List[RuleConfig], rules)

                for config in rule_configs:
                    if not config.rule_client_key:
                        raise ValueError(f"Rule of name '{config.name}' requires a rule_client_key")

                return {
                    "rules": EvaluateRulesFromCurrentRuleVersions(
                        rules=ResourceIdentifiers(
                            client_keys=[ # type: ignore
                                r.rule_client_key for r in rule_configs
                            ]
                        ),
                    ),
                }

        raise ValueError("Invalid rules argument")