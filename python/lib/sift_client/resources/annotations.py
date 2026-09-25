from __future__ import annotations

from typing import TYPE_CHECKING, Any, cast

from sift_client._internal.low_level_wrappers.annotations import AnnotationsLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.annotation import (
    Annotation,
    AnnotationCommentElement,
    AnnotationCreate,
    AnnotationCreateBase,
    AnnotationLog,
    AnnotationLogKind,
    AnnotationState,
    AnnotationType,
    AnnotationUpdate,
    PhaseCreate,
)
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.channel import Channel
from sift_client.sift_types.report import Report
from sift_client.sift_types.rule import Rule
from sift_client.sift_types.run import Run
from sift_client.sift_types.user import User
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from collections.abc import Iterable
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.calculated_channel import CalculatedChannel
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
        annotation: str | Annotation,
        # self ids
        annotation_logs: list[str | AnnotationLog] | None = None,
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
            annotation: The Annotation or annotation ID whose history to list.
            annotation_logs: Filter to these AnnotationLogs or log IDs.
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
        if annotation_logs:
            log_ids = [
                x._id_or_error if isinstance(x, AnnotationLog) else x for x in annotation_logs
            ]
            filter_parts.append(cel.in_("annotation_log_id", log_ids))
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

    async def add_comment(
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


BATCH_LIMIT = 1000
"""Annotations per call to BatchArchiveAnnotations, per the service."""


def _create_from_dict(create: dict) -> AnnotationCreate | PhaseCreate:
    """Pick the create model from a dict's `annotation_type`.

    A dict from a config file holds a name or a number, never a live enum member, and
    omits the key entirely for a data review.
    """
    value = create.get("annotation_type")
    if value is None:
        resolved = AnnotationType.DATA_REVIEW
    elif isinstance(value, AnnotationType):
        resolved = value
    else:
        try:
            resolved = AnnotationType[value] if isinstance(value, str) else AnnotationType(value)
        except (KeyError, ValueError):
            names = ", ".join(t.name for t in AnnotationType)
            raise ValueError(
                f"Unknown annotation_type {value!r}. Expected one of: {names}"
            ) from None
    payload = {**create, "annotation_type": resolved}
    if resolved is AnnotationType.PHASE:
        return PhaseCreate.model_validate(payload)
    return AnnotationCreate.model_validate(payload)


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

    async def get(self, *, annotation_id: str) -> Annotation:
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
        name: str | Iterable[str] | None = None,
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
        modified_by: Any | str | None = None,
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
        rules: list[str | Rule] | None = None,
        reports: list[str | Report] | None = None,
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
            name: Exact name, or any iterable of names to match against.
            name_contains: Partial name of the annotation.
            name_regex: Regular expression to filter annotations by name.
            annotation_ids: Filter to annotations with any of these IDs.
            created_after: Filter annotations created after this datetime.
            created_before: Filter annotations created before this datetime.
            modified_after: Filter annotations modified after this datetime.
            modified_before: Filter annotations modified before this datetime.
            created_by: Filter annotations created by this user ID.
            modified_by: Filter annotations last modified by this user ID.
            tags: Filter annotations with any of these Tags or tag names.
            metadata: Filter annotations by metadata criteria.
            annotation_type: Filter to DATA_REVIEW or PHASE annotations.
            state: Filter to a review state.
            assigned_to: Filter to annotations assigned to this user's name.
            pending: Filter to annotations from an ongoing rule violation.
            rules: Filter to annotations created by any of these Rules or rule IDs.
            reports: Filter to annotations in any of these Reports or report IDs.
            assets: Filter annotations on any of these Assets or asset IDs.
            runs: Filter annotations on any of these Runs or run IDs.
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
                name=name if isinstance(name, str) else None,
                names=None if name is None or isinstance(name, str) else list(name),
                name_contains=name_contains,
                name_regex=name_regex,
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
                modified_by=modified_by,
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
        if rules:
            filter_parts.append(
                cel.in_("rule_id", [r._id_or_error if isinstance(r, Rule) else r for r in rules])
            )
        if reports:
            filter_parts.append(
                cel.in_(
                    "report_id", [r._id_or_error if isinstance(r, Report) else r for r in reports]
                )
            )
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

    async def create(self, create: AnnotationCreateBase | dict) -> Annotation:
        """Create an annotation.

        Pass an `AnnotationCreate` for a data review or a `PhaseCreate` for a phase. A
        dict picks the model from its `annotation_type`, which may be a name or a number.

        Args:
            create: The annotation definition. `assets` and `tags` take names or objects.

        Returns:
            The created Annotation.
        """
        if isinstance(create, dict):
            create = _create_from_dict(create)
        if not create.assets and create.linked_channels:
            create.assets = cast(
                "list[str | Asset]", await self._assets_for_channels(create.linked_channels)
            )
        elif create.assets:
            create.assets = cast("list[str | Asset]", await self._asset_names(create.assets))
        created = await self._low_level_client.create_annotation(create=create)
        return self._apply_client_to_instance(created)

    async def _asset_names(self, assets: list[str | Asset]) -> list[str]:
        names = [a.name for a in assets if isinstance(a, Asset)]
        ids = [a for a in assets if isinstance(a, str)]
        if ids:
            names.extend(a.name for a in await self.client.async_.assets.list_(asset_ids=ids))
        return names

    async def _assets_for_channels(
        self, channels: list[Channel] | list[CalculatedChannel] | list[Channel | CalculatedChannel]
    ) -> list[str]:
        asset_ids = {c.asset_id for c in channels if isinstance(c, Channel) and c.asset_id}
        if not asset_ids:
            return []
        assets = await self.client.async_.assets.list_(asset_ids=list(asset_ids))
        return [a.name for a in assets]

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

    async def batch_archive(self, annotations: list[str | Annotation]) -> list[Annotation]:
        """Archive many annotations, one call per `BATCH_LIMIT` of them.

        Args:
            annotations: The Annotations or annotation IDs to archive.

        Returns:
            The archived Annotations.
        """
        archived = []
        for batch in self._batches(annotations):
            archived.extend(
                await self._low_level_client.batch_archive_annotations(annotation_ids=batch)
            )
        return self._apply_client_to_instances(archived)

    async def batch_unarchive(self, annotations: list[str | Annotation]) -> list[Annotation]:
        """Unarchive many annotations, one call per `BATCH_LIMIT` of them.

        Args:
            annotations: The Annotations or annotation IDs to unarchive.

        Returns:
            The unarchived Annotations.
        """
        unarchived = []
        for batch in self._batches(annotations):
            unarchived.extend(
                await self._low_level_client.batch_unarchive_annotations(annotation_ids=batch)
            )
        return self._apply_client_to_instances(unarchived)

    @staticmethod
    def _batches(annotations: list[str | Annotation]) -> list[list[str]]:
        """Split annotations into ID batches the service will accept."""
        ids = [a._id_or_error if isinstance(a, Annotation) else a for a in annotations]
        return [ids[i : i + BATCH_LIMIT] for i in range(0, len(ids), BATCH_LIMIT)]

    async def assign_to_user(self, annotation: str | Annotation, user: str | User) -> Annotation:
        """Assign an annotation to a user for review.

        Args:
            annotation: The Annotation or annotation ID to assign.
            user: The User or user ID to assign to.

        Returns:
            The updated Annotation.
        """
        user_id = user._id_or_error if isinstance(user, User) else user
        return await self.update(annotation, AnnotationUpdate(assigned_to_user_id=user_id))

    async def _set_state(self, annotation: str | Annotation, state: AnnotationState) -> Annotation:
        """Move an annotation to a review state, skipping the call if already there.

        The server rejects a redundant state change with INVALID_ARGUMENT. The state is
        only known without a fetch when an Annotation was passed rather than an ID.
        """
        if isinstance(annotation, Annotation) and annotation.state is state:
            return annotation
        return await self.update(annotation, AnnotationUpdate(state=state))

    async def set_accepted(self, annotation: str | Annotation) -> Annotation:
        """Close out a review as resolved.

        Args:
            annotation: The Annotation or annotation ID.

        Returns:
            The updated Annotation.
        """
        return await self._set_state(annotation, AnnotationState.ACCEPTED)

    async def set_failed(self, annotation: str | Annotation) -> Annotation:
        """Flag a review as needing attention.

        Args:
            annotation: The Annotation or annotation ID.

        Returns:
            The updated Annotation.
        """
        return await self._set_state(annotation, AnnotationState.FAILED)

    async def set_open(self, annotation: str | Annotation) -> Annotation:
        """Return a review to the open state.

        Args:
            annotation: The Annotation or annotation ID.

        Returns:
            The updated Annotation.
        """
        return await self._set_state(annotation, AnnotationState.OPEN)
