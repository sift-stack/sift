# Auto-generated stub

from __future__ import annotations

import re
from datetime import datetime
from typing import Any, Dict, List

import pandas as pd
import pyarrow as pa

from sift_client.client import SiftClient
from sift_client.sift_types.asset import Asset, AssetUpdate
from sift_client.sift_types.calculated_channel import CalculatedChannel, CalculatedChannelUpdate
from sift_client.sift_types.channel import Channel, ChannelReference
from sift_client.sift_types.rule import Rule, RuleAction, RuleUpdate
from sift_client.sift_types.run import Run, RunUpdate

class AssetsAPI:
    """
    Sync counterpart to `AssetsAPIAsync`.


    High-level API for interacting with assets.

    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """

    def __init__(self, sift_client: "SiftClient"):
        """
        Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro):
        """ """
        ...

    def archive(self, asset: str | Asset, *, archive_runs: bool = False) -> Asset:
        """
        Archive an asset.

         Args:
             asset: The Asset or asset ID to archive.
             archive_runs: If True, archive all Runs associated with the Asset.

         Returns:
             The archived Asset.
        """
        ...

    def find(self, **kwargs) -> Asset | None:
        """
        Find a single asset matching the given query. Takes the same arguments as `list_`. If more than one asset is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Asset found or None.
        """
        ...

    def get(self, *, asset_id: str | None = None, name: str | None = None) -> Asset:
        """
        Get an Asset.

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
        created_by: Any | None = None,
        modified_by: Any | None = None,
        tags: list[str] | None = None,
        tag_ids: list[str] | None = None,
        metadata: list[Any] | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Asset]:
        """
        List assets with optional filtering.

        Args:
            asset_ids: List of asset IDs to filter by.
            name: Exact name of the asset.
            name_contains: Partial name of the asset.
            name_regex: Regular expression string to filter assets by name.
            asset_ids: List of asset IDs to filter by.
            created_after: Created after this date.
            created_before: Created before this date.
            modified_after: Modified after this date.
            modified_before: Modified before this date.
            created_by: Assets created by this user.
            modified_by: Assets last modified by this user.
            tags: Assets with these tags.
            tag_ids: List of asset tag IDs to filter by.
            include_archived: Include archived assets.
            filter_query: Explicit CEL query to filter assets.
            order_by: How to order the retrieved assets. # TODO: tooling for this?
            limit: How many assets to retrieve. If None, retrieves all matches.

        Returns:
            A list of Assets that matches the filter.
        """
        ...

    def update(self, asset: str | Asset, update: AssetUpdate | dict) -> Asset:
        """
        Update an Asset.

        Args:
            asset: The Asset or asset ID to update.
            update: Updates to apply to the Asset.

        Returns:
            The updated Asset.
        """
        ...

