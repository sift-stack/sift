from types import TracebackType
from typing import Generic, List, Optional, Type, TypeVar

from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataStreamRequest
from typing_extensions import Self

from sift_py.ingestion._internal.ingest import _IngestionServiceImpl
from sift_py.ingestion.flow import Flow, FlowOrderedChannelValues

DEFAULT_BUFFER_SIZE = 1_000

T = TypeVar("T", bound=_IngestionServiceImpl)


class BufferedIngestionService(Generic[T]):
    _buffer: List[IngestWithConfigDataStreamRequest]
    _buffer_size: int
    _ingestion_service: T

    def __init__(self, ingestion_service: T, buffer_size: Optional[int]):
        self._buffer = []
        self._buffer_size = buffer_size or DEFAULT_BUFFER_SIZE
        self._ingestion_service = ingestion_service

    def __enter__(self) -> Self:
        return self

    def __exit__(
        self,
        exc_type: Optional[Type[BaseException]],
        exc_val: Optional[BaseException],
        exc_tb: Optional[TracebackType],
    ) -> bool:
        self.flush()

        if exc_val is not None:
            raise exc_val

        return True

    def ingest_flows(self, *flows: FlowOrderedChannelValues):
        """
        Ingests flows in batches for each request generated from a flow.
        See `sift_py.ingestion.service.IngestionService.create_ingestion_request`
        for more information.
        """
        lhs_cursor = 0
        rhs_cursor = min(
            self._buffer_size - len(self._buffer),
            len(flows),
        )

        while lhs_cursor < len(flows):
            for flow in flows[lhs_cursor:rhs_cursor]:
                flow_name = flow["flow_name"]
                timestamp = flow["timestamp"]
                channel_values = flow["channel_values"]

                req = self._ingestion_service.create_ingestion_request(
                    flow_name=flow_name,
                    timestamp=timestamp,
                    channel_values=channel_values,
                )
                self._buffer.append(req)

            if len(self._buffer) >= self._buffer_size:
                self.flush()

            lhs_cursor = rhs_cursor
            rhs_cursor = min(
                rhs_cursor + (self._buffer_size - len(self._buffer)),
                len(flows),
            )

    def try_ingest_flows(self, *flows: Flow):
        """
        Ingests flows in batches and performs client-side validations for each request
        generated from a flow. See `sift_py.ingestion.service.IngestionService.try_create_ingestion_request`
        for more information.
        """
        lhs_cursor = 0
        rhs_cursor = min(
            self._buffer_size - len(self._buffer),
            len(flows),
        )

        while lhs_cursor < len(flows):
            for flow in flows[lhs_cursor:rhs_cursor]:
                flow_name = flow["flow_name"]
                timestamp = flow["timestamp"]
                channel_values = flow["channel_values"]

                req = self._ingestion_service.try_create_ingestion_request(
                    flow_name=flow_name,
                    timestamp=timestamp,
                    channel_values=channel_values,
                )
                self._buffer.append(req)

            if len(self._buffer) >= self._buffer_size:
                self.flush()

            lhs_cursor = rhs_cursor
            rhs_cursor = min(
                rhs_cursor + (self._buffer_size - len(self._buffer)),
                len(flows),
            )

    def flush(self):
        """
        Flush and ingest all requests in buffer.
        """
        if len(self._buffer) > 0:
            self._ingestion_service.ingest(*self._buffer)
            self._buffer.clear()
