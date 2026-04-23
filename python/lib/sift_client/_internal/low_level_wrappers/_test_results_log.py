"""Internal log-format primitives for test-result simulation logs.

Two files per run:

* **Log file** (e.g. ``foo.jsonl``) - append-only record of each logged API call,
  one line per call. Written by :func:`log_request_to_file` in the test process
  and read by :func:`iter_log_data_lines` / the replay subprocess. Has no header:
  every line is a data line.
* **Tracking sidecar** (``foo.jsonl.tracking``) - small JSON file holding the
  incremental replay cursor (``lastUploadedLine``) and the simulated-to-real ID
  map. Written only by the replay subprocess via :meth:`LogTracking.save` using
  a temp-file + ``os.replace`` so a crash can't leave a half-written sidecar.
  Read once at replay start via :meth:`LogTracking.load`. Never touched by the
  test process.

# Concurrency

With tracking moved out of the main log, the log file becomes strictly
append-only and has exactly one in-place mutator (the writer) and one scanner
(the replay subprocess). POSIX guarantees that an ``O_APPEND`` write atomically
bumps the EOF, so parallel writers can't lose data. To keep a concurrent reader
from observing a mid-append partial final line we still take ``LOCK_EX`` on the
writer's single append and ``LOCK_SH`` on the reader's ``readlines()``; there
is never any exclusive-vs-exclusive contention because nothing rewrites the
file any more.

The sidecar has a single writer (the replay subprocess) and no live reader, so
it needs no locking. Atomic rename is still used to keep the on-disk contents
valid across crashes.

``flock`` is advisory, so this contract only holds for processes that use these
helpers; ad-hoc writers are not protected.
"""

from __future__ import annotations

import fcntl
import json
import os
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import TYPE_CHECKING, Any, Generator

from google.protobuf import json_format

if TYPE_CHECKING:
    from sift_client.sift_types.test_report import TestMeasurement, TestReport, TestStep


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


def log_request_to_file(
    log_file: str | Path,
    request_type: str,
    request: Any,
    response_id: str | None = None,
) -> None:
    """Append a request as a JSON-encoded line to ``log_file``.

    Takes ``LOCK_EX`` across the append so a concurrent reader holding
    ``LOCK_SH`` in :func:`iter_log_data_lines` can't see a mid-write partial
    final line. See the module docstring for the full concurrency model.

    Args:
        log_file: Path to the log file.
        request_type: Type of request being logged.
        request: The protobuf request to log.
        response_id: Optional ID from the simulated response, embedded in the tag
            for create operations so replay can map previously simulated IDs used
            by simulated updates.
    """
    log_path = Path(log_file)
    log_path.parent.mkdir(parents=True, exist_ok=True)
    tag = f"{request_type}:{response_id}" if response_id else request_type
    request_dict = json_format.MessageToDict(request)
    request_json = json.dumps(request_dict, separators=(",", ":"))
    line = f"[{tag}] {request_json}\n"
    with open(log_path, "a") as f:
        fcntl.flock(f, fcntl.LOCK_EX)
        # Closing the file flushes and releases the flock atomically; no
        # explicit unlock needed here.
        f.write(line)


def iter_log_data_lines(
    log_path: Path,
    start_line: int = 0,
) -> Generator[tuple[str, str | None, str], None, None]:
    """Parse data lines from a log file.

    Yields ``(request_type, response_id, json_str)`` tuples. Each yielded item
    corresponds to one logged API call.

    ``start_line`` is the count of data lines (1-based) already uploaded; the
    iterator skips the first ``start_line`` lines and yields the rest. Pass 0
    to read all data lines.

    Acquires ``LOCK_SH`` only while snapshotting the file into memory, then
    releases before yielding. Lines appended by a concurrent
    :func:`log_request_to_file` after the snapshot are not visible this call --
    they will be picked up on the next invocation.
    """
    line_pattern = re.compile(r"^\[(\w+)(?::([^\]]+))?\]\s*(.+)$")
    with open(log_path) as f:
        fcntl.flock(f, fcntl.LOCK_SH)
        raw_lines = f.readlines()

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
