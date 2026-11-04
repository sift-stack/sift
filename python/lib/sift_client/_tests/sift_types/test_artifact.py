"""Tests for sift_types.Artifact models."""

from __future__ import annotations

from datetime import datetime, timezone
from typing import Any
from unittest.mock import MagicMock

import pytest
from sift.artifacts.v1 import artifacts_pb2

from sift_client._internal.time import to_timestamp_pb
from sift_client.sift_types import (
    Artifact,
    ArtifactCreate,
    ArtifactCreatedVia,
    ArtifactEntityType,
    ArtifactLink,
    ArtifactLinkRelation,
    ArtifactType,
    ArtifactUpdate,
    ArtifactVersion,
)
from sift_client.util.metadata import metadata_dict_to_proto

NOW = datetime(2026, 1, 1, tzinfo=timezone.utc)


def _version_proto(**overrides) -> artifacts_pb2.ArtifactVersion:
    fields: dict[str, Any] = dict(
        artifact_version_id="v2",
        artifact_id="a1",
        version=2,
        title="Thermal summary",
        remote_file_id="rf1",
        file_name="summary.png",
        file_mime_type="image/png",
        created_date=to_timestamp_pb(NOW),
        metadata=metadata_dict_to_proto({"campaign": "x"}),
        is_current=True,
    )
    fields.update(overrides)
    return artifacts_pb2.ArtifactVersion(**fields)


def _details_proto(**artifact_overrides) -> artifacts_pb2.ArtifactDetails:
    fields: dict[str, Any] = dict(
        artifact_id="a1",
        organization_id="org1",
        created_by_user_id="u1",
        created_date=to_timestamp_pb(NOW),
        storage_class=artifacts_pb2.ARTIFACT_STORAGE_CLASS_FILE,
        created_via=artifacts_pb2.ARTIFACT_CREATED_VIA_UPLOAD,
        current_version_id="v2",
    )
    fields.update(artifact_overrides)
    return artifacts_pb2.ArtifactDetails(
        artifact=artifacts_pb2.Artifact(**fields), artifact_version=_version_proto()
    )


def _bound(model, client):
    model._apply_client_to_instance(client)
    return model


SOURCE_RUN = ArtifactLink(
    relation=ArtifactLinkRelation.SOURCE, entity_type=ArtifactEntityType.RUN, entity_id="run1"
)


class TestArtifactVersion:
    """Unit tests for ArtifactVersion._from_proto."""

    def test_from_proto(self):
        version = ArtifactVersion._from_proto(_version_proto())

        assert version.id_ == "v2"
        assert version.artifact_id == "a1"
        assert version.version == 2
        assert version.title == "Thermal summary"
        assert version.summary is None
        assert version.remote_file_id == "rf1"
        assert version.file_name == "summary.png"
        assert version.metadata == {"campaign": "x"}
        assert version.created_date == NOW
        assert version.is_current is True

    def test_unset_optionals_are_none(self):
        version = ArtifactVersion._from_proto(
            artifacts_pb2.ArtifactVersion(
                artifact_version_id="v1", artifact_id="a1", created_date=to_timestamp_pb(NOW)
            )
        )

        assert version.title is None
        assert version.remote_file_id is None
        assert version.file_name is None
        assert version.file_mime_type is None
        assert version.is_current is False


