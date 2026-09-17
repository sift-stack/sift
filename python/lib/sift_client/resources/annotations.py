from __future__ import annotations

from typing import TYPE_CHECKING, Any, cast

from sift_client._internal.low_level_wrappers.annotations import AnnotationsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.annotation import (
    Annotation,
    AnnotationCommentElement,
    AnnotationCreate,
    AnnotationLinkedChannel,
    AnnotationLog,
    AnnotationLogKind,
    AnnotationLogState,
    AnnotationState,
    AnnotationType,
    AnnotationUpdate,
)
from sift_client.sift_types.run import Run
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.asset import Asset
    from sift_client.sift_types.channel import Channel
    from sift_client.sift_types.tag import Tag


class AnnotationLogsAPIAsync(ResourceBase):
    """High-level API for an annotation's history.

    Each log records one event: an assignment, a state change, or a comment.
    Reachable as `client.annotations.logs`.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the AnnotationLogsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = AnnotationsLowLevelClient(grpc_client=self.client.grpc_client)

    async def list_(
        self,
        *,
        annotation: str | Annotation | None = None,
        # self ids
        annotation_log_ids: list[str] | None = None,
        # created/modified ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        # created/modified users
        created_by: Any | str | None = None,
        # log specific
        kind: AnnotationLogKind | None = None,
        # common filters
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[AnnotationLog]:
        """List annotation logs.

        Args:
            annotation: Restrict results to this Annotation or annotation ID.
            annotation_log_ids: Filter to logs with any of these IDs.
            created_after: Filter logs created after this datetime.
            created_before: Filter logs created before this datetime.
            modified_after: Filter logs modified after this datetime.
            modified_before: Filter logs modified before this datetime.
            created_by: Filter logs created by this user ID.
            kind: Filter to comments, state updates, or assignments.
            filter_query: Explicit CEL query to filter logs.
            order_by: Field and direction to order results by.
            limit: Maximum number of logs to return. If None, returns all matches.
            page_size: Number of results to fetch per request.

        Returns:
            A list of AnnotationLog objects that match the filter criteria.
        """
        filter_parts = [
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
            ),
        ]
        if annotation_log_ids:
            filter_parts.append(cel.in_("annotation_log_id", annotation_log_ids))
        if kind:
            filter_parts.append(cel.equals("kind", kind.to_filter_str()))
        if filter_query:
            filter_parts.append(filter_query)
        query_filter = cel.and_(*filter_parts)

        logs = await self._low_level_client.list_all_annotation_logs(
            annotation_id=annotation._id_or_error
            if isinstance(annotation, Annotation)
            else annotation,
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(logs)

    async def comment(
        self, annotation: str | Annotation, text: str | list[AnnotationCommentElement]
    ) -> AnnotationLog:
        """Add a comment to an annotation.

        Args:
            annotation: The Annotation or annotation ID to comment on.
            text: Plain text, or a list of elements to mix text with user mentions.

        Returns:
            The created AnnotationLog.
        """
        body = [AnnotationCommentElement(text=text)] if isinstance(text, str) else text
        log = await self._low_level_client.create_annotation_log(
            annotation_id=annotation._id_or_error
            if isinstance(annotation, Annotation)
            else annotation,
            kind=AnnotationLogKind.COMMENT,
            comment=body,
        )
        return self._apply_client_to_instance(log)

    async def record_assignment(self, annotation: str | Annotation, user: str) -> AnnotationLog:
        """Record that an annotation was assigned to a user.

        This writes a history entry and nothing else. `annotations.assign` already
        writes one, so you rarely need this.

        Args:
            annotation: The Annotation or annotation ID.
            user: The user ID the annotation was assigned to.

        Returns:
            The created AnnotationLog.
        """
        log = await self._low_level_client.create_annotation_log(
            annotation_id=annotation._id_or_error
            if isinstance(annotation, Annotation)
            else annotation,
            kind=AnnotationLogKind.ASSIGNED,
            assigned_to_user_id=user,
        )
        return self._apply_client_to_instance(log)

    async def record_state(
        self, annotation: str | Annotation, state: AnnotationLogState
    ) -> AnnotationLog:
        """Record a state change on an annotation.

        This writes a history entry and nothing else. It leaves the state alone, so
        use `annotations.resolve`, `flag`, or `reopen` to change it.

        Args:
            annotation: The Annotation or annotation ID.
            state: The state to record.

        Returns:
            The created AnnotationLog.
        """
        log = await self._low_level_client.create_annotation_log(
            annotation_id=annotation._id_or_error
            if isinstance(annotation, Annotation)
            else annotation,
            kind=AnnotationLogKind.STATE_UPDATE,
            state=state,
        )
        return self._apply_client_to_instance(log)

    async def delete(self, annotation: str | Annotation, log: str | AnnotationLog) -> None:
        """Delete an annotation log.

        Args:
            annotation: The Annotation or annotation ID the log belongs to.
            log: The AnnotationLog or log ID to delete.
        """
        await self._low_level_client.delete_annotation_log(
            annotation_id=annotation._id_or_error
            if isinstance(annotation, Annotation)
            else annotation,
            annotation_log_id=log._id_or_error if isinstance(log, AnnotationLog) else log,
        )


class AnnotationsAPIAsync(ResourceBase):
    """High-level API for interacting with annotations.

    An annotation marks a time range on one or more assets. A data review annotation
    carries a review state and an assignee. A phase annotation marks a segment of a run
    and carries no state.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the AnnotationsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = AnnotationsLowLevelClient(grpc_client=self.client.grpc_client)
        self.logs = AnnotationLogsAPIAsync(sift_client)

    async def get(self, annotation_id: str) -> Annotation:
        """Get an Annotation.

        Args:
            annotation_id: The ID of the annotation.

        Returns:
            The Annotation.
        """
        annotation = await self._low_level_client.get_annotation(annotation_id=annotation_id)
        return self._apply_client_to_instance(annotation)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        annotation_ids: list[str] | None = None,
        # created/modified ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        # created/modified users
        created_by: Any | str | None = None,
        # tags and metadata
        tags: list[str] | list[Tag] | None = None,
        metadata: dict[str, Any] | None = None,
        # annotation specific
        annotation_type: AnnotationType | None = None,
        state: AnnotationState | None = None,
        assigned_to: Any | str | None = None,
        pending: bool | None = None,
        assets: list[Asset] | list[str] | None = None,
        runs: list[Run] | list[str] | None = None,
        rule_ids: list[str] | None = None,
        report_ids: list[str] | None = None,
        start_time_after: datetime | None = None,
        start_time_before: datetime | None = None,
        end_time_after: datetime | None = None,
        end_time_before: datetime | None = None,
        # common filters
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[Annotation]:
        """List annotations.

        Args:
            name: Exact name of the annotation.
            names: List of annotation names to filter by.
            name_contains: Partial name of the annotation.
            name_regex: Regular expression to filter annotations by name.
            annotation_ids: Filter to annotations with any of these IDs.
            created_after: Filter annotations created after this datetime.
            created_before: Filter annotations created before this datetime.
            modified_after: Filter annotations modified after this datetime.
            modified_before: Filter annotations modified before this datetime.
            created_by: Filter annotations created by this user ID.
            tags: Filter annotations with any of these Tags or tag names.
            metadata: Filter annotations by metadata criteria.
            annotation_type: Filter to DATA_REVIEW or PHASE annotations.
            state: Filter to a review state.
            assigned_to: Filter to annotations assigned to this user ID.
            pending: Filter to annotations from an ongoing rule violation.
            assets: Filter annotations on any of these Assets or asset IDs.
            runs: Filter annotations on any of these Runs or run IDs.
            rule_ids: Filter annotations created by any of these rules.
            report_ids: Filter annotations belonging to any of these reports.
            start_time_after: Filter annotations that start after this datetime.
            start_time_before: Filter annotations that start before this datetime.
            end_time_after: Filter annotations that end after this datetime.
            end_time_before: Filter annotations that end before this datetime.
            description_contains: Partial description of the annotation.
            include_archived: If True, include archived annotations in results.
            filter_query: Explicit CEL query to filter annotations.
            order_by: Field and direction to order results by.
            limit: Maximum number of annotations to return. If None, returns all matches.
            page_size: Number of results to fetch per request. Lower this if you hit gRPC
                message size limits on responses. If None, uses the server default.

        Returns:
            A list of Annotation objects that match the filter criteria.
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
            ),
            *self._build_tags_metadata_cel_filters(tag_names=tags, metadata=metadata),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                include_archived=include_archived,
                filter_query=filter_query,
            ),
        ]
        if annotation_ids:
            filter_parts.append(cel.in_("annotation_id", annotation_ids))
        if annotation_type:
            filter_parts.append(cel.equals("annotation_type", annotation_type.to_filter_str()))
        if state:
            filter_parts.append(cel.equals("state", state.to_filter_str()))
        if assigned_to:
            filter_parts.append(cel.equals("assignee", assigned_to))
        if pending is not None:
            filter_parts.append(cel.equals("pending", pending))
        if assets:
            if all(isinstance(a, str) for a in assets):
                filter_parts.append(cel.in_("asset_id", cast("list[str]", assets)))
            else:
                asset_objs = cast("list[Asset]", assets)
                filter_parts.append(cel.in_("asset_id", [a._id_or_error for a in asset_objs]))
        if runs:
            run_ids = [r._id_or_error if isinstance(r, Run) else r for r in runs]
            filter_parts.append(cel.in_("run_id", run_ids))
        if rule_ids:
            filter_parts.append(cel.in_("rule_id", rule_ids))
        if report_ids:
            filter_parts.append(cel.in_("report_id", report_ids))
        if start_time_after:
            filter_parts.append(cel.greater_than("start_time", start_time_after))
        if start_time_before:
            filter_parts.append(cel.less_than("start_time", start_time_before))
        if end_time_after:
            filter_parts.append(cel.greater_than("end_time", end_time_after))
        if end_time_before:
            filter_parts.append(cel.less_than("end_time", end_time_before))
        query_filter = cel.and_(*filter_parts)

        annotations = await self._low_level_client.list_all_annotations(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(annotations)

    async def find(self, **kwargs) -> Annotation | None:
        """Find one annotation. Takes the same arguments as `list_`.

        Raises if more than one matches.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Annotation found or None.
        """
        annotations = await self.list_(**kwargs)
        if len(annotations) > 1:
            raise ValueError(f"Multiple ({len(annotations)}) annotations found for query")
        elif len(annotations) == 1:
            return annotations[0]
        return None

    async def create(self, create: AnnotationCreate | dict) -> Annotation:
        """Create a new annotation.

        Args:
            create: The annotation definition. `assets` and `tags` take names, not IDs.

        Returns:
            The created Annotation.
        """
        if isinstance(create, dict):
            create = AnnotationCreate.model_validate(create)
        created = await self._low_level_client.create_annotation(create=create)
        return self._apply_client_to_instance(created)

    async def create_review(
        self,
        name: str,
        start_time: datetime,
        end_time: datetime,
        *,
        assets: list[str] | None = None,
        channels: list[Channel] | None = None,
        run: Run | str | None = None,
        description: str | None = None,
        tags: list[str] | None = None,
        state: AnnotationState | None = None,
        assign_to: str | None = None,
        metadata: dict[str, Any] | None = None,
    ) -> Annotation:
        """Flag a time range for review.

        The annotation must reach an asset. Pass `channels` and the asset comes from
        them, or name the assets directly.

        Args:
            name: The name of the annotation.
            start_time: When the range starts.
            end_time: When the range ends.
            assets: Asset names to associate. Derived from `channels` if omitted.
            channels: Channels to draw the annotation on.
            run: The Run or run ID the annotation belongs to.
            description: A description of what to review.
            tags: Tag names to apply.
            state: The initial review state. Defaults to open.
            assign_to: The user ID to assign the review to.
            metadata: User-defined metadata.

        Returns:
            The created Annotation.
        """
        return await self._create_typed(
            AnnotationType.DATA_REVIEW,
            name=name,
            start_time=start_time,
            end_time=end_time,
            assets=assets,
            channels=channels,
            run=run,
            description=description,
            tags=tags,
            state=state,
            assign_to=assign_to,
            metadata=metadata,
        )

    async def create_phase(
        self,
        name: str,
        start_time: datetime,
        end_time: datetime,
        *,
        assets: list[str] | None = None,
        channels: list[Channel] | None = None,
        run: Run | str | None = None,
        description: str | None = None,
        tags: list[str] | None = None,
        metadata: dict[str, Any] | None = None,
    ) -> Annotation:
        """Mark a time range as a phase.

        A phase labels a segment of a run. It carries no review state, so it has no
        `state` argument.

        Args:
            name: The name of the phase.
            start_time: When the phase starts.
            end_time: When the phase ends.
            assets: Asset names to associate. Derived from `channels` if omitted.
            channels: Channels to draw the phase on.
            run: The Run or run ID the phase belongs to.
            description: A description of the phase.
            tags: Tag names to apply.
            metadata: User-defined metadata.

        Returns:
            The created Annotation.
        """
        return await self._create_typed(
            AnnotationType.PHASE,
            name=name,
            start_time=start_time,
            end_time=end_time,
            assets=assets,
            channels=channels,
            run=run,
            description=description,
            tags=tags,
            metadata=metadata,
        )

    async def _create_typed(
        self,
        annotation_type: AnnotationType,
        *,
        name: str,
        start_time: datetime,
        end_time: datetime,
        assets: list[str] | None,
        channels: list[Channel] | None,
        run: Run | str | None,
        description: str | None,
        tags: list[str] | None,
        metadata: dict[str, Any] | None,
        state: AnnotationState | None = None,
        assign_to: str | None = None,
    ) -> Annotation:
        linked = [AnnotationLinkedChannel(channel_id=c._id_or_error) for c in channels or []]
        if not assets and channels:
            asset_ids = {c.asset_id for c in channels if c.asset_id}
            assets = [
                a.name for a in await self.client.async_.assets.list_(asset_ids=list(asset_ids))
            ]
        return await self.create(
            AnnotationCreate(
                name=name,
                start_time=start_time,
                end_time=end_time,
                annotation_type=annotation_type,
                assets=assets,
                linked_channels=linked or None,
                run_id=run._id_or_error if isinstance(run, Run) else run,
                description=description,
                tags=tags,
                state=state,
                assign_to_user_id=assign_to,
                metadata=metadata,
            )
        )

    async def update(
        self, annotation: str | Annotation, update: AnnotationUpdate | dict
    ) -> Annotation:
        """Update an Annotation.

        `tags`, `linked_channels`, and `metadata` are replaced, not merged.

        Args:
            annotation: The Annotation or annotation ID to update.
            update: Updates to apply to the Annotation.

        Returns:
            The updated Annotation.
        """
        annotation_id = (
            annotation._id_or_error if isinstance(annotation, Annotation) else annotation
        )
        if isinstance(update, dict):
            update = AnnotationUpdate.model_validate(update)
        update.resource_id = annotation_id
        updated = await self._low_level_client.update_annotation(update)
        return self._apply_client_to_instance(updated)

    async def archive(self, annotation: str | Annotation) -> Annotation:
        """Archive an annotation.

        Args:
            annotation: The Annotation or annotation ID to archive.

        Returns:
            The archived Annotation.
        """
        annotation_id = (
            annotation._id_or_error if isinstance(annotation, Annotation) else annotation
        )
        archived = await self._low_level_client.archive_annotation(annotation_id=annotation_id)
        return self._apply_client_to_instance(archived)

    async def unarchive(self, annotation: str | Annotation) -> Annotation:
        """Unarchive an annotation.

        Args:
            annotation: The Annotation or annotation ID to unarchive.

        Returns:
            The unarchived Annotation.
        """
        annotation_id = (
            annotation._id_or_error if isinstance(annotation, Annotation) else annotation
        )
        unarchived = await self._low_level_client.unarchive_annotation(annotation_id=annotation_id)
        return self._apply_client_to_instance(unarchived)

    async def batch_archive(self, annotations: list[str | Annotation]) -> None:
        """Archive many annotations in one call.

        Args:
            annotations: The Annotations or annotation IDs to archive.
        """
        ids = [a._id_or_error if isinstance(a, Annotation) else a for a in annotations]
        await self._low_level_client.batch_archive_annotations(annotation_ids=ids)

    async def batch_unarchive(self, annotations: list[str | Annotation]) -> None:
        """Unarchive many annotations in one call.

        Args:
            annotations: The Annotations or annotation IDs to unarchive.
        """
        ids = [a._id_or_error if isinstance(a, Annotation) else a for a in annotations]
        await self._low_level_client.batch_unarchive_annotations(annotation_ids=ids)

    async def assign(self, annotation: str | Annotation, user: str) -> Annotation:
        """Assign an annotation to a user for review.

        Args:
            annotation: The Annotation or annotation ID to assign.
            user: The user ID to assign to.

        Returns:
            The updated Annotation.
        """
        return await self.update(annotation, AnnotationUpdate(assigned_to_user_id=user))

    async def _set_state(self, annotation: str | Annotation, state: AnnotationState) -> Annotation:
        """Move an annotation to a review state, skipping the call if already there.

        The server rejects a redundant state change with INVALID_ARGUMENT. The state is
        only known without a fetch when an Annotation was passed rather than an ID.
        """
        if isinstance(annotation, Annotation) and annotation.state is state:
            return annotation
        return await self.update(annotation, AnnotationUpdate(state=state))

    async def resolve(self, annotation: str | Annotation) -> Annotation:
        """Close out a review as resolved.

        Args:
            annotation: The Annotation or annotation ID to resolve.

        Returns:
            The updated Annotation.
        """
        return await self._set_state(annotation, AnnotationState.RESOLVED)

    async def flag(self, annotation: str | Annotation) -> Annotation:
        """Flag a review as needing attention.

        Args:
            annotation: The Annotation or annotation ID to flag.

        Returns:
            The updated Annotation.
        """
        return await self._set_state(annotation, AnnotationState.FLAGGED)

    async def reopen(self, annotation: str | Annotation) -> Annotation:
        """Return a review to the open state.

        Args:
            annotation: The Annotation or annotation ID to reopen.

        Returns:
            The updated Annotation.
        """
        return await self._set_state(annotation, AnnotationState.OPEN)
