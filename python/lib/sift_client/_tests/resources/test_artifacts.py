"""Pytest tests for the Artifacts API.

These tests demonstrate and validate the usage of the ArtifactsAPI including:
- Filter building for artifacts and version history
- Create and update with a file upload
- Download of a version's file
- An integration flow against a live Sift instance
"""

from __future__ import annotations

import io
from datetime import datetime, timezone
from unittest.mock import AsyncMock, MagicMock, patch

import pytest
from google.protobuf.timestamp_pb2 import Timestamp
from sift.artifacts.v1 import artifacts_pb2

from sift_client.resources import ArtifactsAPI, ArtifactsAPIAsync, ArtifactVersionsAPI
from sift_client.sift_types import (
    Artifact,
    ArtifactCreatedVia,
    ArtifactEntityType,
    ArtifactLink,
    ArtifactLinkRelation,
    ArtifactType,
    ArtifactVersion,
)
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.run import Run


def _artifact(version_id: str = "v1", remote_file_id: str | None = None) -> Artifact:
    return Artifact._from_proto(
        artifacts_pb2.ArtifactDetails(
            artifact=artifacts_pb2.Artifact(
                artifact_id="a1", current_version_id=version_id, created_date=Timestamp()
            ),
            artifact_version=artifacts_pb2.ArtifactVersion(
                artifact_version_id=version_id,
                artifact_id="a1",
                remote_file_id=remote_file_id,
                created_date=Timestamp(),
            ),
        )
    )


def _run(run_id: str) -> Run:
    run = MagicMock(spec=Run)
    run._id_or_error = run_id
    return run


def _asset(asset_id: str) -> Asset:
    asset = MagicMock(spec=Asset)
    asset._id_or_error = asset_id
    return asset


@pytest.fixture
def artifacts_api():
    client = MagicMock()
    with patch("sift_client.resources.artifacts.ArtifactsLowLevelClient", autospec=True), patch(
        "sift_client.resources.artifacts.RemoteFilesLowLevelClient", autospec=True
    ), patch("sift_client.resources.artifacts.UploadLowLevelClient", autospec=True):
        api = ArtifactsAPIAsync(client)
    api._low_level_client.get_artifact = AsyncMock(return_value=_artifact("v1", "rf1"))
    api._low_level_client.create_artifact = AsyncMock(return_value=_artifact("v1"))
    api._low_level_client.update_artifact = AsyncMock(return_value=_artifact("v2"))
    api._low_level_client.list_all_artifacts = AsyncMock(return_value=[])
    api._low_level_client.list_all_artifact_versions = AsyncMock(return_value=[])
    api._low_level_client.archive_artifact = AsyncMock()
    api._low_level_client.unarchive_artifact = AsyncMock()
    api._low_level_client.promote_artifact_version = AsyncMock(return_value=_artifact("v3"))
    api._upload_client.upload_attachment = AsyncMock(return_value="rf1")
    api._remote_files_client.get_remote_file_download_url = AsyncMock(return_value="https://x")
    return api


