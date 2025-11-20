"""Pytest tests for the File Attachments API.

These tests demonstrate and validate the usage of the File Attachments API including:
- Basic file attachment operations (get, list, upload, download)
- File attachment filtering by entity
- File attachment updates and deletion
- Error handling and edge cases
"""

import os
import tempfile
from pathlib import Path

import pytest
import pytest_asyncio

from sift_client import SiftClient
from sift_client.resources import FileAttachmentsAPI, FileAttachmentsAPIAsync
from sift_client.sift_types.file_attachment import (
    FileAttachment,
    FileAttachmentUpdate,
    RemoteFileEntityType,
)

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    """Test that file_attachments API is properly bound to the client."""
    assert sift_client.file_attachments
    assert isinstance(sift_client.file_attachments, FileAttachmentsAPI)
    assert sift_client.async_.file_attachments
    assert isinstance(sift_client.async_.file_attachments, FileAttachmentsAPIAsync)


@pytest.fixture
def file_attachments_api_async(sift_client: SiftClient):
    """Get the async file attachments API instance."""
    return sift_client.async_.file_attachments


@pytest.fixture
def file_attachments_api_sync(sift_client: SiftClient):
    """Get the synchronous file attachments API instance."""
    return sift_client.file_attachments


@pytest.fixture
def test_run(sift_client: SiftClient):
    """Get a test run to attach files to."""
    runs = sift_client.runs.list_(limit=1)
    if runs:
        return runs[0]
    pytest.fail("No runs available for testing, please add test test runs")


@pytest.fixture
def test_asset(sift_client: SiftClient):
    """Get a test asset to attach files to."""
    assets = sift_client.assets.list_(limit=1)
    if assets:
        return assets[0]
    pytest.fail("No assets available for testing, please add test test assets")


@pytest_asyncio.fixture
async def uploaded_file_attachment(file_attachments_api_async, test_run):
    """Upload a test file and return the file attachment, cleaning up after test."""
    # Create a temporary test file
    with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
        tmp.write("Test file content for integration tests\n")
        tmp.write("This file should be deleted after the test\n")
        tmp_path = tmp.name

    try:
        # Upload the file
        file_attachment = await file_attachments_api_async.upload(
            path=tmp_path,
            entity=test_run,
            description="Integration test file attachment",
        )
        yield file_attachment

        # Cleanup: delete the uploaded file
        try:
            await file_attachments_api_async.delete(file_attachments=file_attachment)
        except Exception:
            pass  # If deletion fails, it's okay for test cleanup
    finally:
        # Cleanup: delete the temporary local file
        if os.path.exists(tmp_path):
            os.unlink(tmp_path)


