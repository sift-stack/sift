import threading
from contextlib import contextmanager
from types import TracebackType
from typing import Callable, Dict, List, Optional, Type, Union

from typing_extensions import Self, TypeAlias

from sift_py.rest import SiftRestConfig
from sift_py.schemaless_ingestion._internal.ingest import _SchemalessIngestionServiceImpl
from sift_py.schemaless_ingestion.data import SchemalessData

DEFAULT_BUFFER_SIZE = 1_000

FlushCallback: TypeAlias = Callable[[], None]
OnErrorCallback: TypeAlias = Callable[[BaseException, List[SchemalessData], FlushCallback], None]


class BufferedSchemalessIngestionService:
    """
    A fully configured service that, when instantiated, is ready to start ingesting schemaless data.

    This class automates buffering requests and sends them in batches. It is recommended to be used
    in a with-block. Failure to put this in a with-block may result in some data not being ingested unless
    the caller explicitly calls `flush` before the instance of `BufferedSchemalessIngestionService` goes out of scope.
    Once the with-block is exited then a final call to the aforementioned `flush` method  will be made to ingest the remaining data.

    Buffered ingestion works by automatically flushing and ingesting data into Sift whenever the buffer is filled.
    The size of the buffer is configured via the `buffer_size` argument and defaults to `sift_py.schemaless_ingestion.service.DEFAULT_BUFFER_SIZE`.

    It is also possible to configure buffered ingestion to periodically flush the buffer regardless of whether or not the buffer
    is filled. The interval between flushes is set via the `flush_interval_sec` argument which is the number of seconds between each flush.
    If a flush were to occur due to the buffer being filled, then the timer will restart. If `flush_interval_sec` is `None`, then flushes will only
    occur once the buffer is filled and at the end of the scope of the with-block.

    If an error were to occur that would cause the context manager to call `__exit__`, one last attempt to flush the buffer will be made
    before the error is re-raised for the caller to handle. If the caller would instead like to customize `__exit__` behavior in the case
    of an error, they can make use of the `on_error` argument whose type signature is a function where the first argument is the error,
    the second is the buffer containing the uningested request, and the third argument being a function where, when called, will attempt
    to flush the buffer.
    """

    _buffer: List[SchemalessData]
    _buffer_size: int
    _ingestion_service: _SchemalessIngestionServiceImpl
    _flush_interval_sec: Optional[float]
    _flush_timer: Optional[threading.Timer]
    _lock: Optional[threading.Lock]
    _on_error: Optional[OnErrorCallback]

    def __init__(
        self,
        rest_conf: SiftRestConfig,
        asset_name: str,
        run_id: Optional[str] = None,
        organization_id: Optional[str] = None,
        buffer_size: Optional[int] = None,
        flush_interval_sec: Optional[float] = None,
        on_error: Optional[OnErrorCallback] = None,
    ):
        """Initializes the instance

        - `rest_cong`: `SiftRestConfig` defining the REST connection.
        - `asset_name`: The name of the asset to telemeter.
        - `run_id`: The ID of the optional run to associated ingested data with. Can be set later with `attach_run`
        - `organization_id`: Optional ID of the organization of the user.
        - `buffer_size`: Optional length of the buffer. Defaults to `DEFAULT_BUFFER_SIZE`
        - `flush_interval_sec`: Periodic time in secs to flush the buffer. Defaults to never.
        - `on_error`: Optional callback to provide custom handling of errors during ingestion. See class documentation.
        """
        self._ingestion_service = _SchemalessIngestionServiceImpl(
            rest_conf=rest_conf,
            asset_name=asset_name,
            run_id=run_id,
            organization_id=organization_id,
        )

        self._buffer = []
        self._buffer_size = buffer_size or DEFAULT_BUFFER_SIZE
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

    def ingest(self, *data: SchemalessData):
        """
        Ingest one or more SchemalessData objects. Data will be added to a buffer and sent in batches.
        """
        with self._use_lock():
            lhs_cursor = 0
            rhs_cursor = min(
                self._buffer_size - len(self._buffer),
                len(data),
            )

            while lhs_cursor < len(data):
                for item in data[lhs_cursor:rhs_cursor]:
                    self._buffer.append(item)

                if len(self._buffer) >= self._buffer_size:
                    self._flush()

                lhs_cursor = rhs_cursor
                rhs_cursor = min(
                    rhs_cursor + (self._buffer_size - len(self._buffer)),
                    len(data),
                )

    def flush(self):
        """
        Flush and ingest all data in buffer
        """

        if self._flush_timer and self._lock:
            with self._lock:
                self._flush()
            self._restart_flush_timer()
        else:
            self._flush()

    def _flush(self):
        if len(self._buffer) > 0:
            print("Ingesting!")
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

    def attach_run(
        self,
        run_name: str,
        description: str = "",
        organization_id: str = "",
        tags: Optional[List[str]] = None,
        metadata: Optional[Dict[str, Union[str, float, bool]]] = None,
        force_new: bool = False,
    ):
        """
        Retrieve an existing run or create one to use during future ingests with this service.
        Will immediately flush existing buffered data prior to run creation.

        Include `force_new=True` to force the creation of a new run, which will allow creation of a new run using an existing name.
        """
        self.flush()
        self._ingestion_service.attach_run(
            run_name, description, organization_id, tags, metadata, force_new
        )

    def detach_run(self):
        """
        Detach run from future ingests. Subsequent data ingested won't be associated with
        the run being detached. Will immediately flush existing buffered data before disassociating from run.
        """
        self.flush()
        self._ingestion_service.detach_run()
