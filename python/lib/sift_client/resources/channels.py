from __future__ import annotations

import re
from datetime import datetime
from typing import TYPE_CHECKING, Dict, List

import pandas as pd
import pyarrow as pa

from sift_client._internal.low_level_wrappers.channels import ChannelsLowLevelClient
from sift_client._internal.low_level_wrappers.data import DataLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.types.channel import Channel
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class ChannelsAPIAsync(ResourceBase):
    """
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
        super().__init__(sift_client)
        self._low_level_client = ChannelsLowLevelClient(grpc_client=self.client.grpc_client)
        self._data_low_level_client = DataLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(
        self,
        *,
        channel_id: str,
    ) -> Channel:
        """
        Get a Channel.

        Args:
            channel_id: The ID of the channel.

        Returns:
            The Channel.
        """
        channel = await self._low_level_client.get_channel(channel_id=channel_id)
        return self._apply_client_to_instance(channel)

    async def list(
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
        if sum(bool(x) for x in [name, name_contains, name_regex]) > 1:
            raise ValueError("Cannot provide more than one of name, name_contains, or name_regex")
        if sum(bool(x) for x in [description, description_contains]) > 1:
            raise ValueError("Cannot provide both description and description_contains")
        if sum(bool(x) for x in [created_before, created_after]) > 1:
            raise ValueError("Cannot provide both created_before and created_after")
        if sum(bool(x) for x in [modified_before, modified_after]) > 1:
            raise ValueError("Cannot provide both modified_before and modified_after")

        filter_parts = []
        if asset_id:
            filter_parts.append(cel.equals("asset_id", asset_id))
        if name:
            filter_parts.append(cel.equals("name", name))
        elif name_contains:
            filter_parts.append(cel.contains("name", name_contains))
        elif name_regex:
            if isinstance(name_regex, re.Pattern):
                name_regex = name_regex.pattern
            filter_parts.append(cel.match("name", name_regex))  # type: ignore
        if description:
            filter_parts.append(cel.equals("description", description))
        elif description_contains:
            filter_parts.append(cel.contains("description", description_contains))
        if active:
            filter_parts.append(cel.equals("active", active))
        if run_id:
            filter_parts.append(cel.equals("run_id", run_id))
        if run_name:
            filter_parts.append(cel.equals("run_name", run_name))
        if client_key:
            filter_parts.append(cel.equals("client_key", client_key))
        if created_before:
            filter_parts.append(cel.less_than("created_date", created_before))
        if created_after:
            filter_parts.append(cel.greater_than("created_date", created_after))
        if modified_before:
            filter_parts.append(cel.less_than("modified_date", modified_before))
        if modified_after:
            filter_parts.append(cel.greater_than("modified_date", modified_after))

        filter_str = " && ".join(filter_parts)

        channels = await self._low_level_client.list_all_channels(
            query_filter=filter_str,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(channels)

    async def find(self, **kwargs) -> Channel | None:
        """
        Find a single channel matching the given query. Takes the same arguments as `list`. If more than one channel is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Channel found or None.
        """
        channels = await self.list(**kwargs)
        if len(channels) > 1:
            raise ValueError("Multiple channels found for query")
        elif len(channels) == 1:
            return channels[0]
        return None

    async def get_data(
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
        return await self._data_low_level_client.get_channel_data(
            channels=channels,
            run_id=run_id,
            start_time=start_time,
            end_time=end_time,
            limit=limit,
        )

    async def get_data_as_arrow(
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
        data = await self.get_data(
            channels=channels,
            run_id=run_id,
            start_time=start_time,
            end_time=end_time,
            limit=limit,
        )
        return {k: pa.Table.from_pandas(v) for k, v in data.items()}