class CalculatedChannelsAPI:
    """
    Sync counterpart to `CalculatedChannelsAPIAsync`.


    High-level API for interacting with calculated channels.

    This class provides a Pythonic, notebook-friendly interface for interacting with the CalculatedChannelsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the CalculatedChannel class from the low-level wrapper, which is a user-friendly
    representation of a calculated channel using standard Python data structures and types.
    """

    def __init__(self, sift_client: "SiftClient"):
        """
        Initialize the CalculatedChannelsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro):
        """ """
        ...

    def archive(self, *, calculated_channel: str | CalculatedChannel) -> None:
        """
        Archive a Calculated Channel.
        """
        ...

    def create(
        self,
        *,
        name: str,
        expression: str,
        channel_references: List[ChannelReference],
        description: str = "",
        units: str | None = None,
        client_key: str | None = None,
        asset_ids: List[str] | None = None,
        tag_ids: List[str] | None = None,
        all_assets: bool = False,
        user_notes: str = "",
    ) -> CalculatedChannel:
        """
        Create a calculated channel.

        Args:
            name: The name of the calculated channel.
            expression: The expression to calculate the value of the calculated channel.
            channel_references: A list of channel references that are used in the expression.
            description: The description of the calculated channel.
            units: The units of the calculated channel.
            client_key: A user-defined unique identifier for the calculated channel.
            asset_ids: A list of asset IDs to make the calculation available for.
            tag_ids: A list of tag IDs to make the calculation available for.
            all_assets: A flag that, when set to True, associates the calculated channel with all assets.
            user_notes: User notes for the calculated channel.

        Returns:
            The created CalculatedChannel.

        Raises:
            ValueError: If asset configuration is invalid.
        """
        ...

    def find(self, **kwargs) -> CalculatedChannel | None:
        """
        Find a single calculated channel matching the given query. Takes the same arguments as `list` but handles checking for multiple matches.
        Will raise an error if multiple calculated channels are found.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The CalculatedChannel found or None.
        """
        ...

    def get(
        self,
        *,
        calculated_channel_id: str | None = None,
        client_key: str | None = None,
        organization_id: str | None = None,
    ) -> CalculatedChannel:
        """
        Get a Calculated Channel.

        Args:
            calculated_channel_id: The ID of the calculated channel.
            client_key: The client key of the calculated channel.
            organization_id: The organization ID (required if using client_key and user belongs to multiple organizations).

        Returns:
            The CalculatedChannel.

        Raises:
            ValueError: If neither calculated_channel_id nor client_key is provided.
        """
        ...

    def list(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        created_by: Any | None = None,
        modified_by: Any | None = None,
        client_key: str | None = None,
        asset_id: str | None = None,
        asset_name: str | None = None,
        tag_id: str | None = None,
        tag_name: str | None = None,
        version: int | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        organization_id: str | None = None,
    ) -> List[CalculatedChannel]:
        """
        List calculated channels with optional filtering.

        Args:
            name: Exact name of the calculated channel.
            name_contains: Partial name of the calculated channel.
            name_regex: Regular expression string to filter calculated channels by name.
            created_after: Created after this date.
            created_before: Created before this date.
            modified_after: Modified after this date.
            modified_before: Modified before this date.
            created_by: Calculated channels created by this user.
            modified_by: Calculated channels last modified by this user.
            client_key: The client key of the calculated channel.
            asset_id: The asset ID associated with the calculated channel.
            asset_name: The asset name associated with the calculated channel.
            tag_id: The tag ID associated with the calculated channel.
            tag_name: The tag name associated with the calculated channel.
            version: The version of the calculated channel.
            include_archived: Include archived calculated channels.
            filter_query: Explicit CEL query to filter calculated channels.
            order_by: How to order the retrieved calculated channels.
            limit: How many calculated channels to retrieve. If None, retrieves all matches.
            organization_id: The organization ID (required if user belongs to multiple organizations).

        Returns:
            A list of CalculatedChannels that matches the filter.
        """
        ...

    def list_versions(
        self,
        *,
        calculated_channel_id: str | None = None,
        client_key: str | None = None,
        organization_id: str | None = None,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        asset_id: str | None = None,
        asset_name: str | None = None,
        tag_id: str | None = None,
        tag_name: str | None = None,
        version: int | None = None,
        include_archived: bool = False,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> List[CalculatedChannel]:
        """
        List versions of a calculated channel.

        Args:
            calculated_channel_id: The ID of the calculated channel.
            client_key: The client key of the calculated channel.
            name: The name of the calculated channel.
            name_contains: The name of the calculated channel.
            name_regex: The name of the calculated channel.
            asset_id: The asset ID of the calculated channel.
            asset_name: The asset name of the calculated channel.
            tag_id: The tag ID of the calculated channel.
            tag_name: The tag name of the calculated channel.
            version: The version of the calculated channel.
            include_archived: Whether to include archived calculated channels.
            organization_id: The organization ID. Required if your user belongs to multiple organizations.
            order_by: The field to order by.
            limit: How many versions to retrieve. If None, retrieves all matches.

        Returns:
            A list of CalculatedChannel versions.

        Raises:
            ValueError: If neither calculated_channel_id nor client_key is provided.
        """
        ...

    def update(
        self,
        *,
        calculated_channel: str | CalculatedChannel,
        update: CalculatedChannelUpdate | dict,
        user_notes: str | None = None,
    ) -> CalculatedChannel:
        """
        Update a Calculated Channel.

        Args:
            calculated_channel: The CalculatedChannel or id of the CalculatedChannel to update.
            update: Updates to apply to the CalculatedChannel.
            user_notes: User notes for the update.

        Returns:
            The updated CalculatedChannel.
        """
        ...

class ChannelsAPI:
    """
    Sync counterpart to `ChannelsAPIAsync`.


    High-level API for interacting with channels.

    This class provides a Pythonic, notebook-friendly interface for interacting with the ChannelsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Channel class from the low-level wrapper, which is a user-friendly
    representation of a channel using standard Python data structures and types.
    """

    def __init__(self, sift_client: "SiftClient"):
        """
        Initialize the ChannelsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro):
        """ """
        ...

    def find(self, **kwargs) -> Channel | None:
        """
        Find a single channel matching the given query. Takes the same arguments as `list`. If more than one channel is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Channel found or None.
        """
        ...

    def get(self, *, channel_id: str) -> Channel:
        """
        Get a Channel.

        Args:
            channel_id: The ID of the channel.

        Returns:
            The Channel.
        """
        ...

    def get_data(
        self,
        *,
        channels: List[Channel],
        run_id: str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
        limit: int | None = None,
    ) -> Dict[str, pd.DataFrame]:
        """
        Get data for one or more channels.

        Args:
            channels: The channels to get data for.
            run_id: The run to get data for.
            start_time: The start time to get data for.
            end_time: The end time to get data for.
            limit: The maximum number of data points to return. Will be in increments of page_size or default page size defined by the call if no page_size is provided.
        """
        ...

    def get_data_as_arrow(
        self,
        *,
        channels: List[Channel],
        run_id: str | None = None,
        start_time: datetime | None = None,
        end_time: datetime | None = None,
        limit: int | None = None,
    ) -> Dict[str, pa.Table]:
        """
        Get data for one or more channels as pyarrow tables.
        """
        ...

    def list(
        self,
        *,
        asset_id: str | None = None,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        description: str | None = None,
        description_contains: str | None = None,
        active: bool | None = None,
        run_id: str | None = None,
        run_name: str | None = None,
        client_key: str | None = None,
        created_before: datetime | None = None,
        created_after: datetime | None = None,
        modified_before: datetime | None = None,
        modified_after: datetime | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Channel]:
        """
        List channels with optional filtering.

        Args:
            asset_id: The asset ID to get.
            name: The name of the channel to get.
            name_contains: The partial name of the channel to get.
            name_regex: The regex name of the channel to get.
            description: The description of the channel to get.
            description_contains: The partial description of the channel to get.
            active: Whether the channel is active.
            run_id: The run ID to get.
            run_name: The name of the run to get.
            client_key: The client key of the run to get.
            created_before: The created date of the channel to get.
            created_after: The created date of the channel to get.
            modified_before: The modified date of the channel to get.
            modified_after: The modified date of the channel to get.
            order_by: How to order the retrieved channels.
            limit: How many channels to retrieve. If None, retrieves all matches.

        Returns:
            A list of Channels that matches the filter.
        """
        ...

