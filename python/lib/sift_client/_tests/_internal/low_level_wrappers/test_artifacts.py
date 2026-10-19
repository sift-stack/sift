"""Tests for the artifacts low-level wrapper."""

from unittest.mock import AsyncMock, MagicMock

import pytest
from google.protobuf.timestamp_pb2 import Timestamp
from sift.artifacts.v1 import artifacts_pb2

from sift_client._internal.low_level_wrappers.artifacts import ArtifactsLowLevelClient
from sift_client.sift_types import ArtifactCreate, ArtifactUpdate


def _client_with_stub(stub: MagicMock) -> ArtifactsLowLevelClient:
    grpc = MagicMock()
    grpc.get_stub.return_value = stub
    return ArtifactsLowLevelClient(grpc)


def _details(artifact_id: str = "a1", version_id: str = "v1") -> artifacts_pb2.ArtifactDetails:
    return artifacts_pb2.ArtifactDetails(
        artifact=artifacts_pb2.Artifact(
            artifact_id=artifact_id, current_version_id=version_id, created_date=Timestamp()
        ),
        artifact_version=artifacts_pb2.ArtifactVersion(
            artifact_version_id=version_id, artifact_id=artifact_id, created_date=Timestamp()
        ),
    )


def _version(version_id: str) -> artifacts_pb2.ArtifactVersion:
    return artifacts_pb2.ArtifactVersion(
        artifact_version_id=version_id, artifact_id="a1", created_date=Timestamp()
    )


class TestCreateArtifact:
    @pytest.mark.asyncio
    async def test_sends_create_request(self):
        stub = MagicMock()
        stub.CreateArtifact = AsyncMock(
            return_value=artifacts_pb2.CreateArtifactResponse(artifact=_details())
        )
        client = _client_with_stub(stub)

        artifact = await client.create_artifact(ArtifactCreate(title="t"))

        request = stub.CreateArtifact.call_args[0][0]
        assert request.title == "t"
        assert artifact.id_ == "a1"
        assert artifact.artifact_version.id_ == "v1"


class TestUpdateArtifact:
    @pytest.mark.asyncio
    async def test_keeps_bytes_by_default(self):
        stub = MagicMock()
        stub.UpdateArtifact = AsyncMock(
            return_value=artifacts_pb2.UpdateArtifactResponse(artifact=_details(version_id="v2"))
        )
        client = _client_with_stub(stub)
        update = ArtifactUpdate(title="t")
        update.resource_id = "a1"

        artifact = await client.update_artifact(update)

        request = stub.UpdateArtifact.call_args[0][0]
        assert request.artifact_id == "a1"
        assert list(request.update_mask.paths) == ["artifact_version.title"]
        assert artifact.artifact_version.id_ == "v2"

    @pytest.mark.asyncio
    async def test_new_file_declares_remote_file_id(self):
        stub = MagicMock()
        stub.UpdateArtifact = AsyncMock(
            return_value=artifacts_pb2.UpdateArtifactResponse(artifact=_details())
        )
        client = _client_with_stub(stub)
        update = ArtifactUpdate()
        update.resource_id = "a1"

        await client.update_artifact(update, new_file=True)

        request = stub.UpdateArtifact.call_args[0][0]
        assert list(request.update_mask.paths) == ["artifact_version.remote_file_id"]


class TestGetArtifact:
    @pytest.mark.asyncio
    async def test_latest(self):
        stub = MagicMock()
        stub.GetArtifact = AsyncMock(
            return_value=artifacts_pb2.GetArtifactResponse(artifact=_details())
        )
        client = _client_with_stub(stub)

        await client.get_artifact("a1")

        request = stub.GetArtifact.call_args[0][0]
        assert request.artifact_id == "a1"
        assert not request.HasField("artifact_version_id")

    @pytest.mark.asyncio
    async def test_pinned_version(self):
        stub = MagicMock()
        stub.GetArtifact = AsyncMock(
            return_value=artifacts_pb2.GetArtifactResponse(artifact=_details())
        )
        client = _client_with_stub(stub)

        await client.get_artifact("a1", artifact_version_id="v1")

        assert stub.GetArtifact.call_args[0][0].artifact_version_id == "v1"


class TestListAllArtifacts:
    @pytest.mark.asyncio
    async def test_follows_pagination_and_passes_query(self):
        stub = MagicMock()
        stub.ListArtifacts = AsyncMock(
            side_effect=[
                artifacts_pb2.ListArtifactsResponse(
                    artifacts=[_details("a1")], next_page_token="tok"
                ),
                artifacts_pb2.ListArtifactsResponse(artifacts=[_details("a2")]),
            ]
        )
        client = _client_with_stub(stub)

        artifacts = await client.list_all_artifacts(
            query_filter="title == 't'", order_by="created_date desc"
        )

        assert [a.id_ for a in artifacts] == ["a1", "a2"]
        first, second = (call[0][0] for call in stub.ListArtifacts.call_args_list)
        assert first.filter == "title == 't'"
        assert first.order_by == "created_date desc"
        assert second.page_token == "tok"


class TestListAllArtifactVersions:
    @pytest.mark.asyncio
    async def test_unset_artifact_searches_all(self):
        stub = MagicMock()
        stub.ListArtifactVersions = AsyncMock(
            return_value=artifacts_pb2.ListArtifactVersionsResponse(
                versions=[_version("v2"), _version("v1")]
            )
        )
        client = _client_with_stub(stub)

        versions = await client.list_all_artifact_versions(query_filter="version > 1")

        request = stub.ListArtifactVersions.call_args[0][0]
        assert request.artifact_id == ""
        assert request.filter == "version > 1"
        assert [v.id_ for v in versions] == ["v2", "v1"]

    @pytest.mark.asyncio
    async def test_named_artifact(self):
        stub = MagicMock()
        stub.ListArtifactVersions = AsyncMock(
            return_value=artifacts_pb2.ListArtifactVersionsResponse()
        )
        client = _client_with_stub(stub)

        await client.list_all_artifact_versions(artifact_id="a1", max_results=5)

        request = stub.ListArtifactVersions.call_args[0][0]
        assert request.artifact_id == "a1"
        assert request.page_size == 5


class TestSimpleCalls:
    @pytest.mark.asyncio
    async def test_promote(self):
        stub = MagicMock()
        stub.PromoteArtifactVersion = AsyncMock(
            return_value=artifacts_pb2.PromoteArtifactVersionResponse(
                artifact=_details(version_id="v3")
            )
        )
        client = _client_with_stub(stub)

        artifact = await client.promote_artifact_version("a1", "v1")

        request = stub.PromoteArtifactVersion.call_args[0][0]
        assert (request.artifact_id, request.artifact_version_id) == ("a1", "v1")
        assert artifact.artifact_version.id_ == "v3"

    @pytest.mark.asyncio
    async def test_archive_and_unarchive(self):
        stub = MagicMock()
        stub.ArchiveArtifact = AsyncMock()
        stub.UnarchiveArtifact = AsyncMock()
        client = _client_with_stub(stub)

        await client.archive_artifact("a1")
        await client.unarchive_artifact("a1")

        assert stub.ArchiveArtifact.call_args[0][0].artifact_id == "a1"
        assert stub.UnarchiveArtifact.call_args[0][0].artifact_id == "a1"
