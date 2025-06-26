from __future__ import annotations

import re
from datetime import datetime
from typing import TYPE_CHECKING, List

from sift_client._internal.low_level_wrappers.runs import RunsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.types.run import Run, RunUpdate
from sift_client.util.cel_utils import contains, equals, equals_null, match, not_

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class RunsAPIAsync(ResourceBase):
    """
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
        super().__init__(sift_client)
        self._low_level_client = RunsLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(
        self,
        *,
        run_id: str,
    ) -> Run:
        """
        Get a Run.

        Args:
            run_id: The ID of the run.

        Returns:
            The Run.
        """
        run = await self._low_level_client.get_run(run_id=run_id)
        return self._apply_client_to_instance(run)

    async def list(
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

        if duration_seconds:
            filter_parts.append(equals("duration", duration_seconds))

        if client_key:
            filter_parts.append(equals("client_key", client_key))

        if asset_id:
            filter_parts.append(equals("asset_id", asset_id))

        if asset_name:
            filter_parts.append(equals("asset_name", asset_name))

        if created_by_user_id:
            filter_parts.append(equals("created_by_user_id", created_by_user_id))

        if is_stopped is not None:
            filter_parts.append(not_(equals_null("stop_time")))

        if not include_archived:
            filter_parts.append(equals("archived_date", None))

        query_filter = " && ".join(filter_parts) if filter_parts else None

        runs = await self._low_level_client.list_all_runs(
            query_filter=query_filter,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(runs)

    async def find(self, **kwargs) -> Run | None:
        """
        Find a single run matching the given query. Takes the same arguments as `list`. If more than one run is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Run found or None.
        """
        runs = await self.list(**kwargs)
        if len(runs) > 1:
            raise ValueError("Multiple runs found for query")
        elif len(runs) == 1:
            return runs[0]
        return None

    async def create(
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
        created_run = await self._low_level_client.create_run(
            name=name,
            description=description,
            tags=tags,
            start_time=start_time,
            stop_time=stop_time,
            organization_id=organization_id,
            client_key=client_key,
            metadata=metadata,
        )
        return self._apply_client_to_instance(created_run)

    async def update(self, run: str | Run, update: RunUpdate | dict) -> Run:
        """
        Update a Run.

        Args:
            run: The Run or run ID to update.
            update: Updates to apply to the Run.

        Returns:
            The updated Run.
        """
        if isinstance(run, str):
            run = await self.get(run_id=run)

        if isinstance(update, dict):
            update = RunUpdate.model_validate(update)

        update.resource_id = run.id
        updated_run = await self._low_level_client.update_run(run, update)
        return self._apply_client_to_instance(updated_run)

    async def delete(
        self,
        *,
        run: str | Run,
    ) -> None:
        """
        Delete a run.

        Args:
            run: The Run or run ID to delete.
        """
        run_id = run.id if isinstance(run, Run) else run
        if not isinstance(run_id, str):
            raise TypeError(f"run_id must be a string not {type(run_id)}")
        await self._low_level_client.delete_run(run_id=run_id)

    async def stop(
        self,
        *,
        run: str | Run,
    ) -> None:
        """
        Stop a run by setting its stop time to the current time.

        Args:
            run: The Run or run ID to stop.
        """
        run_id = run.id if isinstance(run, Run) else run
        await self._low_level_client.stop_run(run_id=run_id)

    async def create_automatic_association_for_assets(
        self,
        run: str | Run,
        asset_names: List[str],
    ) -> None:
        """
        Associate assets with a run for automatic data ingestion.

        Args:
            run: The Run or run ID.
            asset_names: List of asset names to associate.
        """
        run_id = run.id if isinstance(run, Run) else run
        await self._low_level_client.create_automatic_run_association_for_assets(
            run_id=run_id, asset_names=asset_names
        )

    async def stop_run(self, run: str | Run) -> None:
        """
        Stop a run by setting its stop time to the current time.

        Args:
            run: The Run or run ID to stop.
        """
        run_id = run.id if isinstance(run, Run) else run
        await self._low_level_client.stop_run(run_id=run_id)
