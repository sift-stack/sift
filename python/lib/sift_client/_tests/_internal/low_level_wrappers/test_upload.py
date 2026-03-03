"""Tests for UploadLowLevelClient functionality."""

from pathlib import Path

from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient


class TestUploadLowLevelClient:
    class TestMimeAndContentTypeFromPath:
        def test_parquet_file_extension(self):
            _, mime, _ = UploadLowLevelClient._mime_and_content_type_from_path(Path("data.parquet"))
            assert mime == "application/vnd.apache.parquet"

        def test_pqt_file_extension(self):
            _, mime, _ = UploadLowLevelClient._mime_and_content_type_from_path(Path("data.pqt"))
            assert mime == "application/vnd.apache.parquet"
