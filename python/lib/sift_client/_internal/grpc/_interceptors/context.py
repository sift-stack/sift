from typing import Optional, Sequence, Tuple, Union

import grpc


class ClientCallDetails(grpc.ClientCallDetails):
    method: str
    timeout: Optional[float]
    metadata: Optional[Sequence[Tuple[str, Union[str, bytes]]]]
    credentials: Optional[grpc.CallCredentials]
    wait_for_ready: Optional[bool]

    def __init__(
        self,
        method: str,
        timeout: Optional[float],
        metadata: Optional[Sequence[Tuple[str, Union[str, bytes]]]],
        credentials: Optional[grpc.CallCredentials],
        wait_for_ready: Optional[bool],
    ):
        self.method = method
        self.timeout = timeout
        self.metadata = metadata
        self.credentials = credentials
        self.wait_for_ready = wait_for_ready
