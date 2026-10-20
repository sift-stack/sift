"""Tests for UploadLowLevelClient functionality."""

import io
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient


class TestUploadAttachment:
    @pytest.mark.asyncio
    async def test_known_mime_type(self, tmp_path):
        test_file = tmp_path / "video.mp4"
        test_file.write_bytes(b"fake data")

        client = UploadLowLevelClient.__new__(UploadLowLevelClient)

        with patch.object(client, "_upload_file_sync", return_value="remote-file-123") as mock:
            await client.upload_attachment(source=test_file, entity_id="e1", entity_type="runs")

        _, _, mimetype, *_ = mock.call_args[0]
        assert mimetype == "video/mp4"

    @pytest.mark.asyncio
    async def test_unknown_extension_falls_back_to_application_x_ext(self, tmp_path):
        test_file = tmp_path / "data.pcapng"
        test_file.write_bytes(b"fake data")

        client = UploadLowLevelClient.__new__(UploadLowLevelClient)

        with patch.object(client, "_upload_file_sync", return_value="remote-file-123") as mock:
            await client.upload_attachment(source=test_file, entity_id="e1", entity_type="runs")

        _, _, mimetype, *_ = mock.call_args[0]
        assert mimetype == "application/x-pcapng"

    @pytest.mark.asyncio
    async def test_no_extension_falls_back_to_octet_stream(self, tmp_path):
        test_file = tmp_path / "README"
        test_file.write_bytes(b"fake data")

        client = UploadLowLevelClient.__new__(UploadLowLevelClient)

        with patch.object(client, "_upload_file_sync", return_value="remote-file-123") as mock:
            await client.upload_attachment(source=test_file, entity_id="e1", entity_type="runs")

        _, _, mimetype, *_ = mock.call_args[0]
        assert mimetype == "application/octet-stream"

    @pytest.mark.asyncio
    async def test_file_name_preserved(self, tmp_path):
        test_file = tmp_path / "my_file.pcapng"
        test_file.write_bytes(b"fake data")

        client = UploadLowLevelClient.__new__(UploadLowLevelClient)

        with patch.object(client, "_upload_file_sync", return_value="remote-file-123") as mock:
            await client.upload_attachment(source=test_file, entity_id="e1", entity_type="runs")

        file_name, *_ = mock.call_args[0]
        assert file_name == Path(test_file)


class TestUploadFileObject:
    @pytest.mark.asyncio
    async def test_uses_file_name_for_mime_type(self):
        client = UploadLowLevelClient.__new__(UploadLowLevelClient)
        data = io.BytesIO(b"fake data")

        with patch.object(client, "_upload_file_sync", return_value="remote-file-123") as mock:
            remote_file_id = await client.upload_attachment(
                source=data,
                file_name="peaks.parquet",
                entity_id="v1",
                entity_type="artifact_versions",
            )

        source, file_name, mimetype, *_ = mock.call_args[0]
        assert remote_file_id == "remote-file-123"
        assert source is data
        assert file_name == "peaks.parquet"
        assert mimetype == "application/vnd.apache.parquet"

    @pytest.mark.asyncio
    async def test_file_object_needs_file_name(self):
        client = UploadLowLevelClient.__new__(UploadLowLevelClient)

        with pytest.raises(ValueError, match="file_name is required"):
            await client.upload_attachment(
                source=io.BytesIO(b"x"), entity_id="v1", entity_type="artifact_versions"
            )

    def test_posts_file_object_without_opening_a_path(self):
        client = UploadLowLevelClient.__new__(UploadLowLevelClient)
        client._rest_client = MagicMock()
        client._rest_client.post.return_value.status_code = 200
        client._rest_client.post.return_value.json.return_value = {
            "remoteFile": {"remoteFileId": "rf1"}
        }

        remote_file_id = client._upload_file_sync(
            io.BytesIO(b"a,b\n1,2\n"),
            "t.csv",
            "text/csv",
            None,
            "v1",
            "artifact_versions",
            None,
            None,
            None,
        )

        assert remote_file_id == "rf1"
        client._rest_client.post.assert_called_once()
