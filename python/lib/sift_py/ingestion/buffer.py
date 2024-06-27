from types import TracebackType
from typing import List, Optional, Type
from sift.ingest.v1.ingest_pb2_grpc import IngestServiceStub
from typing_extensions import Self

from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataStreamRequest

DEFAULT_BUFFER_SIZE = 1_000

class IngestRequestBuffer:
    _buffer: List[IngestWithConfigDataStreamRequest]
    _buffer_size: int
    _ingest_service_stub: IngestServiceStub

    def __init__(
        self,
        ingest_service_stub: IngestServiceStub,
        buffer_size: int = DEFAULT_BUFFER_SIZE,
    ):
        self._buffer = []
        self._buffer_size = buffer_size
        self._ingest_service_stub = ingest_service_stub

    def __enter__(self) -> Self:
        return self

    def __exit__(
        self,
        exc_type: Optional[Type[BaseException]],
        exc_val: Optional[BaseException],
        exc_tb: Optional[TracebackType],
    ) -> bool:
        if exc_val is not None:
            raise exc_val

        return True

    def ingest(self, ):
        self._buffer.append()

        if len(self._buffer) >= self._buffer_size:
            self._ingest_service_stub.IngestWithConfigDataStream(self._buffer)
            self._buffer.clear()

        