class PingAPI:
    """
    Sync counterpart to `PingAPIAsync`.


    High-level API for performing health checks.
    """

    def __init__(self, sift_client: "SiftClient"):
        """
        Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro):
        """ """
        ...

    def ping(self) -> str:
        """
        Send a ping request to the server.

        Returns:
            The response from the server.
        """
        ...

class RulesAPI:
    """
    Sync counterpart to `RulesAPIAsync`.


    High-level API for interacting with rules.

    This class provides a Pythonic, notebook-friendly interface for interacting with the RulesAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Rule class from the low-level wrapper, which is a user-friendly
    representation of a rule using standard Python data structures and types.
    """

    def __init__(self, sift_client: "SiftClient"):
        """
        Initialize the RulesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro):
        """ """
        ...

    def archive(
        self,
        *,
        rule: str | Rule | None = None,
        rules: List[Rule] | None = None,
        rule_ids: List[str] | None = None,
        client_keys: List[str] | None = None,
    ) -> None:
        """
        Archive a rule or multiple.

        Args:
            rule: The Rule to archive.
            rules: The Rules to archive.
            rule_ids: The rule IDs to archive.
            client_keys: The client keys to archive.
        """
        ...

    def batch_get(
        self, *, rule_ids: List[str] | None = None, client_keys: List[str] | None = None
    ) -> List[Rule]:
        """
        Get multiple rules by rule IDs or client keys.

        Args:
            rule_ids: List of rule IDs to get.
            client_keys: List of client keys to get.

        Returns:
            List of Rules.
        """
        ...

    def batch_restore(
        self, *, rule_ids: List[str] | None = None, client_keys: List[str] | None = None
    ) -> None:
        """
        Batch restore rules.

        Args:
            rule_ids: List of rule IDs to restore.
            client_keys: List of client keys to undelete.
        """
        ...

    def create(
        self,
        name: str,
        description: str,
        expression: str,
        channel_references: List[ChannelReference],
        action: RuleAction,
        organization_id: str | None = None,
        client_key: str | None = None,
        asset_ids: List[str] | None = None,
        contextual_channels: List[str] | None = None,
        is_external: bool = False,
    ) -> Rule:
        """
        Create a new rule.
        """
        ...

    def find(self, **kwargs) -> Rule | None:
        """
        Find a single rule matching the given query. Takes the same arguments as `list`. If more than one rule is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Rule found or None.
        """
        ...

    def get(self, *, rule_id: str | None = None, client_key: str | None = None) -> Rule:
        """
        Get a Rule.

        Args:
            rule_id: The ID of the rule.
            client_key: The client key of the rule.

        Returns:
            The Rule.
        """
        ...

    def list(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        include_deleted: bool = False,
    ) -> list[Rule]:
        """
        List rules with optional filtering.

        Args:
            name: Exact name of the rule.
            name_contains: Partial name of the rule.
            name_regex: Regular expression string to filter rules by name.
            order_by: How to order the retrieved rules.
            limit: How many rules to retrieve. If None, retrieves all matches.

        Returns:
            A list of Rules that matches the filter.
        """
        ...

    def restore(
        self, *, rule: str | Rule, rule_id: str | None = None, client_key: str | None = None
    ) -> Rule:
        """
        Restore a rule.

        Args:
            rule: The Rule or rule ID to restore.
            rule_id: The rule ID to restore (alternative to rule parameter).
            client_key: The client key to restore (alternative to rule parameter).

        Returns:
            The restored Rule.
        """
        ...

    def update(
        self, rule: str | Rule, update: RuleUpdate | dict, version_notes: str | None = None
    ) -> Rule:
        """
        Update a Rule.

        Args:
            rule: The Rule or rule ID to update.
            update: Updates to apply to the Rule.
            version_notes: Notes to include in the rule version.
        Returns:
            The updated Rule.
        """
        ...

