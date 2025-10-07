# Auto-generated stub

from __future__ import annotations

import re
from datetime import datetime, timedelta
from typing import Any

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
from sift_client.sift_types.rule import Rule, RuleCreate, RuleUpdate
from sift_client.sift_types.run import Run, RunCreate, RunUpdate

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
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        asset_ids: list[str] | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        tags: list[Any] | list[str] | None = None,
        _tag_ids: list[str] | None = None,
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
        tags: list[Any] | list[str] | None = None,
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
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        tags: list[Any] | list[str] | None = None,
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
    ) -> dict[str, pd.DataFrame]:
        """Get data for one or more channels.

        Args:
            channels: The channels to get data for.
            run: The Run or run_id to get data for.
            start_time: The start time to get data for.
            end_time: The end time to get data for.
            limit: The maximum number of data points to return. Will be in increments of page_size or default page size defined by the call if no page_size is provided.

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
    ) -> dict[str, pa.Table]:
        """Get data for one or more channels as pyarrow tables."""
        ...

    def list_(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        channel_ids: list[str] | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        asset: Asset | str | None = None,
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
            name_contains: Partial name of the channel.
            name_regex: Regular expression to filter channels by name.
            channel_ids: Filter to channels with any of these IDs.
            created_after: Filter channels created after this datetime.
            created_before: Filter channels created before this datetime.
            modified_after: Filter channels modified after this datetime.
            modified_before: Filter channels modified before this datetime.
            asset: Filter channels associated with this Asset or asset ID.
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

    def create(self, create: RuleCreate | dict) -> Rule:
        """Create a new rule.

        Args:
            create: A RuleCreate object or dictionary with configuration for the new rule.

        Returns:
            The created Rule.
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
        asset_ids: list[str] | None = None,
        asset_tag_ids: list[str] | None = None,
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Rule]:
        """List rules with optional filtering.

        Args:
            name: Exact name of the rule.
            name_contains: Partial name of the rule.
            name_regex: Regular expression string to filter rules by name.
            rule_ids: IDs of rules to filter to.
            client_keys: Client keys of rules to filter to.
            created_after: Rules created after this datetime.
            created_before: Rules created before this datetime.
            modified_after: Rules modified after this datetime.
            modified_before: Rules modified before this datetime.
            created_by: Filter rules created by this User or user ID.
            modified_by: Filter rules last modified by this User or user ID.
            metadata: Filter rules by metadata criteria.
            asset_ids: Filter rules associated with any of these Asset IDs.
            asset_tag_ids: Filter rules associated with any of these Asset Tag IDs.
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

    def create(self, create: RunCreate | dict) -> Run:
        """Create a new run.

        Args:
            create: The Run definition to create.

        Returns:
            The created Run.
        """
        ...

    def create_automatic_association_for_assets(
        self, run: str | Run, *, asset_names: list[str]
    ) -> None:
        """Associate assets with a run for automatic data ingestion.

        Args:
            run: The Run or run ID.
            asset_names: List of asset names to associate.
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
        metadata: list[Any] | None = None,
        assets: list[Asset] | list[str] | None = None,
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
            metadata: Filter runs by metadata criteria.
            assets: Filter runs associated with any of these Assets or asset IDs.
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

    def stop(self, run: str | Run) -> None:
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
