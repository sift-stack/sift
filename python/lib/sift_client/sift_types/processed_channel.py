"""Types for processed channels: channels whose data is written by code, one write per upload.

Processed channels are experimental. The API behind them is unstable and can change without notice.
"""

from __future__ import annotations

from datetime import datetime  # noqa: TC003
from enum import Enum
from typing import TYPE_CHECKING, Any

import grpc
import pandas as pd
from pydantic import BaseModel, ConfigDict, PrivateAttr
from requests import HTTPError

if TYPE_CHECKING:
    from requests import Response

    from sift_client.client import SiftClient


class ProcessedChannelWriteStatus(str, Enum):
    """Status of a processed channel write.

    A write starts ``PENDING`` and ends ``COMPLETE`` or ``FAILED``. Only a complete write is
    readable.
    """

    PENDING = "PENDING"
    COMPLETE = "COMPLETE"
    FAILED = "FAILED"

    @property
    def is_terminal(self) -> bool:
        """Whether the write has finished, successfully or not."""
        return self != ProcessedChannelWriteStatus.PENDING

    def to_filter_str(self) -> str:
        """Convert to the value the list filter expects."""
        return f"PROCESSED_CHANNEL_WRITE_STATUS_{self.value}"

    @classmethod
    def _from_api(cls, value: str) -> ProcessedChannelWriteStatus:
        return cls(value.replace("PROCESSED_CHANNEL_WRITE_STATUS_", ""))


class ProcessedChannelCode(BaseModel):
    """The code that produced a write, as the caller identifies it.

    The server hashes it, with the output schema, into the write's fingerprint. Two writes with
    the same fingerprint ran the same code and produced the same channels.

    Attributes:
        ref: A durable reference to the code, such as a commit and path, a package name and
            version, or a script URL.
        hash: A content hash of the code at ``ref``. Preferred over ``ref`` in the fingerprint,
            because two refs can name the same code.
    """

    model_config = ConfigDict(frozen=True)

    ref: str
    hash: str | None = None

    def _to_rest(self) -> dict[str, Any]:
        body: dict[str, Any] = {"ref": self.ref}
        if self.hash:
            body["hash"] = self.hash
        return body


class ProcessedChannelInput(BaseModel):
    """A channel the code read to produce a write.

    Attributes:
        channel_id: The channel that was read.
        run_id: The run it was read on.
        processed_channel_write_id: The write that was read, when the input is itself a
            processed channel.
    """

    model_config = ConfigDict(frozen=True)

    channel_id: str
    run_id: str
    processed_channel_write_id: str | None = None

    def _to_rest(self) -> dict[str, Any]:
        body: dict[str, Any] = {"channelId": self.channel_id, "runId": self.run_id}
        if self.processed_channel_write_id:
            body["processedChannelWriteId"] = self.processed_channel_write_id
        return body


def _parse_time(value: Any) -> datetime | None:
    if not value:
        return None
    return pd.Timestamp(value).to_pydatetime()


class ProcessedChannelWrite(BaseModel):
    """One write of one processed channel's data to one run.

    For each channel and run, the complete write with the highest ``sequence`` is current, and is
    what reads return. Older writes stay readable by ID.

    Attributes:
        id_: The write's ID.
        channel_id: The channel the write belongs to.
        asset_id: The channel's asset.
        run_id: The run the write targets.
        status: ``PENDING``, ``COMPLETE`` or ``FAILED``.
        sequence: 1-based per channel and run, in creation order. A failed write keeps its number.
        is_current: Whether this is the write that reads of the channel on this run return.
        fingerprint: Server-computed digest of the code and output schema. Compare it, don't
            parse it.
        data_import_id: The data import that carries the write's data.
        supersedes_processed_channel_write_id: The write this one replaced, set on completion.
        data_start_time: The time of the write's first point, set on completion.
        data_stop_time: The time of the write's last point, set on completion.
        point_count: The number of points written, set on completion.
        error_message: Why the write failed.
        code: The code that produced the write.
        created_date: When the write was created.
        completed_date: When the write completed or failed.
        created_by_user_id: Who created the write.
    """

    model_config = ConfigDict(frozen=True)

    id_: str
    channel_id: str
    asset_id: str
    run_id: str
    status: ProcessedChannelWriteStatus
    sequence: int
    is_current: bool = False
    fingerprint: str = ""
    data_import_id: str | None = None
    supersedes_processed_channel_write_id: str | None = None
    data_start_time: datetime | None = None
    data_stop_time: datetime | None = None
    point_count: int | None = None
    error_message: str | None = None
    code: ProcessedChannelCode | None = None
    created_date: datetime | None = None
    completed_date: datetime | None = None
    created_by_user_id: str = ""

    @classmethod
    def _from_rest(cls, body: dict[str, Any]) -> ProcessedChannelWrite:
        """Build a write from its REST (protojson) representation."""
        code = body.get("code")
        point_count = body.get("pointCount")
        return cls(
            id_=body["processedChannelWriteId"],
            channel_id=body.get("channelId", ""),
            asset_id=body.get("assetId", ""),
            run_id=body.get("runId", ""),
            status=ProcessedChannelWriteStatus._from_api(body["status"]),
            sequence=int(body.get("sequence") or 0),
            is_current=bool(body.get("isCurrent", False)),
            fingerprint=body.get("fingerprint", ""),
            data_import_id=body.get("dataImportId") or None,
            supersedes_processed_channel_write_id=(
                body.get("supersedesProcessedChannelWriteId") or None
            ),
            data_start_time=_parse_time(body.get("dataStartTime")),
            data_stop_time=_parse_time(body.get("dataStopTime")),
            point_count=int(point_count) if point_count is not None else None,
            error_message=body.get("errorMessage") or None,
            code=ProcessedChannelCode(ref=code["ref"], hash=code.get("hash") or None)
            if code and code.get("ref")
            else None,
            created_date=_parse_time(body.get("createdDate")),
            completed_date=_parse_time(body.get("completedDate")),
            created_by_user_id=body.get("createdByUserId", ""),
        )


