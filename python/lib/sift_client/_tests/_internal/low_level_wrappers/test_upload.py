"""Tests for UploadLowLevelClient functionality."""

from pathlib import Path
from unittest.mock import patch

import pytest

from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient


class TestUploadAttachment:
    @pytest.mark.asyncio
    async def test_known_mime_type(self, tmp_path):
        test_file = tmp_path / "video.mp4"
        test_file.write_bytes(b"fake data")

        client = UploadLowLevelClient.__new__(UploadLowLevelClient)

        with patch.object(client, "_upload_file_sync", return_value="remote-file-123") as mock:
            await client.upload_attachment(path=test_file, entity_id="e1", entity_type="runs")

        _, _, mimetype, *_ = mock.call_args[0]
        assert mimetype == "video/mp4"

    @pytest.mark.asyncio
    async def test_unknown_extension_falls_back_to_application_x_ext(self, tmp_path):
        test_file = tmp_path / "data.pcapng"
        test_file.write_bytes(b"fake data")

        client = UploadLowLevelClient.__new__(UploadLowLevelClient)

        with patch.object(client, "_upload_file_sync", return_value="remote-file-123") as mock:
            await client.upload_attachment(path=test_file, entity_id="e1", entity_type="runs")

        _, _, mimetype, *_ = mock.call_args[0]
        assert mimetype == "application/x-pcapng"

    @pytest.mark.asyncio
    async def test_no_extension_raises_value_error(self, tmp_path):
        test_file = tmp_path / "README"
        test_file.write_bytes(b"fake data")

        client = UploadLowLevelClient.__new__(UploadLowLevelClient)

        with pytest.raises(ValueError, match="file has no extension"):
            await client.upload_attachment(path=test_file, entity_id="e1", entity_type="runs")

    @pytest.mark.asyncio
    async def test_file_name_preserved(self, tmp_path):
        test_file = tmp_path / "my_file.pcapng"
        test_file.write_bytes(b"fake data")

        client = UploadLowLevelClient.__new__(UploadLowLevelClient)

        with patch.object(client, "_upload_file_sync", return_value="remote-file-123") as mock:
            await client.upload_attachment(path=test_file, entity_id="e1", entity_type="runs")

        file_name, *_ = mock.call_args[0]
        assert file_name == Path(test_file)
