from __future__ import annotations

import asyncio
import logging
from pathlib import Path
from typing import TYPE_CHECKING, Any

from requests_toolbelt import MultipartEncoder

from sift_client._internal.low_level_wrappers.base import LowLevelClientBase
from sift_client.transport import WithRestClient

if TYPE_CHECKING:
    from sift_client.transport.rest_transport import RestClient

# Configure logging
logger = logging.getLogger(__name__)


class UploadLowLevelClient(LowLevelClientBase, WithRestClient):
    """Low-level client for file upload operations.

    This class provides a thin wrapper for uploading file attachments via REST API.

    Example:
        ```python
        from sift_client.client import SiftClient
        from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient

        # Initialize the REST client
        sift_client = SiftClient(rest_url="https://your-api.siftstack.com", grpc_url="https://your-grpc-api.siftstack.com", api_key="your-api-key")

        # Create the upload client
        upload_client = UploadLowLevelClient(sift_client.rest_client)

        # Upload a file
        remote_file_id = await upload_client.upload_attachment(
            path="path/to/file.mp4",
            entity_id="run_12345",
            entity_type="runs",
            description="Video of test run",
        )
        ```
    """

    UPLOAD_PATH = "/api/v0/remote-files/upload"
    UPLOAD_BULK_PATH = "/api/v0/remote-files/upload:bulk"

    def __init__(self, rest_client: RestClient):
        """Initialize the UploadLowLevelClient.

        Args:
            rest_client: The REST client to use for making API calls.
        """
        super().__init__(rest_client)

    async def upload_attachment(
        self,
        path: str | Path,
        entity_id: str,
        entity_type: str,
        metadata: dict[str, Any] | None = None,
        description: str | None = None,
        organization_id: str | None = None,
    ) -> str:
        """Upload a file attachment to an entity.

        Args:
            path: Path to the file to upload.
            entity_id: The ID of the entity to attach the file to.
            entity_type: The type of entity (e.g., "runs", "annotations", "annotation_logs").
            metadata: Optional metadata for the file (e.g., video/image metadata).
            description: Optional description of the file.
            organization_id: Optional organization ID.

        Returns:
            The remote file ID of the uploaded file.

        Raises:
            ValueError: If the path doesn't point to a regular file or MIME type cannot be determined.
            Exception: If the upload fails.
        """
        posix_path = Path(path) if isinstance(path, str) else path

        if not posix_path.is_file():
            raise ValueError(f"Provided path, '{path}', does not point to a regular file.")

        file_name, mimetype, content_encoding = self._mime_and_content_type_from_path(posix_path)

        if not mimetype:
            raise ValueError(f"The MIME-type of '{posix_path}' could not be computed.")

        # Run the synchronous file upload in a thread pool to avoid blocking the event loop
        loop = asyncio.get_event_loop()
        return await loop.run_in_executor(
            None,
            self._upload_file_sync,
            posix_path,
            file_name,
            mimetype,
            content_encoding,
            entity_id,
            entity_type,
            metadata,
            description,
            organization_id,
        )

    def _upload_file_sync(
        self,
        path: Path,
        file_name: str,
        mimetype: str,
        content_encoding: str | None,
        entity_id: str,
        entity_type: str,
        metadata: dict[str, Any] | None,
        description: str | None,
        organization_id: str | None,
    ) -> str:
        """Synchronous helper to upload the file.

        This is called from a thread pool to avoid blocking the async event loop.
        """
        with open(path, "rb") as file:
            form_fields: dict[str, Any] = {
                "entityId": entity_id,
                "entityType": entity_type,
            }

            if content_encoding:
                form_fields["file"] = (
                    file_name,
                    file,
                    mimetype,
                    {
                        "Content-Encoding": content_encoding,
                    },
                )
            else:
                form_fields["file"] = (file_name, file, mimetype)

            if metadata:
                import json

                form_fields["metadata"] = json.dumps(
                    metadata, default=lambda x: x.as_json() if hasattr(x, "as_json") else x
                )

            if organization_id:
                form_fields["organizationId"] = organization_id

            if description:
                form_fields["description"] = description

            form_data = MultipartEncoder(fields=form_fields)

            # Use the RestClient to make the POST request
            response = self._rest_client.post(
                endpoint=self.UPLOAD_PATH,
                data=form_data,  # type: ignore
                headers={
                    "Content-Type": form_data.content_type,
                },
            )

            if response.status_code != 200:
                raise Exception(
                    f"Request failed with status code {response.status_code} ({response.reason})."
                )

            response_data = response.json()
            return response_data.get("remoteFile", {}).get("remoteFileId")

    @staticmethod
    def _mime_and_content_type_from_path(path: Path) -> tuple[str, str | None, str | None]:
        """Determine the MIME type and content encoding from a file path.

        Args:
            path: The file path to analyze.

        Returns:
            A tuple of (file_name, mime_type, content_encoding).
        """
        import mimetypes

        file_name = path.name
        mime, encoding = mimetypes.guess_type(path)
        return file_name, mime, encoding