class ProcessedChannelUpload(BaseModel):
    """The writes created by one ``client.processed_channel.upload`` call.

    One upload writes every channel in it to the same run, and its writes complete or fail
    together.

    Attributes:
        data_import_id: The data import that carries the upload.
        job_id: The import job, once the file is uploaded.
        writes: One write per channel, in the order of the DataFrame's columns.
    """

    data_import_id: str
    job_id: str | None = None
    writes: list[ProcessedChannelWrite]
    _client: SiftClient | None = PrivateAttr(default=None)

    @property
    def is_complete(self) -> bool:
        """Whether every write has finished, successfully or not."""
        return all(w.status.is_terminal for w in self.writes)

    @property
    def succeeded(self) -> bool:
        """Whether every write is complete and readable."""
        return all(w.status == ProcessedChannelWriteStatus.COMPLETE for w in self.writes)

    @property
    def failed_writes(self) -> list[ProcessedChannelWrite]:
        """The writes that failed."""
        return [w for w in self.writes if w.status == ProcessedChannelWriteStatus.FAILED]

    @property
    def client(self) -> SiftClient:
        """The client this upload was made with."""
        if self._client is None:
            raise AttributeError(
                "Sift client not set. Please retrieve with the SiftClient to use this method."
            )
        return self._client

    def _apply_client_to_instance(self, client: SiftClient) -> None:
        self._client = client

    def wait_until_complete(
        self,
        *,
        polling_interval_secs: float = 2,
        timeout_secs: float | None = None,
        raise_on_failure: bool = True,
    ) -> ProcessedChannelUpload:
        """Wait until every write is complete or failed.

        Args:
            polling_interval_secs: Seconds between status polls.
            timeout_secs: Maximum seconds to wait. If None, waits indefinitely.
            raise_on_failure: Raise ``ProcessedChannelUploadError`` if any write failed.

        Returns:
            This upload, with its writes refreshed.
        """
        finished = self.client._processed_channel.wait_until_complete(
            self,
            polling_interval_secs=polling_interval_secs,
            timeout_secs=timeout_secs,
            raise_on_failure=raise_on_failure,
        )
        self.writes = finished.writes
        return self


class ProcessedChannelUploadError(RuntimeError):
    """Raised when a processed channel upload finishes with failed writes.

    Attributes:
        upload: The finished upload. Its ``failed_writes`` carry each failure's error message.
    """

    def __init__(self, upload: ProcessedChannelUpload):
        """Build the error from a finished upload."""
        self.upload = upload
        messages = sorted({w.error_message or "no error message" for w in upload.failed_writes})
        super().__init__(
            f"{len(upload.failed_writes)} of {len(upload.writes)} processed channel writes failed "
            f"(data import {upload.data_import_id}): {'; '.join(messages)}"
        )


class ProcessedChannelRequestError(HTTPError):
    """Raised when Sift refuses a processed channels request.

    For example, when processed channels aren't enabled for the organization, or a channel name
    is already used by ingested data on the asset. Subclasses ``requests.HTTPError``, so
    ``response`` holds the HTTP response.

    Attributes:
        code: The gRPC status code of the refusal, or None if the server didn't return one.
        server_message: The server's explanation.
    """

    def __init__(self, response: Response, *, code: int | None, server_message: str):
        """Build the error from the server's response and status."""
        self.code = next((c for c in grpc.StatusCode if c.value[0] == code), None)
        self.server_message = server_message
        super().__init__(
            f"Sift refused the request (HTTP {response.status_code}): {server_message}",
            response=response,
        )
