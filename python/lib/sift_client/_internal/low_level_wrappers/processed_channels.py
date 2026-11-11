from __future__ import annotations

from typing import TYPE_CHECKING, Any
from urllib.parse import quote

from google.protobuf import json_format

from sift_client._internal.low_level_wrappers.base import DEFAULT_PAGE_SIZE, LowLevelClientBase
from sift_client._internal.util.executor import run_sync_function
from sift_client._internal.util.http import raise_for_status
from sift_client.sift_types.processed_channel import (
    ProcessedChannelRequestError,
    ProcessedChannelWrite,
)
from sift_client.transport import WithRestClient
from sift_client.util.metadata import metadata_dict_to_proto

if TYPE_CHECKING:
    from requests import Response
    from sift.data_imports.v2.data_imports_pb2 import ParquetFlatDatasetConfig

    from sift_client.sift_types.processed_channel import (
        ProcessedChannelCode,
        ProcessedChannelInput,
    )
    from sift_client.transport.rest_transport import RestClient


def _raise_for_status(response: Response) -> None:
    """Raise ``ProcessedChannelRequestError`` for a refusal that carries a gRPC status.

    Raises:
        ProcessedChannelRequestError: If the server returned a status with a message.
        requests.HTTPError: For any other 4xx or 5xx response.
    """
    if response.ok:
        return
    try:
        body = response.json()
    except ValueError:
        body = None
    if isinstance(body, dict) and isinstance(body.get("message"), str):
        code = body.get("code")
        raise ProcessedChannelRequestError(
            response,
            code=code if isinstance(code, int) else None,
            server_message=body["message"],
        )
    raise_for_status(response)


class ProcessedChannelsLowLevelClient(LowLevelClientBase, WithRestClient):
    """Low-level client for the ProcessedChannelService.

    The service's protos are unstable, so they are not published with the client. This client
    calls the service's REST endpoints and maps their JSON to Python objects.
    """

    CREATE_PATH = "/api/v1/processed-channels"
    WRITES_PATH = "/api/v1/processed-channel-writes"

    def __init__(self, rest_client: RestClient):
        WithRestClient.__init__(self, rest_client=rest_client)

    async def create(
        self,
        *,
        asset_id: str,
        run_id: str,
        parquet: ParquetFlatDatasetConfig,
        footer_offset: int,
        footer_length: int,
        code: ProcessedChannelCode,
        inputs: list[ProcessedChannelInput] | None = None,
        metadata: dict[str, str | float | bool] | None = None,
    ) -> tuple[str, str, list[ProcessedChannelWrite]]:
        """Create the pending writes for one upload.

        Returns:
            A tuple of (upload_url, data_import_id, writes), with one pending write per data
            column, in column order.
        """
        body: dict[str, Any] = {
            "assetId": asset_id,
            "runId": run_id,
            "parquet": json_format.MessageToDict(parquet),
            "footerOffset": str(footer_offset),
            "footerLength": footer_length,
            "code": code._to_rest(),
        }
        if inputs:
            body["inputs"] = [i._to_rest() for i in inputs]
        if metadata:
            body["metadata"] = [
                json_format.MessageToDict(m) for m in metadata_dict_to_proto(metadata)
            ]

        response = await run_sync_function(
            lambda: self._rest_client.post(self.CREATE_PATH, json=body)
        )
        _raise_for_status(response)
        data = response.json()
        writes = [ProcessedChannelWrite._from_rest(w) for w in data.get("writes", [])]
        return data["uploadUrl"], data["dataImportId"], writes

    async def get_write(self, processed_channel_write_id: str) -> ProcessedChannelWrite:
        """Get a write by ID."""
        path = f"{self.WRITES_PATH}/{quote(processed_channel_write_id, safe='')}"
        response = await run_sync_function(lambda: self._rest_client.get(path))
        _raise_for_status(response)
        return ProcessedChannelWrite._from_rest(response.json()["write"])

    async def list_writes(
        self,
        *,
        page_size: int | None = None,
        page_token: str | None = None,
        query_filter: str | None = None,
        order_by: str | None = None,
    ) -> tuple[list[ProcessedChannelWrite], str]:
        """List one page of writes, newest first unless ``order_by`` says otherwise."""
        params: dict[str, Any] = {}
        if page_size is not None:
            params["pageSize"] = page_size
        if page_token:
            params["pageToken"] = page_token
        if query_filter:
            params["filter"] = query_filter
        if order_by:
            params["orderBy"] = order_by
        response = await run_sync_function(
            lambda: self._rest_client.get(self.WRITES_PATH, params=params)
        )
        _raise_for_status(response)
        data = response.json()
        writes = [ProcessedChannelWrite._from_rest(w) for w in data.get("writes", [])]
        return writes, data.get("nextPageToken", "")

    async def list_all_writes(
        self,
        *,
        query_filter: str | None = None,
        order_by: str | None = None,
        max_results: int | None = None,
        page_size: int | None = None,
    ) -> list[ProcessedChannelWrite]:
        """List every write that matches the filter, up to ``max_results``."""
        return await self._handle_pagination(
            self.list_writes,
            kwargs={"query_filter": query_filter},
            order_by=order_by,
            max_results=max_results,
            page_size=page_size or DEFAULT_PAGE_SIZE,
        )
