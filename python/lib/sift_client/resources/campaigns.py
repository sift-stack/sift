from __future__ import annotations

from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.campaigns import CampaignsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.campaign import (
    Campaign,
    CampaignCreate,
    CampaignReportSummary,
    CampaignUpdate,
)
from sift_client.sift_types.run import Run
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient
    from sift_client.sift_types.report import Report
    from sift_client.sift_types.tag import Tag


class CampaignsAPIAsync(ResourceBase):
    """High-level API for interacting with campaigns.

    A campaign is a named list of reports. Seeding from runs collects every report
    those runs generated.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the CampaignsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = CampaignsLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(
        self,
        campaign_id: str | None = None,
        *,
        client_key: str | None = None,
        organization_id: str | None = None,
        skip_report_summaries: bool = False,
    ) -> Campaign:
        """Get a Campaign by ID or client key.

        Args:
            campaign_id: The ID of the campaign.
            client_key: The client key, as an alternative to the ID.
            organization_id: Required with `client_key` if you belong to several orgs.
            skip_report_summaries: Omit the per-report counts. Much faster for large campaigns.

        Returns:
            The Campaign.
        """
        campaign = await self._low_level_client.get_campaign(
            campaign_id=campaign_id,
            client_key=client_key,
            organization_id=organization_id,
            skip_report_summaries=skip_report_summaries,
        )
        return self._apply_client_to_instance(campaign)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        campaign_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
        # created/modified users
        created_by: Any | str | None = None,
        # tags and metadata
        tags: list[str] | list[Tag] | None = None,
        metadata: dict[str, Any] | None = None,
        # campaign specific
        reports: list[Report] | list[str] | None = None,
        runs: list[Run] | list[str] | None = None,
        # common filters
        description_contains: str | None = None,
        include_archived: bool = False,
        skip_report_summaries: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[Campaign]:
        """List campaigns.

        Args:
            name: Exact name of the campaign.
            names: List of campaign names to filter by.
            name_contains: Partial name of the campaign.
            name_regex: Regular expression to filter campaigns by name.
            campaign_ids: Filter to campaigns with any of these IDs.
            client_keys: Filter to campaigns with any of these client keys.
            created_by: Filter campaigns created by this user ID.
            tags: Filter campaigns with any of these Tags or tag names.
            metadata: Filter campaigns by metadata criteria.
            reports: Filter campaigns containing any of these Reports or report IDs.
            runs: Filter campaigns containing any of these Runs or run IDs.
            description_contains: Partial description of the campaign.
            include_archived: If True, include archived campaigns in results.
            skip_report_summaries: Omit the per-report counts. Much faster over many
                campaigns; fetch counts for the ones you want with `report_summaries`.
            filter_query: Explicit CEL query to filter campaigns.
            order_by: Field and direction to order results by.
            limit: Maximum number of campaigns to return. If None, returns all matches.
            page_size: Number of results to fetch per request.

        Returns:
            A list of Campaign objects that match the filter criteria.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_time_cel_filters(created_by=created_by),
            *self._build_tags_metadata_cel_filters(tag_names=tags, metadata=metadata),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                filter_query=filter_query,
            ),
        ]
        if campaign_ids:
            filter_parts.append(cel.in_("campaign_id", campaign_ids))
        if client_keys:
            filter_parts.append(cel.in_("client_key", client_keys))
        if reports:
            filter_parts.append(cel.in_("report_id", [self._report_id(r) for r in reports]))
        if runs:
            run_ids = [r._id_or_error if isinstance(r, Run) else r for r in runs]
            filter_parts.append(cel.in_("run_id", run_ids))
        query_filter = cel.and_(*filter_parts)

        campaigns = await self._low_level_client.list_all_campaigns(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            include_archived=include_archived,
            skip_report_summaries=skip_report_summaries,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(campaigns)

    async def find(self, **kwargs) -> Campaign | None:
        """Find one campaign. Takes the same arguments as `list_`.

        Raises if more than one matches.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Campaign found or None.
        """
        campaigns = await self.list_(**kwargs)
        if len(campaigns) > 1:
            raise ValueError(f"Multiple ({len(campaigns)}) campaigns found for query")
        elif len(campaigns) == 1:
            return campaigns[0]
        return None

    async def create(
        self,
        create: CampaignCreate | dict,
        *,
        reports: list[Report] | list[str] | None = None,
        runs: list[Run] | list[str] | None = None,
        campaign: str | Campaign | None = None,
    ) -> Campaign:
        """Create a new campaign, optionally seeded with reports.

        At most one seed may be given.

        Args:
            create: The campaign definition.
            reports: Seed with these Reports or report IDs.
            runs: Seed with every report these Runs generated.
            campaign: Duplicate this Campaign or campaign ID.

        Returns:
            The created Campaign.

        Raises:
            ValueError: If more than one seed is provided.
        """
        if isinstance(create, dict):
            create = CampaignCreate.model_validate(create)
        if len([seed for seed in (reports, runs, campaign) if seed]) > 1:
            raise ValueError("At most one of reports, runs, or campaign may be provided")
        created = await self._low_level_client.create_campaign(
            create=create,
            from_report_ids=[self._report_id(r) for r in reports] if reports else None,
            from_run_ids=(
                [r._id_or_error if isinstance(r, Run) else r for r in runs] if runs else None
            ),
            from_campaign_id=(
                campaign._id_or_error if isinstance(campaign, Campaign) else campaign
            ),
        )
        return self._apply_client_to_instance(created)

    async def update(self, campaign: str | Campaign, update: CampaignUpdate | dict) -> Campaign:
        """Update a Campaign.

        `reports`, `tags`, and `metadata` are replaced, not merged. Prefer
        `add_reports_to_campaign` to grow the report list.

        Args:
            campaign: The Campaign or campaign ID to update.
            update: Updates to apply to the Campaign.

        Returns:
            The updated Campaign.
        """
        campaign_id = campaign._id_or_error if isinstance(campaign, Campaign) else campaign
        if isinstance(update, dict):
            update = CampaignUpdate.model_validate(update)
        update.resource_id = campaign_id
        updated = await self._low_level_client.update_campaign(update)
        return self._apply_client_to_instance(updated)

    async def add_reports_to_campaign(
        self, campaign: str | Campaign, reports: list[Report] | list[str]
    ) -> Campaign:
        """Add reports to a campaign, keeping the ones already there.

        `CampaignService` has no append RPC, so this reads the report list, merges, and
        writes it back. Two concurrent calls drop one side's reports.

        Args:
            campaign: The Campaign or campaign ID to add to.
            reports: The Reports or report IDs to add.

        Returns:
            The updated Campaign.
        """
        campaign_id = campaign._id_or_error if isinstance(campaign, Campaign) else campaign
        current = await self.get(campaign_id, skip_report_summaries=True)
        existing_ids = [r.report_id for r in current.report_summaries]
        merged = existing_ids + [
            report_id
            for report_id in (self._report_id(r) for r in reports)
            if report_id not in existing_ids
        ]
        return await self.update(current, CampaignUpdate(reports=merged))

    async def archive(self, campaign: str | Campaign) -> Campaign:
        """Archive a campaign.

        Args:
            campaign: The Campaign or campaign ID to archive.

        Returns:
            The archived Campaign.
        """
        return await self.update(campaign, CampaignUpdate(is_archived=True))

    async def unarchive(self, campaign: str | Campaign) -> Campaign:
        """Unarchive a campaign.

        Args:
            campaign: The Campaign or campaign ID to unarchive.

        Returns:
            The unarchived Campaign.
        """
        return await self.update(campaign, CampaignUpdate(is_archived=False))

    async def report_summaries(
        self, campaigns: list[str | Campaign], *, organization_id: str | None = None
    ) -> dict[str, list[CampaignReportSummary]]:
        """Get per-report rule counts for several campaigns at once.

        Args:
            campaigns: The Campaigns or campaign IDs to summarize.
            organization_id: Required if you belong to several organizations.

        Returns:
            A mapping of campaign ID to its reports, with counts populated. The service
            returns each campaign's reports in no fixed order; `Campaign.report_summaries`
            orders them to match the campaign.
        """
        ids = [c._id_or_error if isinstance(c, Campaign) else c for c in campaigns]
        if not ids:
            return {}
        found = await self._low_level_client.get_report_summaries(
            campaign_ids=ids, organization_id=organization_id
        )
        return {campaign_id: found.get(campaign_id, []) for campaign_id in ids}

    @staticmethod
    def _report_id(report: Report | str) -> str:
        return report if isinstance(report, str) else report._id_or_error