class TestFileAttachmentsAPIAsync:
    """Test suite for the async File Attachments API functionality."""

    class TestUpload:
        """Tests for the async upload method."""

        @pytest.mark.asyncio
        async def test_upload_to_run(self, file_attachments_api_async, test_run):
            """Test uploading a file attachment to a run."""
            # Create a temporary test file
            with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
                tmp.write("Test file content\n")
                tmp_path = tmp.name

            try:
                # Upload the file
                file_attachment = await file_attachments_api_async.upload(
                    path=tmp_path,
                    entity=test_run,
                    description="Test upload to run",
                )

                # Verify the upload
                assert isinstance(file_attachment, FileAttachment)
                assert file_attachment.id_ is not None
                assert file_attachment.file_name is not None
                assert file_attachment.description == "Test upload to run"
                assert file_attachment.entity_id == test_run.id_

                # Cleanup: delete the uploaded file
                await file_attachments_api_async.delete(file_attachments=file_attachment)
            finally:
                # Cleanup: delete the temporary local file
                if os.path.exists(tmp_path):
                    os.unlink(tmp_path)

        @pytest.mark.asyncio
        async def test_upload_to_asset(self, file_attachments_api_async, test_asset):
            """Test uploading a file attachment to an asset."""
            # Create a temporary test file
            with tempfile.NamedTemporaryFile(mode="w", suffix=".csv", delete=False) as tmp:
                tmp.write("col1,col2,col3\n")
                tmp.write("1,2,3\n")
                tmp_path = tmp.name

            try:
                # Upload the file
                file_attachment = await file_attachments_api_async.upload(
                    path=tmp_path,
                    entity=test_asset,
                    description="Test CSV upload to asset",
                    metadata={"test_key": "test_value"},
                )

                # Verify the upload
                assert isinstance(file_attachment, FileAttachment)
                assert file_attachment.id_ is not None
                assert file_attachment.entity_id == test_asset.id_
                assert file_attachment.description == "Test CSV upload to asset"

                # Cleanup
                await file_attachments_api_async.delete(file_attachments=file_attachment)
            finally:
                if os.path.exists(tmp_path):
                    os.unlink(tmp_path)

        @pytest.mark.asyncio
        async def test_upload_with_pathlib(self, file_attachments_api_async, test_run):
            """Test uploading using pathlib.Path instead of string."""
            # Create a temporary test file
            with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
                tmp.write("Test pathlib upload\n")
                tmp_path = Path(tmp.name)

            try:
                # Upload using Path object
                file_attachment = await file_attachments_api_async.upload(
                    path=tmp_path,
                    entity=test_run,
                    description="Test pathlib upload",
                )

                assert isinstance(file_attachment, FileAttachment)
                assert file_attachment.id_ is not None

                # Cleanup
                await file_attachments_api_async.delete(file_attachments=file_attachment)
            finally:
                if tmp_path.exists():
                    tmp_path.unlink()

    class TestGet:
        """Tests for the async get method."""

        @pytest.mark.asyncio
        async def test_get_by_id(self, file_attachments_api_async, uploaded_file_attachment):
            """Test getting a file attachment by ID."""
            retrieved = await file_attachments_api_async.get(
                file_attachment_id=uploaded_file_attachment.id_
            )

            assert retrieved is not None
            assert retrieved.id_ == uploaded_file_attachment.id_
            assert retrieved.file_name == uploaded_file_attachment.file_name

        @pytest.mark.asyncio
        async def test_get_nonexistent_raises_error(self, file_attachments_api_async):
            """Test that getting a non-existent file attachment raises an error."""
            # Should raise an error for non-existent file attachment
            with pytest.raises(Exception, match="invalid input syntax for type uuid"):
                await file_attachments_api_async.get(file_attachment_id="nonexistent-file-id-12345")

    class TestList:
        """Tests for the async list_ method."""

        @pytest.mark.asyncio
        async def test_basic_list(self, file_attachments_api_async):
            """Test basic file attachment listing functionality."""
            file_attachments = await file_attachments_api_async.list_(limit=5)

            # Verify we get a list
            assert isinstance(file_attachments, list)

            # If we have file attachments, verify their structure
            if file_attachments:
                fa = file_attachments[0]
                assert isinstance(fa, FileAttachment)
                assert fa.id_ is not None

        @pytest.mark.asyncio
        async def test_list_by_entity(
            self, file_attachments_api_async, uploaded_file_attachment, test_run
        ):
            """Test listing file attachments filtered by entity."""
            file_attachments = await file_attachments_api_async.list_(
                entities=[test_run],
                limit=100,
            )

            assert isinstance(file_attachments, list)

            # Should find our uploaded file
            found = any(fa.id_ == uploaded_file_attachment.id_ for fa in file_attachments)
            assert found, "Uploaded file attachment not found in entity list"

            # All returned attachments should belong to the test run
            for fa in file_attachments:
                assert fa.entity_id == test_run.id_

        @pytest.mark.asyncio
        async def test_list_by_entity_id(
            self, file_attachments_api_async, uploaded_file_attachment, test_run
        ):
            """Test listing file attachments filtered by entity_id."""
            file_attachments = await file_attachments_api_async.list_(
                entity_ids=[test_run.id_],
                limit=100,
            )

            assert isinstance(file_attachments, list)

            # Should find our uploaded file
            found = any(fa.id_ == uploaded_file_attachment.id_ for fa in file_attachments)
            assert found, "Uploaded file attachment not found in entity list"

        @pytest.mark.asyncio
        async def test_list_by_entity_type(
            self, file_attachments_api_async, uploaded_file_attachment, test_run
        ):
            """Test listing file attachments filtered by entity_type."""
            # Test filtering by RUNS entity type
            file_attachments = await file_attachments_api_async.list_(
                entity_type=RemoteFileEntityType.RUNS,
                limit=100,
            )
            assert isinstance(file_attachments, list)
            # All returned attachments should be for RUNS
            for fa in file_attachments:
                assert fa.entity_type == RemoteFileEntityType.RUNS

            # Test filtering by ASSETS entity type
            file_attachments = await file_attachments_api_async.list_(
                entity_type=RemoteFileEntityType.ASSETS,
                limit=100,
            )
            assert isinstance(file_attachments, list)
            # All returned attachments should be for ASSETS
            for fa in file_attachments:
                assert fa.entity_type == RemoteFileEntityType.ASSETS

            # Test filtering by TEST_REPORTS entity type
            file_attachments = await file_attachments_api_async.list_(
                entity_type=RemoteFileEntityType.TEST_REPORTS,
                limit=100,
            )
            assert isinstance(file_attachments, list)
            # All returned attachments should be for TEST_REPORTS
            for fa in file_attachments:
                assert fa.entity_type == RemoteFileEntityType.TEST_REPORTS

        @pytest.mark.asyncio
        async def test_list_by_file_name(
            self, file_attachments_api_async, uploaded_file_attachment
        ):
            """Test listing file attachments filtered by file name."""
            file_attachments = await file_attachments_api_async.list_(
                names=[uploaded_file_attachment.file_name],
            )

            assert isinstance(file_attachments, list)

            # Should find at least our uploaded file
            found = any(fa.id_ == uploaded_file_attachment.id_ for fa in file_attachments)
            assert found

            # All returned attachments should have the specified file name
            for fa in file_attachments:
                assert fa.file_name == uploaded_file_attachment.file_name

        @pytest.mark.asyncio
        async def test_list_with_limit(self, file_attachments_api_async):
            """Test file attachment listing with different limits."""
            # Test with limit of 1
            fas_1 = await file_attachments_api_async.list_(limit=1)
            assert isinstance(fas_1, list)
            assert len(fas_1) <= 1

            # Test with limit of 3
            fas_3 = await file_attachments_api_async.list_(limit=3)
            assert isinstance(fas_3, list)
            assert len(fas_3) <= 3

    class TestUpdate:
        """Tests for the async update method."""

        @pytest.mark.asyncio
        async def test_update_description(
            self, file_attachments_api_async, uploaded_file_attachment
        ):
            """Test updating a file attachment's description."""
            new_description = "Updated description for integration test"

            update = FileAttachmentUpdate(description=new_description)
            update.resource_id = uploaded_file_attachment.id_

            updated = await file_attachments_api_async.update(file_attachment=update)

            assert updated.id_ == uploaded_file_attachment.id_
            assert updated.description == new_description

        @pytest.mark.asyncio
        async def test_update_with_dict(self, file_attachments_api_async, uploaded_file_attachment):
            """Test updating a file attachment using a dict."""
            new_description = "Updated via dict"

            # When using dict, the ID must be set via resource_id after creating the update object
            update_dict = {"description": new_description}
            update = FileAttachmentUpdate.model_validate(update_dict)
            update.resource_id = uploaded_file_attachment.id_

            updated = await file_attachments_api_async.update(file_attachment=update)

            assert updated.id_ == uploaded_file_attachment.id_
            assert updated.description == new_description

    class TestDelete:
        """Tests for the async delete method."""

        @pytest.mark.asyncio
        async def test_delete_single_by_id(self, file_attachments_api_async, test_run):
            """Test deleting a single file attachment by ID string."""
            # Upload a file to delete
            with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
                tmp.write("File to delete\n")
                tmp_path = tmp.name

            try:
                file_attachment = await file_attachments_api_async.upload(
                    path=tmp_path,
                    entity=test_run,
                    description="File to delete",
                )

                # Delete by ID string
                await file_attachments_api_async.delete(file_attachments=file_attachment.id_)

                # Verify it's deleted by attempting to get it (should raise error)
                with pytest.raises(Exception, match="An error occurred"):
                    await file_attachments_api_async.get(file_attachment_id=file_attachment.id_)
            finally:
                if os.path.exists(tmp_path):
                    os.unlink(tmp_path)

        @pytest.mark.asyncio
        async def test_delete_single_by_object(self, file_attachments_api_async, test_run):
            """Test deleting a single file attachment by FileAttachment object."""
            # Upload a file to delete
            with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
                tmp.write("File to delete\n")
                tmp_path = tmp.name

            try:
                file_attachment = await file_attachments_api_async.upload(
                    path=tmp_path,
                    entity=test_run,
                    description="File to delete by object",
                )

                # Delete by FileAttachment object
                await file_attachments_api_async.delete(file_attachments=file_attachment)

                # Verify it's deleted by attempting to get it (should raise error)
                with pytest.raises(Exception, match="An error occurred"):
                    await file_attachments_api_async.get(file_attachment_id=file_attachment.id_)
            finally:
                if os.path.exists(tmp_path):
                    os.unlink(tmp_path)

        @pytest.mark.asyncio
        async def test_delete_multiple(self, file_attachments_api_async, test_run):
            """Test deleting multiple file attachments at once."""
            # Upload multiple files
            file_attachments = []
            tmp_paths = []

            try:
                for i in range(3):
                    with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
                        tmp.write(f"File {i} to delete\n")
                        tmp_paths.append(tmp.name)

                    fa = await file_attachments_api_async.upload(
                        path=tmp_paths[-1],
                        entity=test_run,
                        description=f"File {i} to delete",
                    )
                    file_attachments.append(fa)

                # Delete all at once
                await file_attachments_api_async.delete(file_attachments=file_attachments)

                # Verify they're all deleted
                for fa in file_attachments:
                    with pytest.raises(Exception, match="An error occurred"):
                        await file_attachments_api_async.get(file_attachment_id=fa.id_)
            finally:
                for tmp_path in tmp_paths:
                    if os.path.exists(tmp_path):
                        os.unlink(tmp_path)

        @pytest.mark.asyncio
        async def test_delete_list_of_ids(self, file_attachments_api_async, test_run):
            """Test deleting multiple file attachments using a list of ID strings."""
            # Upload multiple files
            file_attachments = []
            tmp_paths = []

            try:
                for i in range(2):
                    with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
                        tmp.write(f"File {i} to delete\n")
                        tmp_paths.append(tmp.name)

                    fa = await file_attachments_api_async.upload(
                        path=tmp_paths[-1],
                        entity=test_run,
                        description=f"File {i} to delete by ID",
                    )
                    file_attachments.append(fa)

                # Delete using list of IDs
                ids = [fa.id_ for fa in file_attachments]
                await file_attachments_api_async.delete(file_attachments=ids)

                # Verify they're all deleted
                for fa_id in ids:
                    with pytest.raises(Exception, match="An error occurred"):
                        await file_attachments_api_async.get(file_attachment_id=fa_id)
            finally:
                for tmp_path in tmp_paths:
                    if os.path.exists(tmp_path):
                        os.unlink(tmp_path)

    class TestDownload:
        """Tests for the async download methods."""

        @pytest.mark.asyncio
        async def test_get_download_url(self, file_attachments_api_async, uploaded_file_attachment):
            """Test getting a download URL for a file attachment."""
            url = await file_attachments_api_async.get_download_url(
                file_attachment=uploaded_file_attachment
            )

            assert isinstance(url, str)
            assert len(url) > 0
            # URL should be a valid HTTP/HTTPS URL
            assert url.startswith("http://") or url.startswith("https://")

        @pytest.mark.asyncio
        async def test_get_download_url_by_id(
            self, file_attachments_api_async, uploaded_file_attachment
        ):
            """Test getting a download URL using file attachment ID."""
            url = await file_attachments_api_async.get_download_url(
                file_attachment=uploaded_file_attachment.id_
            )

            assert isinstance(url, str)
            assert len(url) > 0


