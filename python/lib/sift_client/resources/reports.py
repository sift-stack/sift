from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.reports import ReportsLowLevelClient
from sift_client._internal.low_level_wrappers.rules import RulesLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.report import Report, ReportUpdate
from sift_client.sift_types.rule import Rule
from sift_client.sift_types.run import Run
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.tag import Tag


class ReportsAPIAsync(ResourceBase):
    """High-level API for interacting with reports."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the ReportsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ReportsLowLevelClient(grpc_client=self.client.grpc_client)
        self._rules_low_level_client = RulesLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(
        self,
        *,
        report_id: str,
    ) -> Report:
        """Get a Report.

        Args:
            report_id: The ID of the report.

        Returns:
            The Report.
        """
        report = await self._low_level_client.get_report(report_id=report_id)
        return self._apply_client_to_instance(report)

    async def list_(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        names: list[str] | None = None,
        description_contains: str | None = None,
        run: Run | str | None = None,
        organization_id: str | None = None,
        report_ids: list[str] | None = None,
        report_template_id: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
        tag_names: list[str] | list[Tag] | None = None,
        created_by: str | None = None,
        modified_by: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
    ) -> list[Report]:
        """List reports with optional filtering.

        Args:
            name: Exact name of the report.
            name_contains: Partial name of the report.
            name_regex: Regular expression string to filter reports by name.
            names: List of report names to filter by.
            description_contains: Partial description of the report.
            run: Run/run ID to filter by.
            organization_id: Organization ID to filter by.
            report_ids: List of report IDs to filter by.
            report_template_id: Report template ID to filter by.
            metadata: Metadata to filter by.
            tag_names: List of tags or tag names to filter by.
            created_by: The user ID of the creator of the reports.
            modified_by: The user ID of the last modifier of the reports.
            order_by: How to order the retrieved reports.
            limit: How many reports to retrieve. If None, retrieves all matches.
            include_archived: Whether to include archived reports.
            filter_query: Explicit CEL query to filter reports.
            created_after: Filter reports created after this datetime.
            created_before: Filter reports created before this datetime.
            modified_after: Filter reports modified after this datetime.
            modified_before: Filter reports modified before this datetime.

        Returns:
            A list of Reports that matches the filter.
        """
        # Build CEL filter
        filter_parts = [
            *self._build_name_cel_filters(
                name=name,
                names=names,
                name_contains=name_contains,
                name_regex=name_regex,
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
                modified_by=modified_by,
            ),
            *self._build_tags_metadata_cel_filters(tag_names=tag_names, metadata=metadata),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                include_archived=include_archived,
                filter_query=filter_query,
            ),
        ]

        if run:
            run_id = run.id_ if isinstance(run, Run) else run
            filter_parts.append(cel.equals("run_id", run_id))

        if report_ids:
            filter_parts.append(cel.in_("report_id", report_ids))

        if report_template_id:
            filter_parts.append(cel.equals("report_template_id", report_template_id))

        query_filter = cel.and_(*filter_parts) if filter_parts else None

        reports = await self._low_level_client.list_all_reports(
            query_filter=query_filter,
            organization_id=organization_id,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(reports)

    async def find(self, **kwargs) -> Report | None:
        """Find a single report matching the given query. Takes the same arguments as `list`. If more than one report is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Report found or None.
        """
        reports = await self.list_(**kwargs)
        if len(reports) > 1:
            raise ValueError("Multiple reports found for query")
        elif len(reports) == 1:
            return reports[0]
        return None

    async def create_from_template(
        self,
        *,
        report_template_id: str,
        run_id: str,
        organization_id: str | None = None,
        name: str | None = None,
    ) -> Report | None:
        """Create a new report from a report template.

        Args:
            report_template_id: The ID of the report template to use.
            run_id: The run ID to associate with the report.
            organization_id: The organization ID.
            name: Optional name for the report.

        Returns:
            The created Report or None if no report was created.
        """
        (
            created_annotation_count,
            created_report,
            job_id,
        ) = await self._rules_low_level_client.evaluate_rules(
            report_template_id=report_template_id,
            run_id=run_id,
            organization_id=organization_id,
            report_name=name,
        )
        if created_report:
            return self._apply_client_to_instance(created_report)
        return None

    async def create_from_rules(
        self,
        *,
        name: str,
        run: Run | str | None = None,
        organization_id: str | None = None,
        rules: list[Rule] | list[str],
    ) -> Report | None:
        """Create a new report from rules.

        Args:
            name: The name of the report.
            run: The run or run ID to associate with the report.
            organization_id: The organization ID.
            rules: List of rules or rule IDs to include in the report.

        Returns:
            The created Report or None if no report was created.
        """
        (
            created_annotation_count,
            created_report,
            job_id,
        ) = await self._rules_low_level_client.evaluate_rules(
            run_id=run._id_or_error if isinstance(run, Run) else run,
            organization_id=organization_id,
            rule_ids=[rule._id_or_error if isinstance(rule, Rule) else rule for rule in rules]
            or [],
            report_name=name,
        )
        if created_report:
            return self._apply_client_to_instance(created_report)
        return None

    async def create_from_applicable_rules(
        self,
        *,
        run: Run | str | None = None,
        organization_id: str | None = None,
        name: str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
    ) -> Report | None:
        """Create a new report from applicable rules based on a run.
        If you want to evaluate against assets, use the rules client instead since no report is created in that case.

        Args:
            run: The run or run ID to associate with the report.
            organization_id: The organization ID.
            name: Optional name for the report.
            start_time: Optional start time to evaluate rules against.
            end_time: Optional end time to evaluate rules against.

        Returns:
            The created Report or None if no report was created.
        """
        (
            created_annotation_count,
            created_report,
            job_id,
        ) = await self._rules_low_level_client.evaluate_rules(
            run_id=run._id_or_error if isinstance(run, Run) else run,
            organization_id=organization_id,
            start_time=start_time,
            end_time=end_time,
            report_name=name,
            all_applicable_rules=True,
        )
        if created_report:
            return self._apply_client_to_instance(created_report)
        return None

    async def rerun(
        self,
        *,
        report: str | Report,
    ) -> tuple[str, str]:
        """Rerun a report.

        Args:
            report: The Report or report ID to rerun.

        Returns:
            A tuple of (job_id, new_report_id).
        """
        report_id = report.id_ if isinstance(report, Report) else report
        if not isinstance(report_id, str):
            raise TypeError(f"report_id must be a string not {type(report_id)}")
        return await self._low_level_client.rerun_report(report_id=report_id)

    async def cancel(
        self,
        *,
        report: str | Report,
    ) -> None:
        """Cancel a report.

        Args:
            report: The Report or report ID to cancel.
        """
        report_id = report.id_ if isinstance(report, Report) else report
        if not isinstance(report_id, str):
            raise TypeError(f"report_id must be a string not {type(report_id)}")
        await self._low_level_client.cancel_report(report_id=report_id)

    async def update(self, report: str | Report, update: ReportUpdate | dict) -> Report:
        """Update a report.

        Args:
            report: The Report or report ID to update.
            update: The updates to apply.
        """
        report_id = report.id_ if isinstance(report, Report) else report

        if isinstance(update, dict):
            update = ReportUpdate.model_validate(update)
        update.resource_id = report_id
        updated_report = await self._low_level_client.update_report(update=update)
        return self._apply_client_to_instance(updated_report)

    async def archive(
        self,
        *,
        report: str | Report,
    ) -> Report:
        """Archive a report."""
        report_id = report.id_ if isinstance(report, Report) else report
        update = ReportUpdate(is_archived=True)
        update.resource_id = report_id
        updated_report = await self._low_level_client.update_report(update=update)
        return self._apply_client_to_instance(updated_report)

    async def unarchive(
        self,
        *,
        report: str | Report,
    ) -> Report:
        """Unarchive a report."""
        report_id = report.id_ if isinstance(report, Report) else report
        update = ReportUpdate(is_archived=False)
        update.resource_id = report_id
        updated_report = await self._low_level_client.update_report(update=update)
        return self._apply_client_to_instance(updated_report)
