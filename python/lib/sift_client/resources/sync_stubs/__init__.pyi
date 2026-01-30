# Auto-generated stub

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import re
    from datetime import datetime, timedelta
    from pathlib import Path
    from typing import TYPE_CHECKING, Any, Sequence

    import pandas as pd
    import pyarrow as pa

    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset, AssetUpdate
    from sift_client.sift_types.calculated_channel import (
        CalculatedChannel,
        CalculatedChannelCreate,
        CalculatedChannelUpdate,
    )
    from sift_client.sift_types.channel import Channel
    from sift_client.sift_types.file_attachment import (
        FileAttachment,
        FileAttachmentUpdate,
        RemoteFileEntityType,
    )
    from sift_client.sift_types.job import Job, JobStatus, JobType
    from sift_client.sift_types.report import Report, ReportUpdate
    from sift_client.sift_types.rule import Rule, RuleCreate, RuleUpdate
    from sift_client.sift_types.run import Run, RunCreate, RunUpdate
    from sift_client.sift_types.tag import Tag, TagUpdate
    from sift_client.sift_types.test_report import (
        TestMeasurement,
        TestMeasurementCreate,
        TestMeasurementType,
        TestMeasurementUpdate,
        TestReport,
        TestReportCreate,
        TestReportUpdate,
        TestStatus,
        TestStep,
        TestStepCreate,
        TestStepType,
        TestStepUpdate,
    )