class TestListFilters:
    @pytest.mark.asyncio
    async def test_no_arguments_sends_no_filter(self, artifacts_api):
        await artifacts_api.list_()

        kwargs = artifacts_api._low_level_client.list_all_artifacts.call_args.kwargs
        assert kwargs["query_filter"] is None

    @pytest.mark.asyncio
    async def test_combines_filters(self, artifacts_api):
        await artifacts_api.list_(
            title_contains="thermal",
            created_via=ArtifactCreatedVia.CANVAS,
            artifact_type=ArtifactType.FILE,
            include_archived=True,
            filter_query='runs.exists(r, r.relation == "SOURCE" && r.name.contains("hot"))',
            order_by="created_date desc",
            limit=10,
        )

        kwargs = artifacts_api._low_level_client.list_all_artifacts.call_args.kwargs
        assert kwargs["query_filter"] == (
            "title.contains('thermal') && created_via == 'CANVAS' && storage_class == 'FILE'"
            " && include_archived == true"
            ' && runs.exists(r, r.relation == "SOURCE" && r.name.contains("hot"))'
        )
        assert kwargs["order_by"] == "created_date desc"
        assert kwargs["max_results"] == 10

    @pytest.mark.asyncio
    async def test_ids_and_creation(self, artifacts_api):
        created_after = datetime(2026, 1, 1, tzinfo=timezone.utc)

        await artifacts_api.list_(
            artifact_ids=["a1", "a2"],
            created_after=created_after,
            created_by="u1",
            file_name="plot.png",
        )

        query = artifacts_api._low_level_client.list_all_artifacts.call_args.kwargs["query_filter"]
        assert query.split(" && ") == [
            "created_date > timestamp('2026-01-01T00:00:00+00:00')",
            "created_by_user_id == 'u1'",
            "artifact_id in ['a1','a2']",
            "file_name == 'plot.png'",
        ]

    @pytest.mark.asyncio
    async def test_titles_and_metadata(self, artifacts_api):
        await artifacts_api.list_(
            titles=["a", "b"], metadata={"campaign": "qual-2", "rig": 4, "passed": True}
        )

        query = artifacts_api._low_level_client.list_all_artifacts.call_args.kwargs["query_filter"]
        assert query.split(" && ") == [
            "title in ['a','b']",
            "metadata[\"campaign\"] == 'qual-2'",
            'metadata["rig"] == 4.0',
            'metadata["passed"] == true',
        ]

    @pytest.mark.asyncio
    async def test_sources_match_any_entity_by_relation(self, artifacts_api):
        await artifacts_api.list_(sources=[_run("r1"), _asset("as1")])

        query = artifacts_api._low_level_client.list_all_artifacts.call_args.kwargs["query_filter"]
        assert query == (
            "links.exists(l, l.relation == 'SOURCE'"
            " && ((l.entity_type == 'RUN' && l.entity_id == 'r1')"
            " || (l.entity_type == 'ASSET' && l.entity_id == 'as1')))"
        )

    @pytest.mark.asyncio
    async def test_derived_from_takes_artifacts_or_ids(self, artifacts_api):
        await artifacts_api.list_(derived_from=[_artifact(), "a2"])

        query = artifacts_api._low_level_client.list_all_artifacts.call_args.kwargs["query_filter"]
        assert query == (
            "links.exists(l, l.relation == 'DERIVED_FROM'"
            " && ((l.entity_type == 'ARTIFACT' && l.entity_id == 'a1')"
            " || (l.entity_type == 'ARTIFACT' && l.entity_id == 'a2')))"
        )

    @pytest.mark.asyncio
    async def test_related_to_ignores_relation(self, artifacts_api):
        await artifacts_api.list_(related_to=[_run("r1")])

        query = artifacts_api._low_level_client.list_all_artifacts.call_args.kwargs["query_filter"]
        assert query == "links.exists(l, ((l.entity_type == 'RUN' && l.entity_id == 'r1')))"

    @pytest.mark.asyncio
    async def test_unsupported_link_target_rejected(self, artifacts_api):
        with pytest.raises(TypeError, match="Cannot filter by a link"):
            await artifacts_api.list_(related_to=["r1"])  # type: ignore[list-item]

    @pytest.mark.asyncio
    async def test_find_raises_on_many(self, artifacts_api):
        artifacts_api._low_level_client.list_all_artifacts.return_value = [
            _artifact(),
            _artifact(),
        ]

        with pytest.raises(ValueError, match="Multiple"):
            await artifacts_api.find(title="t")


class TestListVersions:
    @pytest.mark.asyncio
    async def test_search_all_with_archived(self, artifacts_api):
        await artifacts_api.versions.list_(include_archived=True, filter_query="version > 1")

        kwargs = artifacts_api._low_level_client.list_all_artifact_versions.call_args.kwargs
        assert kwargs["artifact_id"] is None
        assert kwargs["query_filter"] == "include_archived == true && version > 1"

    @pytest.mark.asyncio
    async def test_takes_the_list_filters(self, artifacts_api):
        await artifacts_api.versions.list_(
            title_contains="thermal", sources=[_run("r1")], artifact_type=ArtifactType.BLOB
        )

        query = artifacts_api._low_level_client.list_all_artifact_versions.call_args.kwargs[
            "query_filter"
        ]
        assert query == (
            "title.contains('thermal') && links.exists(l, l.relation == 'SOURCE'"
            " && ((l.entity_type == 'RUN' && l.entity_id == 'r1'))) && storage_class == 'BLOB'"
        )

    @pytest.mark.asyncio
    async def test_named_artifact_skips_archive_directive(self, artifacts_api):
        # The service always lists a named artifact's versions, archived or not.
        await artifacts_api.versions.list_(artifact=_artifact(), include_archived=True)

        kwargs = artifacts_api._low_level_client.list_all_artifact_versions.call_args.kwargs
        assert kwargs["artifact_id"] == "a1"
        assert kwargs["query_filter"] is None


