import threading
from contextlib import contextmanager
from types import TracebackType
from typing import Callable, Generic, List, Optional, Type, TypeVar

from sift.ingest.v1.ingest_pb2 import IngestWithConfigDataStreamRequest
from typing_extensions import Self, TypeAlias

from sift_py.ingestion._internal.ingest import _IngestionServiceImpl
from sift_py.ingestion.flow import Flow, FlowOrderedChannelValues

DEFAULT_BUFFER_SIZE = 1_000

T = TypeVar("T", bound=_IngestionServiceImpl)

FlushCallback: TypeAlias = Callable[[], None]
OnErrorCallback: TypeAlias = Callable[
    [BaseException, List[IngestWithConfigDataStreamRequest], FlushCallback], None
]


class BufferedIngestionService(Generic[T]):
    """
    See `sift_py.ingestion.service.IngestionService.buffered_ingestion`
    for more information and how to leverage buffered ingestion.
    """

    _buffer: List[IngestWithConfigDataStreamRequest]
    _buffer_size: int
    _ingestion_service: T
    _flush_interval_sec: Optional[float]
    _flush_timer: Optional[threading.Timer]
    _lock: Optional[threading.Lock]
    _on_error: Optional[OnErrorCallback]

    def __init__(
        self,
        ingestion_service: T,
        buffer_size: Optional[int],
        flush_interval_sec: Optional[float],
        on_error: Optional[OnErrorCallback],
    ):
        self._buffer = []
        self._buffer_size = buffer_size or DEFAULT_BUFFER_SIZE
        self._ingestion_service = ingestion_service
        self._on_error = on_error
        self._flush_timer = None

        if flush_interval_sec:
            self._flush_interval_sec = flush_interval_sec
            self._lock = threading.Lock()
            self._start_flush_timer()
        else:
            self._flush_interval_sec = None
            self._lock = None

    def __enter__(self) -> Self:
        return self

    def __exit__(
        self,
        exc_type: Optional[Type[BaseException]],
        exc_val: Optional[BaseException],
        exc_tb: Optional[TracebackType],
    ) -> bool:
        self._cancel_flush_timer()

        if exc_val is not None:
            if self._on_error is not None:
                self._on_error(exc_val, self._buffer, self.flush)
            else:
                self.flush()

            raise exc_val
        else:
            self.flush()

        return True

    def ingest_flows(self, *flows: FlowOrderedChannelValues):
        """
        Ingests flows in batches for each request generated from a flow.
        See `sift_py.ingestion.service.IngestionService.create_ingestion_request`
        for more information.
        """
        with self._use_lock():
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
                    self._flush()

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
        with self._use_lock():
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
                    self._flush()

                lhs_cursor = rhs_cursor
                rhs_cursor = min(
                    rhs_cursor + (self._buffer_size - len(self._buffer)),
                    len(flows),
                )

    def flush(self):
        """
        Flush and ingest all requests in buffer.
        """

        if self._flush_timer and self._lock:
            with self._lock:
                self._flush()
            self._restart_flush_timer()
        else:
            self._flush()

    def _flush(self):
        if len(self._buffer) > 0:
            self._ingestion_service.ingest(*self._buffer)
            self._buffer.clear()

    def _start_flush_timer(self):
        if self._flush_interval_sec:
            self._flush_timer = threading.Timer(self._flush_interval_sec, self.flush)
            self._flush_timer.start()

    def _cancel_flush_timer(self):
        if self._flush_timer:
            self._flush_timer.cancel()
            self._flush_timer = None

    def _restart_flush_timer(self):
        self._cancel_flush_timer()
        self._start_flush_timer()

    @contextmanager
    def _use_lock(self):
        try:
            if self._lock:
                self._lock.acquire()
            yield
        finally:
            if self._lock:
                self._lock.release()
