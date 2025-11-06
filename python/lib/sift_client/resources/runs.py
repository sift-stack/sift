from __future__ import annotations

from typing import TYPE_CHECKING, Any, cast

from sift_client._internal.low_level_wrappers.runs import RunsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.run import Run, RunCreate, RunUpdate
from sift_client.sift_types.tag import Tag
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from datetime import datetime, timedelta

    from sift_client.client import SiftClient


class RunsAPIAsync(ResourceBase):
    """High-level API for interacting with runs.

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
        super().__init__(sift_client)
        self._low_level_client = RunsLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(self, *, run_id: str | None = None, client_key: str | None = None) -> Run:
        """Get a Run.

        Args:
            run_id: The ID of the run.
            client_key: The client key of the run.

        Returns:
            The Run.
        """
        run: Run | None
        if run_id is not None:
            run = await self._low_level_client.get_run(run_id=run_id)
        elif client_key is not None:
            run = await self.find(client_keys=[client_key])
            if run is None:
                raise ValueError(f"Run with client_key {client_key} not found")
        else:
            raise ValueError("Either run_id or client_key must be provided")
        return self._apply_client_to_instance(run)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        run_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
        # created/modified ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        # created/modified users
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        # tags
        tags: list[str | Tag] | None = None,
        # metadata
        metadata: list[Any] | None = None,
        # run specific
        assets: list[Asset] | list[str] | None = None,
        asset_tags: list[str | Tag] | None = None,
        duration_less_than: timedelta | None = None,
        duration_greater_than: timedelta | None = None,
        start_time_after: datetime | None = None,
        start_time_before: datetime | None = None,
        stop_time_after: datetime | None = None,
        stop_time_before: datetime | None = None,
        is_stopped: bool | None = None,
        # common filters
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
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
                modified_by=modified_by,
            ),
            *self._build_tags_metadata_cel_filters(tag_ids=tags, metadata=metadata),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                include_archived=include_archived,
                filter_query=filter_query,
            ),
        ]
        if run_ids:
            filter_parts.append(cel.in_("run_id", run_ids))
        if client_keys:
            filter_parts.append(cel.in_("client_key", client_keys))
        if assets:
            if all(isinstance(s, str) for s in assets):
                ids = cast("list[str]", assets)  # linting
                filter_parts.append(cel.in_("asset_id", ids))
            else:
                asset = cast("list[Asset]", assets)  # linting
                filter_parts.append(cel.in_("asset_id", [a._id_or_error for a in asset]))
        if asset_tags:
            asset_tag_ids = [
                tag._id_or_error if isinstance(tag, Tag) else tag or "" for tag in asset_tags
            ]
            filter_parts.append(cel.in_("asset_tag_id", asset_tag_ids))
        if duration_less_than:
            filter_parts.append(cel.less_than("duration_string", duration_less_than))
        if duration_greater_than:
            filter_parts.append(cel.greater_than("duration_string", duration_greater_than))
        if start_time_after:
            filter_parts.append(cel.greater_than("start_time", start_time_after))
        if start_time_before:
            filter_parts.append(cel.less_than("start_time", start_time_before))
        if stop_time_after:
            filter_parts.append(cel.greater_than("stop_time", stop_time_after))
        if stop_time_before:
            filter_parts.append(cel.less_than("stop_time", stop_time_before))
        if is_stopped is not None:
            filter_parts.append(cel.not_(cel.equals_null("stop_time")))
        query_filter = cel.and_(*filter_parts)

        runs = await self._low_level_client.list_all_runs(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(runs)

    async def find(self, **kwargs) -> Run | None:
        """Find a single run matching the given query. Takes the same arguments as `list_`. If more than one run is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Run found or None.
        """
        runs = await self.list_(**kwargs)
        if len(runs) > 1:
            raise ValueError(f"Multiple ({len(runs)}) runs found for query")
        elif len(runs) == 1:
            return runs[0]
        return None

    async def create(
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
        if isinstance(create, dict):
            create = RunCreate.model_validate(create)

        asset_names: list[str] = []
        if associate_new_data:
            if not assets:
                raise ValueError(
                    "Assets/asset names must be provided when associate_new_data is True"
                )
            asset_names = [asset.name if isinstance(asset, Asset) else asset for asset in assets]
        elif assets:
            asset_ids = [
                asset._id_or_error if isinstance(asset, Asset) else asset for asset in assets
            ]
            tag_names = (
                [tag.name if isinstance(tag, Tag) else tag for tag in create.tags]
                if create.tags
                else None
            )
            created_run = await self._low_level_client.create_adhoc_run(
                name=create.name,
                description=create.description,
                asset_ids=asset_ids or [],
                start_time=create.start_time,
                stop_time=create.stop_time,
                tag_names=tag_names,
                metadata=create.metadata,
                client_key=create.client_key,
            )
            return self._apply_client_to_instance(created_run)
        created_run = await self._low_level_client.create_run(create=create)
        created_run = self._apply_client_to_instance(created_run)
        if associate_new_data:
            await self._low_level_client.create_automatic_run_association_for_assets(
                run_id=created_run._id_or_error, asset_names=asset_names
            )
        return created_run

    async def update(self, run: str | Run, update: RunUpdate | dict) -> Run:
        """Update a Run.

        Args:
            run: The Run or run ID to update.
            update: Updates to apply to the Run.

        Returns:
            The updated Run.
        """
        run_id = run._id_or_error if isinstance(run, Run) else run
        if isinstance(update, dict):
            update = RunUpdate.model_validate(update)
        update.resource_id = run_id
        updated_run = await self._low_level_client.update_run(update)
        return self._apply_client_to_instance(updated_run)

    async def archive(
        self,
        run: str | Run,
    ) -> Run:
        """Archive a run.

        Args:
            run: The Run or run ID to archive.
        """
        return await self.update(run, RunUpdate(is_archived=True))

    async def unarchive(
        self,
        run: str | Run,
    ) -> Run:
        """Unarchive a run.

        Args:
            run: The Run or run ID to unarchive.
        """
        return await self.update(run, RunUpdate(is_archived=False))

    async def stop(
        self,
        run: str | Run,
    ) -> Run:
        """Stop a run by setting its stop time to the current time.

        Args:
            run: The Run or run ID to stop.
        """
        run_id = run._id_or_error if isinstance(run, Run) else run
        await self._low_level_client.stop_run(run_id=run_id or "")
        return await self.get(run_id=run_id)