class TestArtifact:
    """Unit tests for Artifact._from_proto and its helpers."""

    def test_from_proto(self):
        artifact = Artifact._from_proto(_details_proto())

        assert artifact.id_ == "a1"
        assert artifact.organization_id == "org1"
        assert artifact.artifact_type is ArtifactType.FILE
        assert artifact.created_via is ArtifactCreatedVia.UPLOAD
        assert artifact.current_version_id == "v2"
        assert artifact.artifact_version.id_ == "v2"
        assert artifact.archived_date is None
        assert artifact.is_archived is False

    def test_unmodeled_enum_values_read_as_none(self):
        artifact = Artifact._from_proto(
            _details_proto(
                storage_class=artifacts_pb2.ARTIFACT_STORAGE_CLASS_STRUCTURED,
                created_via=artifacts_pb2.ARTIFACT_CREATED_VIA_UNSPECIFIED,
            )
        )

        assert artifact.artifact_type is None
        assert artifact.created_via is None

    def test_archived(self):
        artifact = Artifact._from_proto(_details_proto(archived_date=to_timestamp_pb(NOW)))

        assert artifact.archived_date == NOW
        assert artifact.is_archived is True

    def test_client_reaches_the_version(self, mock_client):
        artifact = Artifact._from_proto(_details_proto())

        artifact._apply_client_to_instance(mock_client)

        assert artifact.artifact_version.client is mock_client

    def test_versions_property(self, mock_client):
        artifact = _bound(Artifact._from_proto(_details_proto()), mock_client)
        mock_client.artifacts.versions.list_.return_value = []

        assert artifact.versions == []
        mock_client.artifacts.versions.list_.assert_called_once_with(artifact=artifact)

    def test_created_by(self, mock_client):
        artifact = _bound(Artifact._from_proto(_details_proto()), mock_client)

        mock_client.users = MagicMock()

        assert artifact.created_by is mock_client.users.get.return_value
        mock_client.users.get.assert_called_once_with(user_id="u1")

    def test_update_refreshes_in_place(self, mock_client):
        artifact = _bound(Artifact._from_proto(_details_proto()), mock_client)
        newer = _bound(Artifact._from_proto(_details_proto(current_version_id="v3")), mock_client)
        mock_client.artifacts.update.return_value = newer

        result = artifact.update({"title": "new"}, file=b"data", file_name="a.txt")

        mock_client.artifacts.update.assert_called_once_with(
            artifact, {"title": "new"}, file=b"data", file_name="a.txt"
        )
        assert result is artifact
        assert artifact.current_version_id == "v3"

    def test_archive_refreshes_in_place(self, mock_client):
        artifact = _bound(Artifact._from_proto(_details_proto()), mock_client)
        mock_client.artifacts.archive.return_value = Artifact._from_proto(
            _details_proto(archived_date=to_timestamp_pb(NOW))
        )

        artifact.archive()

        assert artifact.is_archived is True

    def test_unarchive_refreshes_in_place(self, mock_client):
        artifact = _bound(
            Artifact._from_proto(_details_proto(archived_date=to_timestamp_pb(NOW))), mock_client
        )
        mock_client.artifacts.unarchive.return_value = Artifact._from_proto(_details_proto())

        artifact.unarchive()

        mock_client.artifacts.unarchive.assert_called_once_with(artifact)
        assert artifact.is_archived is False

    def test_version_artifact_is_pinned(self, mock_client):
        version = _bound(ArtifactVersion._from_proto(_version_proto()), mock_client)

        assert version.artifact is mock_client.artifacts.get.return_value
        mock_client.artifacts.get.assert_called_once_with(
            artifact_id="a1", artifact_version_id="v2"
        )

    def test_version_promote_and_download(self, mock_client, tmp_path):
        version = _bound(ArtifactVersion._from_proto(_version_proto()), mock_client)

        version.promote()
        version.download(tmp_path / "out.png")

        mock_client.artifacts.versions.promote.assert_called_once_with(version=version)
        assert version.read_bytes() is mock_client.artifacts.read_bytes.return_value
        mock_client.artifacts.read_bytes.assert_called_once_with(artifact=version)
        mock_client.artifacts.download.assert_called_once_with(
            artifact=version, output_path=tmp_path / "out.png"
        )


class TestArtifactCreate:
    """Unit tests for ArtifactCreate.to_proto."""

    def test_to_proto(self):
        proto = ArtifactCreate(
            title="Thermal summary",
            summary="Peak temperatures",
            metadata={"campaign": "x"},
            links=[SOURCE_RUN],
            created_via=ArtifactCreatedVia.UPLOAD,
            artifact_type=ArtifactType.BLOB,
        ).to_proto()

        assert proto.title == "Thermal summary"
        assert proto.summary == "Peak temperatures"
        assert proto.created_via == artifacts_pb2.ARTIFACT_CREATED_VIA_UPLOAD
        assert proto.storage_class == artifacts_pb2.ARTIFACT_STORAGE_CLASS_BLOB
        assert proto.metadata[0].key.name == "campaign"
        assert len(proto.links) == 1
        assert proto.links[0].relation == artifacts_pb2.ARTIFACT_LINK_RELATION_SOURCE
        assert proto.links[0].entity_type == artifacts_pb2.ARTIFACT_ENTITY_TYPE_RUN
        assert proto.links[0].entity_id == "run1"

    def test_unset_optionals_stay_unset(self):
        proto = ArtifactCreate().to_proto()

        assert not proto.HasField("title")
        assert proto.created_via == artifacts_pb2.ARTIFACT_CREATED_VIA_UPLOAD
        assert not proto.HasField("storage_class")
        assert len(proto.links) == 0


class TestArtifactUpdate:
    """Unit tests for ArtifactUpdate.to_proto_with_mask."""

    def test_mask_names_artifact_version_paths(self):
        update = ArtifactUpdate(title="New", summary="Updated", metadata={"campaign": "y"})
        update.resource_id = "a1"

        proto, mask = update.to_proto_with_mask()

        assert proto.artifact_id == "a1"
        assert proto.artifact.artifact_version.title == "New"
        assert proto.artifact.artifact_version.summary == "Updated"
        assert proto.artifact.artifact_version.metadata[0].string_value == "y"
        assert list(mask.paths) == [
            "artifact_version.title",
            "artifact_version.summary",
            "artifact_version.metadata",
        ]

    def test_links_are_request_level(self):
        update = ArtifactUpdate(links=[SOURCE_RUN])
        update.resource_id = "a1"

        proto, mask = update.to_proto_with_mask()

        assert list(mask.paths) == ["links"]
        assert proto.links[0].entity_id == "run1"
        assert not proto.HasField("artifact")

    def test_requires_resource_id(self):
        with pytest.raises(ValueError, match="Resource ID"):
            ArtifactUpdate(title="New").to_proto_with_mask()
