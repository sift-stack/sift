from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.channels import ChannelsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.run import Run
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from datetime import datetime

    import pandas as pd
    import pyarrow as pa

    from sift_client.client import SiftClient
    from sift_client.sift_types.channel import Channel


class ChannelsAPIAsync(ResourceBase):
    """High-level API for interacting with channels.

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
        super().__init__(sift_client)
        self._low_level_client = ChannelsLowLevelClient(grpc_client=self.client.grpc_client)
        self._data_low_level_client = None

    async def get(
        self,
        *,
        channel_id: str,
    ) -> Channel:
        """Get a Channel.

        Args:
            channel_id: The ID of the channel.

        Returns:
            The Channel.
        """
        channel = await self._low_level_client.get_channel(channel_id=channel_id)
        return self._apply_client_to_instance(channel)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        channel_ids: list[str] | None = None,
        # created/modified ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        # channel specific
        asset: Asset | str | None = None,
        assets: list[str | Asset] | None = None,
        run: Run | str | None = None,
        # common filters
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
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
            ),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                filter_query=filter_query,
                include_archived=include_archived,
            ),
        ]
        if channel_ids:
            filter_parts.append(cel.in_("channel_id", channel_ids))
        if asset is not None:
            asset_id = asset._id_or_error if isinstance(asset, Asset) else asset
            filter_parts.append(cel.equals("asset_id", asset_id))
        if assets:
            asset_ids = [
                asset._id_or_error if isinstance(asset, Asset) else asset for asset in assets
            ]
            filter_parts.append(cel.in_("asset_id", asset_ids))
        if run is not None:
            run_id = run.id_ if isinstance(run, Run) else run
            filter_parts.append(cel.equals("run_id", run_id))
        # This is opposite of usual archived state
        if include_archived is not None:
            filter_parts.append(cel.equals("active", not include_archived))

        query_filter = cel.and_(*filter_parts)

        channels = await self._low_level_client.list_all_channels(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(channels)

    async def find(self, **kwargs) -> Channel | None:
        """Find a single channel matching the given query. Takes the same arguments as `list`. If more than one channel is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Channel found or None.
        """
        channels = await self.list_(**kwargs)
        if len(channels) > 1:
            raise ValueError(f"Multiple ({len(channels)}) channels found for query")
        elif len(channels) == 1:
            return channels[0]
        return None

    def _ensure_data_low_level_client(self):
        """Ensure that the data low level client is initialized. Separated out like this to not require large dependencies (pandas/pyarrow) for the client if not fetching data."""
        if self._data_low_level_client is None:
            from sift_client._internal.low_level_wrappers.data import DataLowLevelClient

            self._data_low_level_client = DataLowLevelClient(grpc_client=self.client.grpc_client)

    async def get_data(
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
        self._ensure_data_low_level_client()

        run_id = run._id_or_error if isinstance(run, Run) else run
        return await self._data_low_level_client.get_channel_data(  # type: ignore
            channels=channels,
            run_id=run_id,
            start_time=start_time,
            end_time=end_time,
            max_results=limit,
            ignore_cache=ignore_cache,
        )

    async def get_data_as_arrow(
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
        from pyarrow import Table as ArrowTable

        run_id = run.id_ if isinstance(run, Run) else run
        data = await self.get_data(
            channels=channels,
            run=run_id,
            start_time=start_time,
            end_time=end_time,
            limit=limit,
            ignore_cache=ignore_cache,
        )
        return {k: ArrowTable.from_pandas(v) for k, v in data.items()}
