from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, BinaryIO, ClassVar, TypeVar

from pydantic import BaseModel, model_validator
from sift.artifacts.v1.artifacts_pb2 import (
    ArtifactCreatedVia as ArtifactCreatedViaProto,
)
from sift.artifacts.v1.artifacts_pb2 import (
    ArtifactDetails as ArtifactDetailsProto,
)
from sift.artifacts.v1.artifacts_pb2 import (
    ArtifactEntityType as ArtifactEntityTypeProto,
)
from sift.artifacts.v1.artifacts_pb2 import (
    ArtifactLinkInput as ArtifactLinkInputProto,
)
from sift.artifacts.v1.artifacts_pb2 import (
    ArtifactLinkRelation as ArtifactLinkRelationProto,
)
from sift.artifacts.v1.artifacts_pb2 import (
    ArtifactStorageClass as ArtifactStorageClassProto,
)
from sift.artifacts.v1.artifacts_pb2 import (
    ArtifactVersion as ArtifactVersionProto,
)
from sift.artifacts.v1.artifacts_pb2 import (
    CreateArtifactRequest as CreateArtifactRequestProto,
)
from sift.artifacts.v1.artifacts_pb2 import (
    UpdateArtifactRequest as UpdateArtifactRequestProto,
)

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from pathlib import Path

    from sift_client.client import SiftClient
    from sift_client.sift_types.user import User


class ArtifactType(Enum):
    """How the Sift app presents an artifact's file."""

    FILE = ArtifactStorageClassProto.ARTIFACT_STORAGE_CLASS_FILE  # 1
    """Bytes in a remote file that the Sift app can preview by MIME type."""
    BLOB = ArtifactStorageClassProto.ARTIFACT_STORAGE_CLASS_BLOB  # 3
    """Bytes in a remote file that the Sift app offers only as a download."""


class ArtifactCreatedVia(Enum):
    """The surface that wrote an artifact."""

    AGENT = ArtifactCreatedViaProto.ARTIFACT_CREATED_VIA_AGENT  # 1
    CANVAS = ArtifactCreatedViaProto.ARTIFACT_CREATED_VIA_CANVAS  # 2
    UPLOAD = ArtifactCreatedViaProto.ARTIFACT_CREATED_VIA_UPLOAD  # 3


class ArtifactLinkRelation(Enum):
    """How a link relates an artifact version to another entity.

    `ATTACHED_TO` links carry forward to each new version. `SOURCE` and `DERIVED_FROM`
    links describe the bytes, so they carry forward only while the bytes do.
    """

    ATTACHED_TO = ArtifactLinkRelationProto.ARTIFACT_LINK_RELATION_ATTACHED_TO  # 1
    """Where the artifact surfaces, such as a conversation or canvas."""
    SOURCE = ArtifactLinkRelationProto.ARTIFACT_LINK_RELATION_SOURCE  # 2
    """What the artifact was produced from, such as a run or asset."""
    DERIVED_FROM = ArtifactLinkRelationProto.ARTIFACT_LINK_RELATION_DERIVED_FROM  # 3
    """Another artifact this one was derived from."""


class ArtifactEntityType(Enum):
    """The kind of entity an artifact link points at."""

    CONVERSATION = ArtifactEntityTypeProto.ARTIFACT_ENTITY_TYPE_CONVERSATION  # 1
    CANVAS = ArtifactEntityTypeProto.ARTIFACT_ENTITY_TYPE_CANVAS  # 2
    RUN = ArtifactEntityTypeProto.ARTIFACT_ENTITY_TYPE_RUN  # 3
    ASSET = ArtifactEntityTypeProto.ARTIFACT_ENTITY_TYPE_ASSET  # 4
    ARTIFACT = ArtifactEntityTypeProto.ARTIFACT_ENTITY_TYPE_ARTIFACT  # 5
    TOOL_USE = ArtifactEntityTypeProto.ARTIFACT_ENTITY_TYPE_TOOL_USE  # 6


EnumT = TypeVar("EnumT", bound=Enum)


def _enum_or_none(enum_cls: type[EnumT], value: int) -> EnumT | None:
    """Map a proto enum value to `enum_cls`, or None for a value the client does not model."""
    try:
        return enum_cls(value)
    except ValueError:
        return None