class TestFileAttachmentsAPISync:
    """Test suite for the synchronous File Attachments API functionality.

    Only includes a single test for basic sync generation. No specific sync behavior difference tests are needed.
    """

    class TestList:
        """Tests for the sync list_ method."""

        def test_basic_list(self, file_attachments_api_sync):
            """Test basic synchronous file attachment listing functionality."""
            file_attachments = file_attachments_api_sync.list_(limit=5)

            # Verify we get a list
            assert isinstance(file_attachments, list)

            # If we have file attachments, verify their structure
            if file_attachments:
                assert isinstance(file_attachments[0], FileAttachment)

    class TestUpload:
        """Tests for the sync upload method."""

        def test_upload_download_and_delete(self, file_attachments_api_sync, test_run):
            """Test synchronous upload, download, and cleanup."""
            # Create a temporary test file
            completed = False
            with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
                tmp.write("Sync test file\n")
                tmp_path = tmp.name

            # Create a temporary download path
            with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
                download_path = tmp.name

            try:
                # Upload using sync API
                file_attachment = file_attachments_api_sync.upload(
                    path=tmp_path,
                    entity=test_run,
                    description="Sync upload test",
                )

                # Verify the upload
                assert isinstance(file_attachment, FileAttachment)
                assert file_attachment.id_ is not None

                # Download the file
                file_attachments_api_sync.download(
                    file_attachment=file_attachment, output_path=download_path
                )

                # Verify the downloaded content matches the original
                with open(download_path) as f:
                    downloaded_content = f.read()

                assert downloaded_content == "Sync test file\n"

                # Cleanup
                file_attachments_api_sync.delete(file_attachments=file_attachment)
                completed = True
            finally:
                if os.path.exists(tmp_path):
                    os.unlink(tmp_path)
                if os.path.exists(download_path):
                    os.unlink(download_path)
                assert completed
