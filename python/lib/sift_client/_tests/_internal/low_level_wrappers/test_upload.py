"""Tests for UploadLowLevelClient functionality."""

import hashlib
import io
from pathlib import Path
from typing import cast
from unittest.mock import MagicMock, patch

import pytest
from requests import HTTPError, Request, Response
from requests_toolbelt import MultipartEncoder

from sift_client._internal.low_level_wrappers.upload import (
    UploadLowLevelClient,
    _sha256_from_position,
)


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
            client._upload_file_sync(
                io.BytesIO(b"x"),
                "t.csv",
                "text/csv",
                None,
                "v1",
                "artifact_versions",
                None,
                None,
                None,
            )


def _posting_client() -> UploadLowLevelClient:
    client = UploadLowLevelClient.__new__(UploadLowLevelClient)
    client._rest_client = MagicMock()
    client._rest_client.post.return_value.status_code = 200
    client._rest_client.post.return_value.json.return_value = {
        "remoteFile": {"remoteFileId": "rf1"}
    }
    return client


def _posted_form(client: UploadLowLevelClient) -> MultipartEncoder:
    return cast("MagicMock", client._rest_client.post).call_args.kwargs["data"]


def _posted_fields(client: UploadLowLevelClient) -> dict:
    return _posted_form(client).fields


def _posted_body(client: UploadLowLevelClient) -> bytes:
    return _posted_form(client).to_string()


class _Unseekable(io.RawIOBase):
    def __init__(self, data: bytes):
        self._data = io.BytesIO(data)

    def readable(self) -> bool:
        return True

    def readinto(self, buffer) -> int:
        chunk = self._data.read(len(buffer))
        buffer[: len(chunk)] = chunk
        return len(chunk)


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

        assert _posted_fields(client)["sha256"] == hashlib.sha256(b"a,b\n1,2\n").hexdigest()

    @pytest.mark.parametrize(
        "data",
        [b"", b"hello", bytes(range(256)) * 10_000],
        ids=["empty", "small", "multi_chunk"],
    )
    def test_bytes(self, data):
        client = _posting_client()

        client._upload_file_sync(
            io.BytesIO(data),
            "x.bin",
            "application/octet-stream",
            None,
            "v1",
            "artifact_versions",
            None,
            None,
            None,
            True,
        )

        digest = _posted_fields(client)["sha256"]
        assert digest == hashlib.sha256(data).hexdigest()
        assert digest == digest.lower()

    @pytest.mark.parametrize("kind", ["file", "bytes_io"])
    def test_file_object_hashes_from_its_position(self, tmp_path, kind):
        path = tmp_path / "data.bin"
        path.write_bytes(b"header|payload")
        client = _posting_client()

        with open(path, "rb") if kind == "file" else io.BytesIO(b"header|payload") as file:
            file.seek(len(b"header|"))
            client._upload_file_sync(
                file,
                "payload.bin",
                "application/octet-stream",
                None,
                "v1",
                "artifact_versions",
                None,
                None,
                None,
                True,
            )
            assert _posted_fields(client)["sha256"] == hashlib.sha256(b"payload").hexdigest()
            body = _posted_body(client)

        assert b"\r\n\r\npayload\r\n" in body
        assert b"header|" not in body

    def test_unseekable_file_object_is_not_hashed(self):
        stream = io.BufferedReader(_Unseekable(b"stream"))

        assert _sha256_from_position(stream) is None
        assert stream.read() == b"stream"

    def test_off_by_default(self):
        client = _posting_client()

        client._upload_file_sync(
            io.BytesIO(b"x"),
            "x.bin",
            "application/octet-stream",
            None,
            "v1",
            "runs",
            None,
            None,
            None,
        )

        assert "sha256" not in _posted_fields(client)
