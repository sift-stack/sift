"""Internal log-format primitives for test-result simulation logs.

Three files per run:

* **Log file** (e.g. ``foo.jsonl``) - append-only record of each logged API call,
  one line per call. Written by :func:`log_request_to_file` in the test process
  and read by :func:`_read_log_lines` / the replay subprocess. Has no header:
  every line is a data line.
* **Lock sidecar** (``foo.jsonl.lock``) - empty file used by ``filelock`` locks
  to coordinate appends and snapshot reads across processes. Created on demand;
  ``filelock`` leaves it in place on release on Unix (unlinking is racy) and
  removes it on Windows when uncontended. Either way it is harmless to leave
  or delete after the run.
* **Tracking sidecar** (``foo.jsonl.tracking``) - small JSON file holding the
  incremental replay cursor (``lastUploadedLine``) and the simulated-to-real ID
  map. Written only by the replay subprocess via :meth:`LogTracking.save` using
  a temp-file + ``os.replace`` so a crash can't leave a half-written sidecar.
  Read once at replay start via :meth:`LogTracking.load`. Never touched by the
  test process.

# Concurrency

With tracking moved out of the main log, the log file is strictly append-only
and has exactly one in-place mutator (the writer) and one scanner (the replay
subprocess). Both serialize through a cross-platform exclusive
:class:`filelock.AsyncFileLock` on the lock sidecar: the writer holds it across
a single append, the reader holds it across the snapshot ``readlines()``. That
keeps a concurrent reader from observing a mid-append partial final line on any
OS, including Windows where POSIX advisory ``flock`` is unavailable. The
exclusive-only lock means a hot reader briefly blocks the writer (and vice
versa), which is acceptable because writes are tiny and we only have one reader.

Waiting for the lock is cooperative: ``AsyncFileLock`` retries a non-blocking
acquire and sleeps on the event loop between attempts, so a contended or stale
lock never blocks the loop thread (the hang in ENG-12003). The acquire is also
bounded by ``LOG_LOCK_TIMEOUT_SECONDS``, surfacing a stuck lock as
``TimeoutError`` instead of waiting forever. The file read/write under the lock
runs inline on the loop; it is a tiny local-file operation, the same class of
brief synchronous work async methods do throughout this client.

The tracking sidecar has a single writer (the replay subprocess) and no live
reader, so it needs no locking. Atomic rename is still used to keep the on-disk
contents valid across crashes.

The lock is advisory: it only protects callers that go through these helpers.
Ad-hoc writers to the same path are not protected.
"""

from __future__ import annotations

import json
import os
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import TYPE_CHECKING, Any, Generator

from filelock import AsyncFileLock, Timeout
from google.protobuf import json_format

if TYPE_CHECKING:
    from sift_client.sift_types.test_report import TestMeasurement, TestReport, TestStep


# Seconds to wait for the sidecar lock before raising TimeoutError. Long enough
# to absorb brief contention, short enough that a stale holder can't block forever.
# AsyncFileLock's own default is -1 (wait forever), which is the original hang.
LOG_LOCK_TIMEOUT_SECONDS = 60.0


def _client_version() -> str:
    from importlib.metadata import PackageNotFoundError, version

    try:
        return version("sift_stack_py")
    except PackageNotFoundError:
        return "unknown"


@dataclass
class LogTracking:
    """Incremental-replay cursor and simulated-to-real ID map.

    Persisted beside the log file (see module docstring for layout). The log
    file itself is append-only and stores only API-call data lines.

    * ``last_uploaded_line`` is the count of data lines that have been
      successfully replayed against the server. Each data line corresponds to a
      single API call, so line granularity matches the atomic unit of work: a
      line is either fully replayed or must be retried in its entirety. Data
      lines are strictly append-only, so this counter is stable across runs.
    * ``id_map`` maps simulated response IDs (created during the original test
      run) to the real IDs assigned by the server during replay. Subsequent
      ``Update*`` entries consult this map to translate IDs.
    """

    last_uploaded_line: int = 0
    id_map: dict[str, str] = field(default_factory=dict)
    client_version: str = field(default_factory=_client_version)

    @staticmethod
    def sidecar_path(log_path: str | Path) -> Path:
        """Return the sidecar path for a given log file (``<log>.tracking``)."""
        p = Path(log_path)
        return p.with_name(p.name + ".tracking")

    @classmethod
    def load(cls, log_path: str | Path) -> LogTracking:
        """Read tracking state for ``log_path``; return a fresh instance if missing or corrupt.

        A missing sidecar is the normal state before the first incremental tick.
        A malformed sidecar is treated the same so a crash mid-write can't brick
        replay; the worst case is a re-replay of already-uploaded lines, which
        the server must be prepared for anyway.
        """
        sidecar = cls.sidecar_path(log_path)
        try:
            data = json.loads(sidecar.read_text())
        except (FileNotFoundError, json.JSONDecodeError, OSError):
            return cls()
        return cls(
            last_uploaded_line=data.get("lastUploadedLine", 0),
            id_map=data.get("idMap", {}),
            client_version=data.get("clientVersion", "unknown"),
        )

    def save(self, log_path: str | Path) -> None:
        """Atomically write tracking state to the sidecar for ``log_path``.

        Uses temp-file + ``os.replace`` so readers (and crash recovery) never
        observe a partially written sidecar.
        """
        sidecar = self.sidecar_path(log_path)
        sidecar.parent.mkdir(parents=True, exist_ok=True)
        payload = json.dumps(
            {
                "clientVersion": self.client_version,
                "lastUploadedLine": self.last_uploaded_line,
                "idMap": self.id_map,
            },
            separators=(",", ":"),
        )
        tmp = sidecar.with_name(sidecar.name + ".tmp")
        tmp.write_text(payload)
        os.replace(tmp, sidecar)


