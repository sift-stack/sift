from __future__ import annotations

import io
import json
from pathlib import Path
from typing import TYPE_CHECKING, BinaryIO, Sequence

from sift_client._internal.low_level_wrappers.artifacts import ArtifactsLowLevelClient
from sift_client._internal.low_level_wrappers.remote_files import RemoteFilesLowLevelClient
from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.file import download_file, read_url_bytes
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.artifact import (
    Artifact,
    ArtifactCreate,
    ArtifactCreatedVia,
    ArtifactEntityType,
    ArtifactLinkRelation,
    ArtifactType,
    ArtifactUpdate,
    ArtifactVersion,
)
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.run import Run
from sift_client.sift_types.user import User
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.client import SiftClient

# The remote-files entity type that holds the bytes of an artifact version.
ARTIFACT_VERSION_ENTITY_TYPE = "artifact_versions"


class _ArtifactsResourceBase(ResourceBase):
    """Filters shared by the artifact and artifact version listings."""

    def _build_artifact_cel_filters(
        self,
        *,
        title: str | None = None,
        titles: list[str] | None = None,
        title_contains: str | None = None,
        # self ids
        artifact_ids: list[str] | list[Artifact] | None = None,
        # created ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        # created users
        created_by: User | str | None = None,
        # metadata
        metadata: dict[str, str | float | bool] | None = None,
        # artifact specific
        sources: list[Run | Asset] | None = None,
        derived_from: list[Artifact | str] | None = None,
        related_to: list[Run | Asset | Artifact] | None = None,
        file_name: str | None = None,
        created_via: ArtifactCreatedVia | None = None,
        artifact_type: ArtifactType | None = None,
    ) -> list[str]:
        filter_parts = [
            *self._build_name_cel_filters(
                name=title, names=titles, name_contains=title_contains, field="title"
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                created_by=created_by._id_or_error if isinstance(created_by, User) else created_by,
            ),
        ]
        if artifact_ids:
            ids = [a._id_or_error if isinstance(a, Artifact) else a for a in artifact_ids]
            filter_parts.append(cel.in_("artifact_id", ids))
        if metadata:
            filter_parts.extend(_metadata_equals(key, value) for key, value in metadata.items())
        if sources:
            filter_parts.append(_links_to(sources, ArtifactLinkRelation.SOURCE))
        if derived_from:
            filter_parts.append(_links_to(derived_from, ArtifactLinkRelation.DERIVED_FROM))
        if related_to:
            filter_parts.append(_links_to(related_to))
        if file_name:
            filter_parts.append(cel.equals("file_name", file_name))
        if created_via is not None:
            filter_parts.append(cel.equals("created_via", created_via.name))
        if artifact_type is not None:
            filter_parts.append(cel.equals("storage_class", artifact_type.name))
        return filter_parts


