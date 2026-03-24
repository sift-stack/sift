"""Tests for UploadLowLevelClient functionality."""

from pathlib import Path
from unittest.mock import patch

import pytest

from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient


class TestUploadLowLevelClient:
    class TestMimeAndContentTypeFromPath:
        def test_known_mime_type(self):
            _, mime, _ = UploadLowLevelClient._mime_and_content_type_from_path(Path("video.mp4"))
            assert mime == "video/mp4"

        def test_unknown_extension_returns_none(self):
            _, mime, _ = UploadLowLevelClient._mime_and_content_type_from_path(Path("data.parquet"))
            assert mime is None

        def test_no_extension_returns_none(self):
            _, mime, _ = UploadLowLevelClient._mime_and_content_type_from_path(Path("README"))
            assert mime is None

        def test_file_name_preserved(self):
            name, _, _ = UploadLowLevelClient._mime_and_content_type_from_path(
                Path("my_file.pcapng")
            )
            assert name == "my_file.pcapng"

    class TestUploadAttachmentMimeFallback:
        @pytest.mark.asyncio
        async def test_unknown_mime_type_falls_back_to_octet_stream(self, tmp_path):
            test_file = tmp_path / "data.parquet"
            test_file.write_bytes(b"fake parquet data")

            client = UploadLowLevelClient.__new__(UploadLowLevelClient)

            with patch.object(
                client, "_upload_file_sync", return_value="remote-file-123"
            ) as mock_upload:
                result = await client.upload_attachment(
                    path=test_file,
                    entity_id="entity_123",
                    entity_type="runs",
                )

            mock_upload.assert_called_once()
            _, _, mimetype, *_ = mock_upload.call_args[0]
            assert mimetype == "application/octet-stream"
            assert result == "remote-file-123"