class AssetsAPI:
    """Sync counterpart to `AssetsAPIAsync`.

    High-level API for interacting with assets.

    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def archive(self, asset: str | Asset, *, archive_runs: bool = False) -> Asset:
        """Archive an asset.

        Args:
             asset: The Asset or asset ID to archive.
             archive_runs: If True, archive all Runs associated with the Asset.

        Returns:
             The archived Asset.
        """
        ...

    def find(self, **kwargs) -> Asset | None:
        """Find a single asset matching the given query. Takes the same arguments as `list_`. If more than one asset is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Asset found or None.
        """
        ...

    def get(self, *, asset_id: str | None = None, name: str | None = None) -> Asset:
        """Get an Asset.

        Args:
            asset_id: The ID of the asset.
            name: The name of the asset.

        Returns:
            The Asset.
        """
        ...

    def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        asset_ids: list[str] | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        tags: list[Any] | list[str] | list[Tag] | None = None,
        metadata: list[Any] | None = None,
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Asset]:
        """List assets with optional filtering.

        Args:
            name: Exact name of the asset.
            names: List of asset names to filter by.
            name_contains: Partial name of the asset.
            name_regex: Regular expression to filter assets by name.
            asset_ids: Filter to assets with any of these Ids.
            created_after: Filter assets created after this datetime.
            created_before: Filter assets created before this datetime.
            modified_after: Filter assets modified after this datetime.
            modified_before: Filter assets modified before this datetime.
            created_by: Filter assets created by this User or user ID.
            modified_by: Filter assets last modified by this User or user ID.
            tags: Filter assets with any of these Tags or tag names.
            metadata: Filter assets by metadata criteria.
            description_contains: Partial description of the asset.
            include_archived: If True, include archived assets in results.
            filter_query: Explicit CEL query to filter assets.
            order_by: Field and direction to order results by.
            limit: Maximum number of assets to return. If None, returns all matches.

        Returns:
            A list of Asset objects that match the filter criteria.
        """
        ...

    def unarchive(self, asset: str | Asset) -> Asset:
        """Unarchive an asset.

        Args:
             asset: The Asset or asset ID to unarchive.

        Returns:
             The unarchived Asset.
        """
        ...

    def update(self, asset: str | Asset, update: AssetUpdate | dict) -> Asset:
        """Update an Asset.

        Args:
            asset: The Asset or asset ID to update.
            update: Updates to apply to the Asset.

        Returns:
            The updated Asset.
        """
        ...

class CalculatedChannelsAPI:
    """Sync counterpart to `CalculatedChannelsAPIAsync`.

    High-level API for interacting with calculated channels.

    This class provides a Pythonic, notebook-friendly interface for interacting with the CalculatedChannelsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the CalculatedChannel class from the low-level wrapper, which is a user-friendly
    representation of a calculated channel using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the CalculatedChannelsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def archive(self, calculated_channel: str | CalculatedChannel) -> CalculatedChannel:
        """Archive a calculated channel.

        Args:
            calculated_channel: The id or CalculatedChannel object of the calculated channel to archive.

        Returns:
            The archived CalculatedChannel.
        """
        ...

    def create(self, create: CalculatedChannelCreate | dict) -> CalculatedChannel:
        """Create a calculated channel.

        Args:
            create: A CalculatedChannelCreate object or dictionary with configuration for the new calculated channel.
                   This should include properties like name, expression, channel_references, etc.

        Returns:
            The created CalculatedChannel.
        """
        ...

    def find(self, **kwargs) -> CalculatedChannel | None:
        """Find a single calculated channel matching the given query. Takes the same arguments as `list` but handles checking for multiple matches.
        Will raise an error if multiple calculated channels are found.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The CalculatedChannel found or None.
        """
        ...

    def get(
        self, *, calculated_channel_id: str | None = None, client_key: str | None = None
    ) -> CalculatedChannel:
        """Get a Calculated Channel.

        Args:
            calculated_channel_id: The ID of the calculated channel.
            client_key: The client key of the calculated channel.

        Returns:
            The CalculatedChannel.

        Raises:
            ValueError: If neither calculated_channel_id nor client_key is provided.
        """
        ...

    def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        calculated_channel_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        tags: list[Any] | list[str] | list[Tag] | None = None,
        metadata: list[Any] | None = None,
        asset: Asset | str | None = None,
        run: Run | str | None = None,
        version: int | None = None,
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[CalculatedChannel]:
        """List calculated channels with optional filtering. This will return the latest version. To find all versions, use `list_versions`.

        Args:
            name: Exact name of the calculated channel.
            names: List of calculated channel names to filter by.
            name_contains: Partial name of the calculated channel.
            name_regex: Regular expression string to filter calculated channels by name.
            calculated_channel_ids: Filter to calculated channels with any of these IDs.
            client_keys: Filter to calculated channels with any of these client keys.
            created_after: Created after this date.
            created_before: Created before this date.
            modified_after: Modified after this date.
            modified_before: Modified before this date.
            created_by: Calculated channels created by this user.
            modified_by: Calculated channels last modified by this user.
            tags: Filter calculated channels with any of these Tags or tag names.
            metadata: Filter calculated channels by metadata criteria.
            asset: Filter calculated channels associated with this Asset or asset ID.
            run: Filter calculated channels associated with this Run or run ID.
            version: The version of the calculated channel.
            description_contains: Partial description of the calculated channel.
            include_archived: Include archived calculated channels.
            filter_query: Explicit CEL query to filter calculated channels.
            order_by: How to order the retrieved calculated channels.
            limit: How many calculated channels to retrieve. If None, retrieves all matches.

        Returns:
            A list of CalculatedChannels that matches the filter.
        """
        ...

    def list_versions(
        self,
        *,
        calculated_channel: CalculatedChannel | str | None = None,
        client_key: str | None = None,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        tags: list[Any] | list[str] | list[Tag] | None = None,
        metadata: list[Any] | None = None,
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[CalculatedChannel]:
        """List versions of a calculated channel.

        Args:
            calculated_channel: The CalculatedChannel or ID of the calculated channel to get versions for.
            client_key: The client key of the calculated channel.
            name: Exact name of the calculated channel.
            names: List of calculated channel names to filter by.
            name_contains: Partial name of the calculated channel.
            name_regex: Regular expression string to filter calculated channels by name.
            created_after: Filter versions created after this datetime.
            created_before: Filter versions created before this datetime.
            modified_after: Filter versions modified after this datetime.
            modified_before: Filter versions modified before this datetime.
            created_by: Filter versions created by this user or user ID.
            modified_by: Filter versions modified by this user or user ID.
            tags: Filter versions with any of these Tags or tag names.
            metadata: Filter versions by metadata criteria.
            description_contains: Partial description of the calculated channel.
            include_archived: Include archived versions.
            filter_query: Explicit CEL query to filter versions.
            order_by: How to order the retrieved versions.
            limit: Maximum number of versions to return. If None, returns all matches.

        Returns:
            A list of CalculatedChannel versions that match the filter criteria.
        """
        ...

    def unarchive(self, calculated_channel: str | CalculatedChannel) -> CalculatedChannel:
        """Unarchive a calculated channel.

        Args:
            calculated_channel: The id or CalculatedChannel object of the calculated channel to unarchive.

        Returns:
            The unarchived CalculatedChannel.
        """
        ...

    def update(
        self,
        calculated_channel: CalculatedChannel | str,
        update: CalculatedChannelUpdate | dict,
        *,
        user_notes: str | None = None,
    ) -> CalculatedChannel:
        """Update a Calculated Channel.

        Args:
            calculated_channel: The CalculatedChannel or id of the CalculatedChannel to update.
            update: Updates to apply to the CalculatedChannel.
            user_notes: User notes for the update.

        Returns:
            The updated CalculatedChannel.
        """
        ...

class ChannelsAPI:
    """Sync counterpart to `ChannelsAPIAsync`.

    High-level API for interacting with channels.

    This class provides a Pythonic, notebook-friendly interface for interacting with the ChannelsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Channel class from the low-level wrapper, which is a user-friendly
    representation of a channel using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the ChannelsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def find(self, **kwargs) -> Channel | None:
        """Find a single channel matching the given query. Takes the same arguments as `list`. If more than one channel is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Channel found or None.
        """
        ...

    def get(self, *, channel_id: str) -> Channel:
        """Get a Channel.

        Args:
            channel_id: The ID of the channel.

        Returns:
            The Channel.
        """
        ...

    def get_data(
        self,
        *,
        channels: list[Channel],
        run: Run | str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
        limit: int | None = None,
        ignore_cache: bool = False,
    ) -> dict[str, pd.DataFrame]:
        """Get data for one or more channels.

        Args:
            channels: The channels to get data for.
            run: The Run or run_id to get data for.
            start_time: The start time to get data for.
            end_time: The end time to get data for.
            limit: The maximum number of data points to return. Will be in increments of page_size or default page size defined by the call if no page_size is provided.
            ignore_cache: Whether to ignore cached data and fetch fresh data from the server.

        Returns:
            A dictionary mapping channel names to pandas DataFrames containing the channel data.
        """
        ...

    def get_data_as_arrow(
        self,
        *,
        channels: list[Channel],
        run: Run | str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
        limit: int | None = None,
        ignore_cache: bool = False,
    ) -> dict[str, pa.Table]:
        """Get data for one or more channels as pyarrow tables."""
        ...

    def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        channel_ids: list[str] | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        asset: Asset | str | None = None,
        assets: list[str | Asset] | None = None,
        run: Run | str | None = None,
        description_contains: str | None = None,
        include_archived: bool | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Channel]:
        """List channels with optional filtering.

        Args:
            name: Exact name of the channel.
            names: List of channel names to filter by.
            name_contains: Partial name of the channel.
            name_regex: Regular expression to filter channels by name.
            channel_ids: Filter to channels with any of these IDs.
            created_after: Filter channels created after this datetime. Note: This is related to the channel creation time, not the timestamp of the underlying data.
            created_before: Filter channels created before this datetime. Note: This is related to the channel creation time, not the timestamp of the underlying data.
            modified_after: Filter channels modified after this datetime.
            modified_before: Filter channels modified before this datetime.
            asset: Filter channels associated with this Asset or asset ID.
            assets: Filter channels associated with these Assets or asset IDs.
            run: Filter channels associated with this Run or run ID.
            description_contains: Partial description of the channel.
            include_archived: If True, include archived channels in results.
            filter_query: Explicit CEL query to filter channels.
            order_by: Field and direction to order results by.
            limit: Maximum number of channels to return. If None, returns all matches.

        Returns:
            A list of Channels that matches the filter criteria.
        """
        ...

class FileAttachmentsAPI:
    """Sync counterpart to `FileAttachmentsAPIAsync`.

    High-level API for interacting with file attachments (remote files).

    This class provides a Pythonic interface for managing file attachments
    on Sift entities like runs, assets, and test reports.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the FileAttachmentsAPIAsync.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def delete(
        self, *, file_attachments: list[FileAttachment | str] | FileAttachment | str
    ) -> None:
        """Batch delete multiple file attachments.

        Args:
            file_attachments: List of FileAttachments or the IDs of the file attachments to delete (up to 1000).
        """
        ...

    def download(self, *, file_attachment: FileAttachment | str, output_path: str | Path) -> None:
        """Download a file attachment to a local path.

        Args:
            file_attachment: The FileAttachment or the ID of the file attachment to download.
            output_path: The path to download the file attachment to.
        """
        ...

    def get(self, *, file_attachment_id: str) -> FileAttachment:
        """Get a file attachment by ID.

        Args:
            file_attachment_id: The ID of the file attachment to retrieve.

        Returns:
            The FileAttachment.
        """
        ...

    def get_download_url(self, *, file_attachment: FileAttachment | str) -> str:
        """Get a download URL for a file attachment.

        Args:
            file_attachment: The FileAttachment or the ID of the file attachment.

        Returns:
            The download URL for the file attachment.
        """
        ...

    def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        remote_file_ids: list[str] | None = None,
        entities: list[Run | Asset | TestReport | TestStep] | None = None,
        entity_type: RemoteFileEntityType | None = None,
        entity_ids: list[str] | None = None,
        description_contains: str | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[FileAttachment]:
        """List file attachments with optional filtering.

        Args:
            name: Exact name of the file attachment.
            names: List of file attachment names to filter by.
            name_contains: Partial name of the file attachment.
            name_regex: Regular expression to filter file attachments by name.
            remote_file_ids: Filter to file attachments with any of these IDs.
            entities: Filter to file attachments associated with these entities.
            entity_type: Filter to file attachments associated with this entity type.
            entity_ids: Filter to file attachments associated with these entity IDs.
            description_contains: Partial description of the file attachment.
            filter_query: Explicit CEL query to filter file attachments.
            order_by: Field and direction to order results by. Note: Not supported by the backend, but it is here for API consistency.
            limit: Maximum number of file attachments to return. If None, returns all matches.

        Returns:
            A list of FileAttachment objects that match the filter criteria.
        """
        ...

    def update(self, *, file_attachment: FileAttachmentUpdate | dict) -> FileAttachment:
        """Update a file attachment.

        Args:
            file_attachment: The FileAttachmentUpdate with fields to update.

        Returns:
            The updated FileAttachment.
        """
        ...

    def upload(
        self,
        *,
        path: str | Path,
        entity: Asset | Run | TestReport | TestStep,
        metadata: dict[str, Any] | None = None,
        description: str | None = None,
        organization_id: str | None = None,
    ) -> FileAttachment:
        """Upload a file attachment to a remote file.

        Args:
            path: The path to the file to upload.
            entity: The entity that the file is attached to.
            metadata: Optional metadata for the file (e.g., video/image metadata).
            description: Optional description of the file.
            organization_id: Optional organization ID.

        Returns:
            The uploaded FileAttachment.
        """
        ...

class JobsAPI:
    """Sync counterpart to `JobsAPIAsync`.

    High-level API for interacting with jobs.

    This class provides a Pythonic interface for managing jobs in Sift.
    Jobs represent long-running operations like data imports, rule evaluations, and data exports.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the JobsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def cancel(self, job: Job | str) -> None:
        """Cancel a job.

        If the job hasn't started yet, it will be cancelled immediately.
        Jobs that are already finished, failed, or cancelled are not affected.

        Args:
            job: The Job or ID of the job to cancel.
        """
        ...

    def get(self, job_id: str) -> Job:
        """Get a job by ID.

        Args:
            job_id: The ID of the job to retrieve.

        Returns:
            The Job object.
        """
        ...

    def list_(
        self,
        *,
        job_ids: list[str] | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by_user_id: str | None = None,
        modified_by_user_id: str | None = None,
        job_type: JobType | None = None,
        job_status: JobStatus | None = None,
        started_date_after: datetime | None = None,
        started_date_before: datetime | None = None,
        completed_date_after: datetime | None = None,
        completed_date_before: datetime | None = None,
        organization_id: str | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Job]:
        """List jobs with optional filtering.

        Args:
            job_ids: Filter to jobs with any of these IDs.
            created_after: Filter to jobs created after this datetime.
            created_before: Filter to jobs created before this datetime.
            modified_after: Filter to jobs modified after this datetime.
            modified_before: Filter to jobs modified before this datetime.
            created_by_user_id: Filter to jobs created by this user ID.
            modified_by_user_id: Filter to jobs last modified by this user ID.
            job_type: Filter to jobs with this type.
            job_status: Filter to jobs with this status.
            started_date_after: Filter to jobs started after this datetime.
            started_date_before: Filter to jobs started before this datetime.
            completed_date_after: Filter to jobs completed after this datetime.
            completed_date_before: Filter to jobs completed before this datetime.
            organization_id: Organization ID. Required if your user belongs to multiple organizations.
            filter_query: Explicit CEL query to filter jobs. If provided, other filter arguments are ignored.
            order_by: Field and direction to order results by.
            limit: Maximum number of jobs to return. If None, returns all matches.

        Returns:
            A list of Job objects that match the filter criteria.
        """
        ...

    def retry(self, job: Job | str) -> Job:
        """Retry a failed job.

        Jobs that are finished, in progress, or in the process of being cancelled are not affected.

        Args:
            job: The Job or ID of the job to retry.

        Returns:
            The updated Job object.
        """
        ...

class PingAPI:
    """Sync counterpart to `PingAPIAsync`.

    High-level API for performing health checks.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def ping(self) -> str:
        """Send a ping request to the server.

        Returns:
            The response from the server.
        """
        ...

class ReportsAPI:
    """Sync counterpart to `ReportsAPIAsync`.

    High-level API for interacting with reports.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the ReportsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def archive(self, *, report: str | Report) -> Report:
        """Archive a report."""
        ...

    def cancel(self, *, report: str | Report) -> None:
        """Cancel a report.

        Args:
            report: The Report or report ID to cancel.
        """
        ...

    def create_from_applicable_rules(
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
        ...

    def create_from_rules(
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
        ...

    def create_from_template(
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
        ...

    def find(self, **kwargs) -> Report | None:
        """Find a single report matching the given query. Takes the same arguments as `list`. If more than one report is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Report found or None.
        """
        ...

    def get(self, *, report_id: str) -> Report:
        """Get a Report.

        Args:
            report_id: The ID of the report.

        Returns:
            The Report.
        """
        ...

    def list_(
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
        ...

    def rerun(self, *, report: str | Report) -> tuple[str, str]:
        """Rerun a report.

        Args:
            report: The Report or report ID to rerun.

        Returns:
            A tuple of (job_id, new_report_id).
        """
        ...

    def unarchive(self, *, report: str | Report) -> Report:
        """Unarchive a report."""
        ...

    def update(self, report: str | Report, update: ReportUpdate | dict) -> Report:
        """Update a report.

        Args:
            report: The Report or report ID to update.
            update: The updates to apply.
        """
        ...

class RulesAPI:
    """Sync counterpart to `RulesAPIAsync`.

    High-level API for interacting with rules.

    This class provides a Pythonic, notebook-friendly interface for interacting with the RulesAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Rule class from the low-level wrapper, which is a user-friendly
    representation of a rule using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the RulesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def archive(self, rule: str | Rule) -> Rule:
        """Archive a rule.

        Args:
            rule: The id or Rule object of the rule to archive.

        Returns:
            The archived Rule.
        """
        ...

    def batch_update_or_create_rules(
        self,
        rules: Sequence[RuleCreate | RuleUpdate],
        *,
        override_expression_validation: bool = False,
    ) -> list[Rule]:
        """Batch update or create multiple rules.

        Args:
            rules: List of rule creates or updates to apply. RuleUpdate objects must have resource_id set.
            override_expression_validation: When true, the rules will be created even if the expressions are invalid.

        Warnings:
            SiftWarning: If not all rules are created or updated.

        Returns:
            List of updated or created Rules.

        Raises:
            ValueError: If the update/create fails or if not all rules were updated/created.
        """
        ...

    def create(
        self,
        create: RuleCreate | dict | Sequence[RuleCreate | dict],
        *,
        override_expression_validation: bool = True,
    ) -> Rule | list[Rule]:
        """Create a new rule.

        Args:
            create: A RuleCreate object, a dictionary with configuration for the new rule, or a list of the previously mentioned objects.
            override_expression_validation: When true, the rule will be created even if the expression is invalid.

        Warnings:
            SiftWarning: If not all rules are created.

        Returns:
            The created Rule (if a single dictionary or RuleCreate was provided) otherwise a list of the created rules.
        """
        ...

    def find(self, **kwargs) -> Rule | None:
        """Find a single rule matching the given query. Takes the same arguments as `list`. If more than one rule is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Rule found or None.
        """
        ...

    def get(self, *, rule_id: str | None = None, client_key: str | None = None) -> Rule:
        """Get a Rule.

        Args:
            rule_id: The ID of the rule.
            client_key: The client key of the rule.

        Returns:
            The Rule.
        """
        ...

    def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        rule_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        metadata: list[Any] | None = None,
        assets: list[str] | list[Asset] | None = None,
        asset_tags: list[str | Tag] | None = None,
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Rule]:
        """List rules with optional filtering.

        Args:
            name: Exact name of the rule.
            names: List of rule names to filter by.
            name_contains: Partial name of the rule.
            name_regex: Regular expression string to filter rules by name.
            client_keys: Client keys of rules to filter to.
            rule_ids: IDs of rules to filter to.
            created_after: Rules created after this datetime.
            created_before: Rules created before this datetime.
            modified_after: Rules modified after this datetime.
            modified_before: Rules modified before this datetime.
            created_by: Filter rules created by this User or user ID.
            modified_by: Filter rules last modified by this User or user ID.
            metadata: Filter rules by metadata criteria.
            assets: Filter rules associated with any of these Assets.
            asset_tags: Filter rules associated with any Assets that have these Tag IDs.
            description_contains: Partial description of the rule.
            include_archived: If True, include archived rules in results.
            filter_query: Explicit CEL query to filter rules.
            order_by: Field and direction to order results by.
            limit: Maximum number of rules to return. If None, returns all matches.

        Returns:
            A list of Rules that matches the filter.
        """
        ...

    def unarchive(self, rule: str | Rule) -> Rule:
        """Unarchive a rule.

        Args:
            rule: The id or Rule object of the rule to unarchive.

        Returns:
            The unarchived Rule.
        """
        ...

    def update(
        self, rule: Rule | str, update: RuleUpdate | dict, *, version_notes: str | None = None
    ) -> Rule:
        """Update a Rule.

        Args:
            rule: The Rule or rule ID to update.
            update: Updates to apply to the Rule.
            version_notes: Notes to include in the rule version.

        Returns:
            The updated Rule.
        """
        ...