class RunsAPI:
    """
    Sync counterpart to `RunsAPIAsync`.


    High-level API for interacting with runs.

    This class provides a Pythonic, notebook-friendly interface for interacting with the RunsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Run class from the low-level wrapper, which is a user-friendly
    representation of a run using standard Python data structures and types.
    """

    def __init__(self, sift_client: "SiftClient"):
        """
        Initialize the RunsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        ...

    def _run(self, coro):
        """ """
        ...

    def archive(self, *, run: str | Run) -> None:
        """
        Archive a run.

        Args:
            run: The Run or run ID to archive.
        """
        ...

    def create(
        self,
        name: str,
        description: str,
        tags: List[str] | None = None,
        start_time: datetime | None = None,
        stop_time: datetime | None = None,
        organization_id: str | None = None,
        client_key: str | None = None,
        metadata: dict[str, str | float | bool] | None = None,
    ) -> Run:
        """
        Create a new run.

        Args:
            name: The name of the run.
            description: The description of the run.
            tags: Tags to associate with the run.
            start_time: The start time of the run.
            stop_time: The stop time of the run.
            organization_id: The organization ID.
            client_key: A unique client key for the run.
            metadata: Metadata values for the run.

        Returns:
            The created Run.
        """
        ...

    def create_automatic_association_for_assets(
        self, run: str | Run, asset_names: List[str]
    ) -> None:
        """
        Associate assets with a run for automatic data ingestion.

        Args:
            run: The Run or run ID.
            asset_names: List of asset names to associate.
        """
        ...

    def find(self, **kwargs) -> Run | None:
        """
        Find a single run matching the given query. Takes the same arguments as `list`. If more than one run is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Run found or None.
        """
        ...

    def get(self, *, run_id: str) -> Run:
        """
        Get a Run.

        Args:
            run_id: The ID of the run.

        Returns:
            The Run.
        """
        ...

    def list(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        description: str | None = None,
        description_contains: str | None = None,
        duration_seconds: int | None = None,
        client_key: str | None = None,
        asset_id: str | None = None,
        asset_name: str | None = None,
        created_by_user_id: str | None = None,
        is_stopped: bool | None = None,
        include_archived: bool = False,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> List[Run]:
        """
        List runs with optional filtering.

        Args:
            name: Exact name of the run.
            name_contains: Partial name of the run.
            name_regex: Regular expression string to filter runs by name.
            description: Exact description of the run.
            description_contains: Partial description of the run.
            duration_seconds: Duration of the run in seconds.
            client_key: Client key to filter by.
            asset_id: Asset ID to filter by.
            asset_name: Asset name to filter by.
            created_by_user_id: User ID who created the run.
            is_stopped: Whether the run is stopped.
            include_archived: Whether to include archived runs.
            order_by: How to order the retrieved runs.
            limit: How many runs to retrieve. If None, retrieves all matches.

        Returns:
            A list of Runs that matches the filter.
        """
        ...

    def stop(self, *, run: str | Run) -> None:
        """
        Stop a run by setting its stop time to the current time.

        Args:
            run: The Run or run ID to stop.
        """
        ...

    def stop_run(self, run: str | Run) -> None:
        """
        Stop a run by setting its stop time to the current time.

        Args:
            run: The Run or run ID to stop.
        """
        ...

    def update(self, run: str | Run, update: RunUpdate | dict) -> Run:
        """
        Update a Run.

        Args:
            run: The Run or run ID to update.
            update: Updates to apply to the Run.

        Returns:
            The updated Run.
        """
        ...
