from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.remote_files import RemoteFilesLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.remote_file import (
    RemoteFile,
    RemoteFileEntityType,
    RemoteFileUpdate,
)
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient


class RemoteFilesAPIAsync(ResourceBase):
    """High-level API for interacting with remote files.

    This class provides a Pythonic, notebook-friendly interface for interacting with the RemoteFilesAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the RemoteFile class from the low-level wrapper, which is a user-friendly
    representation of a remote file using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the RemoteFilesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = RemoteFilesLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(
        self,
        *,
        remote_file_id: str,
    ) -> RemoteFile:
        """Get a RemoteFile.

        Args:
            remote_file_id: The ID of the remote file.

        Returns:
            The RemoteFile.
        """
        remote_file = await self._low_level_client.get_remote_file(remote_file_id=remote_file_id)
        return self._apply_client_to_instance(remote_file)

    async def list_(
        self,
        *,
        remote_file_id: str | None = None,
        remote_file_ids: list[str] | None = None,
        entity_id: str | None = None,
        entity_ids: list[str] | None = None,
        entity_type: RemoteFileEntityType | None = None,
        entity_types: list[RemoteFileEntityType] | None = None,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
    ) -> list[RemoteFile]:
        """List RemoteFiles.

        Args:
            remote_file_id: The ID of the remote file.
            remote_file_ids: List of remote file IDs.
            entity_id: The entity ID.
            entity_ids: List of entity IDs.
            entity_type: The entity type.
            entity_types: List of entity types.
            name: The name of the file.
            names: List of file names.
            name_contains: String that the name should contain.
            name_regex: Regex pattern for the name.

        Returns:
            A list of RemoteFiles matching the filters.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
        ]
        if remote_file_id:
            filter_parts.append(cel.equals("remote_file_id", remote_file_id))
        if remote_file_ids:
            filter_parts.append(cel.in_("remote_file_id", remote_file_ids))
        if entity_id:
            filter_parts.append(cel.equals("entity_id", entity_id))
        if entity_ids:
            filter_parts.append(cel.in_("entity_id", entity_ids))
        if entity_type:
            filter_parts.append(cel.equals("entity_type", entity_type.name.lower()))
        if entity_types:
            filter_parts.append(cel.in_("entity_type", [et.name.lower() for et in entity_types]))
        query_filter = cel.and_(*filter_parts) if filter_parts else None
        remote_files = await self._low_level_client.list_all_remote_files(query_filter=query_filter)
        return [self._apply_client_to_instance(rf) for rf in remote_files]

    async def update(
        self, remote_file: str | RemoteFile, update: RemoteFileUpdate | dict
    ) -> RemoteFile:
        """Update a remote file.

        Args:
            remote_file: The RemoteFile or remote file ID to update.
            update: Updates to apply to the RemoteFile.

        Returns:
            The updated RemoteFile.

        """
        remote_file_id = (
            remote_file._id_or_error if isinstance(remote_file, RemoteFile) else remote_file
        )
        if isinstance(update, dict):
            update = RemoteFileUpdate.model_validate(update)
        update._resource_id = remote_file_id
        updated_remote_file = await self._low_level_client.update_remote_file(update=update)
        return self._apply_client_to_instance(updated_remote_file)

    async def delete(self, remote_file: str | RemoteFile) -> None:
        """Delete a RemoteFile.

        Args:
            remote_file: The RemoteFile or remote file ID to delete.
        """
        remote_file_id = (
            remote_file._id_or_error if isinstance(remote_file, RemoteFile) else remote_file
        )
        await self._low_level_client.delete_remote_file(remote_file_id=remote_file_id)

    async def batch_delete(self, remote_files: list[str | RemoteFile]) -> None:
        """Batch delete RemoteFiles.

        Args:
            remote_files: The RemoteFiles or remote file IDs to delete.
        """
        remote_file_ids = [
            remote_file._id_or_error if isinstance(remote_file, RemoteFile) else remote_file
            for remote_file in remote_files
        ]
        await self._low_level_client.batch_delete_remote_files(remote_file_ids=remote_file_ids)

    async def get_download_url(self, remote_file: str | RemoteFile) -> str:
        """Get a download URL for a RemoteFile.

        Args:
            remote_file: The RemoteFile or remote file ID to get the download URL for.
        """
        remote_file_id = (
            remote_file._id_or_error if isinstance(remote_file, RemoteFile) else remote_file
        )
        return await self._low_level_client.get_remote_file_download_url(
            remote_file_id=remote_file_id
        )
