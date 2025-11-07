from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.remote_files import RemoteFilesLowLevelClient
from sift_client.resources._base import ResourceBase

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.file_attachment import FileAttachment


class FileAttachmentsAPIAsync(ResourceBase):
    """High-level API for interacting with file attachments (remote files).

    This class provides a Pythonic, notebook-friendly interface for interacting with the AssetsAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Asset class from the low-level wrapper, which is a user-friendly
    representation of an asset using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the AssetsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = RemoteFilesLowLevelClient(grpc_client=self.client.grpc_client)

    def get(
        self, *, file_id: str | None = None, client_key: str | None = None
    ) -> FileAttachment: ...

    def list_(self) -> list[FileAttachment]: ...

    def find(self) -> FileAttachment: ...

    def update(self) -> FileAttachment | list[FileAttachment]: ...

    def delete(self) -> None: ...

    def download(self, output_path: str | Path) -> None: ...
