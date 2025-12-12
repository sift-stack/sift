from __future__ import annotations

from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Union, cast

from sift.common.type.v1.resource_identifier_pb2 import (
    ClientKeys,
    Ids,
    NamedResources,
    Names,
    ResourceIdentifier,
    ResourceIdentifiers,
)
from sift.rule_evaluation.v1.rule_evaluation_pb2 import (
    AssetsTimeRange,
    EvaluateRulesFromCurrentRuleVersions,
    EvaluateRulesFromReportTemplate,
    EvaluateRulesPreviewRequest,
    EvaluateRulesPreviewResponse,
    EvaluateRulesRequest,
    EvaluateRulesResponse,
    RunTimeRange,
)
from sift.rule_evaluation.v1.rule_evaluation_pb2_grpc import RuleEvaluationServiceStub
from sift_py._internal.time import to_timestamp_pb
from sift_py.grpc.transport import SiftChannel
from sift_py.report.service import ReportService
from sift_py.report_templates.config import ReportTemplateConfig
from sift_py.rule.config import RuleConfig
from sift_py.rule.service import RuleIdentifier, RuleService


class RuleEvaluationService:
    """
    A service for evaluating rules. Provides methods to evaluate rules and perform dry-run evaluations.

    Args:
        enable_caching: Enable caching during rule evaluation. This is enabled by default.
            This service is typically short lived in a workflows so assets, channels, and
            users are unlikely to change during its lifetime to invalidate caches.
    """

    _channel: SiftChannel
    _rule_evaluation_stub: RuleEvaluationServiceStub
    _rule_service: RuleService

    def __init__(self, channel: SiftChannel, enable_caching: bool = True):
        self._channel = channel
        self._rule_evaluation_stub = RuleEvaluationServiceStub(channel)
        self._rule_service = RuleService(channel, enable_caching=enable_caching)

    def evaluate_against_run(
        self,
        run_id: str,
        rules: Union[ReportTemplateConfig, List[RuleConfig], List[RuleIdentifier]],
        report_name: str = "",
        start_time: Optional[Union[datetime, str, int, float]] = None,
        end_time: Optional[Union[datetime, str, int, float]] = None,
    ) -> ReportService:
        """Evaluate a set of rules against a run.

        Args:
            run_id: The Run ID to run against.
            rules: Either a ReportTemplateConfig, a list of RuleConfigs, or a list of
                RuleIdentifiers (typically from `RuleService.create_external_rules`).
            report_name: Optional report name.
            start_time: Optional start time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).
            end_time: Optional end time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).

        Returns:
            A ReportService object that can be use to get the status of the executed report.
        """
        rules_kwargs = self._get_rules_kwargs(rules)
        run_kwargs = self._get_run_kwargs(run_id, start_time, end_time)

        req = EvaluateRulesRequest(
            report_name=report_name,
            **rules_kwargs,
            **run_kwargs,
        )
        res = cast(EvaluateRulesResponse, self._rule_evaluation_stub.EvaluateRules(req))

        return ReportService(self._channel, res.report_id)

    def evaluate_against_assets(
        self,
        asset_names: List[str],
        start_time: Union[datetime, str, int, float],
        end_time: Union[datetime, str, int, float],
        rules: Union[ReportTemplateConfig, List[RuleConfig], List[RuleIdentifier]],
        report_name: str = "",
    ) -> ReportService:
        """Evaluate a set of rules against assets.

        Args:
            asset_names: The list of assets to run against.
            start_time: The start time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).
            end_time: The end time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).
            rules: Either a ReportTemplateConfig, a list of RuleConfigs, or a list of
                RuleIdentifiers (typically from `RuleService.create_external_rules`).
            report_name: Optional report name.

        Returns:
            A Report object that can be use to get the status of the executed report.
        """
        asset_time_range = AssetsTimeRange(
            assets=NamedResources(names=Names(names=asset_names)),
            start_time=to_timestamp_pb(start_time),
            end_time=to_timestamp_pb(end_time),
        )
        rules_kwargs = self._get_rules_kwargs(rules)

        req = EvaluateRulesRequest(
            report_name=report_name,
            assets=asset_time_range,
            **rules_kwargs,
        )
        res = cast(EvaluateRulesResponse, self._rule_evaluation_stub.EvaluateRules(req))

        return ReportService(self._channel, res.report_id)

    def preview_against_run(
        self,
        run_id: str,
        rules: Union[ReportTemplateConfig, List[RuleConfig], List[RuleIdentifier]],
        start_time: Optional[Union[datetime, str, int, float]] = None,
        end_time: Optional[Union[datetime, str, int, float]] = None,
    ) -> EvaluateRulesPreviewResponse:
        """Preview the evaluation of a set of rules against a run.

        Args:
            run_id: The Run ID to run against.
            rules: Either a ReportTemplateConfig, a list of RuleConfigs, or a list of
                RuleIdentifiers (typically from `RuleService.create_external_rules`).
            start_time: Optional start time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).
            end_time: Optional end time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).

        Returns:
            The EvaluateRulesPreviewResponse object.
        """
        eval_kwargs = self._get_rules_kwargs(rules)
        run_kwargs = self._get_run_kwargs(run_id, start_time, end_time)

        req = EvaluateRulesPreviewRequest(
            **eval_kwargs,
            **run_kwargs,
        )

        return self._rule_evaluation_stub.EvaluateRulesPreview(req)

    def evaluate_external_rules(
        self,
        run_id: str,
        rules: List[RuleConfig],
        report_name: str = "",
        start_time: Optional[Union[datetime, str, int, float]] = None,
        end_time: Optional[Union[datetime, str, int, float]] = None,
    ) -> ReportService:
        """Evaluate a set of external rules against a run.

        Args:
            run_id: The Run ID to run against.
            rules: A list of RuleConfigs. These must be external rules.
            report_name: Optional report name.
            start_time: Optional start time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).
            end_time: Optional end time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).

        Returns:
            A Report object that can be use to get the status of the executed report.
        """
        rule_ids = self._rule_service.create_external_rules(rules)
        return self.evaluate_against_run(run_id, rule_ids, report_name, start_time, end_time)

    def evaluate_external_rules_from_yaml(
        self,
        run_id: str,
        paths: List[Path],
        named_expressions: Optional[Dict[str, str]] = None,
        report_name: str = "",
        start_time: Optional[Union[datetime, str, int, float]] = None,
        end_time: Optional[Union[datetime, str, int, float]] = None,
    ) -> ReportService:
        """Evaluate a set of external rules from a YAML config against a run.

        Args:
            run_id: The Run ID to run against.
            paths: The YAML paths to load rules from.
            report_name: Optional report name.
            start_time: Optional start time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).
            end_time: Optional end time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).

        Returns:
            A Report object that can be use to get the status of the executed report.
        """
        rule_ids = self._rule_service.create_external_rules_from_yaml(paths, named_expressions)
        return self.evaluate_against_run(run_id, rule_ids, report_name, start_time, end_time)

    def preview_external_rules(
        self,
        run_id: str,
        rules: List[RuleConfig],
        start_time: Optional[Union[datetime, str, int, float]] = None,
        end_time: Optional[Union[datetime, str, int, float]] = None,
    ) -> EvaluateRulesPreviewResponse:
        """Preview the evaluation a set of external rules against a run.

        Args:
            run_id: The Run ID to run against.
            rules: A list of RuleConfigs. These must be external rules.
            start_time: Optional start time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).
            end_time: Optional end time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).

        Returns:
            The EvaluateRulesPreviewResponse object.
        """
        rule_ids = self._rule_service.create_external_rules(rules)
        return self.preview_against_run(run_id, rule_ids, start_time, end_time)

    def preview_external_rules_from_yaml(
        self,
        run_id: str,
        paths: List[Path],
        named_expressions: Optional[Dict[str, str]] = None,
        start_time: Optional[Union[datetime, str, int, float]] = None,
        end_time: Optional[Union[datetime, str, int, float]] = None,
    ) -> EvaluateRulesPreviewResponse:
        """Preview the evaluation a set of external rules from a YAML config against a run.

        Args:
            run_id: The Run ID to run against.
            paths: The YAML paths to load rules from.
            named_expressions: The named expressions to substitute in the rules.
            start_time: Optional start time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).
            end_time: Optional end time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).

        Returns:
            The EvaluateRulesPreviewResponse object.
        """
        rule_ids = self._rule_service.create_external_rules_from_yaml(paths, named_expressions)
        return self.preview_against_run(run_id, rule_ids, start_time, end_time)

    def _get_rules_kwargs(
        self, rules: Union[ReportTemplateConfig, List[RuleConfig], List[RuleIdentifier]]
    ) -> dict:
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
            return {
                "report_template": EvaluateRulesFromReportTemplate(
                    report_template=ResourceIdentifier(id=rules.template_id)
                )
            }
        else:
            if len(rules) == 0:
                raise ValueError("Rule set is empty")

            if isinstance(rules[0], RuleIdentifier):
                rule_ids = cast(List[RuleIdentifier], rules)

                return {
                    "rules": EvaluateRulesFromCurrentRuleVersions(
                        rules=ResourceIdentifiers(ids=Ids(ids=[r.rule_id for r in rule_ids])),
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
                            client_keys=ClientKeys(
                                client_keys=[r.rule_client_key for r in rule_configs]  # type: ignore
                            ),
                        ),
                    ),
                }

        raise ValueError("Invalid rules argument")

    def _get_run_kwargs(
        self,
        run_id: str,
        start_time: Optional[Union[datetime, str, int, float]] = None,
        end_time: Optional[Union[datetime, str, int, float]] = None,
    ) -> dict:
        """Returns the Run specific keyword arguments for a EvalutateRules request based on the input type.

        Args:
            run_id: The Run ID to run against.
            start_time: Optional start time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).
            end_time: Optional end time to evaluate (datetime, ISO 8601 formatted string, or POSIX timestamp).

        Returns:
            dict: The keyword arguments.
        """
        run = ResourceIdentifier(id=run_id)

        if start_time or end_time:
            return {
                "run_time_range": RunTimeRange(
                    run=run,
                    start_time=to_timestamp_pb(start_time) if start_time else None,
                    end_time=to_timestamp_pb(end_time) if end_time else None,
                )
            }
        else:
            return {"run": run}
