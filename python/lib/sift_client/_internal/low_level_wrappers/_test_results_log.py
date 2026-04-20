"""Internal log-format primitives for test-result simulation logs.

Houses the file-format pieces that used to live inline in ``test_results.py``:

* Dataclasses describing the log header (``LogTracking``) and the intermediate
  state accumulated while replaying a log (``_ReplayState``, ``ReplayResult``).
* Pure functions for writing log entries, rewriting the tracking header, and
  parsing data lines.

This module has no dependency on the low-level gRPC client; the replay
orchestration still lives on ``TestResultsLowLevelClient`` and uses these
helpers.
"""

from __future__ import annotations

import fcntl
import json
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
    """Tracking metadata stored as line 0 of a log file.

    ``last_uploaded_line`` is the count of data lines (i.e. non-header lines) that
    have been successfully uploaded. Each data line corresponds to a single API
    call, so line granularity matches the atomic unit of work: a line is either
    fully replayed or must be retried in its entirety. Data lines are strictly
    append-only, so this counter is stable across header rewrites.
    """

    last_uploaded_line: int = 0
    id_map: dict[str, str] = field(default_factory=dict)
    client_version: str = field(default_factory=_client_version)

    def to_log_line(self) -> str:
        data = {
            "clientVersion": self.client_version,
            "lastUploadedLine": self.last_uploaded_line,
            "idMap": self.id_map,
        }
        return f"[LogTracking] {json.dumps(data, separators=(',', ':'))}\n"

    @staticmethod
    def from_log_line(line: str) -> LogTracking:
        match = re.match(r"^\[LogTracking\]\s*(.+)$", line.strip())
        if not match:
            return LogTracking()
        data = json.loads(match.group(1))
        return LogTracking(
            last_uploaded_line=data.get("lastUploadedLine", 0),
            id_map=data.get("idMap", {}),
            client_version=data.get("clientVersion", "unknown"),
        )


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


# Concurrency
# -----------------
# A test-result log file is written by the test process (:func:`log_request_to_file`)
# while the ``import-test-result-log --incremental`` subprocess concurrently reads it
# (:func:`iter_log_data_lines`) and rewrites its header line (:func:`update_tracking`).
#
# All three functions synchronize via ``fcntl.flock`` on an exclusive file lock


def log_request_to_file(
    log_file: str | Path,
    request_type: str,
    request: Any,
    response_id: str | None = None,
) -> None:
    """Append a request as a JSON-encoded line to ``log_file``.

    Holds ``LOCK_EX`` across the append so the incremental importer's
    :func:`update_tracking` rewrite cannot race with it. See the module docstring
    above for the full concurrency contract.

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
        # Closing the file flushes and releases the flock atomically; no explicit
        # unlock needed here.
        f.write(line)


def update_tracking(log_file: str | Path, tracking: LogTracking) -> None:
    """Write the LogTracking header as line 0, creating it if missing.

    Holds ``LOCK_EX`` across the entire read-rewrite-truncate cycle so concurrent
    :func:`log_request_to_file` appends cannot slip in between ``readlines()`` and
    ``truncate()``; re-reads inside the lock so any lines appended since the last
    tick are preserved when rewriting. See the module docstring above.

    If the file already has a ``[LogTracking]`` header on line 0, it is replaced
    in place. Otherwise the header is inserted as a new line 0 (so we don't
    clobber an existing data line).
    """
    log_path = Path(log_file)
    new_header = tracking.to_log_line()
    with open(log_path, "r+") as f:
        fcntl.flock(f, fcntl.LOCK_EX)
        lines = f.readlines()
        if lines and lines[0].startswith("[LogTracking]"):
            lines[0] = new_header
        else:
            lines.insert(0, new_header)
        f.seek(0)
        f.writelines(lines)
        f.truncate()


def iter_log_data_lines(
    log_path: Path,
    start_line: int = 0,
) -> Generator[tuple[str, str | None, str], None, None]:
    """Parse data lines from a log file, skipping the LogTracking header.

    Yields ``(request_type, response_id, json_str)`` tuples. Each yielded item
    corresponds to one logged API call.

    ``start_line`` is the count of data lines (1-based) already uploaded; the
    iterator skips the first ``start_line`` data lines and yields the rest.
    Pass 0 to read all data lines.

    Acquires ``LOCK_SH`` only while snapshotting the file into memory, then
    releases before yielding so callers can take ``LOCK_EX`` during iteration
    (e.g. for :func:`update_tracking`). Any lines appended by a concurrent
    :func:`log_request_to_file` call after the snapshot are not visible this
    call -- they will be picked up on the next invocation.
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
        request_type = match.group(1)
        if request_type == "LogTracking":
            continue
        data_line_count += 1
        if data_line_count <= start_line:
            continue
        yield (request_type, match.group(2), match.group(3))