class TestCreate:
    @pytest.mark.asyncio
    async def test_uploads_path_to_new_version(self, artifacts_api, tmp_path):
        path = tmp_path / "plot.png"
        path.write_bytes(b"png")
        links = [
            ArtifactLink(
                relation=ArtifactLinkRelation.SOURCE,
                entity_type=ArtifactEntityType.RUN,
                entity_id="run1",
            )
        ]

        artifact = await artifacts_api.create({"title": "Plot", "links": links}, file=path)

        create = artifacts_api._low_level_client.create_artifact.call_args.kwargs["create"]
        assert create.title == "Plot"
        assert create.links == links
        assert create.created_via is ArtifactCreatedVia.UPLOAD
        artifacts_api._upload_client.upload_attachment.assert_awaited_once_with(
            source=path, entity_id="v1", entity_type="artifact_versions", file_name=None
        )
        artifacts_api._low_level_client.get_artifact.assert_awaited_once_with(
            artifact_id="a1", artifact_version_id="v1"
        )
        assert artifact.artifact_version.remote_file_id == "rf1"

    @pytest.mark.asyncio
    async def test_uploads_bytes_under_file_name(self, artifacts_api):
        await artifacts_api.create({"title": "Notes"}, file=b"hello", file_name="notes.txt")

        kwargs = artifacts_api._upload_client.upload_attachment.call_args.kwargs
        assert kwargs["source"].read() == b"hello"
        assert kwargs["file_name"] == "notes.txt"
        assert kwargs["entity_id"] == "v1"

    @pytest.mark.asyncio
    async def test_uploads_file_object(self, artifacts_api):
        data = io.BytesIO(b"a,b\n1,2\n")

        await artifacts_api.create({}, file=data, file_name="table.csv")

        kwargs = artifacts_api._upload_client.upload_attachment.call_args.kwargs
        assert kwargs["source"] is data
        assert kwargs["file_name"] == "table.csv"

    @pytest.mark.asyncio
    async def test_file_object_needs_file_name(self, artifacts_api):
        with pytest.raises(ValueError, match="file_name is required"):
            await artifacts_api.create({}, file=io.BytesIO(b"x"))

    @pytest.mark.asyncio
    async def test_bytes_need_file_name(self, artifacts_api):
        with pytest.raises(ValueError, match="file_name is required"):
            await artifacts_api.create({}, file=b"hello")

    @pytest.mark.asyncio
    async def test_file_name_rejected_for_paths(self, artifacts_api, tmp_path):
        with pytest.raises(ValueError, match="only when file is bytes"):
            await artifacts_api.create({}, file=tmp_path / "a.txt", file_name="b.txt")

    @pytest.mark.asyncio
    async def test_file_name_must_be_bare(self, artifacts_api):
        with pytest.raises(ValueError, match="bare file name"):
            await artifacts_api.create({}, file=b"x", file_name="../escape.txt")
        artifacts_api._low_level_client.create_artifact.assert_not_awaited()

    @pytest.mark.asyncio
    async def test_missing_path_rejected_before_create(self, artifacts_api, tmp_path):
        with pytest.raises(ValueError, match="does not point to a regular file"):
            await artifacts_api.create({}, file=tmp_path / "missing.png")
        artifacts_api._low_level_client.create_artifact.assert_not_awaited()

    @pytest.mark.asyncio
    async def test_keeps_explicit_created_via(self, artifacts_api):
        await artifacts_api.create(
            {"created_via": ArtifactCreatedVia.CANVAS}, file=b"x", file_name="x.txt"
        )

        create = artifacts_api._low_level_client.create_artifact.call_args.kwargs["create"]
        assert create.created_via is ArtifactCreatedVia.CANVAS


class TestUpdate:
    @pytest.mark.asyncio
    async def test_fields_only(self, artifacts_api):
        artifact = await artifacts_api.update("a1", {"title": "New"})

        call = artifacts_api._low_level_client.update_artifact.call_args
        assert call.args[0].resource_id == "a1"
        assert call.kwargs["new_file"] is False
        artifacts_api._upload_client.upload_attachment.assert_not_awaited()
        assert artifact.artifact_version.id_ == "v2"

    @pytest.mark.asyncio
    async def test_new_file(self, artifacts_api, tmp_path):
        path = tmp_path / "plot.png"
        path.write_bytes(b"png")

        await artifacts_api.update(_artifact(), {}, file=path)

        assert artifacts_api._low_level_client.update_artifact.call_args.kwargs["new_file"] is True
        artifacts_api._upload_client.upload_attachment.assert_awaited_once_with(
            source=path, entity_id="v2", entity_type="artifact_versions", file_name=None
        )
        artifacts_api._low_level_client.get_artifact.assert_awaited_once_with(
            artifact_id="a1", artifact_version_id="v2"
        )

    @pytest.mark.asyncio
    async def test_empty_update_rejected(self, artifacts_api):
        with pytest.raises(ValueError, match="at least one field"):
            await artifacts_api.update("a1", {})