class ArtifactVersionsAPIAsync(_ArtifactsResourceBase):
    """High-level API for an artifact's version history.

    Every create or update writes a new version. Reachable as `client.artifacts.versions`.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the ArtifactVersionsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ArtifactsLowLevelClient(grpc_client=self.client.grpc_client)

    async def list_(
        self,
        *,
        artifact: str | Artifact | None = None,
        title: str | None = None,
        titles: list[str] | None = None,
        title_contains: str | None = None,
        # self ids
        artifact_ids: list[str] | list[Artifact] | None = None,
        # created ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        # created users
        created_by: User | str | None = None,
        # metadata
        metadata: dict[str, str | float | bool] | None = None,
        # artifact specific
        sources: list[Run | Asset] | None = None,
        derived_from: list[Artifact | str] | None = None,
        related_to: list[Run | Asset | Artifact] | None = None,
        file_name: str | None = None,
        created_via: ArtifactCreatedVia | None = None,
        artifact_type: ArtifactType | None = None,
        # common filters
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[ArtifactVersion]:
        """List artifact versions, newest first by default.

        Takes the same filters as `ArtifactsAPI.list_`, evaluated against each version, so a
        filter can match an earlier version that the latest no longer matches.
        `ArtifactVersion.is_current` marks each artifact's current version.

        Args:
            artifact: Only this Artifact or artifact ID's versions. If None, searches the
                versions of every artifact.
            title: Exact title of the version.
            titles: Filter to versions with any of these titles.
            title_contains: Partial title of the version.
            artifact_ids: Filter to versions of these Artifacts or artifact IDs.
            created_after: Filter to versions of artifacts created after this datetime.
            created_before: Filter to versions of artifacts created before this datetime.
            created_by: Filter to versions of artifacts created by this User or user ID.
            metadata: Filter to versions whose metadata has all of these key-value pairs.
            sources: Filter to versions produced from any of these Runs or Assets.
            derived_from: Filter to versions derived from any of these Artifacts or
                artifact IDs.
            related_to: Filter to versions linked to any of these Runs, Assets, or
                Artifacts, by any relation.
            file_name: Exact file name of the version.
            created_via: Filter to versions of artifacts written by this surface.
            artifact_type: Filter to versions of artifacts of this type.
            include_archived: If True, include versions of archived artifacts. A named
                `artifact` lists its versions even when it is archived.
            filter_query: Explicit CEL query with the `ArtifactsAPI.list_` fields,
                evaluated against each version.
            order_by: Field and direction to order results by, over `created_date`,
                `version`, and `title`. The service default is `created_date desc`.
            limit: Maximum number of versions to return. If None, returns all matches.
            page_size: Number of results to fetch per request.

        Returns:
            A list of ArtifactVersion objects that match the filter criteria.
        """
        filter_parts = self._build_artifact_cel_filters(
            title=title,
            titles=titles,
            title_contains=title_contains,
            artifact_ids=artifact_ids,
            created_after=created_after,
            created_before=created_before,
            created_by=created_by,
            metadata=metadata,
            sources=sources,
            derived_from=derived_from,
            related_to=related_to,
            file_name=file_name,
            created_via=created_via,
            artifact_type=artifact_type,
        )
        if include_archived and artifact is None:
            filter_parts.append(cel.equals("include_archived", True))
        filter_parts.extend(self._build_common_cel_filters(filter_query=filter_query))
        query_filter = cel.and_(*filter_parts)

        versions = await self._low_level_client.list_all_artifact_versions(
            query_filter=query_filter or None,
            order_by=order_by,
            artifact_id=None if artifact is None else _artifact_id(artifact),
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(versions)

    async def promote(self, *, version: ArtifactVersion) -> Artifact:
        """Write a new latest version that copies an earlier version.

        Copies the title, summary, metadata, file, and `SOURCE` and `DERIVED_FROM` links of
        `version`. `ATTACHED_TO` links stay as on the latest version. Only the artifact's
        creator can promote, and promoting the latest version fails.

        Args:
            version: The ArtifactVersion to promote.

        Returns:
            The Artifact, resolved to the version the promotion wrote.
        """
        promoted = await self._low_level_client.promote_artifact_version(
            artifact_id=version.artifact_id, artifact_version_id=version._id_or_error
        )
        return self._apply_client_to_instance(promoted)


class ArtifactsAPIAsync(_ArtifactsResourceBase):
    """High-level API for interacting with artifacts.

    An artifact is a versioned file, such as a plot or a report, that Sift stores and
    shows in the app. Links record where the artifact surfaces (`ATTACHED_TO`) and what
    it was produced from (`SOURCE`), so you can find artifacts by the runs and assets
    they link to.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the ArtifactsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = ArtifactsLowLevelClient(grpc_client=self.client.grpc_client)
        self._remote_files_client = RemoteFilesLowLevelClient(grpc_client=self.client.grpc_client)
        self._upload_client = UploadLowLevelClient(rest_client=self.client.rest_client)
        self.versions = ArtifactVersionsAPIAsync(sift_client)

    async def get(self, *, artifact_id: str, artifact_version_id: str | None = None) -> Artifact:
        """Get an artifact.

        Args:
            artifact_id: The artifact to get.
            artifact_version_id: Resolve to this version instead of the latest.

        Returns:
            The Artifact.
        """
        artifact = await self._low_level_client.get_artifact(
            artifact_id=artifact_id, artifact_version_id=artifact_version_id
        )
        return self._apply_client_to_instance(artifact)

    async def list_(
        self,
        *,
        title: str | None = None,
        titles: list[str] | None = None,
        title_contains: str | None = None,
        # self ids
        artifact_ids: list[str] | list[Artifact] | None = None,
        # created ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        # created users
        created_by: User | str | None = None,
        # metadata
        metadata: dict[str, str | float | bool] | None = None,
        # artifact specific
        sources: list[Run | Asset] | None = None,
        derived_from: list[Artifact | str] | None = None,
        related_to: list[Run | Asset | Artifact] | None = None,
        file_name: str | None = None,
        created_via: ArtifactCreatedVia | None = None,
        artifact_type: ArtifactType | None = None,
        # common filters
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[Artifact]:
        """List artifacts, each resolved to its latest version.

        `sources`, `derived_from`, and `related_to` match the latest version's links. For
        other fields of the linked entity, such as a run name, use `filter_query`.

        Args:
            title: Exact title of the latest version.
            titles: Filter to artifacts whose latest version has any of these titles.
            title_contains: Partial title of the latest version.
            artifact_ids: Filter to these Artifacts or artifact IDs.
            created_after: Filter to artifacts created after this datetime.
            created_before: Filter to artifacts created before this datetime.
            created_by: Filter to artifacts created by this User or user ID.
            metadata: Filter to artifacts whose latest version has all of these key-value
                pairs.
            sources: Filter to artifacts produced from any of these Runs or Assets.
            derived_from: Filter to artifacts derived from any of these Artifacts or
                artifact IDs.
            related_to: Filter to artifacts linked to any of these Runs, Assets, or
                Artifacts, by any relation.
            file_name: Exact file name of the latest version.
            created_via: Filter to artifacts written by this surface.
            artifact_type: Filter to artifacts of this type.
            include_archived: If True, include archived artifacts in results.
            filter_query: Explicit CEL query to filter artifacts. Links filter with a
                comprehension over `runs` (relation, run_id, name, description, client_key,
                start_time, stop_time), `assets` (relation, asset_id, name), `canvases`
                (relation, canvas_id, name, description), or `links` (relation,
                entity_type, entity_id).
            order_by: Field and direction to order results by, over `created_date`,
                `version_created_date`, `archived_date`, `title`, `version`, and
                `file_type`. The service default is `created_date` ascending.
            limit: Maximum number of artifacts to return. If None, returns all matches.
            page_size: Number of results to fetch per request.

        Returns:
            A list of Artifact objects that match the filter criteria.

        Example:
            Artifacts produced from a run whose name contains "thermal":

                client.artifacts.list_(
                    filter_query='runs.exists(r, r.relation == "SOURCE" && r.name.contains("thermal"))'
                )
        """
        filter_parts = self._build_artifact_cel_filters(
            title=title,
            titles=titles,
            title_contains=title_contains,
            artifact_ids=artifact_ids,
            created_after=created_after,
            created_before=created_before,
            created_by=created_by,
            metadata=metadata,
            sources=sources,
            derived_from=derived_from,
            related_to=related_to,
            file_name=file_name,
            created_via=created_via,
            artifact_type=artifact_type,
        )
        if include_archived:
            # The service excludes archived artifacts unless the filter names this directive.
            filter_parts.append(cel.equals("include_archived", True))
        filter_parts.extend(self._build_common_cel_filters(filter_query=filter_query))
        query_filter = cel.and_(*filter_parts)

        artifacts = await self._low_level_client.list_all_artifacts(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(artifacts)

    async def find(self, **kwargs) -> Artifact | None:
        """Find one artifact. Takes the same arguments as `list_`.

        Raises if more than one matches.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Artifact found or None.
        """
        artifacts = await self.list_(**kwargs)
        if len(artifacts) > 1:
            raise ValueError(f"Multiple ({len(artifacts)}) artifacts found for query")
        elif len(artifacts) == 1:
            return artifacts[0]
        return None

    async def create(
        self,
        create: ArtifactCreate | dict,
        *,
        file: str | Path | bytes | BinaryIO,
        file_name: str | None = None,
    ) -> Artifact:
        """Create an artifact from a file.

        Creates the artifact and its first version, then uploads the file to that version.

        Args:
            create: The artifact's title, summary, metadata, links, and surface.
            file: The file to upload: a path, bytes, or a binary file object such as
                `io.BytesIO`.
            file_name: The stored file name when `file` is bytes or a file object. A path
                upload uses the file's own name.

        Returns:
            The created Artifact, resolved to version 1.

        Raises:
            ValueError: If `file` is a path that is not a regular file, or `file_name` is
                missing for bytes, not a bare name, or given with a path.

        Example:
            Publish bytes built in memory, without writing a file first:

                client.artifacts.create(
                    {"title": "Peak temperatures"},
                    file=json.dumps(peaks).encode(),
                    file_name="peak_temperatures.json",
                )
        """
        if isinstance(create, dict):
            create = ArtifactCreate.model_validate(create)
        _check_file(file, file_name)

        created = await self._low_level_client.create_artifact(create=create)
        return await self._upload_and_get(created, file=file, file_name=file_name)

    async def update(
        self,
        artifact: str | Artifact,
        update: ArtifactUpdate | dict,
        *,
        file: str | Path | bytes | BinaryIO | None = None,
        file_name: str | None = None,
    ) -> Artifact:
        """Write a new version of an artifact.

        `ArtifactUpdate` says how each field carries forward. With `file`, the new version
        gets new bytes, and its only `SOURCE` and `DERIVED_FROM` links are the ones in
        `update.links`.

        Args:
            artifact: The Artifact or artifact ID to update.
            update: The fields to change.
            file: New bytes for the new version: a path, bytes, or a binary file object.
            file_name: The stored file name when `file` is bytes or a file object. A path
                upload uses the file's own name.

        Returns:
            The Artifact, resolved to the new version.

        Raises:
            ValueError: If the update sets no field and has no `file`, or `file` and
                `file_name` are invalid as for `create`.
        """
        if isinstance(update, dict):
            update = ArtifactUpdate.model_validate(update)
        if file is None and not update.model_dump(exclude_unset=True, exclude_none=True):
            raise ValueError("update needs at least one field or a file")
        if file is not None:
            _check_file(file, file_name)
        update.resource_id = _artifact_id(artifact)

        updated = await self._low_level_client.update_artifact(update, new_file=file is not None)
        if file is None:
            return self._apply_client_to_instance(updated)
        return await self._upload_and_get(updated, file=file, file_name=file_name)

    async def archive(self, artifact: str | Artifact) -> Artifact:
        """Archive an artifact. Its versions and files stay in place.

        Args:
            artifact: The Artifact or artifact ID to archive.

        Returns:
            The archived Artifact.
        """
        artifact_id = _artifact_id(artifact)
        await self._low_level_client.archive_artifact(artifact_id)
        return await self.get(artifact_id=artifact_id)

    async def unarchive(self, artifact: str | Artifact) -> Artifact:
        """Unarchive an artifact.

        Args:
            artifact: The Artifact or artifact ID to unarchive.

        Returns:
            The unarchived Artifact.
        """
        artifact_id = _artifact_id(artifact)
        await self._low_level_client.unarchive_artifact(artifact_id)
        return await self.get(artifact_id=artifact_id)

    async def get_download_url(self, *, artifact: str | Artifact | ArtifactVersion) -> str:
        """Get a download URL for an artifact version's file.

        Args:
            artifact: An ArtifactVersion, an Artifact (uses its `artifact_version`), or an
                artifact ID (uses the latest version).

        Returns:
            The download URL.

        Raises:
            ValueError: If the version has no file.
        """
        version = await self._resolve_version(artifact)
        if version.remote_file_id is None:
            raise ValueError(f"Artifact version {version.id_} has no file")
        return await self._remote_files_client.get_remote_file_download_url(
            remote_file_id=version.remote_file_id
        )

    async def download(
        self, *, artifact: str | Artifact | ArtifactVersion, output_path: str | Path
    ) -> None:
        """Download an artifact version's file to a local path.

        Args:
            artifact: An ArtifactVersion, an Artifact (uses its `artifact_version`), or an
                artifact ID (uses the latest version).
            output_path: The path to download the file to.

        Raises:
            ValueError: If the version has no file.
        """
        url = await self.get_download_url(artifact=artifact)
        await run_sync_function(
            lambda: download_file(url, Path(output_path), rest_client=self.client.rest_client)
        )

    async def read_bytes(self, *, artifact: str | Artifact | ArtifactVersion) -> bytes:
        """Read an artifact version's file into memory.

        This holds the whole file in memory. Use `download` for large files.

        Args:
            artifact: An ArtifactVersion, an Artifact (uses its `artifact_version`), or an
                artifact ID (uses the latest version).

        Returns:
            The file's bytes.

        Raises:
            ValueError: If the version has no file.

        Example:
            config = json.loads(client.artifacts.read_bytes(artifact=artifact))
        """
        url = await self.get_download_url(artifact=artifact)
        return await run_sync_function(
            lambda: read_url_bytes(url, rest_client=self.client.rest_client)
        )

    async def _resolve_version(self, artifact: str | Artifact | ArtifactVersion) -> ArtifactVersion:
        if isinstance(artifact, ArtifactVersion):
            return artifact
        if isinstance(artifact, Artifact):
            return artifact.artifact_version
        return (await self.get(artifact_id=artifact)).artifact_version

    async def _upload_and_get(
        self, artifact: Artifact, *, file: str | Path | bytes | BinaryIO, file_name: str | None
    ) -> Artifact:
        """Upload the file to the version `artifact` resolves to, then fetch that version.

        The version becomes the artifact's current version once the upload commits.
        """
        version_id = artifact.artifact_version._id_or_error
        await self._upload_client.upload_attachment(
            source=io.BytesIO(file) if isinstance(file, bytes) else file,
            entity_id=version_id,
            entity_type=ARTIFACT_VERSION_ENTITY_TYPE,
            file_name=file_name,
        )
        return await self.get(artifact_id=artifact._id_or_error, artifact_version_id=version_id)


def _artifact_id(artifact: str | Artifact) -> str:
    return artifact._id_or_error if isinstance(artifact, Artifact) else artifact


def _metadata_equals(key: str, value: str | float | bool) -> str:
    """Match one metadata key. The service takes the key as a quoted index, `metadata["key"]`."""
    quoted_key = json.dumps(key)
    if isinstance(value, (bool, str)):
        return cel.equals(f"metadata[{quoted_key}]", value)
    return f"metadata[{quoted_key}] == {float(value)}"


def _links_to(
    entities: Sequence[Run | Asset | Artifact | str],
    relation: ArtifactLinkRelation | None = None,
) -> str:
    """Match a version linked to any of `entities`, by `relation` when one is given.

    A string is an artifact ID, and is accepted only for DERIVED_FROM, where the entity
    type cannot be anything else.
    """
    targets = []
    for entity in entities:
        if isinstance(entity, Run):
            entity_type, entity_id = ArtifactEntityType.RUN, entity._id_or_error
        elif isinstance(entity, Asset):
            entity_type, entity_id = ArtifactEntityType.ASSET, entity._id_or_error
        elif isinstance(entity, Artifact):
            entity_type, entity_id = ArtifactEntityType.ARTIFACT, entity._id_or_error
        elif isinstance(entity, str) and relation == ArtifactLinkRelation.DERIVED_FROM:
            entity_type, entity_id = ArtifactEntityType.ARTIFACT, entity
        else:
            raise TypeError(
                f"Cannot filter by a link to {entity!r}; pass a Run, Asset, or Artifact"
            )
        targets.append(
            cel.parens(
                cel.and_(
                    cel.equals("l.entity_type", entity_type.name),
                    cel.equals("l.entity_id", entity_id),
                )
            )
        )
    match = cel.parens(cel.or_(*targets))
    if relation is not None:
        match = cel.and_(cel.equals("l.relation", relation.name), match)
    return f"links.exists(l, {match})"


def _check_file(file: str | Path | bytes | BinaryIO, file_name: str | None) -> None:
    """Reject a bad file before any RPC, so a failed upload cannot strand a new version."""
    if isinstance(file, (str, Path)):
        if file_name is not None:
            raise ValueError("file_name applies only when file is bytes or a file object")
        if not Path(file).is_file():
            raise ValueError(f"Provided path, '{file}', does not point to a regular file.")
        return
    if not file_name:
        raise ValueError("file_name is required when file is bytes or a file object")
    if Path(file_name).name != file_name:
        raise ValueError(f"file_name must be a bare file name, not a path: {file_name!r}")
