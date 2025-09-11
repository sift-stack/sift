from __future__ import annotations

import re
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.reports import ReportsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.report import Report
from sift_client.util.cel_utils import contains, equals, match

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class ReportsAPIAsync(ResourceBase):
    """High-level API for interacting with reports."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the ReportsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ReportsLowLevelClient(grpc_client=self.client.grpc_client)

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
        description: str | None = None,
        description_contains: str | None = None,
        run_id: str | None = None,
        organization_id: str | None = None,
        created_by_user_id: str | None = None,
        modified_by_user_id: str | None = None,
        report_template_id: str | None = None,
        tag_name: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Report]:
        """List reports with optional filtering.

        Args:
            name: Exact name of the report.
            name_contains: Partial name of the report.
            name_regex: Regular expression string to filter reports by name.
            description: Exact description of the report.
            description_contains: Partial description of the report.
            run_id: Run ID to filter by.
            organization_id: Organization ID to filter by.
            created_by_user_id: User ID who created the report.
            modified_by_user_id: User ID who modified the report.
            report_template_id: Report template ID to filter by.
            tag_name: Tag name to filter by.
            order_by: How to order the retrieved reports.
            limit: How many reports to retrieve. If None, retrieves all matches.

        Returns:
            A list of Reports that matches the filter.
        """
        # Build CEL filter
        filter_parts = []

        if name:
            filter_parts.append(equals("name", name))
        elif name_contains:
            filter_parts.append(contains("name", name_contains))
        elif name_regex:
            if isinstance(name_regex, re.Pattern):
                name_regex = name_regex.pattern
            filter_parts.append(match("name", name_regex))  # type: ignore

        if description:
            filter_parts.append(equals("description", description))
        elif description_contains:
            filter_parts.append(contains("description", description_contains))

        if run_id:
            filter_parts.append(equals("run_id", run_id))

        if organization_id:
            filter_parts.append(equals("organization_id", organization_id))

        if created_by_user_id:
            filter_parts.append(equals("created_by_user_id", created_by_user_id))

        if modified_by_user_id:
            filter_parts.append(equals("modified_by_user_id", modified_by_user_id))

        if report_template_id:
            filter_parts.append(equals("report_template_id", report_template_id))

        if tag_name:
            filter_parts.append(contains("tags", tag_name))

        query_filter = " && ".join(filter_parts) if filter_parts else None

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
        report_template_id: str,
        run_id: str,
        organization_id: str,
        name: str | None = None,
    ) -> Report:
        """Create a new report from a report template.

        Args:
            report_template_id: The ID of the report template to use.
            run_id: The run ID to associate with the report.
            organization_id: The organization ID.
            name: Optional name for the report.

        Returns:
            The created Report.
        """
        created_report = await self._low_level_client.create_report_from_template(
            report_template_id=report_template_id,
            run_id=run_id,
            organization_id=organization_id,
            name=name,
        )
        return self._apply_client_to_instance(created_report)

    async def create_from_rules(
        self,
        name: str,
        run_id: str,
        organization_id: str,
        description: str | None = None,
        tag_names: list[str] | None = None,
        rule_ids: list[str] | None = None,
        rule_client_keys: list[str] | None = None,
    ) -> Report:
        """Create a new report from rules.

        Args:
            name: The name of the report.
            run_id: The run ID to associate with the report.
            organization_id: The organization ID.
            description: Optional description of the report.
            tag_names: List of tag names to associate with the report.
            rule_ids: List of rule IDs to include in the report.
            rule_client_keys: List of rule client keys to include in the report.

        Returns:
            The created Report.
        """
        created_report = await self._low_level_client.create_report_from_rules(
            name=name,
            description=description,
            tag_names=tag_names,
            rule_ids=rule_ids,
            rule_client_keys=rule_client_keys,
            run_id=run_id,
            organization_id=organization_id,
        )
        return self._apply_client_to_instance(created_report)

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