class ArtifactLink(BaseModel):
    """A link to write on an artifact version.

    Links are write-only: the service stores them, and filters can match on them, but
    the client cannot read them back.

    Attributes:
        relation: How the artifact relates to the entity.
        entity_type: The kind of entity.
        entity_id: The entity's ID, such as a run ID.
    """

    relation: ArtifactLinkRelation
    entity_type: ArtifactEntityType
    entity_id: str


def _artifact_link_to_proto(**kwargs) -> ArtifactLinkInputProto:
    """Convert an artifact link dict (from model_dump) into its proto form."""
    relation = kwargs["relation"]
    entity_type = kwargs["entity_type"]
    return ArtifactLinkInputProto(
        relation=relation.value if isinstance(relation, ArtifactLinkRelation) else relation,
        entity_type=(
            entity_type.value if isinstance(entity_type, ArtifactEntityType) else entity_type
        ),
        entity_id=kwargs["entity_id"],
    )


class ArtifactVersion(BaseType[ArtifactVersionProto, "ArtifactVersion"]):
    """One version of an artifact. `id_` is the version ID."""

    artifact_id: str
    version: int
    title: str | None
    summary: str | None
    remote_file_id: str | None
    file_name: str | None
    file_mime_type: str | None
    metadata: dict[str, str | float | bool]
    created_date: datetime
    is_current: bool
    """Whether this is the version `get` returns without a pinned version."""

    @classmethod
    def _from_proto(
        cls, proto: ArtifactVersionProto, sift_client: SiftClient | None = None
    ) -> ArtifactVersion:
        return cls(
            proto=proto,
            id_=proto.artifact_version_id,
            artifact_id=proto.artifact_id,
            version=proto.version,
            title=proto.title if proto.HasField("title") else None,
            summary=proto.summary if proto.HasField("summary") else None,
            remote_file_id=proto.remote_file_id if proto.HasField("remote_file_id") else None,
            file_name=proto.file_name if proto.HasField("file_name") else None,
            file_mime_type=proto.file_mime_type if proto.HasField("file_mime_type") else None,
            metadata=metadata_proto_to_dict(list(proto.metadata)),
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            is_current=proto.is_current,
            _client=sift_client,
        )

    @property
    def artifact(self) -> Artifact:
        """Fetch the artifact, resolved to this version."""
        return self.client.artifacts.get(
            artifact_id=self.artifact_id, artifact_version_id=self._id_or_error
        )

    def promote(self) -> Artifact:
        """Write a new latest version that copies this version's content.

        Returns:
            The artifact, resolved to the new version that the promotion wrote.
        """
        return self.client.artifacts.versions.promote(version=self)

    def download(self, output_path: str | Path) -> None:
        """Download this version's file to a local path."""
        self.client.artifacts.download(artifact=self, output_path=output_path)

    def read_bytes(self) -> bytes:
        """Read this version's file into memory. See `ArtifactsAPI.read_bytes`."""
        return self.client.artifacts.read_bytes(artifact=self)