@dataclass
class _ReplayState:
    """Mutable state accumulated during log replay."""

    report: TestReport | None = None
    steps_by_id: dict[str, TestStep] = field(default_factory=dict)
    steps_order: list[str] = field(default_factory=list)
    measurements_by_id: dict[str, TestMeasurement] = field(default_factory=dict)
    measurements_order: list[str] = field(default_factory=list)


@dataclass
class ReplayResult:
    """Result of replaying a log file."""

    report: TestReport
    steps: list[TestStep] = field(default_factory=list)
    measurements: list[TestMeasurement] = field(default_factory=list)


async def log_request_to_file(
    log_file: str | Path,
    request_type: str,
    request: Any,
    response_id: str | None = None,
    timeout: float = LOG_LOCK_TIMEOUT_SECONDS,
) -> None:
    """Append a request as a JSON-encoded line to ``log_file``.

    Holds an exclusive :class:`filelock.AsyncFileLock` on the sidecar across the
    append so a concurrent reader in :func:`_read_log_lines` can't see a
    mid-write partial final line. Waiting for the lock is cooperative and never
    blocks the event loop; see the module docstring for the full concurrency
    model.

    Args:
        log_file: Path to the log file.
        request_type: Type of request being logged.
        request: The protobuf request to log.
        response_id: Optional ID from the simulated response, embedded in the tag
            for create operations so replay can map previously simulated IDs used
            by simulated updates.
        timeout: Seconds to wait for the sidecar lock before raising TimeoutError.

    Raises:
        TimeoutError: If the lock is not acquired within ``timeout`` seconds.
    """
    log_path = Path(log_file)
    log_path.parent.mkdir(parents=True, exist_ok=True)
    tag = f"{request_type}:{response_id}" if response_id else request_type
    request_dict = json_format.MessageToDict(request)
    request_json = json.dumps(request_dict, separators=(",", ":"))
    line = f"[{tag}] {request_json}\n"
    lock_path = log_path.with_name(log_path.name + ".lock")
    try:
        # run_in_executor=False keeps each (instant, non-blocking) acquire
        # attempt inline on the loop thread. With the default executor-based
        # attempts, a task cancellation landing during the attempt's executor
        # round-trip strands the lock: the worker still takes the flock, but
        # CancelledError skips release and the fd is held until process exit.
        # Inline, the only cancellation point is the poll sleep between failed
        # attempts, where no fd is held. Waiting stays cooperative either way.
        async with AsyncFileLock(str(lock_path), timeout=timeout, run_in_executor=False):
            with open(log_path, "a") as f:
                # The inner ``with`` flushes and closes the file before the
                # lock is released, so no reader sees a partial line.
                f.write(line)
    except Timeout as exc:
        raise TimeoutError(
            f"Timed out after {timeout}s acquiring the test-results log lock at "
            f"{lock_path}; another process or thread is holding it."
        ) from exc


async def _read_log_lines(
    log_path: str | Path,
    timeout: float = LOG_LOCK_TIMEOUT_SECONDS,
) -> list[str]:
    """Snapshot the log file's raw lines under the sidecar lock.

    Holds the exclusive :class:`filelock.AsyncFileLock` only across the
    ``readlines`` so a concurrent :func:`log_request_to_file` append can't be
    observed as a partial final line, then releases it. Waiting for the lock is
    cooperative and never blocks the event loop. Parsing happens lock-free in
    :func:`parse_log_data_lines`.

    Raises:
        TimeoutError: If the lock is not acquired within ``timeout`` seconds.
    """
    log_path = Path(log_path)
    lock_path = log_path.with_name(log_path.name + ".lock")
    try:
        # run_in_executor=False for cancel-safety; see log_request_to_file.
        async with AsyncFileLock(str(lock_path), timeout=timeout, run_in_executor=False):
            with open(log_path) as f:
                return f.readlines()
    except Timeout as exc:
        raise TimeoutError(
            f"Timed out after {timeout}s acquiring the test-results log lock at "
            f"{lock_path}; another process or thread is holding it."
        ) from exc


def parse_log_data_lines(
    raw_lines: list[str],
    start_line: int = 0,
) -> Generator[tuple[str, str | None, str], None, None]:
    """Parse a snapshot of raw log lines into data-line tuples.

    Yields ``(request_type, response_id, json_str)`` tuples, one per logged API
    call. Pure and lock-free: operates on the in-memory snapshot returned by
    :func:`_read_log_lines`.

    ``start_line`` is the count of data lines (1-based) already uploaded; the
    iterator skips the first ``start_line`` lines and yields the rest. Pass 0
    to read all data lines.
    """
    line_pattern = re.compile(r"^\[(\w+)(?::([^\]]+))?\]\s*(.+)$")
    data_line_count = 0
    for raw_line in raw_lines:
        line = raw_line.strip()
        if not line:
            continue
        match = line_pattern.match(line)
        if not match:
            raise ValueError(f"Invalid log line: {line}")
        data_line_count += 1
        if data_line_count <= start_line:
            continue
        yield (match.group(1), match.group(2), match.group(3))