class TestGetArchivePromote:
    @pytest.mark.asyncio
    async def test_pinned_get(self, artifacts_api):
        await artifacts_api.get(artifact_id="a1", artifact_version_id="v1")

        artifacts_api._low_level_client.get_artifact.assert_awaited_once_with(
            artifact_id="a1", artifact_version_id="v1"
        )

    @pytest.mark.asyncio
    async def test_archive_refetches(self, artifacts_api):
        artifact = await artifacts_api.archive(_artifact())

        artifacts_api._low_level_client.archive_artifact.assert_awaited_once_with("a1")
        artifacts_api._low_level_client.get_artifact.assert_awaited_once_with(
            artifact_id="a1", artifact_version_id=None
        )
        assert artifact.client is artifacts_api.client

    @pytest.mark.asyncio
    async def test_unarchive_refetches(self, artifacts_api):
        await artifacts_api.unarchive("a1")

        artifacts_api._low_level_client.unarchive_artifact.assert_awaited_once_with("a1")
        artifacts_api._low_level_client.get_artifact.assert_awaited_once()

    @pytest.mark.asyncio
    async def test_promote(self, artifacts_api):
        version = _artifact("v1").artifact_version

        promoted = await artifacts_api.versions.promote(version=version)

        artifacts_api._low_level_client.promote_artifact_version.assert_awaited_once_with(
            artifact_id="a1", artifact_version_id="v1"
        )
        assert promoted.artifact_version.id_ == "v3"
        assert promoted.artifact_version.client is artifacts_api.client


class TestReadInMemory:
    @pytest.mark.asyncio
    async def test_read_bytes(self, artifacts_api):
        with patch(
            "sift_client.resources.artifacts.read_url_bytes", return_value=b"data"
        ) as read_url_bytes:
            data = await artifacts_api.read_bytes(artifact=_artifact("v1", "rf1").artifact_version)

        assert data == b"data"
        assert read_url_bytes.call_args.args == ("https://x",)


class TestDownload:
    @pytest.mark.asyncio
    async def test_artifact_id_resolves_latest_version(self, artifacts_api):
        url = await artifacts_api.get_download_url(artifact="a1")

        assert url == "https://x"
        artifacts_api._remote_files_client.get_remote_file_download_url.assert_awaited_once_with(
            remote_file_id="rf1"
        )

    @pytest.mark.asyncio
    async def test_version_without_file(self, artifacts_api):
        version = _artifact("v1").artifact_version

        with pytest.raises(ValueError, match="has no file"):
            await artifacts_api.get_download_url(artifact=version)

    @pytest.mark.asyncio
    async def test_download_writes_to_output_path(self, artifacts_api, tmp_path):
        with patch("sift_client.resources.artifacts.download_file") as download_file:
            await artifacts_api.download(
                artifact=_artifact("v1", "rf1"), output_path=tmp_path / "out"
            )

        download_file.assert_called_once()
        assert download_file.call_args.args == ("https://x", tmp_path / "out")


@pytest.mark.integration
def test_client_binding(sift_client):
    assert isinstance(sift_client.artifacts, ArtifactsAPI)
    assert isinstance(sift_client.artifacts.versions, ArtifactVersionsAPI)
    assert isinstance(sift_client.async_.artifacts, ArtifactsAPIAsync)


@pytest.mark.integration
def test_publish_find_and_version(sift_client):
    stamp = datetime.now(timezone.utc).strftime("%Y%m%d%H%M%S%f")
    title = f"sift-client-test-artifact-{stamp}"
    artifact = sift_client.artifacts.create(
        {"title": title, "created_via": ArtifactCreatedVia.UPLOAD},
        file=b"first",
        file_name="first.txt",
    )
    try:
        assert artifact.artifact_version.version == 1
        assert artifact.artifact_version.file_name == "first.txt"
        assert [a.id_ for a in sift_client.artifacts.list_(title=title)] == [artifact.id_]

        artifact.update({"summary": "second"}, file=b"second", file_name="second.txt")
        assert artifact.artifact_version.version == 2

        versions = sift_client.artifacts.versions.list_(artifact=artifact)
        assert [v.version for v in versions] == [2, 1]
        assert [v.is_current for v in versions] == [True, False]
        assert isinstance(versions[0], ArtifactVersion)
    finally:
        sift_client.artifacts.archive(artifact)