class Artifact(BaseType[ArtifactDetailsProto, "Artifact"]):
    """A versioned file, resolved to one of its versions.

    The container fields are the same for every version. Title, summary, file, and
    metadata live on `artifact_version`, which is the latest version unless the
    artifact was fetched with a pinned version.
    """

    organization_id: str
    created_by_user_id: str
    created_date: datetime
    artifact_type: ArtifactType | None
    """None for a storage class the client does not model."""
    created_via: ArtifactCreatedVia | None
    current_version_id: str
    """The ID of the artifact's latest version."""
    artifact_version: ArtifactVersion
    is_archived: bool
    archived_date: datetime | None

    @classmethod
    def _from_proto(
        cls, proto: ArtifactDetailsProto, sift_client: SiftClient | None = None
    ) -> Artifact:
        artifact = proto.artifact
        return cls(
            proto=proto,
            id_=artifact.artifact_id,
            organization_id=artifact.organization_id,
            created_by_user_id=artifact.created_by_user_id,
            created_date=artifact.created_date.ToDatetime(tzinfo=timezone.utc),
            artifact_type=_enum_or_none(ArtifactType, artifact.storage_class),
            created_via=_enum_or_none(ArtifactCreatedVia, artifact.created_via),
            current_version_id=artifact.current_version_id,
            artifact_version=ArtifactVersion._from_proto(proto.artifact_version, sift_client),
            is_archived=artifact.HasField("archived_date"),
            archived_date=(
                artifact.archived_date.ToDatetime(tzinfo=timezone.utc)
                if artifact.HasField("archived_date")
                else None
            ),
            _client=sift_client,
        )

    def _apply_client_to_instance(self, client: SiftClient) -> None:
        super()._apply_client_to_instance(client)
        self.artifact_version._apply_client_to_instance(client)

    @property
    def versions(self) -> list[ArtifactVersion]:
        """Fetch every version of this artifact, newest first."""
        return self.client.artifacts.versions.list_(artifact=self)

    @property
    def created_by(self) -> User:
        """Fetch the User that created this artifact."""
        return self.client.users.get(user_id=self.created_by_user_id)

    def update(
        self,
        update: ArtifactUpdate | dict,
        *,
        file: str | Path | bytes | BinaryIO | None = None,
        file_name: str | None = None,
    ) -> Artifact:
        """Write a new version of the artifact. See `ArtifactsAPI.update`.

        Args:
            update: The fields to change.
            file: New bytes for the new version: a path, bytes, or a binary file object.
            file_name: The stored file name. Required when `file` is bytes or a file object.

        Returns:
            The artifact, resolved to the new version.
        """
        updated = self.client.artifacts.update(self, update, file=file, file_name=file_name)
        self._update(updated)
        return self

    def archive(self) -> Artifact:
        """Archive the artifact."""
        updated = self.client.artifacts.archive(self)
        self._update(updated)
        return self

    def unarchive(self) -> Artifact:
        """Unarchive the artifact."""
        updated = self.client.artifacts.unarchive(self)
        self._update(updated)
        return self

    def download(self, output_path: str | Path) -> None:
        """Download the file of `artifact_version` to a local path."""
        self.client.artifacts.download(artifact=self, output_path=output_path)

    def read_bytes(self) -> bytes:
        """Read the file of `artifact_version` into memory. See `ArtifactsAPI.read_bytes`."""
        return self.client.artifacts.read_bytes(artifact=self)


class ArtifactBase(ModelCreateUpdateBase):
    """Base class for Artifact create and update models."""

    title: str | None = None
    summary: str | None = None
    metadata: dict[str, str | float | bool] | None = None
    links: list[ArtifactLink] | None = None


class ArtifactCreate(ArtifactBase, ModelCreate[CreateArtifactRequestProto]):
    """Create model for Artifact.

    `links` are written at version 1, for example `SOURCE` links to the runs the file
    was produced from.
    """

    created_via: ArtifactCreatedVia | None = None
    """The surface that wrote the artifact. Defaults to `UPLOAD`."""
    artifact_type: ArtifactType | None = None
    """The service defaults to `FILE`."""

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "artifact_type": MappingHelper(proto_attr_path="storage_class"),
        "metadata": MappingHelper(
            proto_attr_path="metadata",
            converter=metadata_dict_to_proto,
        ),
        "links": MappingHelper(
            proto_attr_path="links",
            converter=_artifact_link_to_proto,  # type: ignore[arg-type]
        ),
    }

    @model_validator(mode="after")
    def _default_created_via(self) -> ArtifactCreate:
        # The service would record AGENT, and an AGENT artifact's links name no user.
        # Assigning marks the field as set, so `to_proto` sends it.
        if self.created_via is None:
            self.created_via = ArtifactCreatedVia.UPLOAD
        return self

    def _get_proto_class(self) -> type[CreateArtifactRequestProto]:
        return CreateArtifactRequestProto


class ArtifactUpdate(ArtifactBase, ModelUpdate[UpdateArtifactRequestProto]):
    """Update model for Artifact.

    Each update writes a new version. Fields left unset are copied from the latest
    version. `metadata` replaces the latest version's metadata as a whole. `links` are
    added to the links that carry forward.
    """

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "title": MappingHelper(
            proto_attr_path="artifact.artifact_version.title",
            update_field="artifact_version.title",
        ),
        "summary": MappingHelper(
            proto_attr_path="artifact.artifact_version.summary",
            update_field="artifact_version.summary",
        ),
        "metadata": MappingHelper(
            proto_attr_path="artifact.artifact_version.metadata",
            update_field="artifact_version.metadata",
            converter=metadata_dict_to_proto,
        ),
        "links": MappingHelper(
            proto_attr_path="links",
            update_field="links",
            converter=_artifact_link_to_proto,  # type: ignore[arg-type]
        ),
    }

    def _get_proto_class(self) -> type[UpdateArtifactRequestProto]:
        return UpdateArtifactRequestProto

    def _add_resource_id_to_proto(self, proto_msg: UpdateArtifactRequestProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.artifact_id = self._resource_id
