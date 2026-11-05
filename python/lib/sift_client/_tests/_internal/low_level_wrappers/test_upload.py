"""Tests for UploadLowLevelClient functionality."""

import hashlib
import io
from pathlib import Path
from typing import BinaryIO, cast
from unittest.mock import MagicMock, patch

import pytest
from requests import HTTPError, Request, Response
from requests_toolbelt import MultipartEncoder

from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient


def _posting_client() -> UploadLowLevelClient:
    client = UploadLowLevelClient.__new__(UploadLowLevelClient)
    client._rest_client = MagicMock()
    client._rest_client.post.return_value.status_code = 200
    client._rest_client.post.return_value.json.return_value = {
        "remoteFile": {"remoteFileId": "rf1"}
    }
    return client


def _upload(
    client: UploadLowLevelClient,
    file: BinaryIO,
    *,
    sha256: bool = False,
    entity_type: str = "artifact_versions",
) -> str:
    return client._upload_file_sync(
        file,
        "x.bin",
        "application/octet-stream",
        None,
        "v1",
        entity_type,
        None,
        None,
        None,
        sha256,
    )


def _posted_form(client: UploadLowLevelClient) -> MultipartEncoder:
    return cast("MagicMock", client._rest_client.post).call_args.kwargs["data"]


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
        client = _posting_client()

        remote_file_id = _upload(client, io.BytesIO(b"a,b\n1,2\n"))

        assert remote_file_id == "rf1"
        client._rest_client.post.assert_called_once()

    def test_upload_error_includes_server_body(self):
        response = Response()
        response.status_code, response.reason = 400, "Bad Request"
        response.url = "https://example.test/api/v0/remote-files/upload"
        response.request = Request("POST", response.url).prepare()
        response._content = b"invalid entity_type"
        client = UploadLowLevelClient.__new__(UploadLowLevelClient)
        client._rest_client = MagicMock()
        client._rest_client.post.return_value = response

        with pytest.raises(HTTPError, match="Server response: invalid entity_type"):
            _upload(client, io.BytesIO(b"x"))


class TestUploadSha256:
    @pytest.mark.asyncio
    async def test_path(self, tmp_path):
        path = tmp_path / "table.csv"
        path.write_bytes(b"a,b\n1,2\n")
        client = _posting_client()

        with patch(
            "sift_client._internal.low_level_wrappers.upload.run_sync_function",
            side_effect=lambda fn, *args: fn(*args),
        ):
            await client.upload_attachment(
                source=path, entity_id="v1", entity_type="artifact_versions", sha256=True
            )

        assert _posted_form(client).fields["sha256"] == hashlib.sha256(b"a,b\n1,2\n").hexdigest()

    @pytest.mark.parametrize(
        "data",
        [b"", b"hello", bytes(range(256)) * 10_000],
        ids=["empty", "small", "multi_chunk"],
    )
    def test_bytes(self, data):
        client = _posting_client()

        _upload(client, io.BytesIO(data), sha256=True)

        assert _posted_form(client).fields["sha256"] == hashlib.sha256(data).hexdigest()

    @pytest.mark.parametrize(
        "open_file",
        [lambda path: open(path, "rb"), lambda path: io.BytesIO(path.read_bytes())],
        ids=["file", "bytes_io"],
    )
    def test_file_object_hashes_from_its_position(self, tmp_path, open_file):
        path = tmp_path / "data.bin"
        path.write_bytes(b"header|payload")
        client = _posting_client()

        with open_file(path) as file:
            file.seek(len(b"header|"))
            _upload(client, file, sha256=True)
            form = _posted_form(client)
            assert form.fields["sha256"] == hashlib.sha256(b"payload").hexdigest()
            body = form.to_string()

        assert b"\r\n\r\npayload\r\n" in body
        assert b"header|" not in body

    def test_off_by_default(self):
        client = _posting_client()

        _upload(client, io.BytesIO(b"x"), entity_type="runs")

        assert "sha256" not in _posted_form(client).fields
