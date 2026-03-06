from __future__ import annotations

from typing import Sequence

import grpc


class ClientCallDetails(grpc.ClientCallDetails):
    method: str
    timeout: float | None
    metadata: Sequence[tuple[str, str | bytes]] | None
    credentials: grpc.CallCredentials | None
    wait_for_ready: bool | None

    def __init__(
        self,
        method: str,
        timeout: float | None,
        metadata: Sequence[tuple[str, str | bytes]] | None,
        credentials: grpc.CallCredentials | None,
        wait_for_ready: bool | None,
    ):
        self.method = method
        self.timeout = timeout
        self.metadata = metadata
        self.credentials = credentials
        self.wait_for_ready = wait_for_ready