class RunsAPI:
    """Sync counterpart to `RunsAPIAsync`.

    High-level API for interacting with runs.

    This class provides a Pythonic, notebook-friendly interface for interacting with the RunsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Run class from the low-level wrapper, which is a user-friendly
    representation of a run using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the RunsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def archive(self, run: str | Run) -> Run:
        """Archive a run.

        Args:
            run: The Run or run ID to archive.
        """
        ...

    def create(
        self,
        create: RunCreate | dict,
        assets: list[str | Asset] | None = None,
        associate_new_data: bool = False,
    ) -> Run:
        """Create a new run.

        Note on assets: You do not need to provide asset info when creating a run.
        If you pass a Run to future ingestion configs associated with assets, the association will happen automatically then.
        However, if you do pass assets and set associate_new_data=True, future ingested data that falls within the Run's time period will be associated with the Run. Even if that data's timestamp is in the past, if it has not been ingested yet and the Run's timestamp envelopes it, it will be associated with the Run. This may be useful if you want to capture a time range for a specific asset or won't know about this Run when ingesting to that asset.
        If the data has already been ingested, leave associate_new_data=False.

        Args:
            create: The Run definition to create.
            assets: List of assets to associate with the run. Note: if you are associating new data, you must provide assets/asset names.
            associate_new_data: If True, future ingested data that falls within the Run's time period will be associated with the Run. Even if that data's timestamp is in the past, if it has not been ingested yet and the Run's timestamp envelopes it, it will be associated with the Run.

        Returns:
            The created Run.
        """
        ...

    def find(self, **kwargs) -> Run | None:
        """Find a single run matching the given query. Takes the same arguments as `list_`. If more than one run is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Run found or None.
        """
        ...

    def get(self, *, run_id: str | None = None, client_key: str | None = None) -> Run:
        """Get a Run.

        Args:
            run_id: The ID of the run.
            client_key: The client key of the run.

        Returns:
            The Run.
        """
        ...

    def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        run_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        tags: list[str | Tag] | None = None,
        metadata: list[Any] | None = None,
        assets: list[Asset] | list[str] | None = None,
        asset_tags: list[str | Tag] | None = None,
        duration_less_than: timedelta | None = None,
        duration_greater_than: timedelta | None = None,
        start_time_after: datetime | None = None,
        start_time_before: datetime | None = None,
        stop_time_after: datetime | None = None,
        stop_time_before: datetime | None = None,
        is_stopped: bool | None = None,
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Run]:
        """List runs with optional filtering.

        Args:
            name: Exact name of the run.
            names: List of run names to filter by.
            name_contains: Partial name of the run.
            name_regex: Regular expression to filter runs by name.
            run_ids: Filter to runs with any of these IDs.
            client_keys: Filter to runs with any of these client keys.
            created_after: Filter runs created after this datetime.
            created_before: Filter runs created before this datetime.
            modified_after: Filter runs modified after this datetime.
            modified_before: Filter runs modified before this datetime.
            created_by: Filter runs created by this User or user ID.
            modified_by: Filter runs last modified by this User or user ID.
            tags: Filter runs with any of these Tags IDs.
            metadata: Filter runs by metadata criteria.
            assets: Filter runs associated with any of these Assets or asset IDs.
            asset_tags: Filter runs associated with any Assets that have these Tag IDs.
            duration_less_than: Filter runs with duration less than this time.
            duration_greater_than: Filter runs with duration greater than this time.
            start_time_after: Filter runs that started after this datetime.
            start_time_before: Filter runs that started before this datetime.
            stop_time_after: Filter runs that stopped after this datetime.
            stop_time_before: Filter runs that stopped before this datetime.
            is_stopped: Whether the run is stopped.
            description_contains: Partial description of the run.
            include_archived: If True, include archived runs in results.
            filter_query: Explicit CEL query to filter runs.
            order_by: Field and direction to order results by.
            limit: Maximum number of runs to return. If None, returns all matches.

        Returns:
            A list of Run objects that match the filter criteria.
        """
        ...

    def stop(self, run: str | Run) -> Run:
        """Stop a run by setting its stop time to the current time.

        Args:
            run: The Run or run ID to stop.
        """
        ...

    def unarchive(self, run: str | Run) -> Run:
        """Unarchive a run.

        Args:
            run: The Run or run ID to unarchive.
        """
        ...

    def update(self, run: str | Run, update: RunUpdate | dict) -> Run:
        """Update a Run.

        Args:
            run: The Run or run ID to update.
            update: Updates to apply to the Run.

        Returns:
            The updated Run.
        """
        ...

class TagsAPI:
    """Sync counterpart to `TagsAPIAsync`.

    High-level API for interacting with tags.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the TagsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def create(self, name: str) -> Tag:
        """Create a new tag.

        Args:
            name: The name of the tag.

        Returns:
            The created Tag.
        """
        ...

    def find(self, **kwargs) -> Tag | None:
        """Find a single tag matching the given query. Takes the same arguments as `list`. If more than one tag is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Tag found or None.
        """
        ...

    def find_or_create(self, names: list[str]) -> list[Tag]:
        """Find tags by name or create them if they don't exist.

        Args:
            names: List of tag names to find or create.

        Returns:
            List of Tags that were found or created.
        """
        ...

    def list_(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        names: list[str] | None = None,
        tag_ids: list[str] | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Tag]:
        """List tags with optional filtering.

        Args:
            name: Exact name of the tag.
            name_contains: Partial name of the tag.
            name_regex: Regular expression string to filter tags by name.
            names: List of tag names to filter by.
            tag_ids: List of tag IDs to filter by.
            filter_query: Explicit CEL query to filter tags.
            order_by: How to order the retrieved tags.
            limit: How many tags to retrieve. If None, retrieves all matches.

        Returns:
            A list of Tags that matches the filter.
        """
        ...

    def update(self, tag: str | Tag, update: TagUpdate | dict) -> Tag:
        """Update a Tag.

        Args:
            tag: The Tag or tag ID to update.
            update: Updates to apply to the Tag.

        Returns:
            The updated Tag.

        Note:
            The tags API doesn't have an update method in the proto,
            so this would need to be implemented if the API supports it.
        """
        ...

class TestResultsAPI:
    """Sync counterpart to `TestResultsAPIAsync`.

    High-level API for interacting with test reports, steps, and measurements.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the TestResultsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro): ...
    def archive(self, *, test_report: str | TestReport) -> TestReport:
        """Archive a test report.

        Args:
            test_report: The TestReport or test report ID to archive.
        """
        ...

    def create(self, test_report: TestReportCreate | dict) -> TestReport:
        """Create a new test report.

        Args:
            test_report: The test report to create (can be TestReport or TestReportCreate).

        Returns:
            The created TestReport.
        """
        ...

    def create_measurement(
        self, test_measurement: TestMeasurementCreate | dict, update_step: bool = False
    ) -> TestMeasurement:
        """Create a new test measurement.

        Args:
            test_measurement: The test measurement to create (can be TestMeasurement or TestMeasurementCreate).
            update_step: Whether to update the step to failed if the measurement is being created is failed.

        Returns:
            The created TestMeasurement.
        """
        ...

    def create_measurements(
        self, test_measurements: list[TestMeasurementCreate]
    ) -> tuple[int, list[str]]:
        """Create multiple test measurements in a single request.

        Args:
            test_measurements: The test measurements to create.

        Returns:
            A tuple of (measurements_created_count, measurement_ids).
        """
        ...

    def create_step(self, test_step: TestStepCreate | dict) -> TestStep:
        """Create a new test step.

        Args:
            test_step: The test step to create (can be TestStep or TestStepCreate).

        Returns:
            The created TestStep.
        """
        ...

    def delete(self, *, test_report: str | TestReport) -> None:
        """Delete a test report.

        Args:
            test_report: The TestReport or test report ID to delete.
        """
        ...

    def delete_measurement(self, *, test_measurement: str | TestMeasurement) -> None:
        """Delete a test measurement.

        Args:
            test_measurement: The TestMeasurement or measurement ID to delete.
        """
        ...

    def delete_step(self, *, test_step: str | TestStep) -> None:
        """Delete a test step.

        Args:
            test_step: The TestStep or test step ID to delete.
        """
        ...

    def find(self, **kwargs) -> TestReport | None:
        """Find a single test report matching the given query. Takes the same arguments as `list_`. If more than one test report is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The TestReport found or None.
        """
        ...

    def get(self, *, test_report_id: str) -> TestReport:
        """Get a TestReport.

        Args:
            test_report_id: The ID of the test report.

        Returns:
            The TestReport.
        """
        ...

    def get_step(self, test_step: str | TestStep) -> TestStep:
        """Get a TestStep.

        Args:
            test_step: The TestStep or test step ID to get.
        """
        ...

    def import_(self, test_file: str | Path) -> TestReport:
        """Import a test report from an already-uploaded file.

        Args:
            test_file: The path to the test report file to import. We currently only support XML files exported from NI TestStand.

        Returns:
            The imported TestReport.
        """
        ...

    def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        test_report_ids: list[str] | None = None,
        status: TestStatus | None = None,
        test_system_name: str | None = None,
        test_case: str | None = None,
        serial_numbers: list[str] | None = None,
        part_numbers: list[str] | None = None,
        system_operator: str | None = None,
        created_by: str | None = None,
        modified_by: str | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        metadata: list[Any] | dict[str, Any] | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[TestReport]:
        """List test reports with optional filtering.

        Args:
            name: Exact name of the test report.
            names: List of test report names to filter by.
            name_contains: Partial name of the test report.
            name_regex: Regular expression string to filter test reports by name.
            test_report_ids: Test report IDs to filter by.
            status: Status to filter by (TestStatus enum).
            test_system_name: Test system name to filter by.
            test_case: Test case to filter by.
            serial_numbers: Serial numbers to filter by.
            part_numbers: Part numbers to filter by.
            system_operator: System operator to filter by.
            created_by: User ID who created the test report.
            modified_by: User ID who last modified the test report.
            created_after: Filter test reports created after this datetime.
            created_before: Filter test reports created before this datetime.
            modified_after: Filter test reports modified after this datetime.
            modified_before: Filter test reports modified before this datetime.
            metadata: Filter test reports by metadata criteria.
            include_archived: Whether to include only archived or non-archived reports.
            filter_query: Custom filter to apply to the test reports.
            order_by: How to order the retrieved test reports. If used, this will override the other filters.
            limit: How many test reports to retrieve. If None, retrieves all matches.

        Returns:
            A list of TestReports that matches the filter.
        """
        ...

    def list_measurements(
        self,
        *,
        measurements: list[str] | list[TestMeasurement] | None = None,
        test_steps: list[str] | list[TestStep] | None = None,
        test_reports: list[str] | list[TestReport] | None = None,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        measurement_type: TestMeasurementType | None = None,
        passed: bool | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[TestMeasurement]:
        """List test measurements with optional filtering.

        Args:
            measurements: Measurements to filter by.
            test_steps: Test steps to filter by.
            test_reports: Test reports to filter by.
            name: Exact name of the test measurement.
            names: List of test measurement names to filter by.
            name_contains: Partial name of the test measurement.
            name_regex: Regular expression string to filter test measurements by name.
            measurement_type: Measurement type to filter by (TestMeasurementType enum).
            passed: Whether the measurement passed.
            filter_query: Explicit CEL query to filter test measurements.
            order_by: How to order the retrieved test measurements.
            limit: How many test measurements to retrieve. If None, retrieves all matches.

        Returns:
            A list of TestMeasurements that matches the filter.
        """
        ...

    def list_steps(
        self,
        *,
        test_steps: list[str] | list[TestStep] | None = None,
        test_reports: list[str] | list[TestReport] | None = None,
        parent_steps: list[str] | list[TestStep] | None = None,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        status: TestStatus | None = None,
        step_type: TestStepType | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[TestStep]:
        """List test steps with optional filtering.

        Args:
            test_steps: Test steps to filter by.
            test_reports: Test reports to filter by.
            parent_steps: Parent steps to filter by.
            name: Exact name of the test step.
            names: List of test step names to filter by.
            name_contains: Partial name of the test step.
            name_regex: Regular expression string to filter test steps by name.
            status: Status to filter by (TestStatus enum).
            step_type: Step type to filter by (TestStepType enum).
            filter_query: Explicit CEL query to filter test steps.
            order_by: How to order the retrieved test steps.
            limit: How many test steps to retrieve. If None, retrieves all matches.

        Returns:
            A list of TestSteps that matches the filter.
        """
        ...

    def unarchive(self, *, test_report: str | TestReport) -> TestReport:
        """Unarchive a test report.

        Args:
            test_report: The TestReport or test report ID to unarchive.
        """
        ...

    def update(self, test_report: str | TestReport, update: TestReportUpdate | dict) -> TestReport:
        """Update a TestReport.

        Args:
            test_report: The TestReport or test report ID to update.
            update: Updates to apply to the TestReport.

        Returns:
            The updated TestReport.
        """
        ...

    def update_measurement(
        self,
        test_measurement: TestMeasurement,
        update: TestMeasurementUpdate | dict,
        update_step: bool = False,
    ) -> TestMeasurement:
        """Update a TestMeasurement.

        Args:
            test_measurement: The TestMeasurement or measurement ID to update.
            update: Updates to apply to the TestMeasurement.
            update_step: Whether to update the step to failed if the measurement is being updated to failed.

        Returns:
            The updated TestMeasurement.
        """
        ...

    def update_step(self, test_step: str | TestStep, update: TestStepUpdate | dict) -> TestStep:
        """Update a TestStep.

        Args:
            test_step: The TestStep or test step ID to update.
            update: Updates to apply to the TestStep.

        Returns:
            The updated TestStep.
        """
        ...
